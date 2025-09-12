use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::{CalendarJurisdiction, CalendarType};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CalendarMetadata {
    pub r#type: CalendarType,
    pub jurisdiction: CalendarJurisdiction,
}
