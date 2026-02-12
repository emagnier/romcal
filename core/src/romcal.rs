//! Core Romcal configuration and instance management.
//!
//! This module provides the main `Romcal` struct and `Preset` configuration
//! for initializing and customizing liturgical calendar generation.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::engine::calendar_definition::CalendarDefinition;
use crate::engine::dates::LiturgicalDates;
use crate::engine::resources::Resources;
use crate::error::RomcalError;
use crate::martyrology_resolution::{MartyrologyResolver, normalize_locale};
use crate::martyrology_search::{MartyrologyMatcher, MartyrologyQuery, MartyrologySearchResult};
use crate::types::martyrology::MartyrologyEntry;
use crate::types::{CalendarContext, EasterCalculationType, OrdinalFormat};

// Default configuration constants
const DEFAULT_CALENDAR: &str = "general_roman";
/// Calendar value indicating only the temporal cycle (proper of time) should be used,
/// without any calendar definitions (general roman, country, diocese, etc.).
pub const TEMPORAL_CYCLE: &str = "temporal_cycle";
const DEFAULT_LOCALE: &str = "en";
const DEFAULT_EASTER_TYPE: EasterCalculationType = EasterCalculationType::Gregorian;
const DEFAULT_CONTEXT: CalendarContext = CalendarContext::Gregorian;
const DEFAULT_EPIPHANY_ON_SUNDAY: bool = false;
const DEFAULT_CORPUS_CHRISTI_ON_SUNDAY: bool = true;
const DEFAULT_ASCENSION_ON_SUNDAY: bool = false;
const DEFAULT_ORDINAL_FORMAT: OrdinalFormat = OrdinalFormat::Numeric;

/// Configuration for romcal
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Preset {
    /// Calendar type (e.g., 'general_roman', 'france', 'united_states')
    pub calendar: Option<String>,
    /// Locale (e.g., 'en', 'fr', 'es')
    pub locale: Option<String>,
    /// Calendar context
    pub context: Option<CalendarContext>,
    /// Easter calculation type
    pub easter_calculation_type: Option<EasterCalculationType>,
    /// Epiphany is celebrated on a Sunday (between January 2-8)
    pub epiphany_on_sunday: Option<bool>,
    /// Ascension is celebrated on a Sunday (7th Sunday of Easter)
    pub ascension_on_sunday: Option<bool>,
    /// Corpus Christi is celebrated on a Sunday
    pub corpus_christi_on_sunday: Option<bool>,
    /// Format for displaying ordinal numbers (letters or numeric)
    pub ordinal_format: Option<OrdinalFormat>,
    /// Array of calendar definitions
    pub calendar_definitions: Option<Vec<CalendarDefinition>>,
    /// Array of resources definitions
    pub resources: Option<Vec<Resources>>,
}

/// Main romcal instance for generating liturgical calendars
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Romcal {
    /// Calendar type (e.g., 'general_roman', 'france', 'united_states')
    pub calendar: String,
    /// Locale (e.g., 'en', 'fr', 'es')
    pub locale: String,
    /// Calendar context
    pub context: CalendarContext,
    /// Epiphany is celebrated on a Sunday (between January 2-8)
    pub epiphany_on_sunday: bool,
    /// Ascension is celebrated on a Sunday (7th Sunday of Easter)
    pub ascension_on_sunday: bool,
    /// Corpus Christi is celebrated on a Sunday
    pub corpus_christi_on_sunday: bool,
    /// Easter calculation type
    pub easter_calculation_type: EasterCalculationType,
    /// Format for displaying ordinal numbers (letters or numeric)
    pub ordinal_format: OrdinalFormat,
    /// Array of calendar definitions
    pub calendar_definitions: Vec<CalendarDefinition>,
    /// Array of resources definitions
    pub resources: Vec<Resources>,
}

impl Default for Romcal {
    fn default() -> Self {
        Self {
            calendar: DEFAULT_CALENDAR.to_string(),
            locale: DEFAULT_LOCALE.to_string(),
            context: DEFAULT_CONTEXT,
            easter_calculation_type: DEFAULT_EASTER_TYPE,
            epiphany_on_sunday: DEFAULT_EPIPHANY_ON_SUNDAY,
            corpus_christi_on_sunday: DEFAULT_CORPUS_CHRISTI_ON_SUNDAY,
            ascension_on_sunday: DEFAULT_ASCENSION_ON_SUNDAY,
            ordinal_format: DEFAULT_ORDINAL_FORMAT,
            calendar_definitions: Self::default_calendar_definitions(),
            resources: Self::default_resources(),
        }
    }
}

impl Romcal {
    /// Returns bundled calendar definitions if the `bundled-data` feature is enabled.
    fn default_calendar_definitions() -> Vec<CalendarDefinition> {
        #[cfg(feature = "bundled-data")]
        {
            crate::bundled_data::get_all_calendar_definitions().unwrap_or_default()
        }
        #[cfg(not(feature = "bundled-data"))]
        {
            Vec::new()
        }
    }

    /// Returns bundled resources if the `bundled-data` feature is enabled.
    fn default_resources() -> Vec<Resources> {
        #[cfg(feature = "bundled-data")]
        {
            crate::bundled_data::get_all_resources().unwrap_or_default()
        }
        #[cfg(not(feature = "bundled-data"))]
        {
            Vec::new()
        }
    }

    /// Creates a new Romcal instance with default values applied to any None fields
    ///
    /// Priority for ordinal_format:
    /// 1. Value from Preset (highest priority)
    /// 2. Value from ResourcesMetadata of the target locale
    /// 3. Default value (Numeric)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The requested calendar is not found in `calendar_definitions`
    /// - The requested locale (or its fallback chain) is not found in `resources`
    pub fn new(config: Preset) -> Result<Self, RomcalError> {
        // None = use bundled data if available, Some(...) = use exactly what was provided
        let calendar_definitions = match config.calendar_definitions {
            Some(defs) => defs,
            None => Self::default_calendar_definitions(),
        };

        // Normalize locale to lowercase (BCP 47 is case-insensitive)
        let locale = normalize_locale(config.locale.as_deref().unwrap_or(DEFAULT_LOCALE));

        // None = use bundled data if available, Some(...) = use exactly what was provided
        let resources: Vec<Resources> = match config.resources {
            Some(res) => res,
            None => Self::default_resources(),
        }
        .into_iter()
        .map(|mut res| {
            res.locale = normalize_locale(&res.locale);
            res
        })
        .collect();

        // Get the calendar ID (use default if not provided)
        let calendar = config
            .calendar
            .unwrap_or_else(|| DEFAULT_CALENDAR.to_string());

        // Validate calendar exists in definitions
        // Skip if no definitions provided or if using temporal_cycle (proper of time only)
        if !calendar_definitions.is_empty()
            && calendar != TEMPORAL_CYCLE
            && !calendar_definitions.iter().any(|def| def.id == calendar)
        {
            let available: Vec<String> =
                calendar_definitions.iter().map(|d| d.id.clone()).collect();
            return Err(RomcalError::CalendarNotFound(calendar, available));
        }

        // Validate locale exists in resources (skip if no resources provided)
        // Check for exact match or base locale fallback (e.g., "fr" for "fr-ca")
        // No implicit fallback to DEFAULT_LOCALE - user must explicitly provide the locale
        if !resources.is_empty() {
            let base_locale = locale.split('-').next().unwrap_or(&locale);
            let locale_found = resources
                .iter()
                .any(|res| res.locale == locale || res.locale == base_locale);
            if !locale_found {
                let available: Vec<String> = resources.iter().map(|r| r.locale.clone()).collect();
                return Err(RomcalError::LocaleNotFound(locale.to_string(), available));
            }
        }

        // Get ordinal_format from locale's ResourcesMetadata if not set in Preset
        let ordinal_format_from_locale = resources
            .iter()
            .find(|res| res.locale == locale)
            .and_then(|res| res.metadata.as_ref())
            .and_then(|meta| meta.ordinal_format);

        Ok(Self {
            calendar,
            locale: locale.to_string(),
            context: config.context.unwrap_or(DEFAULT_CONTEXT),
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
            ordinal_format: config
                .ordinal_format
                .or(ordinal_format_from_locale)
                .unwrap_or(DEFAULT_ORDINAL_FORMAT),
            calendar_definitions,
            resources,
        })
    }

    /// Creates a new Romcal instance with empty calendar definitions and resources.
    /// Uses `temporal_cycle` as the default calendar (proper of time only).
    pub fn empty() -> Self {
        Self {
            calendar: TEMPORAL_CYCLE.to_string(),
            locale: DEFAULT_LOCALE.to_string(),
            context: DEFAULT_CONTEXT,
            easter_calculation_type: DEFAULT_EASTER_TYPE,
            epiphany_on_sunday: DEFAULT_EPIPHANY_ON_SUNDAY,
            corpus_christi_on_sunday: DEFAULT_CORPUS_CHRISTI_ON_SUNDAY,
            ascension_on_sunday: DEFAULT_ASCENSION_ON_SUNDAY,
            ordinal_format: DEFAULT_ORDINAL_FORMAT,
            calendar_definitions: Vec::new(),
            resources: Vec::new(),
        }
    }

    /// Get a calendar definition by ID
    pub fn get_calendar_definition(&self, id: &str) -> Option<&CalendarDefinition> {
        self.calendar_definitions.iter().find(|def| def.id == id)
    }

    /// Get a resources definition by locale
    pub fn get_resources(&self, locale: &str) -> Option<&Resources> {
        self.resources.iter().find(|res| res.locale == locale)
    }

    /// Add a calendar definition to the configuration
    pub fn add_calendar_definition(&mut self, calendar_def: CalendarDefinition) {
        self.calendar_definitions.push(calendar_def);
    }

    /// Add a resources definition to the configuration.
    ///
    /// If resources for the same locale already exist, the new entries are
    /// merged into the existing resource (martyrology entries are added or
    /// replaced, metadata is overwritten when provided). This prevents
    /// duplicate locale entries that would be invisible to `get_resources`.
    pub fn add_resources(&mut self, mut resources: Resources) {
        resources.locale = normalize_locale(&resources.locale);
        if let Some(existing) = self
            .resources
            .iter_mut()
            .find(|r| r.locale == resources.locale)
        {
            // Merge martyrology entries into existing resource
            if let Some(new_martyrology) = resources.martyrology {
                let existing_martyrology = existing.martyrology.get_or_insert_with(BTreeMap::new);
                for (id, entry) in new_martyrology {
                    existing_martyrology.insert(id, entry);
                }
            }
            // Merge metadata if provided
            if resources.metadata.is_some() {
                existing.metadata = resources.metadata;
            }
        } else {
            self.resources.push(resources);
        }
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
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Duplicate calendar IDs or locales are found
    /// - Required calendars or locales are missing
    /// - JSON serialization fails
    pub fn create_bundle(&self) -> Result<String, serde_json::Error> {
        crate::engine::bundle::bundle(self)
            .map_err(|e| serde_json::Error::io(std::io::Error::other(e.to_string())))
    }

    /// Generate the complete liturgical calendar for a given liturgical year
    ///
    /// This method combines the Proper of Time with particular calendars
    /// and applies precedence rules according to GNLY #49.
    ///
    /// # Arguments
    ///
    /// * `year` - The liturgical year (e.g., 2026 for liturgical year 2025-2026)
    ///
    /// # Returns
    ///
    /// A BTreeMap of date strings (YYYY-MM-DD) to vectors of LiturgicalDay objects
    ///
    /// # Errors
    ///
    /// Returns an error if the year is invalid or if there's a calculation error
    pub fn generate_liturgical_calendar(
        &self,
        year: i32,
    ) -> crate::RomcalResult<crate::engine::calendar::LiturgicalCalendar> {
        crate::engine::calendar::Calendar::new(self.clone(), year)?.generate()
    }

    /// Generate a mass-centric view of the liturgical calendar for a given year
    ///
    /// Unlike `generate_liturgical_calendar()` which groups by liturgical date,
    /// this method groups by civil date and mass time. Evening masses
    /// (EasterVigil, PreviousEveningMass) appear on the PREVIOUS civil day.
    ///
    /// # Arguments
    ///
    /// * `year` - The liturgical year (e.g., 2026 for liturgical year 2025-2026)
    ///
    /// # Returns
    ///
    /// A BTreeMap of civil date strings (YYYY-MM-DD) to vectors of MassContext objects
    ///
    /// # Errors
    ///
    /// Returns an error if the year is invalid or if there's a calculation error
    pub fn generate_mass_calendar(
        &self,
        year: i32,
    ) -> crate::RomcalResult<crate::types::mass::MassCalendar> {
        crate::engine::calendar::Calendar::new(self.clone(), year)?.generate_mass_calendar()
    }

    /// Get a martyrology entry by its exact ID.
    ///
    /// Searches in the current locale's resources first, then falls back to other locales.
    ///
    /// # Arguments
    ///
    /// * `id` - The martyrology entry ID (e.g., "francis_of_assisi")
    ///
    /// # Returns
    ///
    /// The entry if found, or `None` if not found.
    ///
    /// Uses locale fallback: en → parent locale → specific locale
    pub fn get_martyrology_entry(&self, id: &str) -> Option<MartyrologyEntry> {
        let resolver = MartyrologyResolver::new(self);
        resolver.resolve_entry(id).cloned()
    }

    /// Search martyrology entries with fuzzy matching and filters.
    ///
    /// Searches entries merged from all locales (en → parent → specific).
    ///
    /// # Arguments
    ///
    /// * `query` - The search query with optional text and filters
    ///
    /// # Returns
    ///
    /// A vector of search results sorted by score (highest first).
    pub fn search_martyrology(&self, query: MartyrologyQuery) -> Vec<MartyrologySearchResult> {
        let resolver = MartyrologyResolver::new(self);
        let matcher = MartyrologyMatcher::new();
        matcher.search(resolver.get_all_entries().values(), &query)
    }

    /// Get a liturgical date by its ID for a given year
    ///
    /// # Arguments
    ///
    /// * `id` - The date ID (e.g., "easter_sunday", "christmas")
    /// * `year` - The year
    ///
    /// # Returns
    ///
    /// Date in YYYY-MM-DD format
    ///
    /// # Errors
    ///
    /// Returns `RomcalError::InvalidDateName` if the date ID is not found
    pub fn get_date(&self, id: &str, year: i32) -> crate::RomcalResult<String> {
        let dates = LiturgicalDates::new(self.clone(), year)?;

        // 1. Try direct calculation for known dates
        if let Some(date) = dates.get_date_by_id(id) {
            return Ok(date.format("%Y-%m-%d").to_string());
        }

        // 2. Generate calendar and search by ID
        let calendar = self.generate_liturgical_calendar(year)?;
        for (date, days) in &calendar {
            for day in days {
                if day.id == id {
                    return Ok(date.clone());
                }
            }
        }

        // 3. Not found
        Err(RomcalError::InvalidDateName(id.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_romcal() -> Romcal {
        Romcal::empty()
    }

    #[test]
    fn test_get_date_easter_sunday() {
        let romcal = create_test_romcal();
        let result = romcal.get_date("easter_sunday", 2026);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "2026-04-05");
    }

    #[test]
    fn test_get_date_christmas() {
        let romcal = create_test_romcal();
        let result = romcal.get_date("christmas", 2026);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "2026-12-25");
    }

    #[test]
    fn test_get_date_pentecost() {
        let romcal = create_test_romcal();
        let result = romcal.get_date("pentecost_sunday", 2026);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "2026-05-24");
    }

    #[test]
    fn test_get_date_ash_wednesday() {
        let romcal = create_test_romcal();
        let result = romcal.get_date("ash_wednesday", 2026);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "2026-02-18");
    }

    #[test]
    fn test_get_date_invalid_name() {
        let romcal = create_test_romcal();
        let result = romcal.get_date("invalid_date_name", 2026);
        assert!(result.is_err());
        match result {
            Err(RomcalError::InvalidDateName(name)) => {
                assert_eq!(name, "invalid_date_name");
            }
            _ => panic!("Expected InvalidDateName error"),
        }
    }

    #[test]
    fn test_get_date_from_calendar_fallback() {
        let romcal = create_test_romcal();
        // This date is not in direct calculation, requires calendar generation
        let result = romcal.get_date("ordinary_time_5_monday", 2026);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_date_first_sunday_of_advent() {
        let romcal = create_test_romcal();
        let result = romcal.get_date("first_sunday_of_advent", 2026);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "2026-11-29");
    }

    #[test]
    fn test_locale_case_insensitive() {
        // BCP 47 locales are case-insensitive
        // Note: With no resources provided, validation is skipped
        let romcal_lower = Romcal::new(Preset {
            locale: Some("fr-ca".to_string()),
            ..Default::default()
        })
        .unwrap();
        let romcal_upper = Romcal::new(Preset {
            locale: Some("FR-CA".to_string()),
            ..Default::default()
        })
        .unwrap();
        let romcal_mixed = Romcal::new(Preset {
            locale: Some("Fr-Ca".to_string()),
            ..Default::default()
        })
        .unwrap();

        // All should normalize to lowercase
        assert_eq!(romcal_lower.locale, "fr-ca");
        assert_eq!(romcal_upper.locale, "fr-ca");
        assert_eq!(romcal_mixed.locale, "fr-ca");
    }

    #[test]
    fn test_resources_locale_case_insensitive() {
        let mut romcal = Romcal::new(Preset {
            locale: Some("fr-CA".to_string()),
            resources: Some(vec![Resources::new("FR-CA".to_string())]),
            ..Default::default()
        })
        .unwrap();

        // Resource locale should be normalized
        assert_eq!(romcal.resources[0].locale, "fr-ca");

        // get_resources should find it
        assert!(romcal.get_resources("fr-ca").is_some());

        // Adding resources should also normalize
        romcal.add_resources(Resources::new("EN-GB".to_string()));
        assert!(romcal.get_resources("en-gb").is_some());
    }

    #[test]
    fn test_calendar_not_found_error() {
        let result = Romcal::new(Preset {
            calendar: Some("nonexistent_calendar".to_string()),
            calendar_definitions: Some(vec![CalendarDefinition::new("general_roman".to_string())]),
            ..Default::default()
        });

        assert!(result.is_err());
        match result {
            Err(RomcalError::CalendarNotFound(calendar, available)) => {
                assert_eq!(calendar, "nonexistent_calendar");
                assert!(available.contains(&"general_roman".to_string()));
            }
            _ => panic!("Expected CalendarNotFound error"),
        }
    }

    #[test]
    fn test_locale_not_found_error() {
        let result = Romcal::new(Preset {
            locale: Some("nonexistent_locale".to_string()),
            resources: Some(vec![Resources::new("en".to_string())]),
            ..Default::default()
        });

        assert!(result.is_err());
        match result {
            Err(RomcalError::LocaleNotFound(locale, available)) => {
                assert_eq!(locale, "nonexistent_locale");
                assert!(available.contains(&"en".to_string()));
            }
            _ => panic!("Expected LocaleNotFound error"),
        }
    }

    #[test]
    fn test_locale_fallback_to_base() {
        // fr-ca should be accepted if fr is available (base locale fallback)
        let result = Romcal::new(Preset {
            locale: Some("fr-ca".to_string()),
            resources: Some(vec![Resources::new("fr".to_string())]),
            ..Default::default()
        });

        assert!(result.is_ok());
    }

    #[test]
    fn test_locale_no_implicit_english_fallback() {
        // Unknown locale should NOT fallback to "en" - explicit error is raised
        let result = Romcal::new(Preset {
            locale: Some("xx".to_string()),
            resources: Some(vec![Resources::new("en".to_string())]),
            ..Default::default()
        });

        assert!(result.is_err());
        match result {
            Err(RomcalError::LocaleNotFound(locale, _)) => {
                assert_eq!(locale, "xx");
            }
            _ => panic!("Expected LocaleNotFound error"),
        }
    }

    #[test]
    fn test_add_resources_merges_same_locale() {
        use crate::types::martyrology::MartyrologyEntryDef;

        let mut romcal = Romcal::empty();

        // Add first resource for "en" with entry "saint_a"
        let mut res1 = Resources::new("en".to_string());
        let mut def_a = MartyrologyEntryDef::new();
        def_a.name = Some("Saint A".to_string());
        res1.add_martyrology_entry("saint_a".to_string(), def_a);
        romcal.add_resources(res1);

        // Add second resource for "en" with entry "saint_b"
        let mut res2 = Resources::new("en".to_string());
        let mut def_b = MartyrologyEntryDef::new();
        def_b.name = Some("Saint B".to_string());
        res2.add_martyrology_entry("saint_b".to_string(), def_b);
        romcal.add_resources(res2);

        // Should have only one resource for "en" (merged, not duplicated)
        let en_count = romcal.resources.iter().filter(|r| r.locale == "en").count();
        assert_eq!(en_count, 1, "Should have exactly one resource for 'en'");

        // Both entries should be accessible via get_resources
        let en_res = romcal.get_resources("en").unwrap();
        let martyrology = en_res.martyrology.as_ref().unwrap();
        assert!(
            martyrology.contains_key("saint_a"),
            "saint_a should be present"
        );
        assert!(
            martyrology.contains_key("saint_b"),
            "saint_b should be present"
        );
    }

    #[test]
    fn test_add_resources_overwrites_existing_entry() {
        use crate::types::martyrology::MartyrologyEntryDef;

        let mut romcal = Romcal::empty();

        // Add resource with entry "saint_a" name="Original"
        let mut res1 = Resources::new("en".to_string());
        let mut def1 = MartyrologyEntryDef::new();
        def1.name = Some("Original".to_string());
        res1.add_martyrology_entry("saint_a".to_string(), def1);
        romcal.add_resources(res1);

        // Add another resource for "en" with same entry overwritten
        let mut res2 = Resources::new("en".to_string());
        let mut def2 = MartyrologyEntryDef::new();
        def2.name = Some("Updated".to_string());
        res2.add_martyrology_entry("saint_a".to_string(), def2);
        romcal.add_resources(res2);

        // The entry should be overwritten
        let en_res = romcal.get_resources("en").unwrap();
        let entry = en_res.martyrology.as_ref().unwrap().get("saint_a").unwrap();
        assert_eq!(entry.name, Some("Updated".to_string()));
    }

    #[test]
    fn test_empty_definitions_skip_validation() {
        // With empty definitions, calendar validation is skipped
        let result = Romcal::new(Preset {
            calendar: Some("any_calendar".to_string()),
            calendar_definitions: Some(vec![]),
            ..Default::default()
        });

        assert!(result.is_ok());
    }
}
