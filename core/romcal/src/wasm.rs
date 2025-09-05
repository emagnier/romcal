use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

// Default value constants
const DEFAULT_CALENDAR: &str = "general_roman";
const DEFAULT_LOCALE: &str = "en";
const DEFAULT_EASTER_TYPE: &str = "GREGORIAN";

/// WASM-compatible configuration structure
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RomcalConfig {
    /// Calendar type (e.g., 'general_roman')
    calendar: String,
    /// Locale (e.g., 'en', 'fr')
    locale: String,
    /// Epiphany is celebrated on a Sunday
    epiphany_on_sunday: bool,
    /// Corpus Christi is celebrated on a Sunday
    corpus_christi_on_sunday: bool,
    /// Ascension is celebrated on a Sunday
    ascension_on_sunday: bool,
    /// Easter calculation type
    easter_calculation_type: String,
}

/// WASM-compatible partial configuration structure (builder pattern)
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartialRomcalConfig {
    /// Calendar type (e.g., 'general_roman')
    calendar: Option<String>,
    /// Locale (e.g., 'en', 'fr')
    locale: Option<String>,
    /// Epiphany is celebrated on a Sunday
    epiphany_on_sunday: Option<bool>,
    /// Corpus Christi is celebrated on a Sunday
    corpus_christi_on_sunday: Option<bool>,
    /// Ascension is celebrated on a Sunday
    ascension_on_sunday: Option<bool>,
    /// Easter calculation type
    easter_calculation_type: Option<String>,
}

#[wasm_bindgen]
impl PartialRomcalConfig {
    /// Create a new partial configuration
    #[wasm_bindgen(constructor)]
    pub fn new() -> PartialRomcalConfig {
        PartialRomcalConfig {
            calendar: None,
            locale: None,
            epiphany_on_sunday: None,
            corpus_christi_on_sunday: None,
            ascension_on_sunday: None,
            easter_calculation_type: None,
        }
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

    /// Validate calendar name
    #[wasm_bindgen]
    pub fn validate_calendar(&self) -> bool {
        if let Some(ref calendar) = self.calendar {
            !calendar.is_empty()
                && calendar
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_')
        } else {
            true
        }
    }

    /// Validate locale code
    #[wasm_bindgen]
    pub fn validate_locale(&self) -> bool {
        if let Some(ref locale) = self.locale {
            !locale.is_empty()
                && locale.len() <= 5
                && locale.chars().all(|c| c.is_ascii_alphabetic() || c == '-')
        } else {
            true
        }
    }

    /// Build the configuration with defaults
    pub fn build(&self) -> RomcalConfig {
        RomcalConfig {
            calendar: self
                .calendar
                .clone()
                .unwrap_or_else(|| DEFAULT_CALENDAR.to_string()),
            locale: self
                .locale
                .clone()
                .unwrap_or_else(|| DEFAULT_LOCALE.to_string()),
            epiphany_on_sunday: self.epiphany_on_sunday.unwrap_or(false),
            corpus_christi_on_sunday: self.corpus_christi_on_sunday.unwrap_or(false),
            ascension_on_sunday: self.ascension_on_sunday.unwrap_or(false),
            easter_calculation_type: self
                .easter_calculation_type
                .clone()
                .unwrap_or_else(|| DEFAULT_EASTER_TYPE.to_string()),
        }
    }
}

#[wasm_bindgen]
impl RomcalConfig {
    /// Get the calendar type
    #[wasm_bindgen(getter)]
    pub fn calendar(&self) -> String {
        self.calendar.clone()
    }

    /// Get the locale
    #[wasm_bindgen(getter)]
    pub fn locale(&self) -> String {
        self.locale.clone()
    }

    /// Get epiphany on Sunday setting
    #[wasm_bindgen(getter)]
    pub fn epiphany_on_sunday(&self) -> bool {
        self.epiphany_on_sunday
    }

    /// Get corpus christi on Sunday setting
    #[wasm_bindgen(getter)]
    pub fn corpus_christi_on_sunday(&self) -> bool {
        self.corpus_christi_on_sunday
    }

    /// Get ascension on Sunday setting
    #[wasm_bindgen(getter)]
    pub fn ascension_on_sunday(&self) -> bool {
        self.ascension_on_sunday
    }

    /// Get easter calculation type
    #[wasm_bindgen(getter)]
    pub fn easter_calculation_type(&self) -> String {
        self.easter_calculation_type.clone()
    }

    /// Get the full config as a JSON string
    #[wasm_bindgen(js_name = "toJSON")]
    pub fn to_json(&self) -> Result<String, JsValue> {
        serde_json::to_string(self).map_err(|e| JsValue::from_str(&e.to_string()))
    }
}

/// WASM-compatible Romcal instance
#[wasm_bindgen]
pub struct Romcal {
    config: RomcalConfig,
}

#[wasm_bindgen]
impl Romcal {
    /// Create a new Romcal instance with default configuration
    #[wasm_bindgen(constructor)]
    pub fn new() -> Romcal {
        Romcal {
            config: RomcalConfig::default(),
        }
    }

    /// Create a new Romcal instance with custom calendar and locale
    #[wasm_bindgen]
    pub fn with_calendar_and_locale(calendar: &str, locale: &str) -> Romcal {
        Romcal {
            config: RomcalConfig::new(calendar, locale),
        }
    }

    /// Create a new Romcal instance with full configuration
    #[wasm_bindgen]
    pub fn with_full_config(
        calendar: &str,
        locale: &str,
        epiphany_on_sunday: bool,
        corpus_christi_on_sunday: bool,
        ascension_on_sunday: bool,
        easter_calculation_type: &str,
    ) -> Romcal {
        Romcal {
            config: RomcalConfig::with_full_config(
                calendar,
                locale,
                epiphany_on_sunday,
                corpus_christi_on_sunday,
                ascension_on_sunday,
                easter_calculation_type,
            ),
        }
    }

    /// Get the configuration
    #[wasm_bindgen(getter)]
    pub fn config(&self) -> RomcalConfig {
        self.config.clone()
    }
}

impl Default for RomcalConfig {
    fn default() -> Self {
        Self {
            calendar: DEFAULT_CALENDAR.to_string(),
            locale: DEFAULT_LOCALE.to_string(),
            epiphany_on_sunday: false,
            corpus_christi_on_sunday: false,
            ascension_on_sunday: false,
            easter_calculation_type: DEFAULT_EASTER_TYPE.to_string(),
        }
    }
}

impl RomcalConfig {
    pub fn new(calendar: &str, locale: &str) -> Self {
        Self {
            calendar: calendar.to_string(),
            locale: locale.to_string(),
            epiphany_on_sunday: false,
            corpus_christi_on_sunday: false,
            ascension_on_sunday: false,
            easter_calculation_type: DEFAULT_EASTER_TYPE.to_string(),
        }
    }

    pub fn with_full_config(
        calendar: &str,
        locale: &str,
        epiphany_on_sunday: bool,
        corpus_christi_on_sunday: bool,
        ascension_on_sunday: bool,
        easter_calculation_type: &str,
    ) -> Self {
        Self {
            calendar: calendar.to_string(),
            locale: locale.to_string(),
            epiphany_on_sunday,
            corpus_christi_on_sunday,
            ascension_on_sunday,
            easter_calculation_type: easter_calculation_type.to_string(),
        }
    }
}

/// Create a new Romcal instance (default configuration)
#[wasm_bindgen]
pub fn romcal() -> Romcal {
    Romcal::new()
}

/// Create a new Romcal instance with calendar and locale
#[wasm_bindgen]
pub fn romcal_with_config(calendar: &str, locale: &str) -> Romcal {
    Romcal::with_calendar_and_locale(calendar, locale)
}

/// Create a new Romcal instance with partial configuration object
/// All parameters are optional and will use default values if not provided
#[wasm_bindgen]
pub fn romcal_with_partial_config(
    calendar: Option<String>,
    locale: Option<String>,
    epiphany_on_sunday: Option<bool>,
    corpus_christi_on_sunday: Option<bool>,
    ascension_on_sunday: Option<bool>,
    easter_calculation_type: Option<String>,
) -> Romcal {
    let mut config = PartialRomcalConfig::new();

    if let Some(calendar) = calendar {
        config.set_calendar(Some(calendar));
    }
    if let Some(locale) = locale {
        config.set_locale(Some(locale));
    }
    if let Some(epiphany_on_sunday) = epiphany_on_sunday {
        config.set_epiphany_on_sunday(Some(epiphany_on_sunday));
    }
    if let Some(corpus_christi_on_sunday) = corpus_christi_on_sunday {
        config.set_corpus_christi_on_sunday(Some(corpus_christi_on_sunday));
    }
    if let Some(ascension_on_sunday) = ascension_on_sunday {
        config.set_ascension_on_sunday(Some(ascension_on_sunday));
    }
    if let Some(easter_calculation_type) = easter_calculation_type {
        config.set_easter_calculation_type(Some(easter_calculation_type));
    }

    Romcal {
        config: config.build(),
    }
}

/// Create a new Romcal instance with a partial configuration object
/// This is the main function that should be used from TypeScript
#[wasm_bindgen]
pub fn romcal_with_config_object(config: &PartialRomcalConfig) -> Romcal {
    Romcal {
        config: config.build(),
    }
}
