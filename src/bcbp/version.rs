use super::{format::Field, Error};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize))]
pub enum Version {
    Value(u8),
    Other(u8),
}

const LATEST_ENCODING_NUMBER: u8 = 6;

impl Version {
    /// Latest Resolution 792 version supported for encoding.
    pub const LATEST: Self = Self::Value(LATEST_ENCODING_NUMBER);

    pub(super) fn encoding_number(self) -> Result<u8, Error> {
        let number = match self {
            Self::Value(number) => number,
            Self::Other(byte) if byte.is_ascii_digit() => byte - b'0',
            Self::Other(byte) => return Err(Error::InvalidVersion(char::from(byte))),
        };
        // The versioned structured sections used by this encoder begin with v2.
        if !(2..=LATEST_ENCODING_NUMBER).contains(&number) {
            return Err(Error::UnsupportedEncodingVersion(number));
        }
        Ok(number)
    }
}

impl TryFrom<char> for Version {
    type Error = Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if !value.is_ascii_digit() {
            return Err(Error::InvalidVersion(value));
        }

        let v = value as u8;
        match v {
            b'0'..=b'9' => Ok(Version::Value(v - b'0')),
            _ => Ok(Version::Other(v)),
        }
    }
}

impl TryFrom<Version> for char {
    type Error = Error;

    /// Converts a numeric version to one decimal digit or preserves an `Other` byte.
    fn try_from(value: Version) -> Result<Self, Self::Error> {
        match value {
            Version::Value(digit) => {
                char::from_digit(u32::from(digit), 10).ok_or(Error::InvalidFieldFormat {
                    field: Field::VersionNumber,
                })
            }
            Version::Other(byte) => Ok(char::from(byte)),
        }
    }
}
