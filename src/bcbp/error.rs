use super::format::Field;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum Error {
    #[error("BCBP is shorter than minimum length")]
    BcbpTooShort,

    #[error("invalid format code: {0}")]
    InvalidFormatCode(char),

    #[error("insufficient data length")]
    InsufficientDataLength,

    #[error("invalid prefix for {0}: got '{1}'")]
    InvalidPrefix(Field, char),

    #[error("invalid number of legs")]
    InvalidNumberOfLegs,

    #[error("invalid format")]
    InvalidFormat,

    #[error("invalid format for field {field}")]
    InvalidFieldFormat { field: Field },

    #[error("field size exceeded")]
    FieldTooLong,

    #[error("field size exceeded: {field}, max {max}")]
    FieldLengthExceeded { field: Field, max: usize },

    #[error("conditional data is present but not allowed here")]
    ConditionalData,

    #[error("not enough data for declared conditional data length")]
    ConditionalDataLengthMismatch,

    #[error("invalid version: {0}")]
    InvalidVersion(char),

    #[error("invalid check-in sequence")]
    InvalidCheckInSequence,

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
    AlphanumericExpected,

    /// Returned when alphabetic characters were expected
    #[error("alphabetic characters expected")]
    AlphabeticExpected,

    /// Returned when digit characters were expected
    #[error("digits expected")]
    DigitsExpected,

    #[error(transparent)]
    FmtError(#[from] std::fmt::Error),

    #[error(transparent)]
    DateTimeErro(#[from] crate::datetime::Error),
}

#[derive(Debug, PartialEq)]
pub enum FixError {
    InsufficientDataLength,
}

pub type BcbpResult<T> = std::result::Result<T, Error>;
