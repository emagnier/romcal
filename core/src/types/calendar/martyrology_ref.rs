#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ts-bindings")]
use ts_rs::TS;

use crate::types::MartyrologyEntryOverride;

/// Resource identifier for referencing entries in the martyrology catalog.
pub type ResourceId = String;

/// A reference to an entry in the martyrology catalog.
/// Can either reference an existing entry by ID or define a custom entry with additional properties.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[cfg_attr(feature = "ts-bindings", derive(TS))]
#[cfg_attr(feature = "ts-bindings", ts(export))]
#[serde(untagged)]
pub enum MartyrologyRef {
    /// Reference to an existing martyrology entry by its ID
    ResourceId(ResourceId),
    /// Custom entry definition with additional properties specific to a liturgical day
    Override(MartyrologyEntryOverride),
}
