use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Times of Mass celebrations in the liturgical calendar.
/// Different Masses are celebrated at various times and occasions throughout the liturgical year.
#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, EnumIter, PartialOrd, Ord,
)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_mass_time_iteration_order() {
        let variants: Vec<MassTime> = MassTime::iter().collect();

        // Verify that the order is exactly the declaration order
        assert_eq!(variants[0], MassTime::EasterVigil);
        assert_eq!(variants[1], MassTime::PreviousEveningMass);
        assert_eq!(variants[2], MassTime::NightMass);
        assert_eq!(variants[3], MassTime::MassAtDawn);
        assert_eq!(variants[4], MassTime::MorningMass);
        assert_eq!(variants[5], MassTime::MassOfThePassion);
        assert_eq!(variants[6], MassTime::CelebrationOfThePassion);
        assert_eq!(variants[7], MassTime::DayMass);
        assert_eq!(variants[8], MassTime::ChrismMass);
        assert_eq!(variants[9], MassTime::EveningMass);

        // Verify that we have all variants
        assert_eq!(variants.len(), 10);
    }

    #[test]
    fn test_mass_time_iteration_consistency() {
        // Verify that the order is always the same across multiple iterations
        let first_iteration: Vec<MassTime> = MassTime::iter().collect();
        let second_iteration: Vec<MassTime> = MassTime::iter().collect();

        assert_eq!(first_iteration, second_iteration);
    }

    #[test]
    fn test_mass_time_serialization() {
        // Verify that serialization always works
        let mass_time = MassTime::DayMass;
        let json = serde_json::to_string(&mass_time).unwrap();
        assert_eq!(json, "\"DAY_MASS\"");

        let deserialized: MassTime = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, MassTime::DayMass);
    }
}
