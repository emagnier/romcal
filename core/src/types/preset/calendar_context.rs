#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

/// Calendar year context for date boundaries.
///
/// Determines how the calendar year is structured and which dates are included
/// in a given year's calendar output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumIter)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum CalendarContext {
    /// Gregorian year (January 1 to December 31)
    Gregorian,
    /// Liturgical year (first Sunday of Advent to the day before the first Sunday of Advent of the next year)
    Liturgical,
}
