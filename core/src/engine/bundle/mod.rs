//! Bundle Romcal configuration for distribution.
//!
//! This module provides functionality to create optimized JSON bundles from
//! a Romcal configuration. The bundling process:
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
//! For a locale hierarchy `fr-ca → fr → en`, after bundling:
//! - `fr-ca`: Contains the final values for translated properties
//! - `fr`: Contains only properties not defined in `fr-ca`
//! - `en`: Contains only properties not defined in `fr-ca` or `fr`

mod deduplicate;
mod filter;
#[cfg(test)]
mod tests;

use std::collections::{HashMap, HashSet};

use serde_json::Value;

use crate::{Romcal, RomcalError, RomcalResult};

use filter::{filter_calendar_definitions, filter_resources};

// ============================================================================
// Type Aliases
// ============================================================================

/// Maps lowercase locale to original case (for case-insensitive lookup).
pub(super) type LocaleMap = HashMap<String, String>;

/// Set of IDs (martyrology entries, calendars, etc.).
pub(super) type IdSet = HashSet<String>;

/// Set of property names for tracking defined properties during deduplication.
pub(super) type PropertySet = HashSet<String>;

/// Maps martyrology entry ID to its set of defined properties across locales.
pub(super) type MartyrologyPropertiesMap = HashMap<String, PropertySet>;

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
/// * `romcal` - The Romcal configuration to bundle
///
/// # Returns
///
/// A pretty-printed JSON string of the bundled configuration.
///
/// # Errors
///
/// Returns an error if:
/// - Duplicate calendar IDs or locales are found
/// - Required calendars or locales are missing
/// - JSON serialization fails
pub fn bundle(romcal: &Romcal) -> RomcalResult<String> {
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
fn validate_unique_calendar_ids(
    calendar_definitions: &[crate::CalendarDefinition],
) -> RomcalResult<()> {
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
fn validate_unique_resource_locales(resources: &[crate::Resources]) -> RomcalResult<()> {
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
