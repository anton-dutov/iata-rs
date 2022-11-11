use std::str;
use std::u32;

pub use time::Date;

pub mod error;
pub mod field;
pub mod raw;
pub(crate) mod chunk;

use chunk::Chunk;
use field::Field;

pub use crate::bcbp::error::{
    Error,
    FixError,
    Result,
};


#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Tolerant,
    Strict
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PaxStatus {
    None,
    NotCheckedIn,
    CheckedIn,
    Other(char),
}

impl PaxStatus {
    pub fn from_char(t: char) -> Self {
        use PaxStatus::*;
        match t {
            ' ' => None,
            '0' => NotCheckedIn,
            '1' => CheckedIn,
            _   => Other(t)
        }
    }

    pub fn to_char(&self) -> char {
        use PaxStatus::*;
        match *self {
            None          => ' ',
            NotCheckedIn  => '0',
            CheckedIn     => '1',
            Other(t)      => t
        }

    }
}

impl Default for PaxStatus {
    fn default() -> Self { PaxStatus::NotCheckedIn }
}



#[derive(Debug, Clone, PartialEq, Eq)]
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

impl PaxType {
    pub fn from_char(t: char) -> Self {
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

impl Default for PaxType {
    fn default() -> Self { PaxType::None }
}




#[derive(Debug, Default, Clone)]
pub struct Leg {
    pub pnr: String,
    pub src_airport: String,
    pub dst_airport: String,
    pub operating_airline: String,
    pub flight_number: String,
    pub flight_day: u16,
    pub compartment: Option<char>,
    pub seat: Option<String>,
    pub airline_num: Option<u16>,
    pub checkin_sequence: Option<u16>,
    pub pax_status: PaxStatus,
    pub doc_number: Option<String>,
    // Selectee
    // marketing_airline
    pub marketing_airline: Option<String>,
    pub frequent_flyer_airline: Option<String>,
    pub frequent_flyer_number: Option<String>,
    pub fast_track: Option<char>,
    // ID/AD indicator
    pub bag_allowance: Option<String>,
    // data
    pub var: Option<String>,
}

impl Leg {

    pub fn flight_date(&self, year: i32) -> Result<Date> {

        let day = if self.flight_day > 0 && self.flight_day < 366 { self.flight_day } else { 1 };

        Date::from_ordinal_date(year, day)
            .map_err(Error::Time)
    }

    pub fn flight_day_aligned(&self) -> String {
        if self.flight_day == 0 {
            return String::new()
        }
        format!("{:0>3}", self.flight_day)
    }

    pub fn flight_date_set(&mut self, date: Date) {
        self.flight_day = date.ordinal() as u16;
    }
}

#[derive(Debug, Default, Clone)]
pub struct BCBP {
    pub version: Option<char>,
    pub pax_type: PaxType,
    pub doc_type: Option<char>,
    pub name_last: String,
    pub name_first: Option<String>,
    pub ticket_flag: Option<char>,
    pub legs: Vec<Leg>,
    pub bagtags: Vec<String>,
    pub checkin_src: Option<char>,
    pub boardingpass_src: Option<char>,
    pub boardingpass_issued: Option<u16>,
    pub boardingpass_airline: Option<String>,
    pub security_data_type: Option<char>,
    pub security_data: Option<String>,
}

impl BCBP {
    pub fn name(&self) -> String {
        let mut tmp = if let Some(ref name_first) = self.name_first {
            format!("{}/{}", self.name_last, name_first)
        } else {
            self.name_last.clone()
        };

        tmp.truncate(20);
        tmp
    }

    pub fn legs_count(&self) -> u8 {
        let mut cnt = self.legs.len();
        if cnt > 9 {
            cnt = 9;
        }
        cnt as u8
    }

    pub fn legs(&self) -> &[Leg] {
        &self.legs
    }

    pub fn legs_mut(&mut self) -> &mut Vec<Leg> {
        &mut self.legs
    }

    pub fn build(&self, _mode: Mode) -> Result<String> {


        let mut ret = format!("M{}{:<20}{}", self.legs_count(), self.name(), self.ticket_flag.unwrap_or(' '));

        for leg in &self.legs {

            let seat = if let Some(ref seat) = leg.seat {
                format!("{:0>4}", seat)
            } else {
                "    ".into()
            };

            let seq = if let Some(seq) = leg.checkin_sequence {
                format!("{:0>4}", seq)
            } else {
                "    ".into()
            };


            ret = format!("{}{:<7}{:<3}{:<3}{:<3}{:<5}{:3}{:1}{:>4}{:<5}{:1}00",
                ret,
                leg.pnr,
                leg.src_airport,
                leg.dst_airport,
                leg.operating_airline,
                leg.flight_number,
                leg.flight_day_aligned(),
                leg.compartment.unwrap_or(' '),
                seat,
                seq,
                leg.pax_status.to_char());
        }
        Ok(ret)
    }

    pub fn from(src: &str) -> Result<BCBP> {


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
        let legs = chunk.fetch_usize(Field::LegsCount, 10)?;

        if !(1..=9).contains(&legs) {
            return Err(Error::InvalidLegsCount)
        }

        let mut bcbp = BCBP::default();


        let name = chunk.fetch_str(Field::PaxName)?;

        let (first, last) = bcbp_name(name);
        bcbp.name_last   = first;
        bcbp.name_first  = last;
        bcbp.ticket_flag = match chunk.fetch_char(Field::ETicketIndicator)? {
            ' ' => None,
            c   => Some(c),
        };

        for leg_index in 0 .. legs {

            // Mandatory fields common to all legs.
            let mut leg = Leg {
                pnr: chunk
                    .fetch_str(Field::OperatingAirlinePnr)?
                    .trim()
                    .into(),
                src_airport: chunk.fetch_str(Field::FromCityAirportCode)?
                    .trim()
                    .into(),
                dst_airport: chunk.fetch_str(Field::ToCityAirportCode)?
                    .trim()
                    .into(),
                operating_airline: chunk
                    .fetch_str(Field::OperatingAirline)?
                    .trim()
                    .into(),
                flight_number: chunk.fetch_str(Field::FlightNumber)?.trim().into(),
                flight_day:    u16_from_str_force(chunk.fetch_str(Field::DateOfFlight)?, 10),
                compartment: match chunk.fetch_char(Field::CompartmentCode)? {
                    ' ' => None,
                    c   => Some(c),
                },
                seat:       seat_opt(chunk.fetch_str(Field::SeatNumber)?.trim()),
                checkin_sequence:   u32_from_str_opt(chunk.fetch_str(Field::CheckInSequence)?, 10),

                pax_status: PaxStatus::from_char(chunk.fetch_char(Field::PaxStatus)?),

                .. Default::default()
            };


            // Field size of the variable size field that follows for the leg.
            let conditional_size =
                chunk.fetch_usize(Field::VariableBlockSize, 16)?;

            if conditional_size > chunk.len() {
                return Err(Error::CoditionalDataSize)
            }

            if conditional_size > 0 {

                // chunk over the entire set of conditional fields.
                let mut conditional_item = chunk.fetch_chunk(conditional_size)?;

                // The first leg may contain some optional fields at the root level.
                if leg_index == 0 {

                    // Validate the beginning of version number tag as a sanity check.
                    let prefix = conditional_item.fetch_char(Field::VersionBegin)?;
                    if prefix != '<' && prefix != '>' {
                        return Err(Error::InvalidPrefix(Field::VersionBegin, prefix))
                    }

                    bcbp.version = Some(conditional_item.fetch_char(Field::Version)?);

                    // Conditional unique fields are embedded in their own variable-length wrapper.
                    if conditional_item.len() > 0 {
                    let len = conditional_item
                        .fetch_usize(Field::UniqueBlockSize, 16)?;
                    if len > 0 {
                        let mut unique_chunk = conditional_item.fetch_chunk(len)?;

                        bcbp.pax_type =
                            unique_chunk
                            .fetch_char_opt(Field::PaxDescription)?
                            .map(PaxType::from_char).unwrap_or_default();
                        bcbp.checkin_src =
                            unique_chunk.fetch_char_opt(Field::CheckInSrc)?;
                        bcbp.boardingpass_src = unique_chunk
                            .fetch_char_opt(Field::BoardingPassIssueSrc)?;
                        bcbp.boardingpass_issued = unique_chunk
                            .fetch_str_opt(Field::BoardingPassIssueDate)?
                            .map(|x| u16_from_str_force(x, 10));
                        bcbp.doc_type = unique_chunk.fetch_char_opt(Field::DocumentType)?;
                        bcbp.boardingpass_airline = unique_chunk
                            .fetch_str_opt(Field::BoardingPassIssueAirline)?
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
                    .fetch_usize(Field::RepeatedBlockSize, 16)?;
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
                        .fetch_str_opt(Field::MarketingAirline)?
                        .map(|x| x.trim().into());
                    leg.frequent_flyer_airline = repeated_chunk
                        .fetch_str_opt(Field::FrequentFlyerAirline)?
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

            }

            bcbp.legs.push(leg);
        }

        // Remaining input is ascribed to Security Data.
        if chunk.len() > 0 {

            let prefix = chunk.fetch_char(Field::SecurityDataBegin)?;
            if prefix != '^' {
                return Err(Error::InvalidPrefix(Field::SecurityDataBegin, prefix))
            }

            // The security data type captured as a separate field set as the next field, data length, is discarded.
            bcbp.security_data_type = chunk.fetch_char_opt(Field::SecurityDataKind)?;

            // Scan the length of the security data.
            if chunk.len() > 0 {
                let len = chunk.fetch_usize(Field::SecurityDataLen, 16)?;
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
        Some(first.to_string())
    } else {
        None
    };

    (last, first)
}

pub fn fix_length(src: &str) -> std::result::Result<String, FixError> {

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
            ("BRUNER ROMAN MR", Some(""))
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