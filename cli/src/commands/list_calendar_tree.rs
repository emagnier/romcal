//! List calendars tree command.
//!
//! This module provides functionality to display the hierarchical structure
//! of all calendars in the romcal project as a tree.

use crate::enums::OutputFormat;
use crate::error::RomcalCliError;
use romcal_core::generated_constants::CALENDAR_TREE_JSON;
use serde_json::Value;

/// Display the calendar tree structure.
///
/// This function parses the calendar tree JSON and displays it in a hierarchical format.
/// The tree shows the relationships between different calendars, with regions containing
/// countries, and countries containing their sub-regions.
pub fn list_calendar_tree(format: OutputFormat) -> Result<(), RomcalCliError> {
    match format {
        OutputFormat::Json => {
            // Output pretty-printed JSON
            let tree: Value = serde_json::from_str(CALENDAR_TREE_JSON)?;
            let pretty_json = serde_json::to_string_pretty(&tree)?;
            println!("{}", pretty_json);
        }
        OutputFormat::Yaml => {
            // Parse JSON and convert to YAML
            let tree: Value = serde_json::from_str(CALENDAR_TREE_JSON)?;
            let yaml = serde_yaml::to_string(&tree)?;
            print!("{}", yaml);
        }
        OutputFormat::Csv => {
            // Display as CSV Parent-Child format
            display_as_csv();
        }
        OutputFormat::Lines => {
            // Display as formatted tree
            display_as_lines();
        }
    }

    Ok(())
}

/// Display the calendar tree as lines.
fn display_as_lines() {
    if let Ok(tree) = serde_json::from_str::<Value>(CALENDAR_TREE_JSON) {
        print_line_node(&tree, 0);
    } else {
        eprintln!("Failed to parse calendar tree JSON");
    }
}

/// Recursively print a tree node as lines.
fn print_line_node(node: &Value, depth: usize) {
    let indent = "  ".repeat(depth);

    if let Some(id) = node.get("id").and_then(|v| v.as_str()) {
        println!("{}{}", indent, id);

        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                print_line_node(child, depth + 1);
            }
        }
    }
}

/// Display the calendar tree as CSV in Parent-Child format.
fn display_as_csv() {
    println!("id,parent_id,level");

    if let Ok(tree) = serde_json::from_str::<Value>(CALENDAR_TREE_JSON) {
        print_csv_node(&tree, None, 0);
    } else {
        eprintln!("Failed to parse calendar tree JSON");
    }
}

/// Recursively print a tree node in CSV format.
fn print_csv_node(node: &Value, parent_id: Option<&str>, level: usize) {
    if let Some(id) = node.get("id").and_then(|v| v.as_str()) {
        let parent_str = parent_id.unwrap_or("");
        println!("{},{},{}", id, parent_str, level);

        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                print_csv_node(child, Some(id), level + 1);
            }
        }
    }
}
