use std::{fmt, str::FromStr};

use jiff::{
    civil::Date,
    tz::{Offset, TimeZone},
    Timestamp,
};

use super::{adapted_year, days_in_year, Error, Month};

/// A calendar month and day without a year, written as `DDMMM`.
#[derive(Clone, Debug, PartialEq)]
pub struct MonthDay {
    month: Month,
    day: u8,
}

impl MonthDay {
    /// Constructs a month and day, allowing February 29.
    /// [`Self::to_date`] checks it against the supplied year.
    ///
    /// # Errors
    /// All attempts to give the function an illegal day of month will result in
    /// [`Error::InvalidDayForMonth`].
    ///
    /// # Examples
    ///
    /// ```
    /// use iata::datetime::{MonthDay, Month};
    ///
    /// // These are okay
    /// assert!(MonthDay::new(Month::January, 31).is_ok());
    /// assert!(MonthDay::new(Month::February, 28).is_ok());
    /// assert!(MonthDay::new(Month::February, 29).is_ok());
    ///
    /// // These are not okay
    /// assert!(MonthDay::new(Month::February, 30).is_err());
    /// assert!(MonthDay::new(Month::January, 0).is_err());
    /// ```
    pub fn new(month: Month, day: u8) -> Result<Self, Error> {
        use Month::*;

        let max = match month {
            January => 31,
            February => 29,
            March => 31,
            April => 30,
            May => 31,
            June => 30,
            July => 31,
            August => 31,
            September => 30,
            October => 31,
            November => 30,
            December => 31,
        };

        if day == 0 || day > max {
            return Err(Error::InvalidDayForMonth(month, day));
        }

        Ok(Self { month, day })
    }

    /// Returns the day component of the date.
    pub fn day(&self) -> u8 {
        self.day
    }

    /// Returns the month component of the date.
    pub fn month(&self) -> Month {
        self.month
    }

    /// Converts the date into [`jiff::civil::Date`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::OverflowNotLeapYear`] for February 29 in a common year
    /// or for a year outside the range of [`Date`].
    pub fn to_date(&self, year: i16) -> Result<Date, Error> {
        Date::new(year, self.month.into(), self.day as i8).map_err(|_| Error::OverflowNotLeapYear)
    }

    /// Tries to figure out what year the date belongs to. Equivalent to
    /// `self.to_date_adapt(today, days)`.
    ///
    /// For more information about algorithm and some examples, see
    /// [`super::DayOfYear::to_date_adapt`].
    pub fn to_date_adapt_year(&self, offset: Offset, days: u8) -> Result<Date, Error> {
        let now = Timestamp::now().to_zoned_datetime(TimeZone::fixed(offset));

        self.to_date_adapt(now.date(), days)
    }

    /// Tries to figure out what year the date belongs to.
    ///
    /// For more information about algorithm and some examples, see
    /// [`super::DayOfYear::to_date_adapt`].
    pub fn to_date_adapt(&self, for_date: Date, days: u8) -> Result<Date, Error> {
        let year = adapted_year(for_date, days, |year| match self.month {
            Month::January => Some(u16::from(self.day)),
            Month::December => Some(days_in_year(year) - 31 + u16::from(self.day)),
            _ => None,
        })?;
        self.to_date(year)
    }
}

impl fmt::Display for MonthDay {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}{}", self.day, self.month.as_str())
    }
}

impl FromStr for MonthDay {
    type Err = Error;

    /// Returns [`Error::InvalidInput`] if the day and month cannot be split at byte 2.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            return Err(Error::InvalidInput(s.to_owned()));
        }

        let (day, month) = s
            .split_at_checked(2)
            .ok_or_else(|| Error::InvalidInput(s.to_owned()))?;

        Self::new(
            Month::from_str(month)?,
            day.parse()
                .map_err(|_| Error::InvalidInput(day.to_owned()))?,
        )
    }
}
