use super::super::serialize_safely::SerializeSafely;
use crate::util::tracing::ExpectLog;

#[derive(Debug)]
pub(in super::super) struct Yaml;

impl SerializeSafely for Yaml {
    fn serialize_safely(value: &serde_json::Value) -> String {
        serde_yaml::to_string(value).expect_log("failed to serialize with yaml fmt")
    }
}
