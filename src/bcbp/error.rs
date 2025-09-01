use super::format::Field;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    // --- базовый формат / размеры ---
    #[error("mandatory data size is missing")]
    MandatoryDataSize,

    #[error("insufficient data length")]
    InsufficientDataLength,

    #[error("invalid format code: {0}")]
    InvalidFormatCode(char),

    #[error("invalid prefix for {0}: got '{1}'")]
    InvalidPrefix(Field, char),

    #[error("invalid number of legs")]
    InvalidLegsCount,

    #[error("invalid format")]
    InvalidFormat,

    #[error("field size exceeded")]
    FieldSizeExceeded,

    #[error("field size exceeded: {0}, max {1}")]
    FieldSizeExceeded2(Field, usize),

    // --- условные данные ---
    #[error("conditional data is present but not allowed here")]
    ConditionalData,

    #[error("invalid conditional data size")]
    ConditionalDataSize,

    // --- версия / последовательности ---
    #[error("invalid version: {0}")]
    InvalidVersion(char),

    #[error("invalid check-in sequence")]
    InvalidCheckInSequence,

    // --- парсинг / ввод ---
    /// The end of the input was reached prematurely.
    #[error("unexpected end of input while parsing {0}")]
    UnexpectedEndOfInput(Field),

    /// The length of the subsection encoded exceeds the remaining length of the input.
    #[error("subsection too long")]
    SubsectionTooLong,

    /// The contents of a field parsed as a numeric was not a numeric value.
    #[error("expected integer in field {0}")]
    ExpectedInteger(Field),

    /// The BCBP string does not contain exclusively ASCII characters.
    #[error("input contains non-ASCII characters")]
    InvalidCharacters,

    /// After parsing, additional characters remain.
    #[error("trailing data after the expected end")]
    TrailingData,

    /// Returned when alphanumeric characters were expected
    #[error("alphanumeric characters expected")]
    AlphaNumExpected,

    /// Returned when alphabetic characters were expected
    #[error("alphabetic characters expected")]
    AlphaExpected,

    /// Returned when digit characters were expected
    #[error("digits expected")]
    DigitsExpected,

    #[error(transparent)]
    FmtError(#[from] std::fmt::Error),

    #[error("InvalidAirlineNumber")]
    InvalidAirlineNum,
}

#[derive(Debug, PartialEq)]
pub enum FixError {
    InsufficientDataLength,
}

pub type BcbpResult<T> = std::result::Result<T, Error>;
