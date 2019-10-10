use log::*;

use super::{
//     raw,
//     field,
    error::{Error, Result}
};

use super::field::Field;

// #[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct Chunk<'a> {
    input: &'a str,
}

impl<'a> Chunk<'a> {

    /// Return a new intance of the receiver over the `input`.
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    /// Returns `true` if no more input is available.
    #[inline]
    pub fn eof(&self) -> bool {
        self.input.is_empty()
    }

    /// Returns the number of bytes of input remaining to process.
    #[inline]
    pub fn len(&self) -> usize {
        self.input.len()
    }

    /// Returns a chunk over a fixed-length sub-section of the input.
    /// The entire amount is consumed immediately if space is available whether or not
    /// any fields within the sub-section are invalid.
    ///
    /// # Panics
    /// Will panic if `len` is `0`.
    pub fn fetch_chunk(&mut self, len: usize) -> Result<Chunk<'a>> {
        assert!(
            len > 0,
            "Attempting to scan a zero-length sub-field list is not valid."
        );
        trace!("Scanning Subsection (Length {})", len);
        if self.len() < len {
            Err(Error::SubsectionTooLong)
        } else {
            let sub_fields = &self.input[..len];
            self.input = &self.input[len..];
            Ok(Self::new(sub_fields))
        }
    }

    /// Scans and returns the string underlying a field (variable or fixed-length)
    /// with a specified length value.
    ///
    /// # Panics
    /// Will panic if `len` is `0`.
    /// Will panic if the fixed-length field intrinsic length is not equal to `len`.
    pub fn fetch_str_len(&mut self, field: Field, len: usize) -> Result<&'a str> {
        assert!(len > 0, "Attempting to scan zero bytes of data.");
        assert!(
            field.len() == 0 || field.len() == len,
            "Length is not compatible the intrinsic length of the field."
        );
        if self.len() < len {
            trace!(
                "Unexpected End of Input Scanning {} (Length {})",
                field,
                len
            );
            Err(Error::UnexpectedEndOfInput(field))
        } else {
            let substring = &self.input[..len];
            self.input = &self.input[len..];
            trace!("Scanning {} (Length {}) - '{}'", field, len, substring);
            Ok(substring)
        }
    }

    /// Scans and returns the string underlying a fixed-length field.
    /// Uses the intrinsic length.
    ///
    /// # Panics
    /// Will panic if `field` is variable-length.
    pub fn fetch_str(&mut self, field: Field) -> Result<&'a str> {
        assert!(
            field.len() != 0,
            "Attempting to scan a variable-length field as fixed-length."
        );
        self.fetch_str_len(field, field.len())
    }

    /// Scans and returns an optional string underlying a fixed-length field.
    /// If there is no more input to process, returns `Ok(None)`.
    /// Uses the intrinsic length.
    ///
    /// # Panics
    /// Will panic if `field` is variable-length.
    pub fn fetch_str_opt(&mut self, field: Field) -> Result<Option<&'a str>> {
        assert!(
            field.len() != 0,
            "Attempting to scan a variable-length field as fixed-length."
        );
        if self.eof() {
            Ok(None)
        } else {
            self.fetch_str(field).map(Some)
        }
    }

    /// Scans and returns the character value underlying a fixed-length field.
    ///
    /// # Panics
    /// Will panic if `field` is a length other than 1.
    pub fn fetch_char(&mut self, field: Field) -> Result<char> {
        assert!(
            field.len() == 1,
            "Attempting to scan a single character out of a longer field."
        );
        self.fetch_str(field)
            .map(|value| value.chars().next().unwrap())
    }

    /// Scans and returns an optional character value underlying a fixed-length field.
    /// If there is no more input to process, returns `Ok(None)`.
    ///
    /// # Panics
    /// Will panic if `field` is a length other than 1.
    pub fn fetch_char_opt(&mut self, field: Field) -> Result<Option<char>> {
        assert!(
            field.len() == 1,
            "Attempting to scan a single character out of a longer field."
        );
        if self.eof() {
            Ok(None)
        } else {
            self.fetch_char(field).map(Some)
        }
    }

    /// Scans a fixed-length numeric field yielding the numeric value interpreted
    /// with the given `radix`.
    ///
    /// # Panics
    /// Will panic if `field` is variable-length.
    ///
    /// # Issues
    /// Should not advance the input until the numeric value is sucessfully scanned.
    pub fn fetch_usize(&mut self, field: Field, radix: u32) -> Result<usize> {
        self.fetch_str(field).and_then(|str_value| {
            usize::from_str_radix(str_value, radix).map_err(|_| Error::ExpectedInteger(field))
        })
    }
}
