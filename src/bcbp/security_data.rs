#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "with-serde", derive(serde::Serialize))]
pub struct SecurityData {
    pub kind: char,   // Item 28
    pub data: String, // length given by Item 29 (two hex digits)
}
