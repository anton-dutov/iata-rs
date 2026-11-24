use std::str::FromStr;

use iata::datetime::*;
use rand::RngExt;

const DAYS_IN_YEAR: u16 = 365;
const MONTH_LENS: [(Month, u8); 12] = [
    (Month::January, 31),
    (Month::February, 29),
    (Month::March, 31),
    (Month::April, 30),
    (Month::May, 31),
    (Month::June, 30),
    (Month::July, 31),
    (Month::August, 31),
    (Month::September, 30),
    (Month::October, 31),
    (Month::November, 30),
    (Month::December, 31),
];
const TIME_ZONE_INDICATORS: [TimeZoneIndicator; 3] = [
    TimeZoneIndicator::Local,
    TimeZoneIndicator::None,
    TimeZoneIndicator::Utc,
];
const MIN_YEAR: i16 = 1997;
const MAX_YEAR: i16 = 3000;

#[test]
fn test_month_as_str() {
    use Month::*;

    assert_eq!(January.as_str(), "JAN");
    assert_eq!(February.as_str(), "FEB");
    assert_eq!(March.as_str(), "MAR");
    assert_eq!(April.as_str(), "APR");
    assert_eq!(May.as_str(), "MAY");
    assert_eq!(June.as_str(), "JUN");
    assert_eq!(July.as_str(), "JUL");
    assert_eq!(August.as_str(), "AUG");
    assert_eq!(September.as_str(), "SEP");
    assert_eq!(October.as_str(), "OCT");
    assert_eq!(November.as_str(), "NOV");
    assert_eq!(December.as_str(), "DEC");
}

#[test]
fn test_month_to_as_str() {
    use Month::*;

    assert_eq!(Month::from_str(January.as_str()).unwrap(), January);
    assert_eq!(Month::from_str(February.as_str()).unwrap(), February);
    assert_eq!(Month::from_str(March.as_str()).unwrap(), March);
    assert_eq!(Month::from_str(April.as_str()).unwrap(), April);
    assert_eq!(Month::from_str(May.as_str()).unwrap(), May);
    assert_eq!(Month::from_str(June.as_str()).unwrap(), June);
    assert_eq!(Month::from_str(July.as_str()).unwrap(), July);
    assert_eq!(Month::from_str(August.as_str()).unwrap(), August);
    assert_eq!(Month::from_str(September.as_str()).unwrap(), September);
    assert_eq!(Month::from_str(October.as_str()).unwrap(), October);
    assert_eq!(Month::from_str(November.as_str()).unwrap(), November);
    assert_eq!(Month::from_str(December.as_str()).unwrap(), December);
}

#[test]
fn test_day_of_year_valid() {
    for day in 1..=(DAYS_IN_YEAR + 1) {
        assert!(
            DayOfYear::new(day).is_ok(),
            "Day {day} must be a valid day of year"
        );
    }
}

#[test]
fn test_day_of_year_invalid() {
    for day in std::iter::once(0).chain((DAYS_IN_YEAR + 2)..=u16::MAX) {
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
        let january_first = jiff::civil::date(year, 1, 1);
        assert_eq!(
            january_first.in_leap_year(),
            DayOfYear::new(366).unwrap().to_date(year).is_ok()
        );
        if !january_first.in_leap_year() {
            assert_eq!(
                DayOfYear::new(366).unwrap().to_date(year),
                Err(Error::OverflowNotLeapYear)
            );
        }
    }
}

#[test]
fn test_day_of_year_to_date() {
    use jiff::{civil::Date, ToSpan};

    for year in MIN_YEAR..MAX_YEAR {
        let jan1 = Date::new(year, 1, 1).unwrap();
        let it = (1..=DAYS_IN_YEAR).filter_map(|day| {
            jan1.checked_add(((day as i64) - 1).days())
                .map(|x| (day, x))
                .ok()
        });

        for (day, date) in it {
            assert_eq!(DayOfYear::new(day).unwrap().to_date(year), Ok(date));
        }
    }
}

#[test]
fn test_day_of_year_to_date_adapt() {
    use jiff::{civil::Date, ToSpan};

    const WINDOW_SIZE_MAX: u16 = 31;

    let ordinal = |year: i16, day: u16| {
        Date::new(year, 1, 1)
            .unwrap()
            .checked_add(((day as i64) - 1).days())
            .unwrap()
    };

    // When ticket is for the next year
    let next_year_test = |window, year: i16| {
        for offset in 0..window {
            for day in 1..window {
                assert_eq!(
                    DayOfYear::new(day).unwrap().to_date_adapt(
                        Date::new(year, 12, 31 - offset as i8).unwrap(),
                        window as u8
                    ),
                    Ok(Date::new(year + 1, 1, day as i8).unwrap())
                );
            }
        }
    };

    // When ticket is for the previous year
    let prev_year_test = |window, year: i16| {
        for offset in 1..window {
            let edge_date = DayOfYear::new(DAYS_IN_YEAR + 1)
                .unwrap()
                .to_date_adapt(Date::new(year, 1, offset as i8).unwrap(), window as u8);

            if ordinal(year - 1, 1).in_leap_year() {
                assert_eq!(edge_date, Ok(ordinal(year - 1, DAYS_IN_YEAR + 1)));
            } else {
                assert_eq!(edge_date, Err(Error::OverflowNotLeapYear));
            }

            for day in (1..window).map(|x| DAYS_IN_YEAR - x + 1) {
                assert_eq!(
                    DayOfYear::new(day)
                        .unwrap()
                        .to_date_adapt(Date::new(year, 1, offset as i8).unwrap(), window as u8),
                    Ok(ordinal(year - 1, day))
                );
            }
        }
    };

    for window in 1..WINDOW_SIZE_MAX {
        (MIN_YEAR..(MAX_YEAR - 1)).for_each(|year| next_year_test(window, year));
        ((MIN_YEAR + 1)..MAX_YEAR).for_each(|year| prev_year_test(window, year));
    }
}

#[test]
fn ordinal_and_calendar_dates_share_year_boundaries() {
    use jiff::civil::date;

    for (reference, ordinal, calendar, expected) in [
        (date(2024, 12, 24), 1, "01JAN", date(2024, 1, 1)),
        (date(2024, 12, 25), 1, "01JAN", date(2025, 1, 1)),
        (date(2024, 12, 31), 6, "06JAN", date(2025, 1, 6)),
        (date(2024, 12, 31), 7, "07JAN", date(2024, 1, 7)),
        (date(2025, 1, 6), 360, "25DEC", date(2024, 12, 25)),
        (date(2025, 1, 7), 365, "31DEC", date(2025, 12, 31)),
        (date(2025, 1, 6), 366, "31DEC", date(2024, 12, 31)),
    ] {
        assert_eq!(
            DayOfYear::new(ordinal).unwrap().to_date_adapt(reference, 7),
            Ok(expected)
        );
        assert_eq!(
            calendar
                .parse::<MonthDay>()
                .unwrap()
                .to_date_adapt(reference, 7),
            Ok(expected)
        );
    }
    // Day 359 is Dec 24 in the candidate leap year, outside the seven-day window.
    assert_eq!(
        DayOfYear::new(359)
            .unwrap()
            .to_date_adapt(date(2025, 1, 6), 7),
        Ok(date(2025, 12, 25))
    );
}

#[test]
fn dates_reject_years_outside_jiff_range() {
    let january = MonthDay::new(Month::January, 1).unwrap();
    let first = DayOfYear::new(1).unwrap();
    let midnight = MonthDayTime::new(january.clone(), Time::default());
    for year in [i16::MIN, -10000, 10000, i16::MAX] {
        assert!(january.to_date(year).is_err(), "year {year}");
        assert!(first.to_date(year).is_err(), "year {year}");
        assert!(midnight.to_datetime(year).is_err(), "year {year}");
    }
}

#[test]
fn dates_resolve_at_jiff_limits() {
    use jiff::civil::Date;

    for (ordinal, calendar, reference) in [(1, "01JAN", Date::MIN), (365, "31DEC", Date::MAX)] {
        assert_eq!(
            DayOfYear::new(ordinal).unwrap().to_date_adapt(reference, 7),
            Ok(reference)
        );
        assert_eq!(
            calendar
                .parse::<MonthDay>()
                .unwrap()
                .to_date_adapt(reference, 7),
            Ok(reference)
        );
    }
    for (ordinal, calendar, reference) in [(360, "25DEC", Date::MIN), (1, "01JAN", Date::MAX)] {
        assert!(DayOfYear::new(ordinal)
            .unwrap()
            .to_date_adapt(reference, 7)
            .is_err());
        assert!(calendar
            .parse::<MonthDay>()
            .unwrap()
            .to_date_adapt(reference, 7)
            .is_err());
    }
}

#[test]
fn century_leap_dates() {
    use jiff::civil::date;

    let leap_day = "29FEB".parse::<MonthDay>().unwrap();
    let last_day = DayOfYear::new(366).unwrap();
    assert_eq!(leap_day.to_string(), "29FEB");
    assert!("30FEB".parse::<MonthDay>().is_err());
    assert_eq!(leap_day.to_date(1900), Err(Error::OverflowNotLeapYear));
    assert_eq!(last_day.to_date(1900), Err(Error::OverflowNotLeapYear));
    assert_eq!(leap_day.to_date(2000), Ok(date(2000, 2, 29)));
    assert_eq!(last_day.to_date(2000), Ok(date(2000, 12, 31)));
}

#[test]
fn test_time_zone_indicator_as_str() {
    assert_eq!(TimeZoneIndicator::Local.as_str().as_deref(), Some("L"));
    assert_eq!(TimeZoneIndicator::Utc.as_str().as_deref(), Some("Z"));
    assert_eq!(TimeZoneIndicator::None.as_str(), None);
}

#[test]
fn test_month_day() {
    for (month, len) in MONTH_LENS {
        for day in 1..=len {
            let month_day = MonthDay::new(month, day).expect(&format!(
                "{day} must be a valid of day of {}",
                month.as_str()
            ));

            assert_eq!(month_day.day(), day);
            assert_eq!(month_day.month(), month);
            assert_eq!(month_day.to_string(), format!("{day:02}{}", month.as_str()));
        }
    }
}

#[test]
fn test_month_day_invalid() {
    for (month, len) in MONTH_LENS {
        let it = std::iter::once(0).chain((len + 1)..u8::MAX);

        for day in it {
            assert_eq!(
                MonthDay::new(month, day),
                Err(Error::InvalidDayForMonth(month, day))
            );
        }
    }
}

#[test]
fn test_month_day_leap_year() {
    for year in MIN_YEAR..MAX_YEAR {
        let january_first = jiff::civil::date(year, 1, 1);
        assert_eq!(
            january_first.in_leap_year(),
            MonthDay::new(Month::February, 29)
                .unwrap()
                .to_date(year)
                .is_ok()
        );
        if !january_first.in_leap_year() {
            assert_eq!(
                MonthDay::new(Month::February, 29).unwrap().to_date(year),
                Err(Error::OverflowNotLeapYear)
            );
        }
    }
}

#[test]
fn test_month_day_to_date_adapt() {
    use jiff::civil::Date;

    const WINDOW_SIZE_MAX: u16 = 31;

    // When ticket is for the next year
    let next_year_test = |window, year: i16| {
        for offset in 0..window {
            for day in 1..window {
                assert_eq!(
                    MonthDay::new(Month::January, day as u8)
                        .unwrap()
                        .to_date_adapt(
                            Date::new(year, 12, 31 - offset as i8).unwrap(),
                            window as u8
                        ),
                    Ok(Date::new(year + 1, 1, day as i8).unwrap())
                );
            }
        }
    };

    // When ticket is for the previous year
    let prev_year_test = |window, year: i16| {
        for offset in 1..window {
            for day in (1..window).map(|x| 31 - x + 1) {
                assert_eq!(
                    MonthDay::new(Month::December, day as u8)
                        .unwrap()
                        .to_date_adapt(Date::new(year, 1, offset as i8).unwrap(), window as u8),
                    Ok(Date::new(year - 1, 12, day as i8).unwrap())
                );
            }
        }
    };

    for window in 1..WINDOW_SIZE_MAX {
        (MIN_YEAR..(MAX_YEAR - 1)).for_each(|year| next_year_test(window, year));
        ((MIN_YEAR + 1)..MAX_YEAR).for_each(|year| prev_year_test(window, year));
    }
}

#[test]
fn test_month_day_to_string_from_string() {
    for (month, len) in MONTH_LENS {
        for day in 1..=len {
            assert_eq!(
                MonthDay::from_str(&MonthDay::new(month, day).unwrap().to_string()),
                MonthDay::new(month, day),
            );
        }
    }
}

#[test]
fn test_month_day_bad_strs() {
    const MAX_SAMPLE_LEN: usize = 1_000_00;
    use rand::distr::{Alphanumeric, SampleString};
    let mut rng = rand::rng();

    for _ in 0..1000 {
        let len = rng.random_range(0..MAX_SAMPLE_LEN);

        let s = Alphanumeric.sample_string(&mut rng, len);

        if len != 5 {
            assert!(MonthDay::from_str(&s).is_err());
        }
    }
}

#[test]
fn month_day_and_time_reject_utf8_across_field_boundaries() {
    let date = "0éAN";
    assert_eq!(
        date.parse::<MonthDay>(),
        Err(Error::InvalidInput(date.into()))
    );

    let short = "0é0";
    assert_eq!(
        Time::from_short_str(short),
        Err(Error::InvalidInput(short.into()))
    );

    for full in ["0é0Z", "123é", "12345é"] {
        assert_eq!(
            Time::from_full_str(full),
            Err(Error::InvalidInput(full.into()))
        );
    }
}

#[test]
fn utf8_within_a_field_reports_the_field_error() {
    assert_eq!(
        "12MÄ".parse::<MonthDay>(),
        Err(Error::InvalidMonth("MÄ".into()))
    );
    assert_eq!(
        Time::from_short_str("é12"),
        Err(Error::InvalidHour("é".into()))
    );
    assert_eq!(
        Time::from_full_str("1234éZ"),
        Err(Error::InvalidSecond("é".into()))
    );
}

#[test]
fn test_time_bad_strs() {
    const MAX_SAMPLE_LEN: usize = 1_000_00;
    use rand::distr::{Alphanumeric, SampleString};
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

            for timezone in TIME_ZONE_INDICATORS {
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
                        jiff::civil::Time::new(
                            hour as i8,
                            minute as i8,
                            second.unwrap_or_default() as i8,
                            0,
                        )
                        .unwrap()
                    );

                    if let (Some(tz), Some(second)) = (tz_str, second) {
                        assert!(Time::from_full_str(&format!(
                            "{hour:02}{minute:02}{second:02}{tz}"
                        ))
                        .is_ok());
                        let tz = tz.to_lowercase();
                        assert!(Time::from_full_str(&format!(
                            "{hour:02}{minute:02}{second:02}{tz}"
                        ))
                        .is_ok());
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
            for timezone in TIME_ZONE_INDICATORS {
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

            for timezone in TIME_ZONE_INDICATORS {
                let tz_str = timezone.as_str();
                if let Some(tz) = tz_str {
                    assert!(Time::from_full_str(&format!("{hour:02}{minute:02}{tz}")).is_err());
                    let tz = tz.to_lowercase();
                    assert!(Time::from_full_str(&format!("{hour:02}{minute:02}{tz}")).is_err());
                }

                for second in (60..100).map(Some) {
                    if let (Some(tz), Some(second)) = (tz_str, second) {
                        assert!(Time::from_full_str(&format!(
                            "{hour:02}{minute:02}{second:02}{tz}"
                        ))
                        .is_err());
                        let tz = tz.to_lowercase();
                        assert!(Time::from_full_str(&format!(
                            "{hour:02}{minute:02}{second:02}{tz}"
                        ))
                        .is_err());
                    }
                }
            }
        }
    }
}
