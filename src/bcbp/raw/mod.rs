// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

mod parser;

use crate::bcbp::error::BcbpResult;

use parser::from_str;

// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

#[derive(Clone,Eq,PartialEq,Hash,Debug,Default)]
pub struct Leg<'a> {
    pub(crate) pnr: &'a str,
    pub(crate) src_airport: &'a str,
    pub(crate) dst_airport: &'a str,
    pub(crate) airline: &'a str,
    pub(crate) flight_number: &'a str,
    pub(crate) flight_day: &'a str,
    pub(crate) compartment: char,
    pub(crate) seat: &'a str,
    pub(crate) checkin_sequence: &'a str,
    pub(crate) pax_status: char,
    pub(crate) airline_numeric_code: Option<&'a str>,
    pub(crate) document_form_serial_number: Option<&'a str>,
    pub(crate) selectee_indicator: Option<char>,
    pub(crate) international_document_verification: Option<char>,
    pub(crate) marketing_carrier_designator: Option<&'a str>,
    pub(crate) frequent_flyer_airline: Option<&'a str>,
    pub(crate) frequent_flyer_number: Option<&'a str>,
    pub(crate) id_ad_indicator: Option<char>,
    pub(crate) free_baggage_allowance: Option<&'a str>,
    pub(crate) fast_track: Option<char>,
    pub(crate) airline_individual_use: Option<&'a str>,
}

impl Leg<'_> {

    /// An alphanumeric string of up to 6 characters, left-aligned, space-padded.
    /// This is the Passenger Name Record used to identify the booking
    /// in the reservation system of the operating carrier.
    pub fn pnr(&self) -> &str {
        &self.pnr
    }

    /// Three-letter or four-letter IATA code of the origin airport.
    /// Spaces indicate the field is not set.
    /// Any other values are invalid.
    pub fn src_airport(&self) -> &str {
        &self.src_airport
    }

    /// Three-letter or four-letter IATA code of the destination airport.
    /// Spaces indicate the field is not set.
    /// Any other values are invalid.
    pub fn dst_airport(&self) -> &str {
        &self.dst_airport
    }

    /// Airline code of the marketing carrier, which can be the same as the operating carrier.
    /// Two-character and three-letter IATA carrier designators
    /// are permitted and the string is left-justified and space padded.
    /// Spaces indicate the field is not set.
    /// Any other values are invalid.
    pub fn marketing_carrier_designator(&self) -> Option<&str> {
        self.marketing_carrier_designator.as_deref()
    }

    /// Airline code associated with the frequent flyer number.
    /// Two-character and three-letter IATA carrier designators
    /// are permitted and the string is left-justified and space padded.
    /// Spaces indicate the field is not set.
    /// Any other values are invalid.
    pub fn frequent_flyer_airline(&self) -> Option<&str> {
        self.frequent_flyer_airline.as_deref()
    }

    /// 2 character or 3 letter airline designator followed by up to 13 numerics or
    /// alphanumerics, or 16 numerics if the FFN is 16 digits.
    /// Spaces indicate the field is not set.
    /// Any other values are invalid.
    pub fn frequent_flyer_number(&self) -> Option<&str> {
        self.frequent_flyer_number.as_deref()
    }

    /// Values are defined in Resolution 792.
    /// Spaces indicate the field is not set.
    /// Any other values are invalid.
    pub fn id_ad_indicator(&self) -> Option<char> {
        self.id_ad_indicator
    }

    /// Airline code of the operating carrier, which can be the same as the marketing carrier.
    /// Two-character and three-letter IATA carrier designators
    /// are permitted and the string is left-justified and space padded.
    /// Spaces indicate the field is not set.
    /// Any other values are invalid.
    pub fn airline(&self) -> &str {
        &self.airline
    }

    /// A flight number comprised of four numeric characters followed by an optional
    /// alphabetic suffix. This refers to the operating carrier.
    /// Spaces indicate the field is not set.
    pub fn flight_number(&self) -> &str {
        &self.flight_number
    }

    /// The Julian date code for the flight. The 3-digit number reflects the
    /// day of the year beginning with '0'. The year is to be inferred.
    /// Spaces indicate the field is not set.
    pub fn flight_day(&self) -> &str {
        &self.flight_day
    }

    /// IATA compartment code indiciating the class of service.
    /// Values are defined in Resolution 792.
    /// A space indicates the field is not set.
    /// Any other values are invalid.
    pub fn compartment(&self) -> char {
        self.compartment
    }

    /// Seat number of the passenger.
    /// Usually 3 numerics followed by a single alphabetic.
    /// In the case of infants, can be any 4 ASCII characters, often 'INF '.
    /// Spaces indicate the field is not set.
    pub fn seat(&self) -> &str {
        &self.seat
    }

    /// Check-in sequence number.
    /// Usually 4 numerics followed by an optional alpha or blank, however in the case of
    /// infants, the format is defined by the host system and can be any 5 ASCII characters.
    pub fn checkin_sequence(&self) -> &str {
        &self.checkin_sequence
    }

    /// The status of the passenger.
    /// Field values are defined in Resolution 792.
    /// A space indicates the field is not set.
    pub fn pax_status(&self) -> char {
        self.pax_status
    }

    /// The three-digit airline numeric code.
    /// This is also the first three digits of the eTicket number.
    /// Spaces indicate the field is not set.
    pub fn airline_numeric_code(&self) -> Option<&str> {
        self.airline_numeric_code.as_deref()
    }

    /// The ten-digit DSN.
    /// This is also the last ten digits of the eTicket number.
    /// Spaces indicate the field is not set.
    pub fn document_form_serial_number(&self) -> Option<&str> {
        self.document_form_serial_number.as_deref()
    }

    /// This field is used by certain agencies to demarcate individuals requiring extra screening.
    /// Although a conditional field, it is now required as of Resolotion 792 Version 6 when
    /// travel involves the United States. Values '0', '1', or '3' determine the type
    /// of screening the passenger will receive at US airports.
    /// A space indicates the field is not set.
    pub fn selectee_indicator(&self) -> Option<char> {
        self.selectee_indicator
    }

    /// This field is used by carriers to identify passengers requiring document verification.
    /// Connected to the display of the 'DOCS OK' string on international boarding passes.
    pub fn international_document_verification(&self) -> Option<char> {
        self.international_document_verification
    }

    /// Indicates if the passenger is eligible for fast track.
    /// If 'Y', the passenger is eligible, 'N' if not, ' ' if not set.
    /// Any other values are invalid.
    pub fn fast_track(&self) -> Option<char> {
        self.fast_track
    }

    /// Three characters, unstructured, left-aligned and space padded,
    /// indicating how much baggage passengers are able to take with them free of charge.
    /// Spaces indicate the field is not set.
    pub fn free_baggage_allowance(&self) -> Option<&str> {
        self.free_baggage_allowance.as_deref()
    }

    /// Optional unstructured data for airline individual use.
    /// Content frequently includes frequent flyer tier, passenger preferences, etc.
    pub fn airline_individual_use(&self) -> Option<&str> {
        self.airline_individual_use.as_deref()
    }

}

#[derive(Clone,Eq,PartialEq,Hash,Debug,Default)]
pub struct SecurityData {
    pub(crate) type_of_security_data: Option<char>,
    pub(crate) security_data: Option<String>,
}

impl SecurityData {

    /// Vendor specific flag indicating the type of the security data which follows.
    pub fn type_of_security_data(&self) -> Option<char> {
        self.type_of_security_data
    }

    /// Security data used to verify the boarding pass was not tampered with.
    pub fn security_data(&self) -> Option<&str> {
        self.security_data.as_deref()
    }

}

#[derive(Clone,Eq,PartialEq,Hash,Debug,Default)]
pub struct Bcbp<'a> {
    pub(crate) pax_name: &'a str,
    pub(crate) eticket_flag: char,
    pub(crate) pax_description: Option<char>,
    pub(crate) source_of_check_in: Option<char>,
    pub(crate) source_of_boarding_pass_issuance: Option<char>,
    pub(crate) date_of_issue_of_boarding_pass: Option<&'a str>,
    pub(crate) doc_type: Option<char>,
    pub(crate) airline_designator_of_boarding_pass_issuer: Option<&'a str>,
    pub(crate) baggage_tag_license_plate_numbers: Option<&'a str>,
    pub(crate) first_non_consecutive_baggage_tag_license_plate_numbers: Option<&'a str>,
    pub(crate) second_non_consecutive_baggage_tag_license_plate_numbers: Option<&'a str>,
    pub(crate) legs: Vec<Leg<'a>>,
    pub(crate) security_data: SecurityData,
}

impl<'a> Bcbp<'a> {

    pub fn from(input: &'a str) -> BcbpResult<Bcbp<'a>> {
        from_str(input)
    }

    /// All legs encoded into the boarding pass.
    /// At least one needs to be present to form a valid boarding pass.
    pub fn legs(&self) -> &[Leg] {
        &self.legs
    }

    /// A reference to the optional security data used to verify a boarding pass was not tamptered with.
    pub fn security_data(&self) -> &SecurityData {
        &self.security_data
    }

    /// Used to differentiate between an electronic ticket ('E') and another type of travel document.
    /// Values are defined in Resolution 792.
    /// A space indicates the field is not set.
    pub fn eticket_flag(&self) -> char {
        self.eticket_flag
    }

    /// This describes the passenger.
    /// Values are defined in Resolution 792.
    /// Spaces indicate the field is not set.
    pub fn pax_description(&self) -> Option<char> {
        self.pax_description
    }

    /// The name of the passenger. Up to 20 characters, left-aligned, space padded.
    /// The format is `LAST_NAME/FIRST_NAME[TITLE]`. There is no separator between
    /// the first name and the title, and no indication a title is present.
    /// Certain names have characters which cannot be translated and
    /// special handling may be required.
    /// Spaces indicate the field is not set.
    pub fn pax_name(&self) -> &str {
        &self.pax_name
    }

    /// This field reflects channel in which the customer initiated check-in.
    /// Values are defined in Resolution 792 Attachment C.
    /// Spaces indicate the field is not set.
    pub fn source_of_check_in(&self) -> Option<char> {
        self.source_of_check_in
    }

    /// This field reflects channel which issued the boarding pass.
    /// Values are defined in Resolution 792.
    /// Spaces indicate the field is not set.
    pub fn source_of_boarding_pass_issuance(&self) -> Option<char> {
        self.source_of_boarding_pass_issuance
    }

    /// Optionally the 4-digit Julian date representing when the boarding pass
    /// was issued. The first digit is the last digit of the year and the next three
    /// represent the number of days elapsed.
    /// For example:
    ///   "6001" represnts January 1, 2016.
    ///   "6366" represaents December 31, 2016 (a leap year).
    /// Spaces indicate the field is not set.
    pub fn date_of_issue_of_boarding_pass(&self) -> Option<&str> {
        self.date_of_issue_of_boarding_pass.as_deref()
    }

    /// The type of the document, 'B' indicating a boarding pass.
    /// Spaces indicate the field is not set.
    pub fn doc_type(&self) -> Option<char> {
        self.doc_type
    }

    /// Airline code of the boarding pass issuer.
    /// Two-character and three-letter IATA carrier designators
    /// are permitted and the string is left-justified and space padded.
    /// Spaces indicate the field is not set.
    pub fn airline_designator_of_boarding_pass_issuer(&self) -> Option<&str> {
        self.airline_designator_of_boarding_pass_issuer.as_deref()
    }

    /// This field allows carriers to populate baggage tag numbers and the number
    /// of consecutive bags. This 13-character fiels is divided into:
    ///         0: '0' for interline tag, '1' for fall-back tag, '2' for interline rush tag.
    ///    2... 4: carrier numeric code.
    ///    5...10: carrier initial tag number with leading zeroes.
    ///   11...13: number of consecutive bags (up to 999).
    /// Spaces indicate the field is not set.
    pub fn baggage_tag_license_plate_numbers(&self) -> Option<&str> {
        self.baggage_tag_license_plate_numbers.as_deref()
    }

    /// This field allows carriers who handle non-sequential bags to include a second set of them
    /// in the boarding pass data in in the same format as `baggage_tag_license_plate_numbers`.
    /// Spaces indicate the field is not set.
    pub fn first_non_consecutive_baggage_tag_license_plate_numbers(&self) -> Option<&str> {
        self.first_non_consecutive_baggage_tag_license_plate_numbers.as_deref()
    }

    /// This field allows carriers who handle non-sequential bags to include a third set of them
    /// in the boarding pass data in in the same format as `baggage_tag_license_plate_numbers`.
    /// Spaces indicate the field is not set.
    pub fn second_non_consecutive_baggage_tag_license_plate_numbers(&self) -> Option<&str> {
        self.second_non_consecutive_baggage_tag_license_plate_numbers.as_deref()
    }
}
