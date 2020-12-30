
use iata::datetime::*;

#[test]
fn test_month() {
    use Month::*;

    assert!(January  .as_str() == "JAN");
    assert!(February .as_str() == "FEB");
    assert!(March    .as_str() == "MAR");
    assert!(April    .as_str() == "APR");
    assert!(May      .as_str() == "MAY");
    assert!(June     .as_str() == "JUN");
    assert!(July     .as_str() == "JUL");
    assert!(August   .as_str() == "AUG");
    assert!(September.as_str() == "SEP");
    assert!(October  .as_str() == "OCT");
    assert!(November .as_str() == "NOV");
    assert!(December .as_str() == "DEC");
}

#[test]
fn test_short_date() {

    use Month::*;

    let month = May;
    let day   = 8;

    let short_date = ShortDate::new(month, day);

    assert!(short_date.day() == day);
    assert!(short_date.month() == month);
    assert!(short_date.to_string() == "08MAY");
}