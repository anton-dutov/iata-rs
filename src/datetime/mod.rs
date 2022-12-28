use std::str::FromStr;

mod error;

pub use error::Error;

use time::{
    Date,
    UtcOffset,
    OffsetDateTime,
    PrimitiveDateTime,
};

const MAX_ADAPT_DAYS: u16 = 31;

// pub struct DayOfMonth(u32);
#[derive(Debug, Clone, PartialEq)]
pub struct DayOfYear(u16);
// pub struct DayOfYearCheck(u32, u8);


 //

pub fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

impl DayOfYear {

    pub fn new(day: u16) -> Result<Self, Error> {

        if day == 0 || day > 366 {
            return Err(Error::InvalidDayOfYearRange(day))
        }

        Ok(Self(day))
    }

    pub fn ordinal(&self) -> u16 {
        self.0
    }

    pub fn to_date(&self, year: i32) -> Result<Date, Error> {
        Date::from_ordinal_date(year, self.0)
        .map_err(|_| Error::OverflowNotLeapYear)
    }

    pub fn to_date_adapt_year(&self, offset: UtcOffset, days: u16) -> Result<Date, Error> {

        let now = OffsetDateTime::now_utc().to_offset(offset);

        self.to_date_adapt(now.date(), days)
    }

    pub fn to_date_adapt(&self, for_date: Date, days: u16) -> Result<Date, Error> {

        if days == 0 || days > MAX_ADAPT_DAYS {
            return Err(Error::InvalidAdaptRange(days))
        }

        let mut year = for_date.year();

        let upper_limit = 365 - days;
        if self.0 < days && for_date.ordinal() > upper_limit {
            // Next year
            year += 1;
        } else if self.0 > upper_limit && for_date.ordinal() < days {
            // Previous year
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


#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TzTag {
    None,
    Local,
    Utc,
}

impl TzTag {
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




#[derive(Clone, Debug, PartialEq)]
pub struct ShortDate {
    month: Month,
    day: u8,
}

impl ShortDate {
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

    pub fn day(&self) -> u8 {
        self.day
    }

    pub fn month(&self) -> Month {
        self.month
    }

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

    pub fn to_date_adapt_year(&self, offset: UtcOffset, days: u16) -> Result<Date, Error> {

        let now = OffsetDateTime::now_utc().to_offset(offset);

        self.to_date_adapt(now.date(), days)
    }


    pub fn to_date_adapt(&self, for_date: Date, days: u16) -> Result<Date, Error> {

        use Month::*;

        if days == 0 || days > MAX_ADAPT_DAYS {
            return Err(Error::InvalidAdaptRange(days))
        }

        let mut year = for_date.year();

        if self.month == December && self.day as u16 > (31 - days) {
            year += 1;
        } else if self.month == December && self.day as u16 > days {
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

#[derive(Clone, Debug, PartialEq)]
pub struct Time {
    hour: u8,
    minute: u8,
    second: Option<u8>,
    timezone: TzTag,
}

impl Time {
    pub fn new(hour: u8, minute: u8, second: Option<u8>, timezone: TzTag) -> Result<Self, Error> {

        if hour >= 23 {
            return Err(Error::InvalidHourValue(hour));
        }

        if minute >= 59 {
            return Err(Error::InvalidMinuteValue(minute));
        }

        if let Some(second) = second {
            if second >= 59 {
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

    pub fn to_datetime_adapt_year(&self, offset: UtcOffset, days: u16) -> Result<PrimitiveDateTime, Error> {
        self.date
            .to_date_adapt_year(offset, days)
            .map(|date| PrimitiveDateTime::new(date, self.time.to_time()))

    }

    pub fn to_datetime_adapt(&self, for_date: Date, days: u16) -> Result<PrimitiveDateTime, Error> {
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