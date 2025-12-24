use crate::{enums::OutputFormat, error::RomcalCliError};
use romcal::generated::calendar_ids::{CALENDAR_IDS, CALENDAR_TREE_JSON};
use romcal::generated::locale_ids::{LOCALE_CODES, LOCALE_TREE_JSON};
use serde_json::{self, Value};
use serde_saphyr;

/// Generic function to list items in various formats
fn list_items(items: &[&str], output_format: OutputFormat) -> Result<(), RomcalCliError> {
    match output_format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(items)?);
        }
        OutputFormat::Lines => {
            for item in items {
                println!("{}", item);
            }
        }
        OutputFormat::Csv => {
            println!("{}", items.join(","));
        }
        OutputFormat::Yaml => {
            let yaml = serde_saphyr::to_string(&items.to_vec()).map_err(|e| {
                RomcalCliError::config_error(format!("Failed to serialize to YAML: {}", e))
            })?;
            println!("{}", yaml);
        }
    }
    Ok(())
}

/// Handle list calendars command
pub fn handle_calendars(output_format: OutputFormat, tree: bool) -> Result<(), RomcalCliError> {
    if tree {
        display_calendar_tree(output_format)
    } else {
        list_items(CALENDAR_IDS, output_format)
    }
}

/// Handle list locales command
pub fn handle_locales(output_format: OutputFormat, tree: bool) -> Result<(), RomcalCliError> {
    if tree {
        display_locale_tree(output_format)
    } else {
        list_items(LOCALE_CODES, output_format)
    }
}

/// Print tree data as JSON or YAML
fn print_tree_structured(
    json_str: &str,
    format: OutputFormat,
    name: &str,
) -> Result<(), RomcalCliError> {
    let tree: Value = serde_json::from_str(json_str)?;
    match format {
        OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&tree)?),
        OutputFormat::Yaml => {
            let yaml = serde_saphyr::to_string(&tree).map_err(|e| {
                RomcalCliError::config_error(format!("Failed to serialize {} to YAML: {}", name, e))
            })?;
            print!("{}", yaml);
        }
        _ => unreachable!(),
    }
    Ok(())
}

/// Display the calendar tree structure
fn display_calendar_tree(format: OutputFormat) -> Result<(), RomcalCliError> {
    match format {
        OutputFormat::Json | OutputFormat::Yaml => {
            print_tree_structured(CALENDAR_TREE_JSON, format, "calendar tree")
        }
        OutputFormat::Csv => {
            display_calendar_as_csv();
            Ok(())
        }
        OutputFormat::Lines => {
            display_calendar_as_lines();
            Ok(())
        }
    }
}

/// Display the locale tree structure
fn display_locale_tree(format: OutputFormat) -> Result<(), RomcalCliError> {
    match format {
        OutputFormat::Json | OutputFormat::Yaml => {
            print_tree_structured(LOCALE_TREE_JSON, format, "locale tree")
        }
        OutputFormat::Csv => {
            display_locale_as_csv();
            Ok(())
        }
        OutputFormat::Lines => {
            display_locale_as_lines();
            Ok(())
        }
    }
}

/// Display the calendar tree as lines.
fn display_calendar_as_lines() {
    if let Ok(tree) = serde_json::from_str::<Value>(CALENDAR_TREE_JSON) {
        print_calendar_line_node(&tree, 0);
    } else {
        eprintln!("Failed to parse calendar tree JSON");
    }
}

/// Recursively print a calendar tree node as lines.
fn print_calendar_line_node(node: &Value, depth: usize) {
    let indent = "  ".repeat(depth);

    if let Some(id) = node.get("id").and_then(|v| v.as_str()) {
        println!("{}{}", indent, id);

        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                print_calendar_line_node(child, depth + 1);
            }
        }
    }
}

/// Display the calendar tree as CSV in Parent-Child format.
fn display_calendar_as_csv() {
    println!("id,parent_id,level");

    if let Ok(tree) = serde_json::from_str::<Value>(CALENDAR_TREE_JSON) {
        print_calendar_csv_node(&tree, None, 0);
    } else {
        eprintln!("Failed to parse calendar tree JSON");
    }
}

/// Recursively print a calendar tree node in CSV format.
fn print_calendar_csv_node(node: &Value, parent_id: Option<&str>, level: usize) {
    if let Some(id) = node.get("id").and_then(|v| v.as_str()) {
        let parent_str = parent_id.unwrap_or("");
        println!("{},{},{}", id, parent_str, level);

        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                print_calendar_csv_node(child, Some(id), level + 1);
            }
        }
    }
}

/// Display the locale tree as lines.
fn display_locale_as_lines() {
    if let Ok(tree) = serde_json::from_str::<Value>(LOCALE_TREE_JSON) {
        if let Some(array) = tree.as_array() {
            for node in array {
                print_locale_line_node(node, 0);
            }
        } else {
            // Single root node
            print_locale_line_node(&tree, 0);
        }
    } else {
        eprintln!("Failed to parse locale tree JSON");
    }
}

/// Recursively print a locale tree node as lines.
fn print_locale_line_node(node: &Value, depth: usize) {
    let indent = "  ".repeat(depth);

    if let Some(locale) = node.get("locale").and_then(|v| v.as_str()) {
        println!("{}{}", indent, locale);

        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                print_locale_line_node(child, depth + 1);
            }
        }
    }
}

/// Display the locale tree as CSV in Parent-Child format.
fn display_locale_as_csv() {
    println!("locale,parent_locale,level");

    if let Ok(tree) = serde_json::from_str::<Value>(LOCALE_TREE_JSON) {
        if let Some(array) = tree.as_array() {
            for node in array {
                print_locale_csv_node(node, None, 0);
            }
        } else {
            // Single root node
            print_locale_csv_node(&tree, None, 0);
        }
    } else {
        eprintln!("Failed to parse locale tree JSON");
    }
}

/// Recursively print a locale tree node in CSV format.
fn print_locale_csv_node(node: &Value, parent_locale: Option<&str>, level: usize) {
    if let Some(locale) = node.get("locale").and_then(|v| v.as_str()) {
        let parent_str = parent_locale.unwrap_or("");
        println!("{},{},{}", locale, parent_str, level);

        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                print_locale_csv_node(child, Some(locale), level + 1);
            }
        }
    }
}
