pub trait Desrializable {
    fn deserialize(bytes: Vec<u8>) -> Option<Self>
        where Self: std::marker::Sized;
}

pub fn desrialize<D: Desrializable>(bytes: Vec<u8>) -> Option<D> {
    D::deserialize(bytes)
}