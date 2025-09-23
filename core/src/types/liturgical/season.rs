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

/// Liturgical season information with localized name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SeasonInfo {
    /// The season key
    pub key: Season,
    /// The localized name of the season
    pub name: String,
}
