use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

use super::valid_ascii::VALID_ASCII_CHARS;

/// 再生リストのID
///
/// 34文字の固定長
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Hash)]
pub struct PlaylistId(String);

impl PlaylistId {
    pub fn new(id: String) -> Result<Self, crate::Error> {
        if id.len() == 34 && id.chars().all(|c| VALID_ASCII_CHARS.contains(&c)) {
            Ok(Self(id))
        } else {
            Err(crate::Error::InvalidInput)
        }
    }
}

impl FromStr for PlaylistId {
    type Err = crate::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}

impl Display for PlaylistId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<'de> Deserialize<'de> for PlaylistId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let id = String::deserialize(deserializer)?;
        Self::new(id).map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
impl PlaylistId {
    /// only test, return:`Self(1234567890123...1234)`
    pub(crate) fn inc_from_1() -> Self {
        let id = "1234567890123456789012345678901234".to_string();
        Self::new(id).unwrap()
    }
    /// only test, return `Self(111111...111)`
    pub(crate) fn all_1() -> Self {
        let id = "1111111111111111111111111111111111".to_string();
        Self::new(id).unwrap()
    }
    /// only test, return `Self(222222...222)`
    pub(crate) fn all_2() -> Self {
        let id = "2222222222222222222222222222222222".to_string();
        Self::new(id).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // `playlist_id`
    #[test]
    fn test_playlist_id_gives_valid() {
        let chars_len_34 = "aaaa-bbbb-cccc-dddd-eeee-ffff-gggg".to_string();
        assert!(PlaylistId::new(chars_len_34).is_ok());
    }

    #[test]
    fn test_playlist_id_gives_invalid() {
        let chars_len_33 = "aaaa-bbbb-cccc-dddd-eeee-ffff-ggg".to_string();
        assert!(PlaylistId::new(chars_len_33).is_err());
        let chars_len_35 = "aaaa-bbbb-cccc-dddd-eeee-ffff-gggg-".to_string();
        assert!(PlaylistId::new(chars_len_35).is_err());
    }

    #[test]
    fn test_playlist_id_deserialize() {
        let chars_len_34 = "\"aaaa-bbbb-cccc-dddd-eeee-ffff-gggg\"";
        assert!(serde_json::from_str::<PlaylistId>(chars_len_34).is_ok());
    }

    #[test]
    fn test_playlist_id_for_test_fn() {
        let _a = PlaylistId::inc_from_1();
        let _b = PlaylistId::all_1();
        let _c = PlaylistId::all_2();
    }
}
