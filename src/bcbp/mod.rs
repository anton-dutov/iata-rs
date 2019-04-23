use std::str;
use std::u32;
use std::usize;

use nom::{
    IResult,
    anychar,
};

// use nom::{IResult, ErrorKind, alpha, alphanumeric, digit, space, anychar, rest_s};
// use chrono::Duration;
pub use chrono::prelude::*;

mod errors;

pub use crate::bcbp::errors::{
    ParseError,
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
    flight_code: String,
    pub flight_day: u16,
    pub compartment: char,
    seat: Option<String>,
    pub airline_num: Option<u16>,
    pub sequence: Option<u16>,
    pub pax_status: char,
    pub document_num: Option<String>,
    // Selectee
    // marketing_airline
    ff_airline: Option<String>,
    ff_number: Option<String>,
    fast_track: Option<char>,
    // ID/AD indicator
    // bag allowance
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

    pub fn flight_code(&self) -> &str {
        self.flight_code.as_ref()
    }

    pub fn flight_code_set<S>(&mut self, code: S) where S: Into<String> {
        self.flight_code = code.into();
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
            return String::new()
        }
    }

    pub fn sequence_aligned(&self) -> String {
        if let Some(seq) = self.sequence {
            format!("{:0>4}", seq)
        } else {
            return String::new()
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
    pub boardingpass_day: Option<u16>,
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
                s.flight_code,
                s.flight_day_aligned(),
                s.compartment,
                s.seat_aligned(),
                s.sequence_aligned(),
                s.pax_status);
        }
        Ok(ret)
    }

    pub fn from(src: &str) -> Result<BCBP, ParseError> {
        let src = src.to_uppercase();

        if src.len() < 60 {
            Err(ParseError::MandatoryDataSize)?
        }

        let code = &src[0..1];
        let legs = &src[1..2];
        let name = &src[2..22];
        let flag = &src[22..23];
        let rest =  &src[23..];

        if code != "M" {
            Err(ParseError::InvalidFormatCode(code.chars().next().unwrap_or_default()))?
        }

        let legs = usize::from_str_radix(legs, 10).map_err(|_| ParseError::InvalidLegsCount)?;

        if legs < 1 || legs > 9 {
            Err(ParseError::InvalidLegsCount)?
        }

        let mut bcbp = BCBP::default();

        match bcbp_name(name) {
            Ok((name_rest, name))    => {
                if name_rest != "" {
                    Err(ParseError::Name)?
                }
                bcbp.name_last  = name.0;
                bcbp.name_first = name.1.unwrap_or_default().trim().into();
            },
            _ => Err(ParseError::Name)?
        }

        bcbp.ticket_flag = flag.chars().next().ok_or_else(|| ParseError::InvalidFormat)?;

        let mut next_segment = rest;

        for i in 0 .. legs {
            match bcbp_leg(next_segment) {
                Ok((leg_rest, o))    => {
                    let sz = usize::from_str_radix(o.1, 16).map_err(|_| ParseError::CoditionalDataSize)?;

                    if sz > leg_rest.len() {
                        Err(ParseError::CoditionalDataSize)?
                    }

                    let (first, last) = leg_rest.split_at(sz);

                    bcbp.legs.push(o.0);

                    next_segment  = last;

                    let mut chunk = first;

                    if sz == 0 {
                        continue;
                    }

                    // Extended: Unique data
                    if i == 0 {
                        let tag = chunk[0..1].chars().next().unwrap_or_default();

                        if tag != '<' && tag != '>' {
                            Err(ParseError::InvalidVersionBegin(tag))?
                        }

                        bcbp.version = Some(chunk[1..2].chars().next().unwrap_or_default());


                        chunk =  &chunk[2..];

                        // 1 size: take!(2) >>
                        // 2 pax_type: opt!(complete!(anychar)) >>
                        // 3 checkin_src: opt!(complete!(anychar)) >>
                        // 4 boardingpass_src: opt!(complete!(anychar)) >>
                        // 5 boardingpass_day: opt!(complete!(take!(4))) >>
                        // 6 doc_type: opt!(complete!(anychar)) >>
                        // 7 boardingpass_airline: opt!(complete!(take!(3))) >>
                        // 8 tags: opt!(complete!(take!(13))) >>

                        match bcbp_ext_v1(chunk) {
                            Ok((rest, o)) => {
                                // println!("U << {:?}", chunk);

                                let sz  = usize::from_str_radix(o.0, 16).map_err(|_| ParseError::CoditionalDataSize)?;
                                let split_pos = sz + 2;
                                if  split_pos > chunk.len() {
                                    Err(ParseError::CoditionalDataSize)?
                                }

                                let (_first, last) = chunk.split_at(split_pos);
                                // println!("U [REST] {:?}", rest);
                                // println!("U [LAST] {:?}", last);

                                bcbp.pax_type    = o.1;
                                bcbp.checkin_src = o.2;
                                bcbp.boardingpass_src = o.3;
                                bcbp.boardingpass_day = o.4.map(|x| u16_from_str_force(x, 10));
                                bcbp.doc_type = o.5;
                                bcbp.boardingpass_airline = o.6.map(|x| x.trim_end().to_owned());

                                if let Some(t) = o.7.map(str::trim) {
                                    if !t.is_empty() {
                                        bcbp.bagtags.push(t.to_owned());
                                    }
                                }

                                chunk = last;

                                // println!("U >> {:?}", chunk);
                            },
                            _ => Err(ParseError::CoditionalData)?
                        }
                    }

                    // Repeat data
                    match bcbp.version {

                        _ => match bcbp_v3(chunk) {
                            Ok((rest, o)) => {
                                // println!("S << {:?}", chunk);


                                // println!("S [REST] {:?}", rest);


                                // println!("{:#?}", o);

                                let sz = usize::from_str_radix(o.0, 16).map_err(|_| ParseError::CoditionalDataSize)?;
                                let split_pos = sz + 2;
                                if  split_pos > chunk.len() {
                                    Err(ParseError::CoditionalDataSize)?
                                }

                                let (_first, last) = chunk.split_at(split_pos);

                                bcbp.legs[i].airline_num  = o.1.map(|x| u16_from_str_force(x, 10));
                                bcbp.legs[i].document_num = o.2.map(String::from);
                                bcbp.legs[i].ff_airline  = o.6.map(String::from);
                                bcbp.legs[i].ff_number   = o.7.map(String::from);
                //                 bcbp.legs[i].var         = Some(last.to_owned());

                                chunk = last;

                                // println!("S >> {:?}", chunk);

                            },
                            _ => Err(ParseError::CoditionalData)?
                        }
                    }
                },
                Err(e) => {
                    if e.is_incomplete() {
                        Err(ParseError::InsufficientDataLength)?
                    }
                    println!("{:?}", e);
                }
            }
        }

        Ok(bcbp)
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


fn bcbp_name(input: &str) -> IResult<&str, (String, Option<String>)> {
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

    Ok(("", (last, first)))
}

named!(bcbp_leg<&str, (Leg, &str)>,
    do_parse!(
        pnr: add_return_error!(
            ErrorKind::Custom(1001),
            take!(7)
        ) >>
        src: add_return_error!(
            ErrorKind::Custom(1002),
            take!(3)
        ) >>
        dst: add_return_error!(
            ErrorKind::Custom(1003),
            take!(3)
        ) >>
        airline: add_return_error!(
            ErrorKind::Custom(1004),
            take!(3)
        ) >>
        flight_code: add_return_error!(
            ErrorKind::Custom(1005),
            take!(5)
        ) >>
        flight_day: add_return_error!(
            ErrorKind::Custom(1006),
            take!(3)
        ) >>
        compartment: add_return_error!(
            ErrorKind::Custom(1007),
            anychar
        ) >>
        seat: add_return_error!(
            ErrorKind::Custom(1008),
            take!(4)
        ) >>
        sequence: add_return_error!(
            ErrorKind::Custom(1009),
            take!(5)
        ) >>
        pax_status: add_return_error!(
            ErrorKind::Custom(1010),
            anychar
        ) >>
        size_ext: add_return_error!(
            ErrorKind::Custom(1011),
            take!(2)
        ) >>
        (
            Leg {
                pnr: pnr.trim().into(),
                src_airport: src.trim().into(),
                dst_airport: dst.trim().into(),
                airline: airline.trim().into(),
                flight_code: flight_code.trim().into(),
                flight_day: u16_from_str_force(flight_day, 10),
                compartment,
                seat: seat_opt(seat),
                sequence: u32_from_str_opt(sequence, 10),
                pax_status: pax_status,
                .. Default::default()
            },
            size_ext
        )
    )
);

named!(bcbp_ext_v1<&str, (&str, PaxType, Option<char>, Option<char>, Option<&str>, Option<char>, Option<&str>, Option<&str>)>,
    do_parse!(
        size: take!(2) >>
        pax_type: opt!(complete!(anychar)) >>
        ci_src: opt!(complete!(anychar)) >>
        bp_src: opt!(complete!(anychar)) >>
        bp_day: opt!(complete!(take!(4))) >>
        doc_type: opt!(complete!(anychar)) >>
        bp_airline: opt!(complete!(take!(3))) >>
        bagtag: opt!(complete!(take!(13))) >>
        (
            size,
            pax_type.map(PaxType::from_char).unwrap_or_default(),
            ci_src,
            bp_src,
            bp_day,
            doc_type,
            bp_airline,
            bagtag
        )
    )
);

named!(bcbp_v3<&str, (&str, Option<&str>, Option<&str>, Option<char>, Option<char>, Option<&str>, Option<&str>, Option<&str>, Option<char>, Option<&str>, Option<char>)>,
    do_parse!(
        size: take!(2) >>
        prefix: opt!(complete!(take!(3))) >>
        number: opt!(complete!(take!(10))) >>
        indicator: opt!(complete!(anychar)) >>
        verify: opt!(complete!(anychar)) >>
        airline: opt!(complete!(take!(3))) >>
        ff_airline: opt!(complete!(take!(3))) >>
        ff_number: opt!(complete!(take!(16))) >>
        id_ad: opt!(complete!(anychar)) >>
        bag_allowance: opt!(complete!(take!(3))) >>
        fast_track: opt!(complete!(anychar)) >>
        (
            size,
            prefix.map(str::trim),
            number.map(str::trim),
            indicator,
            verify,
            airline.map(str::trim),
            ff_airline.map(str::trim),
            ff_number.map(str::trim),
            id_ad,
            bag_allowance.map(str::trim),
            fast_track
        )
    )
);

pub fn fix_length(src: &str) -> Result<String, FixError> {

    if src.len() < 60 {
        Err(FixError::InsufficientDataLength)?
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
            let (left, names) = bcbp_name(names[i]).unwrap();

            assert!(left.is_empty());

            let (last, first) = names;
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