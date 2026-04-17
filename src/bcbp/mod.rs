use std::str;

use jiff::civil::Date;

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

use crate::bcbp::format::Field;
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
    gen_get!(passenger_name as_ref &str);

    gen_get!(eticket_indicator as Option<char>);
    gen_set!(eticket_indicator as Option<char>);

    gen_get!(security_data as_ref Option<&SecurityData>);

    gen_get!(cond pax_kind as Option<PaxKind>);
    gen_set!(cond pax_kind as Option<PaxKind>);

    gen_get!(cond checkin_src as Option<char>);
    gen_set!(cond checkin_src as Option<char>);

    gen_get!(cond boardingpass_src as Option<char>);
    gen_set!(cond boardingpass_src as Option<char>);

    gen_get!(cond doc_type as Option<char>);
    gen_set!(cond doc_type as Option<char>);

    gen_get!(cond boardingpass_airline as_deref Option<&str>);
    gen_set!(cond boardingpass_airline(str::trim) as Option<&str> using Field::AirlineDesignatorOfBoardingPassIssuer);

    gen_get!(cond bagtags as_deref Option<&str>);
    gen_set!(cond bagtags(str::trim) as Option<&str> using Field::BaggageTagNumbers);

    gen_get!(cond nonconsecutive_bagtag1 as_deref Option<&str>);
    gen_set!(cond nonconsecutive_bagtag1(str::trim) as Option<&str> using Field::FirstNonConsecutiveBaggageTagNumbers);

    gen_get!(cond nonconsecutive_bagtag2 as_deref Option<&str>);
    gen_set!(cond nonconsecutive_bagtag2(str::trim) as Option<&str> using Field::SecondNonConsecutiveBaggageTagNumbers);

    pub fn has_conditional_data(&self) -> bool {
        self.conditional_data.is_some()
    }

    #[inline]
    fn conditional_mut(&mut self) -> &mut ConditionalData {
        self.conditional_data
            .get_or_insert_with(ConditionalData::default)
    }

    pub fn is_extednded(&self) -> bool {
        self.has_conditional_data()
            || self.legs_count() > 1
            || self
                .legs
                .first()
                .is_some_and(Leg::has_conditional_data)
    }

    pub fn set_passenger_name(&mut self, value: &str) -> BcbpResult<()> {
        let value = value.trim();
        let field = Field::PassengerName;
        let max = field.len();

        if value.len() > max {
            return Err(Error::FieldLengthExceeded { field, max });
        }
        if !value.is_ascii() {
            return Err(Error::InvalidCharacters);
        }
        self.passenger_name = value.to_owned();

        Ok(())
    }

    pub fn leg(&self, index: usize) -> &Leg {
        &self.legs[index]
    }

    /// Returns a readonly iterator over the legs.
    pub fn legs(&self) -> std::slice::Iter<'_, Leg> {
        self.legs.iter()
    }

    /// Returns the number of legs, capped at 9.
    pub fn legs_count(&self) -> usize {
        self.legs.len().min(9)
    }

    /// Retains only the legs specified by the predicate.
    pub fn legs_retain(&mut self, f: impl FnMut(&Leg) -> bool) {
        self.legs.retain(f);
    }

    /// Returns a mutable iterator over the legs.
    pub fn legs_mut(&mut self) -> std::slice::IterMut<'_, Leg> {
        self.legs.iter_mut()
    }

    pub fn add_leg(&mut self, leg: Leg) -> BcbpResult<()> {
        if self.legs.len() >= 9 {
            return Err(Error::InvalidNumberOfLegs);
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

    pub fn boardingpass_issued(&self) -> Option<u16> {
        self.conditional_data
            .as_ref()
            .and_then(|c| c.boardingpass_issued)
    }

    pub fn set_boardingpass_issued(&mut self, boardingpass_issued: Option<u16>) -> BcbpResult<()> {
        self.conditional_mut().boardingpass_issued = boardingpass_issued;

        Ok(())
    }
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize))]
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

#[allow(dead_code)]
fn verify_bagtag(value: &str) -> BcbpResult<()> {
    let value = value.trim();
    let field = Field::BaggageTagNumbers;
    let max = field.len();

    if value.len() > max {
        Err(Error::FieldLengthExceeded { field, max })
    } else if !value.as_bytes().iter().all(u8::is_ascii_digit) {
        Err(Error::DigitsExpected)
    } else {
        Ok(())
    }
}
