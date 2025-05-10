use super::format_error::FormatError;

#[derive(Debug, PartialEq, Eq)]
pub enum SettingValue {
    String(String),
    Number(i32),
    Boolean(bool),
}

impl SettingValue {
    pub fn convert_to_valid_type<T>(value: &str) -> Result<T, FormatError>
    where
        T: std::str::FromStr,
    {
        value
            .parse::<T>()
            .map_err(|_| FormatError::InvalidValueType)
    }
}

impl TryFrom<&str> for SettingValue {
    type Error = FormatError;

    /// Tries to determine the value type to ``&str`` using the experimental [if-let-guard](https://rust-lang.github.io/rfcs/2294-if-let-guard.html).
    /// Returns [FormatError::InvalidValueType] if there is not a type corresponded.
    /// ### Example:
    /// ```
    /// let true_str = "true";
    /// let setting_value = true_str::try_into();
    /// assert_eq!(setting_value, Ok(SettingValue::Boolean(true)));
    /// ```
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            _ if let Ok(parsed_int) = SettingValue::convert_to_valid_type::<i32>(value) => {
                Ok(SettingValue::Number(parsed_int))
            }
            _ if let Ok(parsed_bool) = SettingValue::convert_to_valid_type::<bool>(value) => {
                Ok(SettingValue::Boolean(parsed_bool))
            }
            _ if let Ok(parsed) = SettingValue::convert_to_valid_type::<String>(value) => {
                Ok(SettingValue::String(parsed))
            }
            _ => Err(FormatError::InvalidValueType),
        }
    }
}
