use std::str;
use std::u32;
use std::usize;
use self::str::FromStr;

use nom::{IResult, ErrorKind, alpha, alphanumeric, digit, space, anychar, rest_s};
use chrono::Duration;
pub use chrono::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Error {
    DataLength,
    FormatCode,
    SegmentsCount,
    Format,
    Name,
    Date,
    CoditionalData,
    CoditionalDataSize,
    SecurityDataSize,
    SecurityData,
}

#[derive(Debug, Clone)]
pub struct Segment {
    pnr: String,
    src_airport: String,
    dst_airport: String,
    airline: String,
    flight_code: String,
    flight_day: u32,
    compartment: char,
    seat: String,
    sequence: u32,
    pax_status: String,
}

impl Segment {
    pub fn new() -> Segment {
        Segment {
            pnr: String::new(),
            airline: String::new(),
            src_airport: String::new(),
            dst_airport: String::new(),
            flight_code: String::new(),
            flight_day: 0,
            compartment: ' ',
            seat: String::new(),
            sequence: 0,
            pax_status: String::new(),
        }
    }

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

    pub fn flight_day(&self) -> u32 {
        self.flight_day
    }

    pub fn flight_date(&self, year: i32) -> NaiveDate {

        let day = if self.flight_day > 0 && self.flight_day < 366 { self.flight_day } else { 1 };

        NaiveDate::from_yo(year, day)
    }

    pub fn flight_date_current_year(&self) -> NaiveDate {
        let now = Utc::today();

        self.flight_date(now.year())
    }

    pub fn flight_day_aligned(&self) -> String {
        if self.flight_day == 0 {
            return String::new()
        }
        format!("{:0>3}", self.flight_day).into()
    }

    pub fn flight_date_set(&mut self, date: NaiveDate) {
        self.flight_day = date.ordinal();
    }

    pub fn compartment(&self) -> char {
        self.compartment
    }

    pub fn compartment_set(&mut self, c: char) -> bool {
        // if c.is_ascii_alphabetic() {
            self.compartment = c;
            true
        // } else {
        //     false
        // }
    }

    pub fn seat(&self) -> &str {
        self.seat.as_ref()
    }

    pub fn seat_set<S>(&mut self, seat: S) where S: Into<String>{
        self.seat = seat.into();
    }

    pub fn seat_aligned(&self) -> String {
        if self.seat.len() == 0 {
            return String::new()
        }
        format!("{:0>4}", self.seat).into()
    }

    pub fn sequence(&self) -> u32 {
        self.sequence
    }

    pub fn sequence_mut(&mut self) -> &mut u32 {
        &mut self.sequence
    }

    pub fn sequence_aligned(&self) -> String {
        if self.sequence == 0 {
            return String::new()
        }
        format!("{:0>4}", self.sequence).into()
    }

    pub fn pax_status(&self) -> &str {
        self.pax_status.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct BCBP {
    ticket_flag: char,
    name_first: String,
    name_last: String,
    pub segments: Vec<Segment>,
    conditional_version: Option<char>,
    conditional_data: Option<String>,
    pax_type: Option<char>,
    doc_type: Option<char>,
    checkin_src: Option<char>,
    boardingpass_src: Option<char>,
    boardingpass_day: Option<u32>,
    boardingpass_airline: Option<String>,
    security_data_type: Option<char>,
    security_data: Option<String>,
}

impl BCBP {

    pub fn new() -> BCBP {
        BCBP {
            name_first: String::new(),
            name_last:  String::new(),
            ticket_flag: ' ',
            segments: Vec::new(),
            conditional_version: None,
            conditional_data: None,
            pax_type: None,
            doc_type: None,
            checkin_src: None,
            boardingpass_src: None,
            boardingpass_day: None,
            boardingpass_airline: None,
            security_data_type: None,
            security_data: None,
        }
    }

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

    pub fn ticket_flag(&self) -> char {
        self.ticket_flag
    }

    pub fn segments_count(&self) -> u8 {
        let mut cnt = self.segments.len();
        if cnt > 9 {
            cnt = 9;
        }
        cnt as u8
    }

    pub fn segments(&self) -> &Vec<Segment> {
        &self.segments
    }

    pub fn segments_mut(&mut self) -> &mut Vec<Segment> {
        &mut self.segments
    }

    pub fn conditional_version(&self) -> Option<char> {
        self.conditional_version
    }

    pub fn checkin_src(&self) -> Option<char> {
        self.checkin_src
    }

    pub fn boardingpass_src(&self) -> Option<char> {
        self.boardingpass_src
    }

    pub fn boardingpass_day(&self) -> Option<u32> {
        self.boardingpass_day
    }

    pub fn boardingpass_airline(&self) -> Option<&String> {
        self.boardingpass_airline.as_ref()
    }

    pub fn pax_type(&self) -> Option<char> {
        self.pax_type
    }

    pub fn doc_type(&self) -> Option<char> {
        self.doc_type
    }

    pub fn build(&self) -> Result<String, String> {

        let mut ret = format!("M{}{:<20}{}", self.segments_count(), self.name(), self.ticket_flag);

        for s in &self.segments {
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

    pub fn from(src: &str) -> Result<BCBP, Error> {
        let src = src.to_uppercase();

        if src.len() < 60 {
            return Err(Error::DataLength)
        }

        let mut bcbp = BCBP::new();

        match bcbp_main(src.as_ref()) {
            IResult::Done(rest, parts)    => {

                let legs_count = parts.0 as i8 - '0' as i8;

                if legs_count < 1 || legs_count > 9 {
                    return Err(Error::SegmentsCount)
                }

                bcbp.ticket_flag = parts.2;

                match bcbp_name(parts.1) {
                    IResult::Done(name_rest, name)    => {
                        if name_rest != "" {
                            return Err(Error::Name)
                        }
                        bcbp.name_last  = name.0;
                        bcbp.name_first = name.1.unwrap_or(String::from("")).trim().into();
                    },
                    _ => return Err(Error::Name)
                }

                let mut next_segment = rest;

                for i in 0 .. legs_count {
                    match bcbp_segment(next_segment) {
                        IResult::Done(leg_rest, o)    => {
                            let sz = usize::from_str_radix(o.1, 16).map_err(|_| Error::CoditionalDataSize)?;

                            if sz > leg_rest.len() {
                                return Err(Error::CoditionalDataSize)
                            }

                            let (first, last) = leg_rest.split_at(sz);

                            // #[cfg(test)] println!("{:?} | {:?}", first, last);
                            bcbp.segments.push(o.0);

                            next_segment = last;

                            let mut chunk = first;

                            if sz != 0 {
                                if i == 0 {
                                    match bcbp_ext_uniq(chunk) {
                                        IResult::Done(_, o)    => {
                                            //println!("U== {:?}", o);

                                            let sz  = usize::from_str_radix(o.1, 16).map_err(|_| Error::CoditionalDataSize)?;
                                            let split_pos = sz + 4;
                                            if  split_pos > chunk.len() {
                                                return Err(Error::CoditionalDataSize)
                                            }

                                            let (first, last) = chunk.split_at(split_pos);

                                            bcbp.conditional_version = Some(o.0);
                                            bcbp.conditional_data    = Some(first.into());
                                            bcbp.pax_type = o.2;
                                            bcbp.checkin_src = o.3;
                                            bcbp.boardingpass_src = o.4;
                                            bcbp.boardingpass_day = o.5.map(|x| u32_from_str_force(x, 10));
                                            bcbp.doc_type = o.6;
                                            bcbp.boardingpass_airline = o.7.map(|x| x.trim_right().to_owned());

                                            // 0 ver: anychar >>
                                            // 1 size: take!(2) >>
                                            // 2 pax_type: opt!(complete!(anychar)) >>
                                            // 3 checkin_src: opt!(complete!(anychar)) >>
                                            // 4 boardingpass_src: opt!(complete!(anychar)) >>
                                            // 5 boardingpass_day: opt!(complete!(take!(4))) >>
                                            // 6 doc_type: opt!(complete!(anychar)) >>
                                            // 7 boardingpass_airline: opt!(complete!(take!(3))) >>
                                            // 8 tags: opt!(complete!(take!(13))) >>

                                            chunk = last;

                                            //println!("U>> {:?}", chunk);
                                        },
                                        _ => return Err(Error::CoditionalData)
                                    }
                                }

                                match bcbp_ext_seg(chunk) {
                                    IResult::Done(_, o)    => {
                                        let sz = usize::from_str_radix(o.0, 16).map_err(|_| Error::CoditionalDataSize)?;
                                        let split_pos = sz + 2;
                                        if  split_pos > chunk.len() {
                                            return Err(Error::CoditionalDataSize)
                                        }

                                        let (_, last) = chunk.split_at(split_pos);

                                        chunk = last;

                                        #[cfg(test)] println!("S>> {:?}", chunk);

                                    },
                                    _ => return Err(Error::CoditionalData)
                                }

                            }
                        },
                        IResult::Error(e)      => println!("{:?}", e),
                        IResult::Incomplete(_) => {
                            return Err(Error::DataLength)
                        }
                    }
                }
            },
            IResult::Error(e) => {
                match e {
                    ErrorKind::Custom(1) => return Err(Error::FormatCode),
                    _ => return Err(Error::Format),
                }
            },
            IResult::Incomplete(_) => {
                return Err(Error::DataLength)
            }
        }

        Ok(bcbp)
    }
}

fn u32_from_str_force(src: &str, radix: u32) -> u32 {
    match u32::from_str_radix(src.trim().trim_left_matches('0'), radix) {
        Ok(v) => v,
        _     => 0,
    }
}

named!(bcbp_main<&str, (char, &str, char)>,
    do_parse!(
        add_return_error!(
            ErrorKind::Custom(1),
            char!('M')
        ) >>
        segments: add_return_error!(
            ErrorKind::Custom(2),
            anychar
        ) >>
        name: add_return_error!(
            ErrorKind::Custom(3),
            take!(20)
        ) >>
        ticket_flag: add_return_error!(
            ErrorKind::Custom(4),
            anychar
        ) >>
        (
            segments,
            name,
            ticket_flag
        )
    )
);

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
    let last = input[last_start_idx..last_end_idx].trim_right().to_string();
    let first = if have_first {
        let first = input[first_start_idx..first_end_idx].trim_right();
        if !first.is_empty() {
            Some(first.to_string())
        } else {
            None
        }
    } else {
        None
    };

    IResult::Done("", (last, first))
}

named!(bcbp_segment<&str, (Segment, &str)>,
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
            take!(1)
        ) >>
        size_ext: add_return_error!(
            ErrorKind::Custom(1011),
            take!(2)
        ) >>
        (
            Segment{
                pnr: pnr.trim().into(),
                src_airport: src.trim().into(),
                dst_airport: dst.trim().into(),
                airline: airline.trim().into(),
                flight_code: flight_code.trim().into(),
                flight_day: u32_from_str_force(flight_day, 10),
                compartment: compartment,
                seat: seat.trim().trim_left_matches('0').to_string(),
                sequence: u32_from_str_force(sequence, 10),
                pax_status: pax_status.trim().into(),
            },
            size_ext
        )
    )
);

named!(bcbp_ext_uniq<&str, (char, &str, Option<char>, Option<char>, Option<char>, Option<&str>, Option<char>, Option<&str>, Option<&str>)>,
    do_parse!(
        add_return_error!(
            ErrorKind::Custom(2001),
            alt!(char!('>') | char!('<'))
        ) >>
        ver: anychar >>
        size: take!(2) >>
        pax_type: opt!(complete!(anychar)) >>
        checkin_src: opt!(complete!(anychar)) >>
        boardingpass_src: opt!(complete!(anychar)) >>
        boardingpass_day: opt!(complete!(take!(4))) >>
        doc_type: opt!(complete!(anychar)) >>
        boardingpass_airline: opt!(complete!(take!(3))) >>
        tags: opt!(complete!(take!(13))) >>
        (
            ver,
            size,
            pax_type,
            checkin_src,
            boardingpass_src,
            boardingpass_day,
            doc_type,
            boardingpass_airline,
            tags
        )
    )
);

named!(bcbp_ext_seg<&str, (&str, Option<&str>, Option<&str>, Option<char>, Option<char>, Option<&str>, Option<&str>, Option<&str>, Option<char>, Option<&str>)>,
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
        (
            size,
            prefix,
            number,
            indicator,
            verify,
            airline,
            ff_airline,
            ff_number,
            id_ad,
            bag_allowance
        )
    )
);

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