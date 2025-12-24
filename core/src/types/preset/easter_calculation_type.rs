#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

/// Easter date calculation method.
///
/// Determines which algorithm to use for calculating the date of Easter Sunday,
/// which is the basis for most movable feasts in the liturgical calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Display, EnumIter)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum EasterCalculationType {
    /// Gregorian calculation (default)
    Gregorian,
    /// Julian calculation converted to Gregorian
    Julian,
}
