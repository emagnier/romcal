#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
#[cfg(feature = "ts-bindings")]
use ts_rs::TS;

use crate::types::MartyrologyEntryId;
use crate::types::martyrology::{CanonizationLevel, MartyrologyEntryDef};
use crate::types::resource::ResourcesMetadata;

/// Locale code of the resources, in BCP-47 IETF tag format
pub type LocaleId = String;

/// Resources definition
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[cfg_attr(feature = "ts-bindings", derive(TS))]
#[cfg_attr(feature = "ts-bindings", ts(export))]
pub struct Resources {
    #[serde(rename = "$schema", skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "ts-bindings", ts(rename = "$schema"))]
    pub schema: Option<String>,

    /// Locale code of the resources, in BCP-47 IETF tag format
    pub locale: LocaleId,

    /// Metadata of the resources
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ResourcesMetadata>,

    /// Martyrology entries: saints, blessed, places, events from the Roman Martyrology
    #[serde(skip_serializing_if = "Option::is_none")]
    pub martyrology: Option<BTreeMap<MartyrologyEntryId, MartyrologyEntryDef>>,
}

impl Resources {
    /// Create a new Resources with the given locale
    pub fn new(locale: LocaleId) -> Self {
        Self {
            schema: None,
            locale,
            metadata: None,
            martyrology: None,
        }
    }

    /// Add a martyrology entry definition to the resources
    pub fn add_martyrology_entry(&mut self, id: MartyrologyEntryId, entry: MartyrologyEntryDef) {
        let martyrology = self.martyrology.get_or_insert_with(BTreeMap::new);
        martyrology.insert(id, entry);
    }

    /// Get a martyrology entry definition by its ID
    pub fn get_martyrology_entry(&self, id: &str) -> Option<&MartyrologyEntryDef> {
        self.martyrology.as_ref()?.get(id)
    }

    /// Get a mutable reference to a martyrology entry definition by its ID
    pub fn get_martyrology_entry_mut(&mut self, id: &str) -> Option<&mut MartyrologyEntryDef> {
        self.martyrology.as_mut()?.get_mut(id)
    }

    /// Remove a martyrology entry definition by its ID
    pub fn remove_martyrology_entry(&mut self, id: &str) -> Option<MartyrologyEntryDef> {
        self.martyrology.as_mut()?.remove(id)
    }

    /// Get all martyrology entry IDs
    pub fn get_martyrology_entry_ids(&self) -> Vec<&String> {
        self.martyrology
            .as_ref()
            .map(|martyrology| martyrology.keys().collect())
            .unwrap_or_default()
    }

    /// Validate that all martyrology entries are properly structured
    /// Check for entry structure (uniqueness is guaranteed by BTreeMap)
    pub fn validate_martyrology(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if let Some(martyrology) = &self.martyrology {
            for (id, entry) in martyrology {
                // Validate entry structure
                if entry.name.is_none() && entry.fullname.is_none() {
                    errors.push(format!(
                        "Martyrology entry '{}' must have either 'name' or 'fullname'",
                        id
                    ));
                }

                // Validate canonization level consistency
                if let Some(level) = &entry.canonization_level {
                    if entry.hide_canonization_level == Some(true) && entry.fullname.is_some() {
                        // This is OK - canonization level is hidden because it's in the fullname
                    } else if entry.fullname.is_none() {
                        errors.push(format!(
                            "Martyrology entry '{}' has canonization level '{}' but no fullname to display it",
                            id,
                            match level {
                                CanonizationLevel::Blessed => "blessed",
                                CanonizationLevel::Saint => "saint",
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

    /// Merge martyrology entries from another Resources
    pub fn merge_martyrology(&mut self, other: &Resources) {
        if let Some(other_martyrology) = &other.martyrology {
            let martyrology = self.martyrology.get_or_insert_with(BTreeMap::new);
            martyrology.extend(other_martyrology.clone());
        }
    }

    /// Get all martyrology entry definitions as a reference to the map
    pub fn get_martyrology(&self) -> Option<&BTreeMap<MartyrologyEntryId, MartyrologyEntryDef>> {
        self.martyrology.as_ref()
    }

    /// Get all martyrology entry definitions as a mutable reference to the map
    pub fn get_martyrology_mut(
        &mut self,
    ) -> Option<&mut BTreeMap<MartyrologyEntryId, MartyrologyEntryDef>> {
        self.martyrology.as_mut()
    }
}
