use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::PathBuf;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

lazy_static! {
    pub static ref NAME_MAP: HashMap<&'static str, &'static str> =
        [("And", "&")].iter().cloned().collect();
}

fn map_term(name: &str) -> &str {
    NAME_MAP.get(name).unwrap_or(&name)
}

#[derive(Serialize)]
pub struct FullService<'a> {
    title: &'a str,
    subtitle: &'a OsStr,
    id: &'a OsStr,
}

pub struct Service {
    pub path: PathBuf,
}

impl Service {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Service { path: path.into() }
    }

    fn file_name(&self) -> &str {
        let path_process = self.path.file_stem().and_then(OsStr::to_str);
        match self.path.extension().and_then(|ext| ext.to_str()) {
            Some("prefPane") => path_process.map(map_term),
            _ => path_process,
        }
        .unwrap_or_default()
    }

    pub fn full_service(&self) -> FullService {
        FullService {
            title: self.file_name(),
            subtitle: self.path.as_os_str(),
            id: self.path.as_os_str(),
        }
    }
}

impl Serialize for Service {
    fn serialize<S>(&self, serializer: S) -> Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        self.path.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Service {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
    where
        D: Deserializer<'de>,
    {
        let path = PathBuf::deserialize(deserializer)?;
        Ok(Service::new(path))
    }
}

#[cfg(test)]
mod service_test {
    use crate::query::service::{map_term, Service};

    #[test]
    fn test_map_term() {
        assert_eq!(map_term("And"), "&");
    }

    #[test]
    fn test_serialize() {
        let path = "/System/Applications/Book.app";
        let service = Service::new(path);
        let serialized = serde_json::to_string(&service).expect("Unable to serialize");
        let expected = r#""/System/Applications/Book.app""#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_deserialize() {
        let source = r#""/System/Applications/Book.app""#;
        let service: Service = serde_json::from_str(source).expect("Unable to deserialize");
        let expected = "/System/Applications/Book.app";
        assert_eq!(service.path.to_str().unwrap_or(""), expected);
    }

    #[test]
    fn test_bunch_serialize() {
        let services = vec![
            Service::new("/System/Applications/Book.app"),
            Service::new("/System/Applications/Safari.app"),
        ];
        let serialized = serde_json::to_string(&services).expect("Unable to serialize");
        let expected = r#"["/System/Applications/Book.app","/System/Applications/Safari.app"]"#;
        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_bunch_deserialize() {
        let source = r#"["/System/Applications/Book.app", "/System/Applications/Safari.app"]"#;
        let services: Vec<Service> = serde_json::from_str(source).expect("Unable to deserialize");
        let services = services
            .iter()
            .map(|service| service.path.to_str().unwrap_or(""))
            .collect::<Vec<_>>();
        let expected = vec![
            "/System/Applications/Book.app",
            "/System/Applications/Safari.app",
        ];
        assert_eq!(services, expected);
    }
}
