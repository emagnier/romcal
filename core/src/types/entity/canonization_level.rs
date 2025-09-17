use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Canonization level indicating the official recognition status of a person.
/// Defines whether someone is beatified (Blessed) or canonized (Saint).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CanonizationLevel {
    /// Beatified person (Blessed) - first step toward sainthood
    Blessed,
    /// Canonized person (Saint) - fully recognized as a saint
    Saint,
}
