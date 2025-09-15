use crate::error::RomcalCliError;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

/// Output format options for the CLI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Yaml,
    Json,
    Csv,
    Lines,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "yaml" => Ok(OutputFormat::Yaml),
            "json" => Ok(OutputFormat::Json),
            "csv" => Ok(OutputFormat::Csv),
            "lines" => Ok(OutputFormat::Lines),
            _ => Err(format!("Invalid output format: {}", s)),
        }
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OutputFormat::Yaml => write!(f, "yaml"),
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Csv => write!(f, "csv"),
            OutputFormat::Lines => write!(f, "lines"),
        }
    }
}

impl OutputFormat {
    /// Print data in the specified format
    pub fn print(&self, data: &str) -> Result<(), RomcalCliError> {
        match self {
            OutputFormat::Yaml => {
                println!("{}", serde_yaml::to_string(data)?);
            }
            OutputFormat::Json => {
                println!("{}", serde_json::to_string_pretty(data)?);
            }
            OutputFormat::Csv | OutputFormat::Lines => {
                println!("{}", data);
            }
        }
        Ok(())
    }
}

/// Output format for CLI
#[derive(ValueEnum, Clone, Debug)]
pub enum CliOutputFormat {
    /// YAML format
    Yaml,
    /// JSON format
    Json,
    /// CSV format
    Csv,
    /// Lines format
    Lines,
}

impl From<CliOutputFormat> for OutputFormat {
    fn from(format: CliOutputFormat) -> Self {
        match format {
            CliOutputFormat::Yaml => OutputFormat::Yaml,
            CliOutputFormat::Json => OutputFormat::Json,
            CliOutputFormat::Csv => OutputFormat::Csv,
            CliOutputFormat::Lines => OutputFormat::Lines,
        }
    }
}
