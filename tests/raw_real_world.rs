// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

//! Test cases derived from real-world boarding pass data.

use iata::bcbp::raw::*;


#[test]
fn alaska_boardingpass() {
    const PASS_STR: &str = "M1MROZ/MARTIN         EXXXXXX SJCLAXAS 3317 207U001A0006 34D>218 VV8207BAS              2502771980993865 AS AS XXXXX55200000000Z29  00010";
    let pass_data = Bcbp::from(PASS_STR).unwrap();
    assert_eq!(pass_data.pax_name(), "MROZ/MARTIN         ");
    assert_eq!(pass_data.eticket_flag(), 'E');
    assert_eq!(pass_data.legs().len(), 1);

    assert_eq!(pass_data.pax_description(), Some(' '));
    assert_eq!(pass_data.checkin_src(), Some('V'));
    assert_eq!(pass_data.boardingpass_issue_src(), Some('V'));
    assert_eq!(pass_data.boardingpass_issue_date(), Some("8207"));
    assert_eq!(pass_data.doc_type(), Some('B'));
    assert_eq!(pass_data.boardingpass_issue_airline(), Some("AS "));
    assert_eq!(pass_data.bagtags(), Some("             "));
    assert_eq!(pass_data.bagtags_nc1(), None);
    assert_eq!(pass_data.bagtags_nc2(), None);

    { // Fields in leg 1 of 1.
        let first_leg = &pass_data.legs()[0];
        assert_eq!(first_leg.pnr(), "XXXXXX ");
        assert_eq!(first_leg.src_airport(), "SJC");
        assert_eq!(first_leg.dst_airport(), "LAX");
        assert_eq!(first_leg.operating_airline(), "AS ");
        assert_eq!(first_leg.flight_number(), "3317 ");
        assert_eq!(first_leg.flight_day(), "207");
        assert_eq!(first_leg.compartment(), 'U');
        assert_eq!(first_leg.seat(), "001A");
        assert_eq!(first_leg.checkin_sequence(), "0006 ");
        assert_eq!(first_leg.pax_status(), '3');

        assert_eq!(first_leg.airline_numeric_code(), Some("027"));
        assert_eq!(first_leg.doc_number(), Some("7198099386"));
        assert_eq!(first_leg.selectee_indicator(), Some('5'));
        assert_eq!(first_leg.international_document_verification(), Some(' '));
        assert_eq!(first_leg.marketing_airline(), Some("AS "));
        assert_eq!(first_leg.frequent_flyer_airline(), Some("AS "));
        assert_eq!(first_leg.frequent_flyer_number(), Some("XXXXX55200000000"));
        assert_eq!(first_leg.id_ad_indicator(), None);
        assert_eq!(first_leg.free_baggage_allowance(), None);
        assert_eq!(first_leg.fast_track(), None);
        assert_eq!(first_leg.airline_individual_use(), Some("Z29  00010"));
    }
}

#[test]
fn air_canada_boardingpass() {
    const PASS_STR: &str = "M1Mroz/Martin         EXXXXXX YVRYOWAC 0344 211          072>20B0  8203IAC 250140000000000 0AC AC AC000000000     *20000AC 223                14080003068        0B          N";
    let pass_data = Bcbp::from(PASS_STR).unwrap();
    assert_eq!(pass_data.pax_name(), "Mroz/Martin         ");
    assert_eq!(pass_data.eticket_flag(), 'E');
    assert_eq!(pass_data.legs().len(), 1);

    assert_eq!(pass_data.pax_description(), Some('0'));
    assert_eq!(pass_data.checkin_src(), Some(' '));
    assert_eq!(pass_data.boardingpass_issue_src(), Some(' '));
    assert_eq!(pass_data.boardingpass_issue_date(), Some("8203"));
    assert_eq!(pass_data.doc_type(), Some('I'));
    assert_eq!(pass_data.boardingpass_issue_airline(), Some("AC "));
    assert_eq!(pass_data.bagtags(), None);
    assert_eq!(pass_data.bagtags_nc1(), None);
    assert_eq!(pass_data.bagtags_nc2(), None);

    { // Fields in leg 1 of 1.
        let first_leg = &pass_data.legs()[0];
        assert_eq!(first_leg.pnr(), "XXXXXX ");
        assert_eq!(first_leg.src_airport(), "YVR");
        assert_eq!(first_leg.dst_airport(), "YOW");
        assert_eq!(first_leg.operating_airline(), "AC ");
        assert_eq!(first_leg.flight_number(), "0344 ");
        assert_eq!(first_leg.flight_day(), "211");
        assert_eq!(first_leg.compartment(), ' ');
        assert_eq!(first_leg.seat(), "    ");
        assert_eq!(first_leg.checkin_sequence(), "     ");
        assert_eq!(first_leg.pax_status(), '0');

        assert_eq!(first_leg.airline_numeric_code(), Some("014"));
        assert_eq!(first_leg.doc_number(), Some("0000000000"));
        assert_eq!(first_leg.selectee_indicator(), Some(' '));
        assert_eq!(first_leg.international_document_verification(), Some('0'));
        assert_eq!(first_leg.marketing_airline(), Some("AC "));
        assert_eq!(first_leg.frequent_flyer_airline(), Some("AC "));
        assert_eq!(first_leg.frequent_flyer_number(), Some("AC000000000     "));
        assert_eq!(first_leg.id_ad_indicator(), None);
        assert_eq!(first_leg.free_baggage_allowance(), None);
        assert_eq!(first_leg.fast_track(), None);
        assert_eq!(first_leg.airline_individual_use(), Some("*20000AC 223                14080003068        0B          N"));
    }
}
