use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Metadata for localized resources.
/// Contains all the localized strings and configurations for a specific locale.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ResourcesMetadata {
    /// Ordinal numbers (1st, 2nd, 3rd, etc.) in the locale language
    pub ordinals: Option<BTreeMap<String, String>>,
    /// Weekday names (Sunday, Monday, etc.) in the locale language
    pub weekdays: Option<BTreeMap<String, String>>,
    /// Month names (January, February, etc.) in the locale language
    pub months: Option<BTreeMap<String, String>>,
    /// Liturgical color names in the locale language
    pub colors: Option<LocaleColors>,
    /// Liturgical season names and descriptions in the locale language
    pub seasons: Option<SeasonsMetadata>,
    /// Liturgical period names in the locale language
    pub periods: Option<PeriodsMetadata>,
    /// Liturgical rank names in the locale language
    pub ranks: Option<RanksMetadata>,
    /// Liturgical cycle names in the locale language
    pub cycles: Option<CyclesMetadata>,
}

/// Liturgical color names in the locale language.
/// Provides localized names for each liturgical color.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct LocaleColors {
    /// Black color name in the locale language
    pub black: Option<String>,
    /// Gold color name in the locale language
    pub gold: Option<String>,
    /// Green color name in the locale language
    pub green: Option<String>,
    /// Purple color name in the locale language
    pub purple: Option<String>,
    /// Red color name in the locale language
    pub red: Option<String>,
    /// Rose color name in the locale language
    pub rose: Option<String>,
    /// White color name in the locale language
    pub white: Option<String>,
}

/// Liturgical season names and descriptions in the locale language.
/// Provides localized names for each liturgical season and their components.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SeasonsMetadata {
    /// Advent season names and descriptions
    pub advent: Option<AdventSeason>,
    /// Christmas Time season names and descriptions
    pub christmas_time: Option<ChristmasTimeSeason>,
    /// Ordinary Time season names and descriptions
    pub ordinary_time: Option<OrdinaryTimeSeason>,
    /// Lent season names and descriptions
    pub lent: Option<LentSeason>,
    /// Paschal Triduum season names and descriptions
    pub paschal_triduum: Option<PaschalTriduumSeason>,
    /// Easter Time season names and descriptions
    pub easter_time: Option<EasterTimeSeason>,
}

/// Advent season localized names and descriptions.
/// Provides specific terminology for the Advent season in the locale language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct AdventSeason {
    /// General season name for Advent
    pub season: Option<String>,
    /// Weekday terminology during Advent
    pub weekday: Option<String>,
    /// Sunday terminology during Advent
    pub sunday: Option<String>,
    /// Privileged weekday terminology during Advent
    pub privileged_weekday: Option<String>,
}

/// Christmas Time season localized names and descriptions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ChristmasTimeSeason {
    /// General season name for Christmas Time
    pub season: Option<String>,
    /// Day terminology during Christmas Time
    pub day: Option<String>,
    /// Octave terminology during Christmas Time
    pub octave: Option<String>,
    /// Before Epiphany terminology
    pub before_epiphany: Option<String>,
    /// Second Sunday after Christmas terminology
    pub second_sunday_after_christmas: Option<String>,
    /// After Epiphany terminology
    pub after_epiphany: Option<String>,
}

/// Ordinary Time season localized names and descriptions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct OrdinaryTimeSeason {
    /// General season name for Ordinary Time
    pub season: Option<String>,
    /// Weekday terminology during Ordinary Time
    pub weekday: Option<String>,
    /// Sunday terminology during Ordinary Time
    pub sunday: Option<String>,
}

/// Lent season localized names and descriptions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct LentSeason {
    /// General season name for Lent
    pub season: Option<String>,
    /// Weekday terminology during Lent
    pub weekday: Option<String>,
    /// Sunday terminology during Lent
    pub sunday: Option<String>,
    /// Day after Ash Wednesday terminology
    pub day_after_ash_wed: Option<String>,
    /// Holy Week day terminology
    pub holy_week_day: Option<String>,
}

/// Paschal Triduum season localized names and descriptions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PaschalTriduumSeason {
    /// General season name for Paschal Triduum
    pub season: Option<String>,
}

/// Easter Time season localized names and descriptions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct EasterTimeSeason {
    /// General season name for Easter Time
    pub season: Option<String>,
    /// Weekday terminology during Easter Time
    pub weekday: Option<String>,
    /// Sunday terminology during Easter Time
    pub sunday: Option<String>,
    /// Octave terminology during Easter Time
    pub octave: Option<String>,
}

/// Liturgical period names in the locale language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PeriodsMetadata {
    /// Epiphany period name
    pub epiphany: Option<String>,
    /// Holy Week period name
    pub holy_week: Option<String>,
}

/// Liturgical rank names in the locale language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RanksMetadata {
    /// Solemnity rank name
    pub solemnity: Option<String>,
    /// Sunday rank name
    pub sunday: Option<String>,
    /// Feast rank name
    pub feast: Option<String>,
    /// Memorial rank name
    pub memorial: Option<String>,
    /// Optional memorial rank name
    pub optional_memorial: Option<String>,
    /// Weekday rank name
    pub weekday: Option<String>,
}

/// Liturgical cycle names in the locale language.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CyclesMetadata {
    /// Proper of Time cycle name
    pub proper_of_time: Option<String>,
    /// Proper of Saints cycle name
    pub proper_of_saints: Option<String>,
    /// Sunday Year A cycle name
    pub sunday_year_a: Option<String>,
    /// Sunday Year B cycle name
    pub sunday_year_b: Option<String>,
    /// Sunday Year C cycle name
    pub sunday_year_c: Option<String>,
    /// Weekday Year 1 cycle name
    pub weekday_year_1: Option<String>,
    /// Weekday Year 2 cycle name
    pub weekday_year_2: Option<String>,
    /// Psalter Week 1 cycle name
    pub psalter_week_1: Option<String>,
    /// Psalter Week 2 cycle name
    pub psalter_week_2: Option<String>,
    /// Psalter Week 3 cycle name
    pub psalter_week_3: Option<String>,
    /// Psalter Week 4 cycle name
    pub psalter_week_4: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;

    #[test]
    fn test_serialize_btreemap_alphabetically() {
        let mut ordinals = BTreeMap::new();
        ordinals.insert("3rd".to_string(), "troisième".to_string());
        ordinals.insert("1st".to_string(), "premier".to_string());
        ordinals.insert("2nd".to_string(), "deuxième".to_string());

        let metadata = ResourcesMetadata {
            ordinals: Some(ordinals),
            weekdays: None,
            months: None,
            colors: None,
            seasons: None,
            periods: None,
            ranks: None,
            cycles: None,
        };

        let json = serde_json::to_string_pretty(&metadata).unwrap();

        // Verify that keys are in alphabetical order
        assert!(json.contains("\"1st\""));
        assert!(json.contains("\"2nd\""));
        assert!(json.contains("\"3rd\""));

        // Verify order by checking key positions
        let first_pos = json.find("\"1st\"").unwrap();
        let second_pos = json.find("\"2nd\"").unwrap();
        let third_pos = json.find("\"3rd\"").unwrap();

        assert!(first_pos < second_pos);
        assert!(second_pos < third_pos);
    }
}
