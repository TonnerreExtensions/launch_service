use std::path::PathBuf;
use std::string::FromUtf8Error;

use crate::utils::serde::BinaryStyle;

pub trait Deserializable {
    fn deserialize(mut bytes: Vec<u8>) -> Result<Self, FromUtf8Error>
        where Self: std::marker::Sized;
}

/// TODO: Supports reference
pub fn deserialize_from_bytes<D: Deserializable>(bytes: &mut Vec<u8>) -> Result<D, FromUtf8Error> {
    let size = read_len(bytes);
    let style: BinaryStyle = bytes.remove(0).into();
    match style {
        BinaryStyle::Value => D::deserialize(bytes.drain(..size as usize).collect()),
        BinaryStyle::Reference => unimplemented!("Enable a storage with index that allows pick up")
    }
}

/// Read a usize from the vec
fn read_len(bytes: &mut Vec<u8>) -> u32 {
    let len_size = 4;
    assert!(bytes.len() >= len_size, "Value bytes must have size 8");

    let mut res = [0_u8; 4];

    for (index, byte) in bytes.drain(..len_size).into_iter().enumerate() {
        res[index] = byte;
    }
    u32::from_be_bytes(res)
}

#[cfg(test)]
mod deserializer_test {
    use std::string::FromUtf8Error;

    use crate::utils::serde::BinaryStyle;
    use crate::utils::serde::deserializer::{Deserializable, deserialize_from_bytes, read_len};

    impl Deserializable for String {
        fn deserialize(mut bytes: Vec<u8>) -> Result<Self, FromUtf8Error> where Self: std::marker::Sized {
            String::from_utf8(bytes)
        }
    }

    #[test]
    fn test_read_usize() {
        let origin: u32 = 11;
        let mut bytes: Vec<u8> = origin.to_be_bytes().to_vec()
            .into_iter()
            .chain(vec![0_u8, 1_u8, 20_u8].into_iter())
            .collect();
        assert_eq!(read_len(&mut bytes), origin);
    }

    #[test]
    fn test_deserialize() {
        let mut bytes: Vec<u8> = vec![0, 0, 0, 5, 0, 72, 101, 108, 108, 111];
        let res: String = deserialize_from_bytes(&mut bytes).unwrap();
        assert_eq!(res, "Hello")
    }
}