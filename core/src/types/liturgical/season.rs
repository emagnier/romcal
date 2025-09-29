use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Liturgical seasons of the Church year.
/// Represents the major periods that structure the liturgical calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Season {
    /// Advent
    Advent,
    /// Christmas Time
    ChristmasTime,
    /// Lent
    Lent,
    /// Paschal Triduum
    PaschalTriduum,
    /// Easter Time
    EasterTime,
    /// Ordinary Time
    OrdinaryTime,
}
