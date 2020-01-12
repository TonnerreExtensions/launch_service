use std::fs::{read_dir, ReadDir, DirEntry};
use std::path::{Path, PathBuf};
use std::borrow::Borrow;

pub fn query(req: String) -> String {
    "".to_string()
}

fn walk_dir(entry: PathBuf) -> Vec<PathBuf> {
    if let Ok(files) = read_dir(&entry) {
        files.filter_map(Result::ok)
            .map(|entry| entry.path())
            .flat_map(walk_dir)
            .collect()
    } else if entry.is_file() {
        vec![entry]
    } else {
        vec![]
    }
}

#[cfg(test)]
mod query_test {
    use crate::query::query;
    use std::path::PathBuf;

    #[test]
    fn test_walk_dir_single() {
        let single_file = PathBuf::from("/Applications/IINA.app");
        let res = query::walk_dir(single_file);
        println!("{:?}", res);
    }
}