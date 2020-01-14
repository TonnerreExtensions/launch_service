use std::collections::HashSet;
use std::path::{Path, PathBuf};

use crate::query::checkers::Checker;

/// Checks if the path should be ignored
pub struct IgnoreChecker {
    paths: HashSet<PathBuf>
}

impl IgnoreChecker {
    pub fn new(paths: HashSet<PathBuf>) -> Self {
        IgnoreChecker { paths }
    }
}

impl Checker for IgnoreChecker {
    fn is_legit(&self, path: &Path) -> bool {
        self.paths.contains(path)
    }
}

#[cfg(test)]
mod ignore_checker_test {
    use std::collections::HashSet;
    use std::path::{Path, PathBuf};

    use crate::query::checkers::{Checker, IgnoreChecker};

    #[test]
    fn test_is_legit() {
        let ignored_paths: HashSet<PathBuf> = vec!["/Users/cheng", "/usr/bin"]
            .into_iter()
            .map(PathBuf::from)
            .collect();
        let checker = IgnoreChecker::new(ignored_paths);
        assert!(checker.is_legit(Path::new("/Users/cheng")))
    }

    #[test]
    fn test_is_not_legit() {
        let ignored_paths: HashSet<PathBuf> = vec!["/Users/cheng", "/usr/bin"]
            .into_iter()
            .map(PathBuf::from)
            .collect();
        let checker = IgnoreChecker::new(ignored_paths);
        assert_eq!(checker.is_legit(Path::new("/Users/cheng/Applications")), false)
    }
}