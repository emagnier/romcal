use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// The jurisdiction of the calendar.
/// Determines whether the calendar follows ecclesiastical or civil authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CalendarJurisdiction {
    /// Calendar under ecclesiastical authority (Church)
    Ecclesiastical,
    /// Calendar under civil authority (State)
    Civil,
}
