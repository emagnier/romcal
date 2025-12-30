//! Helper functions for merging resource and calendar definition files.
//!
//! These helpers allow users to load data files however they want (fetch, import, fs, etc.)
//! and then use romcal to merge them into the expected structures.

use serde_json::Value;

use crate::engine::calendar_definition::CalendarDefinition;
use crate::engine::resources::Resources;
use crate::error::RomcalError;

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
    let mut result = Resources::new(locale.to_string());

    for file_json in files_json {
        let file: Resources = serde_json::from_str(file_json).map_err(|e| {
            RomcalError::ValidationError(format!("Failed to parse resource file: {}", e))
        })?;

        // Merge entities first (before moving metadata)
        result.merge_entities(&file);

        // Extract metadata if present
        if file.metadata.is_some() {
            result.metadata = file.metadata;
        }
    }

    Ok(result)
}

/// Parse and validate multiple calendar definition files.
///
/// This function validates that each JSON string is a valid calendar definition,
/// then returns them as JSON Values to preserve the original structure.
/// This avoids issues with asymmetric serialization (e.g., MassTime uses snake_case
/// for input but SCREAMING_SNAKE_CASE for output).
///
/// # Arguments
///
/// * `files_json` - A list of JSON strings, each representing a calendar definition
///
/// # Returns
///
/// A vector of validated JSON Values representing CalendarDefinition objects.
///
/// # Example
///
/// ```ignore
/// let france = r#"{"id": "france", ...}"#;
/// let usa = r#"{"id": "usa", ...}"#;
/// let definitions = merge_calendar_definitions(vec![france, usa])?;
/// ```
pub fn merge_calendar_definitions(files_json: Vec<&str>) -> Result<Vec<Value>, RomcalError> {
    let mut definitions: Vec<Value> = Vec::with_capacity(files_json.len());

    for file_json in files_json {
        // Validate by parsing into CalendarDefinition (discarded)
        let _: CalendarDefinition = serde_json::from_str(file_json).map_err(|e| {
            RomcalError::ValidationError(format!("Failed to parse calendar definition: {}", e))
        })?;

        // Keep the original JSON structure as Value
        let value: Value = serde_json::from_str(file_json).map_err(|e| {
            RomcalError::ValidationError(format!("Failed to parse calendar definition: {}", e))
        })?;
        definitions.push(value);
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
