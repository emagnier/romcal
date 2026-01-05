use romcal::engine::calendar_definition::CalendarDefinition;
use romcal::engine::resources::Resources;
use romcal::entity_search::{EntityQuery as CoreEntityQuery, MatchType as CoreMatchType};
use romcal::romcal::{Preset, Romcal as RomcalCore};
use romcal::types::entity::{CanonizationLevel, EntityType, Sex, Title};
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
            CoreError::InvalidYear(_, _) => RomcalError::InvalidYear(err.to_string()),
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

// ============================================================================
// Entity Search Types
// ============================================================================

/// Type of match that was found for a search result.
#[derive(Debug, Clone, PartialEq, Eq, uniffi::Enum)]
pub enum MatchType {
    /// Exact ID match (score = 1.0)
    ExactId,
    /// Fuzzy match on text fields (score < 1.0)
    Fuzzy,
    /// Match by filters only (no text query provided)
    FilterOnly,
}

impl From<CoreMatchType> for MatchType {
    fn from(m: CoreMatchType) -> Self {
        match m {
            CoreMatchType::ExactId => MatchType::ExactId,
            CoreMatchType::Fuzzy => MatchType::Fuzzy,
            CoreMatchType::FilterOnly => MatchType::FilterOnly,
        }
    }
}

/// Query parameters for searching entities.
#[derive(Debug, Clone, Default, uniffi::Record)]
pub struct EntityQuery {
    /// Fuzzy text search on id, fullname, and name fields.
    pub text: Option<String>,
    /// Filter by entity type ('PERSON', 'PLACE', 'EVENT').
    pub entity_type: Option<String>,
    /// Filter by canonization level ('SAINT', 'BLESSED').
    pub canonization_level: Option<String>,
    /// Filter by sex ('MALE', 'FEMALE').
    pub sex: Option<String>,
    /// Filter by titles (entity must have at least one).
    pub titles: Option<Vec<String>>,
    /// Maximum number of results (default: 20).
    pub limit: Option<u32>,
    /// Minimum score threshold 0.0-1.0 (default: 0.3).
    pub min_score: Option<f64>,
}

impl EntityQuery {
    /// Convert to core EntityQuery
    fn to_core(&self) -> Result<CoreEntityQuery, RomcalError> {
        let entity_type = self
            .entity_type
            .as_ref()
            .map(|s| {
                s.parse::<EntityType>().map_err(|_| {
                    RomcalError::InvalidConfig(format!("Invalid entity_type: '{}'", s))
                })
            })
            .transpose()?;

        let canonization_level = self
            .canonization_level
            .as_ref()
            .map(|s| {
                s.parse::<CanonizationLevel>().map_err(|_| {
                    RomcalError::InvalidConfig(format!("Invalid canonization_level: '{}'", s))
                })
            })
            .transpose()?;

        let sex = self
            .sex
            .as_ref()
            .map(|s| {
                s.parse::<Sex>()
                    .map_err(|_| RomcalError::InvalidConfig(format!("Invalid sex: '{}'", s)))
            })
            .transpose()?;

        let titles = self
            .titles
            .as_ref()
            .map(|titles| {
                titles
                    .iter()
                    .map(|s| {
                        serde_json::from_str::<Title>(&format!("\"{}\"", s)).map_err(|_| {
                            RomcalError::InvalidConfig(format!("Invalid title: '{}'", s))
                        })
                    })
                    .collect::<Result<Vec<Title>, RomcalError>>()
            })
            .transpose()?;

        Ok(CoreEntityQuery {
            text: self.text.clone(),
            entity_type,
            canonization_level,
            sex,
            titles,
            limit: self.limit.map(|l| l as usize),
            min_score: self.min_score,
        })
    }
}

/// Result of an entity search.
#[derive(Debug, Clone, uniffi::Record)]
pub struct EntitySearchResult {
    /// The matched entity as JSON string.
    pub entity_json: String,
    /// Match score from 0.0 to 1.0.
    pub score: f64,
    /// Type of match that was found.
    pub match_type: MatchType,
    /// Names of fields that matched the query.
    pub matched_fields: Vec<String>,
}

// ============================================================================
// Romcal Configuration
// ============================================================================

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

    /// Get an entity by its exact ID.
    ///
    /// Returns the entity as a JSON string, or None if not found.
    pub fn get_entity(&self, id: &str) -> Option<String> {
        self.inner
            .get_entity(id)
            .and_then(|entity| serde_json::to_string(&entity).ok())
    }

    /// Search entities with fuzzy matching and filters.
    ///
    /// Returns a list of search results sorted by score (highest first).
    pub fn search_entities(
        &self,
        query: EntityQuery,
    ) -> Result<Vec<EntitySearchResult>, RomcalError> {
        let core_query = query.to_core()?;
        let results = self.inner.search_entities(core_query);

        results
            .into_iter()
            .map(|r| {
                let entity_json = serde_json::to_string(&r.entity).map_err(|e| {
                    RomcalError::ParseError(format!("Failed to serialize entity: {}", e))
                })?;
                Ok(EntitySearchResult {
                    entity_json,
                    score: r.score,
                    match_type: r.match_type.into(),
                    matched_fields: r.matched_fields,
                })
            })
            .collect()
    }
}

/// Get the romcal library version
#[uniffi::export]
pub fn version() -> String {
    romcal::VERSION.to_string()
}

/// Merge multiple resource files (meta.json + entities.*.json) into a single Resources JSON.
///
/// This helper allows you to load resource files however you want and then
/// merge them into the expected structure.
///
/// # Arguments
///
/// * `locale` - The locale code (e.g., "fr", "en")
/// * `files_json` - A list of JSON strings, each representing a resource file
///
/// # Returns
///
/// A JSON string representing the merged Resources object.
#[uniffi::export]
pub fn merge_resource_files(
    locale: String,
    files_json: Vec<String>,
) -> Result<String, RomcalError> {
    let files_refs: Vec<&str> = files_json.iter().map(|s| s.as_str()).collect();
    let resources = romcal::merge_resource_files(&locale, files_refs)?;
    serde_json::to_string(&resources)
        .map_err(|e| RomcalError::ParseError(format!("Failed to serialize resources: {}", e)))
}

/// Merge/validate multiple calendar definition files.
///
/// This helper allows you to load calendar definition files however you want
/// and then validate them into the expected structure.
///
/// # Arguments
///
/// * `files_json` - A list of JSON strings, each representing a calendar definition
///
/// # Returns
///
/// A JSON string representing an array of CalendarDefinition objects.
#[uniffi::export]
pub fn merge_calendar_definitions(files_json: Vec<String>) -> Result<String, RomcalError> {
    let files_refs: Vec<&str> = files_json.iter().map(|s| s.as_str()).collect();
    let definitions = romcal::merge_calendar_definitions(files_refs)?;
    serde_json::to_string(&definitions)
        .map_err(|e| RomcalError::ParseError(format!("Failed to serialize definitions: {}", e)))
}
