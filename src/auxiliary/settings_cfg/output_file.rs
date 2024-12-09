use std::str::FromStr;

use super::PathBufValidUtf8;

/// 結果を出力するファイルの設定
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum OutputFile {
    /// ユーザーが指定したパス
    Path(PathBufValidUtf8),
    /// 現在時刻(Utc) e.g. 20241015T0130Z
    Date,
}

impl FromStr for OutputFile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "date" {
            Ok(OutputFile::Date)
        } else {
            let valid_path = PathBufValidUtf8::from_str(s)?;
            Ok(OutputFile::Path(valid_path))
        }
    }
}

impl OutputFile {
    pub(super) fn get_path(&self) -> PathBufValidUtf8 {
        match self {
            Self::Path(path) => path.clone(),
            Self::Date => {
                let today = fetch_date();
                PathBufValidUtf8::from_str(&format!("./{}", today)).unwrap()
            }
        }
    }
}

/// 現在時刻(Utc) e.g. 20241015T0130Z
fn fetch_date() -> String {
    chrono::Utc::now().format("%Y%m%dT%H%M").to_string()
}
