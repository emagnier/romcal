use crate::error::RomcalCliError;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

/// Configuration file structure
#[derive(Debug, Default, Deserialize)]
pub struct Config {
    pub calendar: Option<String>,
    pub locale: Option<String>,
    pub format: Option<String>,
    pub context: Option<String>,
    pub easter_calculation_type: Option<String>,
    pub epiphany_on_sunday: Option<bool>,
    pub ascension_on_sunday: Option<bool>,
    pub corpus_christi_on_sunday: Option<bool>,
}

/// Project config file name
const PROJECT_CONFIG_FILE: &str = ".romcal.toml";

/// User config file name (inside config dir)
const USER_CONFIG_FILE: &str = "config.toml";

/// App name for config directory
const APP_NAME: &str = "romcal";

impl Config {
    /// Load configuration from files with priority:
    /// 1. Custom path (--config flag) - highest priority
    /// 2. Project config (./.romcal.toml)
    /// 3. User config (~/.config/romcal/config.toml)
    pub fn load(custom_path: Option<&Path>) -> Result<Self, RomcalCliError> {
        // If custom path provided, use only that
        if let Some(path) = custom_path {
            return Self::load_from_file(path);
        }

        // Try project config first
        let project_config = Path::new(PROJECT_CONFIG_FILE);
        if project_config.exists() {
            return Self::load_from_file(project_config);
        }

        // Try user config
        if let Some(user_config) = Self::user_config_path() {
            if user_config.exists() {
                return Self::load_from_file(&user_config);
            }
        }

        // No config found, return defaults
        Ok(Self::default())
    }

    /// Load configuration from a specific file
    fn load_from_file(path: &Path) -> Result<Self, RomcalCliError> {
        let content = fs::read_to_string(path).map_err(|e| {
            RomcalCliError::config_error(format!(
                "Failed to read config file '{}': {}",
                path.display(),
                e
            ))
        })?;

        toml::from_str(&content).map_err(|e| {
            RomcalCliError::config_error(format!(
                "Failed to parse config file '{}': {}",
                path.display(),
                e
            ))
        })
    }

    /// Get the user config file path
    /// Uses ~/.config on Unix (Linux/macOS) for consistency, and %APPDATA% on Windows
    fn user_config_path() -> Option<PathBuf> {
        #[cfg(unix)]
        {
            dirs::home_dir().map(|p| p.join(".config").join(APP_NAME).join(USER_CONFIG_FILE))
        }
        #[cfg(windows)]
        {
            dirs::config_dir().map(|p| p.join(APP_NAME).join(USER_CONFIG_FILE))
        }
    }
}
