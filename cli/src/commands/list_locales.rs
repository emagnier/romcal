use crate::{enums::OutputFormat, error::RomcalCliError};
use romcal_core::{generated_constants::LOCALE_TREE_JSON, LOCALE_CODES};
use serde_json::{self, Value};

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
            println!("{}", serde_yaml::to_string(items)?);
        }
    }
    Ok(())
}

/// Handle list locales command
pub fn handle_locales(output_format: OutputFormat, tree: bool) -> Result<(), RomcalCliError> {
    if tree {
        display_locale_tree(output_format)
    } else {
        list_items(LOCALE_CODES, output_format)
    }
}

/// Display the locale tree structure.
///
/// This function parses the locale tree JSON and displays it in a hierarchical format.
/// The tree shows the relationships between different locales, with base languages containing
/// their specific variants (e.g., en containing en-gb, en-ie).
fn display_locale_tree(format: OutputFormat) -> Result<(), RomcalCliError> {
    match format {
        OutputFormat::Json => {
            // Output pretty-printed JSON
            let tree: Value = serde_json::from_str(LOCALE_TREE_JSON)?;
            let pretty_json = serde_json::to_string_pretty(&tree)?;
            println!("{}", pretty_json);
        }
        OutputFormat::Yaml => {
            // Parse JSON and convert to YAML
            let tree: Value = serde_json::from_str(LOCALE_TREE_JSON)?;
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

/// Display the locale tree as lines.
fn display_as_lines() {
    if let Ok(tree) = serde_json::from_str::<Value>(LOCALE_TREE_JSON) {
        if let Some(array) = tree.as_array() {
            for node in array {
                print_line_node(node, 0);
            }
        } else {
            // Single root node
            print_line_node(&tree, 0);
        }
    } else {
        eprintln!("Failed to parse locale tree JSON");
    }
}

/// Recursively print a tree node as lines.
fn print_line_node(node: &Value, depth: usize) {
    let indent = "  ".repeat(depth);

    if let Some(locale) = node.get("locale").and_then(|v| v.as_str()) {
        println!("{}{}", indent, locale);

        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                print_line_node(child, depth + 1);
            }
        }
    }
}

/// Display the locale tree as CSV in Parent-Child format.
fn display_as_csv() {
    println!("locale,parent_locale,level");

    if let Ok(tree) = serde_json::from_str::<Value>(LOCALE_TREE_JSON) {
        if let Some(array) = tree.as_array() {
            for node in array {
                print_csv_node(node, None, 0);
            }
        } else {
            // Single root node
            print_csv_node(&tree, None, 0);
        }
    } else {
        eprintln!("Failed to parse locale tree JSON");
    }
}

/// Recursively print a tree node in CSV format.
fn print_csv_node(node: &Value, parent_locale: Option<&str>, level: usize) {
    if let Some(locale) = node.get("locale").and_then(|v| v.as_str()) {
        let parent_str = parent_locale.unwrap_or("");
        println!("{},{},{}", locale, parent_str, level);

        if let Some(children) = node.get("children").and_then(|v| v.as_array()) {
            for child in children {
                print_csv_node(child, Some(locale), level + 1);
            }
        }
    }
}
