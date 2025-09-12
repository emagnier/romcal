use crate::error::RomcalCliError;
use chrono::Datelike;
use romcal_core::{CalendarScope, EasterCalculationType, LiturgicalConfig};

/// Default configuration for the CLI
pub struct CliConfig {
    pub default_locale: String,
    pub default_calendar: String,
    pub default_scope: CalendarScope,
    pub supported_locales: Vec<String>,
    pub supported_calendars: Vec<String>,
}

impl Default for CliConfig {
    fn default() -> Self {
        Self {
            default_locale: "en".to_string(),
            default_calendar: "general_roman".to_string(),
            default_scope: CalendarScope::Gregorian,
            supported_locales: vec![
                "en".to_string(),
                "fr".to_string(),
                "es".to_string(),
                "de".to_string(),
                "it".to_string(),
                "la".to_string(),
                "pl".to_string(),
                "pt-br".to_string(),
                "sk".to_string(),
                "ta".to_string(),
                "cs".to_string(),
                "en-gb".to_string(),
                "en-ie".to_string(),
            ],
            supported_calendars: vec![
                "general_roman".to_string(),
                "france".to_string(),
                "united_states".to_string(),
                "spain".to_string(),
                "germany".to_string(),
                "italy".to_string(),
                "poland".to_string(),
                "slovakia".to_string(),
                "czech_republic".to_string(),
            ],
        }
    }
}

impl CliConfig {
    /// Create a liturgical configuration from CLI parameters
    pub fn create_liturgical_config(
        &self,
        calendar: Option<&str>,
        locale: Option<&str>,
        easter_type: Option<&str>,
        ascension_on_sunday: Option<bool>,
        corpus_christi_on_sunday: Option<bool>,
        epiphany_on_sunday: Option<bool>,
    ) -> Result<LiturgicalConfig, RomcalCliError> {
        let calendar = calendar.unwrap_or(&self.default_calendar);
        let locale = locale.unwrap_or(&self.default_locale);

        // Validate locale
        if !self.supported_locales.contains(&locale.to_string()) {
            return Err(RomcalCliError::unsupported_locale(locale));
        }

        // Validate calendar
        if !self.supported_calendars.contains(&calendar.to_string()) {
            return Err(RomcalCliError::calendar_not_found(calendar));
        }

        // Parse Easter calculation type
        let easter_calculation_type = match easter_type.unwrap_or("gregorian") {
            "gregorian" => EasterCalculationType::Gregorian,
            "julian" => EasterCalculationType::Julian,
            _ => {
                return Err(RomcalCliError::invalid_calculation_type(
                    easter_type.unwrap(),
                ))
            }
        };

        // Create configuration
        let config = LiturgicalConfig {
            calendar: calendar.to_string(),
            locale: locale.to_string(),
            easter_calculation_type,
            scope: self.default_scope,
            epiphany_on_sunday: epiphany_on_sunday.unwrap_or(false),
            corpus_christi_on_sunday: corpus_christi_on_sunday.unwrap_or(true),
            ascension_on_sunday: ascension_on_sunday.unwrap_or(false),
            calendar_definitions: vec![],
            resources: vec![],
        };

        Ok(config)
    }

    /// Get current year
    pub fn current_year() -> i32 {
        chrono::Utc::now().year()
    }

    /// Validate a year
    pub fn validate_year(year: i32) -> Result<(), RomcalCliError> {
        if year < 1583 {
            Err(RomcalCliError::invalid_year(year))
        } else {
            Ok(())
        }
    }

    /// Get list of supported locales
    pub fn get_supported_locales(&self) -> &Vec<String> {
        &self.supported_locales
    }

    /// Get list of supported calendars
    pub fn get_supported_calendars(&self) -> &Vec<String> {
        &self.supported_calendars
    }
}
