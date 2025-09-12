use crate::error::RomcalCliError;
use glob::glob;
use jsonschema::{validator_for, ValidationError};
use serde_json::Value;
use std::fs;
use std::path::Path;

// Include schemas at compile time
const CALENDAR_DEFINITION_SCHEMA: &str = include_str!("../../../schemas/calendar_definition.json");
const RESOURCES_DEFINITION_SCHEMA: &str =
    include_str!("../../../schemas/resources_definition.json");

/// Collect files based on pattern (supports glob patterns)
fn collect_files(pattern: &str) -> Result<Vec<std::path::PathBuf>, RomcalCliError> {
    let mut files = Vec::new();

    // Check if it's a glob pattern (contains * or **)
    if pattern.contains('*') {
        // It's a glob pattern
        for entry in glob(pattern)
            .map_err(|e| RomcalCliError::config_error(format!("Invalid glob pattern: {}", e)))?
        {
            match entry {
                Ok(path) => {
                    if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
                        files.push(path);
                    }
                }
                Err(e) => eprintln!("⚠️  Error reading glob entry: {}", e),
            }
        }
    } else {
        // It's a single file path
        let path = Path::new(pattern);
        if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
            files.push(path.to_path_buf());
        } else if path.is_dir() {
            // If it's a directory, find all JSON files in it
            for entry in fs::read_dir(path).map_err(|e| {
                RomcalCliError::config_error(format!("Cannot read directory: {}", e))
            })? {
                let entry = entry.map_err(|e| {
                    RomcalCliError::config_error(format!("Cannot read directory entry: {}", e))
                })?;
                let path = entry.path();
                if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
                    files.push(path);
                }
            }
        } else {
            return Err(RomcalCliError::config_error(format!(
                "File does not exist: {}",
                pattern
            )));
        }
    }

    if files.is_empty() {
        return Err(RomcalCliError::config_error(format!(
            "No JSON files found matching: {}",
            pattern
        )));
    }

    Ok(files)
}

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
pub fn handle_validate(
    validation_type: crate::ValidationType,
    file_inputs: &[String],
) -> Result<(), RomcalCliError> {
    let validation_type = ValidationType::from(validation_type);

    // Get schema content based on validation type
    let schema_content = match validation_type {
        ValidationType::CalendarDef => CALENDAR_DEFINITION_SCHEMA,
        ValidationType::Resource => RESOURCES_DEFINITION_SCHEMA,
    };

    println!("🔍 Validating JSON files...");
    println!("📁 Input: {}", file_inputs.join(" "));
    println!(
        "📋 Schema: {} (embedded)",
        match validation_type {
            ValidationType::CalendarDef => "calendar_definition.json",
            ValidationType::Resource => "resources_definition.json",
        }
    );
    println!();

    // Collect files from all inputs
    let mut all_files = Vec::new();
    for input in file_inputs {
        let files = collect_files(input)?;
        all_files.extend(files);
    }

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
