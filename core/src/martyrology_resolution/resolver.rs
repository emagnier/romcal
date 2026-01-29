//! Martyrology resolver implementation.
//!
//! This module provides the main MartyrologyResolver struct for resolving martyrology entries
//! with locale fallback.

use std::collections::BTreeMap;

use super::locale::build_merge_hierarchy;
use super::merge::merge_locale_resources;
use super::pointer::{combine_titles, resolve_martyrology_pointer};
use crate::error::{RomcalError, RomcalResult};
use crate::romcal::Romcal;
use crate::types::calendar::day_definition::DayDefinition;
use crate::types::martyrology::title::TitlesDef;
use crate::types::martyrology::{MartyrologyEntry, MartyrologyEntryId};

/// Resolver for martyrology entries in liturgical days.
///
/// This struct is responsible for:
/// - Merging locale resources (base 'en' + parent locales + target locale)
/// - Resolving martyrology pointers to full MartyrologyEntry objects
/// - Combining titles from multiple entries
pub struct MartyrologyResolver {
    /// Merged entries from all locale resources
    entries: BTreeMap<MartyrologyEntryId, MartyrologyEntry>,
    /// The target locale
    locale: String,
    /// The locale hierarchy that was checked (for error messages)
    locale_hierarchy: Vec<String>,
}

impl MartyrologyResolver {
    /// Creates a new MartyrologyResolver from a Romcal instance.
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
        let locale_hierarchy = build_merge_hierarchy(&locale);
        let entries = merge_locale_resources(romcal);

        Self {
            entries,
            locale,
            locale_hierarchy,
        }
    }

    /// Returns the target locale
    pub fn locale(&self) -> &str {
        &self.locale
    }

    /// Resolves a martyrology entry by its ID.
    ///
    /// Returns the entry if found, or None if not found.
    pub fn resolve_entry(&self, id: &str) -> Option<&MartyrologyEntry> {
        self.entries.get(id)
    }

    /// Resolves all martyrology entries for a day definition.
    ///
    /// Resolution strategy:
    /// 1. If day_def.martyrology is defined: resolve each MartyrologyRef
    /// 2. Otherwise (fallback): look for entry with id == day_id
    ///    - If found: return that entry
    ///    - If not found: return error
    ///
    /// # Errors
    ///
    /// Returns `RomcalError::MartyrologyEntryNotFound` if an entry ID is not found
    /// in the merged entries map (checked locales: specific → parent → en).
    pub fn resolve_martyrology_for_day(
        &self,
        day_def: &DayDefinition,
        day_id: &str,
    ) -> RomcalResult<Vec<MartyrologyEntry>> {
        if let Some(martyrology_pointers) = &day_def.martyrology {
            // Resolve each martyrology pointer
            let mut entries = Vec::with_capacity(martyrology_pointers.len());
            for pointer in martyrology_pointers {
                let entry =
                    resolve_martyrology_pointer(&self.entries, pointer, &self.locale_hierarchy)?;
                entries.push(entry);
            }
            Ok(entries)
        } else {
            // Fallback: try to find entry with same ID as day_id
            if let Some(entry) = self.entries.get(day_id) {
                Ok(vec![entry.clone()])
            } else {
                Err(RomcalError::MartyrologyEntryNotFound(
                    day_id.to_string(),
                    self.locale_hierarchy.clone(),
                ))
            }
        }
    }

    /// Gets the fullname for a liturgical day.
    ///
    /// If custom_locale_id is provided, uses that ID for lookup, otherwise uses day_id.
    /// Returns the fullname from the entry if found, None otherwise.
    pub fn get_fullname_for_day(
        &self,
        day_id: &str,
        custom_locale_id: Option<&str>,
    ) -> Option<String> {
        let lookup_id = custom_locale_id.unwrap_or(day_id);
        self.entries.get(lookup_id).and_then(|e| e.fullname.clone())
    }

    /// Combines titles from all entries into a single TitlesDef.
    ///
    /// This function:
    /// 1. Collects all titles from each entry (respecting hide_titles)
    /// 2. Deduplicates titles
    /// 3. Returns TitlesDef::Titles with combined titles
    pub fn combine_titles(&self, entries: &[MartyrologyEntry]) -> TitlesDef {
        combine_titles(entries)
    }

    /// Gets all merged entries (for searching/iteration)
    pub fn get_all_entries(&self) -> &BTreeMap<MartyrologyEntryId, MartyrologyEntry> {
        &self.entries
    }

    /// Checks if an entry exists by ID
    pub fn has_entry(&self, id: &str) -> bool {
        self.entries.contains_key(id)
    }

    /// Gets the count of merged entries
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::resources::Resources;
    use crate::romcal::Preset;
    use crate::types::calendar::MartyrologyRef;
    use crate::types::martyrology::MartyrologyEntryDef;
    use crate::types::martyrology::entry_override::MartyrologyEntryOverride;
    use crate::types::martyrology::title::{CompoundTitle, Title};

    fn create_test_entry_def(name: &str, titles: Vec<Title>) -> MartyrologyEntryDef {
        let mut definition = MartyrologyEntryDef::new();
        definition.name = Some(name.to_string());
        definition.titles = Some(titles);
        definition
    }

    fn create_test_resources(locale: &str, entries: Vec<(&str, MartyrologyEntryDef)>) -> Resources {
        let mut resources = Resources::new(locale.to_string());
        for (id, definition) in entries {
            resources.add_martyrology_entry(id.to_string(), definition);
        }
        resources
    }

    #[test]
    fn test_martyrology_resolver_creation() {
        let romcal = Romcal::default();
        let resolver = MartyrologyResolver::new(&romcal);

        assert_eq!(resolver.locale(), "en");
    }

    #[test]
    fn test_resolve_martyrology_pointer_resource_id() {
        let mut romcal = Romcal::default();

        // Add test entry
        let definition = create_test_entry_def("Test Saint", vec![Title::Martyr]);
        let resources = create_test_resources("en", vec![("test_saint", definition)]);
        romcal.add_resources(resources);

        let resolver = MartyrologyResolver::new(&romcal);

        // Test resolving by ID
        let entry = resolver.resolve_entry("test_saint").unwrap();

        assert_eq!(entry.name, Some("Test Saint".to_string()));
        assert_eq!(entry.titles, Some(vec![Title::Martyr]));
    }

    #[test]
    fn test_locale_merge_order() {
        let mut romcal = Romcal::new(Preset {
            locale: Some("fr-FR".to_string()),
            ..Preset::default()
        })
        .unwrap();

        // Add English entry (base)
        let en_def = create_test_entry_def("Test Saint (EN)", vec![Title::Martyr]);
        let en_resources = create_test_resources("en", vec![("test_saint", en_def)]);
        romcal.add_resources(en_resources);

        // Add French entry (parent) - should override EN
        let mut fr_def = MartyrologyEntryDef::new();
        fr_def.name = Some("Saint Test (FR)".to_string());
        let fr_resources = create_test_resources("fr", vec![("test_saint", fr_def)]);
        romcal.add_resources(fr_resources);

        // Add French-France entry (specific) - should override FR
        let mut fr_fr_def = MartyrologyEntryDef::new();
        fr_fr_def.fullname = Some("Saint Test de France".to_string());
        let fr_fr_resources = create_test_resources("fr-FR", vec![("test_saint", fr_fr_def)]);
        romcal.add_resources(fr_fr_resources);

        let resolver = MartyrologyResolver::new(&romcal);
        let entry = resolver.resolve_entry("test_saint").unwrap();

        // Name should be from fr (parent), not en
        assert_eq!(entry.name, Some("Saint Test (FR)".to_string()));
        // Fullname should be from fr-FR (most specific)
        assert_eq!(entry.fullname, Some("Saint Test de France".to_string()));
        // Titles should be from en (base, not overridden)
        assert_eq!(entry.titles, Some(vec![Title::Martyr]));
    }

    #[test]
    fn test_locale_merge_specific_overrides_parent() {
        let mut romcal = Romcal::new(Preset {
            locale: Some("fr-FR".to_string()),
            ..Preset::default()
        })
        .unwrap();

        // Add French entry (parent)
        let mut fr_def = MartyrologyEntryDef::new();
        fr_def.name = Some("Nom FR".to_string());
        let fr_resources = create_test_resources("fr", vec![("test_saint", fr_def)]);
        romcal.add_resources(fr_resources);

        // Add French-France entry (specific) - should override FR name
        let mut fr_fr_def = MartyrologyEntryDef::new();
        fr_fr_def.name = Some("Nom FR-FR".to_string());
        let fr_fr_resources = create_test_resources("fr-FR", vec![("test_saint", fr_fr_def)]);
        romcal.add_resources(fr_fr_resources);

        let resolver = MartyrologyResolver::new(&romcal);
        let entry = resolver.resolve_entry("test_saint").unwrap();

        // Name should be from fr-FR (specific), NOT fr (parent)
        assert_eq!(entry.name, Some("Nom FR-FR".to_string()));
    }

    #[test]
    fn test_resolve_martyrology_for_day_with_pointers() {
        let mut romcal = Romcal::default();

        // Add test entries
        let definition1 = create_test_entry_def("Saint Peter", vec![Title::Apostle]);
        let definition2 = create_test_entry_def("Saint Paul", vec![Title::Apostle, Title::Martyr]);
        let resources = create_test_resources(
            "en",
            vec![
                ("peter_apostle", definition1),
                ("paul_apostle", definition2),
            ],
        );
        romcal.add_resources(resources);

        let resolver = MartyrologyResolver::new(&romcal);

        // Create day definition with martyrology entries
        let day_def = DayDefinition {
            date_def: None,
            date_exceptions: None,
            precedence: None,
            commons_def: None,
            is_holy_day_of_obligation: None,
            allow_similar_rank_items: None,
            is_optional: None,
            custom_locale_id: None,
            martyrology: Some(vec![
                MartyrologyRef::ResourceId("peter_apostle".to_string()),
                MartyrologyRef::ResourceId("paul_apostle".to_string()),
            ]),
            titles: None,
            drop: None,
            colors: None,
            masses: None,
        };

        let entries = resolver
            .resolve_martyrology_for_day(&day_def, "test_day")
            .unwrap();

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].name, Some("Saint Peter".to_string()));
        assert_eq!(entries[1].name, Some("Saint Paul".to_string()));
    }

    #[test]
    fn test_resolve_martyrology_for_day_fallback() {
        let mut romcal = Romcal::default();

        // Add entry with same ID as day_id
        let definition = create_test_entry_def("Test Saint", vec![Title::Martyr]);
        let resources = create_test_resources("en", vec![("test_day_id", definition)]);
        romcal.add_resources(resources);

        let resolver = MartyrologyResolver::new(&romcal);

        // Create day definition without martyrology (should fallback to day_id)
        let day_def = DayDefinition {
            date_def: None,
            date_exceptions: None,
            precedence: None,
            commons_def: None,
            is_holy_day_of_obligation: None,
            allow_similar_rank_items: None,
            is_optional: None,
            custom_locale_id: None,
            martyrology: None,
            titles: None,
            drop: None,
            colors: None,
            masses: None,
        };

        let entries = resolver
            .resolve_martyrology_for_day(&day_def, "test_day_id")
            .unwrap();

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, Some("Test Saint".to_string()));
    }

    #[test]
    fn test_resolve_martyrology_for_day_not_found() {
        let romcal = Romcal::default();
        let resolver = MartyrologyResolver::new(&romcal);

        // Create day definition with entry that doesn't exist
        let day_def = DayDefinition {
            date_def: None,
            date_exceptions: None,
            precedence: None,
            commons_def: None,
            is_holy_day_of_obligation: None,
            allow_similar_rank_items: None,
            is_optional: None,
            custom_locale_id: None,
            martyrology: Some(vec![MartyrologyRef::ResourceId(
                "nonexistent_entry".to_string(),
            )]),
            titles: None,
            drop: None,
            colors: None,
            masses: None,
        };

        let result = resolver.resolve_martyrology_for_day(&day_def, "test_day");

        assert!(result.is_err());
        match result {
            Err(RomcalError::MartyrologyEntryNotFound(id, locales)) => {
                assert_eq!(id, "nonexistent_entry");
                assert_eq!(locales, vec!["en"]);
            }
            _ => panic!("Expected MartyrologyEntryNotFound error"),
        }
    }

    #[test]
    fn test_resolve_martyrology_for_day_fallback_not_found() {
        let romcal = Romcal::default();
        let resolver = MartyrologyResolver::new(&romcal);

        // Create day definition without martyrology (should fallback to day_id which doesn't exist)
        let day_def = DayDefinition {
            date_def: None,
            date_exceptions: None,
            precedence: None,
            commons_def: None,
            is_holy_day_of_obligation: None,
            allow_similar_rank_items: None,
            is_optional: None,
            custom_locale_id: None,
            martyrology: None,
            titles: None,
            drop: None,
            colors: None,
            masses: None,
        };

        let result = resolver.resolve_martyrology_for_day(&day_def, "nonexistent_day");

        assert!(result.is_err());
        match result {
            Err(RomcalError::MartyrologyEntryNotFound(id, _)) => {
                assert_eq!(id, "nonexistent_day");
            }
            _ => panic!("Expected MartyrologyEntryNotFound error"),
        }
    }

    #[test]
    fn test_resolve_martyrology_pointer_override() {
        let mut romcal = Romcal::default();

        // Add base entry
        let definition = create_test_entry_def("Test Saint", vec![Title::Martyr]);
        let resources = create_test_resources("en", vec![("test_saint", definition)]);
        romcal.add_resources(resources);

        let resolver = MartyrologyResolver::new(&romcal);

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
            martyrology: Some(vec![MartyrologyRef::Override(MartyrologyEntryOverride {
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

        let entries = resolver
            .resolve_martyrology_for_day(&day_def, "test_day")
            .unwrap();

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].name, Some("Test Saint".to_string()));
        assert_eq!(entries[0].titles, Some(vec![Title::Bishop, Title::Martyr]));
        assert_eq!(entries[0].hide_titles, Some(false));
    }

    #[test]
    fn test_combine_titles() {
        let romcal = Romcal::default();
        let resolver = MartyrologyResolver::new(&romcal);

        fn create_entry_with_titles(id: &str, titles: Vec<Title>) -> MartyrologyEntry {
            let mut definition = MartyrologyEntryDef::new();
            definition.titles = Some(titles);
            MartyrologyEntry::new(id.to_string(), definition)
        }

        let entries = vec![
            create_entry_with_titles("saint_1", vec![Title::Martyr, Title::Bishop]),
            create_entry_with_titles("saint_2", vec![Title::Apostle, Title::Martyr]), // Martyr is duplicate
        ];

        let combined = resolver.combine_titles(&entries);

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
        let resolver = MartyrologyResolver::new(&romcal);

        fn create_entry(id: &str, titles: Vec<Title>, hide: bool) -> MartyrologyEntry {
            let mut definition = MartyrologyEntryDef::new();
            definition.titles = Some(titles);
            let mut entry = MartyrologyEntry::new(id.to_string(), definition);
            entry.hide_titles = Some(hide);
            entry
        }

        let entries = vec![
            create_entry("visible", vec![Title::Martyr], false),
            create_entry("hidden", vec![Title::Pope], true),
        ];

        let combined = resolver.combine_titles(&entries);

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

        // Add base entry
        let definition = create_test_entry_def("Test Saint", vec![Title::Martyr]);
        let resources = create_test_resources("en", vec![("test_saint", definition)]);
        romcal.add_resources(resources);

        let resolver = MartyrologyResolver::new(&romcal);

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
            martyrology: Some(vec![MartyrologyRef::Override(MartyrologyEntryOverride {
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

        let entries = resolver
            .resolve_martyrology_for_day(&day_def, "test_day")
            .unwrap();

        // Should be: [Bishop, Martyr (from base), DoctorOfTheChurch]
        assert_eq!(
            entries[0].titles,
            Some(vec![Title::Bishop, Title::Martyr, Title::DoctorOfTheChurch])
        );
    }
}
