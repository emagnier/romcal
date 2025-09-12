use crate::config::create_liturgical_config;
use crate::error::RomcalCliError;
use crate::output::validate_format;
use crate::output::OutputFormat;
use serde_json;
use serde_yaml;

/// Configuration data for display
#[derive(Debug, Clone)]
struct ConfigDisplayData {
    locale: String,
    calendar: String,
    easter_calculation_type: String,
    scope: String,
    ascension_on_sunday: bool,
    epiphany_on_sunday: bool,
    corpus_christi_on_sunday: bool,
}

impl ConfigDisplayData {
    /// Create from liturgical config
    fn from_liturgical_config(config: &romcal_core::LiturgicalConfig) -> Self {
        Self {
            locale: config.locale.clone(),
            calendar: config.calendar.clone(),
            easter_calculation_type: match config.easter_calculation_type {
                romcal_core::EasterCalculationType::Gregorian => "gregorian",
                romcal_core::EasterCalculationType::Julian => "julian",
            }
            .to_string(),
            scope: match config.scope {
                romcal_core::CalendarScope::Gregorian => "gregorian",
                romcal_core::CalendarScope::Liturgical => "liturgical",
            }
            .to_string(),
            ascension_on_sunday: config.ascension_on_sunday,
            epiphany_on_sunday: config.epiphany_on_sunday,
            corpus_christi_on_sunday: config.corpus_christi_on_sunday,
        }
    }

    /// Convert to JSON value for serialization
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::json!({
            "locale": self.locale,
            "calendar": self.calendar,
            "easter_calculation_type": self.easter_calculation_type,
            "scope": self.scope,
            "ascension_on_sunday": self.ascension_on_sunday,
            "epiphany_on_sunday": self.epiphany_on_sunday,
            "corpus_christi_on_sunday": self.corpus_christi_on_sunday
        })
    }
}

/// Handle configuration display command
pub fn handle(
    calendar: Option<&str>,
    locale: Option<&str>,
    format: &str,
    scope: Option<&str>,
    easter_calculation_type: Option<&str>,
    ascension_on_sunday: Option<bool>,
    epiphany_on_sunday: Option<bool>,
    corpus_christi_on_sunday: Option<bool>,
) -> Result<(), RomcalCliError> {
    validate_format(format)?;

    let liturgical_config = create_liturgical_config(
        calendar,
        locale,
        scope,
        easter_calculation_type,
        ascension_on_sunday,
        corpus_christi_on_sunday,
        epiphany_on_sunday,
    )?;

    // Parse output format
    let output_format = match format.to_lowercase().as_str() {
        "json" => OutputFormat::Json,
        "csv" => OutputFormat::Csv,
        "yaml" => OutputFormat::Yaml,
        "lines" => OutputFormat::Lines,
        _ => unreachable!(), // Already validated above
    };

    // Create display data
    let config_data = ConfigDisplayData::from_liturgical_config(&liturgical_config);

    // Output based on format
    match output_format {
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&config_data.to_json_value())?
            );
        }
        OutputFormat::Csv => {
            println!("setting,value");
            println!("locale,{}", config_data.locale);
            println!("calendar,{}", config_data.calendar);
            println!(
                "easter_calculation_type,{}",
                config_data.easter_calculation_type
            );
            println!("scope,{}", config_data.scope);
            println!("ascension_on_sunday,{}", config_data.ascension_on_sunday);
            println!("epiphany_on_sunday,{}", config_data.epiphany_on_sunday);
            println!(
                "corpus_christi_on_sunday,{}",
                config_data.corpus_christi_on_sunday
            );
        }
        OutputFormat::Lines => {
            println!("locale: {}", config_data.locale);
            println!("calendar: {}", config_data.calendar);
            println!(
                "easter_calculation_type: {}",
                config_data.easter_calculation_type
            );
            println!("scope: {}", config_data.scope);
            println!("ascension_on_sunday: {}", config_data.ascension_on_sunday);
            println!("epiphany_on_sunday: {}", config_data.epiphany_on_sunday);
            println!(
                "corpus_christi_on_sunday: {}",
                config_data.corpus_christi_on_sunday
            );
        }
        OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(&config_data.to_json_value())?);
        }
    }

    Ok(())
}
