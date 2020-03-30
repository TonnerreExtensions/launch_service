use serde::de::DeserializeOwned;
use serde_json::Result;

pub fn deserialize_from_bytes<D: DeserializeOwned>(bytes: &Vec<u8>) -> Result<D> {
    serde_json::from_slice(bytes)
}
