//! Entity with resolved ID.
//!
//! This struct represents an entity that has been resolved from the resources,
//! with a guaranteed ID field.

#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ts-bindings")]
use ts_rs::TS;

use super::{
    CanonizationLevel, EntityDefinition, EntityId, EntityType, SaintCount, SaintDateDef, Sex, Title,
};

/// An entity with a guaranteed ID.
///
/// This struct is used for entities that have been resolved from the resources,
/// where the ID is always present (e.g., in search results, liturgical days).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[cfg_attr(feature = "ts-bindings", derive(TS))]
#[cfg_attr(feature = "ts-bindings", ts(export))]
pub struct Entity {
    /// The unique identifier of the entity (required)
    pub id: EntityId,

    /// The type of the entity.
    pub r#type: EntityType,

    /// The full name of the entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub fullname: Option<String>,

    /// The short name of the entity, without the canonization level and titles.
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

    /// Sources for the information about this entity.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(optional))]
    pub sources: Option<Vec<String>>,
}

impl Entity {
    /// Create a new Entity from an ID and an EntityDefinition.
    pub fn new(id: EntityId, definition: EntityDefinition) -> Self {
        Self {
            id,
            r#type: definition.r#type.unwrap_or(EntityType::Person),
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

impl From<(EntityId, EntityDefinition)> for Entity {
    fn from((id, definition): (EntityId, EntityDefinition)) -> Self {
        Entity::new(id, definition)
    }
}

impl From<(EntityId, &EntityDefinition)> for Entity {
    fn from((id, definition): (EntityId, &EntityDefinition)) -> Self {
        Entity::new(id, definition.clone())
    }
}
