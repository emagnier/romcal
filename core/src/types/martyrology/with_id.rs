//! Martyrology entry with resolved ID.
//!
//! This struct represents an entry in the Roman Martyrology that has been resolved
//! from the resources, with a guaranteed ID field.

#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ts-bindings")]
use ts_rs::TS;

use super::{
    CanonizationLevel, MartyrologyEntryDef, MartyrologyEntryId, MartyrologyEntryType, SaintCount,
    SaintDateDef, Sex, Title,
};

/// An entry in the Roman Martyrology with a guaranteed ID.
///
/// This struct represents a saint, blessed, church dedication, or event
/// from the official catalog of the Catholic Church.
/// It is used for entries that have been resolved from the resources,
/// where the ID is always present (e.g., in search results, liturgical days).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[cfg_attr(feature = "ts-bindings", derive(TS))]
#[cfg_attr(feature = "ts-bindings", ts(export))]
pub struct MartyrologyEntry {
    /// The unique identifier of the entry (required)
    pub id: MartyrologyEntryId,

    /// The type of the entry.
    pub r#type: MartyrologyEntryType,

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

    /// Date of Canonization.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub date_of_canonization: Option<SaintDateDef>,

    /// Specify whether an approximate indicator should be added for canonization date.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub date_of_canonization_is_approximative: Option<bool>,

    /// Date of Beatification.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub date_of_beatification: Option<SaintDateDef>,

    /// Specify whether an approximate indicator should be added for beatification date.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub date_of_beatification_is_approximative: Option<bool>,

    /// Specify if the canonization level should not be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub hide_canonization_level: Option<bool>,

    /// Titles of the Saint or the Blessed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub titles: Option<Vec<Title>>,

    /// Determine if the Saint or the Blessed is a male or a female.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub sex: Option<Sex>,

    /// Specify if the titles should not be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub hide_titles: Option<bool>,

    /// Date of Dedication of a place of worship.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub date_of_dedication: Option<SaintDateDef>,

    /// Date of Birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub date_of_birth: Option<SaintDateDef>,

    /// Specify whether an approximate indicator should be added for birth date.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub date_of_birth_is_approximative: Option<bool>,

    /// Date of Death.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub date_of_death: Option<SaintDateDef>,

    /// Specify whether an approximate indicator should be added for death date.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub date_of_death_is_approximative: Option<bool>,

    /// Number of persons that this definition represents.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub count: Option<SaintCount>,

    /// Sources for the information about this entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub sources: Option<Vec<String>>,
}

impl MartyrologyEntry {
    /// Create a new MartyrologyEntry from an ID and a MartyrologyEntryDef.
    pub fn new(id: MartyrologyEntryId, definition: MartyrologyEntryDef) -> Self {
        Self {
            id,
            r#type: definition.r#type.unwrap_or(MartyrologyEntryType::Person),
            fullname: definition.fullname,
            name: definition.name,
            canonization_level: definition.canonization_level,
            date_of_canonization: definition.date_of_canonization,
            date_of_canonization_is_approximative: definition.date_of_canonization_is_approximative,
            date_of_beatification: definition.date_of_beatification,
            date_of_beatification_is_approximative: definition
                .date_of_beatification_is_approximative,
            hide_canonization_level: definition.hide_canonization_level,
            titles: definition.titles,
            sex: definition.sex,
            hide_titles: definition.hide_titles,
            date_of_dedication: definition.date_of_dedication,
            date_of_birth: definition.date_of_birth,
            date_of_birth_is_approximative: definition.date_of_birth_is_approximative,
            date_of_death: definition.date_of_death,
            date_of_death_is_approximative: definition.date_of_death_is_approximative,
            count: definition.count,
            sources: definition.sources,
        }
    }
}

impl From<(MartyrologyEntryId, MartyrologyEntryDef)> for MartyrologyEntry {
    fn from((id, definition): (MartyrologyEntryId, MartyrologyEntryDef)) -> Self {
        MartyrologyEntry::new(id, definition)
    }
}

impl From<(MartyrologyEntryId, &MartyrologyEntryDef)> for MartyrologyEntry {
    fn from((id, definition): (MartyrologyEntryId, &MartyrologyEntryDef)) -> Self {
        MartyrologyEntry::new(id, definition.clone())
    }
}
