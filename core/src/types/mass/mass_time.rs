use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Times of Mass celebrations in the liturgical calendar.
/// Different Masses are celebrated at various times and occasions throughout the liturgical year.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MassTime {
    /// Easter Vigil - the most important Mass of the liturgical year, celebrated on Holy Saturday night
    EasterVigil,
    /// Previous Evening Mass - Mass celebrated the evening before a major feast
    PreviousEveningMass,
    /// Night Mass - Mass celebrated during the night hours
    NightMass,
    /// Mass at Dawn - Mass celebrated at dawn, particularly on Easter Sunday
    MassAtDawn,
    /// Morning Mass - Mass celebrated in the morning
    MorningMass,
    /// Mass of the Passion - Mass focusing on Christ's passion, beginning with the procession with palms
    MassOfThePassion,
    /// Celebration of the Passion - special celebration of Christ's passion
    CelebrationOfThePassion,
    /// Day Mass - regular Mass celebrated during the day
    DayMass,
    /// Chrism Mass - Mass where holy oils are blessed, typically on Holy Thursday morning
    ChrismMass,
    /// Evening Mass - Mass celebrated in the evening
    EveningMass,
}
