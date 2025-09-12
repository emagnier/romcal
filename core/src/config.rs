use serde::{Deserialize, Serialize};

use crate::resources::ResourcesDefinition;
use crate::{
    calendar_def::CalendarDefinition,
    types::{CalendarScope, EasterCalculationType},
};

// Default configuration constants
const DEFAULT_CALENDAR: &str = "general_roman";
const DEFAULT_LOCALE: &str = "en";
const DEFAULT_EASTER_TYPE: EasterCalculationType = EasterCalculationType::Gregorian;
const DEFAULT_SCOPE: CalendarScope = CalendarScope::Gregorian;
const DEFAULT_EPIPHANY_ON_SUNDAY: bool = false;
const DEFAULT_CORPUS_CHRISTI_ON_SUNDAY: bool = true;
const DEFAULT_ASCENSION_ON_SUNDAY: bool = false;

/// Complete configuration for liturgical date calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiturgicalConfig {
    /// Calendar type (e.g., 'general_roman', 'france', 'united_states')
    pub calendar: String,
    /// Locale (e.g., 'en', 'fr', 'es')
    pub locale: String,
    /// Easter calculation type
    pub easter_calculation_type: EasterCalculationType,
    /// Calendar scope
    pub scope: CalendarScope,
    /// Epiphany is celebrated on a Sunday (between January 2-8)
    pub epiphany_on_sunday: bool,
    /// Corpus Christi is celebrated on a Sunday
    pub corpus_christi_on_sunday: bool,
    /// Ascension is celebrated on a Sunday (7th Sunday of Easter)
    pub ascension_on_sunday: bool,
    /// Array of calendar definitions
    pub calendar_definitions: Vec<CalendarDefinition>,
    /// Array of resources definitions
    pub resources: Vec<ResourcesDefinition>,
}

impl Default for LiturgicalConfig {
    fn default() -> Self {
        Self {
            calendar: DEFAULT_CALENDAR.to_string(),
            locale: DEFAULT_LOCALE.to_string(),
            easter_calculation_type: DEFAULT_EASTER_TYPE,
            scope: DEFAULT_SCOPE,
            epiphany_on_sunday: DEFAULT_EPIPHANY_ON_SUNDAY,
            corpus_christi_on_sunday: DEFAULT_CORPUS_CHRISTI_ON_SUNDAY,
            ascension_on_sunday: DEFAULT_ASCENSION_ON_SUNDAY,
            calendar_definitions: Vec::new(),
            resources: Vec::new(),
        }
    }
}

impl LiturgicalConfig {
    /// Creates a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a new configuration with calendar and locale
    pub fn with_calendar_and_locale(calendar: &str, locale: &str) -> Self {
        Self::new_with_custom_flags(
            calendar,
            locale,
            DEFAULT_EPIPHANY_ON_SUNDAY,
            DEFAULT_CORPUS_CHRISTI_ON_SUNDAY,
            DEFAULT_ASCENSION_ON_SUNDAY,
        )
    }

    /// Creates a custom configuration
    pub fn custom(
        calendar: &str,
        locale: &str,
        easter_calculation_type: EasterCalculationType,
        scope: CalendarScope,
        epiphany_on_sunday: bool,
        corpus_christi_on_sunday: bool,
        ascension_on_sunday: bool,
    ) -> Self {
        Self {
            calendar: calendar.to_string(),
            locale: locale.to_string(),
            easter_calculation_type,
            scope,
            epiphany_on_sunday,
            corpus_christi_on_sunday,
            ascension_on_sunday,
            calendar_definitions: Vec::new(),
            resources: Vec::new(),
        }
    }

    /// Creates a configuration with optional values, using defaults for empty strings
    pub fn with_optional_values(
        calendar: Option<&str>,
        locale: Option<&str>,
        easter_calculation_type: EasterCalculationType,
        scope: CalendarScope,
        epiphany_on_sunday: Option<bool>,
        corpus_christi_on_sunday: Option<bool>,
        ascension_on_sunday: Option<bool>,
    ) -> Self {
        Self {
            calendar: calendar.unwrap_or(DEFAULT_CALENDAR).to_string(),
            locale: locale.unwrap_or(DEFAULT_LOCALE).to_string(),
            easter_calculation_type,
            scope,
            epiphany_on_sunday: epiphany_on_sunday.unwrap_or(DEFAULT_EPIPHANY_ON_SUNDAY),
            corpus_christi_on_sunday: corpus_christi_on_sunday
                .unwrap_or(DEFAULT_CORPUS_CHRISTI_ON_SUNDAY),
            ascension_on_sunday: ascension_on_sunday.unwrap_or(DEFAULT_ASCENSION_ON_SUNDAY),
            calendar_definitions: Vec::new(),
            resources: Vec::new(),
        }
    }

    /// Creates a configuration from WASM string values
    pub fn from_wasm_strings(
        calendar: Option<&str>,
        locale: Option<&str>,
        easter_calculation_type: Option<&str>,
        scope: Option<&str>,
        epiphany_on_sunday: Option<bool>,
        corpus_christi_on_sunday: Option<bool>,
        ascension_on_sunday: Option<bool>,
    ) -> Self {
        let easter_type = match easter_calculation_type {
            Some("JULIAN") => EasterCalculationType::Julian,
            _ => EasterCalculationType::Gregorian,
        };

        let scope = match scope {
            Some("LITURGICAL") => CalendarScope::Liturgical,
            _ => CalendarScope::Gregorian,
        };

        Self::with_optional_values(
            calendar,
            locale,
            easter_type,
            scope,
            epiphany_on_sunday,
            corpus_christi_on_sunday,
            ascension_on_sunday,
        )
    }

    /// Converts easter calculation type to WASM string
    pub fn easter_calculation_type_to_wasm(&self) -> String {
        match self.easter_calculation_type {
            EasterCalculationType::Gregorian => "GREGORIAN".to_string(),
            EasterCalculationType::Julian => "JULIAN".to_string(),
        }
    }

    /// Converts calendar scope to WASM string
    pub fn scope_to_wasm(&self) -> String {
        match self.scope {
            CalendarScope::Gregorian => "GREGORIAN".to_string(),
            CalendarScope::Liturgical => "LITURGICAL".to_string(),
        }
    }

    /// Creates a configuration with custom flags using default values for other fields
    fn new_with_custom_flags(
        calendar: &str,
        locale: &str,
        epiphany_on_sunday: bool,
        corpus_christi_on_sunday: bool,
        ascension_on_sunday: bool,
    ) -> Self {
        Self {
            calendar: calendar.to_string(),
            locale: locale.to_string(),
            easter_calculation_type: DEFAULT_EASTER_TYPE,
            scope: DEFAULT_SCOPE,
            epiphany_on_sunday,
            corpus_christi_on_sunday,
            ascension_on_sunday,
            calendar_definitions: Vec::new(),
            resources: Vec::new(),
        }
    }

    /// Configuration for standard Roman rite
    pub fn roman_rite() -> Self {
        Self::new_with_custom_flags(
            DEFAULT_CALENDAR,
            DEFAULT_LOCALE,
            DEFAULT_EPIPHANY_ON_SUNDAY,
            DEFAULT_CORPUS_CHRISTI_ON_SUNDAY,
            DEFAULT_ASCENSION_ON_SUNDAY,
        )
    }

    /// Configuration for United States (Ascension and Corpus Christi on Sunday)
    pub fn united_states() -> Self {
        Self::new_with_custom_flags(
            "united_states",
            DEFAULT_LOCALE,
            DEFAULT_EPIPHANY_ON_SUNDAY,
            DEFAULT_CORPUS_CHRISTI_ON_SUNDAY,
            true,
        )
    }

    /// Configuration for France
    pub fn france() -> Self {
        Self::new_with_custom_flags(
            "france",
            "fr",
            DEFAULT_EPIPHANY_ON_SUNDAY,
            DEFAULT_CORPUS_CHRISTI_ON_SUNDAY,
            DEFAULT_ASCENSION_ON_SUNDAY,
        )
    }

    /// Configuration for countries where Epiphany is celebrated on a Sunday
    pub fn epiphany_on_sunday() -> Self {
        Self::new_with_custom_flags(
            DEFAULT_CALENDAR,
            DEFAULT_LOCALE,
            true,
            DEFAULT_CORPUS_CHRISTI_ON_SUNDAY,
            DEFAULT_ASCENSION_ON_SUNDAY,
        )
    }

    /// Add a calendar definition to the configuration
    pub fn add_calendar_definition(&mut self, calendar_def: CalendarDefinition) {
        self.calendar_definitions.push(calendar_def);
    }

    /// Add multiple calendar definitions to the configuration
    pub fn add_calendar_definitions(&mut self, calendar_defs: Vec<CalendarDefinition>) {
        self.calendar_definitions.extend(calendar_defs);
    }

    /// Get a calendar definition by ID
    pub fn get_calendar_definition(&self, id: &str) -> Option<&CalendarDefinition> {
        self.calendar_definitions.iter().find(|def| def.id == id)
    }

    /// Add a resources definition to the configuration
    pub fn add_resources(&mut self, resources: ResourcesDefinition) {
        self.resources.push(resources);
    }

    /// Add multiple resources definitions to the configuration
    pub fn add_resources_definitions(&mut self, resources: Vec<ResourcesDefinition>) {
        self.resources.extend(resources);
    }

    /// Get a resources definition by locale
    pub fn get_resources(&self, locale: &str) -> Option<&ResourcesDefinition> {
        self.resources.iter().find(|res| res.locale == locale)
    }

    /// Clear all calendar definitions
    pub fn clear_calendar_definitions(&mut self) {
        self.calendar_definitions.clear();
    }

    /// Clear all resources definitions
    pub fn clear_resources(&mut self) {
        self.resources.clear();
    }
}
