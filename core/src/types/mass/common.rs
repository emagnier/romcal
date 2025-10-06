use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Common prayers and readings for different categories of saints and celebrations.
/// Provides standardized liturgical texts for various types of commemorations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, EnumIter)]
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

/// Common definition for simplified categorization.
/// Provides a simplified version of the Common enum for easier classification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, EnumIter)]
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

/// Liturgical common information with localized name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CommonInfo {
    /// The common key
    pub key: Common,
    /// The localized name of the common
    pub name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_common_iteration_consistency() {
        // Verify that the order is always the same across multiple iterations
        let first_iteration: Vec<Common> = Common::iter().collect();
        let second_iteration: Vec<Common> = Common::iter().collect();

        assert_eq!(first_iteration, second_iteration);
    }

    #[test]
    fn test_common_definition_iteration_consistency() {
        // Verify that the order is always the same across multiple iterations
        let first_iteration: Vec<CommonDefinition> = CommonDefinition::iter().collect();
        let second_iteration: Vec<CommonDefinition> = CommonDefinition::iter().collect();

        assert_eq!(first_iteration, second_iteration);
    }

    #[test]
    fn test_common_serialization() {
        // Verify that serialization works
        let common = Common::BlessedVirginMary_OrdinaryTime;
        let json = serde_json::to_string(&common).unwrap();
        assert_eq!(json, "\"BLESSED_VIRGIN_MARY__ORDINARY_TIME\"");

        let deserialized: Common = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Common::BlessedVirginMary_OrdinaryTime);
    }

    #[test]
    fn test_common_definition_serialization() {
        // Verify that serialization works
        let common_def = CommonDefinition::BlessedVirginMary;
        let json = serde_json::to_string(&common_def).unwrap();
        assert_eq!(json, "\"BLESSED_VIRGIN_MARY\"");

        let deserialized: CommonDefinition = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, CommonDefinition::BlessedVirginMary);
    }
}
