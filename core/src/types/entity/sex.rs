use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Sex of a person.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Sex {
    /// Male person
    Male,
    /// Female person
    Female,
}
