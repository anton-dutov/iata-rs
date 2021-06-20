use iata::bcbp::*;
use iata::datetime::DayOfYear;

#[test]
fn errors() {

    if let Err(e) = Bcbp::from("") {
        assert!(e == Error::MandatoryDataSize);
    }

    if let Err(e) = Bcbp::from("X1BRUNER/ROMAN MR     EJNUFFX MUCSVOSU 2327 231L013A0052 100") {
        assert!(e == Error::InvalidFormatCode('X'));
    }

    if let Err(e) = Bcbp::from("M0BRUNER/ROMAN MR     EJNUFFX MUCSVOSU 2327 231L013A0052 100") {
        assert!(e == Error::InvalidLegsCount);
    }

    if let Err(e) = Bcbp::from("MABRUNER/ROMAN MR     EJNUFFX MUCSVOSU 2327 231L013A0052 100") {
        assert!(e == Error::ExpectedInteger(field::Field::NumberOfLegsEncoded));
    }

    if let Err(e) = Bcbp::from("M1BRUNER/ROMAN MR     EJNUFFX MUCSVOSU 2327 231L013A0052 1FF") {
        assert!(e == Error::CoditionalDataSize);
    }

        // println!("{:?}", Bcbp::from("M1BRUNER/ROMAN MR     EJNUFFX MUCSVOSU 2327 231L013A0052 100"));
        // assert!(false);

}

// Minimal
#[test]
fn minimal() {
    let src = "M1TEST                 8OQ6FU                             00";
    let tmp = Bcbp::from(src);

    println!("RES {:#?}", tmp);

    assert!(tmp.is_ok());

    let bcbp = tmp.unwrap();
    let res  = bcbp.build(Mode::Tolerant).unwrap();

    assert_eq!(bcbp.name(),        "TEST");
    assert_eq!(bcbp.name_last,     "TEST");
    assert_eq!(bcbp.name_first,    None);
    assert_eq!(bcbp.ticket_flag,   None);
    assert_eq!(bcbp.version,       None);

    assert_eq!(bcbp.legs[0].pnr.as_deref(),           Some("8OQ6FU"));
    assert_eq!(bcbp.legs[0].src_airport.as_deref(),   None);
    assert_eq!(bcbp.legs[0].dst_airport.as_deref(),   None);
    assert_eq!(bcbp.legs[0].airline.as_deref(),       None);
    assert_eq!(bcbp.legs[0].flight_number.as_deref(), None);
    assert_eq!(bcbp.legs[0].flight_day,               None);
    assert_eq!(bcbp.legs[0].compartment,              None);
    assert_eq!(bcbp.legs[0].seat,                     None);
    assert_eq!(bcbp.legs[0].sequence,                 None);
    assert_eq!(bcbp.legs[0].pax_status,               PaxStatus::None);

    assert_eq!(src, res);
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

    assert_eq!(bcbp.legs[0].pnr.as_deref(),           Some("8OQ6FU"));
    assert_eq!(bcbp.legs[0].src_airport.as_deref(),   Some("FRA"));
    assert_eq!(bcbp.legs[0].dst_airport.as_deref(),   Some("RLG"));
    assert_eq!(bcbp.legs[0].airline.as_deref(),       Some("LH"));
    assert_eq!(bcbp.legs[0].flight_number.as_deref(), Some("4010"));
    assert_eq!(bcbp.legs[0].flight_day,               Some(DayOfYear::new(12).unwrap()));
    assert_eq!(bcbp.legs[0].compartment,              Some('C'));
    assert_eq!(bcbp.legs[0].seat.as_deref(),          Some("4D"));
    assert_eq!(bcbp.legs[0].sequence,                 Some(1));
    assert_eq!(bcbp.legs[0].pax_status,               PaxStatus::Other('3'));
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
fn mandatory1() {
    let src = "M1JOHN/SMITH JORDAN   EABCDEF JFKSVOSU 1234A001Y001Z0007 000";
    let tmp = Bcbp::from(src);

    println!("RES {:?}", tmp);

    assert!(tmp.is_ok());

    let bcbp = tmp.unwrap();
    let res  = bcbp.build(Mode::Tolerant).unwrap();

    println!("{:?}", src);
    println!("{:?}", res);

    assert!(bcbp.name()      == "JOHN/SMITH JORDAN");
    assert!(bcbp.name_last   == "JOHN");
    assert!(bcbp.name_first  == Some("SMITH JORDAN".to_string()));
    assert!(bcbp.ticket_flag == Some('E'));

    assert_eq!(bcbp.legs[0].pnr.as_deref(),           Some("ABCDEF"));
    assert_eq!(bcbp.legs[0].src_airport.as_deref(),   Some("JFK"));
    assert_eq!(bcbp.legs[0].dst_airport.as_deref(),   Some("SVO"));
    assert_eq!(bcbp.legs[0].airline.as_deref(),       Some("SU"));
    assert_eq!(bcbp.legs[0].flight_number.as_deref(), Some("1234A"));
    assert_eq!(bcbp.legs[0].flight_day,               Some(DayOfYear::new(1).unwrap()));
    assert_eq!(bcbp.legs[0].compartment,              Some('Y'));
    assert_eq!(bcbp.legs[0].seat.as_deref(),          Some("1Z"));
    assert_eq!(bcbp.legs[0].sequence,                 Some(7));
    assert_eq!(bcbp.legs[0].pax_status,               PaxStatus::NotCheckedIn);

    assert_eq!(src, res);
}

#[test]
fn mandatory4() {
    let src = "M4VERYLONGESTLASTNAMEDEABCDEF JFKSVOSU 1234 207          000ABCDEF SVOLEDSU 5678 210          000ABCDEF LEDSVOSU 9876 215          000ABCDEF SVOJFKSU 1357 215          000";
    let tmp = Bcbp::from(src);

    assert!(tmp.is_ok());

    let bcbp = tmp.unwrap();
    let res  = bcbp.build(Mode::Tolerant).unwrap();

    println!("{:?}", src);
    println!("{:?}", res);

    assert_eq!(bcbp.name(),      "VERYLONGESTLASTNAMED");
    assert_eq!(bcbp.name_last,   "VERYLONGESTLASTNAMED");
    assert_eq!(bcbp.name_first,  None);
    assert_eq!(bcbp.ticket_flag, Some('E'));

    assert_eq!(bcbp.legs[0].pnr.as_deref(),           Some("ABCDEF"));
    assert_eq!(bcbp.legs[0].src_airport.as_deref(),   Some("JFK"));
    assert_eq!(bcbp.legs[0].dst_airport.as_deref(),   Some("SVO"));
    assert_eq!(bcbp.legs[0].airline.as_deref(),       Some("SU"));
    assert_eq!(bcbp.legs[0].flight_number.as_deref(), Some("1234"));
    assert_eq!(bcbp.legs[0].flight_day,               Some(DayOfYear::new(207).unwrap()));

    assert_eq!(bcbp.legs[1].pnr.as_deref(),           Some("ABCDEF"));
    assert_eq!(bcbp.legs[1].src_airport.as_deref(),   Some("SVO"));
    assert_eq!(bcbp.legs[1].dst_airport.as_deref(),   Some("LED"));
    assert_eq!(bcbp.legs[1].airline.as_deref(),       Some("SU"));
    assert_eq!(bcbp.legs[1].flight_number.as_deref(), Some("5678"));
    assert_eq!(bcbp.legs[1].flight_day,               Some(DayOfYear::new(210).unwrap()));

    assert_eq!(bcbp.legs[2].pnr.as_deref(),           Some("ABCDEF"));
    assert_eq!(bcbp.legs[2].src_airport.as_deref(),   Some("LED"));
    assert_eq!(bcbp.legs[2].dst_airport.as_deref(),   Some("SVO"));
    assert_eq!(bcbp.legs[2].airline.as_deref(),       Some("SU"));
    assert_eq!(bcbp.legs[2].flight_number.as_deref(), Some("9876"));
    assert_eq!(bcbp.legs[2].flight_day,               Some(DayOfYear::new(215).unwrap()));

    assert_eq!(bcbp.legs[3].pnr.as_deref(),           Some("ABCDEF"));
    assert_eq!(bcbp.legs[3].src_airport.as_deref(),   Some("SVO"));
    assert_eq!(bcbp.legs[3].dst_airport.as_deref(),   Some("JFK"));
    assert_eq!(bcbp.legs[3].airline.as_deref(),       Some("SU"));
    assert_eq!(bcbp.legs[3].flight_number.as_deref(), Some("1357"));
    assert_eq!(bcbp.legs[3].flight_day,               Some(DayOfYear::new(215).unwrap()));

    assert_eq!(src, res);
}

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

    assert_eq!(bcbp.legs[0].pnr.as_deref(),           Some("ABCDEF"));
    assert_eq!(bcbp.legs[0].src_airport.as_deref(),   Some("JFK"));
    assert_eq!(bcbp.legs[0].dst_airport.as_deref(),   Some("SVO"));
    assert_eq!(bcbp.legs[0].airline.as_deref(),       Some("SK"));
    assert_eq!(bcbp.legs[0].flight_number.as_deref(), Some("1234"));
    assert_eq!(bcbp.legs[0].flight_day,               Some(DayOfYear::new(123).unwrap()));

    assert_eq!(bcbp.legs[1].pnr.as_deref(),           Some("ABCDEF"));
    assert_eq!(bcbp.legs[1].src_airport.as_deref(),   Some("SVO"));
    assert_eq!(bcbp.legs[1].dst_airport.as_deref(),   Some("FRA"));
    assert_eq!(bcbp.legs[1].airline.as_deref(),       Some("SU"));
    assert_eq!(bcbp.legs[1].flight_number.as_deref(), Some("5678"));
    assert_eq!(bcbp.legs[1].flight_day,               Some(DayOfYear::new(135).unwrap()));

    assert_eq!(bcbp.legs[2].pnr.as_deref(),           Some("ABCDEF"));
    assert_eq!(bcbp.legs[2].src_airport.as_deref(),   Some("FRA"));
    assert_eq!(bcbp.legs[2].dst_airport.as_deref(),   Some("JFK"));
    assert_eq!(bcbp.legs[2].airline.as_deref(),       Some("SU"));
    assert_eq!(bcbp.legs[2].flight_number.as_deref(), Some("9876"));
    assert_eq!(bcbp.legs[2].flight_day,               Some(DayOfYear::new(231).unwrap()));
}

#[test]
fn surname_with_space() {
    let src = "M1IVANOVA VASILINA/   EABCDEF SVOLEDSU 0036 315YNS  0049 362>5324OO7314BSU                                        2A5551993799397 1                          N";
    let tmp = Bcbp::from(src);

    assert!(tmp.is_ok());

    println!("TMP {:?}", tmp);

    let bcbp = tmp.unwrap();
    let res  = bcbp.build(Mode::Tolerant).unwrap();

    println!("{:?}", src);
    println!("{:?}", res);

    assert!(bcbp.name()          == "IVANOVA VASILINA/");
    assert!(bcbp.name_last     == "IVANOVA VASILINA");
    assert!(bcbp.name_first    == Some("".to_string()));
    assert_eq!(bcbp.ticket_flag, Some('E'));

    assert_eq!(bcbp.legs[0].pnr.as_deref(),           Some("ABCDEF"));
    assert_eq!(bcbp.legs[0].src_airport.as_deref(),   Some("SVO"));
    assert_eq!(bcbp.legs[0].dst_airport.as_deref(),   Some("LED"));
    assert_eq!(bcbp.legs[0].airline.as_deref(),       Some("SU"));
    assert_eq!(bcbp.legs[0].flight_number.as_deref(), Some("0036"));
    assert_eq!(bcbp.legs[0].flight_day,               Some(DayOfYear::new(315).unwrap()));
    assert_eq!(bcbp.legs[0].seat.as_deref(),          Some("NS"));
    assert_eq!(bcbp.legs[0].sequence,                 Some(49));
    assert_eq!(bcbp.legs[0].pax_status,               PaxStatus::Other('3'));

    assert_eq!(bcbp.version,                         Some('5'));
    assert_eq!(bcbp.pax_type,                        PaxType::Infant);
    assert_eq!(bcbp.checkin_src,                     Some('O'));
    assert_eq!(bcbp.boardingpass_src,                Some('O'));
    assert_eq!(bcbp.boardingpass_issued,             Some(7314));
    assert_eq!(bcbp.boardingpass_airline.as_deref(), Some("SU"));
    assert_eq!(bcbp.doc_type,                        Some('B'));

//    assert!(src == res);
}
