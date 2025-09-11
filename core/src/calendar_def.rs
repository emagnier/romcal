use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};

use crate::types::saint_count::SaintCount;

// Macro to generate SingleOrMultiple types
macro_rules! single_or_multiple {
    ($name:ident, $type:ty) => {
        #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
        #[serde(untagged)]
        pub enum $name {
            Single($type),
            Multiple(Vec<$type>),
        }
    };
}

// Type aliases
pub type CalendarId = String;
pub type DayId = String;
pub type LocaleId = String;
pub type ResourceId = String;

// Validated types with automatic serde validation
/// Month index (1-12) with automatic validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
pub struct MonthIndex(pub u8);

impl<'de> Deserialize<'de> for MonthIndex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        if (1..=12).contains(&value) {
            Ok(MonthIndex(value))
        } else {
            Err(serde::de::Error::custom(format!(
                "Month must be between 1 and 12, got {}",
                value
            )))
        }
    }
}

/// Day of week (0-6, where 0=Sunday) with automatic validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
pub struct DayOfWeek(pub u8);

impl<'de> Deserialize<'de> for DayOfWeek {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        if (0..=6).contains(&value) {
            Ok(DayOfWeek(value))
        } else {
            Err(serde::de::Error::custom(format!(
                "Day of week must be between 0 and 6, got {}",
                value
            )))
        }
    }
}

// Enums
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CalendarType {
    GeneralRoman,
    Region,
    Country,
    Archdiocese,
    Diocese,
    City,
    Parish,
    GeneralCommunity,
    RegionalCommunity,
    LocalCommunity,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CalendarJurisdiction {
    Ecclesiastical,
    Civil,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EasterCalculationType {
    Gregorian,
    Julian,
}

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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(non_camel_case_types)]
pub enum Precedence {
    // 1 - The Paschal Triduum of the Passion and Resurrection of the Lord.
    Triduum_1,

    // 2 - The Nativity of the Lord, the Epiphany, the Ascension, or Pentecost.
    ProperOfTimeSolemnity_2,
    // 2 - A Sunday of Advent, Lent, or Easter.
    PrivilegedSunday_2,
    // 2 - Ash Wednesday.
    AshWednesday_2,
    // 2 - A weekday of Holy Week from Monday up to and including Thursday.
    WeekdayOfHolyWeek_2,
    // 2 - A day within the Octave of Easter.
    WeekdayOfEasterOctave_2,

    // 3 - A Solemnity inscribed in the General Calendar, whether of the Lord, of the Blessed Virgin Mary, or of a Saint.
    GeneralSolemnity_3,

    // 3 - The Commemoration of All the Faithful Departed.
    CommemorationOfAllTheFaithfulDeparted_3,

    // 4 - Proper Solemnity.
    // 4a - A proper Solemnity of the principal Patron of the place, city, or state.
    ProperSolemnity_PrincipalPatron_4a,
    // 4b - The Solemnity of the dedication and of the anniversary of the dedication of the own church.
    ProperSolemnity_DedicationOfTheOwnChurch_4b,
    // 4c - The solemnity of the title of the own church.
    ProperSolemnity_TitleOfTheOwnChurch_4c,
    // 4d - A Solemnity either of the Title or of the Founder or of the principal Patron of an Order or Congregation.
    ProperSolemnity_TitleOrFounderOrPrimaryPatronOfAReligiousOrg_4d,

    // 5 - A Feast of the Lord inscribed in the General Calendar.
    GeneralLordFeast_5,

    // 6 - A Sunday of Christmas Time or a Sunday in Ordinary Time.
    UnprivilegedSunday_6,

    // 7 - A Feast of the Blessed Virgin Mary or of a Saint in the General Calendar.
    GeneralFeast_7,

    // 8 - Proper Feast
    // 8a - The Proper Feast of the principal Patron of the diocese.
    ProperFeast_PrincipalPatronOfADiocese_8a,
    // 8b - The Proper Feast of the anniversary of the dedication of the cathedral church
    ProperFeast_DedicationOfTheCathedralChurch_8b,
    // 8c - The Proper Feast of the principal Patron of a region or province, or a country, or of a wider territory.
    ProperFeast_PrincipalPatronOfARegion_8c,
    // 8d - The Proper Feast of the Title, Founder, or principal Patron of an Order or Congregation
    ProperFeast_TitleOrFounderOrPrimaryPatronOfAReligiousOrg_8d,
    // 8e - Other Feast, proper to an individual church.
    ProperFeast_ToAnIndividualChurch_8e,
    // 8f - Other Proper Feast inscribed in the Calendar of each diocese or Order or Congregation.
    ProperFeast_8f,

    // 9 - Privileged Weekday
    PrivilegedWeekday_9,

    // 10 - Obligatory Memorials in the General Calendar.
    GeneralMemorial_10,

    // 11 - Proper Obligatory Memorial.
    // 11a - Proper Obligatory Memorial of a secondary Patron of the place, diocese, region, or religious province.
    ProperMemorial_SecondPatron_11a,
    // 11b - Other Proper Obligatory Memorial inscribed in the Calendar of each diocese, or Order or congregation.
    ProperMemorial_11b,

    // 12 - Optional Memorial
    OptionalMemorial_12,

    // 13 - Weekday
    Weekday_13,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Rank {
    /// Solemnities are counted among the most important days, whose celebration
    /// begins with First Vespers (Evening Prayer I) on the preceding day. Some Solemnities
    /// are also endowed with their own Vigil Mass, which is to be used on the evening of the
    /// preceding day, if an evening Mass is celebrated. (UNLY #11)
    Solemnity,

    /// On the first day of each week, which is known as the Day of the Lord or the Lord's
    /// Day, the Church, by an apostolic tradition that draws its origin from the very day of
    /// the Resurrection of Christ, celebrates the Paschal Mystery. Hence, Sunday must be
    /// considered the primordial feast day. (UNLY #4)
    Sunday,

    /// Feasts are celebrated within the limits of the natural day; accordingly they have
    /// no First Vespers (Evening Prayer I), except in the case of Feasts of the Lord that fall
    /// on a Sunday in Ordinary Time or in Christmas Time and which replace the Sunday
    /// Office. (UNLY #13)
    Feast,

    /// **Obligatory memorials** are liturgical commemorations of saints, events, or aspects of the
    /// faith. Their observance is mandatory and integrated into the celebration of the occurring
    /// weekday, following the liturgical norms outlined in the General Instruction of the Roman Missal
    /// and the Liturgy of the Hours.
    /// When an **obligatory memorial** falls on a weekday during the liturgical season of Lent or a
    /// privileged weekday of Advent, it must only be celebrated as an **optional memorial**, as Lent
    /// and Advent have their own specific liturgical observances that take precedence.
    Memorial,

    /// **Optional memorials** are liturgical commemorations of saints, events, or aspects of the
    /// faith, but they are not obligatory.
    /// Their observance is integrated into the celebration of the occurring weekday, adhering to the
    /// liturgical norms provided in the General Instruction of the Roman Missal and the Liturgy of
    /// the Hours.
    /// In cases where multiple **optional memorials** are designated on the same day in the liturgical
    /// calendar, only one of them may be celebrated, and the others must be omitted (UNLY #14).
    /// This allows for some flexibility in choosing which optional memorial to commemorate when
    /// multiple options are available.
    OptionalMemorial,

    /// The days of the week that follow Sunday are called weekdays; however, they are
    /// celebrated differently according to the importance of each.
    ///
    /// a. Ash Wednesday and the weekdays of Holy Week, from Monday up to and including
    ///    Thursday, take precedence over all other celebrations.
    /// b. The weekdays of Advent from 17 December up to and including 24 December
    ///    and all the weekdays of Lent have precedence over Obligatory Memorials.
    /// c. Other weekdays give way to all Solemnities and Feasts and are combined with
    ///    Memorials.
    ///
    ///  (UNLY #16)
    Weekday,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(non_camel_case_types)]
pub enum Common {
    // No common
    None,

    // Dedication of a Church
    DedicationAnniversary_Inside,
    DedicationAnniversary_Outside,

    // Blessed Virgin Mary
    BlessedVirginMary_OrdinaryTime,
    BlessedVirginMary_Advent,
    BlessedVirginMary_Christmas,
    BlessedVirginMary_Easter,

    // Martyrs
    Martyrs_OutsideEaster_Several,
    Martyrs_OutsideEaster_One,
    Martyrs_Easter_Several,
    Martyrs_Easter_One,
    Martyrs_Missionary_Several,
    Martyrs_Missionary_One,
    Martyrs_Virgin,
    Martyrs_Woman,

    // Pastors
    Pastors_PopeOrBishop,
    Pastors_Bishop,
    Pastors_Several,
    Pastors_One,
    Pastors_Founder_One,
    Pastors_Founder_Several,
    Pastors_Missionary,

    // Doctors of the Church
    DoctorsOfTheChurch,

    // Virgins
    Virgins_Several,
    Virgins_One,

    // Holy Men and Women
    Saints_All_Several,
    Saints_All_One,
    Saints_Abbot,
    Saint_Monk,
    Saints_Nun,
    Saints_Religious,
    Saints_MercyWorks,
    Saints_Educators,
    Saints_HolyWomen,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(non_camel_case_types)]
pub enum CommonDefinition {
    // No common
    None,

    // Dedication of a Church
    DedicationAnniversary_Inside,
    DedicationAnniversary_Outside,

    // Blessed Virgin Mary
    BlessedVirginMary,

    // Martyrs
    Martyrs,
    MissionaryMartyrs,
    VirginMartyrs,
    WomanMartyrs,

    // Pastors
    Pastors,
    Popes,
    Bishops,
    Founders,
    Missionaries,

    // Doctors of the Church
    DoctorsOfTheChurch,

    // Virgins
    Virgins,

    // Holy Men and Women
    Saints,
    Abbots,
    Monks,
    Nuns,
    Religious,
    MercyWorkers,
    Educators,
    HolyWomen,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Season {
    Advent,
    ChristmasTime,
    Lent,
    PaschalTriduum,
    EasterTime,
    OrdinaryTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Colors {
    Red,
    Rose,
    Purple,
    Green,
    White,
    Gold,
    Black,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Period {
    ChristmasOctave,
    DaysBeforeEpiphany,
    DaysFromEpiphany,
    ChristmasToPresentationOfTheLord,
    PresentationOfTheLordToHolyThursday,
    HolyWeek,
    EasterOctave,
    EarlyOrdinaryTime,
    LateOrdinaryTime,
}

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

// Wrapper structs for primitive types with validation
impl MonthIndex {
    pub fn value(&self) -> u8 {
        self.0
    }
}

impl DayOfWeek {
    pub fn value(&self) -> u8 {
        self.0
    }
}

// Union types using enums
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum DateDef {
    /// Simple month/day
    MonthDate {
        month: MonthIndex,
        date: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        day_offset: Option<i32>,
    },
    /// Date function (Easter, Epiphany, etc.)
    DateFunction {
        date_fn: DateFn,
        #[serde(skip_serializing_if = "Option::is_none")]
        day_offset: Option<i32>,
    },
    /// Nth weekday of month
    WeekdayOfMonth {
        month: MonthIndex,
        day_of_week: DayOfWeek,
        nth_week_in_month: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        day_offset: Option<i32>,
    },
    /// Last weekday of month
    LastWeekdayOfMonth {
        month: MonthIndex,
        last_day_of_week_in_month: DayOfWeek,
        #[serde(skip_serializing_if = "Option::is_none")]
        day_offset: Option<i32>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum DateDefExtended {
    DateDef(DateDef),
    WithOffset(DateDefWithOffset),
}

/// The liturgical day date exception
/// Represents a condition and the date to set when that condition is met
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct DateDefException {
    /// Condition that triggers the exception
    pub when: ExceptionCondition,
    /// Date to set when condition is met
    pub then: DateDefExtended,
}

/// Exception conditions that can trigger a date change
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ExceptionCondition {
    /// If date is between two dates
    IsBetween {
        from: Box<DateDef>,
        to: Box<DateDef>,
        inclusive: bool,
    },
    /// If date is same as another date
    IsSameAsDate { date: Box<DateDef> },
    /// If date is a specific day of week
    IsDayOfWeek { day_of_week: DayOfWeek },
}

/// Date exceptions that can be either a single exception or an array of exceptions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum DateDefExceptions {
    Single(DateDefException),
    Multiple(Vec<DateDefException>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum MartyrologyItemPointer {
    ResourceId(ResourceId),
    Redefined(MartyrologyItemRedefined),
}

// Utilisation de la macro pour les types SingleOrMultiple
single_or_multiple!(CommonsDef, CommonDefinition);
single_or_multiple!(ColorsDef, Colors);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum TitlesDef {
    Titles(Vec<Title>),
    CompoundTitle(CompoundTitle),
}

// Structs
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CalendarMetadata {
    pub r#type: CalendarType,
    pub jurisdiction: CalendarJurisdiction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ParticularConfig {
    /// Configuration options specific to this calendar.
    /// These settings can override or extend the default Romcal configuration or any parent calendar
    /// configuration.
    pub ascension_on_sunday: Option<bool>,
    pub epiphany_on_sunday: Option<bool>,
    pub corpus_christi_on_sunday: Option<bool>,
    pub easter_calculation_type: Option<EasterCalculationType>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CalendarDefinition {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,
    pub id: CalendarId,
    pub metadata: CalendarMetadata,
    pub particular_config: Option<ParticularConfig>,
    pub parent_calendar_ids: Vec<CalendarId>,
    pub days_definitions: Vec<DayDefinition>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct DateDefWithOffset {
    pub day_offset: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CompoundTitle {
    /// Add title(s) to the end of the existing list of title(s).
    pub append: Option<Vec<Title>>,
    /// Add title(s) to the beginning of the existing list of title(s).
    pub prepend: Option<Vec<Title>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct MartyrologyItemRedefined {
    /// The ID of the martyrology item.
    pub id: String,
    /// The redefined titles of the martyrology item.
    pub titles: Option<TitlesDef>,
    /// Specify if titles should not be displayed. This can occur when a title is already included in
    /// the name of the martyrology item.
    pub hide_titles: Option<bool>,
    /// Specify the number of persons this martyrology item is representing.
    pub count: Option<SaintCount>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct DayDefinition {
    pub id: DayId,
    /// Date definition
    pub date_def: Option<DateDef>,
    /// Date definition exception
    pub date_exceptions: Option<DateDefExceptions>,
    /// The precedence type of the liturgical day.
    pub precedence: Option<Precedence>,
    /// The **Common** refers to a set of prayers, readings, and chants used for celebrating saints or
    /// feasts that belong to a specific category, such as martyrs, virgins, pastors, or the Blessed
    /// Virgin Mary.
    pub commons_def: Option<CommonsDef>,
    /// Holy days of obligation are days on which the faithful are expected to attend Mass,
    /// and engage in rest from work and recreation.
    pub is_holy_day_of_obligation: Option<bool>,
    /// In addition to this liturgical day, allow similar items that have the same rank,
    /// and the same or lower precedence,
    /// so the current liturgical day will not overwrite another defined item.
    pub allow_similar_rank_items: Option<bool>,
    /// Specify is this LiturgicalDay is optional within a specific liturgical calendar.
    ///
    /// UNLY #14:
    /// Memorials are either obligatory or optional; their observance is integrated into
    /// the celebration of the occurring weekday in accordance with the norms set forth in the
    /// General Instruction of the Roman Missal and of the Liturgy of the Hours
    ///
    /// Note: also used for the dedication of consecrated churches, which is an optional solemnity
    /// that should not overwrite the default weekday.
    pub is_optional: Option<bool>,
    /// Specify a custom locale ID for this date definition, in this calendar.
    pub custom_locale_id: Option<LocaleId>,
    /// Link one or multiple Saints, Blessed, or any other celebrations from the Martyrology catalog.
    pub martyrology: Option<Vec<MartyrologyItemPointer>>,
    /// Combined titles of each Saints linked to this date definition.
    pub titles: Option<TitlesDef>,
    /// If this liturgical day must be removed from this calendar and from all those it inherits
    /// (the parent calendars), on the final calendar generated by romcal.
    pub drop: Option<bool>,
    /// The liturgical color(s) of the liturgical day.
    /// @deprecated
    pub colors: Option<ColorsDef>,
}
