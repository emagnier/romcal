#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString};
#[cfg(feature = "ts-bindings")]
use ts_rs::TS;

/// Sex of a person.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter, EnumString)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[cfg_attr(feature = "ts-bindings", derive(TS))]
#[cfg_attr(feature = "ts-bindings", ts(export))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Sex {
    /// Male person
    Male,
    /// Female person
    Female,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use strum::IntoEnumIterator;

    #[test]
    fn test_sex_iteration_consistency() {
        // Verify that the order is always the same across multiple iterations
        let first_iteration: Vec<Sex> = Sex::iter().collect();
        let second_iteration: Vec<Sex> = Sex::iter().collect();

        assert_eq!(first_iteration, second_iteration);
    }

    #[test]
    fn test_sex_serialization() {
        // Verify that serialization works
        let sex = Sex::Male;
        let json = serde_json::to_string(&sex).unwrap();
        assert_eq!(json, "\"MALE\"");

        let deserialized: Sex = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Sex::Male);
    }

    #[test]
    fn test_sex_parse() {
        assert_eq!(Sex::from_str("MALE").unwrap(), Sex::Male);
        assert_eq!(Sex::from_str("FEMALE").unwrap(), Sex::Female);
        assert!(Sex::from_str("INVALID").is_err());
    }
}
