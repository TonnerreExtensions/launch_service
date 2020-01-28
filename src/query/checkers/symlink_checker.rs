use async_std::path::Path;
use futures::executor::block_on;

use crate::query::checkers::Checker;

pub struct SymlinkChecker;

impl Checker for SymlinkChecker {
    fn is_legit(&self, path: &Path) -> bool {
        let path: &std::path::Path = path.into();
        path.symlink_metadata()
            .map(|metadata| metadata.file_type())
            .map(|file_type| file_type.is_symlink())
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod symlink_test {
    use async_std::path::PathBuf;

    use crate::query::checkers::Checker;
    use crate::query::checkers::symlink_checker::SymlinkChecker;

    const SYMLINK_PATH: &'static str = "/System/Library/PreferencePanes/PrintAndFax.prefPane";
    const APP_PATH: &'static str = "/System/Applications/Books.app";

    #[test]
    fn test_is_legit() {
        let path = PathBuf::from(SYMLINK_PATH);
        let checker = SymlinkChecker {};
        assert!(checker.is_legit(&path))
    }

    #[test]
    fn test_is_not_legit() {
        let path = PathBuf::from(APP_PATH);
        let checker = SymlinkChecker {};
        assert_eq!(checker.is_legit(&path), false);
    }
}