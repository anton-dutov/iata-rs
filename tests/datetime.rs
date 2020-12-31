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
    assert_eq!(DayOfYear::new(0),   Err(Error::InvalidDayOfYearRange(0)));
    assert_eq!(DayOfYear::new(367), Err(Error::InvalidDayOfYearRange(367)));
}

#[test]
fn test_day_of_year_to_naive_date() {

    use chrono::prelude::*;


    assert_eq!(
        DayOfYear::new(1).unwrap().to_naive_date(2020),
        Ok(NaiveDate::from_ymd(2020, 1, 1))
    );

    assert_eq!(
        DayOfYear::new(1).unwrap().to_naive_date(2021),
        Ok(NaiveDate::from_ymd(2021, 1, 1))
    );

    assert_eq!(
        DayOfYear::new(366).unwrap().to_naive_date(2020),
        Ok(NaiveDate::from_ymd(2020, 12, 31))
    );

    assert_eq!(
        DayOfYear::new(366).unwrap().to_naive_date(2021),
        Err(Error::OverflowNotLeapYear(366))
    );
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

    use Month::*;

    let month = May;
    let day   = 8;

    let short_date = ShortDate::new(month, day).unwrap();

    assert_eq!(short_date.day(), day);
    assert_eq!(short_date.month(), month);
    assert_eq!(short_date.to_string(), "08MAY");
}


#[test]
fn test_short_date_invalid() {

    use Month::*;

    assert_eq!(ShortDate::new(January  ,32), Err(Error::InvalidDayForMonth(January  ,32)));
    assert_eq!(ShortDate::new(February ,30), Err(Error::InvalidDayForMonth(February ,30)));
    assert_eq!(ShortDate::new(March    ,32), Err(Error::InvalidDayForMonth(March    ,32)));
    assert_eq!(ShortDate::new(April    ,31), Err(Error::InvalidDayForMonth(April    ,31)));
    assert_eq!(ShortDate::new(May      ,32), Err(Error::InvalidDayForMonth(May      ,32)));
    assert_eq!(ShortDate::new(June     ,31), Err(Error::InvalidDayForMonth(June     ,31)));
    assert_eq!(ShortDate::new(July     ,32), Err(Error::InvalidDayForMonth(July     ,32)));
    assert_eq!(ShortDate::new(August   ,32), Err(Error::InvalidDayForMonth(August   ,32)));
    assert_eq!(ShortDate::new(September,31), Err(Error::InvalidDayForMonth(September,31)));
    assert_eq!(ShortDate::new(October  ,32), Err(Error::InvalidDayForMonth(October  ,32)));
    assert_eq!(ShortDate::new(November ,31), Err(Error::InvalidDayForMonth(November ,31)));
    assert_eq!(ShortDate::new(December ,32), Err(Error::InvalidDayForMonth(December ,32)));
}
