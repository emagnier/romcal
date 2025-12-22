#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::types::EntityId;
use crate::types::entity::{CanonizationLevel, Entity};
use crate::types::resource::ResourcesMetadata;

/// Locale code of the resources, in BCP-47 IETF tag format
pub type LocaleId = String;

/// Resources definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
pub struct Resources {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,

    /// Locale code of the resources, in BCP-47 IETF tag format
    pub locale: LocaleId,

    /// Metadata of the resources
    pub metadata: Option<ResourcesMetadata>,

    /// Entities of the resources: a person, a place, an event, etc.
    pub entities: Option<BTreeMap<EntityId, Entity>>,
}

impl Resources {
    /// Create a new Resources with the given locale
    pub fn new(locale: LocaleId) -> Self {
        Self {
            schema: None,
            locale,
            metadata: None,
            entities: None,
        }
    }

    /// Add an entity to the resources
    pub fn add_entity(&mut self, id: EntityId, entity: Entity) {
        let entities = self.entities.get_or_insert_with(BTreeMap::new);
        entities.insert(id, entity);
    }

    /// Get an entity by its ID
    pub fn get_entity(&self, id: &str) -> Option<&Entity> {
        self.entities.as_ref()?.get(id)
    }

    /// Get a mutable reference to an entity by its ID
    pub fn get_entity_mut(&mut self, id: &str) -> Option<&mut Entity> {
        self.entities.as_mut()?.get_mut(id)
    }

    /// Remove an entity by its ID
    pub fn remove_entity(&mut self, id: &str) -> Option<Entity> {
        self.entities.as_mut()?.remove(id)
    }

    /// Get all entity IDs
    pub fn get_entity_ids(&self) -> Vec<&String> {
        self.entities
            .as_ref()
            .map(|entities| entities.keys().collect())
            .unwrap_or_default()
    }

    /// Validate that all entities are properly structured
    /// Check for entity structure (uniqueness is guaranteed by BTreeMap)
    pub fn validate_entities(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if let Some(entities) = &self.entities {
            for (id, entity) in entities {
                // Validate entity structure
                if entity.name.is_none() && entity.fullname.is_none() {
                    errors.push(format!(
                        "Entity '{}' must have either 'name' or 'fullname'",
                        id
                    ));
                }

                // Validate canonization level consistency
                if let Some(level) = &entity.canonization_level {
                    if entity.hide_canonization_level == Some(true) && entity.fullname.is_some() {
                        // This is OK - canonization level is hidden because it's in the fullname
                    } else if entity.fullname.is_none() {
                        errors.push(format!(
                            "Entity '{}' has canonization level '{}' but no fullname to display it",
                            id,
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

    /// Merge entities from another Resources
    pub fn merge_entities(&mut self, other: &Resources) {
        if let Some(other_entities) = &other.entities {
            let entities = self.entities.get_or_insert_with(BTreeMap::new);
            entities.extend(other_entities.clone());
        }
    }

    /// Get all entities as a reference to the map
    pub fn get_entities(&self) -> Option<&BTreeMap<EntityId, Entity>> {
        self.entities.as_ref()
    }

    /// Get all entities as a mutable reference to the map
    pub fn get_entities_mut(&mut self) -> Option<&mut BTreeMap<EntityId, Entity>> {
        self.entities.as_mut()
    }
}
