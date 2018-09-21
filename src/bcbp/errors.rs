#[derive(Debug, PartialEq)]
pub enum ParseError {
    DataLength,

    FormatCode,

    LegsCount,

    Format,

    Name,

    Date,

    CoditionalData,

    CoditionalDataSize,

    SecurityDataSize,

    SecurityData,
}

#[derive(Debug, PartialEq)]
pub enum FixError {
    InsufficientDataLength,
}