//! Martyrology lookup command handler.

use crate::enums::OutputFormat;
use crate::error::RomcalCliError;
use romcal::Romcal;

/// Handle martyrology command - lookup by exact ID.
pub fn handle(id: &str, output_format: OutputFormat, romcal: Romcal) -> Result<(), RomcalCliError> {
    let entry = romcal
        .get_martyrology_entry(id)
        .ok_or_else(|| RomcalCliError::not_found(format!("Martyrology entry not found: {}", id)))?;

    let json = serde_json::to_value(&entry)?;

    match output_format {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        OutputFormat::Yaml => {
            let yaml = serde_saphyr::to_string(&json).map_err(|e| {
                RomcalCliError::config_error(format!("Failed to serialize to YAML: {}", e))
            })?;
            println!("{}", yaml);
        }
        OutputFormat::Csv => {
            // CSV: id, fullname, name, type, sex, canonizationLevel
            let fullname = entry.fullname.as_deref().unwrap_or("");
            let name = entry.name.as_deref().unwrap_or("");
            let entry_type = format!("{:?}", entry.r#type).to_uppercase();
            let sex = entry
                .sex
                .as_ref()
                .map(|s| format!("{:?}", s).to_uppercase())
                .unwrap_or_default();
            let level = entry
                .canonization_level
                .as_ref()
                .map(|l| format!("{:?}", l).to_uppercase())
                .unwrap_or_default();

            println!("id,fullname,name,type,sex,canonizationLevel");
            println!(
                "{},{},{},{},{},{}",
                escape_csv(&entry.id),
                escape_csv(fullname),
                escape_csv(name),
                entry_type,
                sex,
                level
            );
        }
        OutputFormat::Lines => {
            // Lines: ID/Name  Fullname
            let id = if entry.id.is_empty() {
                entry.name.as_deref().unwrap_or("-")
            } else {
                &entry.id
            };
            let fullname = entry.fullname.as_deref().unwrap_or("");
            println!("{}  {}", id, fullname);
        }
    }

    Ok(())
}

/// Escape a value for CSV output.
fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}
