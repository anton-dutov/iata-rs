use super::*;
use crate::{bcbp::format::Field, datetime::DayOfYear};

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize))]
pub struct Leg {
    pnr: Option<String>,
    src_airport: Option<String>,
    dst_airport: Option<String>,
    operating_carrier: Option<String>,
    flight_number: Option<String>,
    flight_day: Option<DayOfYear>,
    compartment: Option<char>,
    seat: Option<String>,
    checkin_sequence: Option<u16>,
    pax_status: PaxStatus,
    conditional_data: Option<LegConditionalData>,
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize))]
struct LegConditionalData {
    airline_num: Option<u16>,
    doc_number: Option<String>,
    selectee_indicator: Option<char>,
    doc_intl_verification: Option<char>,
    marketing_carrier: Option<String>,
    freq_flyer_airline: Option<String>,
    freq_flyer_number: Option<String>,
    id_ad_indicator: Option<char>,
    baggage_allowance: Option<String>,
    fast_track: Option<char>,
    variable_data: Option<String>,
}

impl Leg {
    gen_get!(pnr as_deref Option<&str>);
    gen_set!(pnr(str::trim) as Option<&str> using Field::OperatingCarrierPnrCode);

    gen_get!(src_airport as_deref Option<&str>);
    gen_set!(src_airport(str::trim) as Option<&str> using Field::FromCityAirportCode);

    gen_get!(dst_airport as_deref Option<&str>);
    gen_set!(dst_airport(str::trim) as Option<&str> using Field::ToCityAirportCode);

    gen_get!(operating_carrier as_deref Option<&str>);
    gen_set!(operating_carrier(str::trim) as Option<&str> using Field::OperatingCarrier);

    gen_get!(flight_number as_deref Option<&str>);
    gen_set!(flight_number(str::trim) as Option<&str> using Field::FlightNumber);

    gen_get!(compartment as Option<char>);
    gen_set!(compartment(|v| v.to_ascii_uppercase()) as Option<char>);

    gen_get!(flight_day as Option<DayOfYear>);
    gen_set!(flight_day as Option<DayOfYear>);

    gen_get!(seat as_deref Option<&str>);
    gen_set!(seat(seat_preprocess) as Option<&str> using Field::SeatNumber);

    gen_get!(checkin_sequence as Option<u16>);
    // if let Some(s) = seq {
    //     if s > 9999 {
    //         return Err(Error::InvalidCheckInSequence);
    //     }
    // }

    gen_set!(checkin_sequence as Option<u16>);

    gen_get!(pax_status as PaxStatus);
    gen_set!(pax_status as PaxStatus);

    gen_get!(cond airline_num as Option<u16>);

    gen_get!(cond fast_track as Option<char>);
    gen_set!(cond fast_track as Option<char>);

    gen_get!(cond selectee_indicator as Option<char>);
    gen_set!(cond selectee_indicator as Option<char>);

    gen_get!(cond id_ad_indicator as Option<char>);
    gen_set!(cond id_ad_indicator as Option<char>);

    gen_get!(cond doc_intl_verification as Option<char>);
    gen_set!(cond doc_intl_verification as Option<char>);

    gen_get!(cond doc_number as_deref Option<&str>);
    gen_set!(cond doc_number(str::trim) as Option<&str> using Field::DocumentFormSerialNumber);

    gen_get!(cond baggage_allowance as_deref Option<&str>);
    gen_set!(cond baggage_allowance(str::trim) as Option<&str> using Field::FreeBaggageAllowance);

    gen_get!(cond marketing_carrier as_deref Option<&str>);
    gen_set!(cond marketing_carrier(str::trim) as Option<&str> using Field::MarketingCarrier);

    gen_get!(cond freq_flyer_airline as_deref Option<&str>);
    gen_set!(cond freq_flyer_airline(str::trim) as Option<&str> using Field::FrequentFlyerAirline);

    gen_get!(cond freq_flyer_number as_deref Option<&str>);
    gen_set!(cond freq_flyer_number(str::trim) as Option<&str> using Field::FrequentFlyerNumber);

    gen_get!(cond variable_data as_deref Option<&str>);
    gen_set!(cond variable_data as Option<&str> using Field::AirlineIndividualUse);

    pub fn has_conditional_data(&self) -> bool {
        self.conditional_data.is_some()
    }

    #[inline]
    fn conditional_mut(&mut self) -> &mut LegConditionalData {
        self.conditional_data
            .get_or_insert_with(LegConditionalData::default)
    }

    pub fn set_flight_date(&mut self, date: Date) -> BcbpResult<()> {
        self.set_flight_day(Some(DayOfYear::new(date.ordinal())?))?;

        Ok(())
    }

    pub fn set_airline_num(&mut self, num: Option<u16>) -> BcbpResult<()> {
        if let Some(n) = num {
            if n > 999 {
                return Err(Error::InvalidFieldFormat {
                    field: Field::AirlineNumericCode,
                });
            }
        }

        self.conditional_mut().airline_num = num;

        Ok(())
    }
}

fn seat_preprocess(s: &str) -> &str {
    s.trim().trim_start_matches('0')
}
