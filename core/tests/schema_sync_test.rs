//! Tests to verify that embedded schema constants are synchronized with generated schemas.
//!
//! These tests require the `schema-gen` feature to be enabled:
//! ```bash
//! cargo test --features schema-gen
//! ```

#![cfg(feature = "schema-gen")]

use romcal::{CalendarDefinition, Resources};
use schemars::schema_for;

/// Helper function to apply the same fixes as generate_schema.rs
fn apply_fixes(schema: &mut serde_json::Value) {
    add_additional_properties_false(schema);
    fix_defs_references(schema);
}

fn add_additional_properties_false(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            if map.get("type") == Some(&serde_json::Value::String("object".to_string()))
                && !map.contains_key("additionalProperties")
            {
                map.insert(
                    "additionalProperties".to_string(),
                    serde_json::Value::Bool(false),
                );
            }
            map.values_mut().for_each(add_additional_properties_false);
        }
        serde_json::Value::Array(arr) => {
            arr.iter_mut().for_each(add_additional_properties_false);
        }
        _ => {}
    }
}

fn fix_defs_references(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Object(map) => {
            if let Some(defs) = map.remove("$defs") {
                map.insert("definitions".to_string(), defs);
            }
            map.values_mut().for_each(fix_defs_references);
        }
        serde_json::Value::Array(arr) => {
            arr.iter_mut().for_each(fix_defs_references);
        }
        serde_json::Value::String(s) => {
            if s.starts_with("#/$defs/") {
                *s = s.replace("#/$defs/", "#/definitions/");
            }
        }
        _ => {}
    }
}

#[test]
fn calendar_definition_schema_is_up_to_date() {
    // Generate schema from the Rust type
    let schema = schema_for!(CalendarDefinition);
    let mut schema_value = serde_json::to_value(&schema).expect("Failed to serialize schema");
    apply_fixes(&mut schema_value);
    let generated =
        serde_json::to_string_pretty(&schema_value).expect("Failed to format schema as JSON");

    // Compare with embedded constant
    let embedded = romcal::schemas::CALENDAR_DEFINITION_SCHEMA;

    assert_eq!(
        embedded.trim(),
        generated.trim(),
        "\n\nCalendarDefinition schema is out of sync!\n\
         Run: cargo run --bin generate-schema --features schema-gen\n"
    );
}

#[test]
fn resources_schema_is_up_to_date() {
    // Generate schema from the Rust type
    let schema = schema_for!(Resources);
    let mut schema_value = serde_json::to_value(&schema).expect("Failed to serialize schema");
    apply_fixes(&mut schema_value);
    let generated =
        serde_json::to_string_pretty(&schema_value).expect("Failed to format schema as JSON");

    // Compare with embedded constant
    let embedded = romcal::schemas::RESOURCES_SCHEMA;

    assert_eq!(
        embedded.trim(),
        generated.trim(),
        "\n\nResources schema is out of sync!\n\
         Run: cargo run --bin generate-schema --features schema-gen\n"
    );
}
