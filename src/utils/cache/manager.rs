use async_std::fs::File;
use async_std::fs::OpenOptions;
use async_std::prelude::*;
use futures::io::SeekFrom;

use crate::utils::serde::deserializer::{Deserializable, deserialize_from_bytes};
use crate::utils::serde::serializer::Serializable;

pub struct CacheManager {
    cache_file: Option<File>
}

impl CacheManager {
    const PATH_KEY: &'static str = "CACHE_PATH";

    pub async fn new() -> Self {
        let path = std::env::var(Self::PATH_KEY);
        let cache_file = match path {
            Ok(path) if !path.is_empty() => OpenOptions::new()
                .create(true)
                .read(true)
                .append(true)
                .open(path).await
                .ok(),
            _ => None
        };
        CacheManager { cache_file }
    }

    pub async fn bunch_read<S: Deserializable>(&mut self) -> Vec<S> {
        let mut bytes = match &mut self.cache_file {
            Some(file) => {
                file.seek(SeekFrom::Start(0)).await;
                let mut bytes: Vec<u8> = vec![];
                file.read_to_end(&mut bytes).await;
                bytes
            }
            _ => vec![]
        };
        let mut res = vec![];
        while let Ok(obj) = deserialize_from_bytes::<S>(&mut bytes) {
            res.push(obj)
        }
        res
    }

    async fn save<S: Serializable>(&mut self, data: S) -> S {
        match &mut self.cache_file {
            Some(file) => {
                file.write_all(&data.serialize()).await;
            }
            _ => ()
        }
        data
    }

    pub async fn bunch_save<S: Serializable>(&mut self, data: Vec<S>) -> Vec<S> {
        if let Some(file) = &mut self.cache_file {
            file.seek(SeekFrom::Start(0)).await;
        }
        let mut res = vec![];
        for datum in data {
            res.push(self.save(datum).await);
        }
        res
    }
}
