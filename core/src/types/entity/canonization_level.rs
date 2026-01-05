#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString};
#[cfg(feature = "ts-bindings")]
use ts_rs::TS;

/// Canonization level indicating the official recognition status of a person.
/// Defines whether someone is beatified (Blessed) or canonized (Saint).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter, EnumString)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[cfg_attr(feature = "ts-bindings", derive(TS))]
#[cfg_attr(feature = "ts-bindings", ts(export))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum CanonizationLevel {
    /// Beatified person (Blessed) - first step toward sainthood
    Blessed,
    /// Canonized person (Saint) - fully recognized as a saint
    Saint,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use strum::IntoEnumIterator;

    #[test]
    fn test_canonization_level_iteration_order() {
        let variants: Vec<CanonizationLevel> = CanonizationLevel::iter().collect();

        // Verify that the order is exactly the declaration order
        assert_eq!(variants[0], CanonizationLevel::Blessed);
        assert_eq!(variants[1], CanonizationLevel::Saint);

        // Verify that we have all variants
        assert_eq!(variants.len(), 2);
    }

    #[test]
    fn test_canonization_level_iteration_consistency() {
        // Verify that the order is always the same across multiple iterations
        let first_iteration: Vec<CanonizationLevel> = CanonizationLevel::iter().collect();
        let second_iteration: Vec<CanonizationLevel> = CanonizationLevel::iter().collect();

        assert_eq!(first_iteration, second_iteration);
    }

    #[test]
    fn test_canonization_level_serialization() {
        // Verify that serialization works
        let level = CanonizationLevel::Blessed;
        let json = serde_json::to_string(&level).unwrap();
        assert_eq!(json, "\"BLESSED\"");

        let deserialized: CanonizationLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, CanonizationLevel::Blessed);
    }

    #[test]
    fn test_canonization_level_parse() {
        assert_eq!(
            CanonizationLevel::from_str("SAINT").unwrap(),
            CanonizationLevel::Saint
        );
        assert_eq!(
            CanonizationLevel::from_str("BLESSED").unwrap(),
            CanonizationLevel::Blessed
        );
        assert!(CanonizationLevel::from_str("INVALID").is_err());
    }
}
