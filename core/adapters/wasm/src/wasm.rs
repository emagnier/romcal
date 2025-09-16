use romcal_core::preset::{Preset, PresetPartial};
use romcal_core::types::{CalendarContext, EasterCalculationType};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/// WASM-compatible configuration structure
#[wasm_bindgen]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RomcalConfig {
    inner: Preset,
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

    /// Build the configuration with defaults
    pub fn build(&self) -> RomcalConfig {
        let easter_type = match self.easter_calculation_type.as_deref() {
            Some("JULIAN") => Some(EasterCalculationType::Julian),
            _ => Some(EasterCalculationType::Gregorian),
        };

        let context = match self.context.as_deref() {
            Some("LITURGICAL") => Some(CalendarContext::Liturgical),
            _ => Some(CalendarContext::Gregorian),
        };

        let config = PresetPartial {
            calendar: self.calendar.clone(),
            locale: self.locale.clone(),
            easter_calculation_type: easter_type,
            context,
            epiphany_on_sunday: self.epiphany_on_sunday,
            corpus_christi_on_sunday: self.corpus_christi_on_sunday,
            ascension_on_sunday: self.ascension_on_sunday,
            calendar_definitions: None,
            resources: None,
        };

        RomcalConfig {
            inner: Preset::new(config),
        }
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
        self.inner.easter_calculation_type_to_wasm()
    }

    /// Get calendar context
    #[wasm_bindgen(getter)]
    pub fn context(&self) -> String {
        self.inner.context_to_wasm()
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
) -> Romcal {
    let mut partial_config = PartialRomcalConfig::new();
    partial_config.calendar = calendar;
    partial_config.locale = locale;
    partial_config.epiphany_on_sunday = epiphany_on_sunday;
    partial_config.corpus_christi_on_sunday = corpus_christi_on_sunday;
    partial_config.ascension_on_sunday = ascension_on_sunday;
    partial_config.easter_calculation_type = easter_calculation_type;
    partial_config.context = context;

    Romcal {
        config: partial_config.build(),
    }
}

/// Create a new Romcal instance with a partial configuration object
#[wasm_bindgen]
pub fn romcal_with_config_object(config: &PartialRomcalConfig) -> Romcal {
    Romcal {
        config: config.build(),
    }
}
