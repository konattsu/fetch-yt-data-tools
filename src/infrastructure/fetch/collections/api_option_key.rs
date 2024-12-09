#[derive(Debug, Clone)]
pub struct ApiOptionKey(Vec<String>);

impl ApiOptionKey {
    /// キーの詳細については以下参照
    ///
    /// - video:
    ///     https://developers.google.com/youtube/v3/docs/videos/list#part
    ///
    /// - playlist:
    ///     https://developers.google.com/youtube/v3/docs/playlists/list#part
    ///
    /// `Err`: 必須のオプションが含まれていないとき
    ///
    /// `Self::required_keys()`で確認可能
    pub fn new(key: Vec<String>) -> Result<Self, String> {
        let required_keys = Self::required_keys();

        if !key.iter().any(|part_key| required_keys.contains(part_key)) {
            Err(format!(
                "keys:`{}` is required to create `ApiOptionKey`, but gives `{}`",
                required_keys.join(", "),
                key.join(", ")
            ))
        } else {
            Ok(Self(key))
        }
    }

    pub fn required_keys() -> Vec<String> {
        // 今のところ必須なものが共通なのでこれで大丈夫
        const REQUIRED_KEY_1: &str = "snippet";
        vec![REQUIRED_KEY_1.into()]
    }

    pub fn join(&self, sep: &str) -> String {
        self.0.join(sep)
    }
}

impl Default for ApiOptionKey {
    fn default() -> Self {
        Self::new(Self::required_keys()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_option_key_for_new() {
        let valid_keys =
            vec!["snippet".to_string(), "foo".to_string(), "bar".to_string()];
        assert!(ApiOptionKey::new(valid_keys).is_ok());

        let invalid_keys = vec!["foo".to_string(), "bar".to_string()];
        assert!(ApiOptionKey::new(invalid_keys).is_err());
    }

    #[test]
    fn test_api_option_key_for_join() {
        let keys = vec!["snippet".to_string(), "foo".to_string()];
        let api_option_key = ApiOptionKey::new(keys).unwrap();
        assert_eq!(api_option_key.join(","), "snippet,foo");
    }
}
