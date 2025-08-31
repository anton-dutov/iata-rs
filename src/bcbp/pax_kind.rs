/// BCBP Item 15 — Passenger Description (1 byte).
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize))]
pub enum PaxKind {
    #[default]
    /// Space (no explicit description).
    None, // ' '
    Adult,  // '0'
    Male,   // '1'
    Female, // '2'
    Child,  // '3'
    Infant, // '4'
    /// No passenger (cabin baggage).
    CabinBaggage, // '5'
    /// Adult traveling with infant.
    AdultWithInfant, // '6'
    /// Unaccompanied minor.
    UnaccompaniedMinor, // '7'
    /// Reserved digits '8'..='9' (stores the digit value).
    Reserved(u8),
    /// Any other character (kept verbatim).
    Other(char),
}

impl From<char> for PaxKind {
    fn from(value: char) -> Self {
        use PaxKind::*;

        match value {
            ' ' => None,
            '0' => Adult,
            '1' => Male,
            '2' => Female,
            '3' => Child,
            '4' => Infant,
            '5' => CabinBaggage,
            '6' => AdultWithInfant,
            '7' => UnaccompaniedMinor,
            '8'..='9' => Reserved((value as u8) - b'0'),
            other => Other(other),
        }
    }
}

impl From<PaxKind> for char {
    fn from(value: PaxKind) -> Self {
        use PaxKind::*;

        match value {
            None => ' ',
            Adult => '0',
            Male => '1',
            Female => '2',
            Child => '3',
            Infant => '4',
            CabinBaggage => '5',
            AdultWithInfant => '6',
            UnaccompaniedMinor => '7',
            // Todo: handle invalid digit?
            Reserved(d) => (b'0' + d) as char,
            Other(c) => c,
        }
    }
}
