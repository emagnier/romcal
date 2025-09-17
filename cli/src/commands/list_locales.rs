use crate::{enums::OutputFormat, error::RomcalCliError};
use romcal_core::LOCALE_CODES;
use serde_json;
use serde_yaml;

/// Generic function to list items in various formats
fn list_items(items: &[&str], output_format: OutputFormat) -> Result<(), RomcalCliError> {
    match output_format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(items)?);
        }
        OutputFormat::Lines => {
            for item in items {
                println!("{}", item);
            }
        }
        OutputFormat::Csv => {
            println!("{}", items.join(","));
        }
        OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(items)?);
        }
    }
    Ok(())
}

/// Handle list locales command
pub fn handle_locales(output_format: OutputFormat) -> Result<(), RomcalCliError> {
    list_items(LOCALE_CODES, output_format)
}
