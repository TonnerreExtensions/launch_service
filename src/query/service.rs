use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::string::FromUtf8Error;

use crate::query::matcher;
use crate::utils::serde::deserializer::Deserializable;
use crate::utils::serde::serializer::{Serializable, serialize_to_bytes};

lazy_static! {
    pub static ref NAME_MAP: HashMap<&'static str, &'static str> = [("And", "&")]
        .iter().cloned().collect();
}

pub struct Service {
    path: PathBuf
}

impl Service {
    pub fn new(path: PathBuf) -> Self {
        Service { path }
    }

    fn map_term(name: &str) -> &str {
        NAME_MAP.get(name).unwrap_or(&name)
    }
}

impl Serializable for Service {
    fn serialize(&self) -> Vec<u8> {
        let name = self.path.file_stem()
            .and_then(OsStr::to_str)
            .map(matcher::tokenize)
            .unwrap_or_default()
            .into_iter()
            .map(Self::map_term)
            .collect::<Vec<&str>>().join(" ");
        let name = serialize_to_bytes(&name[..]);
        let content = serialize_to_bytes(
            self.path.to_str().expect("Path cannot be stringified")
        );
        name.into_iter()
            .chain(content.clone().into_iter())
            .chain(content.into_iter())
            .collect()
    }
}

impl Serializable for &str {
    fn serialize(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl Deserializable for Service {
    fn deserialize(bytes: Vec<u8>) -> Result<Self, FromUtf8Error> where Self: std::marker::Sized {
        unimplemented!()
    }
}

#[cfg(test)]
mod service_serde_test {
    use std::path::PathBuf;

    use crate::query::service::Service;
    use crate::utils::serde::serializer::serialize_to_bytes;

    #[test]
    fn test_map_term() {
        assert_eq!(Service::map_term("And"), "&");
    }

    #[test]
    fn test_serialize() {
        let path = "/System/Applications/Books.app";
        let service = Service::new(PathBuf::from(path));
        let bytes = serialize_to_bytes(service);

        let name = serialize_to_bytes("Books");
        let content = serialize_to_bytes(path);
        let chained: Vec<u8> = name.into_iter()
            .chain(content.clone().into_iter())
            .chain(content.into_iter())
            .collect();
        let expected: Vec<u8> = chained.len().to_be_bytes()
            .to_vec().into_iter()
            .chain(chained.into_iter())
            .collect();

        assert_eq!(bytes, expected);
    }
}