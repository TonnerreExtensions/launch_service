use std::collections::HashSet as Set;
use std::io;
use std::io::ErrorKind;
use std::path::Path;

use async_std::path::PathBuf;
use yaml_rust;
use yaml_rust::yaml::Yaml;
use yaml_rust::YamlLoader;

use crate::utils;

/// Configs that are loaded from a yaml file
#[derive(Debug)]
pub struct Configs {
    file: Yaml
}

impl Configs {
    const CONFIGURABLE_KEY: &'static str = "configurable";
    const IGNORE_KEY: &'static str = "ignorePaths";
    const VALUES_KEY: &'static str = "values";
    const INTERNAL_KEY: &'static str = "internal";
    const CACHED_KEY: &'static str = "cached";
    const UPDATED_KEY: &'static str = "updated";

    /// Construct config from given yaml file
    pub fn from(config_file: &Path) -> io::Result<Self> {
        let content = std::fs::read_to_string(config_file)?;
        match YamlLoader::load_from_str(&content) {
            Ok(mut files) => Ok(
                Configs {
                    file: match files.pop() {
                        Some(file) => file,
                        None => return Err(io::Error::new(ErrorKind::InvalidData, "Failed to pop last of yaml"))
                    }
                }),
            Err(err) => Err(io::Error::new(ErrorKind::InvalidData, err))
        }
    }

    /// Get ignored path
    pub fn get_ignore_paths(&self) -> Set<PathBuf> {
        self.file[Self::CONFIGURABLE_KEY][Self::IGNORE_KEY][Self::VALUES_KEY]
            .as_vec().map(Self::convert_and_box).unwrap_or_default()
    }

    /// Get paths need to be cached
    pub fn get_internal_cached(&self) -> Set<PathBuf> {
        self.file[Self::INTERNAL_KEY][Self::CACHED_KEY]
            .as_vec().map(Self::convert_and_box).unwrap_or_default()
    }

    /// Get paths that updates every time
    pub fn get_internal_updated(&self) -> Set<PathBuf> {
        self.file[Self::INTERNAL_KEY][Self::UPDATED_KEY]
            .as_vec().map(Self::convert_and_box).unwrap_or_default()
    }

    /// Convert yaml data to str and box with vec
    fn convert_and_box(data: &Vec<Yaml>) -> Set<PathBuf> {
        data.iter().filter_map(Yaml::as_str)
            .map(utils::expand_tilde)
            .map(PathBuf::from)
            .collect()
    }
}

#[cfg(test)]
mod configs_test {
    use std::path::Path;

    use crate::configurator::configs::Configs;

    const PATH: &'static str = "settings.yaml";

    #[test]
    fn test_new() {
        let path = Path::new(PATH);
        let res = Configs::from(path);
        assert!(res.is_ok());
    }

    #[test]
    fn test_get_ignore_paths() {
        let path = Path::new(PATH);
        let res = Configs::from(path).unwrap();
        let ignore_path = res.get_ignore_paths();
        assert!(ignore_path.is_empty())
    }

    #[test]
    fn test_get_cached_paths() {
        let path = Path::new(PATH);
        let res = Configs::from(path).unwrap();
        let cached_path = res.get_internal_cached();
        assert_eq!(cached_path.len(), 4);
    }

    #[test]
    fn test_get_updated_paths() {
        let path = Path::new(PATH);
        let res = Configs::from(path).unwrap();
        let updated_path = res.get_internal_updated();
        assert_eq!(updated_path.len(), 2);
    }
}
