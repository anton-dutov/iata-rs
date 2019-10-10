use super::field::Field;

#[derive(Debug, PartialEq)]
pub enum Error {
    MandatoryDataSize,
    InsufficientDataLength,
    InvalidFormatCode(char),
    InvalidPrefix(Field, char),
    InvalidLegsCount,
    InvalidFormat,
    CoditionalData,
    CoditionalDataSize,
    /// The end of the input was reached prematurely.
    UnexpectedEndOfInput(Field),
    /// The length of the subsection encoded exceeds the remaining length of the input.
    SubsectionTooLong,
    /// The contents of a field parsed as a numeric was not a numeric value.
    ExpectedInteger(Field),
    /// The BCBP string does not contain exclusively ASCII characters.
    InvalidCharacters,
    /// After parsing, additional characters remain.
    TrailingData,
}

#[derive(Debug, PartialEq)]
pub enum FixError {
    InsufficientDataLength,
}

pub type Result<T> = std::result::Result<T, Error>;