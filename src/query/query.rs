use std::fs::read_dir;
use std::path::PathBuf;
use crate::query::checkers::Checker;

pub struct QueryProcessor<B: Checker, H: Checker> {
    bundle_checker: B,
    hidden_checker: H
}

impl <B: Checker, H: Checker> QueryProcessor<B, H> {
    pub fn new() -> Self {
        QueryProcessor {
            bundle_checker: B::new(),
            hidden_checker: H::new()
        }
    }

    pub fn query(&self, req: String) -> String {
        unimplemented!()
    }

    fn walk_dir(&self, entry: PathBuf) -> Vec<PathBuf> {
        match (read_dir(&entry),
               self.bundle_checker.is_legit(&entry),
               self.hidden_checker.is_legit(&entry)
        ) {
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
    use crate::query::query::QueryProcessor;
    use std::path::PathBuf;
    use crate::query::checkers::{BundleChecker, HiddenChecker};

    type QP = QueryProcessor<BundleChecker, HiddenChecker>;
    const APP_PATH: &str = "/System/Applications/Books.app";
    const APP_FOLDER_PATH: &str= "/System/Applications";

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