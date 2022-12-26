use iata::datetime::*;

const DAYS_IN_YEAR: u32 = 365;
const MONTH_LENS: [(Month, u32); 12] = [
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
    for day in std::iter::once(0).chain((DAYS_IN_YEAR + 2)..=std::u32::MAX) {
        assert_eq!(
            DayOfYear::new(day),
            Err(Error::InvalidDayOfYearRange(day)),
            "Day {day} must be an invalid day of year"
        );
    }
}

#[test]
fn test_day_of_year_to_naive_date() {

    use chrono::prelude::*;

    for year in 1997..3000 {
        let it =
            (1..=DAYS_IN_YEAR)
                .filter_map(|day|
                    NaiveDate::from_yo_opt(year, day)
                    .map(|x| (day, x))
                );

        for (day, date) in it {
            assert_eq!(DayOfYear::new(day).unwrap().to_naive_date(year), Ok(date));
        }
    }
}

#[test]
fn test_day_of_year_to_naive_date_adapt() {

    use chrono::prelude::*;

    assert_eq!(
        DayOfYear::new(1).unwrap().to_naive_date_adapt(&Utc.ymd(2020, 12, 30), 14),
        Ok(NaiveDate::from_ymd(2021, 1, 1))
    );

    assert_eq!(
        DayOfYear::new(365).unwrap().to_naive_date_adapt(&Utc.ymd(2021, 1, 1), 14),
        Ok(NaiveDate::from_ymd(2020, 12, 30))
    );

    assert_eq!(
        DayOfYear::new(366).unwrap().to_naive_date_adapt(&Utc.ymd(2021, 1, 1), 14),
        Ok(NaiveDate::from_ymd(2020, 12, 31))
    );

    assert_eq!(
        DayOfYear::new(365).unwrap().to_naive_date_adapt(&Utc.ymd(2022, 1, 1), 14),
        Ok(NaiveDate::from_ymd(2021, 12, 31))
    );

    assert_eq!(
        DayOfYear::new(366).unwrap().to_naive_date_adapt(&Utc.ymd(2022, 1, 1), 14),
        Err(Error::OverflowNotLeapYear(366))
    );
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
            .chain((len+1)..u32::MAX);

        for day in it {
            assert_eq!(ShortDate::new(month, day), Err(Error::InvalidDayForMonth(month, day)));
        }
    }
}
