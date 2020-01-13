extern crate shellexpand;
use std::path::PathBuf;

pub fn expand_tilde(path: &str) -> PathBuf {
    PathBuf::from(shellexpand::tilde(path).to_string())
}

#[cfg(test)]
mod utils_test {
    use crate::utils::tilde_expander::expand_tilde;
    use std::path::PathBuf;

    #[test]
    fn test_expand_tilde() {
        let before = "~/Applications";
        let res = expand_tilde(before);
        assert_ne!(res, PathBuf::from(before));
        assert!(res.to_str().unwrap().ends_with("/Applications"));
    }

    #[test]
    fn test_expand_tilde_without_tilde() {
        let before = "/Users";
        let res = expand_tilde(before);
        assert_eq!(res, PathBuf::from(before));
    }
}