use std::ffi::OsStr;

use async_std::path::Path;

use crate::query::checkers::checker::Checker;

/// Checker that checks if a path is hidden by checking its prefix dot
pub struct HiddenChecker;

impl Checker for HiddenChecker {
    fn is_legit(&self, path: &Path) -> bool {
        path.to_str().map(str::len).unwrap_or(0) <= 1 ||
            path.file_stem()
                .and_then(OsStr::to_str)
                .map(|name| name.starts_with("."))
                .unwrap_or(false)
    }
}

#[cfg(test)]
mod hidden_checker_test {
    use async_std::path::Path;

    use crate::query::checkers::checker::Checker;
    use crate::query::checkers::hidden_checker::HiddenChecker;

    #[test]
    fn test_is_hidden() {
        assert!(HiddenChecker {}.is_legit(Path::new(".test")));
    }

    #[test]
    fn test_is_not_hidden() {
        assert_eq!(HiddenChecker {}.is_legit(Path::new("test/test")), false);
    }
}