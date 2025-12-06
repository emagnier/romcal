use thiserror::Error;

/// Errors for the CLI.
/// Defines the different errors that can occur when running the CLI.
#[derive(Error, Debug)]
pub enum RomcalCliError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Invalid year: {0} (must be >= 1583 and <= 9999)")]
    InvalidYear(i32),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("YAML serialization error: {0}")]
    YamlError(#[from] serde_saphyr::Error),

    #[error("Romcal error: {0}")]
    RomcalError(#[from] romcal_core::RomcalError),

    #[error("JSON schema validation error: {0}")]
    SchemaValidationError(#[from] Box<jsonschema::ValidationError<'static>>),
}

impl RomcalCliError {
    pub fn config_error(msg: impl Into<String>) -> Self {
        Self::ConfigError(msg.into())
    }

    pub fn invalid_year(year: i32) -> Self {
        Self::InvalidYear(year)
    }
}
