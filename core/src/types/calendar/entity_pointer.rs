use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::EntityOverride;

// Type alias
pub type ResourceId = String;

/// A pointer to an entity in the entity catalog.
/// Can either reference an existing entity by ID or define a custom entity with additional properties.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum EntityPointer {
    /// Reference to an existing entity by its ID
    ResourceId(ResourceId),
    /// Custom entity definition with additional properties specific to a liturgical day
    Override(EntityOverride),
}
