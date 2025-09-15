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

/// Partial configuration for liturgical date calculations (input)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LiturgicalConfigPartial {
    /// Calendar type (e.g., 'general_roman', 'france', 'united_states')
    pub calendar: Option<String>,
    /// Locale (e.g., 'en', 'fr', 'es')
    pub locale: Option<String>,
    /// Calendar scope
    pub scope: Option<CalendarScope>,
    /// Easter calculation type
    pub easter_calculation_type: Option<EasterCalculationType>,
    /// Epiphany is celebrated on a Sunday (between January 2-8)
    pub epiphany_on_sunday: Option<bool>,
    /// Ascension is celebrated on a Sunday (7th Sunday of Easter)
    pub ascension_on_sunday: Option<bool>,
    /// Corpus Christi is celebrated on a Sunday
    pub corpus_christi_on_sunday: Option<bool>,
    /// Array of calendar definitions
    pub calendar_definitions: Option<Vec<CalendarDefinition>>,
    /// Array of resources definitions
    pub resources: Option<Vec<ResourcesDefinition>>,
}

/// Complete configuration for liturgical date calculations (output)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiturgicalConfig {
    /// Calendar type (e.g., 'general_roman', 'france', 'united_states')
    pub calendar: String,
    /// Locale (e.g., 'en', 'fr', 'es')
    pub locale: String,
    /// Calendar scope
    pub scope: CalendarScope,
    /// Epiphany is celebrated on a Sunday (between January 2-8)
    pub epiphany_on_sunday: bool,
    /// Ascension is celebrated on a Sunday (7th Sunday of Easter)
    pub ascension_on_sunday: bool,
    /// Corpus Christi is celebrated on a Sunday
    pub corpus_christi_on_sunday: bool,
    /// Easter calculation type
    pub easter_calculation_type: EasterCalculationType,
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
            scope: DEFAULT_SCOPE,
            easter_calculation_type: DEFAULT_EASTER_TYPE,
            epiphany_on_sunday: DEFAULT_EPIPHANY_ON_SUNDAY,
            corpus_christi_on_sunday: DEFAULT_CORPUS_CHRISTI_ON_SUNDAY,
            ascension_on_sunday: DEFAULT_ASCENSION_ON_SUNDAY,
            calendar_definitions: Vec::new(),
            resources: Vec::new(),
        }
    }
}

impl LiturgicalConfig {
    /// Creates a new configuration with default values applied to any None fields
    pub fn new(config: LiturgicalConfigPartial) -> Self {
        Self {
            calendar: config
                .calendar
                .unwrap_or_else(|| DEFAULT_CALENDAR.to_string()),
            locale: config.locale.unwrap_or_else(|| DEFAULT_LOCALE.to_string()),
            scope: config.scope.unwrap_or(DEFAULT_SCOPE),
            easter_calculation_type: config
                .easter_calculation_type
                .unwrap_or(DEFAULT_EASTER_TYPE),
            epiphany_on_sunday: config
                .epiphany_on_sunday
                .unwrap_or(DEFAULT_EPIPHANY_ON_SUNDAY),
            ascension_on_sunday: config
                .ascension_on_sunday
                .unwrap_or(DEFAULT_ASCENSION_ON_SUNDAY),
            corpus_christi_on_sunday: config
                .corpus_christi_on_sunday
                .unwrap_or(DEFAULT_CORPUS_CHRISTI_ON_SUNDAY),
            calendar_definitions: config.calendar_definitions.unwrap_or_default(),
            resources: config.resources.unwrap_or_default(),
        }
    }

    /// Converts calendar scope to WASM string
    pub fn scope_to_wasm(&self) -> String {
        match self.scope {
            CalendarScope::Gregorian => "GREGORIAN".to_string(),
            CalendarScope::Liturgical => "LITURGICAL".to_string(),
        }
    }

    /// Converts easter calculation type to WASM string
    pub fn easter_calculation_type_to_wasm(&self) -> String {
        match self.easter_calculation_type {
            EasterCalculationType::Gregorian => "GREGORIAN".to_string(),
            EasterCalculationType::Julian => "JULIAN".to_string(),
        }
    }

    /// Get a calendar definition by ID
    pub fn get_calendar_definition(&self, id: &str) -> Option<&CalendarDefinition> {
        self.calendar_definitions.iter().find(|def| def.id == id)
    }

    /// Get a resources definition by locale
    pub fn get_resources(&self, locale: &str) -> Option<&ResourcesDefinition> {
        self.resources.iter().find(|res| res.locale == locale)
    }

    /// Add a calendar definition to the configuration
    pub fn add_calendar_definition(&mut self, calendar_def: CalendarDefinition) {
        self.calendar_definitions.push(calendar_def);
    }

    /// Add a resources definition to the configuration
    pub fn add_resources(&mut self, resources: ResourcesDefinition) {
        self.resources.push(resources);
    }

    /// Create a JSON bundle of the current configuration
    /// This method serializes the LiturgicalConfig to JSON format
    /// and removes null values and empty objects from the output
    pub fn create_bundle(&self) -> Result<String, serde_json::Error> {
        crate::bundle::create_bundle(self)
            .map_err(|e| serde_json::Error::io(std::io::Error::other(e.to_string())))
    }
}
