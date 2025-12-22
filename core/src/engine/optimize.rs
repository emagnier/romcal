use serde_json::Value;
use std::collections::{HashMap, HashSet};

use crate::{CalendarDefinition, Resources, Romcal, RomcalError, RomcalResult};

// Type aliases for clarity
type LocaleMap = HashMap<String, String>;
type EntityIdSet = HashSet<String>;
type PropertySet = HashSet<String>;

// Constants for metadata properties
const METADATA_PROPERTIES: &[&str] = &[
    "ordinals_letters",
    "ordinals_numeric",
    "weekdays",
    "months",
    "colors",
    "periods",
    "ranks",
    "cycles",
];
const SEASONS_PROPERTIES: &[&str] = &[
    "advent",
    "christmas_time",
    "ordinary_time",
    "lent",
    "paschal_triduum",
    "easter_time",
];

/// Create a JSON bundle of the current configuration
/// This method serializes the Romcal config to JSON format
/// and removes null values and empty objects from the output.
///
/// Only includes calendar_definitions that are:
/// 1. The main calendar (Romcal.calendar)
/// 2. Parent calendars of the main calendar
/// 3. The general_roman calendar
pub fn optimize(romcal: &Romcal) -> RomcalResult<String> {
    // Validate that all calendar IDs are unique
    validate_unique_calendar_ids(&romcal.calendar_definitions)?;

    // Validate that all resource locales are unique
    validate_unique_resource_locales(&romcal.resources)?;

    // Create a filtered version of the config with only relevant calendar_definitions and resources
    let mut filtered_config = romcal.clone();
    filtered_config.calendar_definitions = filter_calendar_definitions(romcal)?;
    filtered_config.resources = filter_resources(romcal, &filtered_config.calendar_definitions)?;

    let value = serde_json::to_value(&filtered_config)
        .map_err(|e| RomcalError::ValidationError(format!("JSON serialization error: {}", e)))?;
    let cleaned_value = remove_null_and_empty_values(value);
    serde_json::to_string_pretty(&cleaned_value)
        .map_err(|e| RomcalError::ValidationError(format!("JSON formatting error: {}", e)))
}

/// Validate that all calendar definitions have unique IDs
/// Returns an error if duplicate calendar IDs are found
fn validate_unique_calendar_ids(calendar_definitions: &[CalendarDefinition]) -> RomcalResult<()> {
    let mut seen_ids = EntityIdSet::new();

    for calendar_def in calendar_definitions {
        if !seen_ids.insert(calendar_def.id.clone()) {
            return Err(RomcalError::ValidationError(format!(
                "Duplicate calendar ID '{}' found in calendar_definitions. Each calendar must have a unique ID.",
                calendar_def.id
            )));
        }
    }

    Ok(())
}

/// Validate that all resource definitions have unique locales
/// Returns an error if duplicate locales are found
fn validate_unique_resource_locales(resources: &[Resources]) -> RomcalResult<()> {
    let mut seen_locales = EntityIdSet::new();

    for resource in resources {
        if !seen_locales.insert(resource.locale.clone()) {
            return Err(RomcalError::ValidationError(format!(
                "Duplicate locale '{}' found in resources. Each resource must have a unique locale.",
                resource.locale
            )));
        }
    }

    Ok(())
}

/// Filter resources to keep only the required locales based on the romcal config
/// Returns resources with hierarchical deduplication: most specific to most general
/// Entities defined in more specific locales are removed from parent locales
/// Only includes entities that are referenced in calendar day_definitions
fn filter_resources(
    romcal: &Romcal,
    filtered_calendars: &[CalendarDefinition],
) -> RomcalResult<Vec<Resources>> {
    let target_locale = &romcal.locale;

    // Build locale maps for efficient lookups
    let (available_locales, resources_by_locale) = build_locale_maps(romcal);

    // Validate target locale exists
    let exact_locale = validate_target_locale(target_locale, &available_locales)?;

    // Collect used entity IDs
    let used_entity_ids = collect_used_entity_ids(filtered_calendars);

    // Build priority list of locales (most specific to most general)
    let priority_locales = build_priority_locales(target_locale, &available_locales, &exact_locale);

    // Apply hierarchical deduplication with entity filtering
    apply_hierarchical_deduplication(priority_locales, &resources_by_locale, &used_entity_ids)
}

/// Collect all entity IDs that are referenced in calendar day_definitions
/// Includes both day_definition IDs and EntityRef references
fn collect_used_entity_ids(calendar_definitions: &[CalendarDefinition]) -> EntityIdSet {
    let mut used_entity_ids = EntityIdSet::new();

    for calendar_def in calendar_definitions {
        for (day_id, day_def) in &calendar_def.days_definitions {
            // Add the day_definition ID itself as a potential entity reference
            used_entity_ids.insert(day_id.clone());

            // Also check EntityRef elements in the day_definition
            if let Some(entities) = &day_def.entities {
                for entity_pointer in entities {
                    match entity_pointer {
                        crate::types::calendar::EntityRef::ResourceId(id) => {
                            used_entity_ids.insert(id.clone());
                        }
                        crate::types::calendar::EntityRef::Override(entity_override) => {
                            used_entity_ids.insert(entity_override.id.clone());
                        }
                    }
                }
            }
        }
    }

    used_entity_ids
}

/// Build locale maps for efficient lookups
fn build_locale_maps(romcal: &Romcal) -> (LocaleMap, HashMap<&str, &Resources>) {
    let available_locales: LocaleMap = romcal
        .resources
        .iter()
        .map(|resource| (resource.locale.to_lowercase(), resource.locale.clone()))
        .collect();

    let resources_by_locale: HashMap<&str, &Resources> = romcal
        .resources
        .iter()
        .map(|resource| (resource.locale.as_str(), resource))
        .collect();

    (available_locales, resources_by_locale)
}

/// Validate that the target locale exists in available resources
fn validate_target_locale(
    target_locale: &str,
    available_locales: &LocaleMap,
) -> RomcalResult<String> {
    let target_locale_lower = target_locale.to_lowercase();
    available_locales
        .get(&target_locale_lower)
        .cloned()
        .ok_or_else(|| {
            RomcalError::ValidationError(format!(
                "Target locale '{}' not found in resources. Available locales: {}",
                target_locale,
                available_locales
                    .values()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
        })
}

/// Build priority list of locales from most specific to most general
fn build_priority_locales(
    target_locale: &str,
    available_locales: &LocaleMap,
    exact_locale: &str,
) -> Vec<String> {
    let mut priority_locales = Vec::new();

    // 1. Add the exact target locale first (most specific)
    priority_locales.push(exact_locale.to_string());

    // 2. Add all parent locales in hierarchy order (most specific to most general)
    let parent_locales = get_all_parent_locales(target_locale);
    for parent in parent_locales {
        if parent != target_locale
            && let Some(parent_locale_actual) = available_locales.get(&parent.to_lowercase())
        {
            priority_locales.push(parent_locale_actual.clone());
        }
    }

    // 3. Always include "en" last (most general fallback)
    if let Some(en_locale) = available_locales.get("en")
        && !priority_locales.contains(en_locale)
    {
        priority_locales.push(en_locale.clone());
    }

    priority_locales
}

/// Apply hierarchical deduplication to resources with entity filtering
fn apply_hierarchical_deduplication(
    priority_locales: Vec<String>,
    resources_by_locale: &HashMap<&str, &Resources>,
    used_entity_ids: &EntityIdSet,
) -> RomcalResult<Vec<Resources>> {
    let mut result = Vec::new();
    let mut defined_entity_ids = EntityIdSet::new();
    let mut defined_metadata_properties = PropertySet::new();
    let mut defined_seasons_properties = PropertySet::new();

    for locale in priority_locales {
        if let Some(resource) = resources_by_locale.get(locale.as_str()) {
            let mut filtered_resource = (*resource).clone();

            // Filter entities to only include those used in calendar day_definitions
            filter_entities_by_usage(&mut filtered_resource, used_entity_ids);

            // Deduplicate entities
            deduplicate_entities(&mut filtered_resource, &mut defined_entity_ids);

            // Deduplicate metadata
            deduplicate_metadata(
                &mut filtered_resource,
                &mut defined_metadata_properties,
                &mut defined_seasons_properties,
            );

            result.push(filtered_resource);
        }
    }

    Ok(result)
}

/// Filter entities to only include those that are used in calendar day_definitions
fn filter_entities_by_usage(resource: &mut Resources, used_entity_ids: &EntityIdSet) {
    if let Some(entities) = &mut resource.entities {
        entities.retain(|id, _entity| used_entity_ids.contains(id));
    }
}

/// Deduplicate entities in a resource
fn deduplicate_entities(resource: &mut Resources, defined_entity_ids: &mut EntityIdSet) {
    if let Some(entities) = &mut resource.entities {
        entities.retain(|id, _entity| {
            if defined_entity_ids.contains(id) {
                false // Remove entity already defined in more specific locale
            } else {
                defined_entity_ids.insert(id.clone());
                true // Keep entity and mark it as defined
            }
        });
    }
}

/// Deduplicate metadata in a resource
fn deduplicate_metadata(
    resource: &mut Resources,
    defined_metadata_properties: &mut PropertySet,
    defined_seasons_properties: &mut PropertySet,
) {
    if let Some(metadata) = &mut resource.metadata {
        // Deduplicate first level properties
        deduplicate_first_level_metadata(metadata, defined_metadata_properties);

        // Deduplicate seasons properties
        deduplicate_seasons_metadata(metadata, defined_seasons_properties);
    }
}

/// Deduplicate first level metadata properties
fn deduplicate_first_level_metadata(
    metadata: &mut crate::types::resource::ResourcesMetadata,
    defined_properties: &mut PropertySet,
) {
    let first_level_props = [
        (METADATA_PROPERTIES[0], metadata.ordinals_letters.is_some()),
        (METADATA_PROPERTIES[1], metadata.ordinals_numeric.is_some()),
        (METADATA_PROPERTIES[2], metadata.weekdays.is_some()),
        (METADATA_PROPERTIES[3], metadata.months.is_some()),
        (METADATA_PROPERTIES[4], metadata.colors.is_some()),
        (METADATA_PROPERTIES[5], metadata.periods.is_some()),
        (METADATA_PROPERTIES[6], metadata.ranks.is_some()),
        (METADATA_PROPERTIES[7], metadata.cycles.is_some()),
    ];

    for (prop_name, is_defined) in first_level_props {
        if is_defined {
            if defined_properties.contains(prop_name) {
                // Remove property already defined in more specific locale
                match prop_name {
                    "ordinals_letters" => metadata.ordinals_letters = None,
                    "ordinals_numeric" => metadata.ordinals_numeric = None,
                    "weekdays" => metadata.weekdays = None,
                    "months" => metadata.months = None,
                    "colors" => metadata.colors = None,
                    "periods" => metadata.periods = None,
                    "ranks" => metadata.ranks = None,
                    "cycles" => metadata.cycles = None,
                    _ => {}
                }
            } else {
                defined_properties.insert(prop_name.to_string());
            }
        }
    }
}

/// Deduplicate seasons metadata properties
fn deduplicate_seasons_metadata(
    metadata: &mut crate::types::resource::ResourcesMetadata,
    defined_seasons_properties: &mut PropertySet,
) {
    if let Some(seasons) = &mut metadata.seasons {
        let seasons_props = [
            (SEASONS_PROPERTIES[0], seasons.advent.is_some()),
            (SEASONS_PROPERTIES[1], seasons.christmas_time.is_some()),
            (SEASONS_PROPERTIES[2], seasons.ordinary_time.is_some()),
            (SEASONS_PROPERTIES[3], seasons.lent.is_some()),
            (SEASONS_PROPERTIES[4], seasons.paschal_triduum.is_some()),
            (SEASONS_PROPERTIES[5], seasons.easter_time.is_some()),
        ];

        for (prop_name, is_defined) in seasons_props {
            if is_defined {
                if defined_seasons_properties.contains(prop_name) {
                    // Remove season property already defined in more specific locale
                    match prop_name {
                        "advent" => seasons.advent = None,
                        "christmas_time" => seasons.christmas_time = None,
                        "ordinary_time" => seasons.ordinary_time = None,
                        "lent" => seasons.lent = None,
                        "paschal_triduum" => seasons.paschal_triduum = None,
                        "easter_time" => seasons.easter_time = None,
                        _ => {}
                    }
                } else {
                    defined_seasons_properties.insert(prop_name.to_string());
                }
            }
        }

        // If all seasons properties are None, set seasons to None
        if seasons.advent.is_none()
            && seasons.christmas_time.is_none()
            && seasons.ordinary_time.is_none()
            && seasons.lent.is_none()
            && seasons.paschal_triduum.is_none()
            && seasons.easter_time.is_none()
        {
            metadata.seasons = None;
        }
    }
}

/// Extract all parent locales from a BCP 47 locale tag in hierarchy order
/// For example: "fr-CA-fonipa" -> ["fr", "fr-CA"]
///              "zh-Hant-TW" -> ["zh", "zh-Hant"]
///              "fr" -> []
fn get_all_parent_locales(locale: &str) -> Vec<String> {
    let parts: Vec<&str> = locale.split('-').collect();
    let mut parents = Vec::new();

    // Generate all possible parent locales by progressively removing the last part
    for i in 1..parts.len() {
        let parent = parts[..parts.len() - i].join("-");
        parents.push(parent);
    }

    parents
}

/// Filter calendar_definitions to keep only:
/// 1. The main calendar (config.calendar)
/// 2. Parent calendars of the main calendar
/// 3. The general_roman calendar
///
/// Returns them ordered according to the priority in keep_ids
/// Returns an error if the main calendar is not found
fn filter_calendar_definitions(romcal: &Romcal) -> RomcalResult<Vec<CalendarDefinition>> {
    // Find the main calendar and its parents
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

    // Collect all required calendar IDs (most specific to most general)
    let mut required_ids = Vec::new();

    // Add main calendar first (most specific)
    required_ids.push(main_calendar.id.clone());

    // Add parent calendars (from most specific to most general)
    for parent_id in main_calendar.parent_calendar_ids.iter().rev() {
        if !required_ids.contains(parent_id) {
            required_ids.push(parent_id.clone());
        }
    }

    // Add general_roman last (most general fallback)
    if !required_ids.contains(&"general_roman".to_string()) {
        required_ids.push("general_roman".to_string());
    }

    // Validate that the main calendar is not in its own parent list (circular reference)
    if main_calendar
        .parent_calendar_ids
        .contains(&main_calendar.id)
    {
        return Err(RomcalError::ValidationError(format!(
            "Main calendar '{}' cannot be its own parent (circular reference detected)",
            main_calendar.id
        )));
    }

    // Validate that all required calendars exist
    let available_ids: EntityIdSet = romcal
        .calendar_definitions
        .iter()
        .map(|cal| cal.id.clone())
        .collect();

    for required_id in &required_ids {
        if !available_ids.contains(required_id) {
            return Err(RomcalError::ValidationError(format!(
                "Required calendar '{}' not found in calendar_definitions",
                required_id
            )));
        }
    }

    // Validate that the main calendar is the first in the hierarchy (most specific)
    if required_ids.len() > 1 {
        let first_id = required_ids.first().unwrap();
        if first_id != &main_calendar.id {
            return Err(RomcalError::ValidationError(format!(
                "Main calendar '{}' must be the first in the hierarchy, but found '{}' at the beginning",
                main_calendar.id, first_id
            )));
        }
    }

    // Filter and order calendar_definitions according to required_ids order
    let mut result = Vec::new();
    for id in required_ids {
        if let Some(calendar_def) = romcal.calendar_definitions.iter().find(|cal| cal.id == id) {
            result.push(calendar_def.clone());
        }
    }

    Ok(result)
}

/// Recursively removes null values, empty objects, and $schema properties from a JSON Value
fn remove_null_and_empty_values(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut cleaned_map = serde_json::Map::new();
            for (key, val) in map {
                // Skip $schema properties
                if key == "$schema" {
                    continue;
                }
                let cleaned_val = remove_null_and_empty_values(val);
                if !cleaned_val.is_null() {
                    cleaned_map.insert(key, cleaned_val);
                }
            }
            // Return null if the object is empty after cleaning, so it gets filtered out
            if cleaned_map.is_empty() {
                Value::Null
            } else {
                Value::Object(cleaned_map)
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
        Value::Null => Value::Null, // This value will be filtered by parent calls
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::calendar::{CalendarJurisdiction, CalendarType, DayDefinition, EntityRef};
    use crate::types::entity::EntityOverride;
    use crate::types::entity::EntityType;

    fn create_test_calendar_definition() -> CalendarDefinition {
        CalendarDefinition {
            schema: None,
            id: "test_calendar".to_string(),
            metadata: crate::types::CalendarMetadata {
                jurisdiction: CalendarJurisdiction::Ecclesiastical,
                r#type: CalendarType::Diocese,
            },
            particular_config: None,
            parent_calendar_ids: vec![],
            days_definitions: {
                let mut map = std::collections::BTreeMap::new();
                map.insert(
                    "saint_john".to_string(),
                    DayDefinition {
                        date_def: None,
                        date_exceptions: None,
                        precedence: None,
                        commons_def: None,
                        is_holy_day_of_obligation: None,
                        allow_similar_rank_items: None,
                        is_optional: None,
                        custom_locale_id: None,
                        entities: Some(vec![
                            EntityRef::ResourceId("john_the_baptist".to_string()),
                            EntityRef::Override(EntityOverride {
                                id: "john_the_evangelist".to_string(),
                                titles: None,
                                hide_titles: None,
                                count: None,
                            }),
                        ]),
                        titles: None,
                        drop: None,
                        colors: None,
                        masses: None,
                    },
                );
                map.insert(
                    "saint_peter".to_string(),
                    DayDefinition {
                        date_def: None,
                        date_exceptions: None,
                        precedence: None,
                        commons_def: None,
                        is_holy_day_of_obligation: None,
                        allow_similar_rank_items: None,
                        is_optional: None,
                        custom_locale_id: None,
                        entities: None,
                        titles: None,
                        drop: None,
                        colors: None,
                        masses: None,
                    },
                );
                map
            },
        }
    }

    fn create_test_resources() -> Resources {
        let mut entities = std::collections::BTreeMap::new();

        entities.insert(
            "john_the_baptist".to_string(),
            crate::types::entity::Entity {
                id: Some("john_the_baptist".to_string()),
                r#type: Some(EntityType::Person),
                fullname: Some("John the Baptist".to_string()),
                name: Some("John".to_string()),
                canonization_level: None,
                date_of_canonization: None,
                date_of_canonization_is_approximative: None,
                date_of_beatification: None,
                date_of_beatification_is_approximative: None,
                hide_canonization_level: None,
                titles: None,
                sex: None,
                hide_titles: None,
                date_of_dedication: None,
                date_of_birth: None,
                date_of_birth_is_approximative: None,
                date_of_death: None,
                date_of_death_is_approximative: None,
                count: None,
                sources: None,
                _todo: None,
            },
        );

        entities.insert(
            "john_the_evangelist".to_string(),
            crate::types::entity::Entity {
                id: Some("john_the_evangelist".to_string()),
                r#type: Some(EntityType::Person),
                fullname: Some("John the Evangelist".to_string()),
                name: Some("John".to_string()),
                canonization_level: None,
                date_of_canonization: None,
                date_of_canonization_is_approximative: None,
                date_of_beatification: None,
                date_of_beatification_is_approximative: None,
                hide_canonization_level: None,
                titles: None,
                sex: None,
                hide_titles: None,
                date_of_dedication: None,
                date_of_birth: None,
                date_of_birth_is_approximative: None,
                date_of_death: None,
                date_of_death_is_approximative: None,
                count: None,
                sources: None,
                _todo: None,
            },
        );

        entities.insert(
            "unused_entity".to_string(),
            crate::types::entity::Entity {
                id: Some("unused_entity".to_string()),
                r#type: Some(EntityType::Person),
                fullname: Some("Unused Entity".to_string()),
                name: Some("Unused".to_string()),
                canonization_level: None,
                date_of_canonization: None,
                date_of_canonization_is_approximative: None,
                date_of_beatification: None,
                date_of_beatification_is_approximative: None,
                hide_canonization_level: None,
                titles: None,
                sex: None,
                hide_titles: None,
                date_of_dedication: None,
                date_of_birth: None,
                date_of_birth_is_approximative: None,
                date_of_death: None,
                date_of_death_is_approximative: None,
                count: None,
                sources: None,
                _todo: None,
            },
        );

        Resources {
            schema: None,
            locale: "en".to_string(),
            metadata: None,
            entities: Some(entities),
        }
    }

    #[test]
    fn test_collect_used_entity_ids() {
        let calendar_definitions = vec![create_test_calendar_definition()];
        let used_entity_ids = collect_used_entity_ids(&calendar_definitions);

        // Should include day_definition IDs
        assert!(used_entity_ids.contains("saint_john"));
        assert!(used_entity_ids.contains("saint_peter"));

        // Should include ResourceId references
        assert!(used_entity_ids.contains("john_the_baptist"));

        // Should include Override entity IDs
        assert!(used_entity_ids.contains("john_the_evangelist"));

        // Should not include entities not referenced
        assert!(!used_entity_ids.contains("unused_entity"));
    }

    #[test]
    fn test_collect_used_entity_ids_empty_entities() {
        let calendar_definitions = vec![CalendarDefinition {
            schema: None,
            id: "test_calendar".to_string(),
            metadata: crate::types::CalendarMetadata {
                jurisdiction: CalendarJurisdiction::Ecclesiastical,
                r#type: CalendarType::Diocese,
            },
            particular_config: None,
            parent_calendar_ids: vec![],
            days_definitions: {
                let mut map = std::collections::BTreeMap::new();
                map.insert(
                    "saint_mary".to_string(),
                    DayDefinition {
                        date_def: None,
                        date_exceptions: None,
                        precedence: None,
                        commons_def: None,
                        is_holy_day_of_obligation: None,
                        allow_similar_rank_items: None,
                        is_optional: None,
                        custom_locale_id: None,
                        entities: None, // No entities
                        titles: None,
                        drop: None,
                        colors: None,
                        masses: None,
                    },
                );
                map
            },
        }];

        let used_entity_ids = collect_used_entity_ids(&calendar_definitions);

        // Should only include the day_definition ID
        assert!(used_entity_ids.contains("saint_mary"));
        assert_eq!(used_entity_ids.len(), 1);
    }

    #[test]
    fn test_filter_entities_by_usage() {
        let mut resources = create_test_resources();
        let used_entity_ids: EntityIdSet = ["john_the_baptist", "john_the_evangelist"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        filter_entities_by_usage(&mut resources, &used_entity_ids);

        let entities = resources.entities.unwrap();
        assert_eq!(entities.len(), 2);

        let entity_ids: Vec<String> = entities.keys().cloned().collect();
        assert!(entity_ids.contains(&"john_the_baptist".to_string()));
        assert!(entity_ids.contains(&"john_the_evangelist".to_string()));
        assert!(!entity_ids.contains(&"unused_entity".to_string()));
    }

    #[test]
    fn test_filter_entities_by_usage_empty_used_ids() {
        let mut resources = create_test_resources();
        let used_entity_ids = EntityIdSet::new();

        filter_entities_by_usage(&mut resources, &used_entity_ids);

        let entities = resources.entities.unwrap();
        assert_eq!(entities.len(), 0);
    }

    #[test]
    fn test_filter_entities_by_usage_no_entities() {
        let mut resources = Resources {
            schema: None,
            locale: "en".to_string(),
            metadata: None,
            entities: None,
        };
        let used_entity_ids: EntityIdSet = ["some_entity"].iter().map(|s| s.to_string()).collect();

        // Should not panic when entities is None
        filter_entities_by_usage(&mut resources, &used_entity_ids);
        assert!(resources.entities.is_none());
    }
}
