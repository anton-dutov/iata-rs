#![cfg(feature = "with-bcbp")]

use iata::{bcbp::*, datetime::DayOfYear};
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

    leg.set_pax_status(PaxStatus::CheckedIn);
    leg.set_pnr(Some("ABCDEFG")).unwrap();
    leg.set_src_airport(Some("SRC")).unwrap();
    leg.set_dst_airport(Some("DST")).unwrap();
    leg.set_operating_carrier(Some("AFL")).unwrap();
    leg.set_flight_number(Some("1234A")).unwrap();
    leg.set_flight_day(Some(DayOfYear::new(123).unwrap()));
    leg.set_compartment(Some('Y')).unwrap();
    leg.set_seat(Some("999A")).unwrap();
    leg.set_checkin_sequence(Some(9876)).unwrap();

    bcbp.add_leg(leg).ok();

    assert_eq!(bcbp.encode_bcbp().unwrap(), test_data.raw);
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
