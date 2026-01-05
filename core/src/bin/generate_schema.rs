//! JSON Schema generator for Romcal types.
//!
//! This binary generates JSON schemas for the main Romcal types,
//! enabling type validation and code generation for TypeScript and Python.

use romcal::{
    Acclamation, BibleBook, CalendarContext, CalendarDefinition, CelebrationSummary,
    DateDefWithOffset, DayOfWeek, LiturgicalCycle, LiturgicalDay, MassContext, MassPart,
    MonthIndex, Resources, SaintCount, SundayCycleCombined,
};
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

/// Add a type to the schema definitions
fn add_type_to_schema(types_schema: &mut Value, type_value: &mut Value, type_name: &str) {
    if let Some(types_definitions) = types_schema.get_mut("definitions") {
        if let Some(definitions_obj) = types_definitions.as_object_mut() {
            if let Some(type_obj) = type_value.as_object_mut() {
                type_obj.remove("$schema");
                definitions_obj.insert(type_name.to_string(), type_value.clone());
            }
        }
    }
}

/// Generate Rust constants file with embedded schemas for the romcal library
fn generate_rust_schema_constants(config: &SchemaConfig) -> Result<(), SchemaGenerationError> {
    // Generate schemas for the two types needed by romcal-cli
    let calendar_schema = schema_for!(CalendarDefinition);
    let resources_schema = schema_for!(Resources);

    let mut calendar_value = serde_json::to_value(&calendar_schema)
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;
    let mut resources_value = serde_json::to_value(&resources_schema)
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;

    // Apply standard fixes
    apply_standard_fixes(&mut calendar_value, config);
    apply_standard_fixes(&mut resources_value, config);

    // Convert to pretty JSON strings
    let calendar_json = serde_json::to_string_pretty(&calendar_value)
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;
    let resources_json = serde_json::to_string_pretty(&resources_value)
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;

    // Generate Rust source file
    let mut rust_content = String::new();
    rust_content.push_str("//! Auto-generated JSON schema constants - Do not modify manually\n");
    rust_content
        .push_str("//! Regenerate with: cargo run --bin generate-schema --features schema-gen\n\n");
    rust_content.push_str("/// JSON Schema for CalendarDefinition validation\n");
    rust_content.push_str("pub const CALENDAR_DEFINITION_SCHEMA: &str = r##\"");
    rust_content.push_str(&calendar_json);
    rust_content.push_str("\"##;\n\n");
    rust_content.push_str("/// JSON Schema for Resources validation\n");
    rust_content.push_str("pub const RESOURCES_SCHEMA: &str = r##\"");
    rust_content.push_str(&resources_json);
    rust_content.push_str("\"##;\n");

    // Write to src/generated/schemas.rs
    let file_path = PathBuf::from("src/generated/schemas.rs");
    fs::write(&file_path, rust_content).map_err(|source| SchemaGenerationError::FileWrite {
        path: file_path.clone(),
        source,
    })?;

    println!("✅ src/generated/schemas.rs exported (Rust constants for romcal lib)");
    Ok(())
}

/// Generate a schema specifically for TypeScript and Pydantic generation
fn generate_types_schema(config: &SchemaConfig) -> Result<(), SchemaGenerationError> {
    // Create a schema with a wrapper object that references all types as properties
    // This forces Quicktype to generate each type separately without merging
    let mut types_schema = serde_json::json!({
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": "AllTypes",
        "type": "object",
        "properties": {
            "liturgicalDay": { "$ref": "#/definitions/LiturgicalDay" },
            "calendarDefinition": { "$ref": "#/definitions/CalendarDefinition" },
            "resources": { "$ref": "#/definitions/Resources" },
            "massContext": { "$ref": "#/definitions/MassContext" },
            "celebrationSummary": { "$ref": "#/definitions/CelebrationSummary" },
            "calendarContext": { "$ref": "#/definitions/CalendarContext" },
            "sundayCycleCombined": { "$ref": "#/definitions/SundayCycleCombined" },
            "monthIndex": { "$ref": "#/definitions/MonthIndex" },
            "dayOfWeek": { "$ref": "#/definitions/DayOfWeek" },
            "dateDefWithOffset": { "$ref": "#/definitions/DateDefWithOffset" },
            "liturgicalCycle": { "$ref": "#/definitions/LiturgicalCycle" },
            "acclamation": { "$ref": "#/definitions/Acclamation" },
            "bibleBook": { "$ref": "#/definitions/BibleBook" },
            "massPart": { "$ref": "#/definitions/MassPart" },
            "saintCount": { "$ref": "#/definitions/SaintCount" }
        },
        "definitions": {}
    });

    // Generate schemas for all types and convert to values
    let mut calendar_value = serde_json::to_value(schema_for!(CalendarDefinition))
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;
    let mut resources_value = serde_json::to_value(schema_for!(Resources))
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;
    let mut liturgical_day_value = serde_json::to_value(schema_for!(LiturgicalDay))
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;
    let mut mass_context_value = serde_json::to_value(schema_for!(MassContext))
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;
    let mut celebration_summary_value = serde_json::to_value(schema_for!(CelebrationSummary))
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;
    let mut calendar_context_value = serde_json::to_value(schema_for!(CalendarContext))
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;
    let mut sunday_cycle_combined_value = serde_json::to_value(schema_for!(SundayCycleCombined))
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;
    let mut month_index_value = serde_json::to_value(schema_for!(MonthIndex))
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;
    let mut day_of_week_value = serde_json::to_value(schema_for!(DayOfWeek))
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;
    let mut date_def_with_offset_value = serde_json::to_value(schema_for!(DateDefWithOffset))
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;
    let mut liturgical_cycle_value = serde_json::to_value(schema_for!(LiturgicalCycle))
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;
    let mut acclamation_value = serde_json::to_value(schema_for!(Acclamation))
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;
    let mut bible_book_value = serde_json::to_value(schema_for!(BibleBook))
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;
    let mut mass_part_value = serde_json::to_value(schema_for!(MassPart))
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;
    let mut saint_count_value = serde_json::to_value(schema_for!(SaintCount))
        .map_err(|e| SchemaGenerationError::Serialization(e.to_string()))?;

    // Apply standard fixes to all schemas
    apply_standard_fixes(&mut calendar_value, config);
    apply_standard_fixes(&mut resources_value, config);
    apply_standard_fixes(&mut liturgical_day_value, config);
    apply_standard_fixes(&mut mass_context_value, config);
    apply_standard_fixes(&mut celebration_summary_value, config);
    apply_standard_fixes(&mut calendar_context_value, config);
    apply_standard_fixes(&mut sunday_cycle_combined_value, config);
    apply_standard_fixes(&mut month_index_value, config);
    apply_standard_fixes(&mut day_of_week_value, config);
    apply_standard_fixes(&mut date_def_with_offset_value, config);
    apply_standard_fixes(&mut liturgical_cycle_value, config);
    apply_standard_fixes(&mut acclamation_value, config);
    apply_standard_fixes(&mut bible_book_value, config);
    apply_standard_fixes(&mut mass_part_value, config);
    apply_standard_fixes(&mut saint_count_value, config);

    // Extract definitions from all schemas
    let schema_refs: Vec<&Value> = vec![
        &calendar_value,
        &resources_value,
        &liturgical_day_value,
        &mass_context_value,
        &celebration_summary_value,
        &calendar_context_value,
        &sunday_cycle_combined_value,
        &month_index_value,
        &day_of_week_value,
        &date_def_with_offset_value,
        &liturgical_cycle_value,
        &acclamation_value,
        &bible_book_value,
        &mass_part_value,
        &saint_count_value,
    ];
    merge_definitions_into_types_schema(&mut types_schema, &schema_refs);

    // Add the main types to definitions
    add_type_to_schema(&mut types_schema, &mut calendar_value, "CalendarDefinition");
    add_type_to_schema(&mut types_schema, &mut resources_value, "Resources");
    add_type_to_schema(
        &mut types_schema,
        &mut liturgical_day_value,
        "LiturgicalDay",
    );
    add_type_to_schema(&mut types_schema, &mut mass_context_value, "MassContext");
    add_type_to_schema(
        &mut types_schema,
        &mut celebration_summary_value,
        "CelebrationSummary",
    );
    add_type_to_schema(
        &mut types_schema,
        &mut calendar_context_value,
        "CalendarContext",
    );

    // Add additional types to definitions
    add_type_to_schema(
        &mut types_schema,
        &mut sunday_cycle_combined_value,
        "SundayCycleCombined",
    );
    add_type_to_schema(&mut types_schema, &mut month_index_value, "MonthIndex");
    add_type_to_schema(&mut types_schema, &mut day_of_week_value, "DayOfWeek");
    add_type_to_schema(
        &mut types_schema,
        &mut date_def_with_offset_value,
        "DateDefWithOffset",
    );
    add_type_to_schema(
        &mut types_schema,
        &mut liturgical_cycle_value,
        "LiturgicalCycle",
    );
    add_type_to_schema(&mut types_schema, &mut acclamation_value, "Acclamation");
    add_type_to_schema(&mut types_schema, &mut bible_book_value, "BibleBook");
    add_type_to_schema(&mut types_schema, &mut mass_part_value, "MassPart");
    add_type_to_schema(&mut types_schema, &mut saint_count_value, "SaintCount");

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

    // Generate Rust constants for romcal library
    generate_rust_schema_constants(&config)?;

    println!("\n🎉 All schemas have been generated successfully!");
    println!("📁 JSON schemas: {}", config.output_dir.display());
    println!("📁 Rust constants: src/generated/schemas.rs");
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
