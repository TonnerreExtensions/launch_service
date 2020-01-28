pub use error::DeserializableError;

mod error {
    use std::fmt::Formatter;
    use std::string::FromUtf8Error;

    #[derive(Debug)]
    pub enum DeserializableError {
        EmptyBytes,
        UnableToDeserialize(FromUtf8Error),
    }

    impl From<FromUtf8Error> for DeserializableError {
        fn from(err: FromUtf8Error) -> Self {
            DeserializableError::UnableToDeserialize(err)
        }
    }

    impl std::fmt::Display for DeserializableError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "Failed to deserialize")
        }
    }

    impl std::error::Error for DeserializableError {}
}

pub trait Deserializable {
    fn deserialize(bytes: Vec<u8>) -> Result<Self, DeserializableError>
        where Self: std::marker::Sized;
}

pub fn deserialize_from_bytes<D: Deserializable>(bytes: &mut Vec<u8>) -> Result<D, DeserializableError> {
    if bytes.is_empty() {
        Err(DeserializableError::EmptyBytes)
    } else {
        let size = read_len(bytes) as usize;
        D::deserialize(bytes.drain(..size).collect())
    }
}

fn read_len(bytes: &mut Vec<u8>) -> u16 {
    let u16_size = std::mem::size_of::<u16>();
    let mut usize_bytes = [0_u8; 2];
    for (index, val) in bytes.drain(..u16_size).enumerate() {
        usize_bytes[index] = val;
    }
    u16::from_be_bytes(usize_bytes)
}

#[cfg(test)]
mod deserializer_test {
    use crate::utils::serde::deserializer::{Deserializable, DeserializableError};

    impl Deserializable for String {
        fn deserialize(bytes: Vec<u8>) -> Result<Self, DeserializableError> where Self: std::marker::Sized {
            Ok(String::from_utf8(bytes)?)
        }
    }

    #[test]
    fn test_deserialize() {
        let bytes: Vec<u8> = vec![72, 101, 108, 108, 111];
        let res = String::deserialize(bytes).unwrap();
        assert_eq!(res, "Hello")
    }
}