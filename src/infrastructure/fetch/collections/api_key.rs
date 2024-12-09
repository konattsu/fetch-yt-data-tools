use std::fmt::Debug;

#[derive(Clone)]
pub struct ApiKey(String);

impl ApiKey {
    pub fn new(api_key: String) -> Self {
        Self(api_key)
    }

    /// `String`に変換するため値をログなどに出力しないように注意
    pub fn as_string(&self) -> String {
        self.0.clone()
    }
}

impl From<String> for ApiKey {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl Debug for ApiKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ApiKey(`secret value`)")
    }
}
