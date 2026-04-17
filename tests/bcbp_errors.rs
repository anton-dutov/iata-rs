#![cfg(feature = "with-bcbp")]

use std::str::from_utf8;

use iata::bcbp::{format::Field, *};
use iata::datetime::DayOfYear;

mod samples {
    use std::str::from_utf8;

    use iata::{
        bcbp::{Leg, PaxStatus},
        datetime::DayOfYear,
    };
    use rand::{
        seq::{IndexedRandom, IteratorRandom},
        Rng,
    };

    pub static BASE_BCBP: &str = "M1BRUNER/ROMAN MR     EJNUFFX MUCSVOSU 2327 231L013A0052 100";

    fn to_utf8(b: &[u8]) -> &str {
        from_utf8(b).unwrap().trim()
    }

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
        b"JNUFFX ", b"8OQ6FU ", b"AGGA   ", b"1234ABC", b"ABC1234", b"1A0D31J",
    ];

    pub static AIRPORTS: &[&[u8; 3]] = &[
        b"AAA", b"AAB", b"AAC", b"AAD", b"AAE", b"AAF", b"GAR", b"GBA", b"GBI", b"AAJ", b"AAK",
        b"AAL", b"NAA", b"NAC", b"AAO", b"CAR", b"CCH", b"JAA", b"JAQ", b"AAU", b"AAV", b"AAX",
        b"AAY", b"AAZ", b"ABA", b"ABB", b"ABC", b"ABD", b"LED", b"FRA",
    ];

    pub static AIRLINES: &[&[u8; 3]] = &[
        b"WOW", b"B  ", b"LH ", b"RU ", b"EN ", b"UK ", b"US ", b"PO ", b"LOL",
    ];

    fn flight_number() -> String {
        let mut rng = rand::rng();

        let val: [u8; 5] = [
            (b'0'..=b'9').choose(&mut rng).unwrap(),
            (b'0'..=b'9').choose(&mut rng).unwrap(),
            (b'0'..=b'9').choose(&mut rng).unwrap(),
            (b'0'..=b'9').choose(&mut rng).unwrap(),
            (b'A'..=b'Z')
                .chain(std::iter::once(b' '))
                .choose(&mut rng)
                .unwrap(),
        ];

        to_utf8(&val).to_string()
    }

    fn compartment() -> u8 {
        let mut rng = rand::rng();

        (b'A'..=b'Z').choose(&mut rng).unwrap()
    }

    fn day_of_year() -> DayOfYear {
        let mut rng = rand::rng();
        let x = (1..=365).choose(&mut rng).unwrap();

        DayOfYear::new(x).unwrap()
    }

    fn seat() -> String {
        let mut rng = rand::rng();

        let val: [u8; 4] = *[
            [
                (b'1'..=b'9').choose(&mut rng).unwrap(),
                (b'1'..=b'9').choose(&mut rng).unwrap(),
                (b'1'..=b'9').choose(&mut rng).unwrap(),
                (b'A'..b'Z').choose(&mut rng).unwrap(),
            ],
            *b"INF ",
        ]
        .choose(&mut rng)
        .unwrap();

        to_utf8(&val).to_string()
    }

    fn pax_status() -> u8 {
        let mut rng = rand::rng();

        (b'0'..=b'9').choose(&mut rng).unwrap()
    }

    fn pnr() -> String {
        let mut rng = rand::rng();

        let val: [u8; 7] = **PNRS.choose(&mut rng).unwrap();

        to_utf8(&val).to_string()
    }

    fn airport() -> String {
        let mut rng = rand::rng();

        let val: [u8; 3] = **AIRPORTS.choose(&mut rng).unwrap();

        to_utf8(&val).to_string()
    }

    fn airline() -> String {
        let mut rng = rand::rng();

        let val: [u8; 3] = **AIRLINES.choose(&mut rng).unwrap();

        to_utf8(&val).to_string()
    }

    pub fn leg() -> (Leg, impl Iterator<Item = u8>) {
        let mut rng = rand::rng();
        let pnr = pnr();
        let src_airport = airport();
        let dst_airport = airport();
        let airline = airline();
        let flight_number = flight_number();
        let flight_day = day_of_year();
        let compartment = compartment();
        let seat = seat();
        let checking_sequence: u16 = rng.random_range(1..=9999);
        let pax_status = pax_status();

        let checkin_sequence_bytes = if checking_sequence < 10000 {
            format!("{checking_sequence:04} ").into_bytes()
        } else {
            checking_sequence.to_string().into_bytes()
        };
        let flight_day_ordinal = flight_day.ordinal();

        let mut leg = Leg::default();
        leg.set_pnr(Some(&pnr)).unwrap();
        leg.set_src_airport(Some(&src_airport)).unwrap();
        leg.set_dst_airport(Some(&dst_airport)).unwrap();
        leg.set_operating_carrier(Some(&airline)).unwrap();
        leg.set_flight_number(Some(&flight_number)).unwrap();
        leg.set_compartment(Some(compartment as char)).unwrap();
        leg.set_seat(Some(&seat)).unwrap();
        leg.set_checkin_sequence(Some(checking_sequence)).unwrap();
        leg.set_flight_day(Some(flight_day));
        leg.set_pax_status(PaxStatus::from(pax_status as char));
        (
            leg,
            std::iter::empty()
                .chain(pnr.into_bytes())
                .chain(src_airport.into_bytes())
                .chain(dst_airport.into_bytes())
                .chain(airline.into_bytes())
                .chain(flight_number.into_bytes())
                .chain(format!("{:03}", flight_day_ordinal).into_bytes())
                .chain(std::iter::once(compartment))
                .chain(seat.into_bytes())
                .chain(checkin_sequence_bytes)
                .chain(std::iter::once(pax_status))
                .chain(*b"00"),
        )
    }
}

fn ascii_bytes() -> impl Iterator<Item = u8> {
    (0..u8::MAX).filter(u8::is_ascii)
}

#[test]
fn basic_parsing() {
    Bcbp::decode_bcbp(samples::BASE_BCBP).expect("Failed to parse a sample BCBP");
}

#[test]
fn error_invalid_format_code() {
    let mut base_bytes = samples::BASE_BCBP.as_bytes().to_owned();

    for b in ascii_bytes() {
        if b == b'M' {
            continue;
        }

        base_bytes[0] = b;

        let s = from_utf8(&base_bytes).unwrap();
        let err = Bcbp::decode_bcbp(s).expect_err("Parsing should have failed");

        assert_eq!(err, Error::InvalidFormatCode(b as char));
    }
}

#[test]
fn error_invalid_legs_count() {
    let mut base_bytes = samples::BASE_BCBP.as_bytes().to_owned();

    for b in ascii_bytes() {
        if ('1'..='9').contains(&(b as char)) {
            continue;
        }

        base_bytes[1] = b;
        let s = from_utf8(&base_bytes).unwrap();
        let err = Bcbp::decode_bcbp(s).expect_err("Parsing should have failed");

        if b == b'0' {
            assert_eq!(err, Error::InvalidNumberOfLegs);
        } else {
            assert_eq!(err, Error::ExpectedInteger(Field::NumberOfLegsEncoded),);
        }
    }
}

#[test]
fn error_data_length() {
    if let Err(e) = Bcbp::decode_bcbp("") {
        assert!(e == Error::BcbpTooShort);
    }

    if let Err(e) =
        Bcbp::decode_bcbp("M1BRUNER/ROMAN MR     EJNUFFX MUCSVOSU 2327 231L013A0052 1FF")
    {
        assert!(e == Error::ConditionalDataLengthMismatch);
    }
}

// Minimal
#[test]
fn minimal() {
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
            let tmp = Bcbp::decode_bcbp(src);

            //println!("RES {tmp:#?}");

            assert!(tmp.is_ok());
            let bcbp = tmp.unwrap();

            assert_eq!(
                bcbp.passenger_name(),
                from_utf8(name.fullname).unwrap().trim()
            );
            assert_eq!(bcbp.eticket_indicator(), None);
            assert_eq!(bcbp.version(), None);

            assert_eq!(bcbp.leg(0).pnr(), Some(from_utf8(*pnr).unwrap().trim()));
            assert_eq!(bcbp.leg(0).src_airport(), None);
            assert_eq!(bcbp.leg(0).dst_airport(), None);
            assert_eq!(bcbp.leg(0).operating_carrier(), None);
            assert_eq!(bcbp.leg(0).flight_number(), None);
            assert_eq!(bcbp.leg(0).flight_day(), None);
            assert_eq!(bcbp.leg(0).compartment(), None);
            assert_eq!(bcbp.leg(0).seat(), None);
            assert_eq!(bcbp.leg(0).checkin_sequence(), None);
            assert_eq!(bcbp.leg(0).pax_status(), PaxStatus::None);

            assert_eq!(bcbp.encode_bcbp().unwrap(), src);
        }
    }
}

// B.1.1 LH Home Printed Boarding Pass
#[test]
fn home_printed_1_1() {
    let src = "M1TEST/HIDDEN         E8OQ6FU FRARLGLH 4010 012C004D0001 35C>2180WW6012BLH              2922023642241060 LH                        *30600000K09         ";
    let tmp = Bcbp::decode_bcbp(src);

    println!("RES {:#?}", tmp);

    assert!(tmp.is_ok());

    let bcbp = tmp.unwrap();

    assert_eq!(bcbp.passenger_name(), "TEST/HIDDEN");
    assert_eq!(bcbp.eticket_indicator(), Some('E'));
    assert_eq!(bcbp.version(), Some(Version::Value(2)));
    assert_eq!(bcbp.pax_kind(), Some(PaxKind::Adult));

    assert_eq!(bcbp.checkin_src(), Some('W'));
    assert_eq!(bcbp.boardingpass_src(), Some('W'));
    assert_eq!(bcbp.boardingpass_issued(), Some(6012));
    assert_eq!(bcbp.doc_type(), Some('B'));
    assert_eq!(bcbp.boardingpass_airline(), Some("LH"));

    assert_eq!(bcbp.leg(0).pnr().as_deref(), Some("8OQ6FU"));
    assert_eq!(bcbp.leg(0).src_airport().as_deref(), Some("FRA"));
    assert_eq!(bcbp.leg(0).dst_airport().as_deref(), Some("RLG"));
    assert_eq!(bcbp.leg(0).operating_carrier().as_deref(), Some("LH"));
    assert_eq!(bcbp.leg(0).flight_number().as_deref(), Some("4010"));
    assert_eq!(bcbp.leg(0).flight_day(), Some(DayOfYear::new(12).unwrap()));
    assert_eq!(bcbp.leg(0).compartment(), Some('C'));
    assert_eq!(bcbp.leg(0).seat().as_deref(), Some("4D"));
    assert_eq!(bcbp.leg(0).checkin_sequence(), Some(1));
    assert_eq!(bcbp.leg(0).pax_status(), PaxStatus::BaggageAndPaxCheckedIn);

    // assert_eq!(bcbp.encode_bcbp().unwrap(), src);
}

// B.1.2 KL – Home Printed Boarding Pass
#[test]
fn home_printed_1_2() {
    let src = "M1TEST/PETER          E24Z5RN AMSBRUKL 1733 019M008A0001 316>503  W0D0742497067621";
    let tmp = Bcbp::decode_bcbp(src);

    println!("RES {:?}", tmp);

    assert!(tmp.is_ok());

    let bcbp = tmp.unwrap();

    assert_eq!(bcbp.passenger_name(), "TEST/PETER");
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
