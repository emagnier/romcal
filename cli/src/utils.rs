use crate::error::RomcalCliError;
use chrono::Datelike;
use glob::glob;
use std::fs;
use std::path::Path;

// =================================================================================
// Private utility functions
// =================================================================================

/// Check if a file has a .json extension (case insensitive)
fn is_json_file(path: &Path) -> bool {
    path.extension()
        .and_then(|s| s.to_str())
        .map(|ext| ext.to_lowercase())
        .as_deref()
        == Some("json")
}

/// Helper function to try adding a valid JSON file to the collection
fn try_add_valid_json_file(files: &mut Vec<std::path::PathBuf>, path: std::path::PathBuf) {
    if let Err(e) = read_json_file(&path.to_string_lossy()) {
        eprintln!("⚠️  Skipping invalid JSON file {}: {}", path.display(), e);
        return;
    }
    files.push(path);
}

// =================================================================================
// Main public functions
// =================================================================================

/// Read and parse a JSON file, returning the parsed value
/// Validates that the file is a valid JSON file
pub fn read_json_file(file_path: &str) -> Result<serde_json::Value, RomcalCliError> {
    let path = Path::new(file_path);

    // Check if file exists
    if !path.exists() {
        return Err(RomcalCliError::config_error(format!(
            "File does not exist: {}",
            file_path
        )));
    }

    // Check if it's a file (not a directory)
    if !path.is_file() {
        return Err(RomcalCliError::config_error(format!(
            "Path is not a file: {}",
            file_path
        )));
    }

    // Check if it has a .json extension (case insensitive)
    if !is_json_file(path) {
        return Err(RomcalCliError::config_error(format!(
            "File is not a JSON file: {}",
            file_path
        )));
    }

    // Read and parse the JSON file
    let content = std::fs::read_to_string(file_path).map_err(|e| {
        RomcalCliError::config_error(format!("Failed to read file '{}': {}", file_path, e))
    })?;

    serde_json::from_str(&content).map_err(|e| {
        RomcalCliError::config_error(format!("Invalid JSON in file '{}': {}", file_path, e))
    })
}

/// Collect JSON files based on patterns (supports glob patterns and directories)
/// Returns a list of PathBuf for valid JSON files
pub fn collect_json_files(patterns: &[String]) -> Result<Vec<std::path::PathBuf>, RomcalCliError> {
    let mut files = Vec::new();

    for pattern in patterns {
        // Check if it's a glob pattern (contains * or **)
        if pattern.contains('*') {
            // It's a glob pattern
            for entry in glob(pattern)
                .map_err(|e| RomcalCliError::config_error(format!("Invalid glob pattern: {}", e)))?
            {
                match entry {
                    Ok(path) => {
                        if path.is_file() && is_json_file(&path) {
                            try_add_valid_json_file(&mut files, path);
                        }
                    }
                    Err(e) => eprintln!("⚠️  Error reading glob entry: {}", e),
                }
            }
        } else {
            // It's a single file path
            let path = Path::new(pattern);
            if path.is_file() && is_json_file(path) {
                try_add_valid_json_file(&mut files, path.to_path_buf());
            } else if path.is_dir() {
                // If it's a directory, find all JSON files in it
                for entry in fs::read_dir(path).map_err(|e| {
                    RomcalCliError::config_error(format!("Cannot read directory: {}", e))
                })? {
                    let entry = entry.map_err(|e| {
                        RomcalCliError::config_error(format!("Cannot read directory entry: {}", e))
                    })?;
                    let path = entry.path();
                    if path.is_file() && is_json_file(&path) {
                        try_add_valid_json_file(&mut files, path);
                    }
                }
            } else {
                return Err(RomcalCliError::config_error(format!(
                    "File does not exist: {}",
                    pattern
                )));
            }
        }
    }

    if files.is_empty() {
        return Err(RomcalCliError::config_error(format!(
            "No valid JSON files found matching: {}",
            patterns.join(", ")
        )));
    }

    Ok(files)
}

/// Collect JSON file paths from patterns (supports glob patterns and directories)
pub fn collect_json_file_paths(patterns: &[String]) -> Result<Vec<String>, RomcalCliError> {
    let files = collect_json_files(patterns)?;

    // Convert PathBuf to String
    let file_strings: Vec<String> = files
        .into_iter()
        .map(|path| path.to_string_lossy().to_string())
        .collect();

    Ok(file_strings)
}

// =================================================================================
// Data loading functions
// =================================================================================

/// Parse calendar definition files and return a Vec of CalendarDefinition
pub fn parse_calendar_definition_files(
    file_paths: &[String],
) -> Result<Vec<romcal_core::CalendarDefinition>, RomcalCliError> {
    let mut calendar_definitions = Vec::new();

    for file_path in file_paths {
        let json_value = read_json_file(file_path)?;

        // Deserialize JSON value to CalendarDefinition
        let calendar_def: romcal_core::CalendarDefinition = serde_json::from_value(json_value)?;

        calendar_definitions.push(calendar_def);
    }

    Ok(calendar_definitions)
}

/// Parse resource files and return a Vec of ResourcesDefinition
pub fn parse_resource_files(
    file_paths: &[String],
) -> Result<Vec<romcal_core::ResourcesDefinition>, RomcalCliError> {
    let mut resource_definitions = Vec::new();

    for file_path in file_paths {
        let json_value = read_json_file(file_path)?;

        // Deserialize JSON value to ResourcesDefinition
        let resource_def: romcal_core::ResourcesDefinition = serde_json::from_value(json_value)?;

        resource_definitions.push(resource_def);
    }

    Ok(resource_definitions)
}

// =================================================================================
// Public utility functions
// =================================================================================

/// Get current year
pub fn current_year() -> i32 {
    chrono::Utc::now().year()
}

/// Validate a year using romcal_core's validation function
pub fn validate_year(year: i32) -> Result<(), RomcalCliError> {
    romcal_core::validate_year(year, 1583).map_err(|e| match e {
        romcal_core::RomcalError::InvalidYear(year) => RomcalCliError::invalid_year(year),
        _ => RomcalCliError::config_error(format!("Year validation error: {}", e)),
    })
}
