#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Date function for calculating liturgical dates.
///
/// Represents movable feasts and special celebrations that require calculation
/// based on Easter or other variable dates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DateFn {
    /// Monday after Pentecost.
    MaryMotherOfTheChurch,
    /// Sunday between January 2 and 8 (or January 6 if not transferred).
    EpiphanySunday,
    /// February 2 (Candlemas).
    PresentationOfTheLord,
    /// March 25 (may be transferred if in Holy Week or Easter Octave).
    Annunciation,
    /// Sunday before Easter.
    PalmSunday,
    /// First Sunday after the Paschal Full Moon.
    EasterSunday,
    /// Second Sunday of Easter.
    DivineMercySunday,
    /// Saturday after the Second Sunday after Pentecost.
    ImmaculateHeartOfMary,
    /// Seventh Sunday after Easter.
    PentecostSunday,
    /// Thursday or Sunday after Trinity Sunday.
    CorpusChristiSunday,
    /// June 24.
    NativityOfJohnTheBaptist,
    /// June 29.
    PeterAndPaulApostles,
    /// August 6.
    Transfiguration,
    /// August 15.
    Assumption,
    /// September 14.
    ExaltationOfTheHolyCross,
    /// November 1.
    AllSaints,
    /// December 8.
    ImmaculateConceptionOfMary,
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_date_fn_iteration_consistency() {
        // Verify that the order is always the same across multiple iterations
        let first_iteration: Vec<DateFn> = DateFn::iter().collect();
        let second_iteration: Vec<DateFn> = DateFn::iter().collect();

        assert_eq!(first_iteration, second_iteration);
    }

    #[test]
    fn test_date_fn_serialization() {
        // Verify that serialization works
        let date_fn = DateFn::EasterSunday;
        let json = serde_json::to_string(&date_fn).unwrap();
        assert_eq!(json, "\"EASTER_SUNDAY\"");

        let deserialized: DateFn = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, DateFn::EasterSunday);
    }
}
