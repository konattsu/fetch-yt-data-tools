use super::super::serialize_safely::SerializeSafely;
use crate::util::tracing::ExpectLog;

#[derive(Debug)]
pub(in super::super) struct Json;

impl SerializeSafely for Json {
    fn serialize_safely(value: &serde_json::Value) -> String {
        serde_json::to_string_pretty(value)
            .expect_log("failed to serialize with json fmt")
    }
}
