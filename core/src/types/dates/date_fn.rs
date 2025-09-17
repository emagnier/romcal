use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Date function for calculating liturgical dates.
/// Represents movable feasts and special celebrations that require calculation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
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
