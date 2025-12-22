//! # Entity Resolver Module
//!
//! This module provides functionality for resolving entities from resources
//! for liturgical days. It handles locale-based resource merging and
//! entity pointer resolution.

use std::collections::BTreeMap;

use crate::romcal::Romcal;
use crate::types::calendar::day_definition::DayDefinition;
use crate::types::calendar::entity_ref::EntityRef;
use crate::types::entity::entity_definition::{Entity, EntityId};
use crate::types::entity::title::{Title, TitlesDef};

/// Resolver for entities in liturgical days.
///
/// This struct is responsible for:
/// - Merging locale resources (base 'en' + target locale)
/// - Resolving entity pointers to full Entity objects
/// - Combining titles from multiple entities
pub struct EntityResolver {
    /// Merged entities from all locale resources (en + target locale)
    entities: BTreeMap<EntityId, Entity>,
    /// The target locale
    locale: String,
}

impl EntityResolver {
    /// Creates a new EntityResolver from a Romcal instance.
    ///
    /// This constructor merges 'en' (default locale) with the target locale
    /// specified in the romcal config. Properties from the target locale override
    /// those from 'en'.
    ///
    /// # Arguments
    ///
    /// * `romcal` - The romcal instance containing resources and locale configuration
    pub fn new(romcal: &Romcal) -> Self {
        let locale = romcal.locale.clone();
        let entities = Self::merge_locale_resources(romcal);

        Self { entities, locale }
    }

    /// Returns the target locale
    pub fn locale(&self) -> &str {
        &self.locale
    }

    /// Merges resources from 'en' locale with the target locale.
    ///
    /// The merge strategy is:
    /// 1. Start with all entities from 'en'
    /// 2. For each entity in the target locale:
    ///    - If exists in 'en': merge properties (target overrides 'en')
    ///    - If doesn't exist: add as new entity
    fn merge_locale_resources(romcal: &Romcal) -> BTreeMap<EntityId, Entity> {
        let mut merged_entities: BTreeMap<EntityId, Entity> = BTreeMap::new();

        // Step 1: Load all entities from 'en' (default locale)
        if let Some(en_resources) = romcal.get_resources("en")
            && let Some(en_entities) = &en_resources.entities
        {
            for (id, mut entity) in en_entities.clone() {
                // Assign the ID
                entity.id = Some(id.clone());
                // Ensure type is set to Person if not defined
                if entity.r#type.is_none() {
                    use crate::types::entity::EntityType;
                    entity.r#type = Some(EntityType::Person);
                }
                merged_entities.insert(id, entity);
            }
        }

        // Step 2: If target locale is not 'en', merge with target locale
        if romcal.locale != "en" {
            if let Some(target_resources) = romcal.get_resources(&romcal.locale)
                && let Some(target_entities) = &target_resources.entities
            {
                for (id, target_entity) in target_entities {
                    if let Some(base_entity) = merged_entities.get_mut(id) {
                        // Merge: target properties override base
                        Self::merge_entity(base_entity, target_entity);
                    } else {
                        // New entity from target locale
                        let mut entity = target_entity.clone();
                        // Assign the ID
                        entity.id = Some(id.clone());
                        // Ensure type is set to Person if not defined
                        if entity.r#type.is_none() {
                            use crate::types::entity::EntityType;
                            entity.r#type = Some(EntityType::Person);
                        }
                        merged_entities.insert(id.clone(), entity);
                    }
                }
            }

            // Also check for locale variants (e.g., 'fr-fr' inherits from 'fr')
            Self::merge_locale_hierarchy(romcal, &mut merged_entities);
        }

        // Ensure all entities have IDs and types set
        for (id, entity) in merged_entities.iter_mut() {
            if entity.id.is_none() {
                entity.id = Some(id.clone());
            }
            if entity.r#type.is_none() {
                use crate::types::entity::EntityType;
                entity.r#type = Some(EntityType::Person);
            }
        }

        merged_entities
    }

    /// Merges entities from locale hierarchy (e.g., 'fr-fr' inherits from 'fr')
    fn merge_locale_hierarchy(romcal: &Romcal, merged_entities: &mut BTreeMap<EntityId, Entity>) {
        let locale = &romcal.locale;

        // Check if locale has a parent (e.g., 'fr-fr' -> 'fr')
        if let Some(hyphen_pos) = locale.find('-') {
            let parent_locale = &locale[..hyphen_pos];

            // Don't process if parent is 'en' (already handled)
            if parent_locale != "en"
                && let Some(parent_resources) = romcal.get_resources(parent_locale)
                && let Some(parent_entities) = &parent_resources.entities
            {
                for (id, parent_entity) in parent_entities {
                    if let Some(base_entity) = merged_entities.get_mut(id) {
                        Self::merge_entity(base_entity, parent_entity);
                    } else {
                        let mut entity = parent_entity.clone();
                        // Assign the ID
                        entity.id = Some(id.clone());
                        // Ensure type is set to Person if not defined
                        if entity.r#type.is_none() {
                            use crate::types::entity::EntityType;
                            entity.r#type = Some(EntityType::Person);
                        }
                        merged_entities.insert(id.clone(), entity);
                    }
                }
            }
        }
    }

    /// Merges properties from source entity into target entity.
    /// Source properties override target properties when defined.
    fn merge_entity(target: &mut Entity, source: &Entity) {
        if source.r#type.is_some() {
            target.r#type = source.r#type.clone();
        }
        if source.fullname.is_some() {
            target.fullname = source.fullname.clone();
        }
        if source.name.is_some() {
            target.name = source.name.clone();
        }
        if source.canonization_level.is_some() {
            target.canonization_level = source.canonization_level.clone();
        }
        if source.date_of_canonization.is_some() {
            target.date_of_canonization = source.date_of_canonization.clone();
        }
        if source.date_of_canonization_is_approximative.is_some() {
            target.date_of_canonization_is_approximative =
                source.date_of_canonization_is_approximative;
        }
        if source.date_of_beatification.is_some() {
            target.date_of_beatification = source.date_of_beatification.clone();
        }
        if source.date_of_beatification_is_approximative.is_some() {
            target.date_of_beatification_is_approximative =
                source.date_of_beatification_is_approximative;
        }
        if source.hide_canonization_level.is_some() {
            target.hide_canonization_level = source.hide_canonization_level;
        }
        if source.titles.is_some() {
            target.titles = source.titles.clone();
        }
        if source.sex.is_some() {
            target.sex = source.sex.clone();
        }
        if source.hide_titles.is_some() {
            target.hide_titles = source.hide_titles;
        }
        if source.date_of_dedication.is_some() {
            target.date_of_dedication = source.date_of_dedication.clone();
        }
        if source.date_of_birth.is_some() {
            target.date_of_birth = source.date_of_birth.clone();
        }
        if source.date_of_birth_is_approximative.is_some() {
            target.date_of_birth_is_approximative = source.date_of_birth_is_approximative;
        }
        if source.date_of_death.is_some() {
            target.date_of_death = source.date_of_death.clone();
        }
        if source.date_of_death_is_approximative.is_some() {
            target.date_of_death_is_approximative = source.date_of_death_is_approximative;
        }
        if source.count.is_some() {
            target.count = source.count.clone();
        }
        if source.sources.is_some() {
            target.sources = source.sources.clone();
        }

        // Ensure type is set to Person if not defined after merge
        if target.r#type.is_none() {
            use crate::types::entity::EntityType;
            target.r#type = Some(EntityType::Person);
        }
    }

    /// Resolves an entity by its ID.
    ///
    /// Returns the entity if found, or None if not found.
    pub fn resolve_entity(&self, id: &str) -> Option<&Entity> {
        self.entities.get(id)
    }

    /// Resolves an EntityRef to a full Entity.
    ///
    /// For ResourceId: looks up the entity by ID, creates empty entity if not found.
    /// For Override: looks up base entity and applies overrides.
    pub fn resolve_entity_pointer(&self, pointer: &EntityRef) -> Entity {
        match pointer {
            EntityRef::ResourceId(id) => {
                // Look up entity by ID, create empty with ID if not found
                let mut entity = self
                    .entities
                    .get(id)
                    .cloned()
                    .unwrap_or_else(|| Self::create_empty_entity_with_id(id));

                // Assign the ID
                entity.id = Some(id.clone());

                // Ensure type is set to Person if not defined
                if entity.r#type.is_none() {
                    use crate::types::entity::EntityType;
                    entity.r#type = Some(EntityType::Person);
                }

                entity
            }
            EntityRef::Override(override_def) => {
                // Look up base entity
                let mut entity = self
                    .entities
                    .get(&override_def.id)
                    .cloned()
                    .unwrap_or_else(|| Self::create_empty_entity_with_id(&override_def.id));

                // Assign the ID
                entity.id = Some(override_def.id.clone());

                // Apply overrides
                if let Some(titles_def) = &override_def.titles {
                    entity.titles =
                        Some(Self::apply_titles_def(entity.titles.as_ref(), titles_def));
                }
                if let Some(hide_titles) = override_def.hide_titles {
                    entity.hide_titles = Some(hide_titles);
                }
                if let Some(count) = &override_def.count {
                    entity.count = Some(count.clone());
                }

                // Ensure type is set to Person if not defined
                if entity.r#type.is_none() {
                    use crate::types::entity::EntityType;
                    entity.r#type = Some(EntityType::Person);
                }

                entity
            }
        }
    }

    /// Creates an empty entity with just an ID (used as fallback when entity not found)
    fn create_empty_entity_with_id(id: &str) -> Entity {
        let mut entity = Entity::new();
        entity.name = Some(id.to_string());
        entity
    }

    /// Applies a TitlesDef to existing titles.
    ///
    /// For simple list: replaces existing titles.
    /// For CompoundTitle: applies prepend/append operations.
    fn apply_titles_def(existing: Option<&Vec<Title>>, titles_def: &TitlesDef) -> Vec<Title> {
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

    /// Resolves all entities for a day definition.
    ///
    /// Resolution strategy:
    /// 1. If day_def.entities is defined: resolve each EntityRef
    /// 2. Otherwise (fallback): look for entity with id == day_id
    ///    - If found: return that entity
    ///    - If not found: return empty Vec
    pub fn resolve_entities_for_day(&self, day_def: &DayDefinition, day_id: &str) -> Vec<Entity> {
        if let Some(entity_pointers) = &day_def.entities {
            // Resolve each entity pointer
            entity_pointers
                .iter()
                .map(|pointer| self.resolve_entity_pointer(pointer))
                .collect()
        } else {
            // Fallback: try to find entity with same ID as day_id
            if let Some(entity) = self.entities.get(day_id) {
                let mut entity = entity.clone();
                // Assign the ID
                entity.id = Some(day_id.to_string());
                // Ensure type is set to Person if not defined
                if entity.r#type.is_none() {
                    use crate::types::entity::EntityType;
                    entity.r#type = Some(EntityType::Person);
                }
                vec![entity]
            } else {
                Vec::new()
            }
        }
    }

    /// Gets the fullname for a liturgical day.
    ///
    /// If custom_locale_id is provided, uses that ID for lookup, otherwise uses day_id.
    /// Returns the fullname from the entity if found, None otherwise.
    pub fn get_fullname_for_day(
        &self,
        day_id: &str,
        custom_locale_id: Option<&str>,
    ) -> Option<String> {
        let lookup_id = custom_locale_id.unwrap_or(day_id);
        self.entities
            .get(lookup_id)
            .and_then(|e| e.fullname.clone())
    }

    /// Combines titles from all entities into a single TitlesDef.
    ///
    /// This function:
    /// 1. Collects all titles from each entity (respecting hide_titles)
    /// 2. Deduplicates titles
    /// 3. Returns TitlesDef::Titles with combined titles
    pub fn combine_titles(&self, entities: &[Entity]) -> TitlesDef {
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

    /// Gets all merged entities (for debugging/testing)
    pub fn get_all_entities(&self) -> &BTreeMap<EntityId, Entity> {
        &self.entities
    }

    /// Checks if an entity exists by ID
    pub fn has_entity(&self, id: &str) -> bool {
        self.entities.contains_key(id)
    }

    /// Gets the count of merged entities
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::resources::Resources;
    use crate::romcal::Preset;
    use crate::types::entity::entity_override::EntityOverride;
    use crate::types::entity::title::CompoundTitle;

    fn create_test_entity(name: &str, titles: Vec<Title>) -> Entity {
        let mut entity = Entity::new();
        entity.name = Some(name.to_string());
        entity.titles = Some(titles);
        entity
    }

    fn create_test_resources(locale: &str, entities: Vec<(&str, Entity)>) -> Resources {
        let mut resources = Resources::new(locale.to_string());
        for (id, entity) in entities {
            resources.add_entity(id.to_string(), entity);
        }
        resources
    }

    #[test]
    fn test_entity_resolver_creation() {
        let romcal = Romcal::default();
        let resolver = EntityResolver::new(&romcal);

        assert_eq!(resolver.locale(), "en");
    }

    #[test]
    fn test_resolve_entity_pointer_resource_id() {
        let mut romcal = Romcal::default();

        // Add test entity
        let entity = create_test_entity("Test Saint", vec![Title::Martyr]);
        let resources = create_test_resources("en", vec![("test_saint", entity)]);
        romcal.add_resources(resources);

        let resolver = EntityResolver::new(&romcal);

        // Test resolving by ResourceId
        let pointer = EntityRef::ResourceId("test_saint".to_string());
        let resolved = resolver.resolve_entity_pointer(&pointer);

        assert_eq!(resolved.name, Some("Test Saint".to_string()));
        assert_eq!(resolved.titles, Some(vec![Title::Martyr]));
    }

    #[test]
    fn test_resolve_entity_pointer_not_found() {
        let romcal = Romcal::default();
        let resolver = EntityResolver::new(&romcal);

        // Test resolving non-existent entity
        let pointer = EntityRef::ResourceId("non_existent".to_string());
        let resolved = resolver.resolve_entity_pointer(&pointer);

        // Should create empty entity with ID as name
        assert_eq!(resolved.name, Some("non_existent".to_string()));
    }

    #[test]
    fn test_resolve_entity_pointer_override() {
        let mut romcal = Romcal::default();

        // Add base entity
        let entity = create_test_entity("Test Saint", vec![Title::Martyr]);
        let resources = create_test_resources("en", vec![("test_saint", entity)]);
        romcal.add_resources(resources);

        let resolver = EntityResolver::new(&romcal);

        // Test resolving with override
        let pointer = EntityRef::Override(EntityOverride {
            id: "test_saint".to_string(),
            titles: Some(TitlesDef::Titles(vec![Title::Bishop, Title::Martyr])),
            hide_titles: Some(false),
            count: None,
        });
        let resolved = resolver.resolve_entity_pointer(&pointer);

        assert_eq!(resolved.name, Some("Test Saint".to_string()));
        assert_eq!(resolved.titles, Some(vec![Title::Bishop, Title::Martyr]));
        assert_eq!(resolved.hide_titles, Some(false));
    }

    #[test]
    fn test_resolve_entity_pointer_compound_titles() {
        let mut romcal = Romcal::default();

        // Add base entity
        let entity = create_test_entity("Test Saint", vec![Title::Martyr]);
        let resources = create_test_resources("en", vec![("test_saint", entity)]);
        romcal.add_resources(resources);

        let resolver = EntityResolver::new(&romcal);

        // Test with compound title
        let pointer = EntityRef::Override(EntityOverride {
            id: "test_saint".to_string(),
            titles: Some(TitlesDef::CompoundTitle(CompoundTitle {
                prepend: Some(vec![Title::Bishop]),
                append: Some(vec![Title::DoctorOfTheChurch]),
            })),
            hide_titles: None,
            count: None,
        });
        let resolved = resolver.resolve_entity_pointer(&pointer);

        // Should be: [Bishop, Martyr (from base), DoctorOfTheChurch]
        assert_eq!(
            resolved.titles,
            Some(vec![Title::Bishop, Title::Martyr, Title::DoctorOfTheChurch])
        );
    }

    #[test]
    fn test_resolve_entities_for_day_with_pointers() {
        let mut romcal = Romcal::default();

        // Add test entities
        let entity1 = create_test_entity("Saint Peter", vec![Title::Apostle]);
        let entity2 = create_test_entity("Saint Paul", vec![Title::Apostle, Title::Martyr]);
        let resources = create_test_resources(
            "en",
            vec![("peter_apostle", entity1), ("paul_apostle", entity2)],
        );
        romcal.add_resources(resources);

        let resolver = EntityResolver::new(&romcal);

        // Create day definition with entities
        let day_def = DayDefinition {
            date_def: None,
            date_exceptions: None,
            precedence: None,
            commons_def: None,
            is_holy_day_of_obligation: None,
            allow_similar_rank_items: None,
            is_optional: None,
            custom_locale_id: None,
            entities: Some(vec![
                EntityRef::ResourceId("peter_apostle".to_string()),
                EntityRef::ResourceId("paul_apostle".to_string()),
            ]),
            titles: None,
            drop: None,
            colors: None,
            masses: None,
        };

        let entities = resolver.resolve_entities_for_day(&day_def, "test_day");

        assert_eq!(entities.len(), 2);
        assert_eq!(entities[0].name, Some("Saint Peter".to_string()));
        assert_eq!(entities[1].name, Some("Saint Paul".to_string()));
    }

    #[test]
    fn test_resolve_entities_for_day_fallback() {
        let mut romcal = Romcal::default();

        // Add entity with same ID as day_id
        let entity = create_test_entity("Test Saint", vec![Title::Martyr]);
        let resources = create_test_resources("en", vec![("test_day_id", entity)]);
        romcal.add_resources(resources);

        let resolver = EntityResolver::new(&romcal);

        // Create day definition without entities (should fallback to day_id)
        let day_def = DayDefinition {
            date_def: None,
            date_exceptions: None,
            precedence: None,
            commons_def: None,
            is_holy_day_of_obligation: None,
            allow_similar_rank_items: None,
            is_optional: None,
            custom_locale_id: None,
            entities: None,
            titles: None,
            drop: None,
            colors: None,
            masses: None,
        };

        let entities = resolver.resolve_entities_for_day(&day_def, "test_day_id");

        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].name, Some("Test Saint".to_string()));
    }

    #[test]
    fn test_resolve_entities_for_day_no_fallback() {
        let romcal = Romcal::default();
        let resolver = EntityResolver::new(&romcal);

        // Create day definition without entities and no matching entity
        let day_def = DayDefinition {
            date_def: None,
            date_exceptions: None,
            precedence: None,
            commons_def: None,
            is_holy_day_of_obligation: None,
            allow_similar_rank_items: None,
            is_optional: None,
            custom_locale_id: None,
            entities: None,
            titles: None,
            drop: None,
            colors: None,
            masses: None,
        };

        let entities = resolver.resolve_entities_for_day(&day_def, "non_existent_day");

        assert!(entities.is_empty());
    }

    #[test]
    fn test_combine_titles() {
        let romcal = Romcal::default();
        let resolver = EntityResolver::new(&romcal);

        let entities = vec![
            create_test_entity("Saint 1", vec![Title::Martyr, Title::Bishop]),
            create_test_entity("Saint 2", vec![Title::Apostle, Title::Martyr]), // Martyr is duplicate
        ];

        let combined = resolver.combine_titles(&entities);

        match combined {
            TitlesDef::Titles(titles) => {
                // Should have unique titles: Martyr, Bishop, Apostle
                assert_eq!(titles.len(), 3);
                assert!(titles.contains(&Title::Martyr));
                assert!(titles.contains(&Title::Bishop));
                assert!(titles.contains(&Title::Apostle));
            }
            _ => panic!("Expected TitlesDef::Titles"),
        }
    }

    #[test]
    fn test_combine_titles_respects_hide_titles() {
        let romcal = Romcal::default();
        let resolver = EntityResolver::new(&romcal);

        let mut hidden_entity = create_test_entity("Hidden", vec![Title::Pope]);
        hidden_entity.hide_titles = Some(true);

        let entities = vec![
            create_test_entity("Visible", vec![Title::Martyr]),
            hidden_entity,
        ];

        let combined = resolver.combine_titles(&entities);

        match combined {
            TitlesDef::Titles(titles) => {
                // Should only have Martyr (Pope is hidden)
                assert_eq!(titles.len(), 1);
                assert!(titles.contains(&Title::Martyr));
                assert!(!titles.contains(&Title::Pope));
            }
            _ => panic!("Expected TitlesDef::Titles"),
        }
    }

    #[test]
    fn test_locale_merge() {
        let mut romcal = Romcal::new(Preset {
            locale: Some("fr".to_string()),
            ..Preset::default()
        });

        // Add English entity
        let en_entity = create_test_entity("Test Saint (EN)", vec![Title::Martyr]);
        let en_resources = create_test_resources("en", vec![("test_saint", en_entity)]);
        romcal.add_resources(en_resources);

        // Add French entity with translated name
        let mut fr_entity = Entity::new();
        fr_entity.name = Some("Saint Test (FR)".to_string());
        // Note: titles not set in FR, should inherit from EN
        let fr_resources = create_test_resources("fr", vec![("test_saint", fr_entity)]);
        romcal.add_resources(fr_resources);

        let resolver = EntityResolver::new(&romcal);

        let entity = resolver.resolve_entity("test_saint").unwrap();

        // Name should be overridden by FR
        assert_eq!(entity.name, Some("Saint Test (FR)".to_string()));
        // Titles should be inherited from EN
        assert_eq!(entity.titles, Some(vec![Title::Martyr]));
    }
}
