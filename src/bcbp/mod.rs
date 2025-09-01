use std::path::Iter;
use std::str;

use time::Date;

mod encode;

mod error;
pub mod format;
mod leg;
mod parser;
mod pax_kind;
mod pax_status;
mod security_data;
pub mod utils;
mod version;
pub mod view;

use crate::macros::*;

pub use self::error::{BcbpResult, Error, FixError};
pub use self::leg::Leg;
pub use self::pax_kind::PaxKind;
pub use self::pax_status::PaxStatus;
pub use self::security_data::SecurityData;
pub use self::version::Version;

use crate::bcbp::encode::encode_bcbp;

#[derive(Debug, PartialEq)]
pub enum Mode {
    Tolerant,
    Strict,
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize))]
pub struct Bcbp {
    // Header
    passenger_name: String,
    eticket_indicator: Option<char>,
    legs: Vec<Leg>,
    conditional_data: Option<ConditionalData>,
    security_data: Option<SecurityData>,
}

impl Bcbp {
    pub fn has_conditional_data(&self) -> bool {
        self.conditional_data.is_some()
    }

    #[inline]
    fn conditional_mut(&mut self) -> &mut ConditionalData {
        self.conditional_data
            .get_or_insert_with(ConditionalData::default)
    }

    fn verify_bagtag(s: &str) -> BcbpResult<()> {
        if s.len() > 13 {
            Err(Error::MandatoryDataSize)
        } else if !s.as_bytes().iter().all(u8::is_ascii_digit) {
            Err(Error::DigitsExpected)
        } else {
            Ok(())
        }
    }

    pub fn is_extednded(&self) -> bool {
        self.has_conditional_data() || self.legs_count() > 1 || self.legs[0].has_conditional_data()
    }

    // gen_get_set!(get_set set_bagtag1(str::trim) for bagtag1 with Bcbp::verify_bagtag);
    // gen_get_set!(get_set set_nonconsecutive_bagtag1(str::trim) for nonconsecutive_bagtag1 with Bcbp::verify_bagtag);
    // gen_get_set!(get_set set_nonconsecutive_bagtag2(str::trim) for nonconsecutive_bagtag2 with Bcbp::verify_bagtag);
    // gen_get_set!(get_set set_boradingpass_airline for boardingpass_airline with len 20);

    gen_get_set_char!(get_set set_eticket_indicator(char::to_ascii_uppercase) for eticket_indicator);

    pub fn passenger_name(&self) -> &str {
        &self.passenger_name
    }

    pub fn set_passenger_name(&mut self, name: &str) -> BcbpResult<()> {
        let name = name.trim();
        if name.len() > 20 {
            return Err(Error::MandatoryDataSize);
        }
        if !name.is_ascii() {
            return Err(Error::InvalidCharacters);
        }
        self.passenger_name = name.to_owned();

        Ok(())
    }

    pub fn legs_count(&self) -> usize {
        self.legs.len().min(9)
    }

    pub fn leg(&self, index: usize) -> &Leg {
        &self.legs[index]
    }

    /// Returns a readonly iterator over the legs.
    pub fn legs(&self) -> std::slice::Iter<'_, Leg> {
        self.legs.iter()
    }

    /// Returns a mutable iterator over the legs.
    pub fn legs_mut(&mut self) -> std::slice::IterMut<'_, Leg> {
        self.legs.iter_mut()
    }

    pub fn add_leg(&mut self, leg: Leg) -> BcbpResult<()> {
        if self.legs.len() >= 9 {
            return Err(Error::InvalidLegsCount);
        }

        self.legs.push(leg);

        Ok(())
    }

    #[deprecated(note = "Use `Bcbp::decode_bcbp` instead")]
    pub fn from(src: impl AsRef<str>) -> BcbpResult<Bcbp> {
        Self::decode_bcbp(src)
    }

    pub fn decode_bcbp(src: impl AsRef<str>) -> BcbpResult<Bcbp> {
        parser::decode_bcbp(src.as_ref())
    }

    pub fn encode_bcbp(&self) -> BcbpResult<String> {
        encode_bcbp(self)
    }
    pub fn version(&self) -> Option<Version> {
        self.conditional_data.as_ref().and_then(|c| c.version)
    }

    pub fn set_version(&mut self, version: Option<Version>) -> BcbpResult<()> {
        self.conditional_mut().version = version;

        Ok(())
    }

    pub fn pax_kind(&self) -> Option<PaxKind> {
        self.conditional_data.as_ref().and_then(|c| c.pax_kind)
    }

    pub fn set_pax_kind(&mut self, pax_kind: Option<PaxKind>) -> BcbpResult<()> {
        self.conditional_mut().pax_kind = pax_kind;

        Ok(())
    }

    pub fn checkin_src(&self) -> Option<char> {
        self.conditional_data.as_ref().and_then(|c| c.checkin_src)
    }

    pub fn set_checkin_src(&mut self, checkin_src: Option<char>) -> BcbpResult<()> {
        self.conditional_mut().checkin_src = checkin_src;

        Ok(())
    }

    pub fn boardingpass_src(&self) -> Option<char> {
        self.conditional_data
            .as_ref()
            .and_then(|c| c.boardingpass_src)
    }

    pub fn set_boardingpass_src(&mut self, boardingpass_src: Option<char>) -> BcbpResult<()> {
        self.conditional_mut().boardingpass_src = boardingpass_src;

        Ok(())
    }

    pub fn boardingpass_issued(&self) -> Option<u16> {
        self.conditional_data
            .as_ref()
            .and_then(|c| c.boardingpass_issued)
    }

    pub fn set_boardingpass_issued(&mut self, boardingpass_issued: Option<u16>) -> BcbpResult<()> {
        self.conditional_mut().boardingpass_issued = boardingpass_issued;

        Ok(())
    }

    pub fn doc_type(&self) -> Option<char> {
        self.conditional_data.as_ref().and_then(|c| c.doc_type)
    }

    pub fn set_doc_type(&mut self, doc_type: Option<char>) -> BcbpResult<()> {
        self.conditional_mut().doc_type = doc_type;

        Ok(())
    }
    pub fn boardingpass_airline(&self) -> Option<&str> {
        self.conditional_data
            .as_ref()
            .and_then(|c| c.boardingpass_airline.as_deref())
    }

    pub fn set_boardingpass_airline(&mut self, airline: Option<&str>) -> BcbpResult<()> {
        if let Some(airline) = airline {
            let airline = airline.trim();
            if airline.len() > 3 {
                return Err(Error::MandatoryDataSize);
            }
            if !airline.is_ascii() {
                return Err(Error::InvalidCharacters);
            }
            self.conditional_mut().boardingpass_airline = Some(airline.to_owned());
        } else {
            self.conditional_mut().boardingpass_airline = None;
        }

        Ok(())
    }

    pub fn bagtags(&self) -> Option<&str> {
        self.conditional_data
            .as_ref()
            .and_then(|c| c.bagtags.as_deref())
    }

    pub fn set_bagtags(&mut self, value: Option<&str>) -> BcbpResult<()> {
        if let Some(value) = value {
            let value = value.trim();
            Self::verify_bagtag(value)?;
            self.conditional_mut().bagtags = Some(value.to_owned());
        } else {
            self.conditional_mut().bagtags = None;
        }

        Ok(())
    }

    pub fn nonconsecutive_bagtag1(&self) -> Option<&str> {
        self.conditional_data
            .as_ref()
            .and_then(|c| c.nonconsecutive_bagtag1.as_deref())
    }

    pub fn set_nonconsecutive_bagtag1(&mut self, value: Option<&str>) -> BcbpResult<()> {
        if let Some(value) = value {
            let value = value.trim();
            Self::verify_bagtag(value)?;
            self.conditional_mut().nonconsecutive_bagtag1 = Some(value.to_owned());
        } else {
            self.conditional_mut().nonconsecutive_bagtag1 = None;
        }

        Ok(())
    }

    pub fn nonconsecutive_bagtag2(&self) -> Option<&str> {
        self.conditional_data
            .as_ref()
            .and_then(|c| c.nonconsecutive_bagtag2.as_deref())
    }

    pub fn set_nonconsecutive_bagtag2(&mut self, value: Option<&str>) -> BcbpResult<()> {
        if let Some(value) = value {
            let value = value.trim();
            Self::verify_bagtag(value)?;
            self.conditional_mut().nonconsecutive_bagtag2 = Some(value.to_owned());
        } else {
            self.conditional_mut().nonconsecutive_bagtag2 = None;
        }

        Ok(())
    }

    fn security_data(&self) -> Option<&SecurityData> {
        self.security_data.as_ref()
    }
}

#[derive(Debug, Default, Clone)]
struct ConditionalData {
    version: Option<Version>,
    pax_kind: Option<PaxKind>,
    checkin_src: Option<char>,
    boardingpass_src: Option<char>,
    boardingpass_issued: Option<u16>,
    doc_type: Option<char>,

    boardingpass_airline: Option<String>,

    bagtags: Option<String>,
    nonconsecutive_bagtag1: Option<String>,
    nonconsecutive_bagtag2: Option<String>,
}

pub fn fix_length(src: &str) -> std::result::Result<String, FixError> {
    if src.len() < 60 {
        return Err(FixError::InsufficientDataLength);
    }

    let mut tmp = src.to_owned();

    // Minimal
    tmp.truncate(58);
    tmp.push_str("00");

    Ok(tmp)
}
