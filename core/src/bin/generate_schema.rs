use romcal_core::calendar_def::*;
use schemars::schema_for;
use serde_json::Value;
use std::fs;
use std::path::Path;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the schemas directory if it doesn't exist (at the project root)
    let schemas_dir = Path::new("../schemas");
    if !schemas_dir.exists() {
        fs::create_dir_all(schemas_dir)?;
    }

    // Generate the main schema for CalendarDefinition
    let calendar_schema = schema_for!(CalendarDefinition);
    let mut calendar_schema_value = serde_json::to_value(&calendar_schema)?;
    add_additional_properties_false(&mut calendar_schema_value);
    let calendar_schema_json = serde_json::to_string_pretty(&calendar_schema_value)?;
    fs::write(
        schemas_dir.join("calendar-definition.json"),
        calendar_schema_json,
    )?;
    println!("✅ CalendarDefinition schema exported to schemas/calendar-definition.json");

    // Generate the schema for DayDefinition
    let day_schema = schema_for!(DayDefinition);
    let mut day_schema_value = serde_json::to_value(&day_schema)?;
    add_additional_properties_false(&mut day_schema_value);
    let day_schema_json = serde_json::to_string_pretty(&day_schema_value)?;
    fs::write(schemas_dir.join("day-definition.json"), day_schema_json)?;
    println!("✅ DayDefinition schema exported to schemas/day-definition.json");

    // Generate the schema for Precedence
    let precedence_schema = schema_for!(Precedence);
    let mut precedence_schema_value = serde_json::to_value(&precedence_schema)?;
    add_additional_properties_false(&mut precedence_schema_value);
    let precedence_schema_json = serde_json::to_string_pretty(&precedence_schema_value)?;
    fs::write(schemas_dir.join("precedence.json"), precedence_schema_json)?;
    println!("✅ Precedence schema exported to schemas/precedence.json");

    // Generate the schema for all main types
    let all_types_schema = schema_for!(CalendarDefinition);
    let mut all_types_schema_value = serde_json::to_value(&all_types_schema)?;
    add_additional_properties_false(&mut all_types_schema_value);
    let all_types_json = serde_json::to_string_pretty(&all_types_schema_value)?;
    fs::write(schemas_dir.join("all-types.json"), all_types_json)?;
    println!("✅ Complete schema exported to schemas/all-types.json");

    // Fix the date_exceptions schema to support both single objects and arrays
    fix_date_exceptions_schema(&mut calendar_schema_value);
    let fixed_calendar_schema_json = serde_json::to_string_pretty(&calendar_schema_value)?;
    fs::write(
        schemas_dir.join("calendar-definition.json"),
        fixed_calendar_schema_json,
    )?;
    println!("✅ Fixed calendar-definition.json schema");

    println!("\n🎉 All JSON schemas have been generated successfully!");
    println!("📁 Destination directory: {}", schemas_dir.display());

    Ok(())
}
