use std::str;
use std::u32;

use time::Date;

mod error;
pub mod field;
pub mod raw;
pub(crate) mod chunk;

use chunk::Chunk;
use field::Field;

pub use crate::bcbp::error::{
    Error,
    FixError,
    BcbpResult,
};

use crate::datetime::{DayOfYear, Error as DateError};


#[derive(Debug, PartialEq)]
pub enum Mode {
    Tolerant,
    Strict
}



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize))]
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



#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize))]
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
#[cfg_attr(feature = "with-serde", derive(serde::Serialize))]
pub struct Leg {
    pnr: Option<String>,
    src_airport: Option<String>,
    dst_airport: Option<String>,
    airline: Option<String>,
    flight_number: Option<String>,
    pub flight_day:    Option<DayOfYear>,
    pub compartment: Option<char>,
    seat: Option<String>,
    pub airline_num: Option<u16>,
    pub sequence: Option<u16>,
    pub pax_status: PaxStatus,
    doc_number: Option<String>,
    // Selectee
    // marketing_airline
    marketing_airline: Option<String>,
    frequent_flyer_airline: Option<String>,
    frequent_flyer_number: Option<String>,
    pub fast_track: Option<char>,
    // ID/AD indicator
    bag_allowance: Option<String>,
    // data
    pub var: Option<String>,
}

macro_rules! gen_get_set {
    (get_set $method_name:ident for $field_name:ident with len $to:literal) => {
        gen_get_set!(get_set $method_name for $field_name with len 1..=$to);
    };
    (get_set $method_name:ident for $field_name:ident with len $from:literal..=$to:literal) => {
        gen_get_set!(get_set $method_name(str::trim) for $field_name with len $from..=$to);
    };
    (get_set $method_name:ident($preprocess:path) for $field_name:ident with len $to:literal) => {
        gen_get_set!(get_set $method_name($preprocess) for $field_name with len 1..=$to);
    };
    (get_set $method_name:ident($preprocess:path) for $field_name:ident with len $from:literal..=$to:literal) => {
        gen_get_set!(
            get_set $method_name($preprocess) for $field_name
            with |s: &str| {
                if !($from..=$to).contains(&s.len()) {
                    Err(Error::MandatoryDataSize)
                } else {
                    Ok(())
                }
            }
        );
    };
    (get_set $method_name:ident($preprocess:path) for $field_name:ident with $verify:expr) => {
        pub fn $method_name(&mut self, s: &str) -> BcbpResult<()> {
            let s = $preprocess(s);

            if s.is_empty() {
                self.$field_name = None;
                return Ok(());
            }

            $verify(s)?;

            self.$field_name = Some(s.to_owned());
            Ok(())
        }

        pub fn $field_name(&self) -> Option<&str> {
            self.$field_name.as_deref()
        }
    };
}

impl Leg {

    pub fn flight_day(&self) -> Option<&DayOfYear> {
        self.flight_day.as_ref()
    }

    pub fn set_flight_date(&mut self, date: Date) -> std::result::Result<(), DateError> {
        self.flight_day = Some(DayOfYear::new(date.ordinal())?);

        Ok(())
    }

    gen_get_set!(get_set set_pnr for pnr with len 7);
    gen_get_set!(get_set set_src_airport for src_airport with len 3);
    gen_get_set!(get_set set_dst_airport for dst_airport with len 3);
    gen_get_set!(get_set set_airline for airline with len 3);
    gen_get_set!(get_set set_flight_number for flight_number with len 5);
    gen_get_set!(get_set set_seat(Leg::seat_preprocess) for seat with len 4);
    gen_get_set!(get_set set_doc_number for doc_number with len 10);
    gen_get_set!(get_set set_marketing_airline for marketing_airline with len 3);
    gen_get_set!(get_set set_frequent_flyer_airline for frequent_flyer_airline with len 3);
    gen_get_set!(get_set set_frequent_flyer_numbder for frequent_flyer_number with len 16);
    gen_get_set!(get_set set_bag_allowance for bag_allowance with len 3);

    fn seat_preprocess(s: &str) -> &str { s.trim().trim_start_matches('0') }
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize))]
pub struct Bcbp {
    pub version: Option<char>,
    pub pax_type: PaxType,
    pub doc_type: Option<char>,
    pub name_last: String,
    pub name_first: Option<String>,
    pub ticket_flag: Option<char>,
    pub legs: Vec<Leg>,
    bagtag1: Option<String>,
    bagtag2: Option<String>,
    bagtag3: Option<String>,
    pub checkin_src: Option<char>,
    pub boardingpass_src: Option<char>,
    pub boardingpass_issued: Option<u16>,
    boardingpass_airline: Option<String>,
    pub security_data_type: Option<char>,
    pub security_data: Option<String>,
}

impl Bcbp {
    fn verify_bagtag(s: &str) -> BcbpResult<()> {
        if s.len() != 13 {
            Err(Error::MandatoryDataSize)
        } else if !s.as_bytes().iter().all(u8::is_ascii_digit) {
            Err(Error::DigitsExpected)
        } else {
            Ok(())
        }
    }

    gen_get_set!(get_set set_bagtag1(str::trim) for bagtag1 with Bcbp::verify_bagtag);
    gen_get_set!(get_set set_bagtag2(str::trim) for bagtag2 with Bcbp::verify_bagtag);
    gen_get_set!(get_set set_bagtag3(str::trim) for bagtag3 with Bcbp::verify_bagtag);
    gen_get_set!(get_set set_boradingpass_airline for boardingpass_airline with len 3);

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

    pub fn build(&self, _mode: Mode) -> BcbpResult<String> {


        let mut ret = format!("M{}{:<20}{}", self.legs_count(), self.name(), self.ticket_flag.unwrap_or(' '));

        for leg in &self.legs {

            let seat = if let Some(ref seat) = leg.seat {
                let is_normal_seat =
                    seat.as_bytes().first()
                    .unwrap() // The code excludes containing empty string
                    .is_ascii_digit();
                if is_normal_seat {
                    format!("{seat:0>4}")
                } else {
                    format!("{seat:<4}")
                }
            } else {
                "    ".into()
            };

            let seq = if let Some(seq) = leg.sequence {
                format!("{:0>4}", seq)
            } else {
                "    ".into()
            };


            ret = format!("{}{:<7}{:<3}{:<3}{:<3}{:<5}{:3}{:1}{:>4}{:<5}{:1}00",
                ret,
                leg.pnr.as_deref().unwrap_or(""),
                leg.src_airport.as_deref().unwrap_or(""),
                leg.dst_airport.as_deref().unwrap_or(""),
                leg.airline.as_deref().unwrap_or(""),
                leg.flight_number.as_deref().unwrap_or(""),
                if let Some(ref day) = leg.flight_day {
                    format!("{:0>3}", day.ordinal())
                } else {
                    String::from("   ")
                },
                leg.compartment.unwrap_or(' '),
                seat,
                seq,
                leg.pax_status.to_char());
        }
        Ok(ret)
    }

    pub fn from(src: &str) -> BcbpResult<Bcbp> {


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

        if !(1..=9).contains(&legs) {
            return Err(Error::InvalidLegsCount)
        }

        let mut bcbp = Bcbp::default();


        let name = chunk.fetch_str(Field::PassengerName)?;

        let (first, last) = bcbp_name(name);
        bcbp.name_last   = first;
        bcbp.name_first  = last;
        bcbp.ticket_flag = match chunk.fetch_char(Field::ElectronicTicketIndicator)? {
            ' ' => None,
            c   => Some(c),
        };

        for leg_index in 0 .. legs {

            let mut leg = Leg::default();

            // Mandatory fields common to all legs.
            leg.set_pnr(chunk.fetch_str(Field::OperatingCarrierPnrCode)?)?;
            leg.set_src_airport(chunk.fetch_str(Field::FromCityAirportCode)?)?;
            leg.set_dst_airport(chunk.fetch_str(Field::ToCityAirportCode)?)?;
            leg.set_airline(chunk.fetch_str(Field::OperatingCarrierDesignator)?)?;
            leg.set_flight_number(chunk.fetch_str(Field::FlightNumber)?)?;

            let flight_day = chunk.fetch_str(Field::DateOfFlight)?;
            leg.flight_day = if !flight_day.trim().is_empty() {
                Some(DayOfYear::new(u16_from_str_force(flight_day, 10)).unwrap())
            } else {
                None
            };

            leg.compartment   = match chunk.fetch_char(Field::CompartmentCode)? {
                ' ' => None,
                c   => Some(c),
            };

            leg.set_seat(chunk.fetch_str(Field::SeatNumber)?)?;
            leg.sequence      = u32_from_str_opt(chunk
                .fetch_str(Field::CheckInSequenceNumber)?, 10);

            leg.pax_status    = PaxStatus::from_char(chunk.fetch_char(Field::PassengerStatus)?);

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
                        bcbp.set_boradingpass_airline(
                            unique_chunk
                            .fetch_str_opt(Field::AirlineDesignatorOfBoardingPassIssuer)?
                            .unwrap_or("")
                        )?;

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

                    leg.set_doc_number(
                        repeated_chunk
                        .fetch_str_opt(Field::DocumentFormSerialNumber)?
                        .unwrap_or("")
                    )?;
                    let _selectee_indicator =
                        repeated_chunk.fetch_char_opt(Field::SelecteeIndicator)?;
                    let _international_document_verification = repeated_chunk
                        .fetch_char_opt(Field::InternationalDocumentVerification)?;
                    leg.set_marketing_airline(
                        repeated_chunk
                        .fetch_str_opt(Field::MarketingCarrierDesignator)?
                        .unwrap_or("")
                    )?;
                    leg.set_frequent_flyer_airline(
                        repeated_chunk
                        .fetch_str_opt(Field::FrequentFlyerAirlineDesignator)?
                        .unwrap_or("")
                    )?;
                    leg.set_frequent_flyer_numbder(
                        repeated_chunk
                        .fetch_str_opt(Field::FrequentFlyerNumber)?
                        .unwrap_or("")
                    )?;
                    let _id_ad_indicator =
                        repeated_chunk.fetch_char_opt(Field::IdAdIndicator)?;
                    leg.set_bag_allowance(
                        repeated_chunk
                        .fetch_str_opt(Field::FreeBaggageAllowance)?
                        .unwrap_or("")
                    )?;
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