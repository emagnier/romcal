use romcal::engine::calendar_definition::CalendarDefinition;
use romcal::engine::resources::Resources;
use romcal::romcal::{Preset, Romcal as RomcalCore};
use romcal::types::{CalendarContext, EasterCalculationType};
use serde::de::DeserializeOwned;
use std::sync::Arc;

uniffi::setup_scaffolding!();

/// Parse an optional JSON string into a typed value
fn parse_json<T: DeserializeOwned>(
    json: &Option<String>,
    field_name: &str,
) -> Result<Option<T>, RomcalError> {
    json.as_ref()
        .map(|s| {
            serde_json::from_str(s).map_err(|e| {
                RomcalError::ParseError(format!("Failed to parse {}: {}", field_name, e))
            })
        })
        .transpose()
}

/// Error type for Romcal operations
#[derive(Debug, thiserror::Error, uniffi::Error)]
pub enum RomcalError {
    #[error("Invalid year: {0}")]
    InvalidYear(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfig(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Calculation error: {0}")]
    CalculationError(String),
}

impl From<romcal::error::RomcalError> for RomcalError {
    fn from(err: romcal::error::RomcalError) -> Self {
        use romcal::error::RomcalError as CoreError;
        match err {
            CoreError::InvalidYear(y) => RomcalError::InvalidYear(format!(
                "{} (must be >= 1583 for the Gregorian calendar)",
                y
            )),
            CoreError::InvalidDateName(name) => RomcalError::NotFound(name),
            CoreError::InvalidConfig => {
                RomcalError::InvalidConfig("Invalid configuration".to_string())
            }
            CoreError::ValidationError(msg) => RomcalError::InvalidConfig(msg),
            CoreError::InvalidDate
            | CoreError::CalculationError
            | CoreError::DateConversionError => RomcalError::CalculationError(err.to_string()),
        }
    }
}

/// Configuration for Romcal
#[derive(Debug, Clone, Default, uniffi::Record)]
pub struct RomcalConfig {
    /// Calendar type (e.g., 'general_roman', 'france')
    pub calendar: Option<String>,
    /// Locale (e.g., 'en', 'fr')
    pub locale: Option<String>,
    /// Epiphany is celebrated on a Sunday
    pub epiphany_on_sunday: Option<bool>,
    /// Ascension is celebrated on a Sunday
    pub ascension_on_sunday: Option<bool>,
    /// Corpus Christi is celebrated on a Sunday
    pub corpus_christi_on_sunday: Option<bool>,
    /// Easter calculation type ('GREGORIAN' or 'JULIAN')
    pub easter_calculation_type: Option<String>,
    /// Calendar context ('GREGORIAN' or 'LITURGICAL')
    pub context: Option<String>,
    /// Calendar definitions as JSON string
    pub calendar_definitions_json: Option<String>,
    /// Resources as JSON string
    pub resources_json: Option<String>,
}

/// Main Romcal instance
#[derive(uniffi::Object)]
pub struct Romcal {
    inner: RomcalCore,
}

#[uniffi::export]
impl Romcal {
    /// Create a new Romcal instance with optional configuration
    #[uniffi::constructor]
    pub fn new(config: Option<RomcalConfig>) -> Result<Arc<Self>, RomcalError> {
        let config = config.unwrap_or_default();

        // Validate and parse easter_calculation_type
        let easter_type = match config.easter_calculation_type.as_deref() {
            Some("JULIAN") => Some(EasterCalculationType::Julian),
            Some("GREGORIAN") | None => Some(EasterCalculationType::Gregorian),
            Some(invalid) => {
                return Err(RomcalError::InvalidConfig(format!(
                    "Invalid easter_calculation_type: '{}'. Expected 'GREGORIAN' or 'JULIAN'",
                    invalid
                )));
            }
        };

        // Validate and parse context
        let context = match config.context.as_deref() {
            Some("LITURGICAL") => Some(CalendarContext::Liturgical),
            Some("GREGORIAN") | None => Some(CalendarContext::Gregorian),
            Some(invalid) => {
                return Err(RomcalError::InvalidConfig(format!(
                    "Invalid context: '{}'. Expected 'GREGORIAN' or 'LITURGICAL'",
                    invalid
                )));
            }
        };

        let calendar_definitions: Option<Vec<CalendarDefinition>> = parse_json(
            &config.calendar_definitions_json,
            "calendar_definitions_json",
        )?;
        let resources: Option<Vec<Resources>> =
            parse_json(&config.resources_json, "resources_json")?;

        let preset = Preset {
            calendar: config.calendar,
            locale: config.locale,
            easter_calculation_type: easter_type,
            context,
            epiphany_on_sunday: config.epiphany_on_sunday,
            corpus_christi_on_sunday: config.corpus_christi_on_sunday,
            ascension_on_sunday: config.ascension_on_sunday,
            ordinal_format: None,
            calendar_definitions,
            resources,
        };

        Ok(Arc::new(Self {
            inner: RomcalCore::new(preset),
        }))
    }

    /// Get the calendar type
    pub fn get_calendar(&self) -> String {
        self.inner.calendar.clone()
    }

    /// Get the locale
    pub fn get_locale(&self) -> String {
        self.inner.locale.clone()
    }

    /// Get epiphany on Sunday setting
    pub fn get_epiphany_on_sunday(&self) -> bool {
        self.inner.epiphany_on_sunday
    }

    /// Get ascension on Sunday setting
    pub fn get_ascension_on_sunday(&self) -> bool {
        self.inner.ascension_on_sunday
    }

    /// Get corpus christi on Sunday setting
    pub fn get_corpus_christi_on_sunday(&self) -> bool {
        self.inner.corpus_christi_on_sunday
    }

    /// Get easter calculation type
    pub fn get_easter_calculation_type(&self) -> String {
        self.inner.easter_calculation_type.to_string()
    }

    /// Get calendar context
    pub fn get_context(&self) -> String {
        self.inner.context.to_string()
    }

    /// Generate the complete liturgical calendar for a given liturgical year
    ///
    /// Returns a JSON string representing BTreeMap<String, Vec<LiturgicalDay>>
    /// where keys are dates in YYYY-MM-DD format
    pub fn generate_liturgical_calendar(&self, year: i32) -> Result<String, RomcalError> {
        let calendar = self.inner.generate_liturgical_calendar(year)?;
        serde_json::to_string(&calendar)
            .map_err(|e| RomcalError::ParseError(format!("Failed to serialize calendar: {}", e)))
    }

    /// Generate a mass-centric view of the liturgical calendar for a given year
    ///
    /// Returns a JSON string representing BTreeMap<String, Vec<MassContext>>
    /// where keys are civil dates in YYYY-MM-DD format
    pub fn generate_mass_calendar(&self, year: i32) -> Result<String, RomcalError> {
        let calendar = self.inner.generate_mass_calendar(year)?;
        serde_json::to_string(&calendar)
            .map_err(|e| RomcalError::ParseError(format!("Failed to serialize calendar: {}", e)))
    }

    /// Get a liturgical date by its ID for a given year
    ///
    /// Returns date in YYYY-MM-DD format
    pub fn get_date(&self, id: &str, year: i32) -> Result<String, RomcalError> {
        self.inner.get_date(id, year).map_err(RomcalError::from)
    }
}
