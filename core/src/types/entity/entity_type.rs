use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EntityType {
    #[default]
    Person,
    Place,
    Event,
}
