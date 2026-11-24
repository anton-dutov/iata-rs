# IATA

[![Build status](https://github.com/anton-dutov/iata-rs/actions/workflows/ci.yml/badge.svg?branch=master&event=push)](https://github.com/anton-dutov/iata-rs/actions/workflows/ci.yml)

`iata::datetime` is available in every build. The default Cargo feature
`with-bcbp` enables `iata::bcbp`. Set `default-features = false` to use
datetime alone. Enable `with-serde` for Serde serialization of the BCBP model
and `DayOfYear`.

## Module: BCBP

The parser and encoder implement the Bar Coded Boarding Pass (BCBP) format
described in IATA Resolution 792, version 6.

Use `BcbpView::parse` to read fixed-width fields with their original padding.
Convert the view into an editable `Bcbp` to normalize strings and validate
typed fields such as the flight day.

```rust
use iata::bcbp::{view::BcbpView, Bcbp, BcbpResult};

fn edit_pass(raw: &str) -> BcbpResult<String> {
    let view = BcbpView::parse(raw)?;
    let mut pass = Bcbp::try_from(view)?;
    pass.set_passenger_name("IVANOV/IVAN")?;
    pass.encode()
}
```

To read directly into the owned model, use `raw.parse::<Bcbp>()?`.
The existing `decode_bcbp` and `encode_bcbp` methods remain available.
The decoder checks the wire layout before converting the fields. If several
fields are malformed, it can report a structural error before an invalid value.
Encoding writes the normalized model, including both nonconsecutive baggage
tags when present. A section that exceeds its two-digit hexadecimal length
returns `Error::FieldLengthExceeded`.

When writing the version field, `Bcbp::encode()` defaults to `Version::LATEST`
(currently 6). The encoder supports versions 2 through 6. Use
`set_version(Some(Version::Value(4)))` to target a particular version or
`set_version(None)` to restore the default. The encoder checks the
completed pass, including every leg. Security data requires version 3, the two
nonconsecutive baggage tag fields require version 4, and Fast Track requires
version 5. A minimal single-leg pass omits the version section and stays at
60 bytes. `Error::FieldRequiresVersion` identifies the field, selected version,
and minimum version. Parsing accepts these fields even on older passes.

## Migrating the BCBP API

| Previous name in `iata::bcbp` | New name |
| --- | --- |
| `PaxKind` | `PassengerDescription` |
| `PaxStatus` | `PassengerStatus` |
| `fix_length` | `try_extract_minimal_pass` |

Use `passenger_description()` for both `Bcbp::pax_kind()` and
`BcbpView::pax_description()`. On a flight leg, use `passenger_status()` for
`pax_status()` in both models. The corresponding setters are
`Bcbp::set_passenger_description()` and `Leg::set_passenger_status()`.
With `with-serde`, the serialized field names are `pax_kind` and `pax_status`.

To convert a `Version` or `PassengerDescription` to a character, replace
`char::from(value)` with `char::try_from(value)?`. Numeric values above 9 in
`Version::Value` or `PassengerDescription::Reserved` return
`Error::InvalidFieldFormat` for the corresponding field.

## Migrating the datetime API

Update imports and type references in `iata::datetime`.

Public date and time conversions now use Jiff 1.

| Previous type | New type |
| --- | --- |
| `time::Date` (including the argument to `Leg::set_flight_date`) | `jiff::civil::Date` |
| `time::Time` | `jiff::civil::Time` |
| `time::PrimitiveDateTime` | `jiff::civil::DateTime` |
| `time::UtcOffset` | `jiff::tz::Offset` |

Use `i8::from(month)` to obtain the month number (1–12).

| Previous name | New name |
| --- | --- |
| `TzTag` | `TimeZoneIndicator` |
| `ShortDate` | `MonthDay` |
| `ShortDateTime` | `MonthDayTime` |

`MonthDay::new` still takes the month followed by the day, and the string
format is `DDMMM`, for example `"29FEB"`.

`DayOfYear::to_date`, `MonthDay::to_date`, and `MonthDayTime::to_datetime`
now take an `i16` year, the type returned by `jiff::civil::Date::year()`.
If you have an `i32` year, convert it with `i16::try_from(year)?`.

The standalone `iata::datetime::is_leap_year` function was removed. Use
`date.in_leap_year()` on a `jiff::civil::Date`. For a year alone, construct
January 1 with `jiff::civil::Date::new(year, 1, 1)?` first. Jiff accepts years
from −9999 through 9999.

## License

The `iata` crate is distributed under the [MIT License](LICENSE).
IATA reference materials are subject to their own terms.
