use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Titles and patronages associated with saints and blessed.
/// Represents the various ecclesiastical titles and patronages that can be assigned to entities.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
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
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CompoundTitle {
    /// The title(s) to add to the end of the existing list of title(s)
    pub append: Option<Vec<Title>>,
    /// The title(s) to add to the beginning of the existing list of title(s)
    pub prepend: Option<Vec<Title>>,
}

/// Title definition that can be either a simple list or a compound definition.
/// Supports both direct title lists and compound title operations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum TitlesDef {
    /// Simple list of titles
    Titles(Vec<Title>),
    /// Compound title definition with append/prepend operations
    CompoundTitle(CompoundTitle),
}
