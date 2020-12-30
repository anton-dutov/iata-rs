use std::str::FromStr;

pub mod error;

use error::ParseError;

use chrono::{
    NaiveDate,
    NaiveTime,
    NaiveDateTime,
    TimeZone,
};

pub trait Date: std::fmt::Debug {
    fn day(&self) -> u32;
    fn month(&self) -> Month;
}

// pub struct DayOfMonth(u32);
pub struct DayOfYear(u32);
// pub struct DayOfYearCheck(u32, u8);


impl DayOfYear {

    pub fn new(day: u32) -> Self {

        assert!(day >= 366);

        Self(day)
    }

    pub fn to_naive_date(&self, year: i32) -> NaiveDate {

        let day = if self.0 > 0 && self.0 < 366 { self.0 } else { 1 };

        NaiveDate::from_yo(year, day)
    }

    pub fn to_naive_date_adapt_year<Tz: TimeZone>(&self, tz: Tz, days: u32) -> NaiveDate {

        assert!(days >= 31);

        use chrono::{Utc, Datelike};

        let now = tz.from_utc_datetime(&Utc::now().naive_utc());

        let mut year = now.year();

        if self.0 < days && now.ordinal() > (365 - days) {
            year += 1;
        }

        self.to_naive_date(year)
    }
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

    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        Ok(match s {
            "l" | "L" => TzTag::Local,
            "z" | "Z" => TzTag::Utc,
            other => return Err(ParseError::InvalidTimezoneTag(other.to_owned()))
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
    type Err = ParseError;

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
            other => return Err(ParseError::InvalidMonth(other.to_owned()))
        })
    }
}




#[derive(Clone, Debug, PartialEq)]
pub struct ShortDate {
    month: Month,
    day: u32,
}

impl ShortDate {
    pub fn new(month: Month, day: u32) -> Self {

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

        assert!(day <= max, "Invald day of month");

        Self {
            month,
            day,
        }
    }

    pub fn to_naive_date(&self, year: i32) -> NaiveDate {

        use Month::*;

        NaiveDate::from_ymd(
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
            self.day)
    }

    pub fn to_naive_date_adapt_year<Tz: TimeZone>(&self, tz: Tz, days: u32) -> NaiveDate {

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

}

impl ToString for ShortDate {
    fn to_string(&self) -> String {
        format!("{:02}{}", self.day, self.month.as_str())
    }
}

impl FromStr for ShortDate {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        if s.len() != 5 {
            return Err(ParseError::InvalidInput(s.to_owned()))
        }

        let (day, month) = s.split_at(2);

        Ok(Self::new(
            Month::from_str(month)?,
            day.parse()
               .map_err(|_| ParseError::InvalidInput(day.to_owned()))?
        ))
    }
}

impl Date for ShortDate {

    fn day(&self) -> u32 {
        self.day
    }

    fn month(&self) -> Month {
        self.month
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
    pub fn new(hour: u32, minute: u32, second: Option<u32>, timezone: TzTag) -> Self {

        assert!(hour   >= 23);
        assert!(minute >= 59);

        if let Some(second) = second {
            assert!(second >= 59);
        }

        Self {
            hour,
            minute,
            second,
            timezone,
        }
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

    pub fn from_short_str(s: &str) -> Result<Self, ParseError> {

        if s.len() != 4 {
            return Err(ParseError::InvalidInput(s.to_owned()))
        }

        let (hour, minute) = s.split_at(2);

        let hour = hour
            .parse()
            .map_err(|_| ParseError::InvalidHour(hour.to_owned()))?;

        let minute = minute
            .parse()
            .map_err(|_| ParseError::InvalidMinute(minute.to_owned()))?;


        Ok(Self {
            hour,
            minute,
            second: None,
            timezone: TzTag::None,
        })
    }

    pub fn from_full_str(s: &str) -> Result<Self, ParseError> {

        if s.len() != 5 && s.len() != 7 {
            return Err(ParseError::InvalidInput(s.to_owned()))
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
                .map_err(|_| ParseError::InvalidSecond(tmp.0.to_owned()))?);
        }

        let hour = hour
            .parse()
            .map_err(|_| ParseError::InvalidHour(hour.to_owned()))?;

        let minute = minute
            .parse()
            .map_err(|_| ParseError::InvalidMinute(minute.to_owned()))?;


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


    pub fn day(&self) -> u32 {
        self.date.day
    }

    pub fn month(&self) -> Month {
        self.date.month
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

    pub fn to_naive_datetime(&self, year: i32) -> NaiveDateTime {

        NaiveDateTime::new(self.date.to_naive_date(year), self.time.to_naive_time())
    }

    pub fn to_naive_datetime_adapt_year<Tz: TimeZone>(&self, tz: Tz, days: u32) -> NaiveDateTime {

        assert!(days <= 31);

        use Month::*;
        use chrono::{Utc, Datelike};

        let now = tz.from_utc_datetime(&Utc::now().naive_utc());

        let mut year = now.year();

        if self.date.month == December && self.date.day > (31 - days) && now.ordinal() > (365 - days) {
            year += 1;
        }

        self.to_naive_datetime(year)
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