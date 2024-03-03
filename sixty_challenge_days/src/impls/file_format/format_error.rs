#[derive(Debug, PartialEq)]
pub enum FormatError {
    InvalidFormat(String),
    InvalidValueType,
}
