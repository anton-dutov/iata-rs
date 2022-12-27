use std::str::FromStr;

mod error;

pub use error::Error;

use time::{
    Date,
    UtcOffset,
    OffsetDateTime,
    PrimitiveDateTime,
};

const MAX_ADAPT_DAYS: u8 = 31;

// pub struct DayOfMonth(u32);

/// Contains a number of some day in a year, which is an integer that ranges from
/// 1 to 366 (inclusively). In some places this data structure is refered to as
/// "ordinal".
///
/// This structure DOES NOT guarantee that it contains
/// a day number valid for any year, since a year maybe contain one extra day.
#[derive(Debug, Clone, PartialEq)]
pub struct DayOfYear(u16);
// pub struct DayOfYearCheck(u32, u8);

/// Helper function to check if a given year is actually a leap one.
pub fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

impl DayOfYear {
    /// Constructs a new instance of [`DayOfYear`] based off the day number.
    /// All numbers not within range `1..=366` are accepted.
    ///
    /// # Errors
    /// If `day` is not from `1..=366` [`Error::InvalidDayOfYearRange`] is returned.
    ///
    /// # Exmaple
    ///
    /// ```
    /// use iata::datetime::DayOfYear;
    ///
    /// // Ok
    /// assert!(DayOfYear::new(366).is_ok());
    /// assert!(DayOfYear::new(10).is_ok());
    ///
    /// // Not ok
    /// assert!(DayOfYear::new(0).is_err());
    /// assert!(DayOfYear::new(400).is_err());
    /// ```
    pub fn new(day: u16) -> Result<Self, Error> {

        if day == 0 || day > 366 {
            return Err(Error::InvalidDayOfYearRange(day))
        }

        Ok(Self(day))
    }

    /// Returns the day value, wrapped by this struct.
    pub fn ordinal(&self) -> u16 {
        self.0
    }

    /// Converts this date into `NaiveDate` date type from `chrono`. If it turns out that
    /// `year` is not a leap year and the day is 366'th, [`Error::OverflowNotLeapYear`]
    /// is returned.
    pub fn to_date(&self, year: i32) -> Result<Date, Error> {
        Date::from_ordinal_date(year, self.0)
        .map_err(|_| Error::OverflowNotLeapYear)
    }

    /// Tries to figure out what year the day stored in `self` belongs to. Equivalent to
    /// `self.to_naive_date_adapt(&today, days)`.
    ///
    /// For more information about algorithm and some examples, see
    /// [`Self::to_naive_date_adapt()`].
    pub fn to_date_adapt_year(&self, offset: UtcOffset, days: u8) -> Result<Date, Error> {

        let now = OffsetDateTime::now_utc().to_offset(offset);

        self.to_date_adapt(now.date(), days)
    }

    /// Tries to figure out what year the day stored in `self` belongs to. Here are the
    /// techniques used by the function (here "today's date" is whatever is stored in `for_date`):
    ///
    /// 1. If `self.ordinal()` is more than `days` days earlier than today's date,
    /// `self.ordinal()` is not more than `days` days later than the start of year -- it
    /// is considered that `self` belongs to the next year.
    /// 2. If `self.ordinal()` is less than `days` days away from year's end, today's date
    /// is not more than `days` days later than the start of the year -- it is considered that
    /// `self` belongs to the previous year.
    /// 3. Otherwise we consider `self` belonging to the same year today's date belongs to.
    ///
    /// # Errors
    ///
    /// * If `days` is greater than 31 -- [`Error::InvalidAdaptRange`] is returned.
    /// * If `self.ordinal()` is 366 and and the algorithm has decided that `self` belongs to
    /// a non-leap year -- [`Error::OverflowNotLeapYear`] is returned.
    ///
    /// # Examples
    ///
    /// Let's assume it's 364'th dat of 2015 and we are shown various boarding passes. The boarding passes
    /// don't contain precise dates, instead they contain day of year only (i.e. only the ordinal day).
    ///
    /// it is also a common knowledge, that people can buy tickets with boarding that happens next year.
    /// Because of that, day 4 doesn't mean it's for 2015. It might be a boarding pass for 2016! Strictly
    /// speaking, we will never learn if it's true or not, but assuming that such boarding pass is for 2016
    /// is a good educated guess.
    ///
    /// Of course, this rule shouldn't be abused and that's where the `days` arguments comes in.
    ///
    /// ```
    /// use iata::datetime::DayOfYear;
    /// use chrono::{Utc, NaiveDate, TimeZone};
    ///
    /// let date = Utc.yo(2015, 364);
    ///
    /// // It's day `4`, `days` (the allowed range) is 7. The function will
    /// // assume it's the boarding pass for the next year.
    /// assert_eq!(
    ///     DayOfYear::new(4).unwrap().to_naive_date_adapt(&date, 7),
    ///     Ok(NaiveDate::from_yo_opt(2016, 4).unwrap()),
    /// );
    /// // It's day `4`, `days` (the allowed range) is 2. The function will
    /// // assume it's the boarding pass for this year, since the date is out
    /// // of the allowed range.
    /// assert_eq!(
    ///     DayOfYear::new(4).unwrap().to_naive_date_adapt(&date, 2),
    ///     Ok(NaiveDate::from_yo_opt(2015, 4).unwrap()),
    /// );
    /// ```
    ///
    /// Of course, the situation can be complete 180. It might be the 4th day of 2015 and
    /// the boarding pass seems to contain the date far in the future. In that case it would
    /// be good to assume that this boarding pass was actually for the past year!
    ///
    /// ```
    /// use iata::datetime::DayOfYear;
    /// use chrono::{Utc, NaiveDate, TimeZone};
    ///
    /// let date = Utc.yo(2015, 4);
    ///
    /// assert_eq!(
    ///     DayOfYear::new(364).unwrap().to_naive_date_adapt(&date, 7),
    ///     Ok(NaiveDate::from_yo_opt(2014, 364).unwrap()),
    /// );
    /// ```
    pub fn to_date_adapt(&self, for_date: Date, days: u8) -> Result<Date, Error> {

        if days == 0 || days > MAX_ADAPT_DAYS {
            return Err(Error::InvalidAdaptRange(days))
        }

        let mut year = for_date.year();

        let days = days as u16;
        let upper_limit = 365 - days;
        if self.0 < days && for_date.ordinal() > upper_limit {
            year += 1;
        } else if self.0 > upper_limit && for_date.ordinal() < days {
            year -= 1;
        }

        if self.0 == 366 && !is_leap_year(year) {
            return Err(Error::OverflowNotLeapYear)
        }

        self.to_date(year)
    }
}

impl Default for DayOfYear {
    fn default() -> Self { Self(1) }
}

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
            TzTag::Utc   => Some("Z"),
            TzTag::None  => None,
        }
    }
}


impl FromStr for TzTag {

    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        Ok(match s {
            "l" | "L" => TzTag::Local,
            "z" | "Z" => TzTag::Utc,
            other => return Err(Error::InvalidTimezoneTag(other.into()))
        })
    }
}

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
            January   => "JAN",
            February  => "FEB",
            March     => "MAR",
            April     => "APR",
            May       => "MAY",
            June      => "JUN",
            July      => "JUL",
            August    => "AUG",
            September => "SEP",
            October   => "OCT",
            November  => "NOV",
            December  => "DEC",
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
            other => return Err(Error::InvalidMonth(other.into()))
        })
    }
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
    /// 29th of Febuary.
    ///
    /// # Errors
    /// All attempts to give the function an illegal day of month will result in
    /// [`Error::InvalidDayForMonth`].
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
            January   => 31,
            February  => 29,
            March     => 31,
            April     => 30,
            May       => 31,
            June      => 30,
            July      => 31,
            August    => 31,
            September => 30,
            October   => 31,
            November  => 30,
            December  => 31,
        };

        if day == 0 || day > max {
            return Err(Error::InvalidDayForMonth(month, day))
        }

        Ok(Self {
            month,
            day,
        })
    }

    /// Returns the day component of the date.
    pub fn day(&self) -> u8 {
        self.day
    }

    /// Returns the month component of the date.
    pub fn month(&self) -> Month {
        self.month
    }

    // NOTE doesn't actually ever error. That seems to be an implementation error though.
    /// Converts the date into the [`NaiveDate`] type from `chrono`.
    pub fn to_date(&self, year: i32) -> Result<Date, Error> {

        use Month::*;

        let res = Date::from_calendar_date(
            year,
            match self.month {
                January   => time::Month::January,
                February  => time::Month::February,
                March     => time::Month::March,
                April     => time::Month::April,
                May       => time::Month::May,
                June      => time::Month::June,
                July      => time::Month::July,
                August    => time::Month::August,
                September => time::Month::September,
                October   => time::Month::October,
                November  => time::Month::November,
                December  => time::Month::December,
            },
            self.day
        );

        match res {
            Ok(res) => Ok(res),
            // Since all months are correct, the thing could only error if
            // we got 29th of February, when the year is not a leap one.
            Err(_) => Err(Error::OverflowNotLeapYear),
        }
    }

    /// Tries to figure out what year the date belongs to. Equivalent to
    /// `self.to_naive_date_adapt(&today, days)`.
    ///
    /// For more information about algorithm and some examples, see
    /// [`DayOfYear::to_naive_date_adapt()`].
    pub fn to_date_adapt_year(&self, offset: UtcOffset, days: u8) -> Result<Date, Error> {

        let now = OffsetDateTime::now_utc().to_offset(offset);

        self.to_date_adapt(now.date(), days)
    }

    /// Tries to figure out what year the date belongs to.
    ///
    /// For more information about algorithm and some examples, see
    /// [`DayOfYear::to_naive_date_adapt()`].
    pub fn to_date_adapt(&self, for_date: Date, days: u8) -> Result<Date, Error> {

        use Month::*;

        if days == 0 || days > MAX_ADAPT_DAYS {
            return Err(Error::InvalidAdaptRange(days))
        }

        let mut year = for_date.year();

        if for_date.month() == time::Month::December && for_date.day() > 31 - days &&
            self.month == January && self.day < days
        {
            year += 1;
        } else if for_date.month() == time::Month::January && for_date.day() < days &&
            self.month == December && self.day > 31 - days
        {
            year -= 1;
        }

        if self.month == February && self.day == 29 && !is_leap_year(year) {
            return Err(Error::OverflowNotLeapYear)
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
            return Err(Error::InvalidInput(s.to_owned()))
        }

        let (day, month) = s.split_at(2);

        Self::new(
            Month::from_str(month)?,
            day.parse()
               .map_err(|_| Error::InvalidInput(day.to_owned()))?
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

    pub fn hour(&self) -> u8 {
        self.hour
    }

    pub fn minute(&self) -> u8 {
        self.minute
    }

    pub fn second(&self) -> Option<u8> {
        self.second
    }

    pub fn timezone(&self) -> TzTag {
        self.timezone
    }

    pub fn to_time(&self) -> time::Time {
        time::Time::from_hms(self.hour, self.minute, self.second.unwrap_or_default()).unwrap()
    }

    pub fn from_short_str(s: &str) -> Result<Self, Error> {

        if s.len() != 4 {
            return Err(Error::InvalidInput(s.to_owned()))
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

    pub fn from_full_str(s: &str) -> Result<Self, Error> {

        if s.len() != 5 && s.len() != 7 {
            return Err(Error::InvalidInput(s.to_owned()))
        }

        let (hour, tail)   = s.split_at(2);
        let (minute, tail) = tail.split_at(2);

        let hour = hour
            .parse()
            .map_err(|_| Error::InvalidHour(hour.to_owned()))?;

        let minute = minute
            .parse()
            .map_err(|_| Error::InvalidMinute(minute.to_owned()))?;

        if tail.len() > 1 { // We have seconds and timezone
            let (seconds, tz) = tail.split_at(2);

            let seconds = seconds
                .parse::<u8>()
                .map_err(|_| Error::InvalidSecond(seconds.to_owned()))?;

            Self::new(hour, minute, Some(seconds), TzTag::from_str(tz)?)
        } else { // We have timezone only
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
            timezone: TzTag::None
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


#[derive(Clone, Debug, PartialEq)]
pub struct ShortDateTime {
    pub date: ShortDate,
    pub time: Time,
}

impl ShortDateTime {

    pub fn new(date: ShortDate, time: Time) -> Self {
        Self { date, time }
    }

    pub fn new_time_opt(date: ShortDate, time: Option<Time>) -> Self {
        let time = time.unwrap_or_default();

        Self::new(date, time)
    }

    pub fn date(&self) -> &ShortDate {
        &self.date
    }

    pub fn month(&self) -> Month {
        self.date.month
    }

    pub fn day(&self) -> u8 {
        self.date.day
    }

    pub fn hour(&self) -> u8 {
        self.time.hour
    }

    pub fn minute(&self) -> u8 {
        self.time.minute
    }

    pub fn second(&self) -> Option<u8> {
        self.time.second
    }

    pub fn timezone(&self) -> TzTag {
        self.time.timezone
    }

    pub fn to_datetime(&self, year: i32) -> Result<PrimitiveDateTime, Error> {
        self.date
            .to_date(year)
            .map(|date| PrimitiveDateTime::new(date, self.time.to_time()))
    }

    pub fn to_datetime_adapt_year(&self, offset: UtcOffset, days: u8) -> Result<PrimitiveDateTime, Error> {
        self.date
            .to_date_adapt_year(offset, days)
            .map(|date| PrimitiveDateTime::new(date, self.time.to_time()))

    }

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