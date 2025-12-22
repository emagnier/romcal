#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Common prayers and readings for different categories of saints and celebrations.
/// Provides standardized liturgical texts for various types of commemorations.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(non_camel_case_types)]
pub enum Common {
    /// No common.
    None,

    // Dedication of a Church
    /// Dedication anniversary (in the Church that was Dedicated).
    DedicationAnniversary_Inside,
    /// Dedication anniversary (outside the Church that was Dedicated).
    DedicationAnniversary_Outside,

    // Blessed Virgin Mary
    /// Common of the Blessed Virgin Mary (Ordinary Time).
    BlessedVirginMary_OrdinaryTime,
    /// Common of the Blessed Virgin Mary (Advent).
    BlessedVirginMary_Advent,
    /// Common of the Blessed Virgin Mary (Christmas Time).
    BlessedVirginMary_Christmas,
    /// Common of the Blessed Virgin Mary (Easter Time).
    BlessedVirginMary_Easter,

    // Martyrs
    /// Common of Several Martyrs (outside Easter).
    Martyrs_OutsideEaster_Several,
    /// Common of One Martyr (outside Easter).
    Martyrs_OutsideEaster_One,
    /// Common of Several Martyrs (Easter Time).
    Martyrs_Easter_Several,
    /// Common of One Martyr (Easter Time).
    Martyrs_Easter_One,
    /// Common for Several Missionary Martyrs.
    Martyrs_Missionary_Several,
    /// Common for One Missionary Martyr.
    Martyrs_Missionary_One,
    /// Common for Virgin Martyrs.
    Martyrs_Virgin,
    /// Common for Holy Woman Martyrs.
    Martyrs_Woman,

    // Pastors
    /// Common for a Pope or for a Bishop
    Pastors_PopeOrBishop,
    /// Common for a Bishop
    Pastors_Bishop,
    /// Common for Several Pastors
    Pastors_Several,
    /// Common for One Pastor
    Pastors_One,
    /// Common for one Founder
    Pastors_Founder_One,
    /// Common for several Founders
    Pastors_Founder_Several,
    /// Common for Missionaries
    Pastors_Missionary,

    // Doctors of the Church
    /// Common for Doctors of the Church.
    DoctorsOfTheChurch,

    // Virgins
    /// Common for Several Virgins
    Virgins_Several,
    /// Common for One Virgin
    Virgins_One,

    // Holy Men and Women
    /// Common for Several Holy Men and Women
    Saints_All_Several,
    /// Common for One Holy Man or Woman
    Saints_All_One,
    /// Common for an Abbot
    Saints_Abbot,
    /// Common for a Monk
    Saint_Monk,
    /// Common for a Nun
    Saints_Nun,
    /// Common for Religious
    Saints_Religious,
    /// Common for Those Who Practiced Works of Mercy
    Saints_MercyWorks,
    /// Common for Educators
    Saints_Educators,
    /// Common for Holy Women
    Saints_HolyWomen,
}

/// Common definition for simplified categorization.
/// Provides a simplified version of the Common enum for easier classification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(non_camel_case_types)]
pub enum CommonDefinition {
    /// No common.
    None,

    // Dedication of a Church
    /// Dedication anniversary (in the Church that was Dedicated).
    DedicationAnniversary_Inside,
    /// Dedication anniversary (outside the Church that was Dedicated).
    DedicationAnniversary_Outside,

    // Blessed Virgin Mary
    /// Common of the Blessed Virgin Mary.
    BlessedVirginMary,

    // Martyrs
    /// Common for Martyrs.
    Martyrs,
    /// Common for Missionary Martyrs.
    MissionaryMartyrs,
    /// Common for Virgin Martyrs.
    VirginMartyrs,
    /// Common for Holy Woman Martyrs.
    WomanMartyrs,

    // Pastors
    /// Common for Pastors.
    Pastors,
    /// Common for Popes.
    Popes,
    /// Common for Bishops.
    Bishops,
    /// Common for Founders.
    Founders,
    /// Common for Missionaries.
    Missionaries,

    // Doctors of the Church
    /// Common for Doctors of the Church.
    DoctorsOfTheChurch,

    // Virgins
    /// Common for Virgins.
    Virgins,

    // Holy Men and Women
    /// Common for Holy Men and Women.
    Saints,
    /// Common for Abbots.
    Abbots,
    /// Common for Monks.
    Monks,
    /// Common for Nuns.
    Nuns,
    /// Common for Religious.
    Religious,
    /// Common for Those Who Practiced Works of Mercy.
    MercyWorkers,
    /// Common for Educators.
    Educators,
    /// Common for Holy Women.
    HolyWomen,
}

/// Liturgical common information with localized name.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
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
