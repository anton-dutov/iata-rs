use core::num;

use super::*;
use crate::{
    bcbp::format::Field,
    datetime::{DayOfYear, Error as DateError},
};

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
struct LegConditionalData {
    airline_num: Option<u16>,
    doc_number: Option<String>,
    selectee_indicator: Option<char>,
    doc_int_verification: Option<char>,
    marketing_carrier: Option<String>,
    freq_flyer_airline: Option<String>,
    freq_flyer_number: Option<String>,
    id_ad_indicator: Option<char>,
    bag_allowance: Option<String>,
    fast_track: Option<char>,
    variable_data: Option<String>,
}

impl Leg {
    pub fn has_conditional_data(&self) -> bool {
        self.conditional_data.is_some()
    }

    #[inline]
    fn conditional_mut(&mut self) -> &mut LegConditionalData {
        self.conditional_data
            .get_or_insert_with(LegConditionalData::default)
    }

    gen_get_set_char!(get_set set_compartment(char::to_ascii_uppercase) for compartment);

    gen_get_set!(get_set set_src_airport for src_airport with len 3);
    gen_get_set!(get_set set_dst_airport for dst_airport with len 3);
    gen_get_set!(get_set set_operating_carrier for operating_carrier with len 3);
    gen_get_set!(get_set set_flight_number for flight_number with len 5);
    gen_get_set!(get_set set_seat(seat_preprocess) for seat with len 4);
    gen_get_set!(get_set set_pnr for pnr with len 7);

    pub fn flight_day(&self) -> Option<DayOfYear> {
        self.flight_day
    }

    pub fn set_flight_day(&mut self, day: Option<DayOfYear>) {
        self.flight_day = day;
    }

    pub fn set_flight_date(&mut self, date: Date) -> std::result::Result<(), DateError> {
        self.set_flight_day(Some(DayOfYear::new(date.ordinal())?));

        Ok(())
    }

    // gen_get_set_char!(get_set set_selectee_indicator(char::to_ascii_uppercase) for selectee_indicator);
    // gen_get_set_char!(get_set set_doc_int_verification(char::to_ascii_uppercase) for doc_int_verification);
    // gen_get_set_char!(get_set set_fast_track(char::to_ascii_uppercase) for fast_track);
    // gen_get_set!(get_set set_doc_number for doc_number with len 10);
    // gen_get_set!(get_set set_marketing_carrier for marketing_carrier with len 3);
    // gen_get_set!(get_set set_freq_flyer_airline for freq_flyer_airline with len 3);
    // gen_get_set!(get_set set_freq_flyer_numbder for freq_flyer_number with len 16);
    // gen_get_set!(get_set set_bag_allowance for bag_allowance with len 3);

    pub fn pax_status(&self) -> PaxStatus {
        self.pax_status
    }

    pub fn set_pax_status(&mut self, pax_status: PaxStatus) {
        self.pax_status = pax_status;
    }

    pub fn checkin_sequence(&self) -> Option<u16> {
        self.checkin_sequence
    }

    pub fn set_checkin_sequence(&mut self, seq: Option<u16>) -> std::result::Result<(), Error> {
        if let Some(s) = seq {
            if s > 9999 {
                return Err(Error::InvalidCheckInSequence);
            }
        }

        self.checkin_sequence = seq;

        Ok(())
    }

    pub fn airline_num(&self) -> Option<u16> {
        self.conditional_data.as_ref().and_then(|c| c.airline_num)
    }

    pub fn set_airline_num(&mut self, num: Option<u16>) -> std::result::Result<(), Error> {
        if let Some(n) = num {
            if n > 999 {
                return Err(Error::InvalidAirlineNum);
            }
        }

        self.conditional_mut().airline_num = num;

        Ok(())
    }

    pub fn doc_number(&self) -> Option<&str> {
        self.conditional_data
            .as_ref()
            .and_then(|c| c.doc_number.as_deref())
    }

    pub fn set_doc_number(&mut self, value: Option<&str>) -> std::result::Result<(), Error> {
        let value = value.map(str::trim).unwrap_or_default();

        let max_len = Field::DocumentFormSerialNumber.len();

        if value.len() > max_len {
            return Err(Error::FieldSizeExceeded2(
                Field::DocumentFormSerialNumber,
                max_len,
            ));
        }

        self.conditional_mut().doc_number = if value.is_empty() {
            None
        } else {
            Some(value.to_string())
        };

        Ok(())
    }

    pub fn selectee_indicator(&self) -> Option<char> {
        self.conditional_data
            .as_ref()
            .and_then(|c| c.selectee_indicator)
    }

    pub fn set_selectee_indicator(
        &mut self,
        value: Option<char>,
    ) -> std::result::Result<(), Error> {
        if let Some(c) = value {
            // if !c.is_ascii_alphanumeric() {
            // return Err(Error::AlphaExpected);
            // }
        }

        self.conditional_mut().selectee_indicator = value;

        Ok(())
    }

    pub fn doc_int_verification(&self) -> Option<char> {
        self.conditional_data
            .as_ref()
            .and_then(|c| c.doc_int_verification)
    }

    pub fn set_doc_int_verification(
        &mut self,
        value: Option<char>,
    ) -> std::result::Result<(), Error> {
        if let Some(c) = value {
            // if !c.is_ascii_alphabetic() {
            // return Err(Error::AlphaExpected);
            // }
        }

        self.conditional_mut().doc_int_verification = value;

        Ok(())
    }

    pub fn marketing_carrier(&self) -> Option<&str> {
        self.conditional_data
            .as_ref()
            .and_then(|c| c.marketing_carrier.as_deref())
    }

    pub fn set_marketing_carrier(&mut self, value: Option<&str>) -> std::result::Result<(), Error> {
        let value = value.map(str::trim).unwrap_or_default();

        let max_len = Field::MarketingCarrier.len();

        if value.len() > max_len {
            return Err(Error::FieldSizeExceeded2(Field::MarketingCarrier, max_len));
        }

        self.conditional_mut().marketing_carrier = if value.is_empty() {
            None
        } else {
            Some(value.to_string())
        };

        Ok(())
    }

    pub fn freq_flyer_airline(&self) -> Option<&str> {
        self.conditional_data
            .as_ref()
            .and_then(|c| c.freq_flyer_airline.as_deref())
    }

    pub fn set_freq_flyer_airline(
        &mut self,
        value: Option<&str>,
    ) -> std::result::Result<(), Error> {
        let value = value.map(str::trim).unwrap_or_default();

        let max_len = Field::FrequentFlyerAirline.len();

        if value.len() > max_len {
            return Err(Error::FieldSizeExceeded2(
                Field::FrequentFlyerAirline,
                max_len,
            ));
        }

        self.conditional_mut().freq_flyer_airline = if value.is_empty() {
            None
        } else {
            Some(value.to_string())
        };

        Ok(())
    }

    pub fn freq_flyer_number(&self) -> Option<&str> {
        self.conditional_data
            .as_ref()
            .and_then(|c| c.freq_flyer_number.as_deref())
    }

    pub fn set_freq_flyer_number(&mut self, value: Option<&str>) -> std::result::Result<(), Error> {
        let value = value.map(str::trim).unwrap_or_default();

        let max_len = Field::FrequentFlyerNumber.len();

        if value.len() > max_len {
            return Err(Error::FieldSizeExceeded2(
                Field::FrequentFlyerNumber,
                max_len,
            ));
        }

        self.conditional_mut().freq_flyer_number = if value.is_empty() {
            None
        } else {
            Some(value.to_string())
        };

        Ok(())
    }

    pub fn id_ad_indicator(&self) -> Option<char> {
        self.conditional_data
            .as_ref()
            .and_then(|c| c.id_ad_indicator)
    }

    pub fn set_id_ad_indicator(&mut self, value: Option<char>) -> std::result::Result<(), Error> {
        if let Some(c) = value {
            // if !c.is_ascii_alphabetic() {
            // return Err(Error::AlphaExpected);
            // }
        }

        self.conditional_mut().id_ad_indicator = value;

        Ok(())
    }

    pub fn bag_allowance(&self) -> Option<&str> {
        self.conditional_data
            .as_ref()
            .and_then(|c| c.bag_allowance.as_deref())
    }

    pub fn set_bag_allowance(&mut self, value: Option<&str>) -> std::result::Result<(), Error> {
        let value = value.map(str::trim).unwrap_or_default();

        let max_len = Field::FreeBaggageAllowance.len();

        if value.len() > max_len {
            return Err(Error::FieldSizeExceeded2(
                Field::FreeBaggageAllowance,
                max_len,
            ));
        }

        self.conditional_mut().bag_allowance = if value.is_empty() {
            None
        } else {
            Some(value.to_string())
        };

        Ok(())
    }

    pub fn fast_track(&self) -> Option<char> {
        self.conditional_data.as_ref().and_then(|c| c.fast_track)
    }

    pub fn set_fast_track(&mut self, value: Option<char>) -> std::result::Result<(), Error> {
        if let Some(c) = value {
            if !c.is_ascii_alphabetic() {
                return Err(Error::AlphaExpected);
            }
        }

        self.conditional_mut().fast_track = value;

        Ok(())
    }

    pub fn variable_data(&self) -> Option<&str> {
        self.conditional_data
            .as_ref()
            .and_then(|c| c.variable_data.as_deref())
    }

    pub fn set_variable_data(&mut self, data: Option<&str>) -> std::result::Result<(), Error> {
        if let Some(s) = data {
            if s.len() > 30 {
                return Err(Error::InsufficientDataLength);
            }
        }

        self.conditional_mut().variable_data = data.map(str::to_string);

        Ok(())
    }
}

fn seat_preprocess(s: &str) -> &str {
    s.trim().trim_start_matches('0')
}
