use tracing::level_filters::LevelFilter;

use crate::infrastructure::output_to_file::OutputFileExt;

use super::{
    cli_settings::CliSettings, file_settings::FileSettings, ApiKey, PathBufValidUtf8,
};

type Cli<'a> = &'a CliSettings;
type File<'a> = &'a Option<FileSettings>;

/// 設定を保持
#[derive(Debug, Clone)]
pub struct Settings {
    api_key: ApiKey,
    stdout_log_level: Option<LevelFilter>,
    file_log_level: Option<LevelFilter>,
    output_path_without_ext: PathBufValidUtf8,
    output_file_ext: OutputFileExt,
}

impl Settings {
    /// 設定をcli, file(ops) から読み込み
    ///
    /// 優先順位は `cli` > `file`
    pub fn load() -> Self {
        let cli = CliSettings::parse_from_cmd_args();
        let file = load_file_settings(&cli);

        let stdout_log_level = stdout_log(&cli, &file);
        let file_log_level = file_log(&cli, &file);
        let output_path_without_ext = output_path_without_ext(&cli, &file);
        let output_file_ext = output_file_ext(&cli, &file);
        let api_key = api_key(&cli, &file);
        Self {
            api_key,
            stdout_log_level,
            file_log_level,
            output_path_without_ext,
            output_file_ext,
        }
    }

    pub fn get_api_key(&self) -> ApiKey {
        self.api_key.clone()
    }
    pub fn get_stdout_log_level(&self) -> Option<LevelFilter> {
        self.stdout_log_level
    }
    pub fn get_file_log_level(&self) -> Option<LevelFilter> {
        self.file_log_level
    }
    pub fn get_output_path_without_ext(&self) -> PathBufValidUtf8 {
        self.output_path_without_ext.clone()
    }
    pub fn get_output_file_ext(&self) -> OutputFileExt {
        self.output_file_ext
    }
}

fn load_file_settings(cli: Cli) -> Option<FileSettings> {
    if cli.use_settings_path() {
        match FileSettings::load(&cli.settings_path()) {
            Ok(settings) => Some(settings),
            Err(e) => {
                println!("{}, so ignore settings file", e);
                None
            }
        }
    } else {
        None
    }
}

// ! 優先順位が cmd args > file になるように気を付ける

fn stdout_log(cli: Cli, file: File) -> Option<LevelFilter> {
    cli.stdout_log_level()
        .or_else(|| file.as_ref().and_then(|f| f.stdout_log_level()))
}

fn file_log(cli: Cli, file: File) -> Option<LevelFilter> {
    cli.file_log_level().or_else(|| file.as_ref().and_then(|f| f.file_log_level()))
}

fn output_path_without_ext(cli: Cli, file: File) -> PathBufValidUtf8 {
    cli.output_file()
        .or_else(|| file.as_ref().and_then(|f| f.output_path_without_ext()))
        .unwrap_or_else(|| PathBufValidUtf8::prompt(false))
}

fn output_file_ext(cli: Cli, file: File) -> OutputFileExt {
    cli.output_file_ext()
        .or_else(|| file.as_ref().and_then(|f| f.output_file_ext()))
        .unwrap_or_default()
}

fn api_key(cli: Cli, file: File) -> ApiKey {
    if cli.input_api_key() {
        return ApiKey::prompt();
    }
    if let Some(key) = cli.youtube_data_api_key() {
        return key;
    }
    file.as_ref()
        .and_then(|f| f.youtube_data_api_key())
        .unwrap_or_else(ApiKey::prompt)
}
