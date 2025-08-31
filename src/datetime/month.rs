use std::str::FromStr;

use super::Error;

/// A basic enum that contains all 12 possible months.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    /// Converts a month into a borrowed string. The returned string
    /// is a 3 letter abbreviation of the month written in all uppercase
    /// letters.
    ///
    /// ```
    /// use iata::datetime::Month;
    ///
    /// assert_eq!(Month::January.as_str(), "JAN");
    /// ```
    pub fn as_str(&self) -> &'static str {
        use Month::*;

        match self {
            January => "JAN",
            February => "FEB",
            March => "MAR",
            April => "APR",
            May => "MAY",
            June => "JUN",
            July => "JUL",
            August => "AUG",
            September => "SEP",
            October => "OCT",
            November => "NOV",
            December => "DEC",
        }
    }
}

impl From<Month> for time::Month {
    fn from(m: Month) -> Self {
        use Month::*;
        match m {
            January => time::Month::January,
            February => time::Month::February,
            March => time::Month::March,
            April => time::Month::April,
            May => time::Month::May,
            June => time::Month::June,
            July => time::Month::July,
            August => time::Month::August,
            September => time::Month::September,
            October => time::Month::October,
            November => time::Month::November,
            December => time::Month::December,
        }
    }
}

impl FromStr for Month {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Month::*;

        Ok(match s {
            "JAN" => January,
            "FEB" => February,
            "MAR" => March,
            "APR" => April,
            "MAY" => May,
            "JUN" => June,
            "JUL" => July,
            "AUG" => August,
            "SEP" => September,
            "OCT" => October,
            "NOV" => November,
            "DEC" => December,
            other => return Err(Error::InvalidMonth(other.into())),
        })
    }
}
