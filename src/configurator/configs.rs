use std::collections::{HashMap, HashSet as Set};
use std::io;
use std::io::ErrorKind;

use async_std::path::PathBuf;
use yaml_rust;
use yaml_rust::yaml::Yaml;
use yaml_rust::YamlLoader;

use crate::utils;

/// Configs that are loaded from a yaml file
#[derive(Debug)]
pub struct Configs {
    file: Yaml,
}

impl Configs {
    const CONFIGURABLE_KEY: &'static str = "configurable";
    const IGNORE_KEY: &'static str = "ignorePaths";
    const VALUES_KEY: &'static str = "values";
    const INTERNAL_KEY: &'static str = "internal";
    const CACHED_KEY: &'static str = "cached";
    const UPDATED_KEY: &'static str = "updated";
    const PREFNAME_KEY: &'static str = "prefNames";

    /// Construct config from given yaml file
    pub fn from<S: AsRef<str>>(content: S) -> io::Result<Self> {
        let content = content.as_ref();
        match YamlLoader::load_from_str(content) {
            Ok(mut files) => Ok(Configs {
                file: match files.pop() {
                    Some(file) => file,
                    None => {
                        return Err(io::Error::new(
                            ErrorKind::InvalidData,
                            "Failed to pop last of yaml",
                        ))
                    }
                },
            }),
            Err(err) => Err(io::Error::new(ErrorKind::InvalidData, err)),
        }
    }

    /// Get ignored path
    pub fn get_ignore_paths(&self) -> Set<PathBuf> {
        self.file[Self::CONFIGURABLE_KEY][Self::IGNORE_KEY][Self::VALUES_KEY]
            .as_vec()
            .map(Self::convert_and_box)
            .unwrap_or_default()
    }

    /// Get paths need to be cached
    pub fn get_internal_cached(&self) -> Set<PathBuf> {
        self.file[Self::INTERNAL_KEY][Self::CACHED_KEY]
            .as_vec()
            .map(Self::convert_and_box)
            .unwrap_or_default()
    }

    /// Get paths that updates every time
    pub fn get_internal_updated(&self) -> Set<PathBuf> {
        self.file[Self::INTERNAL_KEY][Self::UPDATED_KEY]
            .as_vec()
            .map(Self::convert_and_box)
            .unwrap_or_default()
    }

    pub fn get_pref_names(&self) -> HashMap<String, String> {
        let names = self.file[Self::INTERNAL_KEY][Self::PREFNAME_KEY].as_hash();
        if names.is_none() {
            return HashMap::new();
        }
        let names = names.unwrap();
        names
            .into_iter()
            .filter_map(|(key, val)| Some((key.as_str()?.to_owned(), val.as_str()?.to_owned())))
            .collect()
    }

    /// Convert yaml data to str and box with vec
    fn convert_and_box(data: &Vec<Yaml>) -> Set<PathBuf> {
        data.iter()
            .filter_map(Yaml::as_str)
            .map(utils::expand_tilde)
            .map(PathBuf::from)
            .collect()
    }
}

#[cfg(test)]
mod configs_test {
    use crate::configurator::configs::Configs;

    fn get_content() -> String {
        std::fs::read_to_string("settings.yaml").expect("Failed to read settings.yaml")
    }

    #[test]
    fn test_new() {
        let res = Configs::from(get_content());
        assert!(res.is_ok());
    }

    #[test]
    fn test_get_ignore_paths() {
        let res = Configs::from(get_content()).unwrap();
        let ignore_path = res.get_ignore_paths();
        assert!(ignore_path.is_empty())
    }

    #[test]
    fn test_get_cached_paths() {
        let res = Configs::from(get_content()).unwrap();
        let cached_path = res.get_internal_cached();
        assert_eq!(cached_path.len(), 4);
    }

    #[test]
    fn test_get_updated_paths() {
        let res = Configs::from(get_content()).unwrap();
        let updated_path = res.get_internal_updated();
        assert_eq!(updated_path.len(), 2);
    }

    #[test]
    fn test_get_pref_names() {
        let res = Configs::from(get_content()).unwrap();
        let pref_names = res.get_pref_names();
        assert_eq!(pref_names.len(), 26);
    }
}
