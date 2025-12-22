use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Times of Mass celebrations in the liturgical calendar.
/// Different Masses are celebrated at various times and occasions throughout the liturgical year.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter, PartialOrd, Ord)]
#[cfg_attr(feature = "schema-gen", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
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
    /// Evening Mass of the Lord's Supper - Mass celebrated on Holy Thursday evening
    EveningMassOfTheLordsSupper,
}

// Schema generation functions (only compiled when feature "schema-gen" is enabled)
#[cfg(feature = "schema-gen")]
pub fn get_mass_time_description(time: &MassTime) -> &'static str {
    match time {
        MassTime::EasterVigil => {
            "Easter Vigil - the most important Mass of the liturgical year, celebrated on Holy Saturday night"
        }
        MassTime::PreviousEveningMass => {
            "Previous Evening Mass - Mass celebrated the evening before a major feast"
        }
        MassTime::NightMass => "Night Mass - Mass celebrated during the night hours",
        MassTime::MassAtDawn => {
            "Mass at Dawn - Mass celebrated at dawn, particularly on Easter Sunday"
        }
        MassTime::MorningMass => "Morning Mass - Mass celebrated in the morning",
        MassTime::MassOfThePassion => {
            "Mass of the Passion - Mass focusing on Christ's passion, beginning with the procession with palms"
        }
        MassTime::CelebrationOfThePassion => {
            "Celebration of the Passion - special celebration of Christ's passion"
        }
        MassTime::DayMass => "Day Mass - regular Mass celebrated during the day",
        MassTime::ChrismMass => {
            "Chrism Mass - Mass where holy oils are blessed, typically on Holy Thursday morning"
        }
        MassTime::EveningMassOfTheLordsSupper => {
            "Evening Mass of the Lord's Supper - Mass celebrated on Holy Thursday evening"
        }
    }
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
        assert_eq!(variants[9], MassTime::EveningMassOfTheLordsSupper);

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
        assert_eq!(json, "\"day_mass\"");

        let deserialized: MassTime = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, MassTime::DayMass);
    }
}
