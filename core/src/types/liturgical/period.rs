#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Specific periods within liturgical seasons.
/// Defines sub-periods that have special liturgical characteristics or rules.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Period {
    /// The eight days following Christmas (December 25 - January 1)
    ChristmasOctave,
    /// Days before Epiphany (January 2 to the day before Epiphany)
    DaysBeforeEpiphany,
    /// Days from Epiphany to the Presentation (January 6 to the day before the Presentation of the Lord)
    DaysFromEpiphany,
    /// Period from Christmas to the Presentation of the Lord
    ChristmasToPresentationOfTheLord,
    /// Period from the Presentation to Holy Thursday
    PresentationOfTheLordToHolyThursday,
    /// Holy Week (Palm Sunday to Holy Saturday)
    HolyWeek,
    /// Paschal Triduum (start from the Thursday of the Lord's Supper to the Easter Sunday Vespers)
    PaschalTriduum,
    /// The eight days following Easter Sunday
    EasterOctave,
    /// Early Ordinary Time (after the Presentation of the Lord to the day before Ash Wednesday)
    EarlyOrdinaryTime,
    /// Late Ordinary Time (after Pentecost to the day before the First Sunday of Advent)
    LateOrdinaryTime,
}

/// Liturgical period information with localized name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
pub struct PeriodInfo {
    /// The period key
    pub key: Period,
    /// The localized name of the period
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_period_iteration_order() {
        let variants: Vec<Period> = Period::iter().collect();

        // Verify that the order is exactly the declaration order
        assert_eq!(variants[0], Period::ChristmasOctave);
        assert_eq!(variants[1], Period::DaysBeforeEpiphany);
        assert_eq!(variants[2], Period::DaysFromEpiphany);
        assert_eq!(variants[3], Period::ChristmasToPresentationOfTheLord);
        assert_eq!(variants[4], Period::PresentationOfTheLordToHolyThursday);
        assert_eq!(variants[5], Period::HolyWeek);
        assert_eq!(variants[6], Period::PaschalTriduum);
        assert_eq!(variants[7], Period::EasterOctave);
        assert_eq!(variants[8], Period::EarlyOrdinaryTime);
        assert_eq!(variants[9], Period::LateOrdinaryTime);

        // Verify that we have all variants
        assert_eq!(variants.len(), 10);
    }

    #[test]
    fn test_period_iteration_consistency() {
        // Verify that the order is always the same across multiple iterations
        let first_iteration: Vec<Period> = Period::iter().collect();
        let second_iteration: Vec<Period> = Period::iter().collect();

        assert_eq!(first_iteration, second_iteration);
    }

    #[test]
    fn test_period_serialization() {
        // Verify that serialization works
        let period = Period::ChristmasOctave;
        let json = serde_json::to_string(&period).unwrap();
        assert_eq!(json, "\"CHRISTMAS_OCTAVE\"");

        let deserialized: Period = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Period::ChristmasOctave);
    }
}
