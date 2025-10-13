#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// The type of entity in the liturgical calendar.
/// Defines whether the entity represents a person, place, or event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default, EnumIter)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EntityType {
    /// A person (saint, blessed, or other individual)
    #[default]
    Person,
    /// A place (shrine, city, or geographical location)
    Place,
    /// An event (historical or liturgical occurrence)
    Event,
}
