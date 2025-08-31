// field.rs - IATA BCBP field definitions and metadata
use std::fmt;

/// Metadata for an IATA BCBP field.
///
/// * `len` — required size in bytes; `0` means variable-length.
/// * `name` — short human-readable label from the Implementation Guide.
///
/// This is generated alongside [`Field`] to keep enum variants and metadata in sync.
#[derive(Copy, Clone)]
pub struct FieldMeta {
    pub len: u8, // 0 = variable length
    pub name: &'static str,
}

/// Declare the BCBP fields once and generate:
/// - `enum Field` with per-variant documentation
/// - `ALL_FIELDS` (canonical order)
/// - `FIELD_META` (parallel metadata table)
/// - convenience methods on `Field`
///
/// Each tuple is: `(Variant, len, "Short Name", "Docstring")`.
macro_rules! define_fields {
    (
        $(
            $(#[$m:meta])*
            ($v:ident, $len:expr, $name:expr)
        ),+ $(,)?
    ) => {
        /// IATA BCBP fields in canonical (BAC) order.
        #[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
        #[repr(u8)]
        pub enum Field {
            $(
                $(#[$m])*
                $v
            ),+
        }

        /// Canonical list of all fields in the same order as [`Field`] discriminants.
        pub const ALL_FIELDS: &[Field] = &[
            $( Field::$v ),+
        ];

        /// Parallel metadata table; indices match [`Field`] discriminants.
        pub const FIELD_META: &[FieldMeta] = &[
            $( FieldMeta { len: $len, name: $name } ),+
        ];

        impl Field {
            /// Returns the zero-based index of this variant in [`FIELD_META`].
            #[inline]
            pub const fn idx(self) -> usize { self as usize }

            /// Required field length in bytes.
            ///
            /// *Returns `0` for variable-length fields.*
            ///
            /// # Examples
            ///
            /// ```
            /// # use iata::bcbp::format::Field;
            /// assert_eq!(Field::FormatCode.len(), 1);
            /// assert_eq!(Field::SecurityData.len(), 0); // variable-length
            /// ```
            #[inline]
            pub const fn len(self) -> usize {
                FIELD_META[self.idx()].len as usize
            }

            /// Human-readable name as specified in the Implementation Guide.
            ///
            /// # Examples
            ///
            /// ```
            /// # use iata::bcbp::format::Field;
            /// assert_eq!(Field::FlightNumber.name(), "Flight Number");
            /// ```
            #[inline]
            pub const fn name(self) -> &'static str {
                FIELD_META[self.idx()].name
            }

            /// Returns `true` if the field has variable length (`len() == 0`).
            #[inline]
            pub const fn is_variable(self) -> bool {
                FIELD_META[self.idx()].len == 0
            }

            /// Returns `true` if the field has fixed length (`len() > 0`).
            #[inline]
            pub const fn is_fixed(self) -> bool {
                !self.is_variable()
            }

            /// Iterates over all fields in canonical order.
            ///
            /// # Examples
            ///
            /// ```
            /// # use iata::bcbp::format::Field;
            /// for f in Field::iter() {
            ///     let _ = (f.name(), f.len());
            /// }
            /// ```
            #[inline]
            pub fn iter() -> impl Iterator<Item = Field> + 'static {
                ALL_FIELDS.iter().copied()
            }
        }

        impl fmt::Display for Field {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.name())
            }
        }
    }
}

// ---- Single source of truth below: add/modify here only ----
#[rustfmt::skip]
define_fields!(
    /// Item 1: Format Code. 1 byte. Format 'f'.
    (FormatCode,                                1,  "Format Code"),

    /// Item 4: Airline Individual Use. n bytes. Format unspecified.
    (AirlineIndividualUse,                      0,  "Airline Individual Use"),

    /// Item 5: Number of Legs Encoded. 1 byte. Format 'N'.
    (NumberOfLegsEncoded,                       1,  "Number of Legs Encoded"),

    /// Item 6: Field Size of Variable Size Field. 2 bytes. Format 'f'. Hexadecimal.
    (FieldSizeOfVariableSizeField,              2,  "Field Size of Variable Size Field"),

    /// Item 7: Operating Carrier PNR Code. 7 bytes. Format 'f'.
    (OperatingCarrierPnrCode,                   7,  "Operating Carrier PNR Code"),

    /// Item 8: Beginning of Version Number. 1 byte. Format 'f'.
    (BeginningOfVersionNumber,                  1,  "Beginning of Version Number"),

    /// Item 9: Version Number. 1 byte. Format 'f'.
    (VersionNumber,                             1,  "Version Number"),

    /// Item 10: Field Size of Structured Message. 2 bytes. Format 'f'. Hexadecimal. (Unique)
    (FieldSizeOfStructuredMessageUnique,        2,  "Field Size of Structured Message (Unique)"),

    /// Item 11: Passenger Name. 20 bytes. Format 'f'.
    (PassengerName,                             20, "Passenger Name"),

    /// Item 12: Source of Check-In. 1 byte. Format 'f'.
    (SourceOfCheckIn,                           1,  "Source of Check-In"),

    /// Item 14: Source of Boarding Pass Issuance. 1 byte. Format 'f'.
    (SourceOfBoardingPassIssuance,              1,  "Source of Boarding Pass Issuance"),

    /// Item 15: Passenger Description. 1 byte. Format 'f'.
    (PassengerDescription,                      1,  "Passenger Description"),

    /// Item 16: Document Type. 1 byte. Format 'f'.
    (DocumentType,                              1,  "Document Type"),

    /// Item 17: Field Size of Structured Message. 2 bytes. Format 'f'. Hexadecimal. (Repeated)
    (FieldSizeOfStructuredMessageRepeated,      2,  "Field Size of Structured Message (Repeated)"),

    /// Item 18: Selectee Indicator. 1 byte. Format 'f'.
    (SelecteeIndicator,                         1,  "Selectee Indicator"),

    /// Item 19: Marketing Carrier Designator. 3 bytes. Format 'f'.
    (MarketingCarrierDesignator,                3,  "Marketing Carrier Designator"),

    /// Item 20: Frequent Flyer Airline Designator. 3 bytes. Format 'f'.
    (FrequentFlyerAirlineDesignator,            3,  "Frequent Flyer Airline Designator"),

    /// Item 21: Airline Designator of Boarding Pass Issuer. 3 bytes. Format 'f'.
    (AirlineDesignatorOfBoardingPassIssuer,     3,  "Airline Designator of Boarding Pass Issuer"),

    /// Item 22: Date of Issue of Boarding Pass. 4 bytes. Format 'N'.
    (DateOfIssueOfBoardingPass,                 4,  "Date of Issue of Boarding Pass"),

    /// Item 23: Baggage Tag License Plate Number(s). 13 bytes. Format 'f'.
    (BaggageTagNumbers,                         13, "Baggage Tag License Plate Number(s)"),

    /// Item 25: Beginning of Security Data. 1 byte. Format 'f'.
    (BeginningOfSecurityData,                   1,  "Beginning of Security Data"),

    /// Item 26: From City Airport Code. 3 bytes. Format 'a'.
    (FromCityAirportCode,                       3,  "From City Airport Code"),

    /// Item 28: Type of Security Data. 1 byte. Format 'f'.
    (TypeOfSecurityData,                        1,  "Type of Security Data"),

    /// Item 29: Length of Security Data. 2 bytes. Format 'f'. Hexadecimal.
    (LengthOfSecurityData,                      2,  "Length of Security Data"),

    /// Item 30: Security Data. n bytes. Format 'f'.
    (SecurityData,                              0,  "Security Data"),

    /// Item 31: First Non-Consecutive Baggage Tag License Plate Number. 13 bytes. Format 'f'.
    (FirstNonConsecutiveBaggageTagNumbers,     13,
        "First Non-Consecutive Baggage Tag License Plate Number"),

    /// Item 32: Second Non-Consecutive Baggage Tag License Plate Number. 13 bytes. Format 'f'.
    (SecondNonConsecutiveBaggageTagNumbers,    13,
        "Second Non-Consecutive Baggage Tag License Plate Number"),

    /// Item 38: To City Airport Code. 3 bytes. Format 'a'.
    (ToCityAirportCode,                         3,  "To City Airport Code"),

    /// Item 42: Operating Carrier Designator. 3 bytes. Format 'f'.
    (OperatingCarrierDesignator,                3,  "Operating Carrier Designator"),

    /// Item 43: Flight Number. 5 bytes. Format 'NNNN[a]'.
    (FlightNumber,                              5,  "Flight Number"),

    /// Item 46: Date of Flight. 3 bytes. Format 'N'.
    (DateOfFlight,                              3,  "Date of Flight"),

    /// Item 71: Compartment Code. 1 byte. Format 'a'.
    (CompartmentCode,                           1,  "Compartment Code"),

    /// Item 89: ID/AD Indicator. 1 byte. Format 'f'.
    (IdAdIndicator,                             1,  "ID/AD Indicator"),

    /// Item 104: Seat Number. 4 bytes. Usually 'NNNa', but can be 'INF ' or similar.
    (SeatNumber,                                4,  "Seat Number"),

    /// Item 107: Check-In Sequence Number. 5 bytes. Usually 'NNNN[f]', but can be 'f'.
    (CheckInSequenceNumber,                     5,  "Check-In Sequence Number"),

    /// Item 108: International Document Verification. 1 byte. Format 'f'.
    (InternationalDocumentVerification,         1,  "International Document Verification"),

    /// Item 113: Passenger Status. 1 byte. Format 'f'.
    (PassengerStatus,                           1,  "Passenger Status"),

    /// Item 118: Free Baggage Allowance. 3 bytes. Format 'f'.
    (FreeBaggageAllowance,                      3,  "Free Baggage Allowance"),

    /// Item 142: Airline Numeric Code. 3 bytes. Format 'N'.
    (AirlineNumericCode,                        3,  "Airline Numeric Code"),

    /// Item 143: Document Form / Serial Number. 10 bytes. Format 'f'.
    (DocumentFormSerialNumber,                  10, "Document Form / Serial Number"),

    /// Item 236: Frequent Flyer Number. 16 bytes. Format 'f'.
    (FrequentFlyerNumber,                       16, "Frequent Flyer Number"),

    /// Item 253: Electronic Ticket Indicator. 1 byte. Format 'f'.
    (ElectronicTicketIndicator,                 1,  "Electronic Ticket Indicator"),

    /// Item 254: Fast Track. 1 byte. Format 'f'.
    (FastTrack,                                 1,  "Fast Track"),
);
