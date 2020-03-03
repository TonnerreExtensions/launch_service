use async_std::fs::File;
use async_std::fs::OpenOptions;
use async_std::sync::Mutex;
use futures::io::SeekFrom;
use futures::stream::FuturesUnordered;
use futures::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, StreamExt};

use crate::utils::serde::deserializer::{deserialize_from_bytes, Deserializable};
use crate::utils::serde::serializer::Serializable;

pub struct CacheManager {
    cache_file: Option<Mutex<File>>,
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
                .open(path)
                .await
                .ok(),
            _ => None,
        }
        .map(Mutex::new);
        CacheManager { cache_file }
    }

    pub async fn bunch_read<S: Deserializable>(&self) -> Vec<S> {
        let mut bytes = match &self.cache_file {
            Some(file) => {
                let res = file.lock().await.seek(SeekFrom::Start(0)).await;
                if let Ok(_) = res {
                    let mut bytes: Vec<u8> = vec![];
                    let _ = file.lock().await.read_to_end(&mut bytes).await;
                    bytes
                } else {
                    vec![]
                }
            }
            _ => vec![],
        };
        let mut res = vec![];
        while let Ok(obj) = deserialize_from_bytes::<S>(&mut bytes) {
            res.push(obj)
        }
        res
    }

    async fn save<S: Serializable>(&self, datum: S) -> S {
        match &self.cache_file {
            Some(file) => {
                let bytes = datum.serialize();
                let _ = file
                    .lock()
                    .await
                    .write_all(
                        &(bytes.len() as u16)
                            .to_be_bytes()
                            .to_vec()
                            .into_iter()
                            .chain(bytes.into_iter())
                            .collect::<Vec<_>>(),
                    )
                    .await;
            }
            _ => (),
        }
        datum
    }

    pub async fn bunch_save<S: Serializable>(&self, data: Vec<S>) -> Vec<S> {
        if let Some(file) = &self.cache_file {
            file.lock()
                .await
                .seek(SeekFrom::Start(0))
                .await
                .expect("Unable to start from beginning");
        }
        let mut futures = data
            .into_iter()
            .map(|datum| self.save(datum))
            .collect::<FuturesUnordered<_>>();
        let mut res = Vec::new();
        while let Some(datum) = futures.next().await {
            res.push(datum)
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

    fn construct_manager(file_name: &str) -> CacheManager {
        let file_name = format!("{}{}", CACHE_FILE_PATH, file_name);
        File::create(&file_name).expect("Unable to create file");
        std::env::set_var(CacheManager::PATH_KEY, file_name);
        block_on(CacheManager::new())
    }

    fn remove_cache_file(file_name: &str) {
        let file_name = format!("{}{}", CACHE_FILE_PATH, file_name);
        std::fs::remove_file(file_name).expect("Unable to remove test file");
    }

    #[test]
    fn test_bunch_save() {
        let expected: Vec<String> = vec!["Hello".into(), "World".into()];
        let file_name = "bunch_save";
        let manager = construct_manager(file_name);
        let res = block_on(manager.bunch_save(expected.clone()));
        remove_cache_file(file_name);
        assert_eq!(res, expected);
    }

    #[test]
    fn test_bunch_read_after_save() {
        let expected: Vec<String> = vec!["Hello".into(), "World".into()];
        let file_name = "_bunch_read";
        let manager = construct_manager(file_name);
        block_on(manager.bunch_save(expected.clone()));
        let read_values: Vec<String> = block_on(manager.bunch_read());
        remove_cache_file(file_name);
        assert_eq!(read_values, expected);
    }

    #[test]
    fn test_multiple_save() {
        let expected: Vec<String> = vec!["Hello".into(), "World".into()];
        let file_name = "_multiple_save";
        let manager = construct_manager(file_name);
        for _ in 0..5 {
            block_on(manager.bunch_save(expected.clone()));
        }
        let read_values: Vec<String> = block_on(manager.bunch_read());
        remove_cache_file(file_name);
        assert_eq!(read_values, expected);
    }
}
