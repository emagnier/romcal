//! Entity lookup command handler.

use crate::enums::OutputFormat;
use crate::error::RomcalCliError;
use romcal::Romcal;

/// Handle entity command - lookup by exact ID.
pub fn handle(id: &str, output_format: OutputFormat, romcal: Romcal) -> Result<(), RomcalCliError> {
    let entity = romcal
        .get_entity(id)
        .ok_or_else(|| RomcalCliError::not_found(format!("Entity not found: {}", id)))?;

    let json = serde_json::to_value(&entity)?;

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
            let id = entity.id.as_deref().unwrap_or("");
            let fullname = entity.fullname.as_deref().unwrap_or("");
            let name = entity.name.as_deref().unwrap_or("");
            let entity_type = entity
                .r#type
                .as_ref()
                .map(|t| format!("{:?}", t).to_uppercase())
                .unwrap_or_default();
            let sex = entity
                .sex
                .as_ref()
                .map(|s| format!("{:?}", s).to_uppercase())
                .unwrap_or_default();
            let level = entity
                .canonization_level
                .as_ref()
                .map(|l| format!("{:?}", l).to_uppercase())
                .unwrap_or_default();

            println!("id,fullname,name,type,sex,canonizationLevel");
            println!(
                "{},{},{},{},{},{}",
                escape_csv(id),
                escape_csv(fullname),
                escape_csv(name),
                entity_type,
                sex,
                level
            );
        }
        OutputFormat::Lines => {
            // Lines: ID/Name  Fullname
            let id = entity
                .id
                .as_deref()
                .or(entity.name.as_deref())
                .unwrap_or("-");
            let fullname = entity.fullname.as_deref().unwrap_or("");
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
