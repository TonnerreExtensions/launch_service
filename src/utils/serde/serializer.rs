use std::path::PathBuf;

pub trait Serializable {
    fn serialize(&self) -> Vec<u8>;
}

/// Serialize object to [size;bytes] format
pub fn serialize<S: Serializable>(obj: S) -> Vec<u8> {
    let bytes = obj.serialize();
    let size = bytes.len() as u32;
    size.to_be_bytes()
        .to_vec()
        .into_iter()
        .chain(bytes.into_iter())
        .collect()
}

impl Serializable for PathBuf {
    fn serialize(&self) -> Vec<u8> {
        /// TODO: serialize pathbuf
        unimplemented!()
    }
}

#[cfg(test)]
mod serialize_test {
    use crate::utils::serde::serializer::{Serializable, serialize};

    impl Serializable for String {
        fn serialize(&self) -> Vec<u8> {
            self.as_bytes().to_vec()
        }
    }

    #[test]
    fn test_serialize_string() {
        let test = "Hello".to_owned();
        let serialized = serialize(test);
        assert_eq!(serialized, vec![0, 0, 0, 5, 72, 101, 108, 108, 111]);
    }
}