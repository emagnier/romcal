//! JSON round-trip test utility.
//!
//! This binary validates that JSON files can be deserialized into Rust types
//! and reserialized without data loss, ensuring type compatibility.

use romcal::{CalendarDefinition, Resources};
use serde_json::{self, Value};
use std::fs;
use std::path::Path;
use std::time::Instant;

/// Compare two JSON values ignoring key order and keys with null values.
/// Used in round-trip tests to verify that serialization/deserialization
/// preserves data integrity between JSON source files and Rust types.
fn json_values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Null, Value::Null) => true,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::Number(a), Value::Number(b)) => a == b,
        (Value::String(a), Value::String(b)) => a == b,
        (Value::Array(a), Value::Array(b)) => {
            if a.len() != b.len() {
                return false;
            }
            a.iter().zip(b.iter()).all(|(a, b)| json_values_equal(a, b))
        }
        (Value::Object(a), Value::Object(b)) => {
            // Create key sets ignoring null values
            let mut a_keys: std::collections::HashSet<&String> = a.keys().collect();
            let mut b_keys: std::collections::HashSet<&String> = b.keys().collect();

            // Remove keys with null values
            a_keys.retain(|k| !a.get(*k).is_some_and(|v| v.is_null()));
            b_keys.retain(|k| !b.get(*k).is_some_and(|v| v.is_null()));

            if a_keys != b_keys {
                return false;
            }

            // Compare values of non-null keys
            for key in &a_keys {
                if let (Some(value_a), Some(value_b)) = (a.get(*key), b.get(*key))
                    && !json_values_equal(value_a, value_b)
                {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}

/// JSON round-trip test for files
/// Reads a JSON file, deserializes it, reserializes it and compares with the original
fn test_json_roundtrip<T>(file_path: &str) -> Result<(), Box<dyn std::error::Error>>
where
    T: serde::de::DeserializeOwned + serde::Serialize + std::fmt::Debug,
{
    println!("Round-trip test for: {}", file_path);

    // Read the original JSON file
    let original_content = fs::read_to_string(file_path)?;

    // Parse the original JSON into Value for robust comparison
    let original_json: Value = serde_json::from_str(&original_content)?;

    // Deserialize JSON into Rust type
    let parsed: T = serde_json::from_str(&original_content)?;

    // Reserialize to JSON
    let reserialized = serde_json::to_string_pretty(&parsed)?;
    let reserialized_json: Value = serde_json::from_str(&reserialized)?;

    // Compare JSON values ignoring order and nulls
    if json_values_equal(&original_json, &reserialized_json) {
        println!(
            "✅ Success: File {} is identical after round-trip",
            file_path
        );
        Ok(())
    } else {
        println!("❌ Failure: File {} changed after round-trip", file_path);

        // Save the reserialized file for inspection
        let debug_path = format!("{}.reserialized", file_path);
        if let Err(e) = fs::write(&debug_path, &reserialized) {
            eprintln!("Unable to save debug file: {}", e);
        } else {
            println!("Reserialized file saved: {}", debug_path);
        }

        Err("Round-trip failed".into())
    }
}

/// Test all calendar files
fn test_calendar_files() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Calendar files test ===");

    let calendar_dirs = vec![
        "data/definitions/general_roman",
        "data/definitions/countries",
        "data/definitions/regions",
        "data/definitions/communities",
    ];

    let mut total_tests = 0;
    let mut passed_tests = 0;

    for dir in calendar_dirs {
        if Path::new(dir).exists() {
            println!("\nTesting directory: {}", dir);

            let entries = fs::read_dir(dir)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    total_tests += 1;
                    match test_json_roundtrip::<CalendarDefinition>(path.to_str().unwrap()) {
                        Ok(_) => passed_tests += 1,
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
        }
    }

    println!("\n=== Calendar tests summary ===");
    println!("Passed tests: {}/{}", passed_tests, total_tests);

    if passed_tests == total_tests {
        Ok(())
    } else {
        Err(format!("{} tests failed", total_tests - passed_tests).into())
    }
}

/// Test all resource files
fn test_resource_files() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Resource files test ===");

    let resource_dirs = vec![
        "data/resources/fr",
        "data/resources/en",
        "data/resources/es",
        "data/resources/de",
        "data/resources/it",
        "data/resources/la",
        "data/resources/pl",
        "data/resources/pt-br",
        "data/resources/sk",
        "data/resources/ta",
        "data/resources/cs",
        "data/resources/en-gb",
        "data/resources/en-ie",
    ];

    let mut total_tests = 0;
    let mut passed_tests = 0;

    for dir in resource_dirs {
        if Path::new(dir).exists() {
            println!("\nTesting directory: {}", dir);

            let entries = fs::read_dir(dir)?;
            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    total_tests += 1;
                    match test_json_roundtrip::<Resources>(path.to_str().unwrap()) {
                        Ok(_) => passed_tests += 1,
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
            }
        }
    }

    println!("\n=== Resource tests summary ===");
    println!("Passed tests: {}/{}", passed_tests, total_tests);

    if passed_tests == total_tests {
        Ok(())
    } else {
        Err(format!("{} tests failed", total_tests - passed_tests).into())
    }
}

/// Test a specific file with detailed analysis
fn test_specific_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Detailed test of file: {} ===", file_path);

    if !Path::new(file_path).exists() {
        return Err(format!("File {} does not exist", file_path).into());
    }

    // Detect file type based on path
    if file_path.contains("calendars") {
        test_json_roundtrip::<CalendarDefinition>(file_path)?;
    } else if file_path.contains("resources") {
        test_json_roundtrip::<Resources>(file_path)?;
    } else {
        return Err(
            "Unable to determine file type. Use a path containing 'calendars' or 'resources'"
                .into(),
        );
    }

    Ok(())
}

/// Display help
fn print_help() {
    println!("Usage: json_roundtrip_test [COMMAND]");
    println!();
    println!("Commands:");
    println!("  all                       Test all files");
    println!("  <file_path>               Test a specific file");
    println!("  --help                    Show this help");
    println!();
    println!("Examples:");
    println!("  cargo run --example json_roundtrip_test all");
    println!(
        "  cargo run --example json_roundtrip_test data/definitions/general_roman/general_roman.json"
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== JSON Round-trip Test for Romcal ===\n");

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print_help();
        return Ok(());
    }

    let command = &args[1];
    let start_time = Instant::now();

    match command.as_str() {
        "all" => {
            // Test all files
            let mut total_passed = 0;
            let mut total_tests = 0;

            // Test calendars
            match test_calendar_files() {
                Ok(_) => total_passed += 1,
                Err(e) => eprintln!("Error in calendar tests: {}", e),
            }
            total_tests += 1;

            // Test resources
            match test_resource_files() {
                Ok(_) => total_passed += 1,
                Err(e) => eprintln!("Error in resource tests: {}", e),
            }
            total_tests += 1;

            println!("\n=== Global summary ===");
            println!(
                "Categories tested successfully: {}/{}",
                total_passed, total_tests
            );

            if total_passed == total_tests {
                println!("🎉 All tests passed successfully!");
            } else {
                println!("⚠️  Some tests failed. Check the error messages above.");
            }
        }
        "--help" => {
            print_help();
        }
        _ => {
            // Test a specific file
            test_specific_file(command)?;
        }
    }

    let duration = start_time.elapsed();
    println!("\nExecution time: {:.2}s", duration.as_secs_f64());

    Ok(())
}
