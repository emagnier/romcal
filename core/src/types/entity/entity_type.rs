#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString};
#[cfg(feature = "ts-bindings")]
use ts_rs::TS;

/// The type of entity in the liturgical calendar.
/// Defines whether the entity represents a person, place, or event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, EnumIter, EnumString)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[cfg_attr(feature = "ts-bindings", derive(TS))]
#[cfg_attr(feature = "ts-bindings", ts(export))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum EntityType {
    /// A person (saint, blessed, or other individual)
    #[default]
    Person,
    /// A place (shrine, city, or geographical location)
    Place,
    /// An event (historical or liturgical occurrence)
    Event,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_entity_type_parse() {
        assert_eq!(EntityType::from_str("PERSON").unwrap(), EntityType::Person);
        assert_eq!(EntityType::from_str("PLACE").unwrap(), EntityType::Place);
        assert_eq!(EntityType::from_str("EVENT").unwrap(), EntityType::Event);
    }

    #[test]
    fn test_entity_type_parse_invalid() {
        assert!(EntityType::from_str("INVALID").is_err());
        assert!(EntityType::from_str("person").is_err()); // Case sensitive
    }
}
