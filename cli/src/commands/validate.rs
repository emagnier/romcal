use crate::error::RomcalCliError;
use crate::utils;
use jsonschema::{ValidationError, validator_for};
use romcal::schemas::{CALENDAR_DEFINITION_SCHEMA, RESOURCES_SCHEMA};
use serde_json::Value;
use std::fs;

/// Type of validation to perform
#[derive(Clone, Debug)]
pub enum ValidationType {
    /// Validate calendar definition JSON file
    Definitions,
    /// Validate resource JSON file
    Resources,
}

impl From<crate::ValidationType> for ValidationType {
    fn from(vt: crate::ValidationType) -> Self {
        match vt {
            crate::ValidationType::Definitions => ValidationType::Definitions,
            crate::ValidationType::Resources => ValidationType::Resources,
        }
    }
}

/// Handle validate command
pub fn handle(
    validation_type: crate::ValidationType,
    file_paths: &[String],
) -> Result<(), RomcalCliError> {
    let validation_type = ValidationType::from(validation_type);

    let schema_content = match validation_type {
        ValidationType::Definitions => CALENDAR_DEFINITION_SCHEMA,
        ValidationType::Resources => RESOURCES_SCHEMA,
    };

    println!("🔍 Validating JSON files...");
    println!("📁 Input: {}", file_paths.join(" "));
    println!(
        "📋 Schema: {} (embedded)",
        match validation_type {
            ValidationType::Definitions => "calendar_definition.json",
            ValidationType::Resources => "resources_definition.json",
        }
    );
    println!();

    let all_files = utils::collect_json_files(file_paths)?;

    // Deduplicate while preserving order
    let mut seen = std::collections::HashSet::new();
    let files: Vec<_> = all_files
        .into_iter()
        .filter(|f| seen.insert(f.clone()))
        .collect();

    let schema_json: Value = serde_json::from_str(schema_content)?;
    let compiled_schema = validator_for(&schema_json)
        .map_err(|e| RomcalCliError::SchemaValidationError(Box::new(e)))?;

    let mut total_files = 0;
    let mut valid_files = 0;
    let mut invalid_files = 0;
    let mut errors: Vec<(String, String)> = Vec::new();

    for file_path in files {
        total_files += 1;

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
                .map(|e| format!("{}: {}", e.instance_path(), e))
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
