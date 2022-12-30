use std::str::from_utf8;

use iata::bcbp::*;
use iata::datetime::DayOfYear;

mod samples {
    use std::{str::from_utf8, num::NonZeroU16};

    use rand::{prelude, seq::{IteratorRandom, SliceRandom}, distributions::Standard, thread_rng};
    use iata::{datetime::DayOfYear, bcbp::{Leg, PaxStatus}};

    pub static BASE_BCBP: &str = "M1BRUNER/ROMAN MR     EJNUFFX MUCSVOSU 2327 231L013A0052 100";

    pub struct NameSample<'a> {
        pub fullname: &'a [u8; 20],
        pub lastname: &'a [u8],
        pub firstname: Option<&'a [u8]>,
    }

    pub static NAMES: &[NameSample] = &[
        NameSample {
            fullname: b"TEST/TESTOVICH      ",
            lastname: b"TEST",
            firstname: Some(b"TESTOVICH"),
        },
        NameSample {
            fullname: b"TEST                ",
            lastname: b"TEST",
            firstname: None,
        },
        NameSample {
            fullname: b"I FILL UP THE ENTIRE",
            lastname: b"I FILL UP THE ENTIRE",
            firstname: None,
        },
        NameSample {
            fullname: b"BIGLONGBIG/FIZZBUZZF",
            lastname: b"BIGLONGBIG",
            firstname: Some(b"FIZZBUZZF"),
        },
        NameSample {
            fullname: b"ROMAN/ROMANOV       ",
            lastname: b"ROMAN",
            firstname: Some(b"ROMANOV"),
        },
        NameSample {
            fullname: b"EGOREGOREGOREGOR    ",
            lastname: b"EGOREGOREGOREGOR",
            firstname: None,
        },
        NameSample {
            fullname: b"ADAM/JOHNSON        ",
            lastname: b"ADAM",
            firstname: Some(b"JOHNSON"),
        },
        NameSample {
            fullname: b"ARNOLD/ROBERTSON    ",
            lastname: b"ARNOLD",
            firstname: Some(b"ROBERTSON"),
        },
        NameSample {
            fullname: b"IVANOVA VASILINA/   ",
            lastname: b"IVANOVA VASILINA",
            firstname: Some(b""),
        },
    ];

    pub static PNRS: &[&[u8; 7]] = &[
        b"JNUFFX ",
        b"8OQ6FU ",
        b"AGGA   ",
        b"1234ABC",
        b"ABC1234",
        b"1A0D31J",
    ];

    pub static AIRPORTS: &[&[u8; 3]] = &[
        b"AAA",
        b"AAB",
        b"AAC",
        b"AAD",
        b"AAE",
        b"AAF",
        b"GAR",
        b"GBA",
        b"GBI",
        b"AAJ",
        b"AAK",
        b"AAL",
        b"NAA",
        b"NAC",
        b"AAO",
        b"CAR",
        b"CCH",
        b"JAA",
        b"JAQ",
        b"AAU",
        b"AAV",
        b"AAX",
        b"AAY",
        b"AAZ",
        b"ABA",
        b"ABB",
        b"ABC",
        b"ABD",
        b"LED",
        b"FRA",

    ];

    pub static AIRLINES: &[&[u8; 3]] = &[
        b"WOW",
        b"B  ",
        b"LH ",
        b"RU ",
        b"EN ",
        b"UK ",
        b"US ",
        b"PO ",
        b"LOL",
    ];

    fn flight_number() -> [u8; 5] {
        let mut rng = thread_rng();

        [
            (b'0'..=b'9').choose(&mut rng).unwrap(),
            (b'0'..=b'9').choose(&mut rng).unwrap(),
            (b'0'..=b'9').choose(&mut rng).unwrap(),
            (b'0'..=b'9').choose(&mut rng).unwrap(),
            (b'A'..=b'Z').chain(std::iter::once(b' ')).choose(&mut rng).unwrap(),
        ]
    }

    fn compartment() -> u8 {
        let mut rng = thread_rng();

        (b'A'..=b'Z').choose(&mut rng).unwrap()
    }

    fn day_of_year() -> DayOfYear {
        let mut rng = thread_rng();
        let x = (1..=365).choose(&mut rng).unwrap();

        DayOfYear::new(x).unwrap()
    }

    fn seat() -> [u8; 4] {
        let mut rng = thread_rng();

        *[
            [
                (b'1'..=b'9').choose(&mut rng).unwrap(),
                (b'1'..=b'9').choose(&mut rng).unwrap(),
                (b'1'..=b'9').choose(&mut rng).unwrap(),
                (b'A'..b'Z').choose(&mut rng).unwrap(),
            ],
            *b"INF "
        ].choose(&mut rng).unwrap()
    }

    fn pax_status() -> u8 {
        let mut rng = thread_rng();

        (b'0'..=b'9').choose(&mut rng).unwrap()
    }

    fn pnr() -> [u8; 7] {
        let mut rng = thread_rng();

        **PNRS.choose(&mut rng).unwrap()
    }

    fn airport() -> [u8; 3] {
        let mut rng = thread_rng();

        **AIRPORTS.choose(&mut rng).unwrap()
    }

    fn airline() -> [u8; 3] {
        let mut rng = thread_rng();

        **AIRLINES.choose(&mut rng).unwrap()
    }

    pub fn leg() -> (Leg, impl Iterator<Item = u8>) {
        fn to_utf8(b: &[u8]) -> &str {
            from_utf8(b).unwrap().trim()
        }

        let pnr = pnr();
        let src_airport = airport();
        let dst_airport = airport();
        let airline = airline();
        let flight_number = flight_number();
        let flight_day = day_of_year();
        let compartment = compartment();
        let seat = seat();
        let checking_sequence = rand::random::<NonZeroU16>().get();
        let pax_status = pax_status();

        let checkin_sequence_bytes = if checking_sequence < 10000 {
            format!("{checking_sequence:04} ").into_bytes()
        } else {
            checking_sequence.to_string().into_bytes()
        };
        let flight_day_ordinal = flight_day.ordinal();

        let mut leg = Leg::default();
        leg.set_pnr(to_utf8(&pnr)).unwrap();
        leg.set_src_airport(to_utf8(&src_airport)).unwrap();
        leg.set_dst_airport(to_utf8(&dst_airport)).unwrap();
        leg.set_airline(to_utf8(&airline)).unwrap();
        leg.set_flight_number(to_utf8(&flight_number)).unwrap();
        leg.compartment = Some(compartment as char);
        leg.set_seat(to_utf8(&seat)).unwrap();
        leg.sequence = Some(checking_sequence);
        leg.flight_day = Some(flight_day);
        leg.pax_status = PaxStatus::from_char(pax_status as char);
        (
            leg,
            std::iter::empty()
                .chain(pnr)
                .chain(src_airport)
                .chain(dst_airport)
                .chain(airline)
                .chain(flight_number)
                .chain(format!("{:03}", flight_day_ordinal).into_bytes())
                .chain(std::iter::once(compartment))
                .chain(seat)
                .chain(checkin_sequence_bytes)
                .chain(std::iter::once(pax_status))
                .chain(*b"00")
        )
    }
}

fn ascii_bytes() -> impl Iterator<Item = u8> {
    (0..u8::MAX).filter(u8::is_ascii)
}

#[test]
fn basic_parsing() {
    Bcbp::from(samples::BASE_BCBP).expect("Failed to parse a sample BCBP");
}

#[test]
fn error_invalid_format_code() {
    let mut base_bytes = samples::BASE_BCBP.as_bytes().to_owned();

    for b in ascii_bytes() {
        if b == b'M' { continue; }

        base_bytes[0] = b;

        let s = from_utf8(&base_bytes).unwrap();
        let err = Bcbp::from(s).expect_err("Parsing should have failed");

        assert_eq!(
            err,
            Error::InvalidFormatCode(b as char),
        );
    }
}

#[test]
fn error_invalid_legs_count() {
    let mut base_bytes = samples::BASE_BCBP.as_bytes().to_owned();

    for b in ascii_bytes() {
        if ('1'..='9').contains(&(b as char)) { continue; }

        base_bytes[1] = b;
        let s = from_utf8(&base_bytes).unwrap();
        let err = Bcbp::from(s).expect_err("Parsing should have failed");

        if b == b'0' {
            assert_eq!(
                err,
                Error::InvalidLegsCount,
            );
        } else {
            assert_eq!(
                err,
                Error::ExpectedInteger(field::Field::NumberOfLegsEncoded),
            );
        }
    }
}

#[test]
fn error_data_size() {
    if let Err(e) = Bcbp::from("") {
        assert!(e == Error::MandatoryDataSize);
    }

    if let Err(e) = Bcbp::from("M1BRUNER/ROMAN MR     EJNUFFX MUCSVOSU 2327 231L013A0052 1FF") {
        assert!(e == Error::CoditionalDataSize);
    }
}

// Minimal
#[test]
fn minimal() {
    use itertools::iproduct;

    for name in samples::NAMES {
        for pnr in samples::PNRS {
            let mut src = Vec::with_capacity(60);

            src.extend(b"M1");
            src.extend(name.fullname);
            src.push(b' ');
            src.extend(**pnr);
            src.extend(std::iter::repeat(b' ').take(28));
            src.extend(b"00");

            assert_eq!(src.len(), 60);
            assert!(src.is_ascii());

            let src = from_utf8(&src).unwrap();
            let tmp = Bcbp::from(src);

            //println!("RES {tmp:#?}");

            assert!(tmp.is_ok());
            let bcbp = tmp.unwrap();

            assert_eq!(bcbp.name(),        from_utf8(name.fullname).unwrap().trim());
            assert_eq!(bcbp.name_last,     from_utf8(name.lastname).unwrap());
            assert_eq!(bcbp.name_first.as_deref(),    name.firstname.map(|x| from_utf8(x).unwrap()));
            assert_eq!(bcbp.ticket_flag,   None);
            assert_eq!(bcbp.version,       None);

            assert_eq!(bcbp.legs[0].pnr(),           Some(from_utf8(*pnr).unwrap().trim()));
            assert_eq!(bcbp.legs[0].src_airport(),   None);
            assert_eq!(bcbp.legs[0].dst_airport(),   None);
            assert_eq!(bcbp.legs[0].airline(),       None);
            assert_eq!(bcbp.legs[0].flight_number(), None);
            assert_eq!(bcbp.legs[0].flight_day,      None);
            assert_eq!(bcbp.legs[0].compartment,     None);
            assert_eq!(bcbp.legs[0].seat(),          None);
            assert_eq!(bcbp.legs[0].sequence,        None);
            assert_eq!(bcbp.legs[0].pax_status,      PaxStatus::None);

            assert_eq!(bcbp.build(Mode::Tolerant).unwrap(), src);
        }
    }
}

#[test]
fn mandatory_legs() {
    for leg_count in 1..=9 {
        println!("leg_count: {leg_count}");
        for name in samples::NAMES {
            for _ in 0..1000 {
                let mut src = Vec::new();

                src.extend(b"M");
                src.extend(format!("{leg_count}").as_bytes());
                src.extend(name.fullname);
                src.extend(b"E");

                let mut legs = Vec::with_capacity(leg_count);
                for _ in 0..leg_count {
                    let (leg, stream) = samples::leg();
                    src.extend(stream);

                    legs.push(leg);
                }

                assert_eq!(src.len(), 23 + leg_count * 37);

                let src = String::from_utf8(src).unwrap();
                let bcbp = Bcbp::from(src.as_str()).expect("Expected the parsing process to succeed");

                //println!("RES {bcbp:#?}");
                //println!("LEGS {legs:#?}");

                assert_eq!(bcbp.name(),        from_utf8(name.fullname).unwrap().trim());
                assert_eq!(bcbp.name_last,     from_utf8(name.lastname).unwrap());
                assert_eq!(bcbp.name_first.as_deref(),    name.firstname.map(|x| from_utf8(x).unwrap()));
                assert_eq!(bcbp.ticket_flag,   Some('E'));
                assert_eq!(bcbp.version,       None);

                for i in 0..leg_count {
                    assert_eq!(bcbp.legs[i].pnr(),            legs[i].pnr());
                    assert_eq!(bcbp.legs[i].src_airport(),    legs[i].src_airport());
                    assert_eq!(bcbp.legs[i].dst_airport(),    legs[i].dst_airport());
                    assert_eq!(bcbp.legs[i].airline(),        legs[i].airline());
                    assert_eq!(bcbp.legs[i].flight_number(),  legs[i].flight_number());
                    assert_eq!(bcbp.legs[i].flight_day,       legs[i].flight_day);
                    assert_eq!(bcbp.legs[i].compartment,      legs[i].compartment);
                    assert_eq!(bcbp.legs[i].seat(),           legs[i].seat());
                    assert_eq!(bcbp.legs[i].sequence,         legs[i].sequence);
                    assert_eq!(bcbp.legs[i].pax_status,       legs[i].pax_status);
                }

                assert_eq!(bcbp.build(Mode::Tolerant).expect("Building shouldn't fail"), src);
            }
        }
    }
}

// B.1.1 LH Home Printed Boarding Pass
#[test]
fn home_printed_1_1() {
    let src = "M1TEST/HIDDEN         E8OQ6FU FRARLGLH 4010 012C004D0001 35C>2180WW6012BLH              2922023642241060 LH                        *30600000K09         ";
    let tmp = Bcbp::from(src);

    println!("RES {:#?}", tmp);

    assert!(tmp.is_ok());

    let bcbp = tmp.unwrap();

    assert_eq!(bcbp.name(),      "TEST/HIDDEN");
    assert_eq!(bcbp.name_last,   "TEST");
    assert_eq!(bcbp.name_first,  Some("HIDDEN".to_string()));
    assert_eq!(bcbp.ticket_flag, Some('E'));
    assert_eq!(bcbp.version,     Some('2'));
    assert_eq!(bcbp.pax_type,    PaxType::Adult);

    assert_eq!(bcbp.checkin_src,                     Some('W'));
    assert_eq!(bcbp.boardingpass_src,                Some('W'));
    assert_eq!(bcbp.boardingpass_issued,             Some(6012));
    assert_eq!(bcbp.doc_type,                        Some('B'));
    assert_eq!(bcbp.boardingpass_airline.as_deref(), Some("LH"));

    assert_eq!(bcbp.legs[0].pnr().as_deref(),           Some("8OQ6FU"));
    assert_eq!(bcbp.legs[0].src_airport().as_deref(),   Some("FRA"));
    assert_eq!(bcbp.legs[0].dst_airport().as_deref(),   Some("RLG"));
    assert_eq!(bcbp.legs[0].airline().as_deref(),       Some("LH"));
    assert_eq!(bcbp.legs[0].flight_number().as_deref(), Some("4010"));
    assert_eq!(bcbp.legs[0].flight_day,                 Some(DayOfYear::new(12).unwrap()));
    assert_eq!(bcbp.legs[0].compartment,                Some('C'));
    assert_eq!(bcbp.legs[0].seat().as_deref(),          Some("4D"));
    assert_eq!(bcbp.legs[0].sequence,                   Some(1));
    assert_eq!(bcbp.legs[0].pax_status,                 PaxStatus::Other('3'));
}

// B.1.2 KL – Home Printed Boarding Pass
#[test]
fn home_printed_1_2() {
    let src = "M1TEST/PETER          E24Z5RN AMSBRUKL 1733 019M008A0001 316>503  W0D0742497067621";
    let tmp = Bcbp::from(src);

    println!("RES {:?}", tmp);

    assert!(tmp.is_ok());

    let bcbp = tmp.unwrap();

    assert_eq!(bcbp.name(),                "TEST/PETER");
    assert_eq!(bcbp.name_last,             "TEST");
    assert_eq!(bcbp.name_first.as_deref(), Some("PETER"));
}





// B.2.1 UA - UA kiosk
//   "M1ASKREN/TEST         EA272SL ORDNRTUA 0881 007F002K0303 15C>3180 K6007BUA              2901624760758980 UA UA EY975897            *30600    09  UAG    "


// B.3.1 LH - Lufthansa mobile BCBP
//   "M1TEST/HIDDEN         E8OQ6FU FRARLGLH 4010 012C004D0001 35C>2180WM6012BLH              2922023642241060 LH                        *30600000K09         "

// B.3.2 UA – UA mobile BCBP
//   "M1ASKREN/TEST         EA272SL ORDNRTUA 0881 007F002K0303 15C>3180 M6007BUA              2901624760758980 UA UA EY975897            *30600    09  UAG    ^160MEYCIQCVDy6sskR0zx8Ac5aXCG0hjkejH587woSGHWnbBRbp8QIhAJ790UHbTHG9nZLnllP+JjStGWPLWGR7Ag5on2FPCeRG"


//   Sample of UA smartphone full mobile boarding pass
//     "M1ASKREN/TEST         EA272SL ORDNRTUA 0881 007F002K0303 15C>3180 M6007BUA              2901624760758980 UA UA EY975897            *30600    09  UAG    ^160MEYCIQCVDy6sskR0zx8Ac5aXCG0hjkejH587woSGHWnbBRbp8QIhAJ790UHbTHG9nZLnllP+JjStGWPLWGR7Ag5on2FPCeRG"

//   Sample of UA iPhone Wallet boarding pass
//     "M1ASKREN/TEST         EA272SL ORDNRTUA 0881 007F002K0303 15C>3180 M6007BUA              2901624760758980 UA UA EY975897            *30600    09  UAG    ^160MEUCIQC1k/QcCEoSFjSivLo3RWiD3268l+OLdrFMTbTyMLRSbAIgb4JVCsWKx/h5HP7+sApYU6nwvM/70IKyUrX28SC+b94="

#[test]
fn conditional3() {
    let src = "M3JOHN/SMITH          EABCDEF JFKSVOSK 1234 123M014C0050 35D>5180O 0276BSK              2A55559467513980 SK                         *30600000K09         ABCDEF SVOFRASU 5678 135Y013A0012 3372A55559467513990 SU SU 12345678             09         ABCDEF FRAJFKSU 9876 231Y022F0052 3372A55559467513990 SU SU 12345678             09         ";
    println!("|");
    let tmp = Bcbp::from(src);
    println!("TMP {:#?}", tmp);

    assert!(tmp.is_ok());

    let bcbp = tmp.unwrap();

    assert!(bcbp.name()     == "JOHN/SMITH");
    assert!(bcbp.name_last  == "JOHN");
    assert!(bcbp.name_first == Some("SMITH".to_string()));
    assert!(bcbp.ticket_flag == Some('E'));

    assert_eq!(bcbp.legs[0].pnr(),           Some("ABCDEF"));
    assert_eq!(bcbp.legs[0].src_airport(),   Some("JFK"));
    assert_eq!(bcbp.legs[0].dst_airport(),   Some("SVO"));
    assert_eq!(bcbp.legs[0].airline(),       Some("SK"));
    assert_eq!(bcbp.legs[0].flight_number(), Some("1234"));
    assert_eq!(bcbp.legs[0].flight_day,      Some(DayOfYear::new(123).unwrap()));

    assert_eq!(bcbp.legs[1].pnr(),           Some("ABCDEF"));
    assert_eq!(bcbp.legs[1].src_airport(),   Some("SVO"));
    assert_eq!(bcbp.legs[1].dst_airport(),   Some("FRA"));
    assert_eq!(bcbp.legs[1].airline(),       Some("SU"));
    assert_eq!(bcbp.legs[1].flight_number(), Some("5678"));
    assert_eq!(bcbp.legs[1].flight_day,      Some(DayOfYear::new(135).unwrap()));

    assert_eq!(bcbp.legs[2].pnr(),           Some("ABCDEF"));
    assert_eq!(bcbp.legs[2].src_airport(),   Some("FRA"));
    assert_eq!(bcbp.legs[2].dst_airport(),   Some("JFK"));
    assert_eq!(bcbp.legs[2].airline(),       Some("SU"));
    assert_eq!(bcbp.legs[2].flight_number(), Some("9876"));
    assert_eq!(bcbp.legs[2].flight_day,      Some(DayOfYear::new(231).unwrap()));
}
