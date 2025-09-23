use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A three-year cycle for Sunday Mass readings (and some solemnities), designated by A, B, or C.
/// Each cycle begins on the First Sunday of Advent of the previous civil year and ends on Saturday
/// after the Christ the King Solemnity. The cycles follow each other in alphabetical order.
/// C year is always divisible by 3, A has remainder of 1, and B remainder of 2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SundayCycle {
    /// Year A
    YearA,
    /// Year B
    YearB,
    /// Year C
    YearC,
}

/// A two-year cycle for the weekday Mass readings (also called Cycle I and Cycle II).
/// Odd-numbered years are the Cycle I (year 1); even-numbered ones are the Cycle II (year 2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WeekdayCycle {
    /// Year 1 (Cycle I)
    Year1,
    /// Year 2 (Cycle II)
    Year2,
}

/// [GILH §133] The four-week cycle of the psalter is coordinated with the liturgical year in such a way that
/// on the First Sunday of Advent, the First Sunday in Ordinary Time, the First Sunday of Lent,
/// and Easter Sunday the cycle is always begun again with Week 1 (others being omitted when necessary).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PsalterWeekCycle {
    /// Week 1
    Week1,
    /// Week 2
    Week2,
    /// Week 3
    Week3,
    /// Week 4
    Week4,
}

/// Sunday cycle information with localized name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SundayCycleInfo {
    /// The Sunday cycle key
    pub key: SundayCycle,
    /// The localized name of the Sunday cycle
    pub name: String,
}

/// Weekday cycle information with localized name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct WeekdayCycleInfo {
    /// The weekday cycle key
    pub key: WeekdayCycle,
    /// The localized name of the weekday cycle
    pub name: String,
}

/// Psalter week cycle information with localized name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PsalterWeekCycleInfo {
    /// The psalter week cycle key
    pub key: PsalterWeekCycle,
    /// The localized name of the psalter week cycle
    pub name: String,
}
