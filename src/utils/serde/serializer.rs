use std::path::PathBuf;

pub trait Serializable {
    fn serialize(&self) -> Vec<u8>;
}

/// Serialize object to [size;bytes] format
pub fn serialize<S: Serializable>(obj: S) -> Vec<u8> {
    let bytes = obj.serialize();
    let size = bytes.len();
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