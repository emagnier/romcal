#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// The type of the calendar.
/// Defines the scope and authority level of the liturgical calendar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CalendarType {
    /// General Roman Calendar (universal)
    GeneralRoman,
    /// Regional calendar (multiple countries)
    Region,
    /// National calendar (single country)
    Country,
    /// Archdiocesan calendar
    Archdiocese,
    /// Diocesan calendar
    Diocese,
    /// City calendar
    City,
    /// Parish calendar
    Parish,
    /// General religious community calendar
    GeneralCommunity,
    /// Regional religious community calendar
    RegionalCommunity,
    /// Local religious community calendar
    LocalCommunity,
    /// Other specialized calendar
    Other,
}
