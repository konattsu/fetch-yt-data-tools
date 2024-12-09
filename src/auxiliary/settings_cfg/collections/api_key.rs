use serde::Deserialize;
use std::{
    fmt::Debug,
    io::{self, Write},
    str::FromStr,
};

#[derive(Clone, Deserialize)]
pub struct ApiKey(String);

impl ApiKey {
    pub fn new(key: String) -> Result<Self, String> {
        if key.is_empty() {
            Err("`key` length must be 1 or more".into())
        } else {
            Ok(Self(key))
        }
    }

    pub fn prompt() -> Self {
        loop {
            let mut buffer = String::new();
            print!("Input YouTube Api Key: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut buffer).unwrap();
            let key = buffer.trim().to_string();
            match Self::new(key) {
                Ok(v) => break v,
                Err(e) => println!("{}. Please input again", e),
            }
        }
    }

    /// `api_key`を`String`として取り出す
    ///
    /// ログなどに出力されないように注意
    pub fn as_string(&self) -> String {
        self.0.clone()
    }
}

// INFO: `String`側の`Debug trait`が使用されないように
// `Deref`,`DerefMut`を実装しない

impl Debug for ApiKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ApiKey(`secret`(holds value))")
    }
}

impl FromStr for ApiKey {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s.to_string())
    }
}
