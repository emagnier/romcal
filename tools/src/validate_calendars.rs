use jsonschema::{validator_for, ValidationError};
use serde_json::Value;
use std::env;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use walkdir::WalkDir;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // Get the project root directory (assuming we're running from core/)
    let project_root = if args.len() > 1 {
        args[1].clone()
    } else {
        // Default to parent directory of core/
        Path::new("..")
            .canonicalize()?
            .to_string_lossy()
            .to_string()
    };

    let calendars_dir = Path::new(&project_root).join("data").join("calendars");
    let schema_path = Path::new(&project_root)
        .join("schemas")
        .join("calendar-definition.json");

    println!("🔍 Validating calendar JSON files...");
    println!("📁 Calendars directory: {}", calendars_dir.display());
    println!("📋 Schema file: {}", schema_path.display());
    println!();

    // Check if directories exist
    if !calendars_dir.exists() {
        eprintln!(
            "❌ Calendars directory does not exist: {}",
            calendars_dir.display()
        );
        std::process::exit(1);
    }

    if !schema_path.exists() {
        eprintln!("❌ Schema file does not exist: {}", schema_path.display());
        std::process::exit(1);
    }

    // Load the JSON schema
    let schema_content = fs::read_to_string(&schema_path)?;
    let schema_json: Value = serde_json::from_str(&schema_content)?;
    let compiled_schema = Arc::new(validator_for(&schema_json)?);

    let mut total_files = 0;
    let mut valid_files = 0;
    let mut invalid_files = 0;
    let mut errors: Vec<(String, String)> = Vec::new();

    // Walk through all JSON files in the calendars directory
    for entry in WalkDir::new(&calendars_dir)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file() && e.path().extension().is_some_and(|ext| ext == "json")
        })
    {
        let file_path = entry.path();
        total_files += 1;

        println!("Validating: {}", file_path.display());

        // Read and parse the JSON file
        let file_content = match fs::read_to_string(file_path) {
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
            println!("  ✅ Valid");
            valid_files += 1;
        } else {
            println!("  ❌ Invalid");
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
        std::process::exit(1);
    } else {
        println!();
        println!("✅ All calendar files are valid!");
    }

    Ok(())
}
