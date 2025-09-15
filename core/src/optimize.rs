use serde_json::Value;
use std::collections::{HashMap, HashSet};

use crate::{CalendarDefinition, Preset, ResourcesDefinition, RomcalError, RomcalResult};

// Type aliases for clarity
type LocaleMap = HashMap<String, String>;
type EntityIdSet = HashSet<String>;
type PropertySet = HashSet<String>;

// Constants for metadata properties
const METADATA_PROPERTIES: &[&str] = &[
    "ordinals", "weekdays", "months", "colors", "periods", "ranks", "cycles",
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
/// This method serializes the Preset to JSON format
/// and removes null values and empty objects from the output.
///
/// Only includes calendar_definitions that are:
/// 1. The main calendar (Preset.calendar)
/// 2. Parent calendars of the main calendar
/// 3. The general_roman calendar
pub fn optimize(preset: &Preset) -> RomcalResult<String> {
    // Validate that all calendar IDs are unique
    validate_unique_calendar_ids(&preset.calendar_definitions)?;

    // Validate that all day definition IDs are unique within each calendar
    validate_unique_day_ids(&preset.calendar_definitions)?;

    // Validate that all resource locales are unique
    validate_unique_resource_locales(&preset.resources)?;

    // Validate that all entity IDs are unique within each resource definition
    validate_unique_entity_ids(&preset.resources)?;

    // Create a filtered version of the config with only relevant calendar_definitions and resources
    let mut filtered_config = preset.clone();
    filtered_config.calendar_definitions = filter_calendar_definitions(preset)?;
    filtered_config.resources = filter_resources(preset)?;

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
fn validate_unique_resource_locales(resources: &[ResourcesDefinition]) -> RomcalResult<()> {
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

/// Validate that all day definition IDs are unique within each calendar definition
/// Returns an error if duplicate day definition IDs are found in any calendar
fn validate_unique_day_ids(calendar_definitions: &[CalendarDefinition]) -> RomcalResult<()> {
    for calendar_def in calendar_definitions {
        let mut seen_ids = EntityIdSet::new();

        for day_def in &calendar_def.days_definitions {
            if !seen_ids.insert(day_def.id.clone()) {
                return Err(RomcalError::ValidationError(format!(
                    "Duplicate day definition ID '{}' found in calendar '{}'. Each day definition must have a unique ID within a calendar.",
                    day_def.id, calendar_def.id
                )));
            }
        }
    }

    Ok(())
}

/// Validate that all entity IDs are unique within each resource definition
/// Returns an error if duplicate entity IDs are found in any resource
fn validate_unique_entity_ids(resources: &[ResourcesDefinition]) -> RomcalResult<()> {
    for resource in resources {
        if let Some(entities) = &resource.entities {
            let mut seen_ids = EntityIdSet::new();

            for entity in entities {
                if !seen_ids.insert(entity.id.clone()) {
                    return Err(RomcalError::ValidationError(format!(
                        "Duplicate entity ID '{}' found in resource '{}'. Each entity must have a unique ID within a resource.",
                        entity.id, resource.locale
                    )));
                }
            }
        }
    }

    Ok(())
}

/// Filter resources to keep only the required locales based on the preset
/// Returns resources with hierarchical deduplication: most specific to most general
/// Entities defined in more specific locales are removed from parent locales
fn filter_resources(preset: &Preset) -> RomcalResult<Vec<ResourcesDefinition>> {
    let target_locale = &preset.locale;

    // Build locale maps for efficient lookups
    let (available_locales, resources_by_locale) = build_locale_maps(preset);

    // Validate target locale exists
    let exact_locale = validate_target_locale(target_locale, &available_locales)?;

    // Build priority list of locales (most specific to most general)
    let priority_locales = build_priority_locales(target_locale, &available_locales, &exact_locale);

    // Apply hierarchical deduplication
    apply_hierarchical_deduplication(priority_locales, &resources_by_locale)
}

/// Build locale maps for efficient lookups
fn build_locale_maps(preset: &Preset) -> (LocaleMap, HashMap<&str, &ResourcesDefinition>) {
    let available_locales: LocaleMap = preset
        .resources
        .iter()
        .map(|resource| (resource.locale.to_lowercase(), resource.locale.clone()))
        .collect();

    let resources_by_locale: HashMap<&str, &ResourcesDefinition> = preset
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
        if parent != target_locale {
            if let Some(parent_locale_actual) = available_locales.get(&parent.to_lowercase()) {
                priority_locales.push(parent_locale_actual.clone());
            }
        }
    }

    // 3. Always include "en" last (most general fallback)
    if let Some(en_locale) = available_locales.get("en") {
        if !priority_locales.contains(en_locale) {
            priority_locales.push(en_locale.clone());
        }
    }

    priority_locales
}

/// Apply hierarchical deduplication to resources
fn apply_hierarchical_deduplication(
    priority_locales: Vec<String>,
    resources_by_locale: &HashMap<&str, &ResourcesDefinition>,
) -> RomcalResult<Vec<ResourcesDefinition>> {
    let mut result = Vec::new();
    let mut defined_entity_ids = EntityIdSet::new();
    let mut defined_metadata_properties = PropertySet::new();
    let mut defined_seasons_properties = PropertySet::new();

    for locale in priority_locales {
        if let Some(resource) = resources_by_locale.get(locale.as_str()) {
            let mut filtered_resource = (*resource).clone();

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

/// Deduplicate entities in a resource
fn deduplicate_entities(resource: &mut ResourcesDefinition, defined_entity_ids: &mut EntityIdSet) {
    if let Some(entities) = &mut resource.entities {
        entities.retain(|entity| {
            if defined_entity_ids.contains(&entity.id) {
                false // Remove entity already defined in more specific locale
            } else {
                defined_entity_ids.insert(entity.id.clone());
                true // Keep entity and mark it as defined
            }
        });
    }
}

/// Deduplicate metadata in a resource
fn deduplicate_metadata(
    resource: &mut ResourcesDefinition,
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
        (METADATA_PROPERTIES[0], metadata.ordinals.is_some()),
        (METADATA_PROPERTIES[1], metadata.weekdays.is_some()),
        (METADATA_PROPERTIES[2], metadata.months.is_some()),
        (METADATA_PROPERTIES[3], metadata.colors.is_some()),
        (METADATA_PROPERTIES[4], metadata.periods.is_some()),
        (METADATA_PROPERTIES[5], metadata.ranks.is_some()),
        (METADATA_PROPERTIES[6], metadata.cycles.is_some()),
    ];

    for (prop_name, is_defined) in first_level_props {
        if is_defined {
            if defined_properties.contains(prop_name) {
                // Remove property already defined in more specific locale
                match prop_name {
                    "ordinals" => metadata.ordinals = None,
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
fn filter_calendar_definitions(preset: &Preset) -> RomcalResult<Vec<CalendarDefinition>> {
    // Find the main calendar and its parents
    let main_calendar = preset
        .calendar_definitions
        .iter()
        .find(|cal| cal.id == preset.calendar)
        .ok_or_else(|| {
            RomcalError::ValidationError(format!(
                "Main calendar '{}' not found in calendar_definitions",
                preset.calendar
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
    let available_ids: EntityIdSet = preset
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
        if let Some(calendar_def) = preset.calendar_definitions.iter().find(|cal| cal.id == id) {
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
