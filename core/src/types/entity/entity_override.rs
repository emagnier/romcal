#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::SaintCount;
use crate::types::TitlesDef;

/// Custom entity definition that extends or overrides properties from the entity catalog.
/// Used when a liturgical day needs specific entity properties that differ from the base entity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
pub struct EntityOverride {
    /// The ID of the entity item (must reference an existing entity in the catalog)
    pub id: String,
    /// The custom titles for this entity in the context of this liturgical day
    pub titles: Option<TitlesDef>,
    /// Whether to hide titles when displaying this entity (useful when titles are already included in the entity name)
    pub hide_titles: Option<bool>,
    /// The number of persons this entity represents (useful for groups of martyrs or saints)
    pub count: Option<SaintCount>,
}
