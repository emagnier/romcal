#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Acclamations used in liturgical celebrations.
/// Acclamations are short liturgical responses or exclamations used during Mass.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Acclamation {
    /// Alleluia - joyful acclamation used outside of Lent
    Alleluia,
    /// Lent - acclamation used during Lenten season
    Lent,
    /// Mixed - combination of different acclamation types
    Mixed,
    /// None - no acclamation
    None,
}

impl Acclamation {
    /// Check if a value is a valid Acclamation.
    /// This function provides the same functionality as the TypeScript `isAcclamationType` function.
    pub fn is_valid(value: &str) -> bool {
        matches!(value, "ALLELUIA" | "LENT" | "MIXED" | "NONE")
    }
}
