use crate::query::checkers::checker::Checker;
use std::path::Path;
use std::ffi::OsStr;

/// Checker that checks if a path is hidden by checking its prefix dot
pub struct HiddenChecker;

impl Checker for HiddenChecker {
    fn new() -> Self {
        HiddenChecker {}
    }

    fn is_legit(&self, path: &Path) -> bool {
        path.to_str().map(str::len).unwrap_or(0) <= 1 ||
            path.file_stem()
                .and_then(OsStr::to_str)
                .and_then(|name| Some(name.starts_with(".")))
                .unwrap_or(false)
    }
}

#[cfg(test)]
mod hidden_checker_test {
    use crate::query::checkers::hidden_checker::HiddenChecker;
    use crate::query::checkers::checker::Checker;
    use std::path::Path;

    #[test]
    fn test_is_hidden() {
        assert!(HiddenChecker::new().is_legit(Path::new(".test")));
    }

    #[test]
    fn test_is_not_hidden() {
        assert_eq!(HiddenChecker::new().is_legit(Path::new("test/test")), false);
    }
}