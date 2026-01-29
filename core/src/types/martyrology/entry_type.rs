#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, EnumString};
#[cfg(feature = "ts-bindings")]
use ts_rs::TS;

/// The type of entry in the Roman Martyrology.
/// Defines whether the entry represents a person, place, or event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, EnumIter, EnumString)]
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[cfg_attr(feature = "ts-bindings", derive(TS))]
#[cfg_attr(feature = "ts-bindings", ts(export))]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MartyrologyEntryType {
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
    fn test_martyrology_entry_type_parse() {
        assert_eq!(
            MartyrologyEntryType::from_str("person").unwrap(),
            MartyrologyEntryType::Person
        );
        assert_eq!(
            MartyrologyEntryType::from_str("place").unwrap(),
            MartyrologyEntryType::Place
        );
        assert_eq!(
            MartyrologyEntryType::from_str("event").unwrap(),
            MartyrologyEntryType::Event
        );
    }

    #[test]
    fn test_martyrology_entry_type_parse_invalid() {
        assert!(MartyrologyEntryType::from_str("INVALID").is_err());
        assert!(MartyrologyEntryType::from_str("PERSON").is_err()); // Case sensitive
    }
}
