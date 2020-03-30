use serde::Serialize;
use serde_json::Result;

/// Serialize object to [size;bytes] format
pub fn serialize_to_bytes<S: Serialize>(obj: &S) -> Result<Vec<u8>> {
    serde_json::to_vec(obj)
}
