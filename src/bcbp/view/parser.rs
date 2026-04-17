// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

use crate::bcbp::{
    error::{BcbpResult, Error},
    format::Field,
    parser::cursor::Cursor,
    view::{BcbpView, Leg, SecurityData},
};

/// Parses a boarding pass from `input_data` representable as a string reference.
pub fn decode_bcbp_view(input: &str) -> BcbpResult<BcbpView<'_>> {
    if !input.is_ascii() {
        return Err(Error::InvalidCharacters);
    }

    let mut cursor = Cursor::new(input);

    // Item 1: Format Code, 1 character, M for standard IATA BCBP.
    let code = cursor.read_char(Field::FormatCode)?;

    if code != 'M' {
        return Err(Error::InvalidFormatCode(code));
    }

    // Item 5: Number of legs encoded, 1 character, 1-9.
    let legs_count = cursor.read_u8(Field::NumberOfLegsEncoded, 10)?;

    if !(1..=9).contains(&legs_count) {
        return Err(Error::InvalidNumberOfLegs);
    }

    let mut bcbp = BcbpView {
        // Item 11: Passenger Name, 20 characters, left justified, space filled.
        pax_name: cursor.read_str(Field::PassengerName)?,
        // Item 253: Electronic Ticket Indicator, 1 character, 'E' for electronic ticket, blank for none.
        eticket_flag: cursor.read_char(Field::ElectronicTicketIndicator)?,
        ..Default::default()
    };

    let mut leg = decode_leg(&mut cursor)?;

    decode_conditional(&mut cursor, &mut bcbp, &mut leg)?;

    bcbp.legs.push(leg);

    for _ in 1..legs_count {
        let mut leg = decode_leg(&mut cursor)?;

        // Item 6: Field Size of variable size field (Conditional + Airline item 4)
        let cond_size = cursor.read_usize(Field::FieldSizeOfVariableSizeField, 16)?;
        if cond_size > 0 {
            let mut cond_data = cursor.read_chunk(cond_size)?;
            decode_leg_conditional(&mut cond_data, &mut leg)?;
        }

        bcbp.legs.push(leg);
    }

    // Remaining input is ascribed to Security Data.
    if cursor.remaining() > 0 {
        bcbp.security_data = decode_security_data(&mut cursor)?;
    }

    if !cursor.is_eof() {
        Err(Error::TrailingData)
    } else {
        Ok(bcbp)
    }
}

fn decode_conditional<'a>(
    cursor: &mut Cursor<'a>,
    bcbp: &mut BcbpView<'a>,
    leg: &mut Leg<'a>,
) -> BcbpResult<()> {
    // Field size of the variable size field that follows for the unique fields.
    let cond_size = cursor.read_usize(Field::FieldSizeOfVariableSizeField, 16)?;
    if cond_size == 0 {
        return Ok(());
    }

    // if cond_size > cursor.remaining() {
    //     return Err(Error::ConditionalDataSize);
    // }

    // chunk over the entire set of conditional fields.
    let mut cond_data = cursor.read_chunk(cond_size)?;

    // Item 8: Beginning of Version Number, 1 character, '<' or '>'.
    let prefix = cond_data.read_char(Field::BeginningOfVersionNumber)?;
    if prefix != '<' && prefix != '>' {
        return Err(Error::InvalidPrefix(
            Field::BeginningOfVersionNumber,
            prefix,
        ));
    }

    // Item 9: Version Number, 1 character, '1'..'9'
    if cond_data.remaining() > 0 {
        let _ = cond_data.read_str(Field::VersionNumber)?;
    }

    // Conditional unique fields are embedded in their own variable-length wrapper.
    if cond_data.remaining() > 0 {
        // Item 10: Field Size of Structured Message Unique, 2 characters
        let len = cond_data.read_usize(Field::FieldSizeOfStructuredMessageUnique, 16)?;
        if len > 0 {
            let mut unique_chunk = cond_data.read_chunk(len)?;

            // Item 15: Passenger Description, 1 character
            bcbp.pax_description = unique_chunk.read_char_opt(Field::PassengerDescription)?;
            // Item 12: Source of Check-In, 1 character
            bcbp.source_of_check_in = unique_chunk.read_char_opt(Field::SourceOfCheckIn)?;
            // Item 14: Source of Boarding Pass Issuance, 1 character
            bcbp.source_of_boarding_pass_issuance =
                unique_chunk.read_char_opt(Field::SourceOfBoardingPassIssuance)?;
            // Item 22: Date of Issue of Boarding Pass, 4 characters
            bcbp.date_of_issue_of_boarding_pass =
                unique_chunk.read_str_opt(Field::DateOfIssueOfBoardingPass)?;
            // Item 16: Document Type, 1 character
            bcbp.doc_type = unique_chunk.read_char_opt(Field::DocumentType)?;
            // Item 21: Airline Designator of Boarding Pass Issuer, 3 characters
            bcbp.airline_designator_of_boarding_pass_issuer = unique_chunk
                .read_str_opt(Field::AirlineDesignatorOfBoardingPassIssuer)?;

            // Item 23: Baggage Tag License Plate, 13 characters
            bcbp.baggage_tags = unique_chunk.read_str_opt(Field::BaggageTagNumbers)?;

            // Item 31: First Non-Consecutive Baggage Tag License Plate, 13 characters
            bcbp.nonconsecutive_baggage_tags1 = unique_chunk
                .read_str_opt(Field::FirstNonConsecutiveBaggageTagNumbers)?;

            // Item 32: Second Non-Consecutive Baggage Tag License Plate, 13 characters
            bcbp.nonconsecutive_baggage_tags2 = unique_chunk
                .read_str_opt(Field::SecondNonConsecutiveBaggageTagNumbers)?;
        }
    }

    decode_leg_conditional(&mut cond_data, leg)?;

    Ok(())
}

fn decode_leg<'a>(cursor: &mut Cursor<'a>) -> BcbpResult<Leg<'a>> {
    Ok(Leg {
        // Item 7: Operating Carrier PNR Code, 7 characters, left justified, space filled.
        pnr: cursor.read_str(Field::OperatingCarrierPnrCode)?,
        // Item 26: From City Airport Code, 3 characters, IATA airport code.
        src_airport: cursor.read_str(Field::FromCityAirportCode)?,
        // Item 38: To City Airport Code, 3 characters, IATA airport code.
        dst_airport: cursor.read_str(Field::ToCityAirportCode)?,
        // Item 42: Operating Carrier Designator, 3 characters, IATA airline designator.
        operating_carrier: cursor.read_str(Field::OperatingCarrier)?,
        // Item 43: Flight Number, 5 characters, numeric, right justified, zero filled.
        flight_number: cursor.read_str(Field::FlightNumber)?,
        // Item 46: Date of Flight, 3 characters, Julian date, DDD.
        flight_day: cursor.read_str(Field::DateOfFlight)?,
        // Item 71: Compartment Code, 1 character, class of service.
        compartment: cursor.read_char(Field::CompartmentCode)?,
        // Item 104: Seat Number, 4 characters, left justified, space filled.
        seat: cursor.read_str(Field::SeatNumber)?,
        // Item 107: Check-in Sequence Number, 5 characters, numeric, right justified, zero filled.
        checkin_sequence: cursor.read_str(Field::CheckInSequenceNumber)?,
        // Item 113: Passenger Status, 1 character
        pax_status: cursor.read_char(Field::PassengerStatus)?,
        ..Default::default()
    })
}

fn decode_leg_conditional<'a>(span: &mut Cursor<'a>, leg: &mut Leg<'a>) -> BcbpResult<()> {
    // Item 6: Field Size of variable size field (Conditional + Airline item 4)
    let len = span.read_usize(Field::FieldSizeOfStructuredMessageRepeated, 16)?;
    if len == 0 {
        return Ok(());
    }

    let mut repeated = span.read_chunk(len)?;

    // Item 142: Airline Numeric Code, 3 characters
    leg.airline_numeric_code = repeated.read_str_opt(Field::AirlineNumericCode)?;

    // Item 143: Document Form/Serial Number, 10 characters
    leg.document_form_serial_number = repeated
        .read_str_opt(Field::DocumentFormSerialNumber)?;

    // Item 18: Selectee Indicator, 1 character
    leg.selectee_indicator = repeated.read_char_opt(Field::SelecteeIndicator)?;

    // Item 108: International Document Verification, 1 character
    leg.international_document_verification =
        repeated.read_char_opt(Field::InternationalDocumentVerification)?;

    // Item 19: Marketing Carrier Designator, 3 characters
    leg.marketing_carrier = repeated.read_str_opt(Field::MarketingCarrier)?;

    // Item 20: Frequent Flyer Airline Designator, 3 characters
    leg.frequent_flyer_airline = repeated
        .read_str_opt(Field::FrequentFlyerAirline)?;

    // Item 236: Frequent Flyer Number, 16 characters
    leg.frequent_flyer_number = repeated
        .read_str_opt(Field::FrequentFlyerNumber)?;

    // Item 89: ID/AD Indicator, 1 character
    leg.id_ad_indicator = repeated.read_char_opt(Field::IdAdIndicator)?;

    // Item 118: Free Baggage Allowance, 3 characters
    leg.free_baggage_allowance = repeated
        .read_str_opt(Field::FreeBaggageAllowance)?;

    // Item 254: Fast Track, 1 character
    leg.fast_track = repeated.read_char_opt(Field::FastTrack)?;

    // Any remaining text is ascribed to airline use.
    if span.remaining() > 0 {
        let len = span.remaining();
        // Item 4: Airline Individual Use
        let body = span.read_str_len(Field::AirlineIndividualUse, len)?;
        leg.airline_individual_use = Some(body);
    }
    Ok(())
}

fn decode_security_data(cursor: &mut Cursor<'_>) -> BcbpResult<SecurityData> {
    // Item 25: Beginning of Security Data, 1 character, '^'.
    let prefix = cursor.read_char(Field::BeginningOfSecurityData)?;

    if prefix != '^' {
        return Err(Error::InvalidPrefix(Field::BeginningOfSecurityData, prefix));
    }

    // Item 28: Type of Security Data, 1 character, vendor specific.
    let kind = cursor.read_char(Field::TypeOfSecurityData)?;
    let mut data = SecurityData {
        kind,
        ..Default::default()
    };

    // Scan the length of the security data.
    if cursor.remaining() > 0 {
        // Item 29: Length of Security Data, 2 characters, numeric, right justified, zero filled.
        let len = cursor.read_usize(Field::LengthOfSecurityData, 16)?;
        if len > 0 {
            // Item 30: Security Data, up to 512 characters, vendor specific.
            let body = cursor.read_str_len(Field::SecurityData, len)?;
            data.data = Some(body.into());
        }
    }

    Ok(data)
}
