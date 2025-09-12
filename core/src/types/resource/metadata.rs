use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ResourcesMetadata {
    pub ordinals: Option<HashMap<String, String>>,
    pub weekdays: Option<HashMap<String, String>>,
    pub months: Option<HashMap<String, String>>,
    pub colors: Option<LocaleColors>,
    pub seasons: Option<SeasonsMetadata>,
    pub periods: Option<PeriodsMetadata>,
    pub ranks: Option<RanksMetadata>,
    pub cycles: Option<CyclesMetadata>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct LocaleColors {
    pub black: Option<String>,
    pub gold: Option<String>,
    pub green: Option<String>,
    pub purple: Option<String>,
    pub red: Option<String>,
    pub rose: Option<String>,
    pub white: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SeasonsMetadata {
    pub advent: Option<AdventSeason>,
    pub christmas_time: Option<ChristmasTimeSeason>,
    pub ordinary_time: Option<OrdinaryTimeSeason>,
    pub lent: Option<LentSeason>,
    pub paschal_triduum: Option<PaschalTriduumSeason>,
    pub easter_time: Option<EasterTimeSeason>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct AdventSeason {
    pub season: Option<String>,
    pub weekday: Option<String>,
    pub sunday: Option<String>,
    pub privileged_weekday: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ChristmasTimeSeason {
    pub season: Option<String>,
    pub day: Option<String>,
    pub octave: Option<String>,
    pub before_epiphany: Option<String>,
    pub second_sunday_after_christmas: Option<String>,
    pub after_epiphany: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct OrdinaryTimeSeason {
    pub season: Option<String>,
    pub weekday: Option<String>,
    pub sunday: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct LentSeason {
    pub season: Option<String>,
    pub weekday: Option<String>,
    pub sunday: Option<String>,
    pub day_after_ash_wed: Option<String>,
    pub holy_week_day: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PaschalTriduumSeason {
    pub season: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct EasterTimeSeason {
    pub season: Option<String>,
    pub weekday: Option<String>,
    pub sunday: Option<String>,
    pub octave: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PeriodsMetadata {
    pub epiphany: Option<String>,
    pub holy_week: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RanksMetadata {
    pub solemnity: Option<String>,
    pub sunday: Option<String>,
    pub feast: Option<String>,
    pub memorial: Option<String>,
    pub optional_memorial: Option<String>,
    pub weekday: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CyclesMetadata {
    pub proper_of_time: Option<String>,
    pub proper_of_saints: Option<String>,
    pub sunday_year_a: Option<String>,
    pub sunday_year_b: Option<String>,
    pub sunday_year_c: Option<String>,
    pub weekday_year_1: Option<String>,
    pub weekday_year_2: Option<String>,
    pub psalter_week_1: Option<String>,
    pub psalter_week_2: Option<String>,
    pub psalter_week_3: Option<String>,
    pub psalter_week_4: Option<String>,
}
