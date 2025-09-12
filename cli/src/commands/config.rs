use crate::config::CliConfig;
use crate::error::RomcalCliError;
use serde_json;
use serde_yaml;

/// Handle configuration display command
pub fn handle(
    format: &str,
    easter_calculation_type: &str,
    scope: &str,
    ascension_on_sunday: bool,
    epiphany_on_sunday: bool,
    corpus_christi_on_sunday: bool,
) -> Result<(), RomcalCliError> {
    let cli_config = CliConfig::default();

    RomcalCliError::validate_format(format)?;

    // Prepare configuration data (current CLI context)
    let config_data = serde_json::json!({
        "locale": cli_config.default_locale,
        "calendar": cli_config.default_calendar,
        "easter_calculation_type": easter_calculation_type,
        "scope": scope,
        "ascension_on_sunday": ascension_on_sunday,
        "epiphany_on_sunday": epiphany_on_sunday,
        "corpus_christi_on_sunday": corpus_christi_on_sunday
    });

    match format {
        "json" => {
            println!("{}", serde_json::to_string_pretty(&config_data)?);
        }
        "csv" => {
            println!("setting,value");
            println!("locale,{}", cli_config.default_locale);
            println!("calendar,{}", cli_config.default_calendar);
            println!("easter_calculation_type,{}", easter_calculation_type);
            println!("scope,{}", scope);
            println!("ascension_on_sunday,{}", ascension_on_sunday);
            println!("epiphany_on_sunday,{}", epiphany_on_sunday);
            println!("corpus_christi_on_sunday,{}", corpus_christi_on_sunday);
        }
        "lines" => {
            println!("locale: {}", cli_config.default_locale);
            println!("calendar: {}", cli_config.default_calendar);
            println!("easter_calculation_type: {}", easter_calculation_type);
            println!("scope: {}", scope);
            println!("ascension_on_sunday: {}", ascension_on_sunday);
            println!("epiphany_on_sunday: {}", epiphany_on_sunday);
            println!("corpus_christi_on_sunday: {}", corpus_christi_on_sunday);
        }
        "yaml" => {
            println!("{}", serde_yaml::to_string(&config_data)?);
        }
        _ => unreachable!(), // Already validated above
    }

    Ok(())
}
