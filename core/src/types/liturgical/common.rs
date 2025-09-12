use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Common prayers and readings for different categories of saints and celebrations
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

/// Common definition for simplified categorization
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
