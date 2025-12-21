#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Titles and patronages associated with saints and blessed.
/// Represents the various ecclesiastical titles and patronages that can be assigned to entities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Title {
    // Original Title variants
    Abbess,
    Abbot,
    Apostle,
    Archangel,
    Bishop,
    Deacon,
    DoctorOfTheChurch,
    Empress,
    Evangelist,
    FirstBishop,
    Hermit,
    King,
    Martyr,
    Missionary,
    Monk,
    MotherAndQueenOfChile,
    ParentsOfTheBlessedVirginMary,
    Pope,
    Patriarch,
    Pilgrim,
    Priest,
    Prophet,
    ProtoMartyrOfOceania,
    Queen,
    QueenOfPoland,
    Religious,
    SlavicMissionary,
    SpouseOfTheBlessedVirginMary,
    TheFirstMartyr,
    Virgin,

    // PatronTitle variants
    CopatronOfEurope,
    CopatronOfIreland,
    CopatronOfCanada,
    CopatronessOfEurope,
    CopatronessOfFrance,
    CopatronessOfIreland,
    CopatronessOfItalyAndEurope,
    CopatronessOfThePhilippines,
    PatronOfCanada,
    PatronOfEngland,
    PatronOfEurope,
    PatronOfFrance,
    PatronOfIreland,
    PatronOfItaly,
    PatronOfOceania,
    PatronOfPoland,
    PatronOfRussia,
    PatronOfScotland,
    PatronOfSpain,
    PatronOfTheCzechNation,
    PatronOfTheDiocese,
    PatronOfWales,
    PatronessOfAlsace,
    PatronessOfArgentina,
    PatronessOfBrazil,
    PatronessOfHungary,
    PatronessOfPuertoRico,
    PatronessOfSlovakia,
    PatronessOfTheAmericas,
    PatronessOfThePhilippines,
    PatronessOfTheProvinceOfQuebec,
    PatronessOfTheUsa,
    PatronOfTheClergyOfTheArchdioceseOfLyon,
    PatronOfTheCityOfLyon,
    PatronessOfCostaRica,
    PrincipalPatronOfTheDiocese,
    SecondPatronOfTheDiocese,
}

/// Compound title definition for combining multiple titles.
/// Allows adding titles to the beginning or end of an existing title list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
pub struct CompoundTitle {
    /// The title(s) to add to the end of the existing list of title(s)
    pub append: Option<Vec<Title>>,
    /// The title(s) to add to the beginning of the existing list of title(s)
    pub prepend: Option<Vec<Title>>,
}

impl Title {
    /// Returns true if this title indicates a martyr.
    ///
    /// Martyr titles include:
    /// - `Martyr`
    /// - `TheFirstMartyr`
    /// - `ProtoMartyrOfOceania`
    pub fn is_martyr_title(&self) -> bool {
        matches!(
            self,
            Title::Martyr | Title::TheFirstMartyr | Title::ProtoMartyrOfOceania
        )
    }
}

/// Title definition that can be either a simple list or a compound definition.
/// Supports both direct title lists and compound title operations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(untagged)]
pub enum TitlesDef {
    /// Simple list of titles
    Titles(Vec<Title>),
    /// Compound title definition with append/prepend operations
    CompoundTitle(CompoundTitle),
}

impl TitlesDef {
    /// Returns true if any of the titles indicates a martyr.
    pub fn contains_martyr(&self) -> bool {
        match self {
            TitlesDef::Titles(titles) => titles.iter().any(|t| t.is_martyr_title()),
            TitlesDef::CompoundTitle(ct) => {
                ct.append
                    .as_ref()
                    .is_some_and(|v| v.iter().any(|t| t.is_martyr_title()))
                    || ct
                        .prepend
                        .as_ref()
                        .is_some_and(|v| v.iter().any(|t| t.is_martyr_title()))
            }
        }
    }

    /// Returns true if the TitlesDef has no titles.
    pub fn is_empty(&self) -> bool {
        match self {
            TitlesDef::Titles(titles) => titles.is_empty(),
            TitlesDef::CompoundTitle(ct) => {
                ct.append.as_ref().is_none_or(|v| v.is_empty())
                    && ct.prepend.as_ref().is_none_or(|v| v.is_empty())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_title_iteration_consistency() {
        // Verify that the order is always the same across multiple iterations
        let first_iteration: Vec<Title> = Title::iter().collect();
        let second_iteration: Vec<Title> = Title::iter().collect();

        assert_eq!(first_iteration, second_iteration);
    }

    #[test]
    fn test_title_serialization() {
        // Verify that serialization works
        let title = Title::Abbot;
        let json = serde_json::to_string(&title).unwrap();
        assert_eq!(json, "\"ABBOT\"");

        let deserialized: Title = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Title::Abbot);
    }

    #[test]
    fn test_is_martyr_title() {
        assert!(Title::Martyr.is_martyr_title());
        assert!(Title::TheFirstMartyr.is_martyr_title());
        assert!(Title::ProtoMartyrOfOceania.is_martyr_title());

        assert!(!Title::Abbot.is_martyr_title());
        assert!(!Title::Bishop.is_martyr_title());
        assert!(!Title::Virgin.is_martyr_title());
    }

    #[test]
    fn test_titles_def_contains_martyr() {
        // Simple list with martyr
        let titles_with_martyr = TitlesDef::Titles(vec![Title::Bishop, Title::Martyr]);
        assert!(titles_with_martyr.contains_martyr());

        // Simple list without martyr
        let titles_without_martyr = TitlesDef::Titles(vec![Title::Bishop, Title::Virgin]);
        assert!(!titles_without_martyr.contains_martyr());

        // Empty list
        let empty_titles = TitlesDef::Titles(vec![]);
        assert!(!empty_titles.contains_martyr());

        // Compound title with martyr in append
        let compound_with_martyr_append = TitlesDef::CompoundTitle(CompoundTitle {
            append: Some(vec![Title::Martyr]),
            prepend: None,
        });
        assert!(compound_with_martyr_append.contains_martyr());

        // Compound title with martyr in prepend
        let compound_with_martyr_prepend = TitlesDef::CompoundTitle(CompoundTitle {
            append: None,
            prepend: Some(vec![Title::TheFirstMartyr]),
        });
        assert!(compound_with_martyr_prepend.contains_martyr());

        // Compound title without martyr
        let compound_without_martyr = TitlesDef::CompoundTitle(CompoundTitle {
            append: Some(vec![Title::Bishop]),
            prepend: Some(vec![Title::Virgin]),
        });
        assert!(!compound_without_martyr.contains_martyr());
    }

    #[test]
    fn test_titles_def_is_empty() {
        assert!(TitlesDef::Titles(vec![]).is_empty());
        assert!(!TitlesDef::Titles(vec![Title::Martyr]).is_empty());

        assert!(
            TitlesDef::CompoundTitle(CompoundTitle {
                append: None,
                prepend: None
            })
            .is_empty()
        );

        assert!(
            !TitlesDef::CompoundTitle(CompoundTitle {
                append: Some(vec![Title::Bishop]),
                prepend: None
            })
            .is_empty()
        );
    }
}
