use async_std::fs::File;
use async_std::fs::OpenOptions;
use futures::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
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
                .create(false)
                .read(true)
                .write(true)
                .open(path).await
                .ok(),
            _ => None
        };
        CacheManager { cache_file }
    }

    pub async fn bunch_read<S: Deserializable>(&mut self) -> Vec<S> {
        let mut bytes = match &mut self.cache_file {
            Some(file) => {
                if let Ok(_) = file.seek(SeekFrom::Start(0)).await {
                    let mut bytes: Vec<u8> = vec![];
                    let _ = file.read_to_end(&mut bytes).await;
                    bytes
                } else { vec![] }
            }
            _ => vec![]
        };
        let mut res = vec![];
        while let Ok(obj) = deserialize_from_bytes::<S>(&mut bytes) {
            res.push(obj)
        }
        res
    }

    async fn save<S: Serializable>(&mut self, datum: S) -> S {
        match &mut self.cache_file {
            Some(file) => {
                let bytes = datum.serialize();
                let _ = file.write_all(
                    &(bytes.len() as u16).to_be_bytes()
                        .to_vec()
                        .into_iter()
                        .chain(bytes.into_iter())
                        .collect::<Vec<_>>()
                ).await;
            }
            _ => ()
        }
        datum
    }

    pub async fn bunch_save<S: Serializable>(&mut self, data: Vec<S>) -> Vec<S> {
        if let Some(file) = &mut self.cache_file {
            file.seek(SeekFrom::Start(0)).await.expect("Unable to start from beginning");
        }
        let mut res = vec![];
        for datum in data {
            res.push(self.save(datum).await);
        }
        res
    }
}

#[cfg(test)]
mod cache_manager_test {
    use std::fs::File;

    use futures::executor::block_on;

    use crate::utils::cache::CacheManager;

    const CACHE_FILE_PATH: &'static str = "/tmp/cacheManagerTestCacheFile";

    fn construct_manager() -> CacheManager {
        File::create(CACHE_FILE_PATH).expect("Unable to create file");
        std::env::set_var(CacheManager::PATH_KEY, CACHE_FILE_PATH);
        block_on(CacheManager::new())
    }

    fn remove_cache_file() {
        std::fs::remove_file(CACHE_FILE_PATH).expect("Unable to remove test file");
    }

    #[test]
    fn test_bunch_save() {
        let expected: Vec<String> = vec!["Hello".into(), "World".into()];
        let mut manager = construct_manager();
        let res = block_on(manager.bunch_save(expected.clone()));
        remove_cache_file();
        assert_eq!(res, expected);
    }

    #[test]
    fn test_bunch_read_after_save() {
        let expected: Vec<String> = vec!["Hello".into(), "World".into()];
        let mut manager = construct_manager();
        block_on(manager.bunch_save(expected.clone()));
        let read_values: Vec<String> = block_on(manager.bunch_read());
        remove_cache_file();
        assert_eq!(read_values, expected);
    }

    #[test]
    fn test_multiple_save() {
        let expected: Vec<String> = vec!["Hello".into(), "World".into()];
        let mut manager = construct_manager();
        for _ in 0..5 {
            block_on(manager.bunch_save(expected.clone()));
        }
        let read_values: Vec<String> = block_on(manager.bunch_read());
        remove_cache_file();
        assert_eq!(read_values, expected);
    }
}