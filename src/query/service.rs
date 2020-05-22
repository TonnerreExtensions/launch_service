use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::matcher;

fn map_term(name: &str) -> String {
    crate::CONFIG
        .get_pref_names()
        .get(name)
        .map(String::to_owned)
        .unwrap_or(matcher::tokenize(name).join(" "))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    pub title: String,
    subtitle: PathBuf,
    id: PathBuf,
}

impl Service {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let path = path.as_ref();
        let title = Self::file_name(&path);
        let subtitle = path.to_path_buf();
        Service {
            title,
            subtitle,
            id: path.to_path_buf(),
        }
    }

    fn file_name<P: AsRef<Path>>(path: P) -> String {
        let path = path.as_ref();
        let path_process = path.file_stem().and_then(OsStr::to_str);
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("prefPane") => path_process.map(map_term),
            _ => path_process.map(String::from),
        }
        .unwrap_or_default()
    }
}

#[cfg(test)]
mod service_test {
    use crate::query::service::Service;

    #[test]
    fn test_serialize() {
        let path = "/System/Applications/Book.app";
        let service = Service::new(path);
        let serialized = serde_json::to_string(&service).expect("Unable to serialize");
        let expected = r#"{"title":"Book","subtitle":"/System/Applications/Book.app","id":"/System/Applications/Book.app"}"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_deserialize() {
        let source = r#"{"title":"Book","subtitle":"/System/Applications/Book.app","id":"/System/Applications/Book.app"}"#;
        let service: Service = serde_json::from_str(source).expect("Unable to deserialize");
        let expected = "/System/Applications/Book.app";
        assert_eq!(service.id.to_str().unwrap_or(""), expected);
    }

    #[test]
    fn test_bunch_serialize() {
        let services = vec![
            Service::new("/System/Applications/Book.app"),
            Service::new("/System/Applications/Safari.app"),
        ];
        let serialized = serde_json::to_string(&services).expect("Unable to serialize");
        let expected = r#"[{"title":"Book","subtitle":"/System/Applications/Book.app","id":"/System/Applications/Book.app"},{"title":"Safari","subtitle":"/System/Applications/Safari.app","id":"/System/Applications/Safari.app"}]"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_bunch_deserialize() {
        let source = r#"[{"title":"Book","subtitle":"/System/Applications/Book.app","id":"/System/Applications/Book.app"}
        ,{"title":"Safari","subtitle":"/System/Applications/Safari.app","id":"/System/Applications/Safari.app"}]"#;
        let services: Vec<Service> = serde_json::from_str(source).expect("Unable to deserialize");
        let services = services
            .iter()
            .map(|service| service.id.to_str().unwrap_or(""))
            .collect::<Vec<_>>();
        let expected = vec![
            "/System/Applications/Book.app",
            "/System/Applications/Safari.app",
        ];
        assert_eq!(services, expected);
    }
}
