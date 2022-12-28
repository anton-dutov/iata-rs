use iata::datetime::*;

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
fn test_day_of_year_to_naive_date() {

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
fn test_day_of_year_to_naive_date_adapt() {
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
                        window
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
                window
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
                        window
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
