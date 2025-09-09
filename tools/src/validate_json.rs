use glob::glob;
use jsonschema::{validator_for, ValidationError};
use serde_json::Value;
use std::env;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use walkdir::WalkDir;

fn collect_json_files(
    path_or_pattern: &str,
    project_root: &str,
) -> Result<Vec<std::path::PathBuf>, Box<dyn std::error::Error>> {
    let mut json_files = Vec::new();

    // Check if it's a glob pattern (contains * or **)
    if path_or_pattern.contains('*') {
        // It's a glob pattern
        let pattern = if Path::new(path_or_pattern).is_absolute() {
            path_or_pattern.to_string()
        } else {
            Path::new(project_root)
                .join(path_or_pattern)
                .to_string_lossy()
                .to_string()
        };

        for entry in glob(&pattern)? {
            match entry {
                Ok(path) => {
                    if path.is_file() && path.extension().is_some_and(|ext| ext == "json") {
                        json_files.push(path);
                    }
                }
                Err(e) => eprintln!("⚠️  Error reading glob entry: {}", e),
            }
        }
    } else {
        // It's a directory path
        let dir_path = if Path::new(path_or_pattern).is_absolute() {
            Path::new(path_or_pattern).to_path_buf()
        } else {
            Path::new(project_root).join(path_or_pattern)
        };

        if !dir_path.exists() {
            eprintln!("❌ Directory does not exist: {}", dir_path.display());
            std::process::exit(1);
        }

        // Walk through all JSON files in the directory
        for entry in WalkDir::new(&dir_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.file_type().is_file() && e.path().extension().is_some_and(|ext| ext == "json")
            })
        {
            json_files.push(entry.path().to_path_buf());
        }
    }

    if json_files.is_empty() {
        eprintln!("⚠️  No JSON files found matching: {}", path_or_pattern);
    }

    Ok(json_files)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!(
            "Usage: {} <json_path_or_pattern> <schema_file> [project_root]",
            args[0]
        );
        eprintln!("  json_path_or_pattern: Directory containing JSON files OR glob pattern (e.g., data/resources/**/*.json)");
        eprintln!("  schema_file: Path to the JSON schema file");
        eprintln!("  project_root: Optional project root (defaults to current directory)");
        std::process::exit(1);
    }

    let json_path_or_pattern = &args[1];
    let schema_file = &args[2];
    let project_root = if args.len() > 3 {
        args[3].clone()
    } else {
        std::env::current_dir()?.to_string_lossy().to_string()
    };

    // Resolve schema path relative to project root
    let schema_path = if Path::new(schema_file).is_absolute() {
        Path::new(schema_file).to_path_buf()
    } else {
        Path::new(&project_root).join(schema_file)
    };

    println!("🔍 Validating JSON files...");
    println!("📁 JSON path/pattern: {}", json_path_or_pattern);
    println!("📋 Schema file: {}", schema_path.display());
    println!();

    // Check if schema file exists
    if !schema_path.exists() {
        eprintln!("❌ Schema file does not exist: {}", schema_path.display());
        std::process::exit(1);
    }

    // Load the JSON schema
    let schema_content = fs::read_to_string(&schema_path)?;
    let schema_json: Value = serde_json::from_str(&schema_content)?;
    let compiled_schema = Arc::new(validator_for(&schema_json)?);

    // Collect JSON files based on whether it's a directory or glob pattern
    let json_files = collect_json_files(json_path_or_pattern, &project_root)?;

    let mut total_files = 0;
    let mut valid_files = 0;
    let mut invalid_files = 0;
    let mut errors: Vec<(String, String)> = Vec::new();

    // Process all collected JSON files
    for file_path in json_files {
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
        std::process::exit(1);
    } else {
        println!();
        println!("✅ All JSON files are valid!");
    }

    Ok(())
}
