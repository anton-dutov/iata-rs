# Changelog

This project follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## [Unreleased]

## [0.11.0-rc.1] - 2026-11-24

### Added

- Added `BcbpView::parse`, `TryFrom<BcbpView>` and `FromStr` for `Bcbp`, and `Bcbp::encode`.
- Added `BcbpView::version` to read the version character and `Field::is_empty` to identify fields with a variable length.
- Added the default `with-bcbp` feature. Disabling default features builds the datetime module alone.

### Changed

- Replaced `From` with `TryFrom` when converting `Version` and `PassengerDescription` to `char`. Numeric values above 9 now return `Error::InvalidFieldFormat`.
- Renamed `bcbp::fix_length` to `bcbp::try_extract_minimal_pass`.
- Switched the public date, time, and UTC offset types from `time` to Jiff 1, including the date accepted by `Leg::set_flight_date`.
- BCBP encoding defaults the version field to version 6 and reports fields incompatible with the selected version.
- Renamed the BCBP passenger description and status types and their accessors to match IATA terminology. See the [BCBP migration guide](README.md#migrating-the-bcbp-api).
- Renamed three public types in `iata::datetime`. See the [datetime migration guide](README.md#migrating-the-datetime-api).
- Changed year parameters from `i32` to `i16` in `DayOfYear::to_date`, `MonthDay::to_date`, and `MonthDayTime::to_datetime` to match Jiff.

### Removed

- Removed `iata::datetime::is_leap_year`. Use `jiff::civil::Date::in_leap_year`.

### Fixed

- Encoding now returns `Error::InvalidCheckInSequence` for values above 9999. A zero check-in sequence round-trips as `Some(0)`, and a blank field as `None`.
- BCBP setters reject issue dates above 9999, passenger description codes wider than one digit, and non-ASCII field contents before updating the pass.
- Blank boarding pass issue dates and airline numeric codes now round-trip as spaces. Explicit zero values retain their zero padding.
- `MonthDay::from_str`, `Time::from_short_str`, and `Time::from_full_str` now return a parse error when a UTF-8 character crosses a field boundary.
- `bcbp::try_extract_minimal_pass` now sets the leg count to one when extracting the first leg of a multi-leg boarding pass.
- Boarding passes with empty legs now round-trip correctly, and encoding preserves both nonconsecutive baggage tags.
- BCBP encoding now returns `Error::FieldLengthExceeded` when a variable section exceeds 255 bytes.
- `DayOfYear` and `MonthDay` now use the same rule to resolve dates near year boundaries, including leap years.

[Unreleased]: https://github.com/anton-dutov/iata-rs/compare/v0.11.0-rc.1...HEAD
[0.11.0-rc.1]: https://github.com/anton-dutov/iata-rs/compare/v0.9.0-alpha.2...v0.11.0-rc.1
