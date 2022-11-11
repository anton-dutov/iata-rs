// Copyright (C) 2018 Martin Mroz
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.

use std::fmt;

#[derive(Copy,Clone,Eq,PartialEq,Ord,PartialOrd,Debug,Hash)]
pub enum Field {
    /// Item 1: Format Code. 1 byte. Data Type 'f'.
    FormatCode,
    /// Item 4: Airline Individual Use. n bytes. Data Type unspecified.
    AirlineIndividualUse,
    /// Item 5: Number of Legs Encoded. 1 byte. Data Type 'N'.
    LegsCount,
    /// Item 6: Field Size of Variable Size Field. 2 byte. Data Type 'f'. Hexadecimal.
    VariableBlockSize,
    /// Item 7: Operating Carrier PNR Code. 7 bytes. Data Type 'f'.
    OperatingAirlinePnr,
    /// Item 8: Beginning of Version Number. 1 byte. Data Type 'f'.
    VersionBegin,
    /// Item 9: Version Number. 1 byte. Data Type 'f'.
    Version,
    /// Item 10: Field Size of Structured Message. 2 byte. Data Type 'f'. Hexadecimal.
    UniqueBlockSize,
    /// Item 11: Passenger Name. 20 bytes. Data Type 'f'.
    PaxName,
    /// Item 12: Source of Check-In. 1 byte. Data Type 'f'.
    CheckInSrc,
    /// Item 14: Source of Boarding Pass Issuance. 1 byte. Data Type 'f'.
    BoardingPassIssueSrc,
    /// Item 15: Passenger Description. 1 byte. Data Type 'f'.
    PaxDescription,
    /// Item 16: Document Type. 1 byte. Data Type 'f'.
    DocumentType,
    /// Item 17: Field Size of Structured Message. 2 byte. Data Type 'f'. Hexadecimal.
    RepeatedBlockSize,
    /// Item 18: Selectee Indicator. 1 byte. Data Type 'f'.
    SelecteeIndicator,
    /// Item 19: Marketing Carrier Designator. 3 bytes. Data Type 'f'.
    MarketingAirline,
    /// Item 20: Frequent Flyer Airline Designator. 3 bytes. Data Type 'f'.
    FrequentFlyerAirline,
    /// Item 21: Airline Designator of Boarding Pass Issuer. 3 bytes. Data Type 'f'.
    BoardingPassIssueAirline,
    /// Item 22: Date of Issue of Boarding Pass. 4 bytes. Data Type 'N'.
    BoardingPassIssueDate,
    /// Item 23: Baggage Tag License Plate Number(s). 13 bytes. Data Type 'f'.
    BagTags,
    /// Item 25: Beginning of Security Data. 1 byte. Data Type 'f'.
    SecurityDataBegin,
    /// Item 26: From City Airport Code. 3 bytes. Data Type 'a'.
    FromCityAirportCode,
    /// Item 28: Type of Security Data. 1 byte. Data Type 'f'.
    SecurityDataKind,
    /// Item 29: Length of Security Data. 2 bytes. Data Type 'f'. Hexadecimal.
    SecurityDataLen,
    /// Item 30: Security Data. n bytes. Data Type 'f'.
    SecurityData,
    /// Item 31: First Non-Consecutive Baggage Tag License Plate Number. 13 bytes. Data Type 'f'.
    BagTagsNc1,
    /// Item 32: Second Non-Consecutive Baggage Tag License Plate Number. 13 bytes. Data Type 'f'.
    BagTagsNc2,
    /// Item 38: To City Airport Code. 3 bytes. Data Type 'a'.
    ToCityAirportCode,
    /// Item 42: Operating Carrier Designator. 3 bytes. Data Type 'f'.
    OperatingAirline,
    /// Item 43: Flight Number. 5 bytes. Data Type 'NNNN\[a\]'.
    FlightNumber,
    /// Item 46: Date of Flight. 3 bytes. Data Type 'N'.
    DateOfFlight,
    /// Item 71: Compartment Code. 1 byte. Data Type 'a'.
    CompartmentCode,
    /// Item 89: Electronic Ticket Indicator. 1 byte. Data Type 'f'.
    IdAdIndicator,
    /// Item 104: Seat Number. 4 bytes. Data Type is usually 'NNNa', but can be 'INF ' or similar.
    SeatNumber,
    /// Item 107: Check-In Sequence Number. 5 bytes. Data Type is usually 'NNNN\[f\]', but can be 'f'.
    CheckInSequence,
    /// Item 108: International Document Verification. 1 byte. Data Type 'f'.
    InternationalDocumentVerification,
    /// Item 117: Passenger Status. 1 byte. Data Type 'f'.
    PaxStatus,
    /// Item 118: Free Baggage Allowance. 3 bytes. Data Type 'f'.
    FreeBaggageAllowance,
    /// Item 142: Airline Numeric Code. 3 bytes. Data Type 'N'.
    AirlineNumericCode,
    /// Item 143: Document Form / Serial Number. 10 bytes. Data Type 'f'.
    DocumentFormSerialNumber,
    /// Item 236: Frequent Flyer Number. 16 bytes. Data Type 'f'.
    FrequentFlyerNumber,
    /// Item 253: Electronic Ticket Indicator. 1 byte. Data Type 'f'.
    ETicketIndicator,
    /// Item 254: Fast Track. 1 byte. Data Type 'f'.
    FastTrack,
}

impl Field {

    /// The required length of the field. If zero, the field may be arbitrarily long.
    pub fn len(self) -> usize {
        match self {
            Field::FormatCode => 1,
            Field::AirlineIndividualUse => 0,
            Field::LegsCount => 1,
            Field::VariableBlockSize => 2,
            Field::OperatingAirlinePnr => 7,
            Field::VersionBegin => 1,
            Field::Version => 1,
            Field::UniqueBlockSize => 2,
            Field::PaxName => 20,
            Field::CheckInSrc => 1,
            Field::BoardingPassIssueSrc => 1,
            Field::PaxDescription => 1,
            Field::DocumentType => 1,
            Field::RepeatedBlockSize => 2,
            Field::SelecteeIndicator => 1,
            Field::MarketingAirline => 3,
            Field::FrequentFlyerAirline => 3,
            Field::BoardingPassIssueAirline => 3,
            Field::BoardingPassIssueDate => 4,
            Field::BagTags => 13,
            Field::SecurityDataBegin => 1,
            Field::FromCityAirportCode => 3,
            Field::SecurityDataKind => 1,
            Field::SecurityDataLen => 2,
            Field::SecurityData => 0,
            Field::BagTagsNc1 => 13,
            Field::BagTagsNc2 => 13,
            Field::ToCityAirportCode => 3,
            Field::OperatingAirline => 3,
            Field::FlightNumber => 5,
            Field::DateOfFlight => 3,
            Field::CompartmentCode => 1,
            Field::IdAdIndicator => 1,
            Field::SeatNumber => 4,
            Field::CheckInSequence => 5,
            Field::InternationalDocumentVerification => 1,
            Field::PaxStatus => 1,
            Field::FreeBaggageAllowance => 3,
            Field::AirlineNumericCode => 3,
            Field::DocumentFormSerialNumber => 10,
            Field::FrequentFlyerNumber => 16,
            Field::ETicketIndicator => 1,
            Field::FastTrack => 1,
        }
    }

    /// Name of the field as defined in the Implementation Guide.
    pub fn name(self) -> &'static str {
        match self {
            Self::FormatCode =>
                "Format Code",
            Self::AirlineIndividualUse =>
                "Airline Individual Use",
            Self::LegsCount =>
                "Number of Legs Encoded",
            Self::VariableBlockSize =>
                "Field Size of Variable Size Field",
            Self::OperatingAirlinePnr =>
                "Operating Carrier PNR Code",
            Self::VersionBegin =>
                "Beginning of Version Number",
            Self::Version =>
                "Version Number",
            Self::UniqueBlockSize =>
                "Field Size of Strutured Message (Unique)",
            Self::PaxName =>
                "Passenger Name",
            Self::CheckInSrc =>
                "Source of Check-In",
            Self::BoardingPassIssueSrc =>
                "Source of Boarding Pass Issuance",
            Self::PaxDescription =>
                "Passenger Description",
            Self::DocumentType =>
                "Document Type",
            Self::RepeatedBlockSize =>
                "Field Size of Strutured Message (Repeated)",
            Self::SelecteeIndicator =>
                "Selectee Indicator",
            Self::MarketingAirline =>
                "Marketing Carrier Designator",
            Self::FrequentFlyerAirline =>
                "Frequent Flyer Airline Designator",
            Self::BoardingPassIssueAirline =>
                "Airline Designator of Boarding Pass Issuer",
            Self::BoardingPassIssueDate =>
                "Date of Issue of Boarding Pass",
            Self::BagTags =>
                "Baggage Tag License Plate Number(s)",
            Self::SecurityDataBegin =>
                "Beginning of Security Data",
            Self::FromCityAirportCode =>
                "From City Airport Code",
            Self::SecurityDataKind =>
                "Type of Security Data",
            Self::SecurityDataLen =>
                "Length of Security Data",
            Self::SecurityData =>
                "Security Data",
            Self::BagTagsNc1 =>
                "First Non-Consecutive Baggage Tag License Plate Number",
            Self::BagTagsNc2 =>
                "Second Non-Consecutive Baggage Tag License Plate Number",
            Self::ToCityAirportCode =>
                "To City Airport Code",
            Self::OperatingAirline =>
                "Operating Carrier Designator",
            Self::FlightNumber =>
                "Flight Number",
            Self::DateOfFlight =>
                "Date of Flight",
            Self::CompartmentCode =>
                "Compartment Code",
            Self::IdAdIndicator =>
                "ID/AD Indicator",
            Self::SeatNumber =>
                "Seat Number",
            Self::CheckInSequence =>
                "Check-In Sequence Number",
            Self::InternationalDocumentVerification =>
                "International Document Verification",
            Self::PaxStatus =>
                "Passenger Status",
            Self::FreeBaggageAllowance =>
                "Free Baggage Allowance",
            Self::AirlineNumericCode =>
                "Airline Numeric Code",
            Self::DocumentFormSerialNumber =>
                "Document Form / Serial Number",
            Self::FrequentFlyerNumber =>
                "Frequent Flyer Number",
            Self::ETicketIndicator =>
                "Electronic Ticket Indicator",
            Self::FastTrack =>
                "Fast Track",
        }
    }

}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(self.name())
    }
}
