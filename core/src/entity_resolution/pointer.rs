//! Entity pointer resolution utilities.
//!
//! This module handles resolution of EntityRef pointers and title operations.

use std::collections::BTreeMap;

use crate::types::calendar::entity_ref::EntityRef;
use crate::types::entity::title::{Title, TitlesDef};
use crate::types::entity::{Entity, EntityDefinition, EntityId};

/// Resolves an EntityRef to a full Entity.
///
/// For ResourceId: looks up the entity by ID, creates empty entity if not found.
/// For Override: looks up base entity and applies overrides.
pub(crate) fn resolve_entity_pointer(
    entities: &BTreeMap<EntityId, Entity>,
    pointer: &EntityRef,
) -> Entity {
    match pointer {
        EntityRef::ResourceId(id) => {
            // Look up entity by ID, create empty with ID if not found
            entities
                .get(id)
                .cloned()
                .unwrap_or_else(|| create_empty_entity_with_id(id))
        }
        EntityRef::Override(override_def) => {
            // Look up base entity
            let mut entity = entities
                .get(&override_def.id)
                .cloned()
                .unwrap_or_else(|| create_empty_entity_with_id(&override_def.id));

            // Update the ID to match the override
            entity.id.clone_from(&override_def.id);

            // Apply overrides
            if let Some(titles_def) = &override_def.titles {
                entity.titles = Some(apply_titles_def(entity.titles.as_ref(), titles_def));
            }
            if let Some(hide_titles) = override_def.hide_titles {
                entity.hide_titles = Some(hide_titles);
            }
            if let Some(count) = &override_def.count {
                entity.count = Some(count.clone());
            }

            entity
        }
    }
}

/// Creates an empty entity with just an ID (used as fallback when entity not found).
pub(crate) fn create_empty_entity_with_id(id: &str) -> Entity {
    let mut definition = EntityDefinition::new();
    definition.name = Some(id.to_string());
    Entity::new(id.to_string(), definition)
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

/// Combines titles from all entities into a single TitlesDef.
///
/// This function:
/// 1. Collects all titles from each entity (respecting hide_titles)
/// 2. Deduplicates titles
/// 3. Returns TitlesDef::Titles with combined titles
pub(crate) fn combine_titles(entities: &[Entity]) -> TitlesDef {
    let mut combined_titles: Vec<Title> = Vec::new();

    for entity in entities {
        // Skip if hide_titles is true
        if entity.hide_titles == Some(true) {
            continue;
        }

        // Add titles from this entity
        if let Some(titles) = &entity.titles {
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
