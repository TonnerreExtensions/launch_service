use std::collections::HashSet;
use std::ffi::OsString;
use std::path::Path;
use crate::query::checkers::checker::Checker;

pub struct BundleChecker {
    bundle_extensions: HashSet<OsString>
}

impl Checker for BundleChecker {
    fn new() -> Self {
        BundleChecker {
            bundle_extensions: vec!["app", "prefPane"].into_iter().map(OsString::from).collect()
        }
    }

    fn is_legit(&self, path: &Path) -> bool {
        if let Some(extension) = path.extension() {
            self.bundle_extensions.contains(extension)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod bundle_checker_test {
    use crate::query::checkers::BundleChecker;
    use crate::query::checkers::checker::Checker;
    use std::path::Path;

    #[test]
    fn test_is_bundle_app() {
        assert!(BundleChecker::new().is_legit(Path::new("/System/Applications/Safari.app")));
    }

    #[test]
    fn test_is_bundle_pref() {
        assert!(BundleChecker::new().is_legit(Path::new("/System/Library/PreferencePanes/Network.prefPane")));
    }

    #[test]
    fn test_is_bundle_folder() {
        assert_eq!(BundleChecker::new().is_legit(Path::new("/Applications")), false);
    }

    #[test]
    fn test_is_bundle_file() {
        assert_eq!(BundleChecker::new().is_legit(Path::new("/dev/null")), false);
    }
}
