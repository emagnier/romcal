//! Optimize and bundle Romcal configuration for distribution.
//!
//! This module provides functionality to create optimized JSON bundles from
//! a Romcal configuration. The optimization process:
//!
//! 1. **Filters calendar definitions** to include only the main calendar,
//!    its parent calendars, and the `general_roman` fallback
//!
//! 2. **Filters resources** to include only locales in the hierarchy
//!    (e.g., for `fr-ca`: includes `fr-ca`, `fr`, and `en`)
//!
//! 3. **Deduplicates at property level** so parent locales only contain
//!    properties missing in child locales (diff-based approach)
//!
//! 4. **Removes empty values** (null, empty objects) from the JSON output
//!
//! # Example
//!
//! For a locale hierarchy `fr-ca → fr → en`, after optimization:
//! - `fr-ca`: Contains the final values for translated properties
//! - `fr`: Contains only properties not defined in `fr-ca`
//! - `en`: Contains only properties not defined in `fr-ca` or `fr`

use serde_json::Value;
use std::collections::{HashMap, HashSet};

use crate::entity_resolution::locale::get_all_parent_locales;
use crate::types::entity::EntityDefinition;
use crate::types::resource::{
    AdventSeason, ChristmasTimeSeason, CyclesMetadata, EasterTimeSeason, LentSeason, LocaleColors,
    OrdinaryTimeSeason, PaschalTriduumSeason, PeriodsMetadata, RanksMetadata, ResourcesMetadata,
    SeasonsMetadata,
};
use crate::{CalendarDefinition, Resources, Romcal, RomcalError, RomcalResult};

// ============================================================================
// Type Aliases
// ============================================================================

/// Maps lowercase locale to original case (for case-insensitive lookup).
type LocaleMap = HashMap<String, String>;

/// Set of IDs (entities, calendars, etc.).
type IdSet = HashSet<String>;

/// Set of property names for tracking defined properties during deduplication.
type PropertySet = HashSet<String>;

/// Maps entity ID to its set of defined properties across locales.
type EntityPropertiesMap = HashMap<String, PropertySet>;

// ============================================================================
// Main Entry Point
// ============================================================================

/// Create an optimized JSON bundle of the Romcal configuration.
///
/// This function filters and deduplicates the configuration to create a minimal
/// bundle suitable for distribution. The output contains:
///
/// - Only calendar definitions in the hierarchy (general_roman → parents → main)
/// - Only resources for locales in the hierarchy (en → parent → specific)
/// - Property-level deduplication across locale hierarchy
/// - No null values or empty objects
///
/// # Arguments
///
/// * `romcal` - The Romcal configuration to optimize
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
pub fn optimize(romcal: &Romcal) -> RomcalResult<String> {
    // Validate uniqueness constraints
    validate_unique_calendar_ids(&romcal.calendar_definitions)?;
    validate_unique_resource_locales(&romcal.resources)?;

    // Filter to relevant calendars and resources
    let mut filtered_config = romcal.clone();
    filtered_config.calendar_definitions = filter_calendar_definitions(romcal)?;
    filtered_config.resources = filter_resources(romcal, &filtered_config.calendar_definitions)?;

    // Reverse for intuitive output order: general → specific
    // Calendars: [general_roman, europe, france]
    filtered_config.calendar_definitions.reverse();
    // Resources: [en, fr, fr-ca]
    filtered_config.resources.reverse();

    // Serialize and clean
    let value = serde_json::to_value(&filtered_config)
        .map_err(|e| RomcalError::ValidationError(format!("JSON serialization error: {}", e)))?;
    let cleaned_value = remove_null_and_empty_values(value);

    serde_json::to_string_pretty(&cleaned_value)
        .map_err(|e| RomcalError::ValidationError(format!("JSON formatting error: {}", e)))
}

// ============================================================================
// Validation Functions
// ============================================================================

/// Validate that all calendar definitions have unique IDs.
fn validate_unique_calendar_ids(calendar_definitions: &[CalendarDefinition]) -> RomcalResult<()> {
    let mut seen = IdSet::new();
    for cal in calendar_definitions {
        if !seen.insert(cal.id.clone()) {
            return Err(RomcalError::ValidationError(format!(
                "Duplicate calendar ID '{}' found. Each calendar must have a unique ID.",
                cal.id
            )));
        }
    }
    Ok(())
}

/// Validate that all resource definitions have unique locales.
fn validate_unique_resource_locales(resources: &[Resources]) -> RomcalResult<()> {
    let mut seen = IdSet::new();
    for res in resources {
        if !seen.insert(res.locale.clone()) {
            return Err(RomcalError::ValidationError(format!(
                "Duplicate locale '{}' found. Each resource must have a unique locale.",
                res.locale
            )));
        }
    }
    Ok(())
}

// ============================================================================
// Calendar Filtering
// ============================================================================

/// Filter calendar definitions to include only the hierarchy chain.
///
/// Keeps: main calendar → parent calendars → general_roman (fallback).
/// Returns calendars ordered from most specific to most general.
fn filter_calendar_definitions(romcal: &Romcal) -> RomcalResult<Vec<CalendarDefinition>> {
    // Find the main calendar
    let main_calendar = romcal
        .calendar_definitions
        .iter()
        .find(|cal| cal.id == romcal.calendar)
        .ok_or_else(|| {
            RomcalError::ValidationError(format!(
                "Main calendar '{}' not found in calendar_definitions",
                romcal.calendar
            ))
        })?;

    // Check for circular reference
    if main_calendar
        .parent_calendar_ids
        .contains(&main_calendar.id)
    {
        return Err(RomcalError::ValidationError(format!(
            "Calendar '{}' cannot be its own parent (circular reference)",
            main_calendar.id
        )));
    }

    // Build required IDs list: main → parents → general_roman (specific → general)
    // optimize() reverses to: general_roman → parents → main (general → specific)
    let mut required_ids = vec![main_calendar.id.clone()];
    for parent_id in &main_calendar.parent_calendar_ids {
        if !required_ids.contains(parent_id) {
            required_ids.push(parent_id.clone());
        }
    }
    if !required_ids.contains(&"general_roman".to_string()) {
        required_ids.push("general_roman".to_string());
    }

    // Validate all required calendars exist
    let available: IdSet = romcal
        .calendar_definitions
        .iter()
        .map(|c| c.id.clone())
        .collect();
    for id in &required_ids {
        if !available.contains(id) {
            return Err(RomcalError::ValidationError(format!(
                "Required calendar '{}' not found in calendar_definitions",
                id
            )));
        }
    }

    // Collect in order
    Ok(required_ids
        .iter()
        .filter_map(|id| {
            romcal
                .calendar_definitions
                .iter()
                .find(|c| &c.id == id)
                .cloned()
        })
        .collect())
}

// ============================================================================
// Resource Filtering
// ============================================================================

/// Filter resources to include only the locale hierarchy with property-level deduplication.
///
/// For locale `fr-ca`, keeps: `fr-ca` → `fr` → `en` (fallback).
/// Applies property-level deduplication so parent locales only contain
/// properties not defined in child locales.
fn filter_resources(
    romcal: &Romcal,
    filtered_calendars: &[CalendarDefinition],
) -> RomcalResult<Vec<Resources>> {
    let target_locale = &romcal.locale;

    // Build lookup maps
    let (available_locales, resources_by_locale) = build_locale_maps(romcal);

    // Validate target locale exists
    let exact_locale = available_locales
        .get(&target_locale.to_lowercase())
        .cloned()
        .ok_or_else(|| {
            RomcalError::ValidationError(format!(
                "Locale '{}' not found. Available: {}",
                target_locale,
                available_locales
                    .values()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
        })?;

    // Collect entity IDs used in calendar definitions
    let used_entity_ids = collect_used_entity_ids(filtered_calendars);

    // Build locale priority: specific → general → en
    let priority_locales = build_priority_locales(target_locale, &available_locales, &exact_locale);

    // Apply hierarchical deduplication
    apply_hierarchical_deduplication(priority_locales, &resources_by_locale, &used_entity_ids)
}

/// Build maps for efficient locale lookups.
fn build_locale_maps(romcal: &Romcal) -> (LocaleMap, HashMap<&str, &Resources>) {
    let available_locales: LocaleMap = romcal
        .resources
        .iter()
        .map(|r| (r.locale.to_lowercase(), r.locale.clone()))
        .collect();

    let resources_by_locale: HashMap<&str, &Resources> = romcal
        .resources
        .iter()
        .map(|r| (r.locale.as_str(), r))
        .collect();

    (available_locales, resources_by_locale)
}

/// Build priority list of locales from most specific to most general.
fn build_priority_locales(
    target_locale: &str,
    available_locales: &LocaleMap,
    exact_locale: &str,
) -> Vec<String> {
    let mut locales = vec![exact_locale.to_string()];

    // Add parent locales in hierarchy order
    for parent in get_all_parent_locales(target_locale) {
        if parent != target_locale
            && let Some(actual) = available_locales.get(&parent.to_lowercase())
        {
            locales.push(actual.clone());
        }
    }

    // Always include "en" as fallback
    if let Some(en) = available_locales.get("en")
        && !locales.contains(en)
    {
        locales.push(en.clone());
    }

    locales
}

/// Collect all entity IDs referenced in calendar day definitions.
fn collect_used_entity_ids(calendars: &[CalendarDefinition]) -> IdSet {
    let mut ids = IdSet::new();

    for cal in calendars {
        for (day_id, day_def) in &cal.days_definitions {
            // Day definition ID is itself a potential entity reference
            ids.insert(day_id.clone());

            // Collect entity references
            if let Some(entities) = &day_def.entities {
                for entity_ref in entities {
                    match entity_ref {
                        crate::types::calendar::EntityRef::ResourceId(id) => {
                            ids.insert(id.clone());
                        }
                        crate::types::calendar::EntityRef::Override(o) => {
                            ids.insert(o.id.clone());
                        }
                    }
                }
            }
        }
    }

    ids
}

/// Apply hierarchical deduplication to resources.
///
/// Resources are processed from most specific to most general locale.
/// Property-level deduplication ensures parent locales only contain
/// properties that are missing in their child locales.
fn apply_hierarchical_deduplication(
    priority_locales: Vec<String>,
    resources_by_locale: &HashMap<&str, &Resources>,
    used_entity_ids: &IdSet,
) -> RomcalResult<Vec<Resources>> {
    // Build filtered resources list (specific → general)
    let mut resources: Vec<Resources> = priority_locales
        .iter()
        .filter_map(|locale| {
            resources_by_locale.get(locale.as_str()).map(|r| {
                let mut filtered = (*r).clone();
                filter_entities_by_usage(&mut filtered, used_entity_ids);
                filtered
            })
        })
        .collect();

    // Apply property-level deduplication
    deduplicate_entity_properties(&mut resources);
    deduplicate_metadata_properties(&mut resources);

    // Clean up empty entities
    remove_empty_entities(&mut resources);

    Ok(resources)
}

/// Filter entities to only include those referenced in calendar definitions.
fn filter_entities_by_usage(resource: &mut Resources, used_ids: &IdSet) {
    if let Some(entities) = &mut resource.entities {
        entities.retain(|id, _| used_ids.contains(id));
    }
}

// ============================================================================
// Entity Property-Level Deduplication
// ============================================================================

/// Deduplicate entity properties across locales.
///
/// For each entity, if a property exists in a more specific locale,
/// it is removed from parent locales. Resources must be ordered
/// from most specific to most general.
fn deduplicate_entity_properties(resources: &mut [Resources]) {
    let mut defined_props: EntityPropertiesMap = HashMap::new();

    for resource in resources.iter_mut() {
        if let Some(entities) = &mut resource.entities {
            for (entity_id, entity) in entities.iter_mut() {
                let props = defined_props.entry(entity_id.clone()).or_default();
                deduplicate_single_entity(entity, props);
            }
        }
    }
}

/// Deduplicate properties of a single entity.
///
/// For each property: if already defined in a more specific locale, set to None;
/// otherwise if Some, mark as defined for parent locales.
fn deduplicate_single_entity(entity: &mut EntityDefinition, defined: &mut PropertySet) {
    /// Macro to deduplicate a single Option field.
    /// If already defined → set to None. If Some → mark as defined.
    macro_rules! dedup {
        ($field:ident) => {
            if defined.contains(stringify!($field)) {
                entity.$field = None;
            } else if entity.$field.is_some() {
                defined.insert(stringify!($field).to_string());
            }
        };
    }

    dedup!(r#type);
    dedup!(fullname);
    dedup!(name);
    dedup!(canonization_level);
    dedup!(date_of_canonization);
    dedup!(date_of_canonization_is_approximative);
    dedup!(date_of_beatification);
    dedup!(date_of_beatification_is_approximative);
    dedup!(hide_canonization_level);
    dedup!(titles);
    dedup!(sex);
    dedup!(hide_titles);
    dedup!(date_of_dedication);
    dedup!(date_of_birth);
    dedup!(date_of_birth_is_approximative);
    dedup!(date_of_death);
    dedup!(date_of_death_is_approximative);
    dedup!(count);
    dedup!(sources);
}

/// Check if an entity has all properties set to None.
fn is_entity_empty(entity: &EntityDefinition) -> bool {
    /// Macro to check if a field is None.
    macro_rules! is_none {
        ($($field:ident),+) => {
            $(entity.$field.is_none())&&+
        };
    }

    // Note: _todo is excluded as it's internal metadata (not serialized)
    is_none!(
        r#type,
        fullname,
        name,
        canonization_level,
        date_of_canonization,
        date_of_canonization_is_approximative,
        date_of_beatification,
        date_of_beatification_is_approximative,
        hide_canonization_level,
        titles,
        sex,
        hide_titles,
        date_of_dedication,
        date_of_birth,
        date_of_birth_is_approximative,
        date_of_death,
        date_of_death_is_approximative,
        count,
        sources
    )
}

/// Remove entities where all properties are None after deduplication.
fn remove_empty_entities(resources: &mut [Resources]) {
    for resource in resources.iter_mut() {
        if let Some(entities) = &mut resource.entities {
            entities.retain(|_, entity| !is_entity_empty(entity));
        }
    }
}

// ============================================================================
// Metadata Property-Level Deduplication
// ============================================================================

/// Deduplicate metadata properties across locales.
///
/// Uses hierarchical property keys (e.g., `seasons.advent.season`) for tracking.
/// Nested structures are recursively deduplicated.
fn deduplicate_metadata_properties(resources: &mut [Resources]) {
    let mut defined = PropertySet::new();

    for resource in resources.iter_mut() {
        if let Some(metadata) = &mut resource.metadata {
            deduplicate_single_metadata(metadata, &mut defined);
        }
    }
}

/// Deduplicate properties of a single metadata object.
fn deduplicate_single_metadata(metadata: &mut ResourcesMetadata, defined: &mut PropertySet) {
    /// Macro for simple Option properties at metadata level.
    macro_rules! dedup {
        ($field:ident) => {
            if defined.contains(stringify!($field)) {
                metadata.$field = None;
            } else if metadata.$field.is_some() {
                defined.insert(stringify!($field).to_string());
            }
        };
    }

    dedup!(ordinal_format);
    dedup!(ordinals_letters);
    dedup!(ordinals_numeric);
    dedup!(weekdays);
    dedup!(months);

    // Nested structures
    deduplicate_colors(&mut metadata.colors, defined);
    deduplicate_seasons(&mut metadata.seasons, defined);
    deduplicate_periods(&mut metadata.periods, defined);
    deduplicate_ranks(&mut metadata.ranks, defined);
    deduplicate_cycles(&mut metadata.cycles, defined);
}

// ============================================================================
// Nested Metadata Deduplication Helpers
// ============================================================================

/// Macro to generate deduplication functions for nested metadata structs.
///
/// This macro generates a function that:
/// 1. Deduplicates each field using a prefixed key
/// 2. Sets the entire struct to None if all fields become None
macro_rules! impl_nested_dedup {
    (
        $fn_name:ident,
        $struct_type:ty,
        $prefix:literal,
        $($field:ident),+
    ) => {
        fn $fn_name(opt: &mut Option<$struct_type>, defined: &mut PropertySet) {
            if let Some(s) = opt {
                $(
                    let key = concat!($prefix, ".", stringify!($field));
                    if defined.contains(key) {
                        s.$field = None;
                    } else if s.$field.is_some() {
                        defined.insert(key.to_string());
                    }
                )+

                // Remove struct if all fields are None
                if true $(&& s.$field.is_none())+ {
                    *opt = None;
                }
            }
        }
    };
}

impl_nested_dedup!(
    deduplicate_colors,
    LocaleColors,
    "colors",
    black,
    gold,
    green,
    purple,
    red,
    rose,
    white
);

impl_nested_dedup!(
    deduplicate_advent,
    AdventSeason,
    "seasons.advent",
    season,
    weekday,
    sunday,
    privileged_weekday
);

impl_nested_dedup!(
    deduplicate_christmas_time,
    ChristmasTimeSeason,
    "seasons.christmas_time",
    season,
    day,
    octave,
    before_epiphany,
    second_sunday_after_christmas,
    after_epiphany
);

impl_nested_dedup!(
    deduplicate_ordinary_time,
    OrdinaryTimeSeason,
    "seasons.ordinary_time",
    season,
    weekday,
    sunday
);

impl_nested_dedup!(
    deduplicate_lent,
    LentSeason,
    "seasons.lent",
    season,
    weekday,
    sunday,
    day_after_ash_wed,
    holy_week_day
);

impl_nested_dedup!(
    deduplicate_paschal_triduum,
    PaschalTriduumSeason,
    "seasons.paschal_triduum",
    season
);

impl_nested_dedup!(
    deduplicate_easter_time,
    EasterTimeSeason,
    "seasons.easter_time",
    season,
    weekday,
    sunday,
    octave
);

impl_nested_dedup!(
    deduplicate_periods,
    PeriodsMetadata,
    "periods",
    christmas_octave,
    days_before_epiphany,
    days_from_epiphany,
    christmas_to_presentation_of_the_lord,
    presentation_of_the_lord_to_holy_thursday,
    holy_week,
    paschal_triduum,
    easter_octave,
    early_ordinary_time,
    late_ordinary_time
);

impl_nested_dedup!(
    deduplicate_ranks,
    RanksMetadata,
    "ranks",
    solemnity,
    sunday,
    feast,
    memorial,
    optional_memorial,
    weekday
);

impl_nested_dedup!(
    deduplicate_cycles,
    CyclesMetadata,
    "cycles",
    proper_of_time,
    proper_of_saints,
    sunday_year_a,
    sunday_year_b,
    sunday_year_c,
    weekday_year_1,
    weekday_year_2,
    psalter_week_1,
    psalter_week_2,
    psalter_week_3,
    psalter_week_4
);

/// Deduplicate seasons metadata (container for all season types).
fn deduplicate_seasons(seasons: &mut Option<SeasonsMetadata>, defined: &mut PropertySet) {
    if let Some(s) = seasons {
        deduplicate_advent(&mut s.advent, defined);
        deduplicate_christmas_time(&mut s.christmas_time, defined);
        deduplicate_ordinary_time(&mut s.ordinary_time, defined);
        deduplicate_lent(&mut s.lent, defined);
        deduplicate_paschal_triduum(&mut s.paschal_triduum, defined);
        deduplicate_easter_time(&mut s.easter_time, defined);

        // Remove seasons if all are None
        if s.advent.is_none()
            && s.christmas_time.is_none()
            && s.ordinary_time.is_none()
            && s.lent.is_none()
            && s.paschal_triduum.is_none()
            && s.easter_time.is_none()
        {
            *seasons = None;
        }
    }
}

// ============================================================================
// JSON Cleaning
// ============================================================================

/// Recursively remove null values, empty objects, and `$schema` properties.
fn remove_null_and_empty_values(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let cleaned: serde_json::Map<String, Value> = map
                .into_iter()
                .filter(|(k, _)| k != "$schema")
                .map(|(k, v)| (k, remove_null_and_empty_values(v)))
                .filter(|(_, v)| !v.is_null())
                .collect();

            if cleaned.is_empty() {
                Value::Null
            } else {
                Value::Object(cleaned)
            }
        }
        Value::Array(arr) => {
            let cleaned: Vec<Value> = arr
                .into_iter()
                .map(remove_null_and_empty_values)
                .filter(|v| !v.is_null())
                .collect();
            Value::Array(cleaned)
        }
        other => other,
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::calendar::{CalendarJurisdiction, CalendarType, DayDefinition, EntityRef};
    use crate::types::entity::{EntityOverride, EntityType};

    // ------------------------------------------------------------------------
    // Test Helpers
    // ------------------------------------------------------------------------

    /// Create a test entity with specified properties.
    fn entity(
        name: Option<&str>,
        fullname: Option<&str>,
        entity_type: Option<EntityType>,
    ) -> EntityDefinition {
        EntityDefinition {
            name: name.map(String::from),
            fullname: fullname.map(String::from),
            r#type: entity_type,
            ..Default::default()
        }
    }

    /// Create a Resources with a single entity.
    fn resources_with_entity(locale: &str, entity_id: &str, e: EntityDefinition) -> Resources {
        let mut entities = std::collections::BTreeMap::new();
        entities.insert(entity_id.to_string(), e);
        Resources {
            schema: None,
            locale: locale.to_string(),
            metadata: None,
            entities: Some(entities),
        }
    }

    /// Create a Resources with metadata.
    fn resources_with_metadata(locale: &str, metadata: ResourcesMetadata) -> Resources {
        Resources {
            schema: None,
            locale: locale.to_string(),
            metadata: Some(metadata),
            entities: None,
        }
    }

    /// Create a minimal calendar definition.
    fn calendar_def(id: &str, parents: Vec<&str>) -> CalendarDefinition {
        CalendarDefinition {
            schema: None,
            id: id.to_string(),
            metadata: crate::types::CalendarMetadata {
                jurisdiction: CalendarJurisdiction::Ecclesiastical,
                r#type: CalendarType::Diocese,
            },
            particular_config: None,
            parent_calendar_ids: parents.into_iter().map(String::from).collect(),
            days_definitions: std::collections::BTreeMap::new(),
        }
    }

    /// Create an empty ResourcesMetadata.
    fn empty_metadata() -> ResourcesMetadata {
        ResourcesMetadata::default()
    }

    // ------------------------------------------------------------------------
    // Calendar Filtering Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_filter_calendar_definitions_hierarchy() {
        let romcal = Romcal {
            calendar: "france".to_string(),
            locale: "fr".to_string(),
            calendar_definitions: vec![
                calendar_def("general_roman", vec![]),
                calendar_def("europe", vec!["general_roman"]),
                calendar_def("france", vec!["europe", "general_roman"]),
                calendar_def("unrelated", vec!["general_roman"]),
            ],
            ..Default::default()
        };

        // filter_calendar_definitions returns specific → general
        // optimize() reverses to general → specific for output
        let result = filter_calendar_definitions(&romcal).unwrap();

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].id, "france"); // most specific
        assert_eq!(result[1].id, "europe"); // parent
        assert_eq!(result[2].id, "general_roman"); // fallback
    }

    #[test]
    fn test_filter_calendar_definitions_missing_calendar() {
        let romcal = Romcal {
            calendar: "nonexistent".to_string(),
            locale: "en".to_string(),
            calendar_definitions: vec![calendar_def("general_roman", vec![])],
            ..Default::default()
        };

        let result = filter_calendar_definitions(&romcal);
        assert!(result.is_err());
    }

    #[test]
    fn test_filter_calendar_definitions_circular_reference() {
        let romcal = Romcal {
            calendar: "circular".to_string(),
            locale: "en".to_string(),
            calendar_definitions: vec![
                calendar_def("general_roman", vec![]),
                CalendarDefinition {
                    parent_calendar_ids: vec!["circular".to_string()],
                    ..calendar_def("circular", vec![])
                },
            ],
            ..Default::default()
        };

        let result = filter_calendar_definitions(&romcal);
        assert!(result.is_err());
    }

    // ------------------------------------------------------------------------
    // Entity Collection Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_collect_used_entity_ids() {
        let mut days = std::collections::BTreeMap::new();
        days.insert(
            "saint_john".to_string(),
            DayDefinition {
                entities: Some(vec![
                    EntityRef::ResourceId("john_baptist".to_string()),
                    EntityRef::Override(EntityOverride {
                        id: "john_evangelist".to_string(),
                        titles: None,
                        hide_titles: None,
                        count: None,
                    }),
                ]),
                ..Default::default()
            },
        );
        days.insert("saint_peter".to_string(), DayDefinition::default());

        let cal = CalendarDefinition {
            days_definitions: days,
            ..calendar_def("test", vec![])
        };

        let ids = collect_used_entity_ids(&[cal]);

        assert!(ids.contains("saint_john"));
        assert!(ids.contains("saint_peter"));
        assert!(ids.contains("john_baptist"));
        assert!(ids.contains("john_evangelist"));
        assert!(!ids.contains("unused"));
    }

    #[test]
    fn test_filter_entities_by_usage() {
        let mut res = Resources {
            schema: None,
            locale: "en".to_string(),
            metadata: None,
            entities: Some({
                let mut m = std::collections::BTreeMap::new();
                m.insert("used".to_string(), entity(Some("Used"), None, None));
                m.insert("unused".to_string(), entity(Some("Unused"), None, None));
                m
            }),
        };

        let used: IdSet = ["used"].iter().map(|s| s.to_string()).collect();
        filter_entities_by_usage(&mut res, &used);

        let entities = res.entities.unwrap();
        assert_eq!(entities.len(), 1);
        assert!(entities.contains_key("used"));
    }

    // ------------------------------------------------------------------------
    // Entity Property Deduplication Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_deduplicate_entity_properties_hierarchy() {
        // fr-ca (specific) → fr (parent) → en (fallback)
        let mut resources = vec![
            resources_with_entity("fr-ca", "john", entity(Some("Jean"), None, None)),
            resources_with_entity(
                "fr",
                "john",
                entity(Some("Jean"), Some("Jean le Baptiste"), None),
            ),
            resources_with_entity(
                "en",
                "john",
                entity(
                    Some("John"),
                    Some("John the Baptist"),
                    Some(EntityType::Person),
                ),
            ),
        ];

        deduplicate_entity_properties(&mut resources);

        // fr-ca: keeps name (most specific)
        let fr_ca = resources[0].entities.as_ref().unwrap().get("john").unwrap();
        assert!(fr_ca.name.is_some());
        assert!(fr_ca.fullname.is_none());
        assert!(fr_ca.r#type.is_none());

        // fr: name removed (in fr-ca), keeps fullname
        let fr = resources[1].entities.as_ref().unwrap().get("john").unwrap();
        assert!(fr.name.is_none());
        assert!(fr.fullname.is_some());
        assert!(fr.r#type.is_none());

        // en: name & fullname removed, keeps type
        let en = resources[2].entities.as_ref().unwrap().get("john").unwrap();
        assert!(en.name.is_none());
        assert!(en.fullname.is_none());
        assert!(en.r#type.is_some());
    }

    #[test]
    fn test_remove_empty_entities_after_dedup() {
        let mut resources = vec![
            resources_with_entity("fr", "john", entity(Some("Jean"), None, None)),
            resources_with_entity("en", "john", entity(Some("John"), None, None)),
        ];

        deduplicate_entity_properties(&mut resources);
        remove_empty_entities(&mut resources);

        // fr: keeps john
        assert!(resources[0].entities.as_ref().unwrap().contains_key("john"));

        // en: john removed (became empty)
        assert!(!resources[1].entities.as_ref().unwrap().contains_key("john"));
    }

    #[test]
    fn test_is_entity_empty() {
        // Create empty entity using helper (sets all to None via Default)
        let mut empty = entity(None, None, None);
        // Ensure all properties are None
        empty.canonization_level = None;
        empty.date_of_canonization = None;
        empty.date_of_canonization_is_approximative = None;
        empty.date_of_beatification = None;
        empty.date_of_beatification_is_approximative = None;
        empty.hide_canonization_level = None;
        empty.titles = None;
        empty.sex = None;
        empty.hide_titles = None;
        empty.date_of_dedication = None;
        empty.date_of_birth = None;
        empty.date_of_birth_is_approximative = None;
        empty.date_of_death = None;
        empty.date_of_death_is_approximative = None;
        empty.count = None;
        empty.sources = None;
        assert!(is_entity_empty(&empty));

        let with_name = entity(Some("John"), None, None);
        assert!(!is_entity_empty(&with_name));
    }

    // ------------------------------------------------------------------------
    // Metadata Property Deduplication Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_deduplicate_metadata_properties() {
        let mut resources = vec![
            resources_with_metadata(
                "fr",
                ResourcesMetadata {
                    weekdays: Some({
                        let mut m = std::collections::BTreeMap::new();
                        m.insert("0".to_string(), "dimanche".to_string());
                        m
                    }),
                    ..empty_metadata()
                },
            ),
            resources_with_metadata(
                "en",
                ResourcesMetadata {
                    weekdays: Some({
                        let mut m = std::collections::BTreeMap::new();
                        m.insert("0".to_string(), "Sunday".to_string());
                        m
                    }),
                    months: Some({
                        let mut m = std::collections::BTreeMap::new();
                        m.insert("1".to_string(), "January".to_string());
                        m
                    }),
                    ..empty_metadata()
                },
            ),
        ];

        deduplicate_metadata_properties(&mut resources);

        // fr: keeps weekdays
        assert!(resources[0].metadata.as_ref().unwrap().weekdays.is_some());

        // en: weekdays removed, keeps months
        assert!(resources[1].metadata.as_ref().unwrap().weekdays.is_none());
        assert!(resources[1].metadata.as_ref().unwrap().months.is_some());
    }

    #[test]
    fn test_deduplicate_nested_seasons() {
        let mut resources = vec![
            resources_with_metadata(
                "fr",
                ResourcesMetadata {
                    seasons: Some(SeasonsMetadata {
                        advent: Some(AdventSeason {
                            season: Some("Avent".to_string()),
                            weekday: None,
                            sunday: None,
                            privileged_weekday: None,
                        }),
                        christmas_time: None,
                        ordinary_time: None,
                        lent: None,
                        paschal_triduum: None,
                        easter_time: None,
                    }),
                    ..empty_metadata()
                },
            ),
            resources_with_metadata(
                "en",
                ResourcesMetadata {
                    seasons: Some(SeasonsMetadata {
                        advent: Some(AdventSeason {
                            season: Some("Advent".to_string()),
                            weekday: Some("Weekday of Advent".to_string()),
                            sunday: None,
                            privileged_weekday: None,
                        }),
                        christmas_time: None,
                        ordinary_time: None,
                        lent: None,
                        paschal_triduum: None,
                        easter_time: None,
                    }),
                    ..empty_metadata()
                },
            ),
        ];

        deduplicate_metadata_properties(&mut resources);

        // fr: keeps advent.season
        let fr_advent = resources[0]
            .metadata
            .as_ref()
            .unwrap()
            .seasons
            .as_ref()
            .unwrap()
            .advent
            .as_ref()
            .unwrap();
        assert!(fr_advent.season.is_some());

        // en: advent.season removed, keeps advent.weekday
        let en_advent = resources[1]
            .metadata
            .as_ref()
            .unwrap()
            .seasons
            .as_ref()
            .unwrap()
            .advent
            .as_ref()
            .unwrap();
        assert!(en_advent.season.is_none());
        assert!(en_advent.weekday.is_some());
    }

    // ------------------------------------------------------------------------
    // Independent Entities Test
    // ------------------------------------------------------------------------

    #[test]
    fn test_deduplicate_independent_entities() {
        let mut resources = vec![
            Resources {
                schema: None,
                locale: "fr".to_string(),
                metadata: None,
                entities: Some({
                    let mut m = std::collections::BTreeMap::new();
                    m.insert("john".to_string(), entity(Some("Jean"), None, None));
                    m.insert("peter".to_string(), entity(Some("Pierre"), None, None));
                    m
                }),
            },
            Resources {
                schema: None,
                locale: "en".to_string(),
                metadata: None,
                entities: Some({
                    let mut m = std::collections::BTreeMap::new();
                    m.insert(
                        "john".to_string(),
                        entity(Some("John"), Some("John the Baptist"), None),
                    );
                    m.insert(
                        "peter".to_string(),
                        entity(Some("Peter"), Some("Peter the Apostle"), None),
                    );
                    m
                }),
            },
        ];

        deduplicate_entity_properties(&mut resources);

        // fr: both keep name
        let fr = resources[0].entities.as_ref().unwrap();
        assert!(fr.get("john").unwrap().name.is_some());
        assert!(fr.get("peter").unwrap().name.is_some());

        // en: both lose name, keep fullname
        let en = resources[1].entities.as_ref().unwrap();
        assert!(en.get("john").unwrap().name.is_none());
        assert!(en.get("john").unwrap().fullname.is_some());
        assert!(en.get("peter").unwrap().name.is_none());
        assert!(en.get("peter").unwrap().fullname.is_some());
    }
}
