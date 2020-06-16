use std::collections::{HashMap, HashSet as Set};
use std::io::{self, Error, ErrorKind};
use std::path::PathBuf;

use serde::Deserialize;

use crate::utils;
use crate::utils::serde::deserialize_from_bytes;

#[derive(Deserialize)]
pub struct Configs {
    internal: Internal,
    configurable: Configurable,
}

#[derive(Deserialize)]
struct Internal {
    system: Set<PathBuf>,
    users: Set<PathBuf>,
    #[serde(rename = "prefNames")]
    preferred_names: HashMap<String, String>,
}

#[derive(Deserialize)]
struct Configurable {
    #[serde(rename = "ignorePaths")]
    ignored_paths: ConfigurableValue,
}

#[derive(Deserialize)]
struct ConfigurableValue {
    value: Set<PathBuf>,
}

impl Configs {
    /// Construct config from given yaml file
    pub fn from<S: AsRef<str>>(content: S) -> io::Result<Self> {
        let mut configs: Configs = deserialize_from_bytes(content.as_ref().as_bytes())
            .map_err(|error| Error::new(ErrorKind::InvalidData, error))?;
        Self::expand_path(&mut configs.internal.system);
        Self::expand_path(&mut configs.internal.users);
        Self::expand_path(&mut configs.configurable.ignored_paths.value);
        Ok(configs)
    }

    fn expand_path(paths: &mut Set<PathBuf>) {
        let expanded = paths
            .iter()
            .filter_map(|path| Some(utils::expand_tilde(path.to_str()?)))
            .collect();
        *paths = expanded;
    }

    /// Get ignored path
    pub fn get_ignore_paths(&self) -> &Set<PathBuf> {
        &self.configurable.ignored_paths.value
    }

    /// Get paths need to be cached
    pub fn get_system_paths(&self) -> &Set<PathBuf> {
        &self.internal.system
    }

    /// Get paths that updates every time
    pub fn get_users_paths(&self) -> &Set<PathBuf> {
        &self.internal.users
    }

    pub fn get_pref_names(&self) -> &HashMap<String, String> {
        &self.internal.preferred_names
    }
}

#[cfg(test)]
pub mod configs_test {
    use crate::configurator::configs::Configs;

    pub fn get_content() -> String {
        r#"
{
  "configurable": {
    "ignorePaths": {
      "displayName": "Paths to Ignore",
      "value": []
    }
  },
  "internal": {
    "system": [
      "/System/Library/CoreServices/Finder.app",
      "/System/Library/CoreServices/Applications",
      "/System/Library/PreferencePanes",
      "/System/Applications"
    ],
    "users": ["~/Applications", "/Applications"],
    "prefNames": {
      "SoftwareUpdate": "Software Update",
      "iCloudPref": "iCloud",
      "Accounts": "Users & Groups",
      "AppStore": "App Store",
      "Appearance": "General",
      "DateAndTime": "Date & Time",
      "DesktopScreenEffectsPref": "Desktop & Screen Saver",
      "DigiHubDiscs": "CDs & DVDs",
      "EnergySaver": "Energy Saver",
      "Expose": "Mission Control",
      "FibreChannel": "Fibre Channel",
      "InternetAccounts": "Internet Accounts",
      "Localization": "Language & Region",
      "ParentalControls": "Parental Controls",
      "PrintAndFax": "Printers & Fax",
      "PrintAndScan": "Printers & Scanners",
      "Security": "Security & Privacy",
      "SharingPref": "Sharing",
      "Speech": "Siri",
      "StartupDisk": "Startup Disk",
      "TimeMachine": "Time Machine",
      "TouchID": "Touch ID",
      "UniversalAccessPref": "Accessibility",
      "Wallet": "Wallet & Apple Pay",
      "AppleIDPrefPane": "Apple ID",
      "FamilySharingPrefPane": "Family Sharing"
    }
  }
}"#
        .to_owned()
    }

    #[test]
    fn test_new() {
        let res = Configs::from(get_content());
        assert!(res.is_ok(), res.err().unwrap().to_string());
    }

    #[test]
    fn test_get_ignore_paths() {
        let res = Configs::from(get_content()).unwrap();
        let ignore_path = res.get_ignore_paths();
        assert!(ignore_path.is_empty())
    }

    #[test]
    fn test_get_system_paths() {
        let res = Configs::from(get_content()).unwrap();
        let cached_path = res.get_system_paths();
        assert_eq!(cached_path.len(), 4);
    }

    #[test]
    fn test_get_users_paths() {
        let res = Configs::from(get_content()).unwrap();
        let updated_path = res.get_users_paths();
        assert_eq!(updated_path.len(), 2);
    }

    #[test]
    fn test_get_pref_names() {
        let res = Configs::from(get_content()).unwrap();
        let pref_names = res.get_pref_names();
        assert_eq!(pref_names.len(), 26);
    }
}
