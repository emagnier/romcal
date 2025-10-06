use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// The type of Easter calculation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EasterCalculationType {
    /// Gregorian calculation (default)
    Gregorian,
    /// Julian calculation converted to Gregorian
    Julian,
}
