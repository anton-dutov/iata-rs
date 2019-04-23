#[derive(Debug, PartialEq)]
pub enum ParseError {
    MandatoryDataSize,
    InsufficientDataLength,
    InvalidFormatCode(char),
    InvalidVersionBegin(char),
    InvalidLegsCount,

    InvalidFormat,

    Name,

    Date,

    CoditionalData,

    CoditionalDataSize,
    CoditionalUniqueSize,

    SecurityDataSize,

    SecurityData,
}

#[derive(Debug, PartialEq)]
pub enum FixError {
    InsufficientDataLength,
}