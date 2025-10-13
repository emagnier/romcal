#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Liturgical seasons of the Church year.
/// Represents the major periods that structure the liturgical calendar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Season {
    /// Advent
    Advent,
    /// Christmas Time
    ChristmasTime,
    /// Lent
    Lent,
    /// Paschal Triduum
    PaschalTriduum,
    /// Easter Time
    EasterTime,
    /// Ordinary Time
    OrdinaryTime,
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_season_iteration_order() {
        let variants: Vec<Season> = Season::iter().collect();

        // Verify that the order is exactly the declaration order
        assert_eq!(variants[0], Season::Advent);
        assert_eq!(variants[1], Season::ChristmasTime);
        assert_eq!(variants[2], Season::Lent);
        assert_eq!(variants[3], Season::PaschalTriduum);
        assert_eq!(variants[4], Season::EasterTime);
        assert_eq!(variants[5], Season::OrdinaryTime);

        // Verify that we have all variants
        assert_eq!(variants.len(), 6);
    }

    #[test]
    fn test_season_iteration_consistency() {
        // Verify that the order is always the same across multiple iterations
        let first_iteration: Vec<Season> = Season::iter().collect();
        let second_iteration: Vec<Season> = Season::iter().collect();

        assert_eq!(first_iteration, second_iteration);
    }

    #[test]
    fn test_season_serialization() {
        // Verify that serialization works
        let season = Season::Advent;
        let json = serde_json::to_string(&season).unwrap();
        assert_eq!(json, "\"ADVENT\"");

        let deserialized: Season = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Season::Advent);
    }
}
