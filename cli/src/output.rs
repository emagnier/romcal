use crate::error::RomcalCliError;
use serde::{Deserialize, Serialize};

/// Output format options for the CLI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Json,
    Csv,
    Yaml,
    Lines,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "csv" => Ok(OutputFormat::Csv),
            "yaml" => Ok(OutputFormat::Yaml),
            "lines" => Ok(OutputFormat::Lines),
            _ => Err(format!("Invalid output format: {}", s)),
        }
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Csv => write!(f, "csv"),
            OutputFormat::Yaml => write!(f, "yaml"),
            OutputFormat::Lines => write!(f, "lines"),
        }
    }
}

impl OutputFormat {
    /// Print data in the specified format
    pub fn print(&self, data: &str) -> Result<(), RomcalCliError> {
        match self {
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(data)?);
            }
            OutputFormat::Csv | OutputFormat::Lines => {
                println!("{}", data);
            }
            OutputFormat::Yaml => {
                println!("{}", serde_yaml::to_string(data)?);
            }
        }
        Ok(())
    }
}

/// Validate output format and return error if invalid
pub fn validate_format(format: &str) -> Result<(), RomcalCliError> {
    match format.to_lowercase().as_str() {
        "json" | "csv" | "yaml" | "lines" => Ok(()),
        _ => Err(RomcalCliError::config_error(
            "Invalid format. Must be 'json', 'csv', 'yaml', or 'lines'",
        )),
    }
}
