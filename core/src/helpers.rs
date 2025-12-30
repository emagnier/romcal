//! Helper functions for merging resource and calendar definition files.
//!
//! These helpers allow users to load data files however they want (fetch, import, fs, etc.)
//! and then use romcal to merge them into the expected structures.

use std::collections::BTreeMap;

use serde::Deserialize;

use crate::engine::calendar_definition::CalendarDefinition;
use crate::engine::resources::Resources;
use crate::error::RomcalError;
use crate::types::EntityId;
use crate::types::entity::Entity;
use crate::types::resource::ResourcesMetadata;

/// Intermediate structure for parsing resource files.
/// Handles both meta.json (with metadata) and entities.*.json (with entities).
#[derive(Debug, Deserialize)]
struct ResourceFile {
    #[serde(rename = "$schema")]
    #[allow(dead_code)]
    schema: Option<String>,
    #[allow(dead_code)]
    locale: Option<String>,
    metadata: Option<ResourcesMetadata>,
    entities: Option<BTreeMap<EntityId, Entity>>,
}

/// Merge multiple resource files (meta.json + entities.*.json) into a single Resources object.
///
/// # Arguments
///
/// * `locale` - The locale code (e.g., "fr", "en")
/// * `files_json` - A list of JSON strings, each representing a resource file
///
/// # Returns
///
/// A merged Resources object with combined metadata and entities.
///
/// # Example
///
/// ```ignore
/// let meta = r#"{"locale": "fr", "metadata": {...}}"#;
/// let entities = r#"{"locale": "fr", "entities": {...}}"#;
/// let resources = merge_resource_files("fr", vec![meta, entities])?;
/// ```
pub fn merge_resource_files(locale: &str, files_json: Vec<&str>) -> Result<Resources, RomcalError> {
    let mut metadata: Option<ResourcesMetadata> = None;
    let mut entities: BTreeMap<EntityId, Entity> = BTreeMap::new();

    for file_json in files_json {
        let file: ResourceFile = serde_json::from_str(file_json).map_err(|e| {
            RomcalError::ValidationError(format!("Failed to parse resource file: {}", e))
        })?;

        // Extract metadata if present
        if let Some(file_metadata) = file.metadata {
            metadata = Some(file_metadata);
        }

        // Merge entities if present
        if let Some(file_entities) = file.entities {
            entities.extend(file_entities);
        }
    }

    Ok(Resources {
        schema: None,
        locale: locale.to_string(),
        metadata,
        entities: if entities.is_empty() {
            None
        } else {
            Some(entities)
        },
    })
}

/// Merge/validate multiple calendar definition files.
///
/// # Arguments
///
/// * `files_json` - A list of JSON strings, each representing a calendar definition
///
/// # Returns
///
/// A vector of validated CalendarDefinition objects.
///
/// # Example
///
/// ```ignore
/// let france = r#"{"id": "france", ...}"#;
/// let usa = r#"{"id": "usa", ...}"#;
/// let definitions = merge_calendar_definitions(vec![france, usa])?;
/// ```
pub fn merge_calendar_definitions(
    files_json: Vec<&str>,
) -> Result<Vec<CalendarDefinition>, RomcalError> {
    let mut definitions: Vec<CalendarDefinition> = Vec::with_capacity(files_json.len());

    for file_json in files_json {
        let definition: CalendarDefinition = serde_json::from_str(file_json).map_err(|e| {
            RomcalError::ValidationError(format!("Failed to parse calendar definition: {}", e))
        })?;
        definitions.push(definition);
    }

    Ok(definitions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_resource_files_empty() {
        let result = merge_resource_files("en", vec![]);
        assert!(result.is_ok());
        let resources = result.unwrap();
        assert_eq!(resources.locale, "en");
        assert!(resources.metadata.is_none());
        assert!(resources.entities.is_none());
    }

    #[test]
    fn test_merge_resource_files_with_entities() {
        let entities_json = r#"{
            "locale": "en",
            "entities": {
                "saint_peter": {
                    "fullname": "Saint Peter"
                }
            }
        }"#;

        let result = merge_resource_files("en", vec![entities_json]);
        assert!(result.is_ok());
        let resources = result.unwrap();
        assert_eq!(resources.locale, "en");
        assert!(resources.entities.is_some());
        assert!(resources.entities.unwrap().contains_key("saint_peter"));
    }

    #[test]
    fn test_merge_calendar_definitions_empty() {
        let result = merge_calendar_definitions(vec![]);
        assert!(result.is_ok());
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn test_merge_calendar_definitions_invalid_json() {
        let result = merge_calendar_definitions(vec!["invalid json"]);
        assert!(result.is_err());
    }
}
