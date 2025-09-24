use crate::enums::LiturgicalDayFilter;
use crate::error::RomcalCliError;
use crate::utils::current_year;
use romcal_core::Preset;
use serde::{Deserialize, Serialize};
use serde_yaml;

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
    preset: Preset,
) -> Result<(), RomcalCliError> {
    let year = year.unwrap_or_else(current_year);

    // Generate liturgical days using proper_of_time
    let liturgical_days = preset.proper_of_time(year)?;

    // Apply filters if specified
    let output_data = if let Some(filters) = filters {
        let filtered_days: Vec<FilteredLiturgicalDay> = liturgical_days
            .into_iter()
            .map(|day| {
                // Convert to JSON first to preserve field order
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

        filtered_days
    } else {
        // No filters, return all liturgical days
        liturgical_days
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
            .collect::<Result<Vec<FilteredLiturgicalDay>, RomcalCliError>>()?
    };

    // Output the result in YAML format
    let yaml_output = serde_yaml::to_string(&output_data)
        .map_err(|e| RomcalCliError::config_error(format!("Failed to serialize to YAML: {}", e)))?;

    println!("{}", yaml_output);

    Ok(())
}
