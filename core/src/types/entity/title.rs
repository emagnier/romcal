use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Titles and patronages associated with saints and blessed
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CompoundTitle {
    /// Add title(s) to the end of the existing list of title(s).
    pub append: Option<Vec<Title>>,
    /// Add title(s) to the beginning of the existing list of title(s).
    pub prepend: Option<Vec<Title>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum TitlesDef {
    Titles(Vec<Title>),
    CompoundTitle(CompoundTitle),
}
