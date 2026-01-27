//! Bundled data module - provides embedded calendar definitions and resources.
//!
//! This module is only available when the `bundled-data` feature is enabled.
//! It embeds all calendar definitions and locale resources as compile-time constants,
//! eliminating the need to load data from the filesystem at runtime.
//!
//! # Usage
//!
//! ```rust,ignore
//! use romcal::bundled_data;
//!
//! // Get all calendar definitions as parsed structs
//! let definitions = bundled_data::get_all_calendar_definitions()?;
//!
//! // Get all resources as parsed structs
//! let resources = bundled_data::get_all_resources()?;
//!
//! // Or access raw JSON strings directly
//! let france_json = bundled_data::definitions::FRANCE;
//! let fr_meta = bundled_data::resources::fr::META;
//! ```

// Include the generated bundled data from build.rs
include!(concat!(env!("OUT_DIR"), "/bundled_data.rs"));

use crate::engine::{CalendarDefinition, Resources};
use crate::error::RomcalError;
use crate::helpers::merge_resource_files;

/// Get all bundled calendar definitions as parsed structs.
///
/// Returns a vector of all calendar definitions embedded in the binary.
/// This includes all calendars: general_roman, regional, and national calendars.
pub fn get_all_calendar_definitions() -> Result<Vec<CalendarDefinition>, RomcalError> {
    get_all_definition_jsons()
        .into_iter()
        .map(|json_str| {
            serde_json::from_str(json_str).map_err(|e| {
                RomcalError::ValidationError(format!("Failed to parse calendar definition: {}", e))
            })
        })
        .collect()
}

/// Get all bundled resources as parsed structs.
///
/// Returns a vector of all locale resources embedded in the binary.
/// Each locale's multiple JSON files (meta.json, entities.*.json) are merged automatically.
pub fn get_all_resources() -> Result<Vec<Resources>, RomcalError> {
    get_all_resource_jsons()
        .into_iter()
        .map(|(locale, jsons)| merge_resource_files(locale, jsons))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==================== Calendar Definitions Tests ====================

    #[test]
    fn test_definition_jsons_not_empty() {
        let jsons = get_all_definition_jsons();
        assert!(
            !jsons.is_empty(),
            "Should have at least one calendar definition"
        );
    }

    #[test]
    fn test_definition_jsons_count() {
        let jsons = get_all_definition_jsons();
        // We expect at least 69 calendars (countries, regions, dioceses, general_roman, temporal_cycle)
        assert!(
            jsons.len() >= 69,
            "Expected at least 69 calendar definitions, got {}",
            jsons.len()
        );
    }

    #[test]
    fn test_all_definition_jsons_are_valid_json() {
        for (i, json_str) in get_all_definition_jsons().iter().enumerate() {
            let result: Result<serde_json::Value, _> = serde_json::from_str(json_str);
            assert!(
                result.is_ok(),
                "Calendar definition #{} is not valid JSON: {:?}",
                i,
                result.err()
            );
        }
    }

    #[test]
    fn test_all_definitions_parse_successfully() {
        let result = get_all_calendar_definitions();
        assert!(
            result.is_ok(),
            "Failed to parse calendar definitions: {:?}",
            result.err()
        );

        let definitions = result.unwrap();
        assert!(
            definitions.len() >= 69,
            "Expected at least 69 parsed definitions, got {}",
            definitions.len()
        );
    }

    #[test]
    fn test_all_definitions_have_id() {
        let definitions = get_all_calendar_definitions().unwrap();
        for def in &definitions {
            assert!(
                !def.id.is_empty(),
                "Calendar definition should have a non-empty id"
            );
        }
    }

    #[test]
    fn test_key_calendars_exist() {
        let definitions = get_all_calendar_definitions().unwrap();
        let ids: Vec<&str> = definitions.iter().map(|d| d.id.as_str()).collect();

        let required_calendars = [
            "general_roman",
            "temporal_cycle",
            "france",
            "united_states",
            "italy",
            "germany",
            "spain",
            "europe",
            "americas",
        ];

        for calendar_id in required_calendars {
            assert!(
                ids.contains(&calendar_id),
                "Required calendar '{}' not found in bundled definitions",
                calendar_id
            );
        }
    }

    #[test]
    fn test_general_roman_is_present() {
        let definitions = get_all_calendar_definitions().unwrap();
        let general_roman = definitions.iter().find(|d| d.id == "general_roman");
        assert!(
            general_roman.is_some(),
            "general_roman calendar must be present"
        );
    }

    // ==================== Resources Tests ====================

    #[test]
    fn test_resource_jsons_not_empty() {
        let jsons = get_all_resource_jsons();
        assert!(
            !jsons.is_empty(),
            "Should have at least one locale resource"
        );
    }

    #[test]
    fn test_resource_jsons_count() {
        let jsons = get_all_resource_jsons();
        // We expect at least 13 locales
        assert!(
            jsons.len() >= 13,
            "Expected at least 13 locale resources, got {}",
            jsons.len()
        );
    }

    #[test]
    fn test_all_resource_jsons_are_valid_json() {
        for (locale, json_files) in get_all_resource_jsons() {
            for (i, json_str) in json_files.iter().enumerate() {
                let result: Result<serde_json::Value, _> = serde_json::from_str(json_str);
                assert!(
                    result.is_ok(),
                    "Resource file #{} for locale '{}' is not valid JSON: {:?}",
                    i,
                    locale,
                    result.err()
                );
            }
        }
    }

    #[test]
    fn test_all_resources_parse_successfully() {
        let result = get_all_resources();
        assert!(
            result.is_ok(),
            "Failed to parse resources: {:?}",
            result.err()
        );

        let resources = result.unwrap();
        assert!(
            resources.len() >= 13,
            "Expected at least 13 parsed resources, got {}",
            resources.len()
        );
    }

    #[test]
    fn test_all_resources_have_locale() {
        let resources = get_all_resources().unwrap();
        for res in &resources {
            assert!(
                !res.locale.is_empty(),
                "Resource should have a non-empty locale"
            );
        }
    }

    #[test]
    fn test_key_locales_exist() {
        let resources = get_all_resources().unwrap();
        let locales: Vec<&str> = resources.iter().map(|r| r.locale.as_str()).collect();

        let required_locales = ["en", "fr", "es", "it", "de", "la", "pl"];

        for locale_id in required_locales {
            assert!(
                locales.contains(&locale_id),
                "Required locale '{}' not found in bundled resources",
                locale_id
            );
        }
    }

    #[test]
    fn test_english_locale_has_entities() {
        let resources = get_all_resources().unwrap();
        let english = resources.iter().find(|r| r.locale == "en");
        assert!(english.is_some(), "English locale must be present");

        let en = english.unwrap();
        assert!(
            en.entities.as_ref().is_some_and(|e| !e.is_empty()),
            "English locale should have entities"
        );
    }

    // ==================== Raw Module Access Tests ====================

    #[test]
    fn test_definitions_module_general_roman() {
        // Test direct access to the generated constant
        let json_str = definitions::GENERAL_ROMAN;
        assert!(
            !json_str.is_empty(),
            "GENERAL_ROMAN constant should not be empty"
        );

        let parsed: Result<CalendarDefinition, _> = serde_json::from_str(json_str);
        assert!(
            parsed.is_ok(),
            "GENERAL_ROMAN should parse as CalendarDefinition"
        );
        assert_eq!(parsed.unwrap().id, "general_roman");
    }

    #[test]
    fn test_definitions_module_france() {
        let json_str = definitions::FRANCE;
        assert!(!json_str.is_empty(), "FRANCE constant should not be empty");

        let parsed: Result<CalendarDefinition, _> = serde_json::from_str(json_str);
        assert!(parsed.is_ok(), "FRANCE should parse as CalendarDefinition");
        assert_eq!(parsed.unwrap().id, "france");
    }

    #[test]
    fn test_resources_module_en_meta() {
        // Test direct access to the generated constant
        let json_str = resources::en::META;
        assert!(
            !json_str.is_empty(),
            "en::META constant should not be empty"
        );

        let parsed: Result<serde_json::Value, _> = serde_json::from_str(json_str);
        assert!(parsed.is_ok(), "en::META should be valid JSON");
    }

    #[test]
    fn test_resources_module_fr_meta() {
        let json_str = resources::fr::META;
        assert!(
            !json_str.is_empty(),
            "fr::META constant should not be empty"
        );

        let parsed: Result<serde_json::Value, _> = serde_json::from_str(json_str);
        assert!(parsed.is_ok(), "fr::META should be valid JSON");
    }

    // ==================== Consistency Tests ====================

    #[test]
    fn test_definition_ids_are_unique() {
        let definitions = get_all_calendar_definitions().unwrap();
        let mut ids: Vec<&str> = definitions.iter().map(|d| d.id.as_str()).collect();
        let original_count = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(
            ids.len(),
            original_count,
            "Calendar definition IDs should be unique"
        );
    }

    #[test]
    fn test_resource_locales_are_unique() {
        let resources = get_all_resources().unwrap();
        let mut locales: Vec<&str> = resources.iter().map(|r| r.locale.as_str()).collect();
        let original_count = locales.len();
        locales.sort();
        locales.dedup();
        assert_eq!(
            locales.len(),
            original_count,
            "Resource locales should be unique"
        );
    }
}
