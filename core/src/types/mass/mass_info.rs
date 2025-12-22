use serde::{Deserialize, Serialize, Serializer};

use super::MassTime;

/// Information about a mass celebration for a liturgical day.
/// Contains the type of mass and its localized name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(schemars::JsonSchema))]
pub struct MassInfo {
    /// The type of mass (e.g., DayMass, EasterVigil, etc.)
    /// Serialized as SCREAMING_SNAKE_CASE (e.g., "DAY_MASS")
    #[serde(rename = "type", serialize_with = "serialize_mass_type_uppercase")]
    pub mass_type: MassTime,
    /// The localized name of the mass type (translation key in snake_case)
    pub name: String,
}

/// Serializes MassTime to SCREAMING_SNAKE_CASE (e.g., DayMass -> "DAY_MASS")
fn serialize_mass_type_uppercase<S>(mass_type: &MassTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let snake_case = mass_type_to_key(mass_type);
    let screaming_snake_case = snake_case.to_uppercase();
    serializer.serialize_str(&screaming_snake_case)
}

impl MassInfo {
    /// Creates a new MassInfo from a MassTime.
    /// The name is generated from the MassTime enum variant (snake_case).
    pub fn new(mass_type: MassTime) -> Self {
        Self {
            name: mass_type_to_key(&mass_type),
            mass_type,
        }
    }

    /// Creates the default mass info (DayMass)
    pub fn default_day_mass() -> Vec<Self> {
        vec![Self::new(MassTime::DayMass)]
    }

    /// Creates an empty mass list (for aliturgical days like Holy Saturday)
    pub fn none() -> Vec<Self> {
        vec![]
    }
}

/// Converts a MassTime enum variant to its snake_case key.
/// Example: MassTime::DayMass -> "day_mass"
fn mass_type_to_key(mass_type: &MassTime) -> String {
    // MassTime uses #[serde(rename_all = "snake_case")], so we can serialize to get the key
    serde_json::to_string(mass_type)
        .unwrap_or_default()
        .trim_matches('"')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass_info_new() {
        let mass_info = MassInfo::new(MassTime::DayMass);
        assert_eq!(mass_info.mass_type, MassTime::DayMass);
        assert_eq!(mass_info.name, "day_mass");
    }

    #[test]
    fn test_mass_info_easter_vigil() {
        let mass_info = MassInfo::new(MassTime::EasterVigil);
        assert_eq!(mass_info.mass_type, MassTime::EasterVigil);
        assert_eq!(mass_info.name, "easter_vigil");
    }

    #[test]
    fn test_mass_info_default_day_mass() {
        let masses = MassInfo::default_day_mass();
        assert_eq!(masses.len(), 1);
        assert_eq!(masses[0].mass_type, MassTime::DayMass);
    }

    #[test]
    fn test_mass_info_none() {
        let masses = MassInfo::none();
        assert!(masses.is_empty());
    }

    #[test]
    fn test_mass_info_serialization() {
        let mass_info = MassInfo::new(MassTime::DayMass);
        let json = serde_json::to_string(&mass_info).unwrap();
        // type is serialized as SCREAMING_SNAKE_CASE
        assert!(json.contains("\"type\":\"DAY_MASS\""));
        // name remains in snake_case (translation key)
        assert!(json.contains("\"name\":\"day_mass\""));
    }

    #[test]
    fn test_mass_type_to_key() {
        assert_eq!(mass_type_to_key(&MassTime::DayMass), "day_mass");
        assert_eq!(mass_type_to_key(&MassTime::EasterVigil), "easter_vigil");
        assert_eq!(
            mass_type_to_key(&MassTime::PreviousEveningMass),
            "previous_evening_mass"
        );
        assert_eq!(mass_type_to_key(&MassTime::NightMass), "night_mass");
        assert_eq!(mass_type_to_key(&MassTime::MassAtDawn), "mass_at_dawn");
        assert_eq!(mass_type_to_key(&MassTime::MorningMass), "morning_mass");
        assert_eq!(
            mass_type_to_key(&MassTime::MassOfThePassion),
            "mass_of_the_passion"
        );
        assert_eq!(
            mass_type_to_key(&MassTime::CelebrationOfThePassion),
            "celebration_of_the_passion"
        );
        assert_eq!(mass_type_to_key(&MassTime::ChrismMass), "chrism_mass");
        assert_eq!(
            mass_type_to_key(&MassTime::EveningMassOfTheLordsSupper),
            "evening_mass_of_the_lords_supper"
        );
    }
}
