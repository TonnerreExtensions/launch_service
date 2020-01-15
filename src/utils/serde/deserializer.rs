use std::path::PathBuf;
use std::string::FromUtf8Error;

pub trait Deserializable {
    fn deserialize(mut bytes: Vec<u8>) -> Result<Self, FromUtf8Error>
        where Self: std::marker::Sized;
}

/// TODO: Supports reference
pub fn deserialize_from_bytes<D: Deserializable>(bytes: &mut Vec<u8>) -> Result<D, FromUtf8Error> {
    let size = read_len(bytes);
    D::deserialize(bytes.drain(..size).collect())
}

/// Read a usize from the vec
fn read_len(bytes: &mut Vec<u8>) -> usize {
    let len_size = std::mem::size_of::<usize>();
    assert!(bytes.len() >= len_size, "Value bytes must have size 4");
    let mut res = [0_u8; 8];
    for (index, byte) in bytes.drain(..len_size)
        .into_iter().enumerate() {
        res[index] = byte;
    }
    usize::from_be_bytes(res)
}

#[cfg(test)]
mod deserializer_test {
    use std::string::FromUtf8Error;

    use crate::utils::serde::deserializer::{Deserializable, deserialize_from_bytes, read_len};

    impl Deserializable for String {
        fn deserialize(mut bytes: Vec<u8>) -> Result<Self, FromUtf8Error> where Self: std::marker::Sized {
            String::from_utf8(bytes)
        }
    }

    #[test]
    fn test_read_usize() {
        let origin: usize = 11;
        let mut bytes: Vec<u8> = origin.to_be_bytes().to_vec()
            .into_iter()
            .chain(vec![0_u8, 1_u8, 20_u8].into_iter())
            .collect();
        assert_eq!(read_len(&mut bytes), origin);
    }

    #[test]
    fn test_deserialize() {
        let mut bytes: Vec<u8> = vec![0, 0, 0, 0, 0, 0, 0, 5, 72, 101, 108, 108, 111];
        let res: String = deserialize_from_bytes(&mut bytes).unwrap();
        assert_eq!(res, "Hello")
    }
}