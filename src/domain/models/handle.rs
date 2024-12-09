use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// チャンネルのハンドルネーム
///
/// ascii文字以外も対応している
/// 詳しくは: https://support.google.com/youtube/answer/11585688
///
/// 内部の値は接頭語として `@` を持つ
#[derive(Debug, PartialEq, Eq, Clone, Serialize)]
pub struct Handle(String);

impl Handle {
    /// 引数の`id`は`@`が接頭語としてついていても問題ない
    pub fn new(id: String) -> Result<Self, crate::Error> {
        // `@`はハンドルネームとして使用できないかつ接頭語として付与することが多いので
        // `@`が存在すれば`@`を空文字に置換して処理
        let id = id.replace("@", "");
        if id.len() <= 30 && !id.is_empty() {
            Ok(Self(format!("@{}", id)))
        } else {
            Err(crate::Error::InvalidInput)
        }
    }
}

impl TryFrom<String> for Handle {
    type Error = crate::Error;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl TryFrom<&str> for Handle {
    type Error = crate::Error;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value.to_string())
    }
}

impl Display for Handle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for Handle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = String::deserialize(deserializer)?;
        Self::new(id).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_id_gives_valid() {
        let valid_1 = "foo_bar".to_string();
        assert!(Handle::new(valid_1).is_ok());
        let valid_2 = "@なななななんと~".to_string();
        assert!(Handle::new(valid_2).is_ok());
    }

    #[test]
    fn test_handle_id_gives_invalid() {
        let invalid_1 = "@".to_string();
        assert!(Handle::new(invalid_1).is_err());
        let invalid_2 = "too_too_too_too_too_too_too_too_too_long_string".to_string();
        assert!(Handle::new(invalid_2).is_err());
    }

    #[test]
    fn test_handle_id_deserialize() {
        let valid = "\"foo_bar\"";
        assert!(serde_json::from_str::<Handle>(valid).is_ok());
        let valid = "\"@なななななんと~\"";
        assert!(serde_json::from_str::<Handle>(valid).is_ok());
    }
}
