use std::path::PathBuf;

pub trait Desrializable {
    fn deserialize(bytes: Vec<u8>) -> Option<Self>
        where Self: std::marker::Sized;
}

pub fn desrialize<D: Desrializable>(bytes: &mut Vec<u8>) -> Option<D> {
    let size = usize::from_be_bytes(vec_to_bytes(bytes.drain(..8).collect()));
    D::deserialize(
        bytes.drain(..size).collect()
    )
}

/// Convert size 8 vec to size 8 array
fn vec_to_bytes(bytes: Vec<u8>) -> [u8; 8] {
    assert_eq!(bytes.len(), 8, "Value bytes must have size 8");
    let mut res = [0_u8; 8];
    for (index, byte) in bytes.into_iter().enumerate() {
        res[index] = byte;
    }
    res
}

impl Desrializable for PathBuf {
    fn deserialize(bytes: Vec<u8>) -> Option<PathBuf> {
        unimplemented!()
    }
}