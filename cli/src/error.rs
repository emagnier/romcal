use thiserror::Error;

#[derive(Error, Debug)]
pub enum RomcalCliError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Invalid year: {0} (must be >= 1583)")]
    InvalidYear(i32),

    #[error("Invalid calculation type: {0}. Use 'gregorian' or 'julian'")]
    InvalidCalculationType(String),

    #[error("Unsupported locale: {0}")]
    UnsupportedLocale(String),

    #[error("Calendar not found: {0}")]
    CalendarNotFound(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("YAML serialization error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("Romcal error: {0}")]
    RomcalError(#[from] romcal_core::RomcalError),
}

impl RomcalCliError {
    pub fn config_error(msg: impl Into<String>) -> Self {
        Self::ConfigError(msg.into())
    }

    pub fn invalid_year(year: i32) -> Self {
        Self::InvalidYear(year)
    }

    pub fn invalid_calculation_type(calc_type: impl Into<String>) -> Self {
        Self::InvalidCalculationType(calc_type.into())
    }

    pub fn unsupported_locale(locale: impl Into<String>) -> Self {
        Self::UnsupportedLocale(locale.into())
    }

    pub fn calendar_not_found(calendar: impl Into<String>) -> Self {
        Self::CalendarNotFound(calendar.into())
    }

    /// Validate output format and return error if invalid
    pub fn validate_format(format: &str) -> Result<(), Self> {
        match format.to_lowercase().as_str() {
            "json" | "csv" | "yaml" | "lines" => Ok(()),
            _ => Err(Self::config_error(
                "Invalid format. Must be 'json', 'csv', 'yaml', or 'lines'",
            )),
        }
    }
}
