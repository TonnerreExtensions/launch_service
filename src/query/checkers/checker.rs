use std::path::Path;

pub trait Checker {
    fn new() -> Self;
    fn is_legit(&self, path: &Path) -> bool;
}
