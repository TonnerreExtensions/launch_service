use std::fs::read_dir;
use std::io::Write;
use std::path::Path;

use crate::query::checker::{Checker, Outcome};
use crate::query::matcher;
use crate::query::service::Service;
use crate::utils::serde::serialize_to_string;

pub struct QueryProcessor<'a, W: Write> {
    checker: Checker<'a>,
    output: W,
}

impl<'a, W: Write> QueryProcessor<'a, W> {
    /// New query processor
    pub fn new(writer: W) -> Self {
        let ignored_paths = crate::CONFIG.get_ignore_paths();
        QueryProcessor {
            checker: Checker::new(ignored_paths),
            output: writer,
        }
    }

    /// Query based on the request, and return serialized bytes of the services
    pub fn query(&mut self, req: &str) {
        let req = req.to_lowercase();
        crate::CONFIG
            .get_paths()
            .into_iter()
            .for_each(|path| self.walk_paths(path, &req));
    }

    /// Recursively iterate through files and folders, and return all legit file paths
    fn walk_paths<P: AsRef<Path>>(&mut self, entry: P, query: &str) {
        let entry = entry.as_ref();
        match self.checker.check(entry) {
            Outcome::UnwantedPath => (),
            Outcome::BundlePath => self.filter_output_path(entry, query),
            Outcome::NormalPath => {
                let mut read_folder = match read_dir(&entry) {
                    Ok(read_folder) => read_folder,
                    Err(err) => {
                        eprintln!("Read folder error: {}", err);
                        return;
                    }
                };
                while let Some(Ok(path)) = read_folder.next() {
                    self.walk_paths(path.path(), query);
                }
            }
        }
    }

    fn filter_output_path(&mut self, path: &Path, query: &str) {
        let service = Service::new(path);
        if matcher::match_query(query, &service.title) {
            match serialize_to_string(&service) {
                Ok(service) => {
                    if let Err(error) = writeln!(self.output, "{}", service) {
                        eprintln!("Failed to write: {}", error);
                    }
                }
                Err(error) => eprintln!("Failed to serialize: {}", error),
            }
        }
    }
}

#[cfg(test)]
mod query_test {
    use std::path::PathBuf;

    use crate::query::query::QueryProcessor;

    type QP<'a, W> = QueryProcessor<'a, W>;

    const APP_PATH: &str = "/System/Applications/Books.app";
    const APP_FOLDER_PATH: &str = "/System/Applications";

    #[test]
    fn test_walk_dir_single() {
        let settings = crate::configurator::get_content();
        std::env::set_var("SETTINGS", settings);
        let mut output = Vec::<u8>::new();
        let mut processor = QP::new(&mut output);
        let single_file = PathBuf::from(APP_PATH);
        processor.walk_paths(&single_file, "book");
        let string = String::from_utf8(output).expect("Failed to parse");
        let expected = r#"{"title":"Books","subtitle":"/System/Applications/Books.app","id":"/System/Applications/Books.app"}
"#;
        assert_eq!(string, expected);
    }

    #[test]
    fn test_walk_dir_all_apps_starts_with_a() {
        let settings = crate::configurator::get_content();
        std::env::set_var("SETTINGS", settings);
        let mut output = Vec::new();
        let mut processor = QP::new(&mut output);
        let content = PathBuf::from(APP_FOLDER_PATH);
        processor.walk_paths(&content, "a");
        let string = String::from_utf8(output).expect("Failed to parse");
        let lines = string.split("\n").collect::<Vec<_>>();
        assert_eq!(lines.len(), 9);
    }
}
