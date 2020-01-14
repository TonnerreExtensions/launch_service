use std::path::PathBuf;
use std::string::FromUtf8Error;

pub trait Cacheable {
    fn serialize(&self) -> &[u8];
    fn deserialize(bytes: Vec<u8>) -> Result<Self, FromUtf8Error>
        where Self: std::marker::Sized;
}

impl Cacheable for PathBuf {
    fn serialize(&self) -> &[u8] {
        self.to_str().map(str::as_bytes).unwrap_or_default()
    }

    fn deserialize(bytes: Vec<u8>) -> Result<Self, FromUtf8Error> {
        Ok(PathBuf::from(String::from_utf8(bytes)?))
    }
}
