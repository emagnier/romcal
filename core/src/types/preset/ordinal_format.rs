#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Format for displaying ordinal numbers.
///
/// - `Letters`: Display ordinals as words (e.g., "first", "second", "premier", "deuxième")
/// - `Numeric`: Display ordinals as numbers with suffixes (e.g., "1st", "2nd", "1er", "2e")
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum OrdinalFormat {
    /// Ordinals displayed as words
    Letters,
    /// Ordinals displayed as numbers with suffixes (default)
    #[default]
    Numeric,
}
