use std::{any::Any, collections::HashMap, mem::size_of};

use crate::impls::file::File;

use super::{format_error::FormatError, setting::Setting, setting_value::SettingValue};

/// Represents all the [Setting] using the [HashMap] to manage the key/value values.
#[derive(Debug, PartialEq)]
pub struct Config {
    pub(crate) settings: HashMap<String, SettingValue>,
}

impl Config {
    pub fn new(capacity: Option<usize>) -> Self {
        Self {
            settings: HashMap::with_capacity(capacity.unwrap_or_default()),
        }
    }

    /// Try to get the raw number type from the setting using [Downcasting](https://ysantos.com/blog/downcast-rust).
    pub fn get_number_setting(&self, key: &str) -> Option<&dyn ToString> {
        self.settings.get(key).and_then(|value| match value {
            SettingValue::Number(n) => Some(n as &dyn ToString),
            _ => None,
        })
    }

    pub fn get_setting(&self, key: &str) -> Option<&dyn Any> {
        self.settings.get(key).map(|setting| match setting {
            SettingValue::Number(v) => v as &dyn Any,
            SettingValue::String(v) => v as &dyn Any,
            SettingValue::Boolean(v) => v as &dyn Any,
        })
    }
}

impl TryFrom<&File> for Config {
    type Error = FormatError;

    fn try_from(file: &File) -> Result<Self, Self::Error> {
        let lines = file.contents.lines();
        let (lower, higher) = lines.size_hint();
        let mut config = Config::new(Some(size_of::<Setting>() * higher.unwrap_or(lower)));

        for line in lines {
            let setting = Setting::try_from(line)?;

            config.settings.insert(setting.key, setting.value);
        }

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use std::{ffi::OsString, str::FromStr};

    use crate::impls::{
        file::File,
        file_format::{config::Config, format_error::FormatError, setting_value::SettingValue},
    };

    #[test]
    fn should_parser_setting() {
        let file = File {
            name: OsString::from_str("teste").unwrap(),
            contents: "key=teste\nkey=true\n\true=false".to_string(),
        };
        let config: Result<Config, FormatError> = (&file).try_into();

        println!("{config:?}");
    }

    #[test]
    fn should_retrieve_the_correct_setting_value_type() {
        let mut config = Config::new(None);
        config
            .settings
            .insert("teste".to_string(), SettingValue::Boolean(true));
        config
            .settings
            .insert("teste2".to_string(), SettingValue::Number(242));

        let number_setting = config.get_number_setting("teste2").unwrap().to_string();

        assert_eq!(number_setting, "242")
    }
}
