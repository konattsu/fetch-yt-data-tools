use serde::Deserialize;
use std::{fs, path::PathBuf};
use thiserror::Error;
use tracing::level_filters::LevelFilter;

use super::{ApiKey, PathBufValidUtf8};
use crate::{
    infrastructure::output_to_file::{deserialize_option_ext_mode, OutputFileExt},
    util::serde::deserialize_option_level_filter,
};

// NOTE `Config`構造体を作成し,パースに使用した理由
// `.toml`内の名前空間が汚れないようにするため
//
// `Config`構造体をパースに使用したときの`.toml`
// [fetch-yt-data-tools]
// foo=123
// bar=true
// [settings_for_other_systems]
// foo=456
// bar=false
//
// このように同じファイルを様々なシステムごとに使い分けることもできる

#[derive(Debug, Deserialize)]
struct Config {
    fetch_yt_data_tools: FileSettings,
}

/// ファイルから取得した情報を格納
#[derive(Debug, Deserialize)]
pub(super) struct FileSettings {
    youtube_data_api_key: Option<ApiKey>,
    #[serde(default, deserialize_with = "deserialize_option_level_filter")]
    stdout_log_level: Option<LevelFilter>,
    #[serde(default, deserialize_with = "deserialize_option_level_filter")]
    file_log_level: Option<LevelFilter>,
    output_path_without_ext: Option<PathBufValidUtf8>,
    #[serde(default, deserialize_with = "deserialize_option_ext_mode")]
    output_file_ext: Option<OutputFileExt>,
}

#[derive(Debug, Error)]
pub(super) enum FileError {
    #[error("failed to find settings file; specified path: {0}")]
    Find(String),
    #[error("failed to read settings file content: {0}")]
    Read(String),
    #[error("deserialization failed due to invalid values: {0}")]
    Deserialize(String),
}

impl FileSettings {
    pub(super) fn load(file: &PathBuf) -> Result<Self, FileError> {
        if !(file.exists() && file.is_file()) {
            return Err(FileError::Find(file.to_string_lossy().to_string()));
        }
        let content =
            fs::read_to_string(file).map_err(|e| FileError::Read(e.to_string()))?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| FileError::Deserialize(e.to_string()))?;
        Ok(config.fetch_yt_data_tools)
    }

    pub(super) fn youtube_data_api_key(&self) -> Option<ApiKey> {
        self.youtube_data_api_key.clone()
    }
    pub(super) fn stdout_log_level(&self) -> Option<LevelFilter> {
        self.stdout_log_level
    }
    pub(super) fn file_log_level(&self) -> Option<LevelFilter> {
        self.file_log_level
    }
    pub(super) fn output_path_without_ext(&self) -> Option<PathBufValidUtf8> {
        self.output_path_without_ext.clone()
    }
    pub(super) fn output_file_ext(&self) -> Option<OutputFileExt> {
        self.output_file_ext
    }
}
