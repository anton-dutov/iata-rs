pub(crate) mod cursor;

use self::cursor::Cursor;
use crate::bcbp::format::Field;

use crate::{
    bcbp::{
        self,
        error::{BcbpResult, Error},
        Bcbp, Leg, SecurityData,
    },
    datetime::DayOfYear,
};

pub fn decode_bcbp(src: &str) -> BcbpResult<Bcbp> {
    // let src = src_data.as_ref();

    if !src.is_ascii() {
        return Err(Error::InvalidCharacters);
    }

    let mut cursor = Cursor::new(src);

    let src = src.to_uppercase();

    if src.len() < 60 {
        return Err(Error::MandatoryDataSize);
    }

    // Item 1: Format Code, 1 character, M for standard IATA BCBP.
    let code = cursor.read_char(Field::FormatCode)?;

    if code != 'M' {
        return Err(Error::InvalidFormatCode(code));
    }

    // Item 5: Number of legs encoded, 1 character, 1-9.
    let legs_count = cursor.read_u8(Field::NumberOfLegsEncoded, 10)?;

    if !(1..=9).contains(&legs_count) {
        return Err(Error::InvalidLegsCount);
    }

    let mut bcbp = Bcbp::default();

    // Item 11: Passenger Name, 20 characters, left justified, space filled.
    bcbp.set_passenger_name(cursor.read_str(Field::PassengerName)?)?;

    // Item 253: Electronic Ticket Indicator, 1 character, 'E' for electronic ticket, blank for none.
    bcbp.set_eticket_indicator(blank_or_into(
        cursor.read_char(Field::ElectronicTicketIndicator)?,
    ))?;

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
    bcbp.security_data = decode_security_data(&mut cursor)?;

    if !cursor.is_eof() {
        Err(Error::TrailingData)
    } else {
        Ok(bcbp)
    }
}

fn decode_conditional(cursor: &mut Cursor, bcbp: &mut Bcbp, leg: &mut Leg) -> BcbpResult<()> {
    // Item 6: Field Size of Variable Size Field, 2 characters, numeric, right justified, zero filled.
    let cond_size = cursor.read_usize(Field::FieldSizeOfVariableSizeField, 16)?;

    if cond_size == 0 {
        return Ok(());
    }

    if cond_size > cursor.remaining() {
        return Err(Error::ConditionalDataSize);
    }

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
    bcbp.set_version(blank_or_try_into(
        cond_data.read_char(Field::VersionNumber)?,
    )?)?;

    // Conditional unique fields are embedded in their own variable-length wrapper.
    if cond_data.remaining() > 0 {
        // Item 10: Field Size of Structured Message Unique, 2 characters
        let len = cond_data.read_usize(Field::FieldSizeOfStructuredMessageUnique, 16)?;
        if len > 0 {
            let mut unique = cond_data.read_chunk(len)?;

            // Item 15: Passenger Description, 1 character
            bcbp.set_pax_kind(opt_blank_or_into(
                unique.read_char_opt(Field::PassengerDescription)?,
            ))?;
            // Item 12: Source of Check-In, 1 character
            bcbp.set_checkin_src(unique.read_char_opt(Field::SourceOfCheckIn)?)?;
            // Item 14: Source of Boarding Pass Issuance, 1 character
            bcbp.set_boardingpass_src(unique.read_char_opt(Field::SourceOfBoardingPassIssuance)?)?;
            // Item 22: Date of Issue of Boarding Pass, 4 characters
            bcbp.set_boardingpass_issued(
                unique
                    .read_str_opt(Field::DateOfIssueOfBoardingPass)?
                    .map(|x| u16_from_str_force(x, 10)),
            )?;
            // Item 16: Document Type, 1 character
            bcbp.set_doc_type(unique.read_char_opt(Field::DocumentType)?)?;
            // Item 21: Airline Designator of Boarding Pass Issuer, 3 characters
            bcbp.set_boardingpass_airline(Some(
                unique
                    .read_str_opt(Field::AirlineDesignatorOfBoardingPassIssuer)?
                    .unwrap_or(""),
            ))?;

            // Item 23: Baggage Tag License Plate, 13 characters
            bcbp.set_bagtags(
                unique
                    .read_str_opt(Field::BaggageTagNumbers)?
                    .map(|x| x.trim().into()),
            )?;

            // Item 31: First Non-Consecutive Baggage Tag License Plate, 13 characters
            bcbp.set_nonconsecutive_bagtag1(
                unique
                    .read_str_opt(Field::FirstNonConsecutiveBaggageTagNumbers)?
                    .map(|x| x.trim().into()),
            )?;

            // Item 32: Second Non-Consecutive Baggage Tag License Plate, 13 characters
            bcbp.set_nonconsecutive_bagtag2(
                unique
                    .read_str_opt(Field::SecondNonConsecutiveBaggageTagNumbers)?
                    .map(|x| x.trim().into()),
            )?;
        }
    }

    decode_leg_conditional(&mut cond_data, leg)?;

    Ok(())
}

fn decode_leg(cursor: &mut Cursor) -> BcbpResult<Leg> {
    let mut leg = Leg::default();

    // Item 7: Operating Carrier PNR Code, 7 characters, alphanumeric, left justified, blank filled.
    leg.set_pnr(Some(cursor.read_str(Field::OperatingCarrierPnrCode)?))?;

    // Item 26: From City Airport Code, 3 characters, alphabetic.
    leg.set_src_airport(Some(cursor.read_str(Field::FromCityAirportCode)?))?;

    // Item 38: To City Airport Code, 3 characters, alphabetic.
    leg.set_dst_airport(Some(cursor.read_str(Field::ToCityAirportCode)?))?;

    // Item 42: Operating Carrier Designator
    leg.set_operating_carrier(Some(cursor.read_str(Field::OperatingCarrierDesignator)?))?;

    // Item 43: Flight Number, 5 characters
    leg.set_flight_number(Some(cursor.read_str(Field::FlightNumber)?))?;

    // Item 46: Date of Flight, 3 characters, numeric
    let flight_day = cursor.read_str(Field::DateOfFlight)?;
    leg.set_flight_day(if !flight_day.trim().is_empty() {
        Some(DayOfYear::new(u16_from_str_force(flight_day, 10)).unwrap())
    } else {
        None
    });

    // Item 71: Compartment Code, 1 character, alphabetic.
    leg.set_compartment(blank_or_into(cursor.read_char(Field::CompartmentCode)?))?;

    // Item 104: Seat Number. 4 bytes. Usually 'NNNa', but can be 'INF ' or similar.
    leg.set_seat(Some(cursor.read_str(Field::SeatNumber)?))?;

    // Item 107: Check-in Sequence Number, 5 characters, numeric, right justified, zero filled.
    leg.set_checkin_sequence(u32_from_str_opt(
        cursor.read_str(Field::CheckInSequenceNumber)?,
        10,
    ))?;

    // Item 113: Passenger Status. 1 byte. Format 'f'.
    leg.set_pax_status(cursor.read_char(Field::PassengerStatus)?.into());

    Ok(leg)
}

fn decode_leg_conditional(span: &mut Cursor, leg: &mut Leg) -> BcbpResult<()> {
    let len = span.read_usize(Field::FieldSizeOfStructuredMessageRepeated, 16)?;
    if len == 0 {
        return Ok(());
    }
    let mut repeated = span.read_chunk(len)?;

    // Item 142: Airline Numeric Code, 3 characters
    leg.set_airline_num(
        repeated
            .read_str_opt(Field::AirlineNumericCode)?
            .map(|x| u16_from_str_force(x.trim(), 10)),
    )?;

    // Item 143: Document Form/Serial Number, 10 characters
    leg.set_doc_number(Some(
        repeated
            .read_str_opt(Field::DocumentFormSerialNumber)?
            .unwrap_or(""),
    ))?;

    // Item 18: Selectee Indicator, 1 character
    leg.set_selectee_indicator(opt_blank_or_into(
        repeated.read_char_opt(Field::SelecteeIndicator)?,
    ))?;

    // Item 108: International Document Verification, 1 character
    leg.set_doc_int_verification(
        repeated.read_char_opt(Field::InternationalDocumentVerification)?,
    )?;

    // Item 19: Marketing Carrier Designator, 3 characters
    leg.set_marketing_airline(Some(
        repeated
            .read_str_opt(Field::MarketingCarrierDesignator)?
            .unwrap_or(""),
    ))?;

    // Item 20: Frequent Flyer Airline Designator, 3 characters
    leg.set_freq_flyer_airline(Some(
        repeated
            .read_str_opt(Field::FrequentFlyerAirlineDesignator)?
            .unwrap_or(""),
    ))?;

    // Item 236: Frequent Flyer Number, 16 characters
    leg.set_freq_flyer_number(Some(
        repeated
            .read_str_opt(Field::FrequentFlyerNumber)?
            .unwrap_or(""),
    ))?;

    // Item 89: ID/AD Indicator, 1 character
    leg.set_id_ad_indicator(repeated.read_char_opt(Field::IdAdIndicator)?)?;

    // Item 118: Free Baggage Allowance, 3 characters
    leg.set_bag_allowance(Some(
        repeated
            .read_str_opt(Field::FreeBaggageAllowance)?
            .unwrap_or(""),
    ))?;

    // Item 254: Fast Track, 1 character
    leg.set_fast_track(opt_blank_or_into(repeated.read_char_opt(Field::FastTrack)?))?;

    // Any remaining text is ascribed to airline use.
    if span.remaining() > 0 {
        let len = span.remaining();
        // Item 4: Airline Individual Use
        let body = span.read_str_len(Field::AirlineIndividualUse, len)?;
        leg.set_variable_data(Some(body.into()))?;
    }
    Ok(())
}

fn decode_security_data(input: &mut Cursor) -> BcbpResult<Option<SecurityData>> {
    if input.remaining() == 0 {
        return Ok(None);
    }

    // Item 25: Beginning of Security Data, 1 character, '^'.
    let prefix = input.read_char(Field::BeginningOfSecurityData)?;
    if prefix != '^' {
        return Err(Error::InvalidPrefix(Field::BeginningOfSecurityData, prefix));
    }

    // Item 28: Type of Security Data, 1 character, vendor specific.
    let kind = input.read_char(Field::TypeOfSecurityData)?;

    // Item 29: Length of Security Data, 2 characters, numeric, right justified, zero filled.
    let size = input.read_usize(Field::LengthOfSecurityData, 16)?;

    // Item 30: Security Data, up to 512 characters, vendor specific.
    let data = input.read_str_len(Field::SecurityData, size)?;

    Ok(Some(SecurityData {
        kind,
        data: data.into(),
    }))
}

fn u16_from_str_force(src: &str, radix: u32) -> u16 {
    match u16::from_str_radix(src.trim().trim_start_matches('0'), radix) {
        Ok(v) => v,
        _ => 0,
    }
}

fn u32_from_str_opt(src: &str, radix: u32) -> Option<u16> {
    u16::from_str_radix(src.trim().trim_start_matches('0'), radix).ok()
}

fn blank_or_try_into<T: TryFrom<char>>(value: char) -> BcbpResult<Option<T>>
where
    bcbp::error::Error: From<<T as TryFrom<char>>::Error>,
{
    let value = match value {
        ' ' => None,
        c => Some(c.try_into()?),
    };

    Ok(value)
}

fn blank_or_into<T: From<char>>(value: char) -> Option<T> {
    match value {
        ' ' => None,
        c => Some(c.into()),
    }
}

fn opt_blank_or_into<T: From<char>>(value: Option<char>) -> Option<T> {
    match value {
        None => None,
        Some(' ') => None,
        Some(c) => Some(c.into()),
    }
}
