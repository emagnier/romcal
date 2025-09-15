use crate::enums::OutputFormat;
use crate::error::RomcalCliError;
use romcal_core::Preset;
use serde_json;
use serde_yaml;

/// Configuration data for display
#[derive(Debug, Clone)]
struct PresetDisplayData {
    locale: String,
    calendar: String,
    scope: String,
    easter_calculation_type: String,
    epiphany_on_sunday: bool,
    ascension_on_sunday: bool,
    corpus_christi_on_sunday: bool,
}

impl PresetDisplayData {
    /// Create from preset
    fn from_preset(preset: &romcal_core::Preset) -> Self {
        Self {
            locale: preset.locale.clone(),
            calendar: preset.calendar.clone(),
            scope: match preset.scope {
                romcal_core::CalendarScope::Gregorian => "gregorian",
                romcal_core::CalendarScope::Liturgical => "liturgical",
            }
            .to_string(),
            easter_calculation_type: match preset.easter_calculation_type {
                romcal_core::EasterCalculationType::Gregorian => "gregorian",
                romcal_core::EasterCalculationType::Julian => "julian",
            }
            .to_string(),
            epiphany_on_sunday: preset.epiphany_on_sunday,
            ascension_on_sunday: preset.ascension_on_sunday,
            corpus_christi_on_sunday: preset.corpus_christi_on_sunday,
        }
    }

    /// Convert to JSON value for serialization
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::json!({
            "locale": self.locale,
            "calendar": self.calendar,
            "scope": self.scope,
            "easter_calculation_type": self.easter_calculation_type,
            "epiphany_on_sunday": self.epiphany_on_sunday,
            "ascension_on_sunday": self.ascension_on_sunday,
            "corpus_christi_on_sunday": self.corpus_christi_on_sunday
        })
    }
}

/// Handle configuration display command
pub fn handle(output_format: OutputFormat, preset: Preset) -> Result<(), RomcalCliError> {
    // Create display data
    let preset_data = PresetDisplayData::from_preset(&preset);

    // Output based on format
    match output_format {
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&preset_data.to_json_value())?
            );
        }
        OutputFormat::Csv => {
            println!("setting,value");
            println!("locale,{}", preset_data.locale);
            println!("calendar,{}", preset_data.calendar);
            println!("scope,{}", preset_data.scope);
            println!(
                "easter_calculation_type,{}",
                preset_data.easter_calculation_type
            );
            println!("epiphany_on_sunday,{}", preset_data.epiphany_on_sunday);
            println!("ascension_on_sunday,{}", preset_data.ascension_on_sunday);
            println!(
                "corpus_christi_on_sunday,{}",
                preset_data.corpus_christi_on_sunday
            );
        }
        OutputFormat::Lines => {
            println!("locale: {}", preset_data.locale);
            println!("calendar: {}", preset_data.calendar);
            println!("scope: {}", preset_data.scope);
            println!(
                "easter_calculation_type: {}",
                preset_data.easter_calculation_type
            );
            println!("epiphany_on_sunday: {}", preset_data.epiphany_on_sunday);
            println!("ascension_on_sunday: {}", preset_data.ascension_on_sunday);
            println!(
                "corpus_christi_on_sunday: {}",
                preset_data.corpus_christi_on_sunday
            );
        }
        OutputFormat::Yaml => {
            println!("{}", serde_yaml::to_string(&preset_data.to_json_value())?);
        }
    }

    Ok(())
}
