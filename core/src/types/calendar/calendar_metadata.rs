#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::{CalendarJurisdiction, CalendarType};

/// Metadata for a calendar.
/// Contains essential information about the calendar's type and jurisdiction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
pub struct CalendarMetadata {
    /// The type of the calendar
    pub r#type: CalendarType,
    /// The jurisdiction of the calendar
    pub jurisdiction: CalendarJurisdiction,
}
