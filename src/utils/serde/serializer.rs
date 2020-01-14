use std::path::PathBuf;

use crate::utils::serde::BinaryStyle;

pub trait Serializable {
    fn serialize(&self) -> Vec<u8>;
    fn identify(&self) -> BinaryStyle;
}

/// Serialize object to [size;type;bytes] format
pub fn serialize_to_bytes<S: Serializable>(obj: S) -> Vec<u8> {
    let mut bytes = obj.serialize();
    let mut size = (bytes.len() as u32).to_be_bytes().to_vec();
    size.push(obj.identify().convert_to_byte());
    size.append(&mut bytes);
    size
}

#[cfg(test)]
mod serialize_test {
    use crate::utils::serde::BinaryStyle;
    use crate::utils::serde::serializer::{Serializable, serialize_to_bytes};

    impl Serializable for String {
        fn serialize(&self) -> Vec<u8> {
            self.as_bytes().to_vec()
        }
        fn identify(&self) -> BinaryStyle {
            BinaryStyle::Value
        }
    }

    #[test]
    fn test_serialize_string() {
        let test = "Hello".to_owned();
        let serialized = serialize_to_bytes(test);
        assert_eq!(serialized, vec![0, 0, 0, 5, 0, 72, 101, 108, 108, 111]);
    }
}