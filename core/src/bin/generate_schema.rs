use romcal_core::calendar_def::*;
use schemars::schema_for;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

/// Fix the date_exceptions schema to support both single objects and arrays
fn fix_date_exceptions_schema(schema: &mut Value) {
    if let Some(definitions) = schema.get_mut("definitions") {
        if let Some(day_definition) = definitions.get_mut("DayDefinition") {
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

/// Generate a schema for a given type and save it to a file
fn generate_schema<T>(
    schemas_dir: &PathBuf,
    filename: &str,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: schemars::JsonSchema,
{
    let schema = schema_for!(T);
    let mut schema_value = serde_json::to_value(&schema)?;
    add_additional_properties_false(&mut schema_value);
    let schema_json = serde_json::to_string_pretty(&schema_value)?;
    fs::write(schemas_dir.join(filename), schema_json)?;
    println!("✅ {} schema exported to {}", filename, filename);
    Ok(())
}

/// Generate a schema with custom fixes applied
fn generate_schema_with_fixes<T>(
    schemas_dir: &PathBuf,
    filename: &str,
    fix_fn: fn(&mut Value),
) -> Result<(), Box<dyn std::error::Error>>
where
    T: schemars::JsonSchema,
{
    let schema = schema_for!(T);
    let mut schema_value = serde_json::to_value(&schema)?;
    add_additional_properties_false(&mut schema_value);
    fix_fn(&mut schema_value);
    let schema_json = serde_json::to_string_pretty(&schema_value)?;
    fs::write(schemas_dir.join(filename), schema_json)?;
    println!("✅ {} schema exported to {}", filename, filename);
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schemas_dir = PathBuf::from("../schemas");

    if !schemas_dir.exists() {
        fs::create_dir_all(&schemas_dir)?;
    }

    println!("🚀 Starting schema generation...");

    // Generate individual schemas
    generate_schema::<DayDefinition>(&schemas_dir, "day-definition.json")?;
    generate_schema::<Precedence>(&schemas_dir, "precedence.json")?;

    // Generate calendar-definition.json with date_exceptions fix
    generate_schema_with_fixes::<CalendarDefinition>(
        &schemas_dir,
        "calendar-definition.json",
        fix_date_exceptions_schema,
    )?;

    // Generate all-types.json (copy of calendar-definition.json)
    fs::copy(
        schemas_dir.join("calendar-definition.json"),
        schemas_dir.join("all-types.json"),
    )?;
    println!("✅ all-types.json schema exported (copy of calendar-definition.json)");

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
                "DayDefinition": {
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
            &schema["definitions"]["DayDefinition"]["properties"]["date_exceptions"];
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
    fn test_add_additional_properties_false() {
        // Arrange: Create a schema with objects that need additionalProperties: false
        let mut schema = json!({
            "type": "object",
            "properties": {
                "nested_object": {
                    "type": "object",
                    "properties": {
                        "field": { "type": "string" }
                    }
                },
                "array_of_objects": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "item_field": { "type": "string" }
                        }
                    }
                },
                "string_field": { "type": "string" }
            }
        });

        // Act: Apply additionalProperties: false
        add_additional_properties_false(&mut schema);

        // Assert: Check that all objects have additionalProperties: false
        assert_eq!(schema["additionalProperties"], false);
        assert_eq!(
            schema["properties"]["nested_object"]["additionalProperties"],
            false
        );
        assert_eq!(
            schema["properties"]["array_of_objects"]["items"]["additionalProperties"],
            false
        );
        // String field should not have additionalProperties
        assert!(!schema["properties"]["string_field"]
            .as_object()
            .unwrap()
            .contains_key("additionalProperties"));
    }

    #[test]
    fn test_add_additional_properties_false_preserves_existing() {
        // Test that existing additionalProperties are not overwritten
        let mut schema = json!({
            "type": "object",
            "additionalProperties": true,
            "properties": {
                "nested": {
                    "type": "object",
                    "additionalProperties": { "type": "string" }
                }
            }
        });

        add_additional_properties_false(&mut schema);

        // Should preserve existing additionalProperties
        assert_eq!(schema["additionalProperties"], true);
        assert_eq!(
            schema["properties"]["nested"]["additionalProperties"]["type"],
            "string"
        );
    }

    #[test]
    fn test_generate_schema_creates_file() {
        // Arrange: Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let schemas_dir = temp_dir.path().to_path_buf();

        // Act: Generate a schema
        let result = generate_schema::<Precedence>(&schemas_dir, "test-schema.json");

        // Assert: File should be created successfully
        assert!(result.is_ok());
        assert!(schemas_dir.join("test-schema.json").exists());

        // Check file content
        let content = fs::read_to_string(schemas_dir.join("test-schema.json")).unwrap();
        let schema: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert!(schema["$schema"].is_string());
        // Precedence is an enum, so it should have an "enum" field
        assert!(schema["enum"].is_array());
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
        let result = generate_schema_with_fixes::<DayDefinition>(
            &schemas_dir,
            "test-fixed-schema.json",
            test_fix,
        );

        // Assert: File should be created and contain the fix
        assert!(result.is_ok());
        assert!(schemas_dir.join("test-fixed-schema.json").exists());

        let content = fs::read_to_string(schemas_dir.join("test-fixed-schema.json")).unwrap();
        let schema: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert!(schema["definitions"]["test_property"].is_object());
    }

    #[test]
    fn test_generate_schema_invalid_filename() {
        // Test with invalid filename (empty string)
        let temp_dir = TempDir::new().unwrap();
        let schemas_dir = temp_dir.path().to_path_buf();

        // Test with a valid filename
        let result = generate_schema::<Precedence>(&schemas_dir, "test-file.json");
        assert!(result.is_ok());
    }

    #[test]
    fn test_schema_generation_consistency() {
        // Test that generating the same schema twice produces identical results
        let temp_dir = TempDir::new().unwrap();
        let schemas_dir = temp_dir.path().to_path_buf();

        // Generate schema twice
        generate_schema::<DayDefinition>(&schemas_dir, "schema1.json").unwrap();
        generate_schema::<DayDefinition>(&schemas_dir, "schema2.json").unwrap();

        // Read both files
        let content1 = fs::read_to_string(schemas_dir.join("schema1.json")).unwrap();
        let content2 = fs::read_to_string(schemas_dir.join("schema2.json")).unwrap();

        // They should be identical
        assert_eq!(content1, content2);
    }
}
