use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Date function for calculating liturgical dates.
/// Represents movable feasts and special celebrations that require calculation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, EnumIter)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DateFn {
    MaryMotherOfTheChurch,
    EpiphanySunday,
    PresentationOfTheLord,
    Annunciation,
    PalmSunday,
    EasterSunday,
    DivineMercySunday,
    ImmaculateHeartOfMary,
    PentecostSunday,
    CorpusChristiSunday,
    NativityOfJohnTheBaptist,
    PeterAndPaulApostles,
    Transfiguration,
    Assumption,
    ExaltationOfTheHolyCross,
    AllSaints,
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
