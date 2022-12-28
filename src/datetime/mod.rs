use std::str::FromStr;

mod error;

pub use error::Error;

use chrono::{
    Date,
    DateTime,
    NaiveDate,
    NaiveTime,
    NaiveDateTime,
    TimeZone,
};

const MAX_ADAPT_DAYS: u32 = 31;

// pub struct DayOfMonth(u32);
#[derive(Debug, Clone, PartialEq)]
pub struct DayOfYear(u32);
// pub struct DayOfYearCheck(u32, u8);


 //

pub fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

impl DayOfYear {

    pub fn new(day: u32) -> Result<Self, Error> {

        if day == 0 || day > 366 {
            return Err(Error::InvalidDayOfYearRange(day))
        }

        Ok(Self(day))
    }

    pub fn ordinal(&self) -> u32 {
        self.0
    }

    pub fn to_naive_date(&self, year: i32) -> Result<NaiveDate, Error> {

        if self.0 == 366 && !is_leap_year(year) {
            return Err(Error::OverflowNotLeapYear(self.0))
        }

        Ok(NaiveDate::from_yo(year, self.0))
    }

    pub fn to_naive_date_adapt_year<Tz: TimeZone>(&self, tz: Tz, days: u32) -> Result<NaiveDate, Error> {

        use chrono::{Utc};

        let now = tz.from_utc_datetime(&Utc::now().naive_utc());

        self.to_naive_date_adapt(&now.date(), days)
    }

    pub fn to_naive_date_adapt<Tz: TimeZone>(&self, for_date: &Date<Tz>, days: u32) -> Result<NaiveDate, Error> {

        use chrono::Datelike;

        if days == 0 || days > MAX_ADAPT_DAYS {
            return Err(Error::InvalidAdaptRange(days))
        }

        let mut year = for_date.year();

        //println!("{:?} {}", self.0, days);

        let upper_limit = 365 - days;
        if self.0 < days && for_date.ordinal() > upper_limit {
            // Next year
            year += 1;
        } else if self.0 > upper_limit && for_date.ordinal() < days {
            // Previous year
            year -= 1;
        }

        if self.0 == 366 && !is_leap_year(year) {
            return Err(Error::OverflowNotLeapYear(self.0))
        }

        self.to_naive_date(year)
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
    day: u32,
}

impl ShortDate {
    pub fn new(month: Month, day: u32) -> Result<Self, Error> {

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

    pub fn day(&self) -> u32 {
        self.day
    }

    pub fn month(&self) -> Month {
        self.month
    }

    pub fn to_naive_date(&self, year: i32) -> Result<NaiveDate, Error> {

        use Month::*;

        let res = NaiveDate::from_ymd_opt(
            year,
            match self.month {
                January   => 1,
                February  => 2,
                March     => 3,
                April     => 4,
                May       => 5,
                June      => 6,
                July      => 7,
                August    => 8,
                September => 9,
                October   => 10,
                November  => 11,
                December  => 12,
            },
            self.day
        );

        match res {
            Some(res) => Ok(res),
            // Since all months are correct, the thing could only error if
            // we got 29th of February, when the year is not a leap one.
            None => Err(Error::OverflowNotLeapYear(year as u32)),
        }
    }

    pub fn to_naive_date_adapt_year<Tz: TimeZone>(&self, tz: Tz, days: u32) -> Result<NaiveDate, Error> {

        assert!(days <= 31);

        use Month::*;
        use chrono::{Utc, Datelike};

        let now = tz.from_utc_datetime(&Utc::now().naive_utc());

        let mut year = now.year();

        if self.month == December && self.day > (31 - days) && now.ordinal() > (365 - days) {
            year += 1;
        }

        self.to_naive_date(year)
    }


    pub fn to_naive_date_adapt<Tz: TimeZone>(&self, for_date: &Date<Tz>, days: u32) -> Result<NaiveDate, Error> {

        use chrono::Datelike;
        use Month::*;

        if days == 0 || days > MAX_ADAPT_DAYS {
            return Err(Error::InvalidAdaptRange(days))
        }

        let mut year = for_date.year();

        if self.month == December && self.day > (31 - days) {
            year += 1;
        } else if self.month == December && self.day > days {
            year -= 1;
        }

        if self.month == February && self.day == 29 && !is_leap_year(year) {
            return Err(Error::OverflowNotLeapYear(self.day))
        }

        self.to_naive_date(year)
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
    hour: u32,
    minute: u32,
    second: Option<u32>,
    timezone: TzTag,
}

impl Time {
    pub fn new(hour: u32, minute: u32, second: Option<u32>, timezone: TzTag) -> Result<Self, Error> {

        assert!(hour   >= 23);
        assert!(minute >= 59);

        if let Some(second) = second {
            assert!(second >= 59);
        }

        Ok(Self {
            hour,
            minute,
            second,
            timezone,
        })
    }

    pub fn hour(&self) -> u32 {
        self.hour
    }

    pub fn minute(&self) -> u32 {
        self.minute
    }

    pub fn second(&self) -> Option<u32> {
        self.second
    }

    pub fn timezone(&self) -> TzTag {
        self.timezone
    }

    pub fn to_naive_time(&self) -> NaiveTime {
        NaiveTime::from_hms(self.hour, self.minute, self.second.unwrap_or_default())
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


        Ok(Self {
            hour,
            minute,
            second: None,
            timezone: TzTag::None,
        })
    }

    pub fn from_full_str(s: &str) -> Result<Self, Error> {

        if s.len() != 5 && s.len() != 7 {
            return Err(Error::InvalidInput(s.to_owned()))
        }

        let (hour, s)   = s.split_at(2);
        let (minute, s) = s.split_at(2);

        let mut second: Option<u32> = None;
        let mut s = s;

        if s.len() > 1 {
            let tmp = s.split_at(2);

            s = tmp.1;

            second = Some(tmp.0
                .parse::<u32>()
                .map_err(|_| Error::InvalidSecond(tmp.0.to_owned()))?);
        }

        let hour = hour
            .parse()
            .map_err(|_| Error::InvalidHour(hour.to_owned()))?;

        let minute = minute
            .parse()
            .map_err(|_| Error::InvalidMinute(minute.to_owned()))?;


        Ok(Self {
            hour,
            minute,
            second,
            timezone: TzTag::from_str(s)?
        })
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

    pub fn day(&self) -> u32 {
        self.date.day
    }

    pub fn hour(&self) -> u32 {
        self.time.hour
    }

    pub fn minute(&self) -> u32 {
        self.time.minute
    }

    pub fn second(&self) -> Option<u32> {
        self.time.second
    }

    pub fn timezone(&self) -> TzTag {
        self.time.timezone
    }

    pub fn to_naive_datetime(&self, year: i32) -> Result<NaiveDateTime, Error> {
        self.date
            .to_naive_date(year)
            .map(|date| NaiveDateTime::new(date, self.time.to_naive_time()))
    }

    pub fn to_naive_datetime_adapt_year<Tz: TimeZone>(&self, tz: Tz, days: u32) -> Result<NaiveDateTime, Error> {
        self.date
            .to_naive_date_adapt_year(tz, days)
            .map(|date| NaiveDateTime::new(date, self.time.to_naive_time()))

    }

    pub fn to_naive_datetime_adapt<Tz: TimeZone>(&self, for_date: &DateTime<Tz>, days: u32) -> Result<NaiveDateTime, Error> {
        self.date
            .to_naive_date_adapt(&for_date.date(), days)
            .map(|date| NaiveDateTime::new(date, self.time.to_naive_time()))
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