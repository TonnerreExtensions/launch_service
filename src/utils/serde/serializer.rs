use std::path::PathBuf;

pub trait Serializable {
    fn serialize(&self) -> Vec<u8>;
}

pub fn serialize<S: Serializable>(obj: S) -> Vec<u8> {
    obj.serialize()
}

impl Serializable for PathBuf {
    fn serialize(&self) -> Vec<u8> {
        /// TODO: serialize pathbuf
        unimplemented!()
    }
}