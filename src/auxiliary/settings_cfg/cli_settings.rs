use clap::{Parser, ValueEnum};
use std::{path::PathBuf, str::FromStr};
use tracing::level_filters::LevelFilter;

use super::{output_file::OutputFile, ApiKey, PathBufValidUtf8};
use crate::infrastructure::output_to_file::OutputFileExt as InfraOutputFileExt;

/// fetch video data using youtube api
#[derive(Debug, Parser)]
pub(super) struct CliSettings {
    // how to load settings
    /// path to configuration file
    #[arg(short, long, env, default_value_t = default_settings_path_parser())]
    settings_path: PathBufValidUtf8,
    /// no use settings file
    #[arg(short, long, env, default_value_t = false)]
    no_use_settings_file: bool,

    // api
    /// input api the in the program
    #[arg(short, long, env, default_value_t = false)]
    input_api_key: bool,
    /// the key on `YouTube data v3 api`
    #[clap(skip)]
    youtube_data_api_key: Option<ApiKey>,

    // log level
    /// log level of standard output
    #[arg(long, env)]
    stdout_log_level: Option<LogLevel>,
    /// log level of file output
    #[arg(long, env)]
    file_log_level: Option<LogLevel>,

    // output
    /// path to output fetched data
    #[arg(short, long, env)]
    output_file_without_ext: Option<OutputFile>,
    /// output file extension
    #[arg(long, env)]
    output_file_ext: Option<OutputFileExt>,
}

/// デフォルトの設定ファイルへのパス
///
/// パスを変更することもできるが`.toml`でないとファイルの内容を読み取れない
fn default_settings_path_parser() -> PathBufValidUtf8 {
    PathBufValidUtf8::from_str("./settings.toml").unwrap()
}

impl CliSettings {
    /// コマンドライン引数を解析する
    ///
    /// `clap::Parse`よりこれを推奨 (`clap::Parse`は`API_KEY`を環境変数から読み込まない)
    pub(super) fn parse_from_cmd_args() -> Self {
        let mut cli_settings = CliSettings::parse();

        if let Ok(key) = std::env::var("YOUTUBE_DATA_API_KEY") {
            if let Ok(key) = ApiKey::new(key) {
                cli_settings.youtube_data_api_key = Some(key);
            } else {
                // ログの設定をしていないので標準出力
                println!(
                    "`YOUTUBE_DATA_API_KEY` is set, but ignored due to invalid fmt."
                );
            }
        }
        cli_settings
    }

    pub(super) fn settings_path(&self) -> PathBuf {
        self.settings_path.clone().to_path_buf()
    }
    pub(super) fn use_settings_path(&self) -> bool {
        // 呼び出し側で使用しやすいようにbool値を反転させる
        !self.no_use_settings_file
    }
    pub(super) fn input_api_key(&self) -> bool {
        self.input_api_key
    }
    pub(super) fn youtube_data_api_key(&self) -> Option<ApiKey> {
        self.youtube_data_api_key.clone()
    }
    pub(super) fn stdout_log_level(&self) -> Option<LevelFilter> {
        self.stdout_log_level.map(|level| level.into())
    }
    pub(super) fn file_log_level(&self) -> Option<LevelFilter> {
        self.file_log_level.map(|level| level.into())
    }
    pub(super) fn output_file(&self) -> Option<PathBufValidUtf8> {
        self.output_file_without_ext.clone().map(|f| f.get_path())
    }
    pub(super) fn output_file_ext(&self) -> Option<InfraOutputFileExt> {
        self.output_file_ext.map(Into::into)
    }
}

#[derive(ValueEnum, Debug, PartialEq, Eq, Clone, Copy)]
enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl From<LogLevel> for LevelFilter {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Trace => LevelFilter::TRACE,
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Warn => LevelFilter::WARN,
            LogLevel::Error => LevelFilter::ERROR,
        }
    }
}

#[derive(ValueEnum, Debug, PartialEq, Eq, Clone, Copy)]
enum OutputFileExt {
    Json,
    Yaml,
}

impl From<OutputFileExt> for InfraOutputFileExt {
    fn from(value: OutputFileExt) -> Self {
        match value {
            OutputFileExt::Json => InfraOutputFileExt::Json,
            OutputFileExt::Yaml => InfraOutputFileExt::Yaml,
        }
    }
}
