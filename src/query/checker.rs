use std::collections::HashSet;
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};

pub enum Outcome {
    UnwantedPath,
    BundlePath,
    NormalPath,
}

pub struct Checker<'a> {
    ignored_paths: &'a HashSet<PathBuf>,
    bundle_extensions: HashSet<OsString>,
}

impl<'a> Checker<'a> {
    pub fn new(ignored_paths: &'a HashSet<PathBuf>) -> Self {
        let bundle_extensions: HashSet<OsString> = vec!["app", "prefPane"]
            .into_iter()
            .map(OsString::from)
            .collect();
        Checker {
            ignored_paths,
            bundle_extensions,
        }
    }

    pub fn check<P: AsRef<Path>>(&self, path: P) -> Outcome {
        if self.is_symlink(path.as_ref())
            || self.is_hidden(path.as_ref())
            || self.is_ignored_path(path.as_ref())
        {
            Outcome::UnwantedPath
        } else if self.is_bundle(path.as_ref()) {
            Outcome::BundlePath
        } else {
            Outcome::NormalPath
        }
    }

    fn is_bundle(&self, path: &Path) -> bool {
        path.extension()
            .map(|ext| self.bundle_extensions.contains(ext))
            .unwrap_or(false)
    }

    /// Checker that checks if a path is hidden by checking its prefix dot
    fn is_hidden(&self, path: &Path) -> bool {
        path.file_stem()
            .and_then(OsStr::to_str)
            .map(|name| name.starts_with("."))
            .unwrap_or(false)
    }

    fn is_symlink(&self, path: &Path) -> bool {
        let path: &std::path::Path = path.into();
        path.symlink_metadata()
            .map(|metadata| metadata.file_type())
            .map(|file_type| file_type.is_symlink())
            .unwrap_or(false)
    }

    fn is_ignored_path(&self, path: &Path) -> bool {
        self.ignored_paths.contains(path)
    }
}

#[cfg(test)]
mod bundle_checker_test {
    use std::collections::HashSet;
    use std::path::Path;

    use crate::query::checker::Checker;

    #[test]
    fn test_is_bundle_app() {
        let ignored_paths = HashSet::new();
        let checker = Checker::new(&ignored_paths);
        assert!(checker.is_bundle(Path::new("/System/Applications/Books.app")));
    }

    #[test]
    fn test_is_bundle_pref() {
        let ignored_paths = HashSet::new();
        let checker = Checker::new(&ignored_paths);
        assert!(checker.is_bundle(Path::new(
            "/System/Library/PreferencePanes/Network.prefPane"
        )));
    }

    #[test]
    fn test_is_bundle_folder() {
        let ignored_paths = HashSet::new();
        let checker = Checker::new(&ignored_paths);
        assert_eq!(checker.is_bundle(Path::new("/Applications")), false);
    }

    #[test]
    fn test_is_bundle_file() {
        let ignored_paths = HashSet::new();
        let checker = Checker::new(&ignored_paths);
        assert_eq!(checker.is_bundle(Path::new("/dev/null")), false);
    }
}

#[cfg(test)]
mod hidden_checker_test {
    use std::collections::HashSet;
    use std::path::Path;

    use crate::query::checker::Checker;

    #[test]
    fn test_is_hidden() {
        let ignored_paths = HashSet::new();
        let checker = Checker::new(&ignored_paths);
        assert!(checker.is_hidden(Path::new(".test")));
    }

    #[test]
    fn test_is_not_hidden() {
        let ignored_paths = HashSet::new();
        let checker = Checker::new(&ignored_paths);
        assert_eq!(checker.is_hidden(Path::new("test/test")), false);
    }
}

#[cfg(test)]
mod symlink_test {
    use std::collections::HashSet;
    use std::path::PathBuf;

    use crate::query::checker::Checker;

    const SYMLINK_PATH: &'static str = "/System/Library/PreferencePanes/PrintAndFax.prefPane";
    const APP_PATH: &'static str = "/System/Applications/Books.app";

    #[test]
    fn test_is_legit() {
        let path = PathBuf::from(SYMLINK_PATH);
        let ignored_paths = HashSet::new();
        let checker = Checker::new(&ignored_paths);
        assert!(checker.is_symlink(&path))
    }

    #[test]
    fn test_is_not_legit() {
        let path = PathBuf::from(APP_PATH);
        let ignored_paths = HashSet::new();
        let checker = Checker::new(&ignored_paths);
        assert_eq!(checker.is_symlink(&path), false);
    }
}

#[cfg(test)]
mod ignore_checker_test {
    use std::collections::HashSet;
    use std::path::{Path, PathBuf};

    use crate::query::checker::Checker;

    #[test]
    fn test_is_legit() {
        let ignored_paths: HashSet<PathBuf> = vec!["/Users/cheng", "/usr/bin"]
            .into_iter()
            .map(PathBuf::from)
            .collect();
        let checker = Checker::new(&ignored_paths);
        assert!(checker.is_ignored_path(Path::new("/Users/cheng")))
    }

    #[test]
    fn test_is_not_legit() {
        let ignored_paths: HashSet<PathBuf> = vec!["/Users/cheng", "/usr/bin"]
            .into_iter()
            .map(PathBuf::from)
            .collect();
        let checker = Checker::new(&ignored_paths);
        assert_eq!(
            checker.is_ignored_path(Path::new("/Users/cheng/Applications")),
            false
        )
    }
}
