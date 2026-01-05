//! Entity search command handler.

use crate::enums::OutputFormat;
use crate::error::RomcalCliError;
use romcal::Romcal;
use romcal::search::EntityQuery;
use romcal::types::entity::{CanonizationLevel, EntityType, Sex, Title};

/// Search options from CLI arguments.
pub struct SearchOptions {
    pub text: Option<String>,
    pub entity_type: Option<EntityType>,
    pub sex: Option<Sex>,
    pub level: Option<CanonizationLevel>,
    pub titles: Option<Vec<Title>>,
    pub limit: usize,
    pub min_score: f64,
}

/// Handle search command - fuzzy search with filters.
pub fn handle(
    options: SearchOptions,
    output_format: OutputFormat,
    romcal: Romcal,
) -> Result<(), RomcalCliError> {
    // Build the query from CLI options
    let query = EntityQuery {
        text: options.text,
        entity_type: options.entity_type,
        canonization_level: options.level,
        sex: options.sex,
        titles: options.titles,
        limit: Some(options.limit),
        min_score: Some(options.min_score),
    };

    // Execute search
    let results = romcal.search_entities(query);

    if results.is_empty() {
        return Err(RomcalCliError::not_found("No entities found".to_string()));
    }

    match output_format {
        OutputFormat::Json => {
            let json: Vec<serde_json::Value> = results
                .iter()
                .map(|r| {
                    serde_json::json!({
                        "entity": r.entity,
                        "score": r.score,
                        "matchType": r.match_type.to_string(),
                        "matchedFields": r.matched_fields,
                    })
                })
                .collect();
            println!("{}", serde_json::to_string_pretty(&json)?);
        }
        OutputFormat::Yaml => {
            let json: Vec<serde_json::Value> = results
                .iter()
                .map(|r| {
                    serde_json::json!({
                        "entity": r.entity,
                        "score": r.score,
                        "matchType": r.match_type.to_string(),
                        "matchedFields": r.matched_fields,
                    })
                })
                .collect();
            let yaml = serde_saphyr::to_string(&json).map_err(|e| {
                RomcalCliError::config_error(format!("Failed to serialize to YAML: {}", e))
            })?;
            println!("{}", yaml);
        }
        OutputFormat::Csv => {
            // CSV header
            println!("id,score,matchType,fullname,name,type,sex,canonizationLevel");

            for r in &results {
                let fullname = r.entity.fullname.as_deref().unwrap_or("");
                let name = r.entity.name.as_deref().unwrap_or("");
                let entity_type = format!("{:?}", r.entity.r#type).to_uppercase();
                let sex = r
                    .entity
                    .sex
                    .as_ref()
                    .map(|s| format!("{:?}", s).to_uppercase())
                    .unwrap_or_default();
                let level = r
                    .entity
                    .canonization_level
                    .as_ref()
                    .map(|l| format!("{:?}", l).to_uppercase())
                    .unwrap_or_default();

                println!(
                    "{},{:.2},{},{},{},{},{},{}",
                    escape_csv(&r.entity.id),
                    r.score,
                    r.match_type,
                    escape_csv(fullname),
                    escape_csv(name),
                    entity_type,
                    sex,
                    level
                );
            }
        }
        OutputFormat::Lines => {
            // Lines: ID/Name  Score  Fullname
            for r in &results {
                let id = if r.entity.id.is_empty() {
                    r.entity.name.as_deref().unwrap_or("-")
                } else {
                    &r.entity.id
                };
                let fullname = r.entity.fullname.as_deref().unwrap_or("");
                println!("{}  {:.2}  {}", id, r.score, fullname);
            }
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
