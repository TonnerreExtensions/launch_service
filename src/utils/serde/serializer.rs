use serde::Serialize;
use serde_json::Result;

/// Serialize object to [size;bytes] format
pub fn serialize_to_string<S: Serialize>(obj: &S) -> Result<String> {
    serde_json::to_string(obj)
}
