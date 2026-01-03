use serde::{Deserialize, Serialize};
#[cfg(feature = "ts-bindings")]
use ts_rs::TS;

use super::MassTime;

/// Information about a mass celebration for a liturgical day.
/// Contains the type of mass and its localized name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "ts-bindings", derive(TS))]
#[cfg_attr(feature = "ts-bindings", ts(export))]
pub struct MassInfo {
    /// The type of mass (e.g., DayMass, EasterVigil, etc.)
    /// Serialized as SCREAMING_SNAKE_CASE (e.g., "DAY_MASS")
    #[serde(rename = "type")]
    pub mass_type: MassTime,
    /// The localized name of the mass type (translation key in snake_case)
    pub name: String,
}

impl MassInfo {
    /// Creates a new MassInfo from a MassTime.
    /// The name is generated from the MassTime enum variant (snake_case).
    pub fn new(mass_type: MassTime) -> Self {
        Self {
            name: mass_type.to_snake_case_key().to_string(),
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
}
