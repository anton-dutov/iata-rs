use std::str::FromStr;

use super::{format::Field, view, Bcbp, BcbpResult, Error, Leg, SecurityData, Version};
use crate::datetime::DayOfYear;

impl FromStr for Bcbp {
    type Err = Error;

    fn from_str(input: &str) -> BcbpResult<Self> {
        // The owned decoder reports a short ASCII message before inspecting its fields.
        if input.len() < 60 && input.is_ascii() {
            return Err(Error::BcbpTooShort);
        }
        Self::try_from(view::parser::parse(input)?)
    }
}

impl TryFrom<view::BcbpView<'_>> for Bcbp {
    type Error = Error;

    /// Normalizes the wire fields into an editable pass and validates typed values.
    fn try_from(raw: view::BcbpView<'_>) -> BcbpResult<Self> {
        if raw.legs.is_empty() {
            return Err(Error::InvalidNumberOfLegs);
        }
        let mut pass = Self::default();
        pass.set_passenger_name(raw.pax_name)?;
        pass.set_eticket_indicator(nonblank(raw.eticket_flag))?;

        if let Some(version) = raw.version {
            pass.set_version(nonblank(version).map(Version::try_from).transpose()?)?;
            pass.set_passenger_description(
                raw.passenger_description.and_then(nonblank).map(Into::into),
            )?;
            pass.set_checkin_src(raw.source_of_check_in)?;
            pass.set_boardingpass_src(raw.source_of_boarding_pass_issuance)?;
            pass.set_boardingpass_issued(
                raw.date_of_issue_of_boarding_pass
                    .and_then(optional_decimal),
            )?;
            pass.set_doc_type(raw.doc_type)?;
            pass.set_boardingpass_airline(raw.airline_designator_of_boarding_pass_issuer)?;
            pass.set_bagtags(raw.baggage_tags)?;
            pass.set_nonconsecutive_bagtag1(raw.nonconsecutive_baggage_tags1)?;
            pass.set_nonconsecutive_bagtag2(raw.nonconsecutive_baggage_tags2)?;
        }
        for leg in raw.legs {
            pass.add_leg(owned_leg(leg)?)?;
        }
        if let Some(kind) = raw.security_data.kind {
            let data = raw
                .security_data
                .data
                .ok_or(Error::UnexpectedEndOfInput(Field::LengthOfSecurityData))?;
            pass.security_data = Some(SecurityData { kind, data });
        }
        Ok(pass)
    }
}

fn nonblank(value: char) -> Option<char> {
    (value != ' ').then_some(value)
}

fn optional_decimal(value: &str) -> Option<u16> {
    let value = value.trim();
    // Keep the legacy zero fallback for nonempty, malformed numeric fields.
    (!value.is_empty()).then(|| value.trim_start_matches('0').parse().unwrap_or_default())
}

fn owned_leg(raw: view::Leg<'_>) -> BcbpResult<Leg> {
    let mut leg = Leg::default();
    leg.set_pnr(Some(raw.pnr))?;
    leg.set_src_airport(Some(raw.src_airport))?;
    leg.set_dst_airport(Some(raw.dst_airport))?;
    leg.set_operating_carrier(Some(raw.operating_carrier))?;
    leg.set_flight_number(Some(raw.flight_number))?;
    if let Some(day) = optional_decimal(raw.flight_day) {
        leg.set_flight_day(Some(DayOfYear::new(day)?))?;
    }
    leg.set_compartment(nonblank(raw.compartment))?;
    leg.set_seat(Some(raw.seat))?;
    leg.set_checkin_sequence(raw.checkin_sequence.trim().parse().ok())?;
    leg.set_passenger_status(raw.passenger_status.into())?;

    // The numeric code is the first field in every nonempty repeated section.
    if let Some(code) = raw.airline_numeric_code {
        leg.set_airline_num(optional_decimal(code))?;
        leg.set_doc_number(raw.document_form_serial_number)?;
        leg.set_selectee_indicator(raw.selectee_indicator.and_then(nonblank))?;
        leg.set_doc_intl_verification(raw.international_document_verification)?;
        leg.set_marketing_carrier(raw.marketing_carrier)?;
        leg.set_freq_flyer_airline(raw.frequent_flyer_airline)?;
        leg.set_freq_flyer_number(raw.frequent_flyer_number)?;
        leg.set_id_ad_indicator(raw.id_ad_indicator)?;
        leg.set_baggage_allowance(raw.free_baggage_allowance)?;
        leg.set_fast_track(raw.fast_track.and_then(nonblank))?;
    }
    if raw.airline_individual_use.is_some() {
        leg.set_variable_data(raw.airline_individual_use)?;
    }
    Ok(leg)
}
