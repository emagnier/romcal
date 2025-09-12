use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Specific periods within liturgical seasons
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Period {
    ChristmasOctave,
    DaysBeforeEpiphany,
    DaysFromEpiphany,
    ChristmasToPresentationOfTheLord,
    PresentationOfTheLordToHolyThursday,
    HolyWeek,
    EasterOctave,
    EarlyOrdinaryTime,
    LateOrdinaryTime,
}
