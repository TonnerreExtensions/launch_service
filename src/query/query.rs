use std::fs::read_dir;
use std::path::{Path, PathBuf};

use crate::configurator::Configs;
use crate::query::checkers::{BundleChecker, Checker, HiddenChecker, IgnoreChecker};
use crate::query::matcher;
use crate::query::service::Service;
use crate::utils::cache::CacheManager;
use crate::utils::serde::serializer;
use crate::utils::serde::serializer::serialize_to_bytes;

pub struct QueryProcessor {
    config: Configs,
    condition_checker: Box<dyn Checker>,
    terminate_checkers: Vec<Box<dyn Checker>>,
}

impl QueryProcessor {
    const CONFIG_PATH: &'static str = "settings.yaml";

    pub fn new() -> Self {
        let config = Configs::from(Path::new(Self::CONFIG_PATH))
            .expect("settings.yaml is missing");
        let ignore_paths = config.get_ignore_paths();
        QueryProcessor {
            config,
            condition_checker: Box::new(BundleChecker::new()),
            terminate_checkers: if ignore_paths.is_empty() {
                vec![Box::new(HiddenChecker::new())]
            } else {
                vec![
                    Box::new(HiddenChecker::new()),
                    Box::new(IgnoreChecker::new(ignore_paths))
                ]
            },
        }
    }

    pub fn query(&self, req: String) -> Vec<u8> {
        let cache_manager = CacheManager::new();
        let cache_loaded: Vec<Service> = cache_manager.bunch_read();
        let cached_services = if cache_loaded.is_empty() {
            cache_manager.bunch_save(
                self.config.get_internal_cached()
                    .into_iter()
                    .flat_map(|path| self.walk_dir(path))
                    .filter(|path| path.to_str().is_some())
                    .filter(|path| matcher::match_query(&req, path.to_str().unwrap()))
                    .map(Service::new)
                    .collect()
            )
        } else {
            cache_loaded
        };

        let updated_services: Vec<Service> = self.config.get_internal_updated()
            .into_iter()
            .flat_map(|path| self.walk_dir(path))
            .filter(|path| path.to_str().is_some())
            .filter(|path| matcher::match_query(&req, path.to_str().unwrap()))
            .map(Service::new)
            .collect();

        cached_services.into_iter()
            .chain(updated_services.into_iter())
            .flat_map(serializer::serialize_to_bytes)
            .collect()
    }

    fn walk_dir(&self, entry: PathBuf) -> Vec<PathBuf> {
        let terminate_condition = self.terminate_checkers.iter()
            .any(|checker| checker.is_legit(&entry));
        let eligible_condition = self.condition_checker.is_legit(&entry);
        match (read_dir(&entry), eligible_condition, terminate_condition) {
            (Ok(files), false, false) => files.filter_map(Result::ok)
                .map(|entry| entry.path())
                .flat_map(|entry| self.walk_dir(entry))
                .collect(),
            (_, true, false) => vec![entry],
            _ => vec![]
        }
    }
}


#[cfg(test)]
mod query_test {
    use std::path::PathBuf;

    use crate::query::checkers::{BundleChecker, HiddenChecker};
    use crate::query::query::QueryProcessor;

    type QP = QueryProcessor;

    const APP_PATH: &str = "/System/Applications/Books.app";
    const APP_FOLDER_PATH: &str = "/System/Applications";

    #[test]
    fn test_walk_dir_single() {
        let processor = QP::new();
        let single_file = PathBuf::from(APP_PATH);
        let expected = PathBuf::from(APP_PATH);
        let res = processor.walk_dir(single_file);
        assert_eq!(&expected, &res[0]);
    }

    #[test]
    fn test_walk_dir_inside_book() {
        let processor = QP::new();
        let content = PathBuf::from(APP_FOLDER_PATH);
        let res = processor.walk_dir(content);
        assert_eq!(52, res.len());
    }
}