//! JSON Schema generator for Romcal types.
//!
//! This binary generates JSON schemas for the main Romcal types,
//! enabling type validation and code generation for TypeScript and Python.

use romcal_core::{CalendarDefinition, LiturgicalDay, Resources};
use schemars::schema_for;
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

/// Configuration for JSON schema generation
#[derive(Debug, Clone)]
pub struct SchemaConfig {
    /// Output directory for schemas
    pub output_dir: PathBuf,
    /// Enable additionalProperties: false on all objects
    pub enable_additional_properties_false: bool,
    /// Convert $defs to definitions for json2ts compatibility
    pub enable_defs_fix: bool,
}

impl Default for SchemaConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("../schemas"),
            enable_additional_properties_false: true,
            enable_defs_fix: true,
        }
    }
}

/// Specific errors for schema generation
#[derive(Debug)]
pub enum SchemaGenerationError {
    Serialization(String),
    FileWrite {
        path: PathBuf,
        source: std::io::Error,
    },
    DirectoryCreation {
        path: PathBuf,
        source: std::io::Error,
    },
}

impl std::fmt::Display for SchemaGenerationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SchemaGenerationError::Serialization(msg) => {
                write!(f, "Failed to serialize schema: {}", msg)
            }
            SchemaGenerationError::FileWrite { path, source } => {
                write!(f, "Failed to write file {:?}: {}", path, source)
            }
            SchemaGenerationError::DirectoryCreation { path, source } => {
                write!(f, "Failed to create directory {:?}: {}", path, source)
            }
        }
    }
}

impl std::error::Error for SchemaGenerationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SchemaGenerationError::FileWrite { source, .. } => Some(source),
            SchemaGenerationError::DirectoryCreation { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Add `additionalProperties: false` to all objects in the JSON schema
fn add_additional_properties_false(schema: &mut Value) {
    fn process_value(value: &mut Value) {
        match value {
            Value::Object(map) => {
                // Add additionalProperties: false to objects
                if map.get("type") == Some(&Value::String("object".to_string()))
                    && !map.contains_key("additionalProperties")
                {
                    map.insert("additionalProperties".to_string(), Value::Bool(false));
                }

                // Process all children recursively
                map.values_mut().for_each(process_value);
            }
            Value::Array(arr) => {
                arr.iter_mut().for_each(process_value);
            }
            _ => {} // Primitive types
        }
    }

    process_value(schema);
}

/// Fix $defs references to use definitions instead (compatibility with json2ts)
fn fix_defs_references(schema: &mut Value) {
    fn process_value(value: &mut Value) {
        match value {
            Value::Object(map) => {
                // Convert $defs to definitions
                if let Some(defs) = map.remove("$defs") {
                    map.insert("definitions".to_string(), defs);
                }

                // Process all children recursively
                map.values_mut().for_each(process_value);
            }
            Value::Array(arr) => {
                arr.iter_mut().for_each(process_value);
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

    process_value(schema);
}

/// Generate a schema for a given type and save it to a file
fn generate_schema<T>(config: &SchemaConfig, filename: &str) -> Result<(), SchemaGenerationError>
where
    T: schemars::JsonSchema,
{
    let schema = schema_for!(T);
    let mut schema_value = serde_json::to_value(&schema)
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;

    // Apply standard fixes
    if config.enable_additional_properties_false {
        add_additional_properties_false(&mut schema_value);
    }
    if config.enable_defs_fix {
        fix_defs_references(&mut schema_value);
    }

    // Write the schema to file
    let schema_json = serde_json::to_string_pretty(&schema_value)
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;
    let file_path = config.output_dir.join(filename);
    fs::write(&file_path, schema_json).map_err(|source| SchemaGenerationError::FileWrite {
        path: file_path,
        source,
    })?;

    println!("✅ {} schema exported to {}", filename, filename);
    Ok(())
}

/// Apply standard fixes to a schema value
fn apply_standard_fixes(schema_value: &mut Value, config: &SchemaConfig) {
    if config.enable_additional_properties_false {
        add_additional_properties_false(schema_value);
    }
    if config.enable_defs_fix {
        fix_defs_references(schema_value);
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

/// Add main types to the schema definitions
fn add_main_types_to_schema(
    types_schema: &mut Value,
    calendar_value: &mut Value,
    resources_value: &mut Value,
    liturgical_day_value: &mut Value,
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

            // Add LiturgicalDay (it's the root type, not in definitions)
            if let Some(liturgical_day_obj) = liturgical_day_value.as_object_mut() {
                liturgical_day_obj.remove("$schema");
                definitions_obj.insert("LiturgicalDay".to_string(), liturgical_day_value.clone());
            }
        }
    }
}

/// Generate a schema specifically for TypeScript and Pydantic generation
fn generate_types_schema(config: &SchemaConfig) -> Result<(), SchemaGenerationError> {
    // Create a schema that combines all main types as a union
    let mut types_schema = serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "RomcalTypes",
        "oneOf": [
            { "$ref": "#/definitions/CalendarDefinition" },
            { "$ref": "#/definitions/Resources" },
            { "$ref": "#/definitions/LiturgicalDay" }
        ],
        "definitions": {}
    });

    // Generate schemas for all major types and convert to values
    let mut calendar_value = serde_json::to_value(schema_for!(CalendarDefinition))
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;
    let mut resources_value = serde_json::to_value(schema_for!(Resources))
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;
    let mut liturgical_day_value = serde_json::to_value(schema_for!(LiturgicalDay))
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;

    // Apply standard fixes to all schemas
    apply_standard_fixes(&mut calendar_value, config);
    apply_standard_fixes(&mut resources_value, config);
    apply_standard_fixes(&mut liturgical_day_value, config);

    // Extract definitions from all schemas
    let schema_refs: Vec<&Value> = vec![&calendar_value, &resources_value, &liturgical_day_value];
    merge_definitions_into_types_schema(&mut types_schema, &schema_refs);

    // Add the main types as well
    add_main_types_to_schema(
        &mut types_schema,
        &mut calendar_value,
        &mut resources_value,
        &mut liturgical_day_value,
    );

    // Write the types schema
    let schema_json = serde_json::to_string_pretty(&types_schema)
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;
    let file_path = config.output_dir.join("all_types.json");
    fs::write(&file_path, schema_json).map_err(|source| SchemaGenerationError::FileWrite {
        path: file_path,
        source,
    })?;

    println!("✅ all_types.json schema exported (for TypeScript and Pydantic generation)");
    Ok(())
}

fn main() -> Result<(), SchemaGenerationError> {
    let config = SchemaConfig::default();

    // Create output directory if it doesn't exist
    if !config.output_dir.exists() {
        fs::create_dir_all(&config.output_dir).map_err(|source| {
            SchemaGenerationError::DirectoryCreation {
                path: config.output_dir.clone(),
                source,
            }
        })?;
    }

    println!("🚀 Starting schema generation...");

    // Generate resources schema
    generate_schema::<Resources>(&config, "resources.json")?;

    // Generate calendar_definition.json
    generate_schema::<CalendarDefinition>(&config, "calendar_definition.json")?;

    // Generate types schema for TypeScript and Pydantic generation
    generate_types_schema(&config)?;

    println!("\n🎉 All JSON schemas have been generated successfully!");
    println!("📁 Destination directory: {}", config.output_dir.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_generate_schema_creates_file() {
        // Arrange: Create a temporary directory
        let temp_dir = TempDir::new().unwrap();
        let config = SchemaConfig {
            output_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };

        // Act: Generate a schema
        let result = generate_schema::<Resources>(&config, "test_schema.json");

        // Assert: File should be created successfully
        assert!(result.is_ok());
        assert!(config.output_dir.join("test_schema.json").exists());

        // Check file content
        let content = fs::read_to_string(config.output_dir.join("test_schema.json")).unwrap();
        let schema: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert!(schema["$schema"].is_string());
        // Resources should have properties
        assert!(schema["properties"].is_object());
    }

    #[test]
    fn test_generate_schema_invalid_filename() {
        // Test with invalid filename (empty string)
        let temp_dir = TempDir::new().unwrap();
        let config = SchemaConfig {
            output_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };

        // Test with a valid filename
        let result = generate_schema::<Resources>(&config, "test_file.json");
        assert!(result.is_ok());
    }

    #[test]
    fn test_schema_generation_consistency() {
        // Test that generating the same schema twice produces identical results
        let temp_dir = TempDir::new().unwrap();
        let config = SchemaConfig {
            output_dir: temp_dir.path().to_path_buf(),
            ..Default::default()
        };

        // Generate schema twice
        generate_schema::<Resources>(&config, "schema1.json").unwrap();
        generate_schema::<Resources>(&config, "schema2.json").unwrap();

        // Read both files
        let content1 = fs::read_to_string(config.output_dir.join("schema1.json")).unwrap();
        let content2 = fs::read_to_string(config.output_dir.join("schema2.json")).unwrap();

        // They should be identical
        assert_eq!(content1, content2);
    }
}
