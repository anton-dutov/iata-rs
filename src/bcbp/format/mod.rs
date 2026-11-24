mod field;
pub use self::field::Field;

impl Field {
    pub(super) fn validate_text(self, value: &str) -> super::BcbpResult<()> {
        let max = self.len();
        if self.is_fixed() && value.len() > max {
            return Err(super::Error::FieldLengthExceeded { field: self, max });
        }
        if !value.is_ascii() {
            return Err(super::Error::InvalidCharacters);
        }
        Ok(())
    }
}
