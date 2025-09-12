use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::{CalendarMetadata, DayDefinition, ParticularConfig};

// Type aliases
pub type CalendarId = String;

// Structs
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CalendarDefinition {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    pub id: CalendarId,
    pub metadata: CalendarMetadata,
    pub particular_config: Option<ParticularConfig>,
    pub parent_calendar_ids: Vec<CalendarId>,
    pub days_definitions: Vec<DayDefinition>,
}
