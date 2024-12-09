use serde_json::Value;
use std::{fmt::Debug, fs, io::Write, path::PathBuf};
use thiserror::Error;
use tracing::Level;

use super::{
    ext::{json::Json, yaml::Yaml},
    output_file_ext::OutputFileExt,
    serialize_safely::SerializeSafely,
};

#[derive(Debug, Error)]
pub enum OutputError {
    #[error("failed to open file: {0}")]
    Open(#[source] std::io::Error),
    #[error("failed to create file: {0}")]
    Create(#[source] std::io::Error),
    #[error("failed to write to file: {0}")]
    Write(#[source] std::io::Error),
}

#[tracing::instrument(level = Level::TRACE)]
pub fn output_to_file(
    content: &Value,
    file_path_without_ext: PathBuf,
    fmt: Option<OutputFileExt>,
) -> Result<(), OutputError> {
    let serialized_content =
        serialize_content(content, file_path_without_ext, fmt.unwrap_or_default());
    let mut file =
        fs::File::create(serialized_content.file_path).map_err(OutputError::Create)?;
    writeln!(file, "{}", serialized_content.content).map_err(OutputError::Write)?;
    file.flush().map_err(OutputError::Write)?;
    Ok(())
}

#[derive(Debug)]
struct SerializedContent {
    content: String,
    file_path: PathBuf,
}

#[tracing::instrument(level = Level::TRACE)]
fn serialize_content(
    content: &Value,
    mut file_path_without_ext: PathBuf,
    fmt: OutputFileExt,
) -> SerializedContent {
    // `get_ext()`がセパレート文字を必ず含まないのでパニックさせる
    if !file_path_without_ext.set_extension(fmt.get_ext()) {
        panic!("failed set ext to file: {:?}", dbg!(file_path_without_ext));
    }
    // ターボフィッシュを使わないで実装したいのでこのようにしている
    let content = match fmt {
        OutputFileExt::Json => Json::serialize_safely(content),
        OutputFileExt::Yaml => Yaml::serialize_safely(content),
    };
    SerializedContent {
        content,
        file_path: file_path_without_ext,
    }
}
