use super::{format::Field, Error};

/// BCBP Item 15 — Passenger Description (1 byte).
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize))]
#[cfg_attr(feature = "with-serde", serde(rename = "PaxKind"))]
pub enum PassengerDescription {
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

impl From<char> for PassengerDescription {
    fn from(value: char) -> Self {
        use PassengerDescription::*;

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

impl TryFrom<PassengerDescription> for char {
    type Error = Error;

    /// Returns the character code, rejecting reserved values wider than one decimal digit.
    fn try_from(value: PassengerDescription) -> Result<Self, Self::Error> {
        use PassengerDescription::*;

        Ok(match value {
            None => ' ',
            Adult => '0',
            Male => '1',
            Female => '2',
            Child => '3',
            Infant => '4',
            CabinBaggage => '5',
            AdultWithInfant => '6',
            UnaccompaniedMinor => '7',
            Reserved(digit) => {
                char::from_digit(u32::from(digit), 10).ok_or(Error::InvalidFieldFormat {
                    field: Field::PassengerDescription,
                })?
            }
            Other(c) => c,
        })
    }
}
