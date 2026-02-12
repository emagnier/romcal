//! Tests for bundle generation.

use crate::types::calendar::{CalendarJurisdiction, CalendarType, DayDefinition, MartyrologyRef};
use crate::types::martyrology::{
    MartyrologyEntryDef, MartyrologyEntryOverride, MartyrologyEntryType,
};
use crate::types::resource::{AdventSeason, ResourcesMetadata, SeasonsMetadata};
use crate::{CalendarDefinition, Resources, Romcal};

use super::IdSet;
use super::deduplicate::{
    deduplicate_martyrology_properties, deduplicate_metadata_properties, is_entry_empty,
    remove_empty_martyrology_entries,
};
use super::filter::{
    collect_used_martyrology_ids, filter_calendar_definitions, filter_martyrology_by_usage,
};

// ------------------------------------------------------------------------
// Test Helpers
// ------------------------------------------------------------------------

/// Create a test martyrology entry with specified properties.
fn martyrology_entry(
    name: Option<&str>,
    fullname: Option<&str>,
    entry_type: Option<MartyrologyEntryType>,
) -> MartyrologyEntryDef {
    MartyrologyEntryDef {
        name: name.map(String::from),
        fullname: fullname.map(String::from),
        r#type: entry_type,
        ..Default::default()
    }
}

/// Create a Resources with a single martyrology entry.
fn resources_with_martyrology_entry(
    locale: &str,
    entry_id: &str,
    e: MartyrologyEntryDef,
) -> Resources {
    let mut martyrology = std::collections::BTreeMap::new();
    martyrology.insert(entry_id.to_string(), e);
    Resources {
        schema: None,
        locale: locale.to_string(),
        metadata: None,
        martyrology: Some(martyrology),
    }
}

/// Create a Resources with metadata.
fn resources_with_metadata(locale: &str, metadata: ResourcesMetadata) -> Resources {
    Resources {
        schema: None,
        locale: locale.to_string(),
        metadata: Some(metadata),
        martyrology: None,
    }
}

/// Create a minimal calendar definition.
fn calendar_def(id: &str, parents: Vec<&str>) -> CalendarDefinition {
    CalendarDefinition {
        schema: None,
        id: id.to_string(),
        metadata: crate::types::CalendarMetadata {
            jurisdiction: CalendarJurisdiction::Ecclesiastical,
            r#type: CalendarType::Diocese,
        },
        particular_config: None,
        parent_calendar_ids: parents.into_iter().map(String::from).collect(),
        days_definitions: std::collections::BTreeMap::new(),
    }
}

/// Create an empty ResourcesMetadata.
fn empty_metadata() -> ResourcesMetadata {
    ResourcesMetadata::default()
}

// ------------------------------------------------------------------------
// Calendar Filtering Tests
// ------------------------------------------------------------------------

#[test]
fn test_filter_calendar_definitions_hierarchy() {
    let romcal = Romcal {
        calendar: "france".to_string(),
        locale: "fr".to_string(),
        calendar_definitions: vec![
            calendar_def("general_roman", vec![]),
            calendar_def("europe", vec!["general_roman"]),
            calendar_def("france", vec!["europe", "general_roman"]),
            calendar_def("unrelated", vec!["general_roman"]),
        ],
        ..Default::default()
    };

    // filter_calendar_definitions returns specific → general
    // bundle() reverses to general → specific for output
    let result = filter_calendar_definitions(&romcal).unwrap();

    assert_eq!(result.len(), 3);
    assert_eq!(result[0].id, "france"); // most specific
    assert_eq!(result[1].id, "europe"); // parent
    assert_eq!(result[2].id, "general_roman"); // fallback
}

#[test]
fn test_filter_calendar_definitions_missing_calendar() {
    let romcal = Romcal {
        calendar: "nonexistent".to_string(),
        locale: "en".to_string(),
        calendar_definitions: vec![calendar_def("general_roman", vec![])],
        ..Default::default()
    };

    let result = filter_calendar_definitions(&romcal);
    assert!(result.is_err());
}

#[test]
fn test_filter_calendar_definitions_circular_reference() {
    let romcal = Romcal {
        calendar: "circular".to_string(),
        locale: "en".to_string(),
        calendar_definitions: vec![
            calendar_def("general_roman", vec![]),
            CalendarDefinition {
                parent_calendar_ids: vec!["circular".to_string()],
                ..calendar_def("circular", vec![])
            },
        ],
        ..Default::default()
    };

    let result = filter_calendar_definitions(&romcal);
    assert!(result.is_err());
}

// ------------------------------------------------------------------------
// Martyrology Collection Tests
// ------------------------------------------------------------------------

#[test]
fn test_collect_used_martyrology_ids() {
    let mut days = std::collections::BTreeMap::new();
    days.insert(
        "saint_john".to_string(),
        DayDefinition {
            martyrology: Some(vec![
                MartyrologyRef::ResourceId("john_baptist".to_string()),
                MartyrologyRef::Override(MartyrologyEntryOverride {
                    id: "john_evangelist".to_string(),
                    titles: None,
                    hide_titles: None,
                    count: None,
                }),
            ]),
            ..Default::default()
        },
    );
    days.insert("saint_peter".to_string(), DayDefinition::default());

    let cal = CalendarDefinition {
        days_definitions: days,
        ..calendar_def("test", vec![])
    };

    let ids = collect_used_martyrology_ids(&[cal]);

    assert!(ids.contains("saint_john"));
    assert!(ids.contains("saint_peter"));
    assert!(ids.contains("john_baptist"));
    assert!(ids.contains("john_evangelist"));
    assert!(!ids.contains("unused"));
}

#[test]
fn test_filter_martyrology_by_usage() {
    let mut res = Resources {
        schema: None,
        locale: "en".to_string(),
        metadata: None,
        martyrology: Some({
            let mut m = std::collections::BTreeMap::new();
            m.insert(
                "used".to_string(),
                martyrology_entry(Some("Used"), None, None),
            );
            m.insert(
                "unused".to_string(),
                martyrology_entry(Some("Unused"), None, None),
            );
            m
        }),
    };

    let used: IdSet = ["used"].iter().map(|s| s.to_string()).collect();
    filter_martyrology_by_usage(&mut res, &used);

    let martyrology = res.martyrology.unwrap();
    assert_eq!(martyrology.len(), 1);
    assert!(martyrology.contains_key("used"));
}

// ------------------------------------------------------------------------
// Martyrology Property Deduplication Tests
// ------------------------------------------------------------------------

#[test]
fn test_deduplicate_martyrology_properties_hierarchy() {
    // fr-ca (specific) → fr (parent) → en (fallback)
    let mut resources = vec![
        resources_with_martyrology_entry(
            "fr-ca",
            "john",
            martyrology_entry(Some("Jean"), None, None),
        ),
        resources_with_martyrology_entry(
            "fr",
            "john",
            martyrology_entry(Some("Jean"), Some("Jean le Baptiste"), None),
        ),
        resources_with_martyrology_entry(
            "en",
            "john",
            martyrology_entry(
                Some("John"),
                Some("John the Baptist"),
                Some(MartyrologyEntryType::Person),
            ),
        ),
    ];

    deduplicate_martyrology_properties(&mut resources);

    // fr-ca: keeps name (most specific)
    let fr_ca = resources[0]
        .martyrology
        .as_ref()
        .unwrap()
        .get("john")
        .unwrap();
    assert!(fr_ca.name.is_some());
    assert!(fr_ca.fullname.is_none());
    assert!(fr_ca.r#type.is_none());

    // fr: name removed (in fr-ca), keeps fullname
    let fr = resources[1]
        .martyrology
        .as_ref()
        .unwrap()
        .get("john")
        .unwrap();
    assert!(fr.name.is_none());
    assert!(fr.fullname.is_some());
    assert!(fr.r#type.is_none());

    // en: name & fullname removed, keeps type
    let en = resources[2]
        .martyrology
        .as_ref()
        .unwrap()
        .get("john")
        .unwrap();
    assert!(en.name.is_none());
    assert!(en.fullname.is_none());
    assert!(en.r#type.is_some());
}

#[test]
fn test_remove_empty_martyrology_entries_after_dedup() {
    let mut resources = vec![
        resources_with_martyrology_entry("fr", "john", martyrology_entry(Some("Jean"), None, None)),
        resources_with_martyrology_entry("en", "john", martyrology_entry(Some("John"), None, None)),
    ];

    deduplicate_martyrology_properties(&mut resources);
    remove_empty_martyrology_entries(&mut resources);

    // fr: keeps john
    assert!(
        resources[0]
            .martyrology
            .as_ref()
            .unwrap()
            .contains_key("john")
    );

    // en: john removed (became empty)
    assert!(
        !resources[1]
            .martyrology
            .as_ref()
            .unwrap()
            .contains_key("john")
    );
}

#[test]
fn test_is_entry_empty() {
    // Create empty martyrology entry using helper (sets all to None via Default)
    let mut empty = martyrology_entry(None, None, None);
    // Ensure all properties are None
    empty.canonization_level = None;
    empty.date_of_canonization = None;
    empty.date_of_canonization_is_approximative = None;
    empty.date_of_beatification = None;
    empty.date_of_beatification_is_approximative = None;
    empty.hide_canonization_level = None;
    empty.titles = None;
    empty.sex = None;
    empty.hide_titles = None;
    empty.date_of_dedication = None;
    empty.date_of_birth = None;
    empty.date_of_birth_is_approximative = None;
    empty.date_of_death = None;
    empty.date_of_death_is_approximative = None;
    empty.count = None;
    empty.sources = None;
    assert!(is_entry_empty(&empty));

    let with_name = martyrology_entry(Some("John"), None, None);
    assert!(!is_entry_empty(&with_name));
}

// ------------------------------------------------------------------------
// Metadata Property Deduplication Tests
// ------------------------------------------------------------------------

#[test]
fn test_deduplicate_metadata_properties() {
    let mut resources = vec![
        resources_with_metadata(
            "fr",
            ResourcesMetadata {
                weekdays: Some({
                    let mut m = std::collections::BTreeMap::new();
                    m.insert("0".to_string(), "dimanche".to_string());
                    m
                }),
                ..empty_metadata()
            },
        ),
        resources_with_metadata(
            "en",
            ResourcesMetadata {
                weekdays: Some({
                    let mut m = std::collections::BTreeMap::new();
                    m.insert("0".to_string(), "Sunday".to_string());
                    m
                }),
                months: Some({
                    let mut m = std::collections::BTreeMap::new();
                    m.insert("1".to_string(), "January".to_string());
                    m
                }),
                ..empty_metadata()
            },
        ),
    ];

    deduplicate_metadata_properties(&mut resources);

    // fr: keeps weekdays
    assert!(resources[0].metadata.as_ref().unwrap().weekdays.is_some());

    // en: weekdays removed, keeps months
    assert!(resources[1].metadata.as_ref().unwrap().weekdays.is_none());
    assert!(resources[1].metadata.as_ref().unwrap().months.is_some());
}

#[test]
fn test_deduplicate_nested_seasons() {
    let mut resources = vec![
        resources_with_metadata(
            "fr",
            ResourcesMetadata {
                seasons: Some(SeasonsMetadata {
                    advent: Some(AdventSeason {
                        season: Some("Avent".to_string()),
                        weekday: None,
                        sunday: None,
                        privileged_weekday: None,
                    }),
                    christmas_time: None,
                    ordinary_time: None,
                    lent: None,
                    paschal_triduum: None,
                    easter_time: None,
                }),
                ..empty_metadata()
            },
        ),
        resources_with_metadata(
            "en",
            ResourcesMetadata {
                seasons: Some(SeasonsMetadata {
                    advent: Some(AdventSeason {
                        season: Some("Advent".to_string()),
                        weekday: Some("Weekday of Advent".to_string()),
                        sunday: None,
                        privileged_weekday: None,
                    }),
                    christmas_time: None,
                    ordinary_time: None,
                    lent: None,
                    paschal_triduum: None,
                    easter_time: None,
                }),
                ..empty_metadata()
            },
        ),
    ];

    deduplicate_metadata_properties(&mut resources);

    // fr: keeps advent.season
    let fr_advent = resources[0]
        .metadata
        .as_ref()
        .unwrap()
        .seasons
        .as_ref()
        .unwrap()
        .advent
        .as_ref()
        .unwrap();
    assert!(fr_advent.season.is_some());

    // en: advent.season removed, keeps advent.weekday
    let en_advent = resources[1]
        .metadata
        .as_ref()
        .unwrap()
        .seasons
        .as_ref()
        .unwrap()
        .advent
        .as_ref()
        .unwrap();
    assert!(en_advent.season.is_none());
    assert!(en_advent.weekday.is_some());
}

// ------------------------------------------------------------------------
// Independent Martyrology Entries Test
// ------------------------------------------------------------------------

#[test]
fn test_deduplicate_independent_martyrology_entries() {
    let mut resources = vec![
        Resources {
            schema: None,
            locale: "fr".to_string(),
            metadata: None,
            martyrology: Some({
                let mut m = std::collections::BTreeMap::new();
                m.insert(
                    "john".to_string(),
                    martyrology_entry(Some("Jean"), None, None),
                );
                m.insert(
                    "peter".to_string(),
                    martyrology_entry(Some("Pierre"), None, None),
                );
                m
            }),
        },
        Resources {
            schema: None,
            locale: "en".to_string(),
            metadata: None,
            martyrology: Some({
                let mut m = std::collections::BTreeMap::new();
                m.insert(
                    "john".to_string(),
                    martyrology_entry(Some("John"), Some("John the Baptist"), None),
                );
                m.insert(
                    "peter".to_string(),
                    martyrology_entry(Some("Peter"), Some("Peter the Apostle"), None),
                );
                m
            }),
        },
    ];

    deduplicate_martyrology_properties(&mut resources);

    // fr: both keep name
    let fr = resources[0].martyrology.as_ref().unwrap();
    assert!(fr.get("john").unwrap().name.is_some());
    assert!(fr.get("peter").unwrap().name.is_some());

    // en: both lose name, keep fullname
    let en = resources[1].martyrology.as_ref().unwrap();
    assert!(en.get("john").unwrap().name.is_none());
    assert!(en.get("john").unwrap().fullname.is_some());
    assert!(en.get("peter").unwrap().name.is_none());
    assert!(en.get("peter").unwrap().fullname.is_some());
}
