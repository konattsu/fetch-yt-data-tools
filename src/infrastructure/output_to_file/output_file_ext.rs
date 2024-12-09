use std::str::FromStr;

use serde::{de::Error, Deserialize, Deserializer};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OutputFileExt {
    Json,
    Yaml,
}

impl Default for OutputFileExt {
    fn default() -> Self {
        Self::Json
    }
}

impl OutputFileExt {
    /// enum値に対応する拡張子を返す
    ///
    /// 先頭の`.`を含まず,セパレート文字を含まない
    ///
    /// 戻り値の各文字に対して`std::Path::is_separator`が常に`false`
    pub fn get_ext(&self) -> &'static str {
        match self {
            Self::Json => "json",
            Self::Yaml => "yaml",
        }
    }
}

impl FromStr for OutputFileExt {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(Self::Json),
            "yaml" => Ok(Self::Yaml),
            _ => Err(format!("invalid input:`{}`", s)),
        }
    }
}

pub fn deserialize_option_ext_mode<'de, D>(
    deserializer: D,
) -> Result<Option<OutputFileExt>, D::Error>
where
    D: Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer)?
        .map(|m| OutputFileExt::from_str(&m).map_err(D::Error::custom))
        .transpose()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ext_mode_unwrap(s: &str) {
        OutputFileExt::from_str(s).map(|_| ()).unwrap()
    }

    #[test]
    fn test_output_file_ext_from_str() {
        let json_str_1 = "json";
        ext_mode_unwrap(json_str_1);
        let json_str_2 = "JsOn";
        ext_mode_unwrap(json_str_2);
        let json_str_3 = "JSON";
        ext_mode_unwrap(json_str_3);
        let yaml_str = "yaml";
        ext_mode_unwrap(yaml_str);

        let invalid_json_str_1 = "json__";
        assert!(OutputFileExt::from_str(invalid_json_str_1).is_err());
        let invalid_json_str_2 = "J S O N";
        assert!(OutputFileExt::from_str(invalid_json_str_2).is_err());
        let invalid_json_str_3 = "foo";
        assert!(OutputFileExt::from_str(invalid_json_str_3).is_err());
    }

    #[test]
    fn test_output_file_ext_no_include_separator_chars_for_ext() {
        let is_not_include_separator =
            |chars: &str| !chars.chars().any(std::path::is_separator);
        let json_str = OutputFileExt::Json.get_ext();
        let yaml_str = OutputFileExt::Yaml.get_ext();
        assert!(is_not_include_separator(json_str));
        assert!(is_not_include_separator(yaml_str));
    }

    #[test]
    fn test_output_file_ext_get_ext() {
        assert_eq!(OutputFileExt::Json.get_ext(), "json");
        assert_eq!(OutputFileExt::Yaml.get_ext(), "yaml");
    }
}
