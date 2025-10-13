#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Liturgical colors used in the celebration of Mass and other liturgical services.
/// Each color has specific liturgical significance and is used during particular seasons or celebrations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Color {
    /// Red - used for martyrs, Pentecost, and Palm Sunday
    Red,
    /// Rose - used on Gaudete Sunday (3rd Advent) and Laetare Sunday (4th Lent)
    Rose,
    /// Purple - used during Advent and Lent
    Purple,
    /// Green - used during Ordinary Time
    Green,
    /// White - used for Christmas, Easter, and most feasts
    White,
    /// Gold - used for solemn celebrations and special occasions
    Gold,
    /// Black - used for funerals and All Souls' Day
    Black,
}

/// Liturgical color information with localized name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
pub struct ColorInfo {
    /// The color key
    pub key: Color,
    /// The localized name of the color
    pub name: String,
}
