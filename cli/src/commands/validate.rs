use crate::error::RomcalCliError;
use crate::utils;
use jsonschema::{validator_for, ValidationError};
use serde_json::Value;
use std::fs;

// Include schemas at compile time
const CALENDAR_DEFINITION_SCHEMA: &str = include_str!("../../../schemas/calendar_definition.json");
const RESOURCES_DEFINITION_SCHEMA: &str =
    include_str!("../../../schemas/resources_definition.json");

/// Type of validation to perform
#[derive(Clone, Debug)]
pub enum ValidationType {
    /// Validate calendar definition JSON file
    CalendarDef,
    /// Validate resource JSON file
    Resource,
}

impl From<crate::ValidationType> for ValidationType {
    fn from(vt: crate::ValidationType) -> Self {
        match vt {
            crate::ValidationType::CalendarDef => ValidationType::CalendarDef,
            crate::ValidationType::Resource => ValidationType::Resource,
        }
    }
}

/// Handle validate command
pub fn handle(
    validation_type: crate::ValidationType,
    file_paths: &[String],
) -> Result<(), RomcalCliError> {
    let validation_type = ValidationType::from(validation_type);

    // Get schema content based on validation type
    let schema_content = match validation_type {
        ValidationType::CalendarDef => CALENDAR_DEFINITION_SCHEMA,
        ValidationType::Resource => RESOURCES_DEFINITION_SCHEMA,
    };

    println!("🔍 Validating JSON files...");
    println!("📁 Input: {}", file_paths.join(" "));
    println!(
        "📋 Schema: {} (embedded)",
        match validation_type {
            ValidationType::CalendarDef => "calendar_definition.json",
            ValidationType::Resource => "resources_definition.json",
        }
    );
    println!();

    // Collect files from all inputs
    let all_files = utils::collect_json_files(file_paths)?;

    // Remove duplicates while preserving order
    let mut unique_files = Vec::new();
    for file in all_files {
        if !unique_files.contains(&file) {
            unique_files.push(file);
        }
    }

    let files = unique_files;

    // Parse the embedded JSON schema
    let schema_json: Value = serde_json::from_str(schema_content)?;
    let compiled_schema = validator_for(&schema_json)
        .map_err(|e| RomcalCliError::SchemaValidationError(Box::new(e)))?;

    let mut total_files = 0;
    let mut valid_files = 0;
    let mut invalid_files = 0;
    let mut errors: Vec<(String, String)> = Vec::new();

    // Process all collected files
    for file_path in files {
        total_files += 1;

        // Read and parse the JSON file
        let file_content = match fs::read_to_string(&file_path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("❌ Error reading file {}: {}", file_path.display(), e);
                invalid_files += 1;
                continue;
            }
        };

        let json_data: Value = match serde_json::from_str(&file_content) {
            Ok(data) => data,
            Err(e) => {
                eprintln!("❌ Invalid JSON in {}: {}", file_path.display(), e);
                invalid_files += 1;
                continue;
            }
        };

        // Validate against the schema
        let validation_result = compiled_schema.validate(&json_data);

        if validation_result.is_ok() {
            println!("✅ Valid: {}", file_path.display());
            valid_files += 1;
        } else {
            println!("❌ Invalid: {}", file_path.display());
            invalid_files += 1;

            let validation_errors: Vec<ValidationError> =
                compiled_schema.iter_errors(&json_data).collect();

            let error_messages: Vec<String> = validation_errors
                .iter()
                .map(|e| format!("{}: {}", e.instance_path, e))
                .collect();

            errors.push((
                file_path.to_string_lossy().to_string(),
                error_messages.join("; "),
            ));
        }
    }

    println!();
    println!("📊 Validation Summary:");
    println!("  Total files: {}", total_files);
    println!("  Valid files: {}", valid_files);
    println!("  Invalid files: {}", invalid_files);

    if !errors.is_empty() {
        println!();
        println!("❌ Validation Errors:");
        for (file_path, error_message) in errors {
            println!("\n📄 File: {}", file_path);
            println!("  • {}", error_message);
        }
        return Err(RomcalCliError::config_error("JSON validation failed"));
    } else {
        println!();
        println!("🎉 All JSON files are valid!");
    }

    Ok(())
}
