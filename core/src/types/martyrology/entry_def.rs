#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ts-bindings")]
use ts_rs::TS;

use super::{CanonizationLevel, MartyrologyEntryType, SaintCount, SaintDateDef, Sex, Title};

/// The unique identifier of a martyrology entry
pub type MartyrologyEntryId = String;

fn default_entry_type() -> Option<MartyrologyEntryType> {
    Some(MartyrologyEntryType::Person)
}

/// Definition of an entry in the Roman Martyrology.
///
/// A martyrology entry represents a person (saint, blessed), place (church, shrine),
/// or event in the official catalog of the Catholic Church.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[cfg_attr(feature = "ts-bindings", derive(TS))]
#[cfg_attr(feature = "ts-bindings", ts(export))]
pub struct MartyrologyEntryDef {
    /// The type of the martyrology entry.
    ///
    /// Defaults to `MartyrologyEntryType::Person`.
    #[serde(
        default = "default_entry_type",
        skip_serializing_if = "Option::is_none"
    )]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub r#type: Option<MartyrologyEntryType>,

    /// The full name of the entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub fullname: Option<String>,

    /// The short name of the entry, without the canonization level and titles.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub name: Option<String>,

    /// The canonization level of a person.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub canonization_level: Option<CanonizationLevel>,

    /// Date of Canonization, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
    /// or an object describing date range, multiple possible date, or a century.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub date_of_canonization: Option<SaintDateDef>,

    /// Specify whether an approximate indicator should be added, when the date is displayed.
    /// For example in English: 'c. 201'.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub date_of_canonization_is_approximative: Option<bool>,

    /// Date of Beatification, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
    /// or an object describing date range, multiple possible date, or a century.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub date_of_beatification: Option<SaintDateDef>,

    /// Specify whether an approximate indicator should be added, when the date is displayed.
    /// For example in English: 'c. 201'.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub date_of_beatification_is_approximative: Option<bool>,

    /// Specify if the canonization level should not be displayed.
    /// It's generally the case when the canonization are already included in the name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub hide_canonization_level: Option<bool>,

    /// Titles of the Saint or the Blessed
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub titles: Option<Vec<Title>>,

    /// Determine if the Saint or the Blessed is a male or a female.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub sex: Option<Sex>,

    /// Specify if the titles should not be displayed.
    /// It's generally the case when titles are already included in the name.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub hide_titles: Option<bool>,

    /// Date of Dedication of a church, basilica, or cathedral (or other place of worship),
    /// as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
    /// or an object describing date range, multiple possible date, or a century.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub date_of_dedication: Option<SaintDateDef>,

    /// Date of Birth, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
    /// or an object describing date range, multiple possible date, or a century.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub date_of_birth: Option<SaintDateDef>,

    /// Specify whether an approximate indicator should be added, when the date is displayed.
    /// For example in English: 'c. 201'.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub date_of_birth_is_approximative: Option<bool>,

    /// Date of Death, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
    /// or an object describing date range, multiple possible date, or a century.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub date_of_death: Option<SaintDateDef>,

    /// Specify whether an approximate indicator should be added, when the date is displayed.
    /// For example in English: 'c. 201'.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub date_of_death_is_approximative: Option<bool>,

    /// Number of person that this definition represent.
    /// It could be set as 'many' if the number is not defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub count: Option<SaintCount>,

    /// Sources for the information about this entry
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub sources: Option<Vec<String>>,

    /// Internal notes (not serialized).
    #[serde(skip_serializing)]
    #[cfg_attr(feature = "ts-bindings", ts(skip))]
    pub _todo: Option<Vec<String>>,
}

impl Default for MartyrologyEntryDef {
    fn default() -> Self {
        Self::new()
    }
}

impl MartyrologyEntryDef {
    /// Create a new MartyrologyEntryDef with default values
    pub fn new() -> Self {
        Self {
            r#type: Some(MartyrologyEntryType::Person),
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
