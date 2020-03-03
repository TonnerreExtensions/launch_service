use std::collections::VecDeque;

use async_std::fs::read_dir;
use async_std::path::{Path, PathBuf};
use futures::executor::block_on;
use futures::future::join_all;
use futures::join;
use futures::StreamExt;

use crate::configurator::Configs;
use crate::query::checker::{Checker, Outcome};
use crate::query::matcher;
use crate::query::service::Service;
use crate::utils::cache::CacheManager;
use crate::utils::serde::serializer;

pub struct QueryProcessor {
    config: Configs,
    checker: Checker,
}

impl QueryProcessor {
    const CONFIG_PATH: &'static str = "settings.yaml";

    /// New query processor
    pub fn new() -> Self {
        let config =
            Configs::from(Path::new(Self::CONFIG_PATH).into()).expect("settings.yaml is missing");
        let ignored_paths = config.get_ignore_paths();
        QueryProcessor {
            config,
            checker: Checker::new(ignored_paths),
        }
    }

    /// Query based on the request, and return serialized bytes of the services
    pub fn query(&self, req: &str) -> Vec<u8> {
        block_on(self.async_query(req))
    }

    /// Async query
    async fn async_query(&self, req: &str) -> Vec<u8> {
        let (cached_services, updated_services) = join!(
            self.query_cached_services(&req),
            self.query_updated_services(&req)
        );
        cached_services
            .into_iter()
            .chain(updated_services.into_iter())
            .collect()
    }

    /// Cached services are either loaded from cache or generated by walking through directories
    async fn query_cached_services(&self, req: &str) -> Vec<u8> {
        let cache_manager = CacheManager::new().await;
        match Some(cache_manager.bunch_read().await) {
            Some(cache) if !cache.is_empty() => cache,
            _ => {
                cache_manager
                    .bunch_save(
                        join_all(
                            self.config
                                .get_internal_cached()
                                .into_iter()
                                .map(|path| self.walk_paths(path)),
                        )
                        .await
                        .into_iter()
                        .flatten()
                        .map(Service::new)
                        .collect(),
                    )
                    .await
            }
        }
        .into_iter()
        .filter(|service| service.path.to_str().is_some())
        .filter(|service| matcher::match_query(&req, service.path.to_str().unwrap()))
        .flat_map(serializer::serialize_to_bytes)
        .collect()
    }

    async fn query_updated_services(&self, req: &str) -> Vec<u8> {
        join_all(
            self.config
                .get_internal_updated()
                .into_iter()
                .map(|path| self.walk_paths(path)),
        )
        .await
        .into_iter()
        .flatten()
        .filter(|path| path.to_str().is_some())
        .filter(|path| matcher::match_query(&req, path.to_str().unwrap()))
        .map(Service::new)
        .flat_map(serializer::serialize_to_bytes)
        .collect()
    }

    /// Recursively iterate through files and folders, and return all legit file paths
    async fn walk_paths(&self, entry: PathBuf) -> Vec<PathBuf> {
        match self.checker.check(&entry) {
            Outcome::UnwantedPath => vec![],
            Outcome::BundlePath => vec![entry],
            Outcome::NormalPath => {
                let (mut res, mut remaining) = self.separate_files_and_dirs(entry).await;
                while let Some(entry) = remaining.pop_front() {
                    let (processed, unprocessed) = self.separate_files_and_dirs(entry).await;
                    res.extend(processed);
                    remaining.extend(unprocessed);
                }
                res
            }
        }
    }

    /// walk through all files in the given entry, and return paths for files and directories
    async fn separate_files_and_dirs(&self, entry: PathBuf) -> (Vec<PathBuf>, VecDeque<PathBuf>) {
        let mut processed = Vec::new();
        let mut folders = VecDeque::new();
        let mut read_folder = read_dir(&entry).await.expect("Unwrap folder");
        while let Some(Ok(path)) = read_folder.next().await {
            let path = path.path();
            match self.checker.check(&path) {
                Outcome::UnwantedPath => continue,
                Outcome::BundlePath => processed.push(path),
                Outcome::NormalPath => folders.push_back(path),
            }
        }
        (processed, folders)
    }
}

#[cfg(test)]
mod query_test {
    use async_std::path::PathBuf;
    use futures::executor::block_on;

    use crate::query::query::QueryProcessor;

    type QP = QueryProcessor;

    const APP_PATH: &str = "/System/Applications/Books.app";
    const APP_FOLDER_PATH: &str = "/System/Applications";

    #[test]
    fn test_walk_dir_single() {
        let processor = QP::new();
        let single_file = PathBuf::from(APP_PATH);
        let expected = PathBuf::from(APP_PATH);
        let res = block_on(processor.walk_paths(single_file));
        assert_eq!(&expected, &res[0]);
    }

    #[test]
    fn test_walk_dir_inside_book() {
        let processor = QP::new();
        let content = PathBuf::from(APP_FOLDER_PATH);
        let res = block_on(processor.walk_paths(content));
        assert_eq!(52, res.len());
    }
}
