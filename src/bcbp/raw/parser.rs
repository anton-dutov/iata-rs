// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

use crate::bcbp::{
    field::Field,
    chunk::Chunk,
    error::{Error, BcbpResult},
    raw::{Bcbp, Leg, SecurityData},
};

/// Parses a boarding pass from `input_data` representable as a string reference.
pub fn from_str<'a>(input: &'_ str) -> BcbpResult<Bcbp<'_>> {

    if !input.is_ascii() {
        return Err(Error::InvalidCharacters);
    }

    let mut chunk = Chunk::new(input);

    let code = chunk.fetch_char(Field::FormatCode)?;

    if code != 'M' {
        return Err(Error::InvalidFormatCode(code))
    }

    // The number of legs informs the breakdown of the various field iterators.
    let leg_count = chunk.fetch_usize(Field::NumberOfLegsEncoded, 10)?;

    // Create a parser for the mandatory unique fields.
    let mut bcbp = Bcbp::default();

    bcbp.pax_name = chunk.fetch_str(Field::PassengerName)?;
    bcbp.eticket_flag =
        chunk.fetch_char(Field::ElectronicTicketIndicator)?;

    for leg_index in 0..leg_count {
        let mut leg = Leg::default();

        // Mandatory fields common to all legs.
        leg.pnr = chunk.fetch_str(Field::OperatingCarrierPnrCode)?;
        leg.src_airport = chunk.fetch_str(Field::FromCityAirportCode)?;
        leg.dst_airport = chunk.fetch_str(Field::ToCityAirportCode)?;
        leg.airline     = chunk.fetch_str(Field::OperatingCarrierDesignator)?;
        leg.flight_number = chunk.fetch_str(Field::FlightNumber)?;
        leg.flight_day    = chunk.fetch_str(Field::DateOfFlight)?;
        leg.compartment   = chunk.fetch_char(Field::CompartmentCode)?;
        leg.seat          = chunk.fetch_str(Field::SeatNumber)?;
        leg.checkin_sequence = chunk
            .fetch_str(Field::CheckInSequenceNumber)?;
        leg.pax_status = chunk.fetch_char(Field::PassengerStatus)?;

        // Field size of the variable size field that follows for the leg.
        let conditional_item_size =
            chunk.fetch_usize(Field::FieldSizeOfVariableSizeField, 16)?;
        if conditional_item_size > 0 {
            // chunk over the entire set of conditional fields.
            let mut conditional_item_chunk = chunk.fetch_chunk(conditional_item_size)?;

            // The first leg may contain some optional fields at the root level.
            if leg_index == 0 {
                // Validate the beginning of version number tag as a sanity check.
                let prefix = conditional_item_chunk.fetch_char(Field::BeginningOfVersionNumber)?;
                if prefix != '<' && prefix != '>' {
                    return Err(Error::InvalidPrefix(Field::BeginningOfVersionNumber, prefix))
                }

                // The version number is part of the structure and must be consumed, but is not used.
                if conditional_item_chunk.len() > 0 {
                    let _ = conditional_item_chunk.fetch_str(Field::VersionNumber)?;
                }

                // Conditional unique fields are embedded in their own variable-length wrapper.
                if conditional_item_chunk.len() > 0 {
                    let len = conditional_item_chunk
                        .fetch_usize(Field::FieldSizeOfStructuredMessageUnique, 16)?;
                    if len > 0 {
                        let mut unique_chunk = conditional_item_chunk.fetch_chunk(len)?;

                        bcbp.pax_description =
                            unique_chunk.fetch_char_opt(Field::PassengerDescription)?;
                        bcbp.source_of_check_in =
                            unique_chunk.fetch_char_opt(Field::SourceOfCheckIn)?;
                        bcbp.source_of_boarding_pass_issuance = unique_chunk
                            .fetch_char_opt(Field::SourceOfBoardingPassIssuance)?;
                        bcbp.date_of_issue_of_boarding_pass = unique_chunk
                            .fetch_str_opt(Field::DateOfIssueOfBoardingPass)?
                            .map(Into::into);
                        bcbp.doc_type =
                            unique_chunk.fetch_char_opt(Field::DocumentType)?;
                        bcbp.airline_designator_of_boarding_pass_issuer = unique_chunk
                            .fetch_str_opt(Field::AirlineDesignatorOfBoardingPassIssuer)?
                            .map(Into::into);
                        bcbp.baggage_tag_license_plate_numbers = unique_chunk
                            .fetch_str_opt(Field::BaggageTagLicensePlateNumbers)?
                            .map(Into::into);
                        bcbp.first_non_consecutive_baggage_tag_license_plate_numbers =
                            unique_chunk
                                .fetch_str_opt(
                                    Field::FirstNonConsecutiveBaggageTagLicensePlateNumbers,
                                )?
                                .map(Into::into);
                        bcbp.second_non_consecutive_baggage_tag_license_plate_numbers =
                            unique_chunk
                                .fetch_str_opt(
                                    Field::SecondNonConsecutiveBaggageTagLicensePlateNumbers,
                                )?
                                .map(Into::into);
                    }
                }
            }

            // Conditional fields common to all legs.
            if conditional_item_chunk.len() > 0 {
                let len = conditional_item_chunk
                    .fetch_usize(Field::FieldSizeOfStructuredMessageRepeated, 16)?;
                if len > 0 {
                    let mut repeated_chunk = conditional_item_chunk.fetch_chunk(len)?;

                    leg.airline_numeric_code = repeated_chunk
                        .fetch_str_opt(Field::AirlineNumericCode)?
                        .map(Into::into);
                    leg.document_form_serial_number = repeated_chunk
                        .fetch_str_opt(Field::DocumentFormSerialNumber)?
                        .map(Into::into);
                    leg.selectee_indicator =
                        repeated_chunk.fetch_char_opt(Field::SelecteeIndicator)?;
                    leg.international_document_verification = repeated_chunk
                        .fetch_char_opt(Field::InternationalDocumentVerification)?;
                    leg.marketing_carrier_designator = repeated_chunk
                        .fetch_str_opt(Field::MarketingCarrierDesignator)?
                        .map(Into::into);
                    leg.frequent_flyer_airline = repeated_chunk
                        .fetch_str_opt(Field::FrequentFlyerAirlineDesignator)?
                        .map(Into::into);
                    leg.frequent_flyer_number = repeated_chunk
                        .fetch_str_opt(Field::FrequentFlyerNumber)?
                        .map(Into::into);
                    leg.id_ad_indicator =
                        repeated_chunk.fetch_char_opt(Field::IdAdIndicator)?;
                    leg.free_baggage_allowance = repeated_chunk
                        .fetch_str_opt(Field::FreeBaggageAllowance)?
                        .map(Into::into);
                    leg.fast_track = repeated_chunk.fetch_char_opt(Field::FastTrack)?;
                }
            }

            // Any remaining text is ascribed to airline use.
            if conditional_item_chunk.len() > 0 {
                let len = conditional_item_chunk.len();
                let body = conditional_item_chunk.fetch_str_len(Field::AirlineIndividualUse, len)?;
                leg.airline_individual_use = Some(body);
            }
        }

        bcbp.legs.push(leg);
    }

    // Remaining input is ascribed to Security Data.
    if chunk.len() > 0 {

        let prefix = chunk.fetch_char(Field::BeginningOfSecurityData)?;
        if prefix != '^' {
            return Err(Error::InvalidPrefix(Field::BeginningOfSecurityData, prefix))
        }

        let mut security_data = SecurityData::default();

        // The security data type captured as a separate field set as the next field, data length, is discarded.
        security_data.type_of_security_data =
            chunk.fetch_char_opt(Field::TypeOfSecurityData)?;

        // Scan the length of the security data.
        if chunk.len() > 0 {
            let len = chunk.fetch_usize(Field::LengthOfSecurityData, 16)?;
            if len > 0 {
                let body = chunk.fetch_str_len(Field::SecurityData, len as usize)?;
                security_data.security_data = Some(body.into());
            }
        }

        bcbp.security_data = security_data;
    }

    if !chunk.eof() {
        Err(Error::TrailingData)
    } else {
        Ok(bcbp)
    }
}
