use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::resource::ResourcesMetadata;
use crate::types::{CanonizationLevel, EntityType, SaintCount, SaintDateDef, Sex, Title};

// Type aliases
pub type LocaleId = String;
pub type EntityId = String;

// Structs
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ResourcesDefinition {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,

    /// Locale code of the resources, in BCP-47 IETF tag format
    pub locale: LocaleId,

    /// Metadata of the resources
    pub metadata: Option<ResourcesMetadata>,

    /// Entities of the resources: a person, a place, an event, etc.
    pub entities: Option<Vec<EntityDefinition>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct EntityDefinition {
    /// The unique identifier of the entity
    pub id: EntityId,

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

// Implementations

impl EntityDefinition {
    /// Create a new EntityDefinition with the given ID and default values
    pub fn new(id: EntityId) -> Self {
        Self {
            id,
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

impl ResourcesDefinition {
    /// Create a new ResourcesDefinition with the given locale
    pub fn new(locale: LocaleId) -> Self {
        Self {
            schema: None,
            locale,
            metadata: None,
            entities: None,
        }
    }

    /// Add an entity to the resources
    pub fn add_entity(&mut self, entity: EntityDefinition) {
        let entities = self.entities.get_or_insert_with(Vec::new);
        entities.push(entity);
    }

    /// Get an entity by its ID
    pub fn get_entity(&self, id: &str) -> Option<&EntityDefinition> {
        self.entities
            .as_ref()?
            .iter()
            .find(|entity| entity.id == id)
    }

    /// Get a mutable reference to an entity by its ID
    pub fn get_entity_mut(&mut self, id: &str) -> Option<&mut EntityDefinition> {
        self.entities
            .as_mut()?
            .iter_mut()
            .find(|entity| entity.id == id)
    }

    /// Remove an entity by its ID
    pub fn remove_entity(&mut self, id: &str) -> Option<EntityDefinition> {
        if let Some(entities) = &mut self.entities {
            if let Some(pos) = entities.iter().position(|entity| entity.id == id) {
                return Some(entities.remove(pos));
            }
        }
        None
    }

    /// Get all entity IDs
    pub fn get_entity_ids(&self) -> Vec<&String> {
        self.entities
            .as_ref()
            .map(|entities| entities.iter().map(|entity| &entity.id).collect())
            .unwrap_or_default()
    }

    /// Validate that all entities are properly structured
    /// Check for uniqueness of IDs and entity structure
    pub fn validate_entities(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();

        if let Some(entities) = &self.entities {
            for entity in entities {
                // Check for duplicate IDs
                if seen_ids.contains(&entity.id) {
                    errors.push(format!("Duplicate entity ID: '{}'", entity.id));
                } else {
                    seen_ids.insert(&entity.id);
                }

                // Validate entity structure
                if entity.name.is_none() && entity.fullname.is_none() {
                    errors.push(format!(
                        "Entity '{}' must have either 'name' or 'fullname'",
                        entity.id
                    ));
                }

                // Validate canonization level consistency
                if let Some(level) = &entity.canonization_level {
                    if entity.hide_canonization_level == Some(true) && entity.fullname.is_some() {
                        // This is OK - canonization level is hidden because it's in the fullname
                    } else if entity.fullname.is_none() {
                        errors.push(format!(
                            "Entity '{}' has canonization level '{}' but no fullname to display it",
                            entity.id,
                            match level {
                                CanonizationLevel::Blessed => "BLESSED",
                                CanonizationLevel::Saint => "SAINT",
                            }
                        ));
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Merge entities from another ResourcesDefinition
    pub fn merge_entities(&mut self, other: &ResourcesDefinition) {
        if let Some(other_entities) = &other.entities {
            let entities = self.entities.get_or_insert_with(Vec::new);
            entities.extend(other_entities.clone());
        }
    }

    /// Get all entities as a reference to the vector
    pub fn get_entities(&self) -> Option<&Vec<EntityDefinition>> {
        self.entities.as_ref()
    }

    /// Get all entities as a mutable reference to the vector
    pub fn get_entities_mut(&mut self) -> Option<&mut Vec<EntityDefinition>> {
        self.entities.as_mut()
    }
}
