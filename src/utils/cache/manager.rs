use std::io::{Error, ErrorKind};
use std::path::PathBuf;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::utils;

pub struct CacheManager {
    path: Option<PathBuf>,
}

impl CacheManager {
    const PATH_KEY: &'static str = "CACHE_PATH";

    pub fn new() -> Self {
        let path = std::env::var(Self::PATH_KEY).ok().map(PathBuf::from);
        CacheManager { path }
    }

    pub fn bunch_read<S: DeserializeOwned>(&self) -> Vec<S> {
        let bytes = (&self.path)
            .as_ref()
            .and_then(|path| std::fs::read(path).ok())
            .unwrap_or_default();
        utils::serde::deserialize_from_bytes(&bytes).unwrap_or_default()
    }

    pub fn bunch_save<S: Serialize>(&self, data: Vec<S>) -> Vec<S> {
        if self.path.is_none() {
            return data;
        }
        let bytes = utils::serde::serialize_to_bytes(&data);
        let res = match bytes {
            Ok(bytes) => std::fs::write(self.path.as_ref().unwrap(), bytes),
            Err(err) => Err(Error::new(ErrorKind::InvalidData, err)),
        };
        if let Err(err) = res {
            println!("bunch_save error: {}", err);
        }
        data
    }
}
