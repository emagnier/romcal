use crate::config::CliConfig;
use crate::error::RomcalCliError;
use serde_json;
use serde_yaml;

/// Handle list calendars command
pub fn handle_calendars(format: &str) -> Result<(), RomcalCliError> {
    let config = CliConfig::default();
    let calendars = config.get_supported_calendars();

    RomcalCliError::validate_format(format)?;

    match format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&calendars)?);
        }
        "lines" => {
            for calendar in calendars {
                println!("{}", calendar);
            }
        }
        "csv" => {
            println!("{}", calendars.join(","));
        }
        "yaml" => {
            println!("{}", serde_yaml::to_string(&calendars)?);
        }
        _ => unreachable!(), // Already validated above
    }
    Ok(())
}

/// Handle list locales command
pub fn handle_locales(format: &str) -> Result<(), RomcalCliError> {
    let config = CliConfig::default();
    let locales = config.get_supported_locales();

    RomcalCliError::validate_format(format)?;

    match format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&locales)?);
        }
        "lines" => {
            for locale in locales {
                println!("{}", locale);
            }
        }
        "csv" => {
            println!("{}", locales.join(","));
        }
        "yaml" => {
            println!("{}", serde_yaml::to_string(&locales)?);
        }
        _ => unreachable!(), // Already validated above
    }
    Ok(())
}
