use crate::error::RomcalCliError;
use chrono::Datelike;
use glob::glob;
use std::fs;
use std::path::Path;

// =================================================================================
// Private utility functions
// =================================================================================

/// Check if a file has a .json extension (case insensitive)
fn is_json_file(path: &Path) -> bool {
    path.extension()
        .and_then(|s| s.to_str())
        .map(|ext| ext.to_lowercase())
        .as_deref()
        == Some("json")
}

/// Helper function to try adding a valid JSON file to the collection
fn try_add_valid_json_file(files: &mut Vec<std::path::PathBuf>, path: std::path::PathBuf) {
    if let Err(e) = read_json_file(&path.to_string_lossy()) {
        eprintln!("⚠️  Skipping invalid JSON file {}: {}", path.display(), e);
        return;
    }
    files.push(path);
}

// =================================================================================
// Main public functions
// =================================================================================

/// Read and parse a JSON file, returning the parsed value
/// Validates that the file is a valid JSON file
pub fn read_json_file(file_path: &str) -> Result<serde_json::Value, RomcalCliError> {
    let path = Path::new(file_path);

    if !path.exists() {
        return Err(RomcalCliError::config_error(format!(
            "File does not exist: {}",
            file_path
        )));
    }

    if !path.is_file() {
        return Err(RomcalCliError::config_error(format!(
            "Path is not a file: {}",
            file_path
        )));
    }

    if !is_json_file(path) {
        return Err(RomcalCliError::config_error(format!(
            "File is not a JSON file: {}",
            file_path
        )));
    }

    let content = std::fs::read_to_string(file_path).map_err(|e| {
        RomcalCliError::config_error(format!("Failed to read file '{}': {}", file_path, e))
    })?;

    serde_json::from_str(&content).map_err(|e| {
        RomcalCliError::config_error(format!("Invalid JSON in file '{}': {}", file_path, e))
    })
}

/// Collect JSON files based on patterns (supports glob patterns and directories)
/// Returns a list of PathBuf for valid JSON files
pub fn collect_json_files(patterns: &[String]) -> Result<Vec<std::path::PathBuf>, RomcalCliError> {
    let mut files = Vec::new();

    for pattern in patterns {
        // Check if it's a glob pattern (contains * or **)
        if pattern.contains('*') {
            // It's a glob pattern
            for entry in glob(pattern)
                .map_err(|e| RomcalCliError::config_error(format!("Invalid glob pattern: {}", e)))?
            {
                match entry {
                    Ok(path) => {
                        if path.is_file() && is_json_file(&path) {
                            try_add_valid_json_file(&mut files, path);
                        }
                    }
                    Err(e) => eprintln!("⚠️  Error reading glob entry: {}", e),
                }
            }
        } else {
            // It's a single file path
            let path = Path::new(pattern);
            if path.is_file() && is_json_file(path) {
                try_add_valid_json_file(&mut files, path.to_path_buf());
            } else if path.is_dir() {
                // If it's a directory, find all JSON files in it
                for entry in fs::read_dir(path).map_err(|e| {
                    RomcalCliError::config_error(format!("Cannot read directory: {}", e))
                })? {
                    let entry = entry.map_err(|e| {
                        RomcalCliError::config_error(format!("Cannot read directory entry: {}", e))
                    })?;
                    let path = entry.path();
                    if path.is_file() && is_json_file(&path) {
                        try_add_valid_json_file(&mut files, path);
                    }
                }
            } else {
                return Err(RomcalCliError::config_error(format!(
                    "File does not exist: {}",
                    pattern
                )));
            }
        }
    }

    if files.is_empty() {
        return Err(RomcalCliError::config_error(format!(
            "No valid JSON files found matching: {}",
            patterns.join(", ")
        )));
    }

    Ok(files)
}

/// Collect JSON file paths from patterns (supports glob patterns and directories)
pub fn collect_json_file_paths(patterns: &[String]) -> Result<Vec<String>, RomcalCliError> {
    let files = collect_json_files(patterns)?;

    // Convert PathBuf to String
    let file_strings: Vec<String> = files
        .into_iter()
        .map(|path| path.to_string_lossy().to_string())
        .collect();

    Ok(file_strings)
}

// =================================================================================
// Data loading functions
// =================================================================================

/// Parse calendar definition files and return a Vec of CalendarDefinition
pub fn parse_calendar_definition_files(
    file_paths: &[String],
) -> Result<Vec<romcal_core::CalendarDefinition>, RomcalCliError> {
    let mut calendar_definitions = Vec::new();

    for file_path in file_paths {
        let json_value = read_json_file(file_path)?;

        // Deserialize JSON value to CalendarDefinition
        let calendar_def: romcal_core::CalendarDefinition = serde_json::from_value(json_value)?;

        calendar_definitions.push(calendar_def);
    }

    Ok(calendar_definitions)
}

/// Parse resource files and return a Vec of Resources
pub fn parse_resource_files(
    file_paths: &[String],
) -> Result<Vec<romcal_core::Resources>, RomcalCliError> {
    let mut resource_definitions = Vec::new();

    for file_path in file_paths {
        let json_value = read_json_file(file_path)?;

        // Deserialize JSON value to Resources
        let resource_def: romcal_core::Resources = serde_json::from_value(json_value)?;

        resource_definitions.push(resource_def);
    }

    Ok(resource_definitions)
}

// =================================================================================
// Public utility functions
// =================================================================================

/// Get current year
pub fn current_year() -> i32 {
    chrono::Utc::now().year()
}

/// Validate a year using romcal_core's validation function
pub fn validate_year(year: i32) -> Result<(), RomcalCliError> {
    romcal_core::validate_year(year, 1583).map_err(|e| match e {
        romcal_core::RomcalError::InvalidYear(year) => RomcalCliError::invalid_year(year),
        _ => RomcalCliError::config_error(format!("Year validation error: {}", e)),
    })
}

// =================================================================================
// Resources combination functions
// =================================================================================

/// Combine multiple Resources by locale
/// Groups resources by locale and merges them together:
/// - Deep merge of metadata properties
/// - Concatenation of entities arrays
pub fn combine_resources_by_locale(
    resources: Vec<romcal_core::Resources>,
) -> Result<Vec<romcal_core::Resources>, RomcalCliError> {
    use serde_json::{from_value, to_value};
    use std::collections::HashMap;

    let mut grouped_by_locale: HashMap<String, Vec<romcal_core::Resources>> = HashMap::new();

    // Group resources by locale
    for resource in resources {
        let locale = resource.locale.clone();
        grouped_by_locale.entry(locale).or_default().push(resource);
    }

    let mut combined_resources = Vec::new();

    // Combine resources for each locale
    for (_locale, mut locale_resources) in grouped_by_locale {
        if locale_resources.is_empty() {
            continue;
        }

        // Start with the first resource as base
        let mut combined = locale_resources.remove(0);

        // Merge all other resources for this locale
        for resource in locale_resources {
            // Ensure locales match
            if combined.locale != resource.locale {
                return Err(RomcalCliError::config_error(format!(
                    "Cannot merge resources with different locales: '{}' and '{}'",
                    combined.locale, resource.locale
                )));
            }

            // Deep merge metadata using custom JSON merge (handles HashMap and arrays correctly)
            if let (Some(target_metadata), Some(source_metadata)) =
                (&combined.metadata, &resource.metadata)
            {
                // Convert to JSON, merge, then convert back
                let mut target_json = to_value(target_metadata)?;
                let source_json = to_value(source_metadata)?;
                merge_json_values(&mut target_json, source_json);
                let merged_metadata: romcal_core::types::resource::ResourcesMetadata =
                    from_value(target_json)?;

                combined.metadata = Some(merged_metadata);
            } else if resource.metadata.is_some() {
                combined.metadata = resource.metadata;
            }

            // Concatenate entities manually
            if let Some(source_entities) = resource.entities {
                let target_entities = combined
                    .entities
                    .get_or_insert_with(std::collections::BTreeMap::new);
                target_entities.extend(source_entities);
            }
        }

        combined_resources.push(combined);
    }

    Ok(combined_resources)
}

/// Merge two JSON values recursively
/// - Objects: deep merge, source overwrites target (except null values)
/// - Arrays: concatenate (not replace)
/// - Primitives: source wins (except null values preserve target)
fn merge_json_values(target: &mut serde_json::Value, source: serde_json::Value) {
    match (target, source) {
        (serde_json::Value::Object(target_map), serde_json::Value::Object(source_map)) => {
            for (key, source_value) in source_map {
                // Skip null values - don't let them override existing values
                if source_value.is_null() {
                    continue;
                }

                match target_map.get_mut(&key) {
                    Some(target_value) => {
                        merge_json_values(target_value, source_value);
                    }
                    None => {
                        target_map.insert(key, source_value);
                    }
                }
            }
        }
        (serde_json::Value::Array(target_array), serde_json::Value::Array(source_array)) => {
            // Concatenate arrays instead of replacing
            target_array.extend(source_array);
        }
        (target_value, source_value) => {
            // Skip null values - don't let them override existing values
            if !source_value.is_null() {
                *target_value = source_value;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    fn create_test_entity(_id: &str, name: &str) -> romcal_core::Entity {
        let mut entity = romcal_core::Entity::new();
        entity.name = Some(name.to_string());
        entity
    }

    fn create_test_entities_map(
        entities: Vec<(&str, &str)>,
    ) -> BTreeMap<String, romcal_core::Entity> {
        let mut map = BTreeMap::new();
        for (id, name) in entities {
            map.insert(id.to_string(), create_test_entity(id, name));
        }
        map
    }

    fn create_test_resources_definition(
        locale: &str,
        entities: BTreeMap<String, romcal_core::Entity>,
    ) -> romcal_core::Resources {
        let mut resources = romcal_core::Resources::new(locale.to_string());
        resources.entities = Some(entities);
        resources
    }

    fn create_test_resources_definition_with_metadata(
        locale: &str,
        entities: BTreeMap<String, romcal_core::Entity>,
        metadata: romcal_core::types::resource::ResourcesMetadata,
    ) -> romcal_core::Resources {
        let mut resources = romcal_core::Resources::new(locale.to_string());
        resources.entities = Some(entities);
        resources.metadata = Some(metadata);
        resources
    }

    #[test]
    fn test_combine_resources_by_locale_empty_input() {
        let result = combine_resources_by_locale(vec![]).unwrap();
        assert!(result.is_empty());
    }

    #[test]
    fn test_combine_resources_by_locale_single_locale() {
        let resources = vec![
            create_test_resources_definition(
                "fr",
                create_test_entities_map(vec![("entity1", "Entity 1")]),
            ),
            create_test_resources_definition(
                "fr",
                create_test_entities_map(vec![("entity2", "Entity 2")]),
            ),
        ];

        let result = combine_resources_by_locale(resources).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].locale, "fr");
        assert_eq!(result[0].entities.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_combine_resources_by_locale_multiple_locales() {
        let resources = vec![
            create_test_resources_definition(
                "fr",
                create_test_entities_map(vec![("entity1", "Entity 1")]),
            ),
            create_test_resources_definition(
                "en",
                create_test_entities_map(vec![("entity2", "Entity 2")]),
            ),
            create_test_resources_definition(
                "fr",
                create_test_entities_map(vec![("entity3", "Entity 3")]),
            ),
        ];

        let result = combine_resources_by_locale(resources).unwrap();
        assert_eq!(result.len(), 2);

        // Find the French resources
        let fr_resources = result.iter().find(|r| r.locale == "fr").unwrap();
        assert_eq!(fr_resources.entities.as_ref().unwrap().len(), 2);

        // Find the English resources
        let en_resources = result.iter().find(|r| r.locale == "en").unwrap();
        assert_eq!(en_resources.entities.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_combine_resources_by_locale_metadata_merge() {
        use romcal_core::types::resource::*;

        let metadata1 = ResourcesMetadata {
            ordinal_format: None,
            ordinals_letters: Some({
                let mut map = BTreeMap::new();
                map.insert("1st".to_string(), "premier".to_string());
                map
            }),
            ordinals_numeric: None,
            weekdays: None,
            months: None,
            colors: None,
            seasons: None,
            periods: None,
            ranks: None,
            cycles: None,
        };

        let metadata2 = ResourcesMetadata {
            ordinal_format: None,
            ordinals_letters: Some({
                let mut map = BTreeMap::new();
                map.insert("2nd".to_string(), "deuxième".to_string());
                map
            }),
            ordinals_numeric: None,
            weekdays: Some({
                let mut map = BTreeMap::new();
                map.insert("monday".to_string(), "lundi".to_string());
                map
            }),
            months: None,
            colors: None,
            seasons: None,
            periods: None,
            ranks: None,
            cycles: None,
        };

        let resources = vec![
            create_test_resources_definition_with_metadata("fr", BTreeMap::new(), metadata1),
            create_test_resources_definition_with_metadata("fr", BTreeMap::new(), metadata2),
        ];

        let result = combine_resources_by_locale(resources).unwrap();
        assert_eq!(result.len(), 1);

        let combined_metadata = result[0].metadata.as_ref().unwrap();
        assert_eq!(
            combined_metadata.ordinals_letters.as_ref().unwrap().len(),
            2
        );
        assert!(combined_metadata
            .ordinals_letters
            .as_ref()
            .unwrap()
            .contains_key("1st"));
        assert!(combined_metadata
            .ordinals_letters
            .as_ref()
            .unwrap()
            .contains_key("2nd"));
        assert!(combined_metadata
            .weekdays
            .as_ref()
            .unwrap()
            .contains_key("monday"));
    }

    #[test]
    fn test_combine_resources_by_locale_entities_concatenation() {
        let resources = vec![
            create_test_resources_definition(
                "fr",
                create_test_entities_map(vec![("entity1", "Entity 1"), ("entity2", "Entity 2")]),
            ),
            create_test_resources_definition(
                "fr",
                create_test_entities_map(vec![("entity3", "Entity 3")]),
            ),
        ];

        let result = combine_resources_by_locale(resources).unwrap();
        assert_eq!(result.len(), 1);

        let entities = result[0].entities.as_ref().unwrap();
        assert_eq!(entities.len(), 3);
        assert!(entities.contains_key("entity1"));
        assert!(entities.contains_key("entity2"));
        assert!(entities.contains_key("entity3"));
    }

    #[test]
    fn test_combine_resources_by_locale_different_locales_error() {
        // This test verifies that we can't accidentally merge resources with different locales
        // The function should group by locale, so this should work fine
        let resources = vec![
            create_test_resources_definition(
                "fr",
                create_test_entities_map(vec![("entity1", "Entity 1")]),
            ),
            create_test_resources_definition(
                "en",
                create_test_entities_map(vec![("entity2", "Entity 2")]),
            ),
        ];

        let result = combine_resources_by_locale(resources).unwrap();
        assert_eq!(result.len(), 2); // Should have 2 separate resources, not merged
    }

    #[test]
    fn test_combine_resources_by_locale_ignore_null_values() {
        use romcal_core::types::resource::*;

        let metadata1 = ResourcesMetadata {
            ordinal_format: None,
            ordinals_letters: Some({
                let mut map = BTreeMap::new();
                map.insert("1st".to_string(), "premier".to_string());
                map
            }),
            ordinals_numeric: None,
            weekdays: Some({
                let mut map = BTreeMap::new();
                map.insert("monday".to_string(), "lundi".to_string());
                map
            }),
            months: None,
            colors: None,
            seasons: None,
            periods: None,
            ranks: None,
            cycles: None,
        };

        // Second metadata with null values that should not override the first
        let metadata2 = ResourcesMetadata {
            ordinal_format: None,
            ordinals_letters: Some({
                let mut map = BTreeMap::new();
                map.insert("2nd".to_string(), "deuxième".to_string());
                map
            }),
            ordinals_numeric: None,
            weekdays: None, // This null should not override the existing weekdays
            months: Some({
                let mut map = BTreeMap::new();
                map.insert("january".to_string(), "janvier".to_string());
                map
            }),
            colors: None,
            seasons: None,
            periods: None,
            ranks: None,
            cycles: None,
        };

        let resources = vec![
            create_test_resources_definition_with_metadata("fr", BTreeMap::new(), metadata1),
            create_test_resources_definition_with_metadata("fr", BTreeMap::new(), metadata2),
        ];

        let result = combine_resources_by_locale(resources).unwrap();
        assert_eq!(result.len(), 1);

        let combined_metadata = result[0].metadata.as_ref().unwrap();

        // ordinals_letters should have both values
        assert_eq!(
            combined_metadata.ordinals_letters.as_ref().unwrap().len(),
            2
        );
        assert!(combined_metadata
            .ordinals_letters
            .as_ref()
            .unwrap()
            .contains_key("1st"));
        assert!(combined_metadata
            .ordinals_letters
            .as_ref()
            .unwrap()
            .contains_key("2nd"));

        // weekdays should still have the original value (not overridden by null)
        assert!(combined_metadata
            .weekdays
            .as_ref()
            .unwrap()
            .contains_key("monday"));
        assert_eq!(combined_metadata.weekdays.as_ref().unwrap().len(), 1);

        // months should have the new value
        assert!(combined_metadata
            .months
            .as_ref()
            .unwrap()
            .contains_key("january"));
        assert_eq!(combined_metadata.months.as_ref().unwrap().len(), 1);
    }
}
