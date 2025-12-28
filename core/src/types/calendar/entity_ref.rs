#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ts-bindings")]
use ts_rs::TS;

use crate::types::EntityOverride;

/// Resource identifier for referencing entities in the catalog.
pub type ResourceId = String;

/// A reference to an entity in the entity catalog.
/// Can either reference an existing entity by ID or define a custom entity with additional properties.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[cfg_attr(feature = "ts-bindings", derive(TS))]
#[cfg_attr(feature = "ts-bindings", ts(export))]
#[serde(untagged)]
pub enum EntityRef {
    /// Reference to an existing entity by its ID
    ResourceId(ResourceId),
    /// Custom entity definition with additional properties specific to a liturgical day
    Override(EntityOverride),
}
