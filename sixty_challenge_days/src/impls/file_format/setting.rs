use super::{format_error::FormatError, setting_value::SettingValue};

#[derive(Debug, PartialEq)]
pub struct Setting {
    pub(crate) key: String,
    pub(crate) value: SettingValue,
}

impl TryFrom<&str> for Setting {
    type Error = FormatError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let setting = value
            .split("=")
            .map(|value| value.trim())
            .collect::<Vec<&str>>();

        if setting.len() != 2 {
            Err(FormatError::InvalidFormat(
                format!(
                    "Invalid File format! Expect key=value format, receives: {}",
                    value
                )
                .to_string(),
            ))
        } else {
            Ok(Self {
                key: setting[0].to_string(),
                value: setting[1].try_into()?,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::impls::file_format::{
        format_error::FormatError, setting::Setting, setting_value::SettingValue,
    };

    #[test]
    fn should_parser_setting_error() {
        let setting = "key value";
        let setting = Setting::try_from(setting);

        assert_eq!(
            setting,
            Err(FormatError::InvalidFormat(
                "Invalid File format! Expect key=value format, receives: key value".to_string()
            ))
        )
    }

    #[test]
    fn should_parser_setting() {
        let setting = "key = true";
        let setting = Setting::try_from(setting);

        assert_eq!(
            setting,
            Ok(Setting {
                key: "key".to_string(),
                value: SettingValue::Boolean(true),
            },)
        )
    }
}
