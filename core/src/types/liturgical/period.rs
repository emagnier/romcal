use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Specific periods within liturgical seasons.
/// Defines sub-periods that have special liturgical characteristics or rules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Period {
    /// The eight days following Christmas (December 25 - January 1)
    ChristmasOctave,
    /// Days before Epiphany (January 2 to the day before Epiphany)
    DaysBeforeEpiphany,
    /// Days from Epiphany to the Presentation (January 6 to the day before the Presentation of the Lord)
    DaysFromEpiphany,
    /// Period from Christmas to the Presentation of the Lord
    ChristmasToPresentationOfTheLord,
    /// Period from the Presentation to Holy Thursday
    PresentationOfTheLordToHolyThursday,
    /// Holy Week (Palm Sunday to Holy Saturday)
    HolyWeek,
    /// The eight days following Easter Sunday
    EasterOctave,
    /// Early Ordinary Time (after the Presentation of the Lord to the day before Ash Wednesday)
    EarlyOrdinaryTime,
    /// Late Ordinary Time (after Pentecost to the day before the First Sunday of Advent)
    LateOrdinaryTime,
}

/// Liturgical period information with localized name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PeriodInfo {
    /// The period key
    pub key: Period,
    /// The localized name of the period
    pub name: String,
}
