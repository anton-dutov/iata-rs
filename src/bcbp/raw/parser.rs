// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

use crate::bcbp::{
    field::Field,
    chunk::Chunk,
    error::{Error, Result},
    raw::{Bcbp, Leg, SecurityData},
};

/// Parses a boarding pass from `input_data` representable as a string reference.
pub fn from_str<'a>(input: &'a str) -> Result<Bcbp<'a>> {

    if !input.is_ascii() {
        return Err(Error::InvalidCharacters);
    }

    let mut chunk = Chunk::new(input);

    let code = chunk.fetch_char(Field::FormatCode)?;

    if code != 'M' {
        return Err(Error::InvalidFormatCode(code))
    }

    // The number of legs informs the breakdown of the various field iterators.
    let leg_count = chunk.fetch_usize(Field::LegsCount, 10)?;

    // Create a parser for the mandatory unique fields.
    let mut bcbp = Bcbp::default();

    bcbp.pax_name = chunk.fetch_str(Field::PaxName)?;
    bcbp.eticket_flag =
        chunk.fetch_char(Field::ETicketIndicator)?;

    for leg_index in 0..leg_count {

        // Mandatory fields common to all legs.
        let mut leg = Leg {

            pnr: chunk.fetch_str(Field::OperatingAirlinePnr)?,
            src_airport: chunk.fetch_str(Field::FromCityAirportCode)?,
            dst_airport: chunk.fetch_str(Field::ToCityAirportCode)?,
            operating_airline: chunk.fetch_str(Field::OperatingAirline)?,
            flight_number: chunk.fetch_str(Field::FlightNumber)?,
            flight_day: chunk.fetch_str(Field::DateOfFlight)?,
            compartment: chunk.fetch_char(Field::CompartmentCode)?,
            seat: chunk.fetch_str(Field::SeatNumber)?,
            checkin_sequence: chunk.fetch_str(Field::CheckInSequence)?,
            pax_status: chunk.fetch_char(Field::PaxStatus)?,

            .. Default::default()
        };

        // Field size of the variable size field that follows for the leg.
        let conditional_item_size =
            chunk.fetch_usize(Field::VariableBlockSize, 16)?;
        if conditional_item_size > 0 {
            // chunk over the entire set of conditional fields.
            let mut conditional_item_chunk = chunk.fetch_chunk(conditional_item_size)?;

            // The first leg may contain some optional fields at the root level.
            if leg_index == 0 {
                // Validate the beginning of version number tag as a sanity check.
                let prefix = conditional_item_chunk.fetch_char(Field::VersionBegin)?;
                if prefix != '<' && prefix != '>' {
                    return Err(Error::InvalidPrefix(Field::VersionBegin, prefix))
                }

                // The version number is part of the structure and must be consumed, but is not used.
                if conditional_item_chunk.len() > 0 {
                    let _ = conditional_item_chunk.fetch_str(Field::Version)?;
                }

                // Conditional unique fields are embedded in their own variable-length wrapper.
                if conditional_item_chunk.len() > 0 {
                    let len = conditional_item_chunk
                        .fetch_usize(Field::UniqueBlockSize, 16)?;
                    if len > 0 {
                        let mut unique_chunk = conditional_item_chunk.fetch_chunk(len)?;

                        bcbp.pax_description =
                            unique_chunk.fetch_char_opt(Field::PaxDescription)?;
                        bcbp.checkin_src =
                            unique_chunk.fetch_char_opt(Field::CheckInSrc)?;
                        bcbp.boardingpass_issue_src = unique_chunk
                            .fetch_char_opt(Field::BoardingPassIssueSrc)?;
                        bcbp.boardingpass_issue_date = unique_chunk
                            .fetch_str_opt(Field::BoardingPassIssueDate)?
                            .map(Into::into);
                        bcbp.doc_type =
                            unique_chunk.fetch_char_opt(Field::DocumentType)?;
                        bcbp.boardingpass_issue_airline = unique_chunk
                            .fetch_str_opt(Field::BoardingPassIssueAirline)?
                            .map(Into::into);
                        bcbp.bagtags = unique_chunk
                            .fetch_str_opt(Field::BagTags)?
                            .map(Into::into);
                        bcbp.bagtags_nc1 =unique_chunk
                            .fetch_str_opt(Field::BagTagsNc1)?
                            .map(Into::into);
                        bcbp.bagtags_nc2 =
                            unique_chunk
                                .fetch_str_opt(
                                    Field::BagTagsNc2,
                                )?
                                .map(Into::into);
                    }
                }
            }

            // Conditional fields common to all legs.
            if conditional_item_chunk.len() > 0 {
                let len = conditional_item_chunk
                    .fetch_usize(Field::RepeatedBlockSize, 16)?;
                if len > 0 {
                    let mut repeated_chunk = conditional_item_chunk.fetch_chunk(len)?;

                    leg.airline_numeric_code = repeated_chunk
                        .fetch_str_opt(Field::AirlineNumericCode)?
                        .map(Into::into);
                    leg.doc_number = repeated_chunk
                        .fetch_str_opt(Field::DocumentFormSerialNumber)?
                        .map(Into::into);
                    leg.selectee_indicator =
                        repeated_chunk.fetch_char_opt(Field::SelecteeIndicator)?;
                    leg.international_document_verification = repeated_chunk
                        .fetch_char_opt(Field::InternationalDocumentVerification)?;
                    leg.marketing_airline = repeated_chunk
                        .fetch_str_opt(Field::MarketingAirline)?
                        .map(Into::into);
                    leg.frequent_flyer_airline = repeated_chunk
                        .fetch_str_opt(Field::FrequentFlyerAirline)?
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

        let prefix = chunk.fetch_char(Field::SecurityDataBegin)?;
        if prefix != '^' {
            return Err(Error::InvalidPrefix(Field::SecurityDataBegin, prefix))
        }

        let mut security_data = SecurityData {
            // The security data type captured as a separate field set as the next field, data length, is discarded.
            kind: chunk.fetch_char_opt(Field::SecurityDataKind)?,

            .. Default::default()
        };

        // Scan the length of the security data.
        if chunk.len() > 0 {
            let len = chunk.fetch_usize(Field::SecurityDataLen, 16)?;
            if len > 0 {
                let body = chunk.fetch_str_len(Field::SecurityData, len as usize)?;
                security_data.data = Some(body.into());
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
