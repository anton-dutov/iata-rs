use std::str::FromStr;

mod day_of_year;
mod error;
mod month;
mod tz_tag;

pub use day_of_year::DayOfYear;
pub use error::Error;
pub use month::Month;
pub use tz_tag::TzTag;

use time::{Date, OffsetDateTime, PrimitiveDateTime, UtcOffset};

const MAX_ADAPT_DAYS: u8 = 31;

// pub struct DayOfMonth(u32);

// pub struct DayOfYearCheck(u32, u8);

/// Helper function to check if a given year is actually a leap one.
pub fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

/// A date without a year i.e. a date with a month and a day.
#[derive(Clone, Debug, PartialEq)]
pub struct ShortDate {
    month: Month,
    day: u8,
}

impl ShortDate {
    /// Constructs a short day out of a month and a day. Note, that since
    /// this date isn't aware of the year, it is completely legal to construct
    /// 29th of February.
    ///
    /// # Errors
    /// All attempts to give the function an illegal day of month will result in
    /// [`Error::InvalidDayForMonth`].
    ///
    /// # Examples
    ///
    /// ```
    /// use iata::datetime::{ShortDate, Month};
    ///
    /// // These are okay
    /// assert!(ShortDate::new(Month::January, 31).is_ok());
    /// assert!(ShortDate::new(Month::February, 28).is_ok());
    /// assert!(ShortDate::new(Month::February, 29).is_ok());
    ///
    /// // These are not okay
    /// assert!(ShortDate::new(Month::February, 30).is_err());
    /// assert!(ShortDate::new(Month::January, 0).is_err());
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

    /// Converts the date into [`time::Date`].
    ///
    /// # Errors
    ///
    /// Returns [`Error::OverflowNotLeapYear`] if `year` is not a leap year and `self`
    /// turns out to contain a date that can exist only in a leap year.
    pub fn to_date(&self, year: i32) -> Result<Date, Error> {
        Date::from_calendar_date(year, self.month.into(), self.day)
            // Since all months are correct, the thing could only error if
            // we got 29th of February, when the year is not a leap one.
            .map_err(|_| Error::OverflowNotLeapYear)
    }

    /// Tries to figure out what year the date belongs to. Equivalent to
    /// `self.to_date_adapt(&today, days)`.
    ///
    /// For more information about algorithm and some examples, see
    /// [`DayOfYear::to_date_adapt()`].
    pub fn to_date_adapt_year(&self, offset: UtcOffset, days: u8) -> Result<Date, Error> {
        let now = OffsetDateTime::now_utc().to_offset(offset);

        self.to_date_adapt(now.date(), days)
    }

    /// Tries to figure out what year the date belongs to.
    ///
    /// For more information about algorithm and some examples, see
    /// [`DayOfYear::to_date_adapt()`].
    pub fn to_date_adapt(&self, for_date: Date, days: u8) -> Result<Date, Error> {
        use Month::*;

        if !(1..=MAX_ADAPT_DAYS).contains(&days) {
            return Err(Error::InvalidAdaptRange(days));
        }

        let mut year = for_date.year();

        if for_date.month() == time::Month::December
            && for_date.day() > 31 - days
            && self.month == January
            && self.day < days
        {
            year += 1;
        } else if for_date.month() == time::Month::January
            && for_date.day() < days
            && self.month == December
            && self.day > 31 - days
        {
            year -= 1;
        }

        if self.month == February && self.day == 29 && !is_leap_year(year) {
            return Err(Error::OverflowNotLeapYear);
        }

        self.to_date(year)
    }
}

impl ToString for ShortDate {
    fn to_string(&self) -> String {
        format!("{:02}{}", self.day, self.month.as_str())
    }
}

impl FromStr for ShortDate {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 {
            return Err(Error::InvalidInput(s.to_owned()));
        }

        let (day, month) = s.split_at(2);

        Self::new(
            Month::from_str(month)?,
            day.parse()
                .map_err(|_| Error::InvalidInput(day.to_owned()))?,
        )
    }
}

// pub struct DateYear2 {
//     day: u32,
//     month: Month,
//     year: i32,
// }

// struct DateYear4 {
//     day: u32,
//     month: Month,
//     year: i32,
// }

/// A struct that stores HH:MM together with its timezone, optionally
/// including the seconds.
#[derive(Clone, Debug, PartialEq)]
pub struct Time {
    hour: u8,
    minute: u8,
    second: Option<u8>,
    timezone: TzTag,
}

impl Time {
    /// Constructs the struct out of given values.
    ///
    /// # Errors
    ///
    /// * Returns [`Error::InvalidHourValue`] if `hour` isn't in the `0..24` range.
    /// * Returns [`Error::InvalidMinuteValue`] if `minute` isn't in the `0..60` range.
    /// * Returns [`Error::InvalidSecondValue`] if `second` is [`Some`] and the value isn't
    /// in the `0..60` range.
    pub fn new(hour: u8, minute: u8, second: Option<u8>, timezone: TzTag) -> Result<Self, Error> {
        if hour > 23 {
            return Err(Error::InvalidHourValue(hour));
        }

        if minute > 59 {
            return Err(Error::InvalidMinuteValue(minute));
        }

        if let Some(second) = second {
            if second > 59 {
                return Err(Error::InvalidSecondValue(second));
            }
        }

        Ok(Self {
            hour,
            minute,
            second,
            timezone,
        })
    }

    /// Returns the hour component.
    pub fn hour(&self) -> u8 {
        self.hour
    }

    /// Returns the minute component.
    pub fn minute(&self) -> u8 {
        self.minute
    }

    /// Returns the second component.
    pub fn second(&self) -> Option<u8> {
        self.second
    }

    /// Returns the timezone.
    pub fn timezone(&self) -> TzTag {
        self.timezone
    }

    /// Converts this sturct into [`time::Time`].
    pub fn to_time(&self) -> time::Time {
        time::Time::from_hms(self.hour, self.minute, self.second.unwrap_or_default()).unwrap()
    }

    /// Parses the time from a "short string" -- a string of format
    /// `HHMM`.
    ///
    /// # Errors
    ///
    /// * If the input string is not exactly of length 4 -- [`Error::InvalidInput`] is returned.
    /// * If the first two bytes of the string can't be parsed as a non-negative decimal
    ///   integer -- [`Error::InvalidHour`] is returned.
    /// * If the second two bytes of the string can't be parsed as a non-negative decimal
    ///   integer -- [`Error::InvalidMinute`] is returned.
    /// * If the parsed values don't denote a valid time -- same errors as in [`Self::new()`] are
    ///   returned.
    pub fn from_short_str(s: &str) -> Result<Self, Error> {
        if s.len() != 4 {
            return Err(Error::InvalidInput(s.to_owned()));
        }

        let (hour, minute) = s.split_at(2);

        let hour = hour
            .parse()
            .map_err(|_| Error::InvalidHour(hour.to_owned()))?;

        let minute = minute
            .parse()
            .map_err(|_| Error::InvalidMinute(minute.to_owned()))?;

        Self::new(hour, minute, None, TzTag::None)
    }

    /// Parses the time from a "short string" -- a string of format
    /// `HHMMSST` or `HHMMT`.
    ///
    /// # Errors
    ///
    /// * If the input string is not exactly of length 5 or 7 -- [`Error::InvalidInput`] is returned.
    /// * If the first two bytes of the string can't be parsed as a non-negative decimal
    ///   integer -- [`Error::InvalidHour`] is returned.
    /// * If the second two bytes of the string can't be parsed as a non-negative decimal
    ///   integer -- [`Error::InvalidMinute`] is returned.
    /// * If the parsed values don't denote a valid time -- same errors as in [`Self::new()`] are
    ///   returned.
    pub fn from_full_str(s: &str) -> Result<Self, Error> {
        if s.len() != 5 && s.len() != 7 {
            return Err(Error::InvalidInput(s.to_owned()));
        }

        let (hour, tail) = s.split_at(2);
        let (minute, tail) = tail.split_at(2);

        let hour = hour
            .parse()
            .map_err(|_| Error::InvalidHour(hour.to_owned()))?;

        let minute = minute
            .parse()
            .map_err(|_| Error::InvalidMinute(minute.to_owned()))?;

        if tail.len() > 1 {
            // We have seconds and timezone
            let (seconds, tz) = tail.split_at(2);

            let seconds = seconds
                .parse::<u8>()
                .map_err(|_| Error::InvalidSecond(seconds.to_owned()))?;

            Self::new(hour, minute, Some(seconds), TzTag::from_str(tz)?)
        } else {
            // We have timezone only
            let tz = tail;

            Self::new(hour, minute, None, TzTag::from_str(tz)?)
        }
    }
}

impl Default for Time {
    fn default() -> Self {
        Self {
            hour: 0,
            minute: 0,
            second: None,
            timezone: TzTag::None,
        }
    }
}

// #[derive(Debug, Clone)]
// pub struct Time {
//     time: NaiveTime,
//     tz: TzTag,
// }

// impl Time {
//     pub fn new(time: NaiveTime, tz: TzTag) -> Self {
//         Time { time, tz }
//     }

//     pub fn tz_tag(&self) -> TzTag {
//         self.tz
//     }
// }

// impl Deref for Time {
//     type Target = NaiveTime;
//     fn deref(&self) -> &Self::Target {
//         &self.time
//     }
// }

/// A [`ShortDate`] refined with a [`Time`].
#[derive(Clone, Debug, PartialEq)]
pub struct ShortDateTime {
    pub date: ShortDate,
    pub time: Time,
}

impl ShortDateTime {
    /// Constructs a new instance of [`ShortDateTime`]. This function is equivalent
    /// to simply constructing the value.
    ///
    /// ```
    /// use iata::datetime::{ShortDateTime, ShortDate, Time, TzTag, Month};
    ///
    /// assert_eq!(
    ///     ShortDateTime::new(
    ///         ShortDate::new(Month::September, 1).unwrap(),
    ///         Time::new(20, 45, Some(12), TzTag::None).unwrap(),
    ///     ),
    ///     ShortDateTime {
    ///         date: ShortDate::new(Month::September, 1).unwrap(),
    ///         time: Time::new(20, 45, Some(12), TzTag::None).unwrap(),
    ///     },
    /// );
    /// ```
    pub fn new(date: ShortDate, time: Time) -> Self {
        Self { date, time }
    }

    /// Constructs a new instance of [`ShortDateTime`], with an optional time. The
    /// [`None`] value is mapped into `[Time::default()]`.
    pub fn new_time_opt(date: ShortDate, time: Option<Time>) -> Self {
        let time = time.unwrap_or_default();

        Self::new(date, time)
    }

    /// Returns an immutable reference to the date component.
    pub fn date(&self) -> &ShortDate {
        &self.date
    }

    /// A shortcut method that returns the month component of the date.
    pub fn month(&self) -> Month {
        self.date.month()
    }

    /// A shortcut method that returns the day component of the date.
    pub fn day(&self) -> u8 {
        self.date.day()
    }

    /// A shortcut method that returns the hour component of the time.
    pub fn hour(&self) -> u8 {
        self.time.hour()
    }

    /// A shortcut method that returns the minute component of the time.
    pub fn minute(&self) -> u8 {
        self.time.minute()
    }

    /// A shortcut method that returns the second component of the time.
    pub fn second(&self) -> Option<u8> {
        self.time.second
    }

    /// A shortcut method that returns the timezone component of the time.
    pub fn timezone(&self) -> TzTag {
        self.time.timezone
    }

    /// Converts the struct into [`time::PrimitiveDateTime`], using the supplied
    /// year.
    ///
    /// # Errors
    ///
    /// This method returns an error only if `self.date.to_date()` fails. For more
    /// information see [`ShortDate::to_date()`].
    pub fn to_datetime(&self, year: i32) -> Result<PrimitiveDateTime, Error> {
        self.date
            .to_date(year)
            .map(|date| PrimitiveDateTime::new(date, self.time.to_time()))
    }

    /// Tries to figure out what year the date-time belongs to. Equivalent to
    /// `self.to_date_adapt(&today, days)`.
    ///
    /// For more information about algorithm and some examples, see
    /// [`DayOfYear::to_date_adapt()`].
    pub fn to_datetime_adapt_year(
        &self,
        offset: UtcOffset,
        days: u8,
    ) -> Result<PrimitiveDateTime, Error> {
        self.date
            .to_date_adapt_year(offset, days)
            .map(|date| PrimitiveDateTime::new(date, self.time.to_time()))
    }

    /// Tries to figure out what year the date-time belongs to.
    ///
    /// For more information about algorithm and some examples, see
    /// [`DayOfYear::to_date_adapt()`].
    pub fn to_datetime_adapt(&self, for_date: Date, days: u8) -> Result<PrimitiveDateTime, Error> {
        self.date
            .to_date_adapt(for_date, days)
            .map(|date| PrimitiveDateTime::new(date, self.time.to_time()))
    }
}

// impl IataDateTime {

//     pub fn to_naive_datetime(&self) -> NaiveDateTime {
//         self.datetime
//     }

//     pub fn tz_tag(&self) -> TzTag {
//         self.tz
//     }
// }

// impl Deref for IataDateTime {
//     type Target = NaiveDateTime;
//     fn deref(&self) -> &Self::Target {
//         &self.datetime
//     }
// }

// pub trait ToIsoExtString {
//     fn to_iso_ext_string(&self) -> String;
// }

// impl ToIsoExtString for NaiveDate {
//     fn to_iso_ext_string(&self) -> String {
//         self.format("%Y-%m-%d").to_string()
//     }
// }

// Time         = @{ digit{4} }
// TimeFull     = @{ digit{4} ~ digit{2}? ~ ( "L" | "Z" ) }
// DateOffset   = { "-1" | "+1" | "+2" }
// DateDigit    = @{ digit{6} }
// DateShort    = { Day ~ Month }
// DateFull     = { Day ~ Month ~ Year2 }
// DateFullYear = { Day ~ Month ~ Year4 }
// Year         = { Year4 | Year2 }
// Year2        = @{ digit{2} }
// Year4        = @{ digit{4} }
// Month        = { "JAN" | "FEB" | "MAR" | "APR" | "MAY" | "JUN"
//                | "JUL" | "AUG" | "SEP" | "OCT" | "NOV" | "DEC" }
