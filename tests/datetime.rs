use iata::datetime::*;

#[test]
fn test_month() {
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
    assert!(DayOfYear::new(1).is_ok());
    assert!(DayOfYear::new(366).is_ok());
}

#[test]
fn test_day_of_year_invalid() {
    assert_eq!(DayOfYear::new(0),   Err(DateError::InvalidDayOfYearRange(0)));
    assert_eq!(DayOfYear::new(367), Err(DateError::InvalidDayOfYearRange(367)));
}

#[test]
fn test_day_of_year_to_naive_date() {

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
        Err(DateError::OverflowNotLeapYear(366))
    );
}

#[test]
fn test_short_date() {

    use Month::*;

    let month = May;
    let day   = 8;

    let short_date = ShortDate::new(month, day);

    assert_eq!(short_date.day(), day);
    assert_eq!(short_date.month(), month);
    assert_eq!(short_date.to_string(), "08MAY");
}
