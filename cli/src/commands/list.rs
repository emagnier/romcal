use crate::{
    error::RomcalCliError,
    output::{validate_format, OutputFormat},
};
use romcal_core::{CALENDAR_IDS, LOCALE_CODES};
use serde_json;
use serde_yaml;

/// Generic function to list items in various formats
fn list_items(items: &[&str], format: &str) -> Result<(), RomcalCliError> {
    validate_format(format)?;

    let output_format = match format.to_lowercase().as_str() {
        "json" => OutputFormat::Json,
        "csv" => OutputFormat::Csv,
        "yaml" => OutputFormat::Yaml,
        "lines" => OutputFormat::Lines,
        _ => unreachable!(), // Already validated above
    };

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

/// Handle list calendars command
pub fn handle_calendars(format: &str) -> Result<(), RomcalCliError> {
    list_items(CALENDAR_IDS, format)
}

/// Handle list locales command
pub fn handle_locales(format: &str) -> Result<(), RomcalCliError> {
    list_items(LOCALE_CODES, format)
}
