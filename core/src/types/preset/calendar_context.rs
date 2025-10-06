use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// The context of the calendar
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CalendarContext {
    /// Gregorian year (January 1 to December 31)
    Gregorian,
    /// Liturgical year (first Sunday of Advent to the day before the first Sunday of Advent of the next year)
    Liturgical,
}
