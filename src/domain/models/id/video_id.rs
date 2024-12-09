use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

use super::valid_ascii::VALID_ASCII_CHARS;

/// 動画のID
///
/// 11文字の固定長
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Hash)]
pub struct VideoId(String);

impl VideoId {
    pub fn new(id: String) -> Result<Self, crate::Error> {
        if id.len() == 11 && id.chars().all(|c| VALID_ASCII_CHARS.contains(&c)) {
            Ok(Self(id))
        } else {
            Err(crate::Error::InvalidInput)
        }
    }
}

impl FromStr for VideoId {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

impl Display for VideoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for VideoId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = String::deserialize(deserializer)?;
        Self::new(id).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
impl VideoId {
    /// only test, return:`Self(12345678901)`
    pub(crate) fn inc_from_1() -> Self {
        let id = "12345678901".to_string();
        Self::new(id).unwrap()
    }
    /// only test, return:`Self(00000000000)`
    pub(crate) fn all_0() -> Self {
        let id = "00000000000".to_string();
        Self::new(id).unwrap()
    }
    /// only test, return:`Self(11111111111)`
    pub(crate) fn all_1() -> Self {
        let id = "11111111111".to_string();
        Self::new(id).unwrap()
    }
    /// only test, return:`Self(22222222222)`
    pub(crate) fn all_2() -> Self {
        let id = "22222222222".to_string();
        Self::new(id).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // `video_id`
    #[test]
    fn test_video_id_gives_valid() {
        let chars_len_11 = "aaaa-bbbb-c".to_string();
        assert!(VideoId::new(chars_len_11).is_ok());
    }

    #[test]
    fn test_video_id_gives_invalid() {
        let chats_len_10 = "aaaa-bbbb-".to_string();
        assert!(VideoId::new(chats_len_10).is_err());
        let chats_len_12 = "aaaa-bbbb-cc".to_string();
        assert!(VideoId::new(chats_len_12).is_err());
    }

    #[test]
    fn test_video_id_deserialize() {
        let chars_len_11 = "\"aaaa-bbbb-c\"";
        assert!(serde_json::from_str::<VideoId>(chars_len_11).is_ok())
    }

    #[test]
    fn test_video_id_for_test_fn() {
        // これらのメソッドが正常に動かない場合は内部で`panic`させているので
        // 値が返ってくる == メソッドが正常に動く
        let _a = VideoId::inc_from_1();
        let _b = VideoId::all_0();
        let _c = VideoId::all_1();
        let _d = VideoId::all_2();
    }
}
