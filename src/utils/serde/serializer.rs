use std::path::PathBuf;

pub trait Serializable {
    fn serialize(&self) -> Vec<u8>;
}

/// Serialize object to [size;type;bytes] format
pub fn serialize_to_bytes<S: Serializable>(obj: S) -> Vec<u8> {
    let mut bytes = obj.serialize();
    bytes.len().to_be_bytes()
        .to_vec()
        .into_iter()
        .chain(bytes.into_iter())
        .collect()
}

#[cfg(test)]
mod serialize_test {
    use crate::utils::serde::serializer::{Serializable, serialize_to_bytes};

    impl Serializable for String {
        fn serialize(&self) -> Vec<u8> {
            self.as_bytes().to_vec()
        }
    }

    #[test]
    fn test_serialize_string() {
        let test = "Hello".to_owned();
        let serialized = serialize_to_bytes(test);
        assert_eq!(serialized, vec![0, 0, 0, 0, 0, 0, 0, 5, 72, 101, 108, 108, 111]);
    }
}