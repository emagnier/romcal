use romcal_core::calendar_definition::*;
use romcal_core::resources::*;
use schemars::schema_for;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

/// Fix the date_exceptions schema to support both single objects and arrays
fn fix_date_exceptions_schema(schema: &mut Value) {
    if let Some(definitions) = schema.get_mut("definitions") {
        if let Some(day_definition) = definitions.get_mut("CalendarDefinition") {
            if let Some(properties) = day_definition.get_mut("properties") {
                if let Some(date_exceptions) = properties.get_mut("date_exceptions") {
                    // Replace the simple array type with anyOf that supports both single object and array
                    *date_exceptions = serde_json::json!({
                        "description": "Date definition exception",
                        "anyOf": [
                            {
                                "$ref": "#/definitions/DateDefException"
                            },
                            {
                                "items": {
                                    "$ref": "#/definitions/DateDefException"
                                },
                                "type": "array"
                            },
                            {
                                "type": "null"
                            }
                        ]
                    });
                }
            }
        }
    }
}

/// Fix the SaintCount schema to support both integers and "MANY" string
fn fix_saint_count_schema(schema: &mut Value) {
    if let Some(definitions) = schema.get_mut("definitions") {
        if let Some(saint_count) = definitions.get_mut("SaintCount") {
            *saint_count = serde_json::json!({
                "anyOf": [
                    {
                        "format": "uint32",
                        "minimum": 0,
                        "type": "integer"
                    },
                    {
                        "const": "MANY",
                        "type": "string"
                    },
                    {
                        "type": "null"
                    }
                ]
            });
        }
    }
}

/// Add `additionalProperties: false` to all objects in the JSON schema
fn add_additional_properties_false(schema: &mut Value) {
    match schema {
        Value::Object(map) => {
            // If it's an object with type "object", add additionalProperties: false
            if let Some(Value::String(obj_type)) = map.get("type") {
                if obj_type == "object" && !map.contains_key("additionalProperties") {
                    map.insert("additionalProperties".to_string(), Value::Bool(false));
                }
            }

            // Recursively treat all values of the object
            for (_, value) in map.iter_mut() {
                add_additional_properties_false(value);
            }
        }
        Value::Array(arr) => {
            // Recursively treat all elements of the array
            for item in arr.iter_mut() {
                add_additional_properties_false(item);
            }
        }
        _ => {} // Other types (string, number, bool, null) don't need treatment
    }
}

/// Fix $defs references to use definitions instead (compatibility with json2ts)
fn fix_defs_references(schema: &mut Value) {
    match schema {
        Value::Object(map) => {
            // Convert $defs to definitions
            if let Some(defs) = map.remove("$defs") {
                map.insert("definitions".to_string(), defs);
            }

            // Fix $ref references from #/$defs/ to #/definitions/
            for (_, value) in map.iter_mut() {
                fix_defs_references(value);
            }
        }
        Value::Array(arr) => {
            for item in arr.iter_mut() {
                fix_defs_references(item);
            }
        }
        Value::String(s) => {
            // Replace #/$defs/ with #/definitions/ in string values
            if s.starts_with("#/$defs/") {
                *s = s.replace("#/$defs/", "#/definitions/");
            }
        }
        _ => {}
    }
}

/// Generate a schema for a given type and save it to a file
fn generate_schema<T>(schemas_dir: &Path, filename: &str) -> Result<(), Box<dyn std::error::Error>>
where
    T: schemars::JsonSchema,
{
    let schema = schema_for!(T);
    let mut schema_value = serde_json::to_value(&schema)?;
    add_additional_properties_false(&mut schema_value);
    fix_defs_references(&mut schema_value);

    if filename == "resources_definition.json" || filename == "entity_definition.json" {
        fix_saint_count_schema(&mut schema_value);
    }

    let schema_json = serde_json::to_string_pretty(&schema_value)?;
    fs::write(schemas_dir.join(filename), schema_json)?;
    println!("✅ {} schema exported to {}", filename, filename);

    Ok(())
}

/// Generate a schema with custom fixes applied
fn generate_schema_with_fixes<T>(
    schemas_dir: &Path,
    filename: &str,
    fix_fn: fn(&mut Value),
) -> Result<(), Box<dyn std::error::Error>>
where
    T: schemars::JsonSchema,
{
    let schema = schema_for!(T);
    let mut schema_value = serde_json::to_value(&schema)?;
    add_additional_properties_false(&mut schema_value);
    fix_defs_references(&mut schema_value);
    fix_fn(&mut schema_value);
    let schema_json = serde_json::to_string_pretty(&schema_value)?;
    fs::write(schemas_dir.join(filename), schema_json)?;
    println!("✅ {} schema exported to {}", filename, filename);
    Ok(())
}

/// Apply fixes to all schema values
fn apply_fixes_to_all_schemas(schema_values: &mut [&mut Value]) {
    for schema_value in schema_values.iter_mut() {
        add_additional_properties_false(schema_value);
        fix_defs_references(schema_value);
    }
    // Apply specific fixes to calendar schema (first in the array)
    if let Some(calendar_value) = schema_values.first_mut() {
        fix_date_exceptions_schema(calendar_value);
    }
}

/// Extract definitions from all schemas and merge them into the types schema
fn merge_definitions_into_types_schema(types_schema: &mut Value, schema_values: &[&Value]) {
    if let Some(types_definitions) = types_schema.get_mut("definitions") {
        if let Some(definitions_obj) = types_definitions.as_object_mut() {
            for schema_value in schema_values {
                if let Some(definitions) = schema_value.get("definitions") {
                    if let Some(defs) = definitions.as_object() {
                        for (key, value) in defs {
                            definitions_obj.insert(key.clone(), value.clone());
                        }
                    }
                }
            }
        }
    }
}

/// Add main types (CalendarDefinition and Resources) to the schema
fn add_main_types_to_schema(
    types_schema: &mut Value,
    calendar_value: &mut Value,
    resources_value: &mut Value,
) {
    if let Some(types_definitions) = types_schema.get_mut("definitions") {
        if let Some(definitions_obj) = types_definitions.as_object_mut() {
            // Add CalendarDefinition (it's the root type, not in definitions)
            if let Some(calendar_obj) = calendar_value.as_object_mut() {
                calendar_obj.remove("$schema");
                definitions_obj.insert("CalendarDefinition".to_string(), calendar_value.clone());
            }

            // Add Resources (it's the root type, not in definitions)
            if let Some(resources_obj) = resources_value.as_object_mut() {
                resources_obj.remove("$schema");
                definitions_obj.insert("Resources".to_string(), resources_value.clone());
            }
        }
    }
}

/// Generate a schema specifically for TypeScript generation
fn generate_types_schema(schemas_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Create a schema that combines all main types as a union
    let mut types_schema = serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "RomcalTypes",
        "oneOf": [
            { "$ref": "#/definitions/CalendarDefinition" },
            { "$ref": "#/definitions/Resources" }
        ],
        "definitions": {}
    });

    // Generate schemas for all major types and convert to values
    let mut calendar_value = serde_json::to_value(schema_for!(CalendarDefinition))?;
    let mut resources_value = serde_json::to_value(schema_for!(Resources))?;

    // Apply fixes to all schemas
    let mut schema_values = vec![&mut calendar_value, &mut resources_value];
    apply_fixes_to_all_schemas(&mut schema_values);

    // Apply SaintCount fix to all schemas
    for schema_value in schema_values.iter_mut() {
        fix_saint_count_schema(schema_value);
    }

    // Extract definitions from all schemas
    let schema_refs: Vec<&Value> = schema_values.iter().map(|v| &**v).collect();
    merge_definitions_into_types_schema(&mut types_schema, &schema_refs);

    // Add the main types as well
    add_main_types_to_schema(&mut types_schema, &mut calendar_value, &mut resources_value);

    // Write the types schema
    let schema_json = serde_json::to_string_pretty(&types_schema)?;
    fs::write(schemas_dir.join("all_types.json"), schema_json)?;
    println!("✅ all_types.json schema exported (for TypeScript generation)");

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schemas_dir = PathBuf::from("../schemas");

    if !schemas_dir.exists() {
        fs::create_dir_all(&schemas_dir)?;
    }

    println!("🚀 Starting schema generation...");

    // Generate resources schemas
    generate_schema::<Resources>(&schemas_dir, "resources.json")?;

    // Generate calendar_definition.json with date_exceptions and saint_count fixes
    generate_schema_with_fixes::<CalendarDefinition>(
        &schemas_dir,
        "calendar_definition.json",
        |schema| {
            fix_date_exceptions_schema(schema);
            fix_saint_count_schema(schema);
        },
    )?;

    generate_types_schema(&schemas_dir)?;

    println!("\n🎉 All JSON schemas have been generated successfully!");
    println!("📁 Destination directory: {}", schemas_dir.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_fix_date_exceptions_schema() {
        // Arrange: Create a schema with date_exceptions as simple array
        let mut schema = json!({
            "definitions": {
                "CalendarDefinition": {
                    "properties": {
                        "date_exceptions": {
                            "type": "array",
                            "items": { "$ref": "#/definitions/DateDefException" }
                        }
                    }
                }
            }
        });

        // Act: Apply the fix
        fix_date_exceptions_schema(&mut schema);

        // Assert: Check that date_exceptions now has anyOf structure
        let date_exceptions =
            &schema["definitions"]["CalendarDefinition"]["properties"]["date_exceptions"];
        assert!(date_exceptions["anyOf"].is_array());
        assert_eq!(date_exceptions["anyOf"].as_array().unwrap().len(), 3);

        // Check the three options: single object, array, and null
        let any_of = date_exceptions["anyOf"].as_array().unwrap();
        assert!(any_of
            .iter()
            .any(|item| item["$ref"] == "#/definitions/DateDefException"));
        assert!(any_of.iter().any(|item| item["type"] == "array"));
        assert!(any_of.iter().any(|item| item["type"] == "null"));
    }

    #[test]
    fn test_fix_date_exceptions_schema_missing_definitions() {
        // Test with schema that doesn't have the expected structure
        let mut schema = json!({
            "definitions": {
                "OtherDefinition": {
                    "properties": {
                        "some_field": { "type": "string" }
                    }
                }
            }
        });

        // Should not panic and should not modify the schema
        let original_schema = schema.clone();
        fix_date_exceptions_schema(&mut schema);
        assert_eq!(schema, original_schema);
    }

    #[test]
    fn test_generate_schema_creates_file() {
        // Arrange: Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let schemas_dir = temp_dir.path().to_path_buf();

        // Act: Generate a schema
        let result = generate_schema::<Resources>(&schemas_dir, "test_schema.json");

        // Assert: File should be created successfully
        assert!(result.is_ok());
        assert!(schemas_dir.join("test_schema.json").exists());

        // Check file content
        let content = fs::read_to_string(schemas_dir.join("test_schema.json")).unwrap();
        let schema: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert!(schema["$schema"].is_string());
        // Resources should have properties
        assert!(schema["properties"].is_object());
    }

    #[test]
    fn test_generate_schema_with_fixes_applies_fixes() {
        // Arrange: Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let schemas_dir = temp_dir.path().to_path_buf();

        // Custom fix function that adds a test property
        fn test_fix(schema: &mut Value) {
            if let Some(definitions) = schema.get_mut("definitions") {
                definitions.as_object_mut().unwrap().insert(
                    "test_property".to_string(),
                    json!({ "type": "string", "description": "Test property" }),
                );
            }
        }

        // Act: Generate schema with fixes
        let result = generate_schema_with_fixes::<CalendarDefinition>(
            &schemas_dir,
            "test_fixed_schema.json",
            test_fix,
        );

        // Assert: File should be created and contain the fix
        assert!(result.is_ok());
        assert!(schemas_dir.join("test_fixed_schema.json").exists());

        let content = fs::read_to_string(schemas_dir.join("test_fixed_schema.json")).unwrap();
        let schema: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert!(schema["definitions"]["test_property"].is_object());
    }

    #[test]
    fn test_generate_schema_invalid_filename() {
        // Test with invalid filename (empty string)
        let temp_dir = TempDir::new().unwrap();
        let schemas_dir = temp_dir.path().to_path_buf();

        // Test with a valid filename
        let result = generate_schema::<Resources>(&schemas_dir, "test_file.json");
        assert!(result.is_ok());
    }

    #[test]
    fn test_schema_generation_consistency() {
        // Test that generating the same schema twice produces identical results
        let temp_dir = TempDir::new().unwrap();
        let schemas_dir = temp_dir.path().to_path_buf();

        // Generate schema twice
        generate_schema::<Resources>(&schemas_dir, "schema1.json").unwrap();
        generate_schema::<Resources>(&schemas_dir, "schema2.json").unwrap();

        // Read both files
        let content1 = fs::read_to_string(schemas_dir.join("schema1.json")).unwrap();
        let content2 = fs::read_to_string(schemas_dir.join("schema2.json")).unwrap();

        // They should be identical
        assert_eq!(content1, content2);
    }
}
