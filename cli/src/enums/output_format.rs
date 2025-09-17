use crate::error::RomcalCliError;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

/// Output format options for the CLI.
/// Defines the available formats for displaying calendar data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    /// YAML format (human-readable, structured)
    Yaml,
    /// JSON format (machine-readable, structured)
    Json,
    /// CSV format (comma-separated values, tabular)
    Csv,
    /// Lines format (simple text, one item per line)
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
    /// Print data in the specified format.
    /// Formats and displays the provided data according to the selected output format.
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

/// Output format for CLI.
/// CLI-specific enum that implements clap::ValueEnum for command-line argument parsing.
#[derive(ValueEnum, Clone, Debug)]
pub enum CliOutputFormat {
    /// YAML format (human-readable, structured)
    Yaml,
    /// JSON format (machine-readable, structured)
    Json,
    /// CSV format (comma-separated values, tabular)
    Csv,
    /// Lines format (simple text, one item per line)
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
