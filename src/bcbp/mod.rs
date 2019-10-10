use std::str;
use std::u32;

pub use chrono::prelude::*;

pub mod error;
pub mod field;
pub mod raw;
pub(crate) mod chunk;

use chunk::Chunk;
use field::Field;

pub use crate::bcbp::error::{
    Error,
    FixError
};

#[derive(Debug, Clone, PartialEq)]
pub enum PaxStatus {
    None,
    Other(char),
}

impl Default for PaxStatus {
    fn default() -> Self { PaxStatus::None }
}


#[derive(Debug, Clone, PartialEq)]
pub enum PaxType {
    None,            // ' '
    Adult,           // 0
    Male,            // 1
    Female,          // 2
    Child,           // 3
    Infant,          // 4
    CabinBaggage,    // 5
    AdultWithInfant, // 6
    Other(char),
}

impl Default for PaxType {
    fn default() -> Self { PaxType::None }
}

impl PaxType {
    pub fn from_char(t: char) -> PaxType {
        use PaxType::*;
        match t {
            ' ' => None,
            '0' => Adult,
            '1' => Male,
            '2' => Female,
            '3' => Child,
            '4' => Infant,
            '6' => CabinBaggage,
            '7' => AdultWithInfant,
            _   => Other(t)
        }
    }
}


#[derive(Debug, Default, Clone)]
pub struct Leg {
    pnr: String,
    src_airport: String,
    dst_airport: String,
    airline: String,
    flight_number: String,
    pub flight_day: u16,
    pub compartment: char,
    seat: Option<String>,
    pub airline_num: Option<u16>,
    pub sequence: Option<u16>,
    pub pax_status: char,
    pub doc_number: Option<String>,
    // Selectee
    // marketing_airline
    marketing_airline: Option<String>,
    frequent_flyer_airline: Option<String>,
    frequent_flyer_number: Option<String>,
    fast_track: Option<char>,
    // ID/AD indicator
    bag_allowance: Option<String>,
    // data
    var: Option<String>,
}

impl Leg {
    pub fn pnr(&self) -> &str {
        self.pnr.as_ref()
    }

    pub fn pnr_set<S>(&mut self, pnr: S) where S: Into<String> {
        self.pnr = pnr.into();
    }

    pub fn airline(&self) -> &str {
        self.airline.as_ref()
    }

    pub fn airline_mut(&mut self) -> &mut String {
        &mut self.airline
    }

    pub fn src_airport(&self) -> &str {
        self.src_airport.as_ref()
    }

    pub fn src_airport_mut(&mut self) -> &mut String {
        &mut self.src_airport
    }

    pub fn dst_airport(&self) -> &str {
        self.dst_airport.as_ref()
    }

    pub fn dst_airport_mut(&mut self) -> &mut String {
        &mut self.dst_airport
    }

    pub fn flight_number(&self) -> &str {
        self.flight_number.as_ref()
    }

    pub fn flight_number_set<S>(&mut self, code: S) where S: Into<String> {
        self.flight_number = code.into();
    }

    pub fn flight_date(&self, year: i32) -> NaiveDate {

        let day = if self.flight_day > 0 && self.flight_day < 366 { self.flight_day } else { 1 };

        NaiveDate::from_yo(year, u32::from(day))
    }

    pub fn flight_date_current_year(&self) -> NaiveDate {
        let now = Utc::today();

        self.flight_date(now.year())
    }

    pub fn flight_day_aligned(&self) -> String {
        if self.flight_day == 0 {
            return String::new()
        }
        format!("{:0>3}", self.flight_day)
    }

    pub fn flight_date_set(&mut self, date: NaiveDate) {
        self.flight_day = date.ordinal() as u16;
    }

    pub fn seat(&self) -> Option<&str> {
        self.seat.as_ref().map(String::as_str)
    }

    pub fn seat_set<S>(&mut self, seat: S) where S: Into<String>{
        self.seat = Some(seat.into());
    }

    pub fn seat_aligned(&self) -> String {

        if let Some(ref seat) = self.seat {
            format!("{:0>4}", seat)
        } else {
            String::new()
        }
    }

    pub fn sequence_aligned(&self) -> String {
        if let Some(seq) = self.sequence {
            format!("{:0>4}", seq)
        } else {
            String::new()
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct BCBP {
    pub version: Option<char>,
    pub pax_type: PaxType,
    pub doc_type: Option<char>,
    name_first: String,
    name_last: String,
    pub ticket_flag: char,
    pub legs: Vec<Leg>,
    pub bagtags: Vec<String>,
    pub checkin_src: Option<char>,
    pub boardingpass_src: Option<char>,
    pub boardingpass_issued: Option<u16>,
    boardingpass_airline: Option<String>,
    pub security_data_type: Option<char>,
    security_data: Option<String>,
}

impl BCBP {
    pub fn name(&self) -> String {
        let mut tmp = format!("{}/{}", self.name_last, self.name_first);
        tmp.truncate(20);
        tmp
    }

    pub fn name_last(&self) -> &str {
        self.name_last.as_ref()
    }

    pub fn name_last_mut(&mut self) -> &mut String {
        &mut self.name_last
    }

    pub fn name_first(&self) -> &str {
        self.name_first.as_ref()
    }

    pub fn name_first_mut(&mut self) -> &mut String {
        &mut self.name_first
    }

    pub fn legs_count(&self) -> u8 {
        let mut cnt = self.legs.len();
        if cnt > 9 {
            cnt = 9;
        }
        cnt as u8
    }

    pub fn legs(&self) -> &Vec<Leg> {
        &self.legs
    }

    pub fn legs_mut(&mut self) -> &mut Vec<Leg> {
        &mut self.legs
    }

    pub fn boardingpass_airline(&self) -> Option<&String> {
        self.boardingpass_airline.as_ref()
    }

    pub fn pax_type(&self) -> PaxType {
        self.pax_type.clone()
    }

    pub fn build(&self) -> Result<String, String> {

        let mut ret = format!("M{}{:<20}{}", self.legs_count(), self.name(), self.ticket_flag);

        for s in &self.legs {
            ret = format!("{}{:<7}{:<3}{:<3}{:<3}{:<5}{:3}{:1}{:>4}{:<5}{:1}00",
                ret,
                s.pnr,
                s.src_airport,
                s.dst_airport,
                s.airline,
                s.flight_number,
                s.flight_day_aligned(),
                s.compartment,
                s.seat_aligned(),
                s.sequence_aligned(),
                s.pax_status);
        }
        Ok(ret)
    }

    pub fn from(src: &str) -> Result<BCBP, Error> {


        // let src = src_data.as_ref();

        if !src.is_ascii() {
            return Err(Error::InvalidCharacters);
        }

        let mut chunk = Chunk::new(src);

        let src = src.to_uppercase();

        if src.len() < 60 {
            return Err(Error::MandatoryDataSize)
        }

        let code = chunk.fetch_char(Field::FormatCode)?;

        if code != 'M' {
            return Err(Error::InvalidFormatCode(code))
        }

        // The number of legs informs the breakdown of the various field iterators.
        let legs = chunk.fetch_usize(Field::NumberOfLegsEncoded, 10)?;

        if legs < 1 || legs > 9 {
            return Err(Error::InvalidLegsCount)
        }

        let mut bcbp = BCBP::default();


        let name = chunk.fetch_str(Field::PassengerName)?;

        let (first, last) = bcbp_name(name);
        bcbp.name_last  = first;
        bcbp.name_first = last.unwrap_or_default().trim().into();

        bcbp.ticket_flag = chunk.fetch_char(Field::ElectronicTicketIndicator)?;

        for leg_index in 0 .. legs {

            let mut leg = Leg::default();

            // Mandatory fields common to all legs.
            leg.pnr = chunk
                .fetch_str(Field::OperatingCarrierPnrCode)?
                .trim()
                .into();
            leg.src_airport   = chunk.fetch_str(Field::FromCityAirportCode)?.trim().into();
            leg.dst_airport   = chunk.fetch_str(Field::ToCityAirportCode)?.trim().into();
            leg.airline       = chunk
                .fetch_str(Field::OperatingCarrierDesignator)?
                .trim().into();
            leg.flight_number = chunk.fetch_str(Field::FlightNumber)?.trim().into();
            leg.flight_day    = u16_from_str_force(chunk.fetch_str(Field::DateOfFlight)?, 10);
            leg.compartment   = chunk.fetch_char(Field::CompartmentCode)?;
            leg.seat          = seat_opt(chunk.fetch_str(Field::SeatNumber)?.trim());
            leg.sequence      = u32_from_str_opt(chunk
                .fetch_str(Field::CheckInSequenceNumber)?, 10);

            leg.pax_status    = chunk.fetch_char(Field::PassengerStatus)?;

            // Field size of the variable size field that follows for the leg.
            let conditional_size =
                chunk.fetch_usize(Field::FieldSizeOfVariableSizeField, 16)?;

            if conditional_size > chunk.len() {
                return Err(Error::CoditionalDataSize)
            }

            if conditional_size > 0 {

                // chunk over the entire set of conditional fields.
                let mut conditional_item = chunk.fetch_chunk(conditional_size)?;

                // The first leg may contain some optional fields at the root level.
                if leg_index == 0 {

                    // Validate the beginning of version number tag as a sanity check.
                    let prefix = conditional_item.fetch_char(Field::BeginningOfVersionNumber)?;
                    if prefix != '<' && prefix != '>' {
                        return Err(Error::InvalidPrefix(Field::BeginningOfVersionNumber, prefix))
                    }

                    bcbp.version = Some(conditional_item.fetch_char(Field::VersionNumber)?);

                    // Conditional unique fields are embedded in their own variable-length wrapper.
                    if conditional_item.len() > 0 {
                    let len = conditional_item
                        .fetch_usize(Field::FieldSizeOfStructuredMessageUnique, 16)?;
                    if len > 0 {
                        let mut unique_chunk = conditional_item.fetch_chunk(len)?;

                        bcbp.pax_type =
                            unique_chunk
                            .fetch_char_opt(Field::PassengerDescription)?
                            .map(PaxType::from_char).unwrap_or_default();
                        bcbp.checkin_src =
                            unique_chunk.fetch_char_opt(Field::SourceOfCheckIn)?;
                        bcbp.boardingpass_src = unique_chunk
                            .fetch_char_opt(Field::SourceOfBoardingPassIssuance)?;
                        bcbp.boardingpass_issued = unique_chunk
                            .fetch_str_opt(Field::DateOfIssueOfBoardingPass)?
                            .map(|x| u16_from_str_force(x, 10));
                        bcbp.doc_type = unique_chunk.fetch_char_opt(Field::DocumentType)?;
                        bcbp.boardingpass_airline = unique_chunk
                            .fetch_str_opt(Field::AirlineDesignatorOfBoardingPassIssuer)?
                            .map(|x| x.trim().into());

                        // let _ = unique_chunk
                        //     .fetch_str_opt(Field::BaggageTagLicensePlateNumbers)?
                        //     .map(|x| x.trim().into());
                        // let _ =
                        //     unique_chunk
                        //         .fetch_str_opt(
                        //             field::Field::FirstNonConsecutiveBaggageTagLicensePlateNumbers,
                        //         )?
                        //         .map(|x| x.trim().into());
                        // let _ =
                        //     unique_chunk
                        //         .fetch_str_opt(
                        //             field::Field::SecondNonConsecutiveBaggageTagLicensePlateNumbers,
                        //         )?
                        //        .map(|x| x.trim().into());
                    }
                }
            }

            // Conditional fields common to all legs.
            if conditional_item.len() > 0 {
                let len = conditional_item
                    .fetch_usize(Field::FieldSizeOfStructuredMessageRepeated, 16)?;
                if len > 0 {
                    let mut repeated_chunk = conditional_item.fetch_chunk(len)?;

                    leg.airline_num = repeated_chunk
                        .fetch_str_opt(Field::AirlineNumericCode)?
                        .map(|x| u16_from_str_force(x.trim(), 10));
                    leg.doc_number = repeated_chunk
                        .fetch_str_opt(Field::DocumentFormSerialNumber)?
                        .map(|x| x.trim().into());
                    let _selectee_indicator =
                        repeated_chunk.fetch_char_opt(Field::SelecteeIndicator)?;
                    let _international_document_verification = repeated_chunk
                        .fetch_char_opt(Field::InternationalDocumentVerification)?;
                    leg.marketing_airline = repeated_chunk
                        .fetch_str_opt(Field::MarketingCarrierDesignator)?
                        .map(|x| x.trim().into());
                    leg.frequent_flyer_airline = repeated_chunk
                        .fetch_str_opt(Field::FrequentFlyerAirlineDesignator)?
                        .map(|x| x.trim().into());
                    leg.frequent_flyer_number = repeated_chunk
                        .fetch_str_opt(Field::FrequentFlyerNumber)?
                        .map(Into::into);
                    let _id_ad_indicator =
                        repeated_chunk.fetch_char_opt(Field::IdAdIndicator)?;
                    leg.bag_allowance = repeated_chunk
                        .fetch_str_opt(Field::FreeBaggageAllowance)?
                        .map(|x| x.trim().into());
                    leg.fast_track = repeated_chunk.fetch_char_opt(Field::FastTrack)?;
                }
            }

            // Any remaining text is ascribed to airline use.
            if conditional_item.len() > 0 {
                let len  = conditional_item.len();
                let body = conditional_item
                    .fetch_str_len(Field::AirlineIndividualUse, len)?;
                leg.var = Some(body.into());
            }

            // match bcbp_leg(next_segment) {
            //     Ok((leg_rest, o))    => {
            //         let sz = usize::from_str_radix(o.1, 16).map_err(|_| Error::CoditionalDataSize)?;

            //         if sz > leg_rest.len() {
            //             return Err(Error::CoditionalDataSize)
            //         }

            //         let (first, last) = leg_rest.split_at(sz);

            //         bcbp.legs.push(o.0);

            //         next_segment  = last;

            //         let mut chunk = first;

            //         if sz == 0 {
            //             continue;
            //         }

            //         // Extended: Unique data
            //         if i == 0 {
            //             let tag = chunk[0..1].chars().next().unwrap_or_default();

            //             if tag != '<' && tag != '>' {
            //                 return Err(Error::InvalidVersionBegin(tag))
            //             }

            //             bcbp.version = Some(chunk[1..2].chars().next().unwrap_or_default());


            //             chunk =  &chunk[2..];

            //             // 1 size: take!(2) >>
            //             // 2 pax_type: opt!(complete!(anychar)) >>
            //             // 3 checkin_src: opt!(complete!(anychar)) >>
            //             // 4 boardingpass_src: opt!(complete!(anychar)) >>
            //             // 5 boardingpass_issued: opt!(complete!(take!(4))) >>
            //             // 6 doc_type: opt!(complete!(anychar)) >>
            //             // 7 boardingpass_airline: opt!(complete!(take!(3))) >>
            //             // 8 tags: opt!(complete!(take!(13))) >>

            //             match bcbp_ext_v1(chunk) {
            //                 Ok((_rest, o)) => {
            //                     // println!("U << {:?}", chunk);

            //                     let sz  = usize::from_str_radix(o.0, 16).map_err(|_| Error::CoditionalDataSize)?;
            //                     let split_pos = sz + 2;
            //                     if  split_pos > chunk.len() {
            //                         return Err(Error::CoditionalDataSize)
            //                     }

            //                     let (_first, last) = chunk.split_at(split_pos);
            //                     // println!("U [REST] {:?}", rest);
            //                     // println!("U [LAST] {:?}", last);

            //                     bcbp.pax_type    = o.1;
            //                     bcbp.checkin_src = o.2;
            //                     bcbp.boardingpass_src = o.3;
            //                     bcbp.boardingpass_issued = o.4.map(|x| u16_from_str_force(x, 10));
            //                     bcbp.doc_type = o.5;
            //                     bcbp.boardingpass_airline = o.6.map(|x| x.trim_end().to_owned());

            //                     if let Some(t) = o.7.map(str::trim) {
            //                         if !t.is_empty() {
            //                             bcbp.bagtags.push(t.to_owned());
            //                         }
            //                     }

            //                     chunk = last;

            //                     // println!("U >> {:?}", chunk);
            //                 },
            //                 _ => return Err(Error::CoditionalData)
            //             }
            //         }

            //         // Repeat data
            //         match bcbp.version {

            //             _ => match bcbp_v3(chunk) {
            //                 Ok((_rest, o)) => {
            //                     // println!("S << {:?}", chunk);


            //                     // println!("S [REST] {:?}", rest);


            //                     // println!("{:#?}", o);

            //                     let sz = usize::from_str_radix(o.0, 16).map_err(|_| Error::CoditionalDataSize)?;
            //                     let split_pos = sz + 2;
            //                     if  split_pos > chunk.len() {
            //                         return Err(Error::CoditionalDataSize)
            //                     }

            //                     let (_first, last) = chunk.split_at(split_pos);

            //                     bcbp.legs[i].airline_num  = o.1.map(|x| u16_from_str_force(x, 10));
            //                     bcbp.legs[i].doc_num = o.2.map(String::from);
            //                     bcbp.legs[i].ff_airline  = o.6.map(String::from);
            //                     bcbp.legs[i].ff_number   = o.7.map(String::from);
            //     //                 bcbp.legs[i].var         = Some(last.to_owned());

            //                     chunk = last;

            //                     // println!("S >> {:?}", chunk);

            //                 },
            //                 _ => return Err(Error::CoditionalData)
            //             }
            //         }
            //     },
            //     Err(e) => {
            //         if e.is_incomplete() {
            //             return Err(Error::InsufficientDataLength)
            //         }
            //         println!("{:?}", e);
            //     }
            }

            bcbp.legs.push(leg);
        }

        // Remaining input is ascribed to Security Data.
        if chunk.len() > 0 {

            let prefix = chunk.fetch_char(Field::BeginningOfSecurityData)?;
            if prefix != '^' {
                return Err(Error::InvalidPrefix(Field::BeginningOfSecurityData, prefix))
            }

            // The security data type captured as a separate field set as the next field, data length, is discarded.
            bcbp.security_data_type = chunk.fetch_char_opt(Field::TypeOfSecurityData)?;

            // Scan the length of the security data.
            if chunk.len() > 0 {
                let len = chunk.fetch_usize(Field::LengthOfSecurityData, 16)?;
                if len > 0 {
                    let body = chunk.fetch_str_len(Field::SecurityData, len)?;
                    bcbp.security_data = Some(body.into());
                }
            }
        }

        if !chunk.eof() {
            Err(Error::TrailingData)
        } else {
            Ok(bcbp)
        }
    }
}

fn u16_from_str_force(src: &str, radix: u32) -> u16 {
    match u16::from_str_radix(src.trim().trim_start_matches('0'), radix) {
        Ok(v) => v,
        _     => 0,
    }
}

fn u32_from_str_opt(src: &str, radix: u32) -> Option<u16> {
    u16::from_str_radix(src.trim().trim_start_matches('0'), radix).ok()
}

fn seat_opt(seat: &str) -> Option<String> {
    let tmp = seat.trim().trim_start_matches('0').to_string();

    if tmp.len() <= 1 {
        None
    } else {
        Some(tmp)
    }
}


fn bcbp_name(input: &str) -> (String, Option<String>) {
    let last_start_idx = 0;
    let mut last_end_idx = 0;
    let mut first_start_idx = 0;
    let mut first_end_idx = 0;
    let mut have_first = false;

    // input is ASCII so we can do byte-wise indexing safely
    for (idx, c) in input.char_indices() {
        // If haven't consumed last name yet
        if !have_first && c == '/' {
            last_end_idx = idx;
            have_first = true;
            first_start_idx = idx + 1;
            first_end_idx = input.len();
        }
    }

    // if there is no first name, surname occupies whe whole name field
    if !have_first {
        last_end_idx = input.len();
    }

    // extract names
    let last = input[last_start_idx..last_end_idx].trim_end().to_string();
    let first = if have_first {
        let first = input[first_start_idx..first_end_idx].trim_end();
        if !first.is_empty() {
            Some(first.to_string())
        } else {
            None
        }
    } else {
        None
    };

    (last, first)
}

pub fn fix_length(src: &str) -> Result<String, FixError> {

    if src.len() < 60 {
        return Err(FixError::InsufficientDataLength)
    }

    let mut tmp = src.to_owned();

    // Minimal
    tmp.truncate(58);
    tmp.push_str("00");

    Ok(tmp)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_name(last: String, first: Option<String>, right_last: &str, right_first: Option<&str>) {
        assert_eq!(&last, right_last);
        assert_eq!(first, right_first.map(String::from));
    }

    #[test]
    fn test_bcbp_name() {
        let names = &[
            "BRUNER/ROMAN MR     ",
            "JOHN/SMITH JORDAN   ",
            "VERYLONGESTLASTNAMED",
            "JOHN/SMITH          ",
            "BRUNER ROMAN MR/    ",
        ];

        let answers = &[
            ("BRUNER", Some("ROMAN MR")),
            ("JOHN", Some("SMITH JORDAN")),
            ("VERYLONGESTLASTNAMED", None),
            ("JOHN", Some("SMITH")),
            ("BRUNER ROMAN MR", None)
        ];

        for i in 0..names.len() {

            let (last, first) = bcbp_name(names[i]);
            let (right_last, right_first) = answers[i];
            check_name(
                last,
                first,
                right_last,
                right_first
            )
        }
    }
}