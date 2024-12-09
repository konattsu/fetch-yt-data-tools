use std::{
    fmt::Display,
    io::{self, Write},
    ops::Deref,
    path::PathBuf,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

/// PathBufに無効な`utf-8`文字を保持しないという制約を付与
///
/// これにより`to_string()`を安全に扱える
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct PathBufValidUtf8(PathBuf);

impl PathBufValidUtf8 {
    /// 拡張子を付与する
    ///
    /// `.`は`ext`に含まれていても含まれていなくても問題ない
    pub fn add_ext(self, ext: &str) -> Self {
        let path_str = format!("{}.{}", self, ext.trim_start_matches('.'));
        // `PathBufValidUtf8`に有効な`uft-8`文字のみ結合するので
        // この`from_str`は失敗しない
        Self::from_str(&path_str).unwrap()
    }

    /// `should_not_exist`ファイルが存在しないことを確約
    pub fn prompt(should_not_exist: bool) -> Self {
        let mut buffer = String::new();
        loop {
            print!("Input path to output file without ext: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut buffer).unwrap();
            match Self::from_str(buffer.trim()) {
                Ok(path) => {
                    if path.exists() && should_not_exist {
                        println!(
                            "inputted file({}) is already exists, please input again",
                            path
                        );
                    } else {
                        println!("applied output file path:`{}`", path);
                        return path;
                    }
                }
                Err(e) => {
                    println!("{}, please input again", e);
                }
            }
        }
    }
}

impl Display for PathBufValidUtf8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.to_str() {
            Some(path) => write!(f, "{}", path),
            // 無効な`utf-8`文字が含まれている場合は, 型:`Self`を作成できないので
            // 引数に`&self`を必要とするこのメソッドは呼び出せない, だから`panic`呼ぶ
            None => panic!(
                "{}\n{}, self:`{:?}`",
                "Bug! Internal specifications have been changed. ",
                "review the impl of `display trait` for `PathBufValidUtf8`",
                self
            ),
        }
    }
}

impl Deref for PathBufValidUtf8 {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<PathBufValidUtf8> for PathBuf {
    fn from(value: PathBufValidUtf8) -> Self {
        value.0
    }
}

impl FromStr for PathBufValidUtf8 {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(valid_path) = PathBuf::from(s).to_str() {
            if valid_path.is_empty() {
                Err("path must be at least 1 chars".into())
            } else {
                Ok(Self(valid_path.into()))
            }
        } else {
            Err("path contains invalid utf-8 chars".into())
        }
    }
}
