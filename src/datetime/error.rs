use super::Month;

#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("INVALID_DAY_OF_YEAR_RANGE: {0:?}")]
    InvalidDayOfYearRange(u16),

    #[error("INVALID_ADAPT_RANGE: {0:?}")]
    InvalidAdaptRange(u16),

    #[error("OVERFLOW_NOT_LEAP_YEAR")]
    OverflowNotLeapYear,

    #[error("INVALID_DAY_FOR_MONTH: {:?} {0:?}")]
    InvalidDayForMonth(Month, u8),

    #[error("INVALID_FORMAT: {0:?}")]
    InvalidInput(String),

    #[error("INVALID_DAY_FORMAT: {0:?}")]
    InvalidDay(String),

    #[error("INVALID_MONTH_FORMAT: {0:?}")]
    InvalidMonth(String),

    #[error("INVALID_HOUR_FORMAT: {0:?}")]
    InvalidHour(String),

    #[error("INVALID_MINUTE_FORMAT: {0:?}")]
    InvalidMinute(String),

    #[error("INVALID_SECOND_FORMAT: {0:?}")]
    InvalidSecond(String),

    #[error("INVALID_TIMEZONE_TAG: {0:?}")]
    InvalidTimezoneTag(String),

    #[error("INVALID_HOUR_VALUE: {0}")]
    InvalidHourValue(u8),

    #[error("INVALID_MINUTE_VALUE: {0}")]
    InvalidMinuteValue(u8),

    #[error("INVALID_SECOND_VALUE: {0}")]
    InvalidSecondValue(u8),
}