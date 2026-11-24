mod day_of_year;
mod error;
mod month;
mod month_day;
mod month_day_time;
mod time;
mod time_zone_indicator;

pub use day_of_year::DayOfYear;
pub use error::Error;
pub use month::Month;
pub use month_day::MonthDay;
pub use month_day_time::MonthDayTime;
pub use time::Time;
pub use time_zone_indicator::TimeZoneIndicator;

use jiff::{
    civil::Date,
    tz::{Offset, TimeZone},
    Timestamp,
};

const MAX_ADAPT_DAYS: u8 = 31;

// pub struct DayOfMonth(u32);

// pub struct DayOfYearCheck(u32, u8);

// The adjacent year may be just outside Jiff's date range. Its length is
// still needed to decide whether to cross the boundary or keep the current
// year. The Gregorian calendar repeats every 400 years.
fn days_in_year(year: i16) -> u16 {
    let cycle_year = year.rem_euclid(400);
    jiff::civil::date(cycle_year, 1, 1).days_in_year() as u16
}

// Only January and December can cross a year boundary. Evaluate the target
// ordinal in the candidate year, whose December can start on day 335 or 336.
fn adapted_year(
    reference: Date,
    days: u8,
    ordinal_in: impl FnOnce(i16) -> Option<u16>,
) -> Result<i16, Error> {
    if !(1..=MAX_ADAPT_DAYS).contains(&days) {
        return Err(Error::InvalidAdaptRange(days));
    }
    let year = reference.year();
    let day = reference.day() as u8;
    match reference.month() {
        12 if day > 31 - days => {
            if ordinal_in(year + 1).is_some_and(|ordinal| ordinal < u16::from(days)) {
                return Ok(year + 1);
            }
        }
        1 if day < days => {
            let cutoff = days_in_year(year - 1) - u16::from(days);
            if ordinal_in(year - 1).is_some_and(|ordinal| ordinal > cutoff) {
                return Ok(year - 1);
            }
        }
        _ => {}
    }
    Ok(year)
}

// pub struct DateYear2 {
//     day: u32,
//     month: Month,
//     year: i16,
// }

// struct DateYear4 {
//     day: u32,
//     month: Month,
//     year: i16,
// }

// #[derive(Debug, Clone)]
// pub struct Time {
//     time: NaiveTime,
//     tz: TimeZoneIndicator,
// }

// impl Time {
//     pub fn new(time: NaiveTime, tz: TimeZoneIndicator) -> Self {
//         Time { time, tz }
//     }

//     pub fn time_zone_indicator(&self) -> TimeZoneIndicator {
//         self.tz
//     }
// }

// impl Deref for Time {
//     type Target = NaiveTime;
//     fn deref(&self) -> &Self::Target {
//         &self.time
//     }
// }

// impl IataDateTime {

//     pub fn to_naive_datetime(&self) -> NaiveDateTime {
//         self.datetime
//     }

//     pub fn time_zone_indicator(&self) -> TimeZoneIndicator {
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
