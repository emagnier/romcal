use romcal::engine::calendar_definition::CalendarDefinition;
use romcal::engine::resources::Resources;
use romcal::romcal::{Preset, Romcal as RomcalCore};
use romcal::search::EntityQuery;
use romcal::types::entity::{CanonizationLevel, EntityType, Sex, Title};
use romcal::types::{CalendarContext, EasterCalculationType};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// WASM-compatible configuration structure
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RomcalConfig {
    inner: RomcalCore,
}

/// Partial configuration structure for builder pattern
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PartialRomcalConfig {
    /// Calendar type (e.g., 'general_roman')
    calendar: Option<String>,
    /// Locale (e.g., 'en', 'fr')
    locale: Option<String>,
    /// Calendar context
    context: Option<String>,
    /// Easter calculation type
    easter_calculation_type: Option<String>,
    /// Epiphany is celebrated on a Sunday
    epiphany_on_sunday: Option<bool>,
    /// Ascension is celebrated on a Sunday
    ascension_on_sunday: Option<bool>,
    /// Corpus Christi is celebrated on a Sunday
    corpus_christi_on_sunday: Option<bool>,
    /// Calendar definitions as JSON string
    calendar_definitions_json: Option<String>,
    /// Resources as JSON string
    resources_json: Option<String>,
}

#[wasm_bindgen]
impl PartialRomcalConfig {
    /// Create a new partial configuration
    #[wasm_bindgen(constructor)]
    pub fn new() -> PartialRomcalConfig {
        Self::default()
    }

    /// Set calendar
    #[wasm_bindgen]
    pub fn set_calendar(&mut self, calendar: Option<String>) {
        self.calendar = calendar;
    }

    /// Set locale
    #[wasm_bindgen]
    pub fn set_locale(&mut self, locale: Option<String>) {
        self.locale = locale;
    }

    /// Set epiphany on Sunday
    #[wasm_bindgen]
    pub fn set_epiphany_on_sunday(&mut self, epiphany_on_sunday: Option<bool>) {
        self.epiphany_on_sunday = epiphany_on_sunday;
    }

    /// Set corpus Christi on Sunday
    #[wasm_bindgen]
    pub fn set_corpus_christi_on_sunday(&mut self, corpus_christi_on_sunday: Option<bool>) {
        self.corpus_christi_on_sunday = corpus_christi_on_sunday;
    }

    /// Set ascension on Sunday
    #[wasm_bindgen]
    pub fn set_ascension_on_sunday(&mut self, ascension_on_sunday: Option<bool>) {
        self.ascension_on_sunday = ascension_on_sunday;
    }

    /// Set easter calculation type
    #[wasm_bindgen]
    pub fn set_easter_calculation_type(&mut self, easter_calculation_type: Option<String>) {
        self.easter_calculation_type = easter_calculation_type;
    }

    /// Set calendar context
    #[wasm_bindgen]
    pub fn set_context(&mut self, context: Option<String>) {
        self.context = context;
    }

    /// Set calendar definitions from JSON string
    #[wasm_bindgen]
    pub fn set_calendar_definitions(&mut self, json: Option<String>) {
        self.calendar_definitions_json = json;
    }

    /// Set resources from JSON string
    #[wasm_bindgen]
    pub fn set_resources(&mut self, json: Option<String>) {
        self.resources_json = json;
    }

    /// Build the configuration with defaults
    pub fn build(&self) -> Result<RomcalConfig, JsValue> {
        let easter_type = match self.easter_calculation_type.as_deref() {
            Some("GREGORIAN") | None => Some(EasterCalculationType::Gregorian),
            Some("JULIAN") => Some(EasterCalculationType::Julian),
            Some(invalid) => {
                return Err(JsValue::from_str(&format!(
                    "Invalid easter_calculation_type: '{}'. Expected 'GREGORIAN' or 'JULIAN'",
                    invalid
                )));
            }
        };

        let context = match self.context.as_deref() {
            Some("GREGORIAN") | None => Some(CalendarContext::Gregorian),
            Some("LITURGICAL") => Some(CalendarContext::Liturgical),
            Some(invalid) => {
                return Err(JsValue::from_str(&format!(
                    "Invalid context: '{}'. Expected 'GREGORIAN' or 'LITURGICAL'",
                    invalid
                )));
            }
        };

        // Parse calendar definitions from JSON
        let calendar_definitions: Option<Vec<CalendarDefinition>> = self
            .calendar_definitions_json
            .as_ref()
            .and_then(|json| serde_json::from_str(json).ok());

        // Parse resources from JSON
        let resources: Option<Vec<Resources>> = self
            .resources_json
            .as_ref()
            .and_then(|json| serde_json::from_str(json).ok());

        let preset = Preset {
            calendar: self.calendar.clone(),
            locale: self.locale.clone(),
            easter_calculation_type: easter_type,
            context,
            epiphany_on_sunday: self.epiphany_on_sunday,
            corpus_christi_on_sunday: self.corpus_christi_on_sunday,
            ascension_on_sunday: self.ascension_on_sunday,
            ordinal_format: None,
            calendar_definitions,
            resources,
        };

        Ok(RomcalConfig {
            inner: RomcalCore::new(preset),
        })
    }
}

#[wasm_bindgen]
impl RomcalConfig {
    /// Create a new configuration with default values
    #[wasm_bindgen(constructor)]
    pub fn new() -> RomcalConfig {
        Self::default()
    }

    /// Get the calendar type
    #[wasm_bindgen(getter)]
    pub fn calendar(&self) -> String {
        self.inner.calendar.clone()
    }

    /// Get the locale
    #[wasm_bindgen(getter)]
    pub fn locale(&self) -> String {
        self.inner.locale.clone()
    }

    /// Get epiphany on Sunday setting
    #[wasm_bindgen(getter)]
    pub fn epiphany_on_sunday(&self) -> bool {
        self.inner.epiphany_on_sunday
    }

    /// Get corpus christi on Sunday setting
    #[wasm_bindgen(getter)]
    pub fn corpus_christi_on_sunday(&self) -> bool {
        self.inner.corpus_christi_on_sunday
    }

    /// Get ascension on Sunday setting
    #[wasm_bindgen(getter)]
    pub fn ascension_on_sunday(&self) -> bool {
        self.inner.ascension_on_sunday
    }

    /// Get easter calculation type
    #[wasm_bindgen(getter)]
    pub fn easter_calculation_type(&self) -> String {
        self.inner.easter_calculation_type.to_string()
    }

    /// Get calendar context
    #[wasm_bindgen(getter)]
    pub fn context(&self) -> String {
        self.inner.context.to_string()
    }

    /// Get the full config as a JSON string
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string(self).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

/// WASM-compatible Romcal instance
#[wasm_bindgen]
#[derive(Default)]
pub struct Romcal {
    config: RomcalConfig,
}

#[wasm_bindgen]
impl Romcal {
    /// Create a new Romcal instance with default configuration
    #[wasm_bindgen(constructor)]
    pub fn new() -> Romcal {
        Self::default()
    }

    /// Get the configuration
    #[wasm_bindgen(getter)]
    pub fn config(&self) -> RomcalConfig {
        self.config.clone()
    }

    /// Generate the complete liturgical calendar for a given liturgical year
    ///
    /// Returns a JSON string representing BTreeMap<String, Vec<LiturgicalDay>>
    /// where keys are dates in YYYY-MM-DD format
    #[wasm_bindgen(js_name = "generateLiturgicalCalendar")]
    pub fn generate_liturgical_calendar(&self, year: i32) -> Result<String, JsValue> {
        self.config
            .inner
            .generate_liturgical_calendar(year)
            .map(|calendar| serde_json::to_string(&calendar).unwrap_or_default())
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Generate a mass-centric view of the liturgical calendar for a given year
    ///
    /// Returns a JSON string representing BTreeMap<String, Vec<MassContext>>
    /// where keys are civil dates in YYYY-MM-DD format
    #[wasm_bindgen(js_name = "generateMassCalendar")]
    pub fn generate_mass_calendar(&self, year: i32) -> Result<String, JsValue> {
        self.config
            .inner
            .generate_mass_calendar(year)
            .map(|calendar| serde_json::to_string(&calendar).unwrap_or_default())
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Get a liturgical date by its ID for a given year
    ///
    /// Returns date in YYYY-MM-DD format
    #[wasm_bindgen(js_name = "getDate")]
    pub fn get_date(&self, id: &str, year: i32) -> Result<String, JsValue> {
        self.config
            .inner
            .get_date(id, year)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Get an entity by its exact ID.
    ///
    /// Returns the entity as a JSON string, or null if not found.
    #[wasm_bindgen(js_name = "getEntity")]
    pub fn get_entity(&self, id: &str) -> Option<String> {
        self.config
            .inner
            .get_entity(id)
            .and_then(|entity| serde_json::to_string(&entity).ok())
    }

    /// Search entities with fuzzy matching and filters.
    ///
    /// # Arguments
    ///
    /// * `query_json` - A JSON object with search parameters:
    ///   - `text`: Optional fuzzy text search
    ///   - `entityType`: Optional filter ('PERSON', 'PLACE', 'EVENT')
    ///   - `canonizationLevel`: Optional filter ('SAINT', 'BLESSED')
    ///   - `sex`: Optional filter ('MALE', 'FEMALE')
    ///   - `titles`: Optional array of title strings
    ///   - `limit`: Optional maximum results (default: 20)
    ///   - `minScore`: Optional minimum score 0.0-1.0 (default: 0.3)
    ///
    /// # Returns
    ///
    /// A JSON array of search results sorted by score (highest first).
    #[wasm_bindgen(js_name = "searchEntities")]
    pub fn search_entities(&self, query_json: &str) -> Result<String, JsValue> {
        // Parse the query from JSON
        let query: WasmEntityQuery = serde_json::from_str(query_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid query JSON: {}", e)))?;

        // Convert to core query
        let core_query = query.to_core().map_err(|e| JsValue::from_str(&e))?;

        // Execute search
        let results = self.config.inner.search_entities(core_query);

        // Convert results to JSON-serializable format
        let wasm_results: Vec<WasmEntitySearchResult> = results
            .into_iter()
            .map(|r| WasmEntitySearchResult {
                entity: r.entity,
                score: r.score,
                match_type: r.match_type.to_string(),
                matched_fields: r.matched_fields,
            })
            .collect();

        serde_json::to_string(&wasm_results)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize results: {}", e)))
    }
}

/// WASM-compatible entity query for JSON deserialization
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WasmEntityQuery {
    text: Option<String>,
    entity_type: Option<String>,
    canonization_level: Option<String>,
    sex: Option<String>,
    titles: Option<Vec<String>>,
    limit: Option<usize>,
    min_score: Option<f64>,
}

impl WasmEntityQuery {
    fn to_core(&self) -> Result<EntityQuery, String> {
        let entity_type = self
            .entity_type
            .as_ref()
            .map(|s| {
                s.parse::<EntityType>()
                    .map_err(|_| format!("Invalid entityType: '{}'", s))
            })
            .transpose()?;

        let canonization_level = self
            .canonization_level
            .as_ref()
            .map(|s| {
                s.parse::<CanonizationLevel>()
                    .map_err(|_| format!("Invalid canonizationLevel: '{}'", s))
            })
            .transpose()?;

        let sex = self
            .sex
            .as_ref()
            .map(|s| {
                s.parse::<Sex>()
                    .map_err(|_| format!("Invalid sex: '{}'", s))
            })
            .transpose()?;

        let titles = self
            .titles
            .as_ref()
            .map(|titles| {
                titles
                    .iter()
                    .map(|s| {
                        serde_json::from_str::<Title>(&format!("\"{}\"", s))
                            .map_err(|_| format!("Invalid title: '{}'", s))
                    })
                    .collect::<Result<Vec<Title>, String>>()
            })
            .transpose()?;

        Ok(EntityQuery {
            text: self.text.clone(),
            entity_type,
            canonization_level,
            sex,
            titles,
            limit: self.limit,
            min_score: self.min_score,
        })
    }
}

/// WASM-compatible entity search result for JSON serialization
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct WasmEntitySearchResult {
    entity: romcal::types::entity::Entity,
    score: f64,
    match_type: String,
    matched_fields: Vec<String>,
}

/// Create a new Romcal instance (default configuration)
#[wasm_bindgen]
pub fn romcal() -> Romcal {
    Romcal::new()
}

/// Create a new Romcal instance with partial configuration
#[wasm_bindgen]
pub fn romcal_with_partial_config(
    calendar: Option<String>,
    locale: Option<String>,
    epiphany_on_sunday: Option<bool>,
    corpus_christi_on_sunday: Option<bool>,
    ascension_on_sunday: Option<bool>,
    easter_calculation_type: Option<String>,
    context: Option<String>,
) -> Result<Romcal, JsValue> {
    let mut partial_config = PartialRomcalConfig::new();
    partial_config.calendar = calendar;
    partial_config.locale = locale;
    partial_config.epiphany_on_sunday = epiphany_on_sunday;
    partial_config.corpus_christi_on_sunday = corpus_christi_on_sunday;
    partial_config.ascension_on_sunday = ascension_on_sunday;
    partial_config.easter_calculation_type = easter_calculation_type;
    partial_config.context = context;

    Ok(Romcal {
        config: partial_config.build()?,
    })
}

/// Create a new Romcal instance with a partial configuration object
#[wasm_bindgen]
pub fn romcal_with_config_object(config: &PartialRomcalConfig) -> Result<Romcal, JsValue> {
    Ok(Romcal {
        config: config.build()?,
    })
}

/// Get the romcal library version
#[wasm_bindgen]
pub fn version() -> String {
    romcal::VERSION.to_string()
}

/// Merge multiple resource files (meta.json + entities.*.json) into a single Resources JSON.
///
/// # Arguments
///
/// * `locale` - The locale code (e.g., "fr", "en")
/// * `files_json` - A JS array of JSON strings, each representing a resource file
///
/// # Returns
///
/// A JSON string representing the merged Resources object.
#[wasm_bindgen]
pub fn merge_resource_files(locale: &str, files_json: Vec<String>) -> Result<String, JsValue> {
    let files_refs: Vec<&str> = files_json.iter().map(|s| s.as_str()).collect();
    let resources = romcal::merge_resource_files(locale, files_refs)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    serde_json::to_string(&resources).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Merge/validate multiple calendar definition files.
///
/// # Arguments
///
/// * `files_json` - A JS array of JSON strings, each representing a calendar definition
///
/// # Returns
///
/// A JSON string representing an array of CalendarDefinition objects.
#[wasm_bindgen]
pub fn merge_calendar_definitions(files_json: Vec<String>) -> Result<String, JsValue> {
    let files_refs: Vec<&str> = files_json.iter().map(|s| s.as_str()).collect();
    let definitions = romcal::merge_calendar_definitions(files_refs)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    serde_json::to_string(&definitions).map_err(|e| JsValue::from_str(&e.to_string()))
}
