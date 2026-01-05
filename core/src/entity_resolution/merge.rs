//! Entity merging utilities.
//!
//! This module provides functions for merging entities across locales.

use std::collections::BTreeMap;

use super::locale::build_merge_hierarchy;
use crate::romcal::Romcal;
use crate::types::entity::{Entity, EntityDefinition, EntityId};

/// Merge entities from all locales in the hierarchy.
///
/// The merge order is: en → parent → specific (most specific wins).
/// For example, for locale "fr-FR": en → fr → fr-FR
///
/// Properties from more specific locales override those from more general locales.
pub(crate) fn merge_locale_resources(romcal: &Romcal) -> BTreeMap<EntityId, Entity> {
    let mut merged: BTreeMap<EntityId, Entity> = BTreeMap::new();

    // Build hierarchy: ["en", "fr", "fr-FR"] for locale "fr-FR"
    let hierarchy = build_merge_hierarchy(&romcal.locale);

    // Merge each locale in order (later overrides earlier)
    for locale in hierarchy {
        if let Some(resources) = romcal.get_resources(&locale)
            && let Some(entities) = &resources.entities
        {
            for (id, definition) in entities {
                if let Some(existing) = merged.get_mut(id) {
                    // Merge: more specific locale properties override base
                    merge_entity_from_definition(existing, definition);
                } else {
                    // New entity from this locale
                    merged.insert(id.clone(), Entity::new(id.clone(), definition.clone()));
                }
            }
        }
    }

    merged
}

/// Merges properties from an EntityDefinition into an existing Entity.
/// Source properties override target properties when defined.
pub(crate) fn merge_entity_from_definition(target: &mut Entity, source: &EntityDefinition) {
    if let Some(t) = &source.r#type {
        target.r#type = t.clone();
    }
    if source.fullname.is_some() {
        target.fullname.clone_from(&source.fullname);
    }
    if source.name.is_some() {
        target.name.clone_from(&source.name);
    }
    if source.canonization_level.is_some() {
        target
            .canonization_level
            .clone_from(&source.canonization_level);
    }
    if source.date_of_canonization.is_some() {
        target
            .date_of_canonization
            .clone_from(&source.date_of_canonization);
    }
    if source.date_of_canonization_is_approximative.is_some() {
        target.date_of_canonization_is_approximative = source.date_of_canonization_is_approximative;
    }
    if source.date_of_beatification.is_some() {
        target
            .date_of_beatification
            .clone_from(&source.date_of_beatification);
    }
    if source.date_of_beatification_is_approximative.is_some() {
        target.date_of_beatification_is_approximative =
            source.date_of_beatification_is_approximative;
    }
    if source.hide_canonization_level.is_some() {
        target.hide_canonization_level = source.hide_canonization_level;
    }
    if source.titles.is_some() {
        target.titles.clone_from(&source.titles);
    }
    if source.sex.is_some() {
        target.sex.clone_from(&source.sex);
    }
    if source.hide_titles.is_some() {
        target.hide_titles = source.hide_titles;
    }
    if source.date_of_dedication.is_some() {
        target
            .date_of_dedication
            .clone_from(&source.date_of_dedication);
    }
    if source.date_of_birth.is_some() {
        target.date_of_birth.clone_from(&source.date_of_birth);
    }
    if source.date_of_birth_is_approximative.is_some() {
        target.date_of_birth_is_approximative = source.date_of_birth_is_approximative;
    }
    if source.date_of_death.is_some() {
        target.date_of_death.clone_from(&source.date_of_death);
    }
    if source.date_of_death_is_approximative.is_some() {
        target.date_of_death_is_approximative = source.date_of_death_is_approximative;
    }
    if source.count.is_some() {
        target.count.clone_from(&source.count);
    }
    if source.sources.is_some() {
        target.sources.clone_from(&source.sources);
    }
}
