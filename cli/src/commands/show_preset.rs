use crate::enums::OutputFormat;
use crate::error::RomcalCliError;
use romcal::Romcal;
use serde_json;
use serde_saphyr;

/// Configuration data for display
#[derive(Debug, Clone)]
struct PresetDisplayData {
    locale: String,
    calendar: String,
    context: String,
    easter_calculation_type: String,
    epiphany_on_sunday: bool,
    ascension_on_sunday: bool,
    corpus_christi_on_sunday: bool,
}

impl PresetDisplayData {
    /// Create from romcal instance
    fn from_romcal(romcal: &romcal::Romcal) -> Self {
        Self {
            locale: romcal.locale.clone(),
            calendar: romcal.calendar.clone(),
            context: match romcal.context {
                romcal::CalendarContext::Gregorian => "gregorian",
                romcal::CalendarContext::Liturgical => "liturgical",
            }
            .to_string(),
            easter_calculation_type: match romcal.easter_calculation_type {
                romcal::EasterCalculationType::Gregorian => "gregorian",
                romcal::EasterCalculationType::Julian => "julian",
            }
            .to_string(),
            epiphany_on_sunday: romcal.epiphany_on_sunday,
            ascension_on_sunday: romcal.ascension_on_sunday,
            corpus_christi_on_sunday: romcal.corpus_christi_on_sunday,
        }
    }

    /// Convert to JSON value for serialization
    fn to_json_value(&self) -> serde_json::Value {
        serde_json::json!({
            "locale": self.locale,
            "calendar": self.calendar,
            "context": self.context,
            "easter_calculation_type": self.easter_calculation_type,
            "epiphany_on_sunday": self.epiphany_on_sunday,
            "ascension_on_sunday": self.ascension_on_sunday,
            "corpus_christi_on_sunday": self.corpus_christi_on_sunday
        })
    }
}

/// Handle configuration display command
pub fn handle(output_format: OutputFormat, romcal: Romcal) -> Result<(), RomcalCliError> {
    // Create display data
    let preset_data = PresetDisplayData::from_romcal(&romcal);

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
            println!("context,{}", preset_data.context);
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
            println!("context: {}", preset_data.context);
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
            let yaml = serde_saphyr::to_string(&preset_data.to_json_value()).map_err(|e| {
                RomcalCliError::config_error(format!("Failed to serialize preset to YAML: {}", e))
            })?;
            println!("{}", yaml);
        }
    }

    Ok(())
}
