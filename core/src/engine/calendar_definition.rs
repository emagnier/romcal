#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::types::{CalendarMetadata, DayDefinition, DayId, ParticularConfig};

/// Unique identifier for a calendar
pub type CalendarId = String;

/// Calendar definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
pub struct CalendarDefinition {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    pub id: CalendarId,
    pub metadata: CalendarMetadata,
    pub particular_config: Option<ParticularConfig>,
    pub parent_calendar_ids: Vec<CalendarId>,
    pub days_definitions: BTreeMap<DayId, DayDefinition>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;
    use std::collections::BTreeMap;

    #[test]
    fn test_btreemap_serialization() {
        let mut days_definitions: BTreeMap<String, DayDefinition> = BTreeMap::new();

        days_definitions.insert(
            "easter_sunday".to_string(),
            DayDefinition {
                precedence: Some(crate::types::Precedence::Triduum_1),
                date_def: None,
                date_exceptions: None,
                commons_def: None,
                is_holy_day_of_obligation: Some(true),
                allow_similar_rank_items: Some(false),
                is_optional: Some(false),
                custom_locale_id: None,
                entities: None,
                titles: None,
                drop: None,
                colors: None,
                masses: None,
            },
        );

        // Test serialization
        let json = serde_json::to_string_pretty(&days_definitions).unwrap();
        println!("JSON serialization:");
        println!("{}", json);

        // Test deserialization
        let deserialized: BTreeMap<String, DayDefinition> = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.len(), 1);

        // Test key access
        assert!(deserialized.get("easter_sunday").is_some());
    }
}
