use nom::{IResult, ErrorKind, alpha, alphanumeric, digit, space, anychar};
use std::str;
use std::usize;
use self::str::FromStr;

#[derive(Debug, PartialEq)]
enum Error {
    DataLength,
    FormatCode,
    SegmentsCount,
    Format,
    Name,
    Date,
    CoditionalItems,
    SecurityDataLength,
    SecurityData,
}

#[derive(Debug, Clone)]
struct Segment {
    pnr: String,
    src_airport: String,
    dst_airport: String,
    airline: String,
    flight_code: String,
    flight_day: String,
    compartment: char,
    seat: String,
    sequence: String,
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
            flight_day: String::new(),
            compartment: ' ',
            seat: String::new(),
            sequence: String::new(),
            pax_status: String::new(),
        }
    }

    pub fn pnr(&self) -> &str {
        self.pnr.as_ref()
    }

    pub fn airline(&self) -> &str {
        self.airline.as_ref()
    }

    pub fn src_airport(&self) -> &str {
        self.src_airport.as_ref()
    }

    pub fn dst_airport(&self) -> &str {
        self.dst_airport.as_ref()
    }

    pub fn flight_code(&self) -> &str {
        self.flight_code.as_ref()
    }

    pub fn flight_day(&self) -> &str {
        self.flight_day.as_ref()
    }

    pub fn compartment(&self) -> char {
        self.compartment
    }

    pub fn seat(&self) -> &str {
        self.seat.as_ref()
    }

    pub fn sequence(&self) -> &str {
        self.sequence.as_ref()
    }

    pub fn pax_status(&self) -> &str {
        self.pax_status.as_ref()
    }
}

#[derive(Debug, Clone)]
struct BCBP {
    ticket_flag: char,
    name_first: String,
    name_last: String,
    segments: Vec<Segment>,
    conditional_version: Option<char>,
    conditional_data: Option<String>,
    pax_type: Option<char>,
    doc_type: Option<char>,
    checkin_src: Option<char>,
    boardingpass_src: Option<char>,
    boardingpass_day: Option<String>,
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

    pub fn name_first(&self) -> &str {
        self.name_first.as_ref()
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


    pub fn conditional_verion(&self) -> char {
        self.ticket_flag
    }

//     @property char passengerType() const pure nothrow @safe
//     {
//         return _passengerType;
//     }

//     @property char srcOfCheckIn() const pure nothrow @safe
//     {
//         return _srcOfCheckIn;
//     }

//     @property char srcOfBoardingPass() const pure nothrow @safe
//     {
//         return _srcOfBoardingPass;
//     }

//     @property string securityData() const
//     {
//         return _securityData;
//     }
//     //@property void securityData(in string v);

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
                s.flight_day,
                s.compartment,
                s.seat,
                s.sequence,
                s.pax_status);
//         ret.formattedWrite("%-7s%-3s%-3s%-3s%-5s%03d%-1s%-4s%-5s%-1s%02X",
//             _pnr,
//             _srcAirport,
//             _dstAirport,
//             _airline,
//             _flight.rightJustify(4, '0'),
//             _date.dayOfYear,
//             _compartment,
//             _seat.rightJustify(4, '0'),
//             _sequence.rightJustify(4, '0'),
//             _status,
//             0x00 /* Extra length */);

        }

        println!("");
//         if (_securityData.length)
//         {
//             ret.formattedWrite("^1%02X%s", _securityData.length, _securityData);
//         }
//         return ret.data;
        Ok(ret)
    }

    pub fn from(src: &str/*, int year = Clock.currTime.year*/) -> Result<BCBP, Error> {
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
                        bcbp.name_first = name.1.unwrap_or(String::from(""));
                    },
                    _ => return Err(Error::Name)
                }

                let mut next_segment = rest;

                for i in 0 .. legs_count {

                    // #[cfg(test)] println!("{}>> {:?}", i, next_segment);

                    match bcbp_segment(next_segment) {
                        IResult::Done(leg_rest, o)    => {
                            let sz = usize::from_str_radix(o.1, 16).unwrap();
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

                                            let sz = usize::from_str_radix(o.1, 16).unwrap();
                                            let (first, last) = chunk.split_at(sz + 4);

                                            bcbp.conditional_version = Some(o.0);
                                            bcbp.conditional_data    = Some(first.to_string());

                                            chunk = last;

                                            //println!("U>> {:?}", chunk);
                                        },
                                        _ => return Err(Error::CoditionalItems)
                                    }
                                }

                                match bcbp_ext_seg(chunk) {
                                    IResult::Done(_, o)    => {
                                        #[cfg(test)] println!("S== {:?}", o);

                                        let sz = usize::from_str_radix(o.0, 16).unwrap();

                                        let (_, last) = chunk.split_at(sz + 2);

                                        chunk = last;

                                        #[cfg(test)] println!("S>> {:?}", chunk);

                                    },
                                    _ => return Err(Error::CoditionalItems)
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

#[cfg(test)]
mod tests {
    use bcbp::{BCBP, Error};

    #[test]
    fn errors() {
        match BCBP::from("") {
            Ok(_)  => assert!(false),
            Err(e) => assert!(e == Error::DataLength),
        }

        match BCBP::from("X1BRUNER/ROMAN        EJNUFFX MUCSVOSU 2327 231L013A0052 100") {
            Ok(_)  => assert!(false),
            Err(e) => assert!(e == Error::FormatCode),
        }

        match BCBP::from("M0BRUNER/ROMAN1       EJNUFFX MUCSVOSU 2327 231L013A0052 100") {
            Ok(_)  => assert!(false),
            Err(e) => assert!(e == Error::SegmentsCount)
        }

        match BCBP::from("MABRUNER/ROMAN1       EJNUFFX MUCSVOSU 2327 231L013A0052 100") {
            Ok(_)  => assert!(false),
            Err(e) => assert!(e == Error::SegmentsCount)
        }

        match BCBP::from("M1BRUNER/ROMAN1       EJNUFFX MUCSVOSU 2327 231L013A0052 100") {
            Ok(_)  => assert!(false),
            Err(e) => assert!(e == Error::Name)
        }
    }

    #[test]
    fn mandatory1() {

        let src = "M1JOHN/SMITH          EABCDEF JFKSVOSU 1234A123Y123Z9876 000";
        let tmp = BCBP::from(src);

        assert!(tmp.is_ok());

        let bcbp = tmp.unwrap();

        assert!(bcbp.name()        == "JOHN/SMITH");
        assert!(bcbp.name_last()   == "JOHN");
        assert!(bcbp.name_first()  == "SMITH");
        assert!(bcbp.ticket_flag()  == 'E');
        assert!(bcbp.segments[0].pnr() == "ABCDEF");
        assert!(bcbp.segments[0].src_airport()  == "JFK");
        assert!(bcbp.segments[0].dst_airport()  == "SVO");
        assert!(bcbp.segments[0].airline()      == "SU");
        assert!(bcbp.segments[0].flight_code()  == "1234A");
        assert!(bcbp.segments[0].flight_day()   == "123");
        assert!(bcbp.segments[0].compartment()  == 'Y');
        assert!(bcbp.segments[0].seat()         == "123Z");
        assert!(bcbp.segments[0].sequence()     == "9876");
        assert!(bcbp.segments[0].pax_status()   == "0");
        assert!(bcbp.build().unwrap() == src);
    }

    #[test]
    fn mandatory4() {
        let src = "M4VERYLONGESTLASTNAMEDEABCDEF JFKSVOSU 1234 207          000ABCDEF SVOLEDSU 5678 210          000ABCDEF LEDSVOSU 9876 215          000ABCDEF SVOJFKSU 1357 215          000";
        let tmp = BCBP::from(src);

        assert!(tmp.is_ok());

        let bcbp = tmp.unwrap();

        assert!(bcbp.name()       == "VERYLONGESTLASTNAMED");
        assert!(bcbp.name_last()  == "VERYLONGESTLASTNAMED");
        assert!(bcbp.name_first() == "");
        assert!(bcbp.ticket_flag() == 'E');
        assert!(bcbp.segments[0].pnr()  == "ABCDEF");
        assert!(bcbp.segments[0].src_airport()  == "JFK");
        assert!(bcbp.segments[0].dst_airport()  == "SVO");
        assert!(bcbp.segments[0].airline()      == "SU");
        assert!(bcbp.segments[0].flight_code()  == "1234");
        assert!(bcbp.segments[0].flight_day()   == "207");
        assert!(bcbp.segments[1].pnr()  == "ABCDEF");
        assert!(bcbp.segments[1].src_airport()  == "SVO");
        assert!(bcbp.segments[1].dst_airport()  == "LED");
        assert!(bcbp.segments[1].airline()      == "SU");
        assert!(bcbp.segments[1].flight_code()  == "5678");
        assert!(bcbp.segments[1].flight_day()   == "210");
        assert!(bcbp.segments[2].pnr()  == "ABCDEF");
        assert!(bcbp.segments[2].src_airport()  == "LED");
        assert!(bcbp.segments[2].dst_airport()  == "SVO");
        assert!(bcbp.segments[2].airline()      == "SU");
        assert!(bcbp.segments[2].flight_code()  == "9876");
        assert!(bcbp.segments[2].flight_day()   == "215");
        assert!(bcbp.segments[3].pnr()  == "ABCDEF");
        assert!(bcbp.segments[3].src_airport()  == "SVO");
        assert!(bcbp.segments[3].dst_airport()  == "JFK");
        assert!(bcbp.segments[3].airline()      == "SU");
        assert!(bcbp.segments[3].flight_code()  == "1357");
        assert!(bcbp.segments[3].flight_day()   == "215");

        println!("BLD{:?}\nSRC{:?}", bcbp.build().unwrap(), src);

        assert!(bcbp.build().unwrap() == src);
    }

    #[test]
    fn conditional3() {
        let src = "M3JOHN/SMITH          EABCDEF JFKSVOSK 1234 123M014C0050 35D>5180O 0276BSK              2A55559467513980 SK                         *30600000K09         ABCDEF SVOFRASU 5678 135Y013A0012 3372A55559467513990 SU SU 12345678             09         ABCDEF FRAJFKSU 9876 231Y022F0052 3372A55559467513990 SU SU 12345678             09         ";
        println!("|");
        let tmp = BCBP::from(src);

        assert!(tmp.is_ok());

        let bcbp = tmp.unwrap();

        println!("{:?}", bcbp);

        assert!(bcbp.name()       == "JOHN/SMITH");
        assert!(bcbp.name_last()  == "JOHN");
        assert!(bcbp.name_first() == "SMITH");
        assert!(bcbp.ticket_flag() == 'E');
        assert!(bcbp.segments[0].pnr()  == "ABCDEF");
        assert!(bcbp.segments[0].src_airport()  == "JFK");
        assert!(bcbp.segments[0].dst_airport()  == "SVO");
        assert!(bcbp.segments[0].airline()      == "SK");
        assert!(bcbp.segments[0].flight_code()  == "1234");
        assert!(bcbp.segments[0].flight_day()   == "123");
        assert!(bcbp.segments[1].pnr()  == "ABCDEF");
        assert!(bcbp.segments[1].src_airport()  == "SVO");
        assert!(bcbp.segments[1].dst_airport()  == "FRA");
        assert!(bcbp.segments[1].airline()      == "SU");
        assert!(bcbp.segments[1].flight_code()  == "5678");
        assert!(bcbp.segments[1].flight_day()   == "135");
        assert!(bcbp.segments[2].pnr()  == "ABCDEF");
        assert!(bcbp.segments[2].src_airport()  == "FRA");
        assert!(bcbp.segments[2].dst_airport()  == "JFK");
        assert!(bcbp.segments[2].airline()      == "SU");
        assert!(bcbp.segments[2].flight_code()  == "9876");
        assert!(bcbp.segments[2].flight_day()   == "231");
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

named!(bcbp_name<&str, (String, Option<String>, Option<String>)>,
    do_parse!(
        last:  map_res!(alpha, str::FromStr::from_str) >>
        first: opt!(complete!(
            preceded!(
            char!('/'),
            map_res!(alpha, str::FromStr::from_str)
        ))) >>
        fill: opt!(complete!(map_res!(space, str::FromStr::from_str))) >>
        (
            last,
            first,
            fill
        )
    )
);

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
                pnr:pnr.trim().to_string(),
                src_airport:src.trim().to_string(),
                dst_airport: dst.trim().to_string(),
                airline:airline.trim().to_string(),
                flight_code:flight_code.trim().to_string(),
                flight_day:flight_day.trim().to_string(),
                compartment:compartment,
                seat:seat.trim().to_string(),
                sequence:sequence.trim().to_string(),
                pax_status:pax_status.trim().to_string(),
            },
            size_ext
        )
    )
);

named!(bcbp_ext_uniq<&str, (char, &str, Option<char>, Option<char>, Option<char>, Option<char>, Option<&str>, Option<&str>, Option<&str>)>,
    do_parse!(
        add_return_error!(
            ErrorKind::Custom(2001),
            char!('>')
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
            doc_type,
            checkin_src,
            boardingpass_src,
            boardingpass_day,
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