use std::str::FromStr;

use iata::datetime::*;
use rand::Rng;

const DAYS_IN_YEAR: u16 = 365;
const MONTH_LENS: [(Month, u8); 12] = [
    (Month::January,    31),
    (Month::February,   29),
    (Month::March,      31),
    (Month::April,      30),
    (Month::May,        31),
    (Month::June,       30),
    (Month::July,       31),
    (Month::August,     31),
    (Month::September,  30),
    (Month::October,    31),
    (Month::November,   30),
    (Month::December,   31),
];
const TIMEZONE_TAGS: [TzTag; 3] = [
    TzTag::Local,
    TzTag::None,
    TzTag::Utc,
];
const MIN_YEAR: i32 = 1997;
const MAX_YEAR: i32 = 3000;

#[test]
fn test_month_as_str() {
    use Month::*;

    assert_eq!(January  .as_str(), "JAN");
    assert_eq!(February .as_str(), "FEB");
    assert_eq!(March    .as_str(), "MAR");
    assert_eq!(April    .as_str(), "APR");
    assert_eq!(May      .as_str(), "MAY");
    assert_eq!(June     .as_str(), "JUN");
    assert_eq!(July     .as_str(), "JUL");
    assert_eq!(August   .as_str(), "AUG");
    assert_eq!(September.as_str(), "SEP");
    assert_eq!(October  .as_str(), "OCT");
    assert_eq!(November .as_str(), "NOV");
    assert_eq!(December .as_str(), "DEC");
}

#[test]
fn test_month_to_as_str() {
    use Month::*;

    assert_eq!(Month::from_str(January  .as_str()).unwrap(), January    );
    assert_eq!(Month::from_str(February .as_str()).unwrap(), February   );
    assert_eq!(Month::from_str(March    .as_str()).unwrap(), March      );
    assert_eq!(Month::from_str(April    .as_str()).unwrap(), April      );
    assert_eq!(Month::from_str(May      .as_str()).unwrap(), May        );
    assert_eq!(Month::from_str(June     .as_str()).unwrap(), June       );
    assert_eq!(Month::from_str(July     .as_str()).unwrap(), July       );
    assert_eq!(Month::from_str(August   .as_str()).unwrap(), August     );
    assert_eq!(Month::from_str(September.as_str()).unwrap(), September  );
    assert_eq!(Month::from_str(October  .as_str()).unwrap(), October    );
    assert_eq!(Month::from_str(November .as_str()).unwrap(), November   );
    assert_eq!(Month::from_str(December .as_str()).unwrap(), December   );
}

#[test]
fn test_day_of_year_valid() {
    for day in 1..=(DAYS_IN_YEAR + 1) {
        assert!(DayOfYear::new(1).is_ok(), "Day {day} must be a valid day of year");
    }
}

#[test]
fn test_day_of_year_invalid() {
    for day in std::iter::once(0).chain((DAYS_IN_YEAR + 2)..=std::u16::MAX) {
        assert_eq!(
            DayOfYear::new(day),
            Err(Error::InvalidDayOfYearRange(day)),
            "Day {day} must be an invalid day of year"
        );
    }
}

#[test]
fn test_day_of_year_leap_year() {
    for year in MIN_YEAR..MAX_YEAR {
        assert_eq!(
            is_leap_year(year),
            DayOfYear::new(366).unwrap().to_date(year).is_ok()
        );
        if !is_leap_year(year) {
            assert_eq!(DayOfYear::new(366).unwrap().to_date(year), Err(Error::OverflowNotLeapYear));
        }
    }
}

#[test]
fn test_day_of_year_to_date() {

    use time::Date;

    for year in MIN_YEAR..MAX_YEAR {
        let it =
            (1..=DAYS_IN_YEAR)
                .filter_map(|day|
                    Date::from_ordinal_date(year, day)
                    .map(|x| (day, x))
                    .ok()
                );

        for (day, date) in it {
            assert_eq!(DayOfYear::new(day).unwrap().to_date(year), Ok(date));
        }
    }
}

#[test]
fn test_day_of_year_to_date_adapt() {
    use time::{Date, Month};

    const WINDOW_SIZE_MAX: u16 = 31;

    // When ticket is for the next year
    let next_year_test = |window, year| {
        for offset in 0..window {
            for day in 1..window {
                assert_eq!(
                    DayOfYear::new(day).unwrap()
                    .to_date_adapt(
                        Date::from_calendar_date(year, Month::December, 31 - offset as u8).unwrap(),
                        window as u8
                    ),
                    Ok(Date::from_calendar_date(year + 1, Month::January, day as u8).unwrap())
                );
            }
        }
    };

    // When ticket is for the previous year
    let prev_year_test = |window, year| {
        for offset in 1..window {
            let edge_date = DayOfYear::new(DAYS_IN_YEAR + 1).unwrap()
            .to_date_adapt(
                Date::from_calendar_date(year, Month::January, offset as u8).unwrap(),
                window as u8
            );

            if is_leap_year(year - 1) {
                assert_eq!(edge_date, Ok(Date::from_ordinal_date(year - 1, DAYS_IN_YEAR + 1).unwrap()));
            } else {
                assert_eq!(edge_date, Err(Error::OverflowNotLeapYear));
            }

            for day in (1..window).map(|x| DAYS_IN_YEAR - x + 1) {
                assert_eq!(
                    DayOfYear::new(day).unwrap()
                    .to_date_adapt(
                        Date::from_calendar_date(year, Month::January, offset as u8).unwrap(),
                        window as u8
                    ),
                    Ok(Date::from_ordinal_date(year - 1, day).unwrap())
                );
            }
        }
    };

    for window in 1..WINDOW_SIZE_MAX {
        (MIN_YEAR..(MAX_YEAR - 1)).for_each(|year| next_year_test(window, year));
        ((MIN_YEAR+1)..MAX_YEAR).for_each(|year| prev_year_test(window, year));
    }
}

#[test]
fn test_tz_as_str() {
    assert_eq!(TzTag::Local.as_str().as_deref(), Some("L"));
    assert_eq!(TzTag::Utc.as_str().as_deref(), Some("Z"));
    assert_eq!(TzTag::None.as_str(), None);
}

#[test]
fn test_short_date() {
    for (month, len) in MONTH_LENS {
        for day in 1..=len {
            let short_date = ShortDate::new(month, day)
                .expect(&format!("{day} must be a valid of day of {}", month.as_str()));

            assert_eq!(short_date.day(), day);
            assert_eq!(short_date.month(), month);
            assert_eq!(short_date.to_string(), format!("{day:02}{}", month.as_str()));
        }
    }
}

#[test]
fn test_short_date_invalid() {
    for (month, len) in MONTH_LENS {
        let it =
            std::iter::once(0)
            .chain((len+1)..u8::MAX);

        for day in it {
            assert_eq!(ShortDate::new(month, day), Err(Error::InvalidDayForMonth(month, day)));
        }
    }
}

#[test]
fn test_short_date_leap_year() {
    for year in MIN_YEAR..MAX_YEAR {
        assert_eq!(
            is_leap_year(year),
            ShortDate::new(Month::February, 29).unwrap().to_date(year).is_ok()
        );
        if !is_leap_year(year) {
            assert_eq!(
                ShortDate::new(Month::February, 29).unwrap().to_date(year),
                Err(Error::OverflowNotLeapYear)
            );
        }
    }
}

#[test]
fn test_short_date_to_date_adapt() {
    use time::{Date};

    const WINDOW_SIZE_MAX: u16 = 31;

    // When ticket is for the next year
    let next_year_test = |window, year| {
        for offset in 0..window {
            for day in 1..window {
                assert_eq!(
                    ShortDate::new(Month::January, day as u8).unwrap()
                    .to_date_adapt(
                        Date::from_calendar_date(year, time::Month::December, 31 - offset as u8).unwrap(),
                        window as u8
                    ),
                    Ok(Date::from_calendar_date(year + 1, time::Month::January, day as u8).unwrap())
                );
            }
        }
    };

    // When ticket is for the previous year
    let prev_year_test = |window, year| {
        for offset in 1..window {
            for day in (1..window).map(|x| 31 - x + 1) {
                assert_eq!(
                    ShortDate::new(Month::December, day as u8).unwrap()
                    .to_date_adapt(
                        Date::from_calendar_date(year, time::Month::January, offset as u8).unwrap(),
                        window as u8
                    ),
                    Ok(Date::from_calendar_date(year - 1, time::Month::December, day as u8).unwrap())
                );
            }
        }
    };

    for window in 1..WINDOW_SIZE_MAX {
        (MIN_YEAR..(MAX_YEAR - 1)).for_each(|year| next_year_test(window, year));
        ((MIN_YEAR+1)..MAX_YEAR).for_each(|year| prev_year_test(window, year));
    }
}

#[test]
fn test_short_date_to_string_from_string() {
    for (month, len) in MONTH_LENS {
        for day in 1..=len {
            assert_eq!(
                ShortDate::from_str(&ShortDate::new(month, day).unwrap().to_string()),
                ShortDate::new(month, day),
            );
        }
    }
}

#[test]
fn test_short_date_bad_strs() {
    const MAX_SAMPLE_LEN: usize = 1_000_00;
    use rand::{distr::{Alphanumeric, SampleString}};
    let mut rng = rand::rng();

    for _ in 0..1000 {
        let len = rng.random_range(0..MAX_SAMPLE_LEN);

        let s = Alphanumeric.sample_string(&mut rng, len);

        if len != 5 {
            assert!(ShortDate::from_str(&s).is_err());
        }
    }
}

#[test]
fn test_time_bad_strs() {
    const MAX_SAMPLE_LEN: usize = 1_000_00;
    use rand::{distr::{Alphanumeric, SampleString}};
    let mut rng = rand::rng();

    for _ in 0..1000 {

        let len = rng.random_range(0..MAX_SAMPLE_LEN);

        let s = Alphanumeric.sample_string(&mut rng, len);

        if len != 4 {
            assert!(Time::from_short_str(&s).is_err());
        }

        if len != 5 && len != 7 {
            assert!(Time::from_full_str(&s).is_err());
        }
    }
}

#[test]
fn test_time_valid() {
    for hour in 0..24 {
        for minute in 0..60 {

            assert!(Time::from_short_str(&format!("{hour:02}{minute:02}")).is_ok());

            for timezone in TIMEZONE_TAGS {
                let tz_str = timezone.as_str();
                if let Some(tz) = tz_str {
                    assert!(Time::from_full_str(&format!("{hour:02}{minute:02}{tz}")).is_ok());
                    let tz = tz.to_lowercase();
                    assert!(Time::from_full_str(&format!("{hour:02}{minute:02}{tz}")).is_ok());
                }

                for second in std::iter::once(None).chain((0..60).map(Some)) {
                    let time = Time::new(hour, minute, second, timezone);
                    assert!(time.is_ok());
                    assert_eq!(
                        time.unwrap().to_time(),
                        time::Time::from_hms(hour, minute, second.unwrap_or_default()).unwrap()
                    );

                    if let (Some(tz), Some(second)) = (tz_str, second) {
                        assert!(Time::from_full_str(&format!("{hour:02}{minute:02}{second:02}{tz}")).is_ok());
                        let tz = tz.to_lowercase();
                        assert!(Time::from_full_str(&format!("{hour:02}{minute:02}{second:02}{tz}")).is_ok());
                    }
                }
            }
        }
    }
}

#[test]
fn test_time_invalid() {
    for hour in 24..u8::MAX {
        for minute in 60..u8::MAX {
            for timezone in TIMEZONE_TAGS {
                for second in (60..u8::MAX).map(Some) {
                    let time = Time::new(hour, minute, second, timezone);
                    assert!(time.is_err());
                }
            }
        }
    }

    for hour in 24..100 {
        for minute in 60..100 {

            assert!(Time::from_short_str(&format!("{hour:02}{minute:02}")).is_err());

            for timezone in TIMEZONE_TAGS {

                let tz_str = timezone.as_str();
                if let Some(tz) = tz_str {
                    assert!(Time::from_full_str(&format!("{hour:02}{minute:02}{tz}")).is_err());
                    let tz = tz.to_lowercase();
                    assert!(Time::from_full_str(&format!("{hour:02}{minute:02}{tz}")).is_err());
                }

                for second in (60..100).map(Some) {
                    if let (Some(tz), Some(second)) = (tz_str, second) {
                        assert!(Time::from_full_str(&format!("{hour:02}{minute:02}{second:02}{tz}")).is_err());
                        let tz = tz.to_lowercase();
                        assert!(Time::from_full_str(&format!("{hour:02}{minute:02}{second:02}{tz}")).is_err());
                    }
                }
            }
        }
    }
}