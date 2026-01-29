use romcal::engine::calendar_definition::CalendarDefinition;
use romcal::engine::resources::Resources;
use romcal::martyrology_search::MartyrologyQuery;
use romcal::romcal::{Preset, Romcal as RomcalCore};
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
            Some("gregorian") | None => Some(EasterCalculationType::Gregorian),
            Some("julian") => Some(EasterCalculationType::Julian),
            Some(invalid) => {
                return Err(JsValue::from_str(&format!(
                    "Invalid easter_calculation_type: '{}'. Expected 'gregorian' or 'julian'",
                    invalid
                )));
            }
        };

        let context = match self.context.as_deref() {
            Some("gregorian") | None => Some(CalendarContext::Gregorian),
            Some("liturgical") => Some(CalendarContext::Liturgical),
            Some(invalid) => {
                return Err(JsValue::from_str(&format!(
                    "Invalid context: '{}'. Expected 'gregorian' or 'liturgical'",
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

        let inner = RomcalCore::new(preset).map_err(|e| JsValue::from_str(&e.to_string()))?;
        Ok(RomcalConfig { inner })
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
    inner: RomcalConfig,
}

#[wasm_bindgen]
impl Romcal {
    /// Create a new Romcal instance with default configuration
    #[wasm_bindgen(constructor)]
    pub fn new() -> Romcal {
        Self::default()
    }

    /// Get the calendar type
    #[wasm_bindgen(getter)]
    pub fn calendar(&self) -> String {
        self.inner.inner.calendar.clone()
    }

    /// Get the locale
    #[wasm_bindgen(getter)]
    pub fn locale(&self) -> String {
        self.inner.inner.locale.clone()
    }

    /// Get epiphany on Sunday setting
    #[wasm_bindgen(getter, js_name = "epiphanyOnSunday")]
    pub fn epiphany_on_sunday(&self) -> bool {
        self.inner.inner.epiphany_on_sunday
    }

    /// Get corpus christi on Sunday setting
    #[wasm_bindgen(getter, js_name = "corpusChristiOnSunday")]
    pub fn corpus_christi_on_sunday(&self) -> bool {
        self.inner.inner.corpus_christi_on_sunday
    }

    /// Get ascension on Sunday setting
    #[wasm_bindgen(getter, js_name = "ascensionOnSunday")]
    pub fn ascension_on_sunday(&self) -> bool {
        self.inner.inner.ascension_on_sunday
    }

    /// Get easter calculation type
    #[wasm_bindgen(getter, js_name = "easterCalculationType")]
    pub fn easter_calculation_type(&self) -> String {
        self.inner.inner.easter_calculation_type.to_string()
    }

    /// Get calendar context
    #[wasm_bindgen(getter)]
    pub fn context(&self) -> String {
        self.inner.inner.context.to_string()
    }

    /// Generate the complete liturgical calendar for a given liturgical year
    ///
    /// Returns a JSON string representing BTreeMap<String, Vec<LiturgicalDay>>
    /// where keys are dates in YYYY-MM-DD format
    #[wasm_bindgen(js_name = "generateLiturgicalCalendar")]
    pub fn generate_liturgical_calendar(&self, year: i32) -> Result<String, JsValue> {
        self.inner
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
        self.inner
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
        self.inner
            .inner
            .get_date(id, year)
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }

    /// Get a martyrology entry by its exact ID.
    ///
    /// Returns the entry as a JSON string, or null if not found.
    #[wasm_bindgen(js_name = "getMartyrologyEntry")]
    pub fn get_martyrology_entry(&self, id: &str) -> Option<String> {
        self.inner
            .inner
            .get_martyrology_entry(id)
            .and_then(|entry| serde_json::to_string(&entry).ok())
    }

    /// Search martyrology entries with fuzzy matching and filters.
    ///
    /// # Arguments
    ///
    /// * `query_json` - A JSON object with search parameters:
    ///   - `text`: Optional fuzzy text search
    ///   - `entryType`: Optional filter ('person', 'place', 'event')
    ///   - `canonizationLevel`: Optional filter ('saint', 'blessed', etc.)
    ///   - `sex`: Optional filter ('male', 'female')
    ///   - `titles`: Optional array of title strings
    ///   - `limit`: Optional maximum results (default: 20)
    ///   - `minScore`: Optional minimum score 0.0-1.0 (default: 0.3)
    ///
    /// # Returns
    ///
    /// A JSON array of search results sorted by score (highest first).
    #[wasm_bindgen(js_name = "searchMartyrologyEntries")]
    pub fn search_martyrology(&self, query_json: &str) -> Result<String, JsValue> {
        // Parse the query from JSON directly into core type
        let query: MartyrologyQuery = serde_json::from_str(query_json)
            .map_err(|e| JsValue::from_str(&format!("Invalid query JSON: {}", e)))?;

        // Execute search
        let results = self.inner.inner.search_martyrology(query);

        // Serialize results directly (MartyrologySearchResult now has Serialize)
        serde_json::to_string(&results)
            .map_err(|e| JsValue::from_str(&format!("Failed to serialize results: {}", e)))
    }

    /// Create an optimized JSON bundle of the current configuration.
    ///
    /// This method filters and deduplicates the configuration to create a minimal
    /// bundle suitable for distribution. The output contains:
    ///
    /// - Only calendar definitions in the hierarchy (general_roman → parents → main)
    /// - Only resources for locales in the hierarchy (en → parent → specific)
    /// - Property-level deduplication across locale hierarchy
    /// - No null values or empty objects
    ///
    /// # Returns
    ///
    /// A pretty-printed JSON string of the optimized configuration.
    #[wasm_bindgen(js_name = "createBundle")]
    pub fn create_bundle(&self) -> Result<String, JsValue> {
        self.inner
            .inner
            .create_bundle()
            .map_err(|e| JsValue::from_str(&e.to_string()))
    }
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
        inner: partial_config.build()?,
    })
}

/// Create a new Romcal instance with a partial configuration object
#[wasm_bindgen]
pub fn romcal_with_config_object(config: &PartialRomcalConfig) -> Result<Romcal, JsValue> {
    Ok(Romcal {
        inner: config.build()?,
    })
}

/// Get the romcal library version
#[wasm_bindgen]
pub fn version() -> String {
    romcal::VERSION.to_string()
}

/// Merge multiple resource files (meta.json + martyrology.*.json) into a single Resources JSON.
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
