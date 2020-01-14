use std::fs::read_dir;
use std::path::{Path, PathBuf};

use crate::configurator::Configs;
use crate::query::checkers::{BundleChecker, Checker, HiddenChecker, IgnoreChecker};

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

    pub fn query(&self, req: String) -> String {
        unimplemented!()
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