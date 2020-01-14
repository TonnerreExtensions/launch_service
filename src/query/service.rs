use std::path::PathBuf;
use std::string::FromUtf8Error;

use crate::utils::serde::BinaryStyle;
use crate::utils::serde::deserializer::Deserializable;
use crate::utils::serde::serializer::Serializable;

pub struct Service {
    path: PathBuf
}

impl Service {
    pub fn new(path: PathBuf) -> Self {
        Service { path }
    }
}

impl Serializable for Service {
    fn serialize(&self) -> Vec<u8> {
        unimplemented!()
    }

    fn identify(&self) -> BinaryStyle {
        BinaryStyle::Value
    }
}

impl Deserializable for Service {
    fn deserialize(bytes: Vec<u8>) -> Result<Self, FromUtf8Error> where Self: std::marker::Sized {
        unimplemented!()
    }
}
