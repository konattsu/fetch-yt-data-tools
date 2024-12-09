use serde_json::Value;

pub(super) trait SerializeSafely {
    fn serialize_safely(value: &Value) -> String;
}
