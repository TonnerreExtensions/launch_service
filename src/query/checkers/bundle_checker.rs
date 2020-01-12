use std::collections::HashSet;
use std::ffi::OsString;
use std::path::Path;
use crate::query::checkers::checker::Checker;

lazy_static! {
    /// Extensions that can be identified as bundles
    pub static ref EXTENSIONS: HashSet<OsString> = vec!["app", "prefPane"]
                                                    .into_iter().map(OsString::from).collect();
}

/// Checker that checks if a file path is a bundle by inspecting its extensions
pub struct BundleChecker;

impl Checker for BundleChecker {

    fn new() -> Self {
        BundleChecker {}
    }

    fn is_legit(&self, path: &Path) -> bool {
        path.extension()
            .and_then(|ext| Some(EXTENSIONS.contains(ext)))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod bundle_checker_test {
    use crate::query::checkers::BundleChecker;
    use crate::query::checkers::checker::Checker;
    use std::path::Path;

    #[test]
    fn test_is_bundle_app() {
        assert!(BundleChecker::new().is_legit(Path::new("/System/Applications/Books.app")));
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
