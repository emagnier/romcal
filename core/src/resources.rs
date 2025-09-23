use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::entity::{CanonizationLevel, Entity};
use crate::types::resource::ResourcesMetadata;

// Type aliases
pub type LocaleId = String;

/// Resources definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Resources {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,

    /// Locale code of the resources, in BCP-47 IETF tag format
    pub locale: LocaleId,

    /// Metadata of the resources
    pub metadata: Option<ResourcesMetadata>,

    /// Entities of the resources: a person, a place, an event, etc.
    pub entities: Option<Vec<Entity>>,
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
    pub fn add_entity(&mut self, entity: Entity) {
        let entities = self.entities.get_or_insert_with(Vec::new);
        entities.push(entity);
    }

    /// Get an entity by its ID
    pub fn get_entity(&self, id: &str) -> Option<&Entity> {
        self.entities
            .as_ref()?
            .iter()
            .find(|entity| entity.id == id)
    }

    /// Get a mutable reference to an entity by its ID
    pub fn get_entity_mut(&mut self, id: &str) -> Option<&mut Entity> {
        self.entities
            .as_mut()?
            .iter_mut()
            .find(|entity| entity.id == id)
    }

    /// Remove an entity by its ID
    pub fn remove_entity(&mut self, id: &str) -> Option<Entity> {
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

    /// Merge entities from another Resources
    pub fn merge_entities(&mut self, other: &Resources) {
        if let Some(other_entities) = &other.entities {
            let entities = self.entities.get_or_insert_with(Vec::new);
            entities.extend(other_entities.clone());
        }
    }

    /// Get all entities as a reference to the vector
    pub fn get_entities(&self) -> Option<&Vec<Entity>> {
        self.entities.as_ref()
    }

    /// Get all entities as a mutable reference to the vector
    pub fn get_entities_mut(&mut self) -> Option<&mut Vec<Entity>> {
        self.entities.as_mut()
    }
}
