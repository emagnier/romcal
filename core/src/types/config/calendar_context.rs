use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Calendar context
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CalendarContext {
    /// Civil year (January 1 to December 31)
    Gregorian,
    /// Liturgical year (first Sunday of Advent to the day before the first Sunday of Advent of the next year)
    Liturgical,
}
