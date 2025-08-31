use iata::{bcbp::*, datetime::DayOfYear};
const BCBP_MINIMAL: &str = "M1DUTOV/ANTON          ABCDEFGSRCDSTAFL1234A    999A9876 100";
const BCBP_M2: &str = "M2VERYLONGLASTNAME/JANEA1B2C3 SVOAUHEY 9753 204J007H0180 34A>5180 O5203BEY              2A60724100805450                           YABA1B2C3 AUHTRVEY 8642 204J002D0070 32E2A60724100805450                           NCD";

#[test]
fn decode_bcbp_minimal() {
    let bcbp = Bcbp::decode_bcbp(BCBP_MINIMAL).unwrap();

    assert_eq!(bcbp.legs_count(), 1);
    assert_eq!(bcbp.passenger_name(), "DUTOV/ANTON");
    assert_eq!(bcbp.eticket_indicator(), None);

    let leg = bcbp.leg(0);
    assert_eq!(leg.pnr(), Some("ABCDEFG"));
    assert_eq!(leg.src_airport(), Some("SRC"));
    assert_eq!(leg.dst_airport(), Some("DST"));
    assert_eq!(leg.operating_carrier(), Some("AFL"));
    assert_eq!(leg.flight_number(), Some("1234A"));
    assert_eq!(leg.flight_day(), None);
    assert_eq!(leg.compartment(), None);
    assert_eq!(leg.seat(), Some("999A"));
    assert_eq!(leg.checkin_sequence(), Some(9876));
    assert_eq!(leg.pax_status(), PaxStatus::CheckedIn);

    assert_eq!(bcbp.encode_bcbp().unwrap(), BCBP_MINIMAL);
}

#[test]
fn decode_bcbp_2leg() {
    let bcbp = Bcbp::decode_bcbp(BCBP_M2).unwrap();

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

    assert_eq!(bcbp.encode_bcbp().unwrap(), BCBP_M2);
}
