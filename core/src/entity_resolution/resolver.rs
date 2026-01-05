//! Entity resolver implementation.
//!
//! This module provides the main EntityResolver struct for resolving entities
//! with locale fallback.

use std::collections::BTreeMap;

use super::merge::merge_locale_resources;
use super::pointer::{combine_titles, resolve_entity_pointer};
use crate::romcal::Romcal;
use crate::types::calendar::day_definition::DayDefinition;
use crate::types::entity::title::TitlesDef;
use crate::types::entity::{Entity, EntityId};

/// Resolver for entities in liturgical days.
///
/// This struct is responsible for:
/// - Merging locale resources (base 'en' + parent locales + target locale)
/// - Resolving entity pointers to full Entity objects
/// - Combining titles from multiple entities
pub struct EntityResolver {
    /// Merged entities from all locale resources
    entities: BTreeMap<EntityId, Entity>,
    /// The target locale
    locale: String,
}

impl EntityResolver {
    /// Creates a new EntityResolver from a Romcal instance.
    ///
    /// This constructor merges locales in the correct order:
    /// 1. 'en' (default locale)
    /// 2. Parent locales (e.g., 'fr' for 'fr-FR')
    /// 3. Target locale (most specific)
    ///
    /// Properties from more specific locales override those from more general locales.
    ///
    /// # Arguments
    ///
    /// * `romcal` - The romcal instance containing resources and locale configuration
    pub fn new(romcal: &Romcal) -> Self {
        let locale = romcal.locale.clone();
        let entities = merge_locale_resources(romcal);

        Self { entities, locale }
    }

    /// Returns the target locale
    pub fn locale(&self) -> &str {
        &self.locale
    }

    /// Resolves an entity by its ID.
    ///
    /// Returns the entity if found, or None if not found.
    pub fn resolve_entity(&self, id: &str) -> Option<&Entity> {
        self.entities.get(id)
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
                .map(|pointer| resolve_entity_pointer(&self.entities, pointer))
                .collect()
        } else {
            // Fallback: try to find entity with same ID as day_id
            if let Some(entity) = self.entities.get(day_id) {
                vec![entity.clone()]
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
        combine_titles(entities)
    }

    /// Gets all merged entities (for searching/iteration)
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
    use crate::types::calendar::EntityRef;
    use crate::types::entity::EntityDefinition;
    use crate::types::entity::entity_override::EntityOverride;
    use crate::types::entity::title::{CompoundTitle, Title};

    fn create_test_entity_def(name: &str, titles: Vec<Title>) -> EntityDefinition {
        let mut definition = EntityDefinition::new();
        definition.name = Some(name.to_string());
        definition.titles = Some(titles);
        definition
    }

    fn create_test_resources(locale: &str, entities: Vec<(&str, EntityDefinition)>) -> Resources {
        let mut resources = Resources::new(locale.to_string());
        for (id, definition) in entities {
            resources.add_entity(id.to_string(), definition);
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
        let definition = create_test_entity_def("Test Saint", vec![Title::Martyr]);
        let resources = create_test_resources("en", vec![("test_saint", definition)]);
        romcal.add_resources(resources);

        let resolver = EntityResolver::new(&romcal);

        // Test resolving by ID
        let entity = resolver.resolve_entity("test_saint").unwrap();

        assert_eq!(entity.name, Some("Test Saint".to_string()));
        assert_eq!(entity.titles, Some(vec![Title::Martyr]));
    }

    #[test]
    fn test_locale_merge_order() {
        let mut romcal = Romcal::new(Preset {
            locale: Some("fr-FR".to_string()),
            ..Preset::default()
        });

        // Add English entity (base)
        let en_def = create_test_entity_def("Test Saint (EN)", vec![Title::Martyr]);
        let en_resources = create_test_resources("en", vec![("test_saint", en_def)]);
        romcal.add_resources(en_resources);

        // Add French entity (parent) - should override EN
        let mut fr_def = EntityDefinition::new();
        fr_def.name = Some("Saint Test (FR)".to_string());
        let fr_resources = create_test_resources("fr", vec![("test_saint", fr_def)]);
        romcal.add_resources(fr_resources);

        // Add French-France entity (specific) - should override FR
        let mut fr_fr_def = EntityDefinition::new();
        fr_fr_def.fullname = Some("Saint Test de France".to_string());
        let fr_fr_resources = create_test_resources("fr-FR", vec![("test_saint", fr_fr_def)]);
        romcal.add_resources(fr_fr_resources);

        let resolver = EntityResolver::new(&romcal);
        let entity = resolver.resolve_entity("test_saint").unwrap();

        // Name should be from fr (parent), not en
        assert_eq!(entity.name, Some("Saint Test (FR)".to_string()));
        // Fullname should be from fr-FR (most specific)
        assert_eq!(entity.fullname, Some("Saint Test de France".to_string()));
        // Titles should be from en (base, not overridden)
        assert_eq!(entity.titles, Some(vec![Title::Martyr]));
    }

    #[test]
    fn test_locale_merge_specific_overrides_parent() {
        let mut romcal = Romcal::new(Preset {
            locale: Some("fr-FR".to_string()),
            ..Preset::default()
        });

        // Add French entity (parent)
        let mut fr_def = EntityDefinition::new();
        fr_def.name = Some("Nom FR".to_string());
        let fr_resources = create_test_resources("fr", vec![("test_saint", fr_def)]);
        romcal.add_resources(fr_resources);

        // Add French-France entity (specific) - should override FR name
        let mut fr_fr_def = EntityDefinition::new();
        fr_fr_def.name = Some("Nom FR-FR".to_string());
        let fr_fr_resources = create_test_resources("fr-FR", vec![("test_saint", fr_fr_def)]);
        romcal.add_resources(fr_fr_resources);

        let resolver = EntityResolver::new(&romcal);
        let entity = resolver.resolve_entity("test_saint").unwrap();

        // Name should be from fr-FR (specific), NOT fr (parent)
        assert_eq!(entity.name, Some("Nom FR-FR".to_string()));
    }

    #[test]
    fn test_resolve_entities_for_day_with_pointers() {
        let mut romcal = Romcal::default();

        // Add test entities
        let definition1 = create_test_entity_def("Saint Peter", vec![Title::Apostle]);
        let definition2 = create_test_entity_def("Saint Paul", vec![Title::Apostle, Title::Martyr]);
        let resources = create_test_resources(
            "en",
            vec![
                ("peter_apostle", definition1),
                ("paul_apostle", definition2),
            ],
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
        let definition = create_test_entity_def("Test Saint", vec![Title::Martyr]);
        let resources = create_test_resources("en", vec![("test_day_id", definition)]);
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
    fn test_resolve_entity_pointer_override() {
        let mut romcal = Romcal::default();

        // Add base entity
        let definition = create_test_entity_def("Test Saint", vec![Title::Martyr]);
        let resources = create_test_resources("en", vec![("test_saint", definition)]);
        romcal.add_resources(resources);

        let resolver = EntityResolver::new(&romcal);

        // Create day definition with override
        let day_def = DayDefinition {
            date_def: None,
            date_exceptions: None,
            precedence: None,
            commons_def: None,
            is_holy_day_of_obligation: None,
            allow_similar_rank_items: None,
            is_optional: None,
            custom_locale_id: None,
            entities: Some(vec![EntityRef::Override(EntityOverride {
                id: "test_saint".to_string(),
                titles: Some(TitlesDef::Titles(vec![Title::Bishop, Title::Martyr])),
                hide_titles: Some(false),
                count: None,
            })]),
            titles: None,
            drop: None,
            colors: None,
            masses: None,
        };

        let entities = resolver.resolve_entities_for_day(&day_def, "test_day");

        assert_eq!(entities.len(), 1);
        assert_eq!(entities[0].name, Some("Test Saint".to_string()));
        assert_eq!(entities[0].titles, Some(vec![Title::Bishop, Title::Martyr]));
        assert_eq!(entities[0].hide_titles, Some(false));
    }

    #[test]
    fn test_combine_titles() {
        let romcal = Romcal::default();
        let resolver = EntityResolver::new(&romcal);

        fn create_entity_with_titles(id: &str, titles: Vec<Title>) -> Entity {
            let mut definition = EntityDefinition::new();
            definition.titles = Some(titles);
            Entity::new(id.to_string(), definition)
        }

        let entities = vec![
            create_entity_with_titles("saint_1", vec![Title::Martyr, Title::Bishop]),
            create_entity_with_titles("saint_2", vec![Title::Apostle, Title::Martyr]), // Martyr is duplicate
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

        fn create_entity(id: &str, titles: Vec<Title>, hide: bool) -> Entity {
            let mut definition = EntityDefinition::new();
            definition.titles = Some(titles);
            let mut entity = Entity::new(id.to_string(), definition);
            entity.hide_titles = Some(hide);
            entity
        }

        let entities = vec![
            create_entity("visible", vec![Title::Martyr], false),
            create_entity("hidden", vec![Title::Pope], true),
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
    fn test_compound_titles() {
        let mut romcal = Romcal::default();

        // Add base entity
        let definition = create_test_entity_def("Test Saint", vec![Title::Martyr]);
        let resources = create_test_resources("en", vec![("test_saint", definition)]);
        romcal.add_resources(resources);

        let resolver = EntityResolver::new(&romcal);

        // Create day definition with compound title override
        let day_def = DayDefinition {
            date_def: None,
            date_exceptions: None,
            precedence: None,
            commons_def: None,
            is_holy_day_of_obligation: None,
            allow_similar_rank_items: None,
            is_optional: None,
            custom_locale_id: None,
            entities: Some(vec![EntityRef::Override(EntityOverride {
                id: "test_saint".to_string(),
                titles: Some(TitlesDef::CompoundTitle(CompoundTitle {
                    prepend: Some(vec![Title::Bishop]),
                    append: Some(vec![Title::DoctorOfTheChurch]),
                })),
                hide_titles: None,
                count: None,
            })]),
            titles: None,
            drop: None,
            colors: None,
            masses: None,
        };

        let entities = resolver.resolve_entities_for_day(&day_def, "test_day");

        // Should be: [Bishop, Martyr (from base), DoctorOfTheChurch]
        assert_eq!(
            entities[0].titles,
            Some(vec![Title::Bishop, Title::Martyr, Title::DoctorOfTheChurch])
        );
    }
}
