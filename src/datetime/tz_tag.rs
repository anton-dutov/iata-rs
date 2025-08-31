use super::Error;
use std::str::FromStr;

/// Enum, which determines the timezone.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TzTag {
    /// No timezone is specified.
    None,
    /// The timezone is equivalent to the timezone of
    /// the country the machine is located in.
    Local,
    /// The timezone is UTC+0.
    Utc,
}

impl TzTag {
    /// Converts the timezone tag into a borrowed string. Note that
    /// [`TzTag::None`] is mapped into [`None`].
    ///
    /// ```
    /// use iata::datetime::TzTag;
    ///
    /// assert_eq!(TzTag::Local.as_str(), Some("L"));
    /// assert_eq!(TzTag::Utc.as_str(), Some("Z"));
    /// assert_eq!(TzTag::None.as_str(), None);
    /// ```
    pub fn as_str(self) -> Option<&'static str> {
        match self {
            TzTag::Local => Some("L"),
            TzTag::Utc => Some("Z"),
            TzTag::None => None,
        }
    }
}

impl FromStr for TzTag {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "l" | "L" => TzTag::Local,
            "z" | "Z" => TzTag::Utc,
            other => return Err(Error::InvalidTimezoneTag(other.into())),
        })
    }
}
