use async_std::path::Path;

pub trait Checker {
    fn is_legit(&self, path: &Path) -> bool;
}
