#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{CanonizationLevel, EntityType, SaintCount, SaintDateDef, Sex, Title};

/// The unique identifier of the entity
pub type EntityId = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
pub struct Entity {
    /// The type of the entity.
    /// @default EntityType.Person
    pub r#type: Option<EntityType>,

    /// The full name of the entity.
    pub fullname: Option<String>,

    /// The short name of the entity, without the canonization level and titles.
    pub name: Option<String>,

    /// The canonization level of a person.
    pub canonization_level: Option<CanonizationLevel>,

    /// Date of Canonization, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
    /// or an object describing date range, multiple possible date, or a century.
    pub date_of_canonization: Option<SaintDateDef>,

    /// Specify whether an approximate indicator should be added, when the date is displayed.
    /// For example in English: 'c. 201'.
    pub date_of_canonization_is_approximative: Option<bool>,

    /// Date of Beatification, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
    /// or an object describing date range, multiple possible date, or a century.
    pub date_of_beatification: Option<SaintDateDef>,

    /// Specify whether an approximate indicator should be added, when the date is displayed.
    /// For example in English: 'c. 201'.
    pub date_of_beatification_is_approximative: Option<bool>,

    /// Specify if the canonization level should not be displayed.
    /// It's generally the case when the canonization are already included in the name.
    pub hide_canonization_level: Option<bool>,

    /// Titles of the Saint or the Blessed
    pub titles: Option<Vec<Title>>,

    /// Determine if the Saint or the Blessed is a male or a female.
    pub sex: Option<Sex>,

    /// Specify if the titles should not be displayed.
    /// It's generally the case when titles are already included in the name.
    pub hide_titles: Option<bool>,

    /// Date of Dedication of a church, basilica, or cathedral (or other place of worship),
    /// as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
    /// or an object describing date range, multiple possible date, or a century.
    pub date_of_dedication: Option<SaintDateDef>,

    /// Date of Birth, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
    /// or an object describing date range, multiple possible date, or a century.
    pub date_of_birth: Option<SaintDateDef>,

    /// Specify whether an approximate indicator should be added, when the date is displayed.
    /// For example in English: 'c. 201'.
    pub date_of_birth_is_approximative: Option<bool>,

    /// Date of Death, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
    /// or an object describing date range, multiple possible date, or a century.
    pub date_of_death: Option<SaintDateDef>,

    /// Specify whether an approximate indicator should be added, when the date is displayed.
    /// For example in English: 'c. 201'.
    pub date_of_death_is_approximative: Option<bool>,

    /// Number of person that this definition represent.
    /// It could be set as 'many' if the number is not defined.
    pub count: Option<SaintCount>,

    /// Sources for the information about this entity
    pub sources: Option<Vec<String>>,

    /// Internal notes
    /// @private
    pub _todo: Option<Vec<String>>,
}

impl Default for Entity {
    fn default() -> Self {
        Self::new()
    }
}

impl Entity {
    /// Create a new Entity with default values
    pub fn new() -> Self {
        Self {
            r#type: Some(EntityType::Person),
            fullname: None,
            name: None,
            canonization_level: None,
            date_of_canonization: None,
            date_of_canonization_is_approximative: None,
            date_of_beatification: None,
            date_of_beatification_is_approximative: None,
            hide_canonization_level: None,
            titles: None,
            sex: None,
            hide_titles: None,
            date_of_dedication: None,
            date_of_birth: None,
            date_of_birth_is_approximative: None,
            date_of_death: None,
            date_of_death_is_approximative: None,
            count: None,
            sources: None,
            _todo: None,
        }
    }
}
