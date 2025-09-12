use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Liturgical colors used in the celebration of Mass and other liturgical services
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Color {
    Red,
    Rose,
    Purple,
    Green,
    White,
    Gold,
    Black,
}
