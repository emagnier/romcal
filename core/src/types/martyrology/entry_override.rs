#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ts-bindings")]
use ts_rs::TS;

use crate::types::SaintCount;
use crate::types::TitlesDef;

/// Custom martyrology entry definition that extends or overrides properties from the martyrology catalog.
/// Used when a liturgical day needs specific entry properties that differ from the base entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[cfg_attr(feature = "ts-bindings", derive(TS))]
#[cfg_attr(feature = "ts-bindings", ts(export))]
pub struct MartyrologyEntryOverride {
    /// The ID of the martyrology entry (must reference an existing entry in the catalog)
    pub id: String,
    /// The custom titles for this entry in the context of this liturgical day
    pub titles: Option<TitlesDef>,
    /// Whether to hide titles when displaying this entry (useful when titles are already included in the entry name)
    pub hide_titles: Option<bool>,
    /// The number of persons this entry represents (useful for groups of martyrs or saints)
    pub count: Option<SaintCount>,
}
