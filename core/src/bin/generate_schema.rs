use romcal_core::calendar_def::*;
use schemars::schema_for;
use std::fs;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create the schemas directory if it doesn't exist (at the project root)
    let schemas_dir = Path::new("../schemas");
    if !schemas_dir.exists() {
        fs::create_dir_all(schemas_dir)?;
    }

    // Generate the main schema for CalendarDefinition
    let calendar_schema = schema_for!(CalendarDefinition);
    let calendar_schema_json = serde_json::to_string_pretty(&calendar_schema)?;
    fs::write(
        schemas_dir.join("calendar-definition.json"),
        calendar_schema_json,
    )?;
    println!("✅ CalendarDefinition schema exported to schemas/calendar-definition.json");

    // Generate the schema for DayDefinition
    let day_schema = schema_for!(DayDefinition);
    let day_schema_json = serde_json::to_string_pretty(&day_schema)?;
    fs::write(schemas_dir.join("day-definition.json"), day_schema_json)?;
    println!("✅ DayDefinition schema exported to schemas/day-definition.json");

    // Generate the schema for Precedence
    let precedence_schema = schema_for!(Precedence);
    let precedence_schema_json = serde_json::to_string_pretty(&precedence_schema)?;
    fs::write(schemas_dir.join("precedence.json"), precedence_schema_json)?;
    println!("✅ Precedence schema exported to schemas/precedence.json");

    // Generate the schema for all main types
    let all_types_schema = schema_for!(CalendarDefinition);
    let all_types_json = serde_json::to_string_pretty(&all_types_schema)?;
    fs::write(schemas_dir.join("all-types.json"), all_types_json)?;
    println!("✅ Complete schema exported to schemas/all-types.json");

    println!("\n🎉 All JSON schemas have been generated successfully!");
    println!("📁 Destination directory: {}", schemas_dir.display());

    Ok(())
}
