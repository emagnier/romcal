#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Sex of a person.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Sex {
    /// Male person
    Male,
    /// Female person
    Female,
}

#[cfg(test)]
mod tests {
    use super::*;
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
}
