//! Martyrology pointer resolution utilities.
//!
//! This module handles resolution of MartyrologyRef pointers and title operations.

use std::collections::BTreeMap;

use crate::error::{RomcalError, RomcalResult};
use crate::types::calendar::martyrology_ref::MartyrologyRef;
use crate::types::martyrology::title::{Title, TitlesDef};
use crate::types::martyrology::{MartyrologyEntry, MartyrologyEntryId};

/// Resolves a MartyrologyRef to a full MartyrologyEntry.
///
/// For ResourceId: looks up the entry by ID, returns error if not found.
/// For Override: looks up base entry and applies overrides.
///
/// # Errors
///
/// Returns `RomcalError::MartyrologyEntryNotFound` if the entry ID is not found
/// in the merged entries map.
pub(crate) fn resolve_martyrology_pointer(
    entries: &BTreeMap<MartyrologyEntryId, MartyrologyEntry>,
    pointer: &MartyrologyRef,
    locale_hierarchy: &[String],
) -> RomcalResult<MartyrologyEntry> {
    match pointer {
        MartyrologyRef::ResourceId(id) => {
            // Look up entry by ID, return error if not found
            entries.get(id).cloned().ok_or_else(|| {
                RomcalError::MartyrologyEntryNotFound(id.clone(), locale_hierarchy.to_vec())
            })
        }
        MartyrologyRef::Override(override_def) => {
            // Look up base entry, return error if not found
            let mut entry = entries.get(&override_def.id).cloned().ok_or_else(|| {
                RomcalError::MartyrologyEntryNotFound(
                    override_def.id.clone(),
                    locale_hierarchy.to_vec(),
                )
            })?;

            // Update the ID to match the override
            entry.id.clone_from(&override_def.id);

            // Apply overrides
            if let Some(titles_def) = &override_def.titles {
                entry.titles = Some(apply_titles_def(entry.titles.as_ref(), titles_def));
            }
            if let Some(hide_titles) = override_def.hide_titles {
                entry.hide_titles = Some(hide_titles);
            }
            if let Some(count) = &override_def.count {
                entry.count = Some(count.clone());
            }

            Ok(entry)
        }
    }
}

/// Applies a TitlesDef to existing titles.
///
/// For simple list: replaces existing titles.
/// For CompoundTitle: applies prepend/append operations.
pub(crate) fn apply_titles_def(
    existing: Option<&Vec<Title>>,
    titles_def: &TitlesDef,
) -> Vec<Title> {
    match titles_def {
        TitlesDef::Titles(titles) => titles.clone(),
        TitlesDef::CompoundTitle(compound) => {
            let mut result = Vec::new();

            // Apply prepend
            if let Some(prepend) = &compound.prepend {
                result.extend(prepend.clone());
            }

            // Add existing titles
            if let Some(existing_titles) = existing {
                result.extend(existing_titles.clone());
            }

            // Apply append
            if let Some(append) = &compound.append {
                result.extend(append.clone());
            }

            result
        }
    }
}

/// Combines titles from all entries into a single TitlesDef.
///
/// This function:
/// 1. Collects all titles from each entry (respecting hide_titles)
/// 2. Deduplicates titles
/// 3. Returns TitlesDef::Titles with combined titles
pub(crate) fn combine_titles(entries: &[MartyrologyEntry]) -> TitlesDef {
    let mut combined_titles: Vec<Title> = Vec::new();

    for entry in entries {
        // Skip if hide_titles is true
        if entry.hide_titles == Some(true) {
            continue;
        }

        // Add titles from this entry
        if let Some(titles) = &entry.titles {
            for title in titles {
                // Deduplicate
                if !combined_titles.contains(title) {
                    combined_titles.push(title.clone());
                }
            }
        }
    }

    TitlesDef::Titles(combined_titles)
}
