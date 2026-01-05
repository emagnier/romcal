//! Core Romcal configuration and instance management.
//!
//! This module provides the main `Romcal` struct and `Preset` configuration
//! for initializing and customizing liturgical calendar generation.

use serde::{Deserialize, Serialize};

use crate::engine::calendar_definition::CalendarDefinition;
use crate::engine::dates::LiturgicalDates;
use crate::engine::resources::Resources;
use crate::entity_resolution::{EntityResolver, normalize_locale};
use crate::entity_search::{EntityMatcher, EntityQuery, EntitySearchResult};
use crate::error::RomcalError;
use crate::types::entity::Entity;
use crate::types::{CalendarContext, EasterCalculationType, OrdinalFormat};

// Default configuration constants
const DEFAULT_CALENDAR: &str = "general_roman";
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
            calendar_definitions: Vec::new(),
            resources: Vec::new(),
        }
    }
}

impl Romcal {
    /// Creates a new Romcal instance with default values applied to any None fields
    ///
    /// Priority for ordinal_format:
    /// 1. Value from Preset (highest priority)
    /// 2. Value from ResourcesMetadata of the target locale
    /// 3. Default value (Numeric)
    pub fn new(config: Preset) -> Self {
        let calendar_definitions = config.calendar_definitions.unwrap_or_default();

        // Normalize locale to lowercase (BCP 47 is case-insensitive)
        let locale = normalize_locale(config.locale.as_deref().unwrap_or(DEFAULT_LOCALE));

        // Normalize resources locales to lowercase
        let resources: Vec<Resources> = config
            .resources
            .unwrap_or_default()
            .into_iter()
            .map(|mut res| {
                res.locale = normalize_locale(&res.locale);
                res
            })
            .collect();

        // Get ordinal_format from locale's ResourcesMetadata if not set in Preset
        let ordinal_format_from_locale = resources
            .iter()
            .find(|res| res.locale == locale)
            .and_then(|res| res.metadata.as_ref())
            .and_then(|meta| meta.ordinal_format);

        Self {
            calendar: config
                .calendar
                .unwrap_or_else(|| DEFAULT_CALENDAR.to_string()),
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

    /// Add a resources definition to the configuration
    pub fn add_resources(&mut self, mut resources: Resources) {
        resources.locale = normalize_locale(&resources.locale);
        self.resources.push(resources);
    }

    /// Create a JSON bundle of the current configuration
    /// This method serializes the Preset to JSON format
    /// and removes null values and empty objects from the output
    pub fn optimize(&self) -> Result<String, serde_json::Error> {
        crate::engine::optimize::optimize(self)
            .map_err(|e| serde_json::Error::io(std::io::Error::other(e.to_string())))
    }

    /// Generate the complete liturgical calendar for a given liturgical year
    ///
    /// This method combines the Proper of Time with particular calendars
    /// and applies precedence rules according to UNLY #49.
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

    /// Get an entity by its exact ID.
    ///
    /// Searches in the current locale's resources first, then falls back to other locales.
    ///
    /// # Arguments
    ///
    /// * `id` - The entity ID (e.g., "francis_of_assisi")
    ///
    /// # Returns
    ///
    /// The entity if found, or `None` if not found.
    ///
    /// Uses locale fallback: en → parent locale → specific locale
    pub fn get_entity(&self, id: &str) -> Option<Entity> {
        let resolver = EntityResolver::new(self);
        resolver.resolve_entity(id).cloned()
    }

    /// Search entities with fuzzy matching and filters.
    ///
    /// Searches entities merged from all locales (en → parent → specific).
    ///
    /// # Arguments
    ///
    /// * `query` - The search query with optional text and filters
    ///
    /// # Returns
    ///
    /// A vector of search results sorted by score (highest first).
    pub fn search_entities(&self, query: EntityQuery) -> Vec<EntitySearchResult> {
        let resolver = EntityResolver::new(self);
        let matcher = EntityMatcher::new();
        matcher.search(resolver.get_all_entities().values(), &query)
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
        Romcal::default()
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
        let romcal_lower = Romcal::new(Preset {
            locale: Some("fr-ca".to_string()),
            ..Default::default()
        });
        let romcal_upper = Romcal::new(Preset {
            locale: Some("FR-CA".to_string()),
            ..Default::default()
        });
        let romcal_mixed = Romcal::new(Preset {
            locale: Some("Fr-Ca".to_string()),
            ..Default::default()
        });

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
        });

        // Resource locale should be normalized
        assert_eq!(romcal.resources[0].locale, "fr-ca");

        // get_resources should find it
        assert!(romcal.get_resources("fr-ca").is_some());

        // Adding resources should also normalize
        romcal.add_resources(Resources::new("EN-GB".to_string()));
        assert!(romcal.get_resources("en-gb").is_some());
    }
}
