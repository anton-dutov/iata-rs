use iata::{bcbp::*, datetime::DayOfYear};
use rand::rand_core::le;

const BCBP_BLANK: &str = "M1                                                       000";
const BCBP_MINIMAL: &str = "M1DUTOV/ANTON         EABCDEFGSRCDSTAFL1234A123Y999A9876 100";
const BCBP_1LEG: &str = "M1                     ABCDEFG                           000";
const BCBP_2LEG: &str = "M2                     ABCDEFG                           000";
const BCBP_3LEG: &str = "M3                     ABCDEFG                           000";
const BCBP_4LEG: &str = "M4                     ABCDEFG                           000";
const BCBP_5LEG: &str = "M5                     ABCDEFG                           000";
const BCBP_6LEG: &str = "M6                     ABCDEFG                           000";
const BCBP_7LEG: &str = "M7                     ABCDEFG                           000";
const BCBP_8LEG: &str = "M8                     ABCDEFG                           000";
const BCBP_9LEG: &str = "M9                     ABCDEFG                           000";

// const PASS_STR: &str = "M1MROZ/MARTIN         EXXXXXX SJCLAXAS 3317 207U001A0006 34D>218 VV8207BAS              2502771980993865 AS AS XXXXX55200000000Z29  00010";

#[test]
fn decode_bcbp_blank() {
    let bcbp = Bcbp::decode_bcbp(BCBP_BLANK).unwrap();

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

    assert_eq!(bcbp.encode_bcbp().unwrap(), BCBP_BLANK);
}

#[test]
fn encode_bcbp_blank() {
    let mut bcbp = Bcbp::default();

    bcbp.add_leg(Leg::default()).ok();

    assert_eq!(bcbp.encode_bcbp().unwrap(), BCBP_BLANK);
}

#[test]
fn decode_bcbp_minimal() {
    let bcbp = Bcbp::decode_bcbp(BCBP_MINIMAL).unwrap();

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

    assert_eq!(bcbp.encode_bcbp().unwrap(), BCBP_MINIMAL);
}

#[test]
fn encode_bcbp_minimal() {
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

    assert_eq!(bcbp.encode_bcbp().unwrap(), BCBP_MINIMAL);
}

// #[test]
// fn encode_bcbp_1leg() {
//     let mut bcbp = Bcbp::default();

//     let mut leg = Leg::default();
//     leg.set_pax_status(PaxStatus::CheckedIn);
//     leg.set_pnr(Some("ABCDEFG")).unwrap();
//     leg.set_src_airport(Some("SRC")).unwrap();
//     leg.set_dst_airport(Some("DST")).unwrap();
//     leg.set_airline(Some("AFL")).unwrap();
//     leg.set_flight_number(Some("1234A")).unwrap();
//     leg.set_seat(Some("999A")).unwrap();
//     leg.set_fast_track(Some('Y')).unwrap();
//     leg.set_variable_data(Some("VD")).unwrap();

//     bcbp.add_leg(leg).ok();

//     assert_eq!(bcbp.encode_bcbp().unwrap(), BCBP_1LEG);
// }

// #[test]
// fn encode_bcbp_2leg() {
//     let mut bcbp = Bcbp::default();

//     let mut leg = Leg::default();

//     leg.set_pnr(Some("ABCDEFG")).ok();
//     // leg.set_src_airport(Some("SRC")).ok();
//     // leg.set_dst_airport(Some("DST")).ok();
//     // leg.set_airline(Some("AFL")).ok();

//     bcbp.add_leg(leg).ok();

//     assert_eq!(bcbp.encode_bcbp().unwrap(), BCBP_2LEG);
// }

// #[test]
// fn encode_bcbp_3leg() {
//     let mut bcbp = Bcbp::default();

//     let mut leg = Leg::default();

//     leg.set_pnr(Some("ABCDEFG")).ok();
//     // leg.set_src_airport(Some("SRC")).ok();
//     // leg.set_dst_airport(Some("DST")).ok();
//     // leg.set_airline(Some("AFL")).ok();

//     bcbp.add_leg(leg).ok();

//     assert_eq!(bcbp.encode_bcbp().unwrap(), BCBP_3LEG);
// }

// #[test]
// fn encode_bcbp_4leg() {
//     let mut bcbp = Bcbp::default();

//     let mut leg = Leg::default();

//     leg.set_pnr(Some("ABCDEFG")).ok();
//     // leg.set_src_airport(Some("SRC")).ok();
//     // leg.set_dst_airport(Some("DST")).ok();
//     // leg.set_airline(Some("AFL")).ok();

//     bcbp.add_leg(leg).ok();

//     assert_eq!(bcbp.encode_bcbp().unwrap(), BCBP_4LEG);
// }

// #[test]
// fn encode_bcbp_5leg() {
//     let mut bcbp = Bcbp::default();

//     let mut leg = Leg::default();

//     leg.set_pnr(Some("ABCDEFG")).ok();
//     // leg.set_src_airport(Some("SRC")).ok();
//     // leg.set_dst_airport(Some("DST")).ok();
//     // leg.set_airline(Some("AFL")).ok();

//     bcbp.add_leg(leg).ok();

//     assert_eq!(bcbp.encode_bcbp().unwrap(), BCBP_5LEG);
// }

// #[test]
// fn encode_bcbp_6leg() {
//     let mut bcbp = Bcbp::default();

//     let mut leg = Leg::default();

//     leg.set_pnr(Some("ABCDEFG")).ok();
//     // leg.set_src_airport(Some("SRC")).ok();
//     // leg.set_dst_airport(Some("DST")).ok();
//     // leg.set_airline(Some("AFL")).ok();

//     bcbp.add_leg(leg).ok();

//     assert_eq!(bcbp.encode_bcbp().unwrap(), BCBP_6LEG);
// }

// #[test]
// fn encode_bcbp_7leg() {
//     let mut bcbp = Bcbp::default();

//     let mut leg = Leg::default();

//     leg.set_pnr(Some("ABCDEFG")).ok();
//     // leg.set_src_airport(Some("SRC")).ok();
//     // leg.set_dst_airport(Some("DST")).ok();
//     // leg.set_airline(Some("AFL")).ok();

//     bcbp.add_leg(leg).ok();

//     assert_eq!(bcbp.encode_bcbp().unwrap(), BCBP_7LEG);
// }

// #[test]
// fn encode_bcbp_8leg() {
//     let mut bcbp = Bcbp::default();

//     let mut leg = Leg::default();

//     leg.set_pnr(Some("ABCDEFG")).ok();
//     // leg.set_src_airport(Some("SRC")).ok();
//     // leg.set_dst_airport(Some("DST")).ok();
//     // leg.set_airline(Some("AFL")).ok();

//     bcbp.add_leg(leg).ok();

//     assert_eq!(bcbp.encode_bcbp().unwrap(), BCBP_8LEG);
// }

// #[test]
// fn encode_bcbp_9leg() {
//     let mut bcbp = Bcbp::default();

//     let mut leg = Leg::default();

//     leg.set_pnr(Some("ABCDEFG")).ok();
//     // leg.set_src_airport(Some("SRC")).ok();
//     // leg.set_dst_airport(Some("DST")).ok();
//     // leg.set_airline(Some("AFL")).ok();

//     bcbp.add_leg(leg).ok();

//     assert_eq!(bcbp.encode_bcbp().unwrap(), BCBP_9LEG);
// }
