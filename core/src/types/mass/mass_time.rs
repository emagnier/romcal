use serde::{Deserialize, Serialize};
use strum::EnumIter;
#[cfg(feature = "ts-bindings")]
use ts_rs::TS;

/// Times of Mass celebrations in the liturgical calendar.
/// Different Masses are celebrated at various times and occasions throughout the liturgical year.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter, PartialOrd, Ord)]
#[cfg_attr(feature = "schema-gen", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema-gen", schemars(rename_all = "SCREAMING_SNAKE_CASE"))]
#[cfg_attr(feature = "ts-bindings", derive(TS))]
#[cfg_attr(
    feature = "ts-bindings",
    ts(export, rename_all = "SCREAMING_SNAKE_CASE")
)]
#[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE", deserialize = "snake_case"))]
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

impl MassTime {
    /// Returns the snake_case key for this MassTime variant.
    /// Used for translation keys (e.g., "day_mass", "easter_vigil").
    pub fn to_snake_case_key(&self) -> &'static str {
        match self {
            MassTime::EasterVigil => "easter_vigil",
            MassTime::PreviousEveningMass => "previous_evening_mass",
            MassTime::NightMass => "night_mass",
            MassTime::MassAtDawn => "mass_at_dawn",
            MassTime::MorningMass => "morning_mass",
            MassTime::MassOfThePassion => "mass_of_the_passion",
            MassTime::CelebrationOfThePassion => "celebration_of_the_passion",
            MassTime::DayMass => "day_mass",
            MassTime::ChrismMass => "chrism_mass",
            MassTime::EveningMassOfTheLordsSupper => "evening_mass_of_the_lords_supper",
        }
    }
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
        // Verify that serialization produces SCREAMING_SNAKE_CASE
        let mass_time = MassTime::DayMass;
        let json = serde_json::to_string(&mass_time).unwrap();
        assert_eq!(json, "\"DAY_MASS\"");
    }

    #[test]
    fn test_mass_time_deserialization() {
        // Verify that deserialization accepts snake_case (for definition files)
        let deserialized: MassTime = serde_json::from_str("\"day_mass\"").unwrap();
        assert_eq!(deserialized, MassTime::DayMass);

        // SCREAMING_SNAKE_CASE should not work for deserialization
        let result: Result<MassTime, _> = serde_json::from_str("\"DAY_MASS\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_to_snake_case_key() {
        assert_eq!(MassTime::DayMass.to_snake_case_key(), "day_mass");
        assert_eq!(MassTime::EasterVigil.to_snake_case_key(), "easter_vigil");
        assert_eq!(
            MassTime::PreviousEveningMass.to_snake_case_key(),
            "previous_evening_mass"
        );
        assert_eq!(MassTime::NightMass.to_snake_case_key(), "night_mass");
        assert_eq!(MassTime::MassAtDawn.to_snake_case_key(), "mass_at_dawn");
        assert_eq!(MassTime::MorningMass.to_snake_case_key(), "morning_mass");
        assert_eq!(
            MassTime::MassOfThePassion.to_snake_case_key(),
            "mass_of_the_passion"
        );
        assert_eq!(
            MassTime::CelebrationOfThePassion.to_snake_case_key(),
            "celebration_of_the_passion"
        );
        assert_eq!(MassTime::ChrismMass.to_snake_case_key(), "chrism_mass");
        assert_eq!(
            MassTime::EveningMassOfTheLordsSupper.to_snake_case_key(),
            "evening_mass_of_the_lords_supper"
        );
    }
}
