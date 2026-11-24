#![cfg(feature = "with-bcbp")]

use iata::{bcbp::format::Field, bcbp::*, datetime::DayOfYear};
use serde::Deserialize;

const BCBP_BLANK: &str = include_str!("../test_data/bcbp_blank.json");
const BCBP_IMPL5_SIMPLE: &str = include_str!("../test_data/bcbp_impl5th_simple.json");
const BCBP_MINIMAL: &str = include_str!("../test_data/bcbp_minimal.json");
const BCBP_2LEGS: &str = include_str!("../test_data/bcbp_2legs.json");
const BCBP_3LEGS: &str = include_str!("../test_data/bcbp_3legs.json");

// const PASS_STR: &str = "M1MROZ/MARTIN         EXXXXXX SJCLAXAS 3317 207U001A0006 34D>218 VV8207BAS              2502771980993865 AS AS XXXXX55200000000Z29  00010";

#[derive(Deserialize)]
struct Data {
    raw: String,
}

#[test]
fn decode_bcbp_blank() {
    let test_data: Data = serde_json::from_str(BCBP_BLANK).unwrap();

    let bcbp = Bcbp::decode_bcbp(&test_data.raw).unwrap();

    assert_eq!(bcbp.legs_count(), 1);
    assert_eq!(bcbp.passenger_name(), "");
    assert_eq!(bcbp.eticket_indicator(), None);
    let leg = bcbp.leg(0);
    assert_eq!(leg.pnr(), None);
    assert_eq!(leg.src_airport(), None);
    assert_eq!(leg.dst_airport(), None);
    assert_eq!(leg.operating_carrier(), None);
    assert_eq!(leg.flight_number(), None);
    assert_eq!(leg.flight_day(), None);
    assert_eq!(leg.compartment(), None);
    assert_eq!(leg.seat(), None);
    assert_eq!(leg.checkin_sequence(), None);
    assert_eq!(leg.pax_status(), PaxStatus::NotCheckedIn);

    assert_eq!(bcbp.encode_bcbp().unwrap(), test_data.raw);
}

#[test]
fn encode_bcbp_blank() {
    let test_data: Data = serde_json::from_str(BCBP_BLANK).unwrap();

    let mut bcbp = Bcbp::default();

    bcbp.add_leg(Leg::default()).ok();

    assert_eq!(bcbp.encode_bcbp().unwrap(), test_data.raw);
}

#[test]
fn decode_bcbp_impl5th_simple() {
    let test_data: Data = serde_json::from_str(BCBP_IMPL5_SIMPLE).unwrap();

    let bcbp = Bcbp::decode_bcbp(&test_data.raw).unwrap();

    assert_eq!(bcbp.legs_count(), 1);
    assert_eq!(bcbp.passenger_name(), "DESMARAIS/LUC");
    assert_eq!(bcbp.eticket_indicator(), Some('E'));
    let leg = bcbp.leg(0);
    assert_eq!(leg.pnr(), Some("ABC123"));
    assert_eq!(leg.src_airport(), Some("YUL"));
    assert_eq!(leg.dst_airport(), Some("FRA"));
    assert_eq!(leg.operating_carrier(), Some("AC"));
    assert_eq!(leg.flight_number(), Some("0834"));
    assert_eq!(leg.flight_day(), Some(DayOfYear::new(226).unwrap()));
    assert_eq!(leg.compartment(), Some('F'));
    assert_eq!(leg.seat(), Some("1A"));
    assert_eq!(leg.checkin_sequence(), Some(25));
    assert_eq!(leg.pax_status(), PaxStatus::CheckedIn);

    assert_eq!(bcbp.encode_bcbp().unwrap(), test_data.raw);
}

#[test]
fn decode_bcbp_minimal() {
    let test_data: Data = serde_json::from_str(BCBP_MINIMAL).unwrap();

    let bcbp = Bcbp::decode_bcbp(&test_data.raw).unwrap();

    assert_eq!(bcbp.legs_count(), 1);
    assert_eq!(bcbp.passenger_name(), "DUTOV/ANTON");
    assert_eq!(bcbp.eticket_indicator(), Some('E'));
    let leg = bcbp.leg(0);
    assert_eq!(leg.pnr(), Some("ABCDEFG"));
    assert_eq!(leg.src_airport(), Some("SRC"));
    assert_eq!(leg.dst_airport(), Some("DST"));
    assert_eq!(leg.operating_carrier(), Some("AFL"));
    assert_eq!(leg.flight_number(), Some("1234A"));
    assert_eq!(leg.flight_day(), Some(DayOfYear::new(123).unwrap()));
    assert_eq!(leg.compartment(), Some('Y'));
    assert_eq!(leg.seat(), Some("999A"));
    assert_eq!(leg.checkin_sequence(), Some(9876));
    assert_eq!(leg.pax_status(), PaxStatus::CheckedIn);

    assert_eq!(bcbp.encode_bcbp().unwrap(), test_data.raw);
}

#[test]
fn encode_bcbp_minimal() {
    let test_data: Data = serde_json::from_str(BCBP_MINIMAL).unwrap();

    let mut bcbp = Bcbp::default();

    bcbp.set_passenger_name("DUTOV/ANTON").unwrap();
    bcbp.set_eticket_indicator(Some('E')).unwrap();

    let mut leg = Leg::default();

    leg.set_pax_status(PaxStatus::CheckedIn).unwrap();
    leg.set_pnr(Some("ABCDEFG")).unwrap();
    leg.set_src_airport(Some("SRC")).unwrap();
    leg.set_dst_airport(Some("DST")).unwrap();
    leg.set_operating_carrier(Some("AFL")).unwrap();
    leg.set_flight_number(Some("1234A")).unwrap();
    leg.set_flight_day(Some(DayOfYear::new(123).unwrap()))
        .unwrap();
    leg.set_compartment(Some('Y')).unwrap();
    leg.set_seat(Some("999A")).unwrap();
    leg.set_checkin_sequence(Some(9876)).unwrap();

    bcbp.add_leg(leg).ok();

    assert_eq!(bcbp.encode_bcbp().unwrap(), test_data.raw);
}

#[test]
fn roundtrip_empty_legs() {
    for count in [1, 2, 9] {
        let mut pass = Bcbp::default();
        for _ in 0..count {
            pass.add_leg(Leg::default()).unwrap();
        }
        let raw = pass.encode_bcbp().unwrap();
        assert_eq!(
            view::BcbpView::decode_bcbp(&raw).unwrap().legs().len(),
            count
        );
        let decoded = Bcbp::decode_bcbp(&raw).unwrap();
        assert_eq!(decoded.legs_count(), count);
        assert_eq!(decoded.encode_bcbp().unwrap(), raw);
    }
}

#[test]
fn roundtrip_nonconsecutive_bagtags() {
    for (first, second) in [
        (Some("0123456789001"), None),
        (None, Some("0123456789002")),
        (Some("0123456789001"), Some("0123456789002")),
    ] {
        let mut pass = Bcbp::default();
        let mut leg = Leg::default();
        leg.set_airline_num(Some(123)).unwrap();
        pass.add_leg(leg).unwrap();
        pass.set_nonconsecutive_bagtag1(first).unwrap();
        pass.set_nonconsecutive_bagtag2(second).unwrap();
        let decoded = Bcbp::decode_bcbp(pass.encode_bcbp().unwrap()).unwrap();
        assert_eq!(decoded.nonconsecutive_bagtag1(), first);
        assert_eq!(decoded.nonconsecutive_bagtag2(), second);
    }
}

#[test]
fn roundtrip_variable_section_size_limit() {
    let mut bcbp = Bcbp::default();
    let mut leg = Leg::default();
    leg.set_variable_data(Some("X")).unwrap();
    bcbp.add_leg(leg).unwrap();
    let raw = bcbp.encode_bcbp().unwrap();
    let overhead = raw.len() - 60 - 1;
    let payload = "X".repeat(255 - overhead);
    bcbp.legs_mut()
        .next()
        .unwrap()
        .set_variable_data(Some(&payload))
        .unwrap();
    let decoded = Bcbp::decode_bcbp(bcbp.encode_bcbp().unwrap()).unwrap();
    assert_eq!(decoded.leg(0).variable_data(), Some(payload.as_str()));
    bcbp.legs_mut()
        .next()
        .unwrap()
        .set_variable_data(Some(&(payload + "X")))
        .unwrap();
    assert!(matches!(
        bcbp.encode_bcbp(),
        Err(Error::FieldLengthExceeded {
            field: Field::FieldSizeOfVariableSizeField,
            max: 255
        })
    ));
}

#[test]
fn parse_view_and_convert_to_owned() {
    let data: Data = serde_json::from_str(BCBP_2LEGS).unwrap();
    let view = view::BcbpView::parse(&data.raw).unwrap();
    assert_eq!(view.version(), Some('5'));
    assert_eq!(view.legs()[0].seat(), "007H");

    let owned = Bcbp::try_from(view).unwrap();
    assert_eq!(owned.version(), Some(Version::Value(5)));
    assert_eq!(owned.leg(0).seat(), Some("7H"));
    assert_eq!(owned.encode().unwrap(), data.raw);
    assert_eq!(
        data.raw.parse::<Bcbp>().unwrap().encode().unwrap(),
        data.raw
    );

    let invalid_day = data.raw.replacen("204J", "367J", 1);
    let view = view::BcbpView::parse(&invalid_day).unwrap();
    assert!(matches!(Bcbp::try_from(view), Err(Error::DateTimeErro(_))));
    assert!(matches!(
        invalid_day.parse::<Bcbp>(),
        Err(Error::DateTimeErro(_))
    ));
}

#[test]
fn convert_optional_security_section() {
    let data: Data = serde_json::from_str(BCBP_MINIMAL).unwrap();
    assert!(data.raw.parse::<Bcbp>().unwrap().security_data().is_none());
    for (suffix, body) in [("^100", ""), ("^103", "abc")] {
        let raw = format!("{}{suffix}{body}", data.raw);
        let view = view::BcbpView::parse(&raw).unwrap();
        assert_eq!(
            view.security_data().data(),
            (!body.is_empty()).then_some(body)
        );
        let pass = Bcbp::try_from(view).unwrap();
        assert_eq!(pass.security_data().unwrap().data, body);
        assert_eq!(pass.encode().unwrap(), raw);
    }
    let raw = format!("{}^1", data.raw);
    let view = view::BcbpView::parse(&raw).unwrap();
    assert!(matches!(
        Bcbp::try_from(view),
        Err(Error::UnexpectedEndOfInput(Field::LengthOfSecurityData))
    ));
}

#[test]
fn convert_airline_data_following_empty_repeated_section() {
    let data: Data = serde_json::from_str(BCBP_MINIMAL).unwrap();
    let raw = format!("{}09>60000abc", &data.raw[..58]);
    let view = view::BcbpView::parse(&raw).unwrap();
    assert_eq!(view.legs()[0].airline_individual_use(), Some("abc"));
    let pass = Bcbp::try_from(view).unwrap();
    assert_eq!(pass.leg(0).variable_data(), Some("abc"));
}

#[test]
fn is_extended_empty_bcbp_no_panic() {
    let bcbp = Bcbp::default();
    assert!(!bcbp.is_extednded());
}

#[test]
fn boardingpass_airline_len_enforced() {
    let mut bcbp = Bcbp::default();

    assert!(matches!(
        bcbp.set_boardingpass_airline(Some("ABCD")),
        Err(Error::FieldLengthExceeded {
            field: Field::AirlineDesignatorOfBoardingPassIssuer,
            max: 3
        })
    ));

    assert!(bcbp.set_boardingpass_airline(Some("AB")).is_ok());
    assert!(bcbp.set_boardingpass_airline(Some("ABC")).is_ok());
}

#[test]
fn decode_bcbp_2leg() {
    let test_data: Data = serde_json::from_str(BCBP_2LEGS).unwrap();

    let bcbp = Bcbp::decode_bcbp(&test_data.raw).unwrap();

    assert_eq!(bcbp.legs_count(), 2);
    assert_eq!(bcbp.passenger_name(), "VERYLONGLASTNAME/JAN");
    assert_eq!(bcbp.eticket_indicator(), Some('E'));

    let leg1 = bcbp.leg(0);
    assert_eq!(leg1.pnr(), Some("A1B2C3"));
    assert_eq!(leg1.src_airport(), Some("SVO"));
    assert_eq!(leg1.dst_airport(), Some("AUH"));
    assert_eq!(leg1.operating_carrier(), Some("EY"));
    assert_eq!(leg1.flight_number(), Some("9753"));
    assert_eq!(leg1.flight_day(), Some(DayOfYear::new(204).unwrap()));
    assert_eq!(leg1.compartment(), Some('J'));
    assert_eq!(leg1.seat(), Some("7H"));
    assert_eq!(leg1.checkin_sequence(), Some(180));
    assert_eq!(leg1.pax_status(), PaxStatus::BaggageAndPaxCheckedIn);
    assert_eq!(leg1.fast_track(), Some('Y'));
    assert_eq!(leg1.variable_data(), Some("AB"));

    let leg2 = bcbp.leg(1);
    assert_eq!(leg2.pnr(), Some("A1B2C3"));
    assert_eq!(leg2.src_airport(), Some("AUH"));
    assert_eq!(leg2.dst_airport(), Some("TRV"));
    assert_eq!(leg2.operating_carrier(), Some("EY"));
    assert_eq!(leg2.flight_number(), Some("8642"));
    assert_eq!(leg1.flight_day(), Some(DayOfYear::new(204).unwrap()));
    assert_eq!(leg2.compartment(), Some('J'));
    assert_eq!(leg2.seat(), Some("2D"));
    assert_eq!(leg2.checkin_sequence(), Some(70));
    assert_eq!(leg2.pax_status(), PaxStatus::BaggageAndPaxCheckedIn);
    assert_eq!(leg2.fast_track(), Some('N'));
    assert_eq!(leg2.variable_data(), Some("CD"));

    assert_eq!(bcbp.encode_bcbp().unwrap(), test_data.raw);
}

#[test]
fn decode_bcbp_3leg() {
    let test_data: Data = serde_json::from_str(BCBP_3LEGS).unwrap();

    let tmp = Bcbp::decode_bcbp(&test_data.raw);
    println!("TMP {:#?}", tmp);

    assert!(tmp.is_ok());

    let bcbp = tmp.unwrap();

    assert!(bcbp.passenger_name() == "JOHN/SMITH");
    assert!(bcbp.eticket_indicator() == Some('E'));

    assert_eq!(bcbp.leg(0).pnr(), Some("ABCDEF"));
    assert_eq!(bcbp.leg(0).src_airport(), Some("JFK"));
    assert_eq!(bcbp.leg(0).dst_airport(), Some("SVO"));
    assert_eq!(bcbp.leg(0).operating_carrier(), Some("SK"));
    assert_eq!(bcbp.leg(0).flight_number(), Some("1234"));
    assert_eq!(bcbp.leg(0).flight_day(), Some(DayOfYear::new(123).unwrap()));

    assert_eq!(bcbp.leg(1).pnr(), Some("ABCDEF"));
    assert_eq!(bcbp.leg(1).src_airport(), Some("SVO"));
    assert_eq!(bcbp.leg(1).dst_airport(), Some("FRA"));
    assert_eq!(bcbp.leg(1).operating_carrier(), Some("SU"));
    assert_eq!(bcbp.leg(1).flight_number(), Some("5678"));
    assert_eq!(bcbp.leg(1).flight_day(), Some(DayOfYear::new(135).unwrap()));

    assert_eq!(bcbp.leg(2).pnr(), Some("ABCDEF"));
    assert_eq!(bcbp.leg(2).src_airport(), Some("FRA"));
    assert_eq!(bcbp.leg(2).dst_airport(), Some("JFK"));
    assert_eq!(bcbp.leg(2).operating_carrier(), Some("SU"));
    assert_eq!(bcbp.leg(2).flight_number(), Some("9876"));
    assert_eq!(bcbp.leg(2).flight_day(), Some(DayOfYear::new(231).unwrap()));
}
