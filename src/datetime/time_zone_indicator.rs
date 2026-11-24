use super::Error;
use std::str::FromStr;

/// Time zone indicator in an IATA time string.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TimeZoneIndicator {
    /// The time has no zone suffix.
    None,
    /// The time is local (`L`).
    Local,
    /// The time is UTC (`Z`).
    Utc,
}

impl TimeZoneIndicator {
    /// Returns the wire suffix, or [`None`] if the indicator is absent.
    ///
    /// ```
    /// use iata::datetime::TimeZoneIndicator;
    ///
    /// assert_eq!(TimeZoneIndicator::Local.as_str(), Some("L"));
    /// assert_eq!(TimeZoneIndicator::Utc.as_str(), Some("Z"));
    /// assert_eq!(TimeZoneIndicator::None.as_str(), None);
    /// ```
    pub fn as_str(self) -> Option<&'static str> {
        match self {
            TimeZoneIndicator::Local => Some("L"),
            TimeZoneIndicator::Utc => Some("Z"),
            TimeZoneIndicator::None => None,
        }
    }
}

impl FromStr for TimeZoneIndicator {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "l" | "L" => TimeZoneIndicator::Local,
            "z" | "Z" => TimeZoneIndicator::Utc,
            other => return Err(Error::InvalidTimezoneTag(other.into())),
        })
    }
}
