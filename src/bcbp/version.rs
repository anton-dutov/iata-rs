use super::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    Value(u8),
    Other(u8),
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

impl From<Version> for char {
    fn from(value: Version) -> Self {
        match value {
            Version::Value(v) => (b'0' + v) as char,
            Version::Other(v) => v as char,
        }
    }
}
