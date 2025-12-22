use crate::enums::{LiturgicalDayFilter, OutputFormat};
use crate::error::RomcalCliError;
use crate::utils::current_year;
use colored::Colorize;
use csv::Writer;
use romcal_core::Romcal;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_saphyr;
use std::collections::BTreeMap;

/// Filtered liturgical day with only selected properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilteredLiturgicalDay {
    #[serde(flatten)]
    pub fields: serde_json::Map<String, serde_json::Value>,
}

/// Handle days command
pub fn handle(
    year: Option<i32>,
    filters: Option<Vec<LiturgicalDayFilter>>,
    romcal: Romcal,
    output_format: OutputFormat,
) -> Result<(), RomcalCliError> {
    let year = year.unwrap_or_else(current_year);
    let calendar = romcal.generate_calendar(year)?;

    let output_data: BTreeMap<String, Vec<FilteredLiturgicalDay>> = if let Some(filters) = filters {
        calendar
            .into_iter()
            .map(|(date, days)| {
                let filtered_days: Vec<FilteredLiturgicalDay> = days
                    .into_iter()
                    .map(|day| {
                        // Convert to JSON to preserve field order and enable dynamic filtering
                        let day_json = serde_json::to_value(&day).map_err(|e| {
                            RomcalCliError::config_error(format!("Failed to serialize day: {}", e))
                        })?;

                        let mut fields = serde_json::Map::new();
                        if let serde_json::Value::Object(obj) = day_json {
                            // Extract only the requested fields
                            for filter in &filters {
                                let field_name = filter.field_name();
                                if let Some(value) = obj.get(field_name) {
                                    fields.insert(field_name.to_string(), value.clone());
                                }
                            }
                        }

                        Ok(FilteredLiturgicalDay { fields })
                    })
                    .collect::<Result<Vec<FilteredLiturgicalDay>, RomcalCliError>>()?;
                Ok((date, filtered_days))
            })
            .collect::<Result<BTreeMap<String, Vec<FilteredLiturgicalDay>>, RomcalCliError>>()?
    } else {
        // No filters, return all liturgical days
        calendar
            .into_iter()
            .map(|(date, days)| {
                let filtered_days: Vec<FilteredLiturgicalDay> = days
                    .into_iter()
                    .map(|day| {
                        // Convert to JSON first to preserve field order
                        let day_json = serde_json::to_value(&day).map_err(|e| {
                            RomcalCliError::config_error(format!("Failed to serialize day: {}", e))
                        })?;

                        let mut fields = serde_json::Map::new();
                        if let serde_json::Value::Object(obj) = day_json {
                            for (key, value) in obj {
                                fields.insert(key, value);
                            }
                        }

                        Ok(FilteredLiturgicalDay { fields })
                    })
                    .collect::<Result<Vec<FilteredLiturgicalDay>, RomcalCliError>>()?;
                Ok((date, filtered_days))
            })
            .collect::<Result<BTreeMap<String, Vec<FilteredLiturgicalDay>>, RomcalCliError>>()?
    };

    match output_format {
        OutputFormat::Yaml => {
            let yaml_output = serde_saphyr::to_string(&output_data).map_err(|e| {
                RomcalCliError::config_error(format!("Failed to serialize to YAML: {}", e))
            })?;
            let formatted = format_yaml_output(&yaml_output);
            print!("{}", formatted);
        }
        OutputFormat::Json => {
            let json_output = serde_json::to_string_pretty(&output_data).map_err(|e| {
                RomcalCliError::config_error(format!("Failed to serialize to JSON: {}", e))
            })?;
            println!("{}", json_output);
        }
        OutputFormat::Csv => {
            let csv_output = convert_to_csv(&output_data)?;
            println!("{}", csv_output);
        }
        OutputFormat::Lines => {
            let lines_output = convert_to_lines(&output_data)?;
            println!("{}", lines_output);
        }
    }

    Ok(())
}

/// Convert filtered liturgical days to CSV format
fn convert_to_csv(
    data: &BTreeMap<String, Vec<FilteredLiturgicalDay>>,
) -> Result<String, RomcalCliError> {
    if data.is_empty() {
        return Ok(String::new());
    }

    let mut wtr = Writer::from_writer(Vec::new());

    // Get all unique field names from all records, preserving order of appearance
    let mut seen = std::collections::HashSet::new();
    let field_names: Vec<String> = data
        .values()
        .flatten()
        .flat_map(|day| day.fields.keys())
        .filter(|k| seen.insert((*k).clone()))
        .cloned()
        .collect();

    // Add "date" as the first column only if it's not already in the field names
    let header = if field_names.contains(&"date".to_string()) {
        field_names.clone()
    } else {
        let mut h = vec!["date".to_string()];
        h.extend(field_names.clone());
        h
    };

    // Write header
    wtr.write_record(&header)
        .map_err(|e| RomcalCliError::config_error(format!("Failed to write CSV header: {}", e)))?;

    // Write data rows
    for (date, days) in data {
        for day in days {
            let mut record = if field_names.contains(&"date".to_string()) {
                Vec::new() // date field is already in the filtered fields
            } else {
                vec![date.clone()] // add calendar date as first column
            };
            for field_name in &field_names {
                let value = day
                    .fields
                    .get(field_name)
                    .map(|v| match v {
                        serde_json::Value::String(s) => s.clone(),
                        serde_json::Value::Number(n) => n.to_string(),
                        serde_json::Value::Bool(b) => b.to_string(),
                        serde_json::Value::Null => String::new(),
                        serde_json::Value::Array(arr) => {
                            // Convert array to comma-separated string
                            arr.iter()
                                .map(|v| match v {
                                    serde_json::Value::String(s) => s.clone(),
                                    serde_json::Value::Number(n) => n.to_string(),
                                    serde_json::Value::Bool(b) => b.to_string(),
                                    _ => v.to_string(),
                                })
                                .collect::<Vec<_>>()
                                .join(";")
                        }
                        serde_json::Value::Object(_) => v.to_string(),
                    })
                    .unwrap_or_default();
                record.push(value);
            }
            wtr.write_record(&record).map_err(|e| {
                RomcalCliError::config_error(format!("Failed to write CSV record: {}", e))
            })?;
        }
    }

    wtr.flush()
        .map_err(|e| RomcalCliError::config_error(format!("Failed to flush CSV writer: {}", e)))?;

    let csv_data = wtr
        .into_inner()
        .map_err(|e| RomcalCliError::config_error(format!("Failed to get CSV data: {}", e)))?;

    String::from_utf8(csv_data).map_err(|e| {
        RomcalCliError::config_error(format!("Failed to convert CSV to string: {}", e))
    })
}

/// Format YAML output for better readability
///
/// This function:
/// 1. Removes existing empty lines
/// 2. Adds a newline before date keys (root level, no leading space)
/// 3. Adds a newline before array entries (lines starting with "  - "), except the first entry
/// 4. Ensures a single trailing newline
fn format_yaml_output(yaml: &str) -> String {
    let mut result = Vec::new();
    let mut is_first_line = true;
    let mut just_saw_date_key = false;

    for line in yaml.lines() {
        // Skip empty lines
        if line.trim().is_empty() {
            continue;
        }

        // Check if this is a date key (no leading space, ends with :)
        let is_date_key = !line.starts_with(' ') && line.ends_with(':');

        // Check if this is an array entry (but not the first one after a date key)
        let is_array_entry = line.starts_with("  - ") && !just_saw_date_key;

        // Add newline before date keys or array entries (except first line)
        if !is_first_line && (is_date_key || is_array_entry) {
            result.push(String::new());
        }

        result.push(line.to_string());
        is_first_line = false;
        just_saw_date_key = is_date_key;
    }

    // Join with newlines and add single trailing newline
    result.join("\n") + "\n"
}

/// Convert filtered liturgical days to lines format
fn convert_to_lines(
    data: &BTreeMap<String, Vec<FilteredLiturgicalDay>>,
) -> Result<String, RomcalCliError> {
    if data.is_empty() {
        return Ok(String::new());
    }

    // First pass: collect all field names and their maximum widths
    let mut field_widths = std::collections::HashMap::new();

    for days in data.values() {
        for day in days {
            for (key, value) in &day.fields {
                let field_value = match value {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    serde_json::Value::Null => "null".to_string(),
                    serde_json::Value::Array(arr) => {
                        // Handle arrays - for seasons and colors, extract the key
                        if key == "seasons" || key == "colors" {
                            if let Some(first_item) = arr.first() {
                                if let Some(key_value) =
                                    first_item.get("key").and_then(|v| v.as_str())
                                {
                                    key_value.to_string()
                                } else {
                                    format!("{:?}", first_item)
                                }
                            } else {
                                "[]".to_string()
                            }
                        } else {
                            // For other arrays, join with semicolons
                            arr.iter()
                                .map(|v| match v {
                                    serde_json::Value::String(s) => s.clone(),
                                    serde_json::Value::Number(n) => n.to_string(),
                                    serde_json::Value::Bool(b) => b.to_string(),
                                    _ => v.to_string(),
                                })
                                .collect::<Vec<_>>()
                                .join(";")
                        }
                    }
                    serde_json::Value::Object(_) => value.to_string(),
                };

                let current_width = field_widths.get(key).copied().unwrap_or(0);
                // Calculate width without ANSI codes: key + "=" + value
                let new_width = (key.len() + 1 + field_value.len()).max(current_width);
                field_widths.insert(key.clone(), new_width);
            }
        }
    }

    let mut lines = Vec::new();

    for (date, days) in data {
        // Add date header for this group
        lines.push(date.bold().to_string());

        for day in days {
            // Get all available fields for this day in the order they appear
            let mut fields = Vec::new();

            // Iterate through fields in the order they appear in the HashMap
            for (key, value) in &day.fields {
                let field_value = match value {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    serde_json::Value::Null => "null".to_string(),
                    serde_json::Value::Array(arr) => {
                        // Handle arrays - for seasons and colors, extract the key
                        if key == "seasons" || key == "colors" {
                            if let Some(first_item) = arr.first() {
                                if let Some(key_value) =
                                    first_item.get("key").and_then(|v| v.as_str())
                                {
                                    key_value.to_string()
                                } else {
                                    format!("{:?}", first_item)
                                }
                            } else {
                                "[]".to_string()
                            }
                        } else {
                            // For other arrays, join with semicolons
                            arr.iter()
                                .map(|v| match v {
                                    serde_json::Value::String(s) => s.clone(),
                                    serde_json::Value::Number(n) => n.to_string(),
                                    serde_json::Value::Bool(b) => b.to_string(),
                                    _ => v.to_string(),
                                })
                                .collect::<Vec<_>>()
                                .join(";")
                        }
                    }
                    serde_json::Value::Object(_) => value.to_string(),
                };

                let dim_key_equals = format!("{}=", key.dimmed());
                let field_entry = format!("{}{}", dim_key_equals, field_value);
                let max_width = field_widths
                    .get(key)
                    .copied()
                    .unwrap_or(key.len() + 1 + field_value.len());
                // Calculate the actual display width without ANSI codes for padding
                let actual_width = key.len() + 1 + field_value.len();
                let padding_needed = max_width.saturating_sub(actual_width);
                let padded_field = format!("{}{}", field_entry, " ".repeat(padding_needed));
                fields.push(padded_field);
            }

            // Join all fields with one space
            let line = format!("  {}", fields.join("  ")); // Indent days under date
            lines.push(line);
        }

        // Add blank line between dates
        lines.push(String::new());
    }

    Ok(lines.join("\n"))
}
