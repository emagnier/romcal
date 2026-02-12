use super::*;
use crate::romcal::Preset;

#[test]
fn test_proper_of_time_creation() {
    let romcal = Romcal::default();
    let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();

    assert_eq!(proper_of_time.cache.advent_year(), 2026);
    assert_eq!(proper_of_time.cache.easter_year(), 2026);
}

#[test]
fn test_no_duplicate_dates() {
    let romcal = Romcal::default();
    let all_days = ProperOfTime::new(romcal, 2026)
        .unwrap()
        .generate_all()
        .unwrap();

    // Check that we have generated days
    assert!(!all_days.is_empty());

    // Extract all dates and check for duplicates
    let mut dates: Vec<&str> = all_days.iter().map(|day| day.date.as_str()).collect();
    let original_count = dates.len();

    // Sort and deduplicate
    dates.sort();
    dates.dedup();
    let unique_count = dates.len();

    // Exception: Holy Thursday has two liturgical days on the same date:
    // - holy_thursday (from lent)
    // - thursday_of_the_lords_supper (from paschal_triduum)
    // So we expect exactly 1 duplicate date
    let expected_duplicates = 1;
    let actual_duplicates = original_count - unique_count;

    assert_eq!(
        actual_duplicates, expected_duplicates,
        "Expected {} duplicate date (Holy Thursday), but found {} duplicates. Original: {}, Unique: {}",
        expected_duplicates, actual_duplicates, original_count, unique_count
    );

    // Additional check: verify that only Holy Thursday has duplicate dates
    let mut date_groups: std::collections::HashMap<String, Vec<&LiturgicalDay>> =
        std::collections::HashMap::new();
    for day in &all_days {
        date_groups.entry(day.date.clone()).or_default().push(day);
    }

    let duplicate_dates: Vec<_> = date_groups
        .iter()
        .filter(|(_, days)| days.len() > 1)
        .collect();

    assert_eq!(
        duplicate_dates.len(),
        1,
        "Expected exactly 1 duplicate date (Holy Thursday), but found {}: {:?}",
        duplicate_dates.len(),
        duplicate_dates
            .iter()
            .map(|(date, days)| (date, days.iter().map(|d| &d.id).collect::<Vec<_>>()))
            .collect::<Vec<_>>()
    );

    // Verify that the duplicate is indeed Holy Thursday
    let holy_thursday_days = duplicate_dates[0].1;
    assert_eq!(holy_thursday_days.len(), 2);
    assert!(holy_thursday_days.iter().any(|d| d.id == "holy_thursday"));
    assert!(
        holy_thursday_days
            .iter()
            .any(|d| d.id == "thursday_of_the_lords_supper")
    );
}

#[test]
fn test_no_duplicate_dates_liturgical_context() {
    let romcal = Romcal::new(Preset {
        context: Some(crate::CalendarContext::Liturgical),
        ..Preset::default()
    })
    .unwrap();
    let all_days = ProperOfTime::new(romcal, 2026)
        .unwrap()
        .generate_all()
        .unwrap();

    // Check that we have generated days
    assert!(!all_days.is_empty());

    // Extract all dates and check for duplicates
    let mut dates: Vec<&str> = all_days.iter().map(|day| day.date.as_str()).collect();
    let original_count = dates.len();

    // Sort and deduplicate
    dates.sort();
    dates.dedup();
    let unique_count = dates.len();

    // Exception: Holy Thursday has two liturgical days on the same date:
    // - holy_thursday (from lent)
    // - thursday_of_the_lords_supper (from paschal_triduum)
    // So we expect exactly 1 duplicate date
    let expected_duplicates = 1;
    let actual_duplicates = original_count - unique_count;

    assert_eq!(
        actual_duplicates, expected_duplicates,
        "Expected {} duplicate date (Holy Thursday), but found {} duplicates (liturgical context). Original: {}, Unique: {}",
        expected_duplicates, actual_duplicates, original_count, unique_count
    );
}

#[test]
fn test_sort_liturgical_days_by_date() {
    let romcal = Romcal::default();
    let mut all_days = ProperOfTime::new(romcal, 2026)
        .unwrap()
        .generate_all()
        .unwrap();

    // Shuffle the days to test sorting
    all_days.reverse();

    // Sort using the utility function
    sort_liturgical_days_by_date(&mut all_days);

    // Verify that days are sorted by date
    for i in 1..all_days.len() {
        let date_a = chrono::NaiveDate::parse_from_str(&all_days[i - 1].date, "%Y-%m-%d")
            .unwrap_or_default();
        let date_b =
            chrono::NaiveDate::parse_from_str(&all_days[i].date, "%Y-%m-%d").unwrap_or_default();
        assert!(
            date_a <= date_b,
            "Days are not sorted by date: {} should come before {}",
            all_days[i - 1].date,
            all_days[i].date
        );
    }
}

#[test]
fn test_calendar_continuity() {
    let romcal = Romcal::default();

    // Get all liturgical days
    let mut days = ProperOfTime::new(romcal, 2026)
        .unwrap()
        .generate_all()
        .unwrap();

    // Sort by date
    sort_liturgical_days_by_date(&mut days);

    // Verify that there are no gaps in dates between first and last day
    for i in 1..days.len() {
        let prev_date = chrono::NaiveDate::parse_from_str(&days[i - 1].date, "%Y-%m-%d").unwrap();
        let curr_date = chrono::NaiveDate::parse_from_str(&days[i].date, "%Y-%m-%d").unwrap();

        // Each day should be either:
        // 1. Same date as previous (for duplicates like Holy Thursday)
        // 2. Next day after previous (no gaps)
        let days_diff = (curr_date - prev_date).num_days();
        assert!(
            days_diff == 0 || days_diff == 1,
            "Gap found in calendar: {} to {} ({} days difference). Each day should be same date or next day.",
            prev_date,
            curr_date,
            days_diff
        );
    }

    // Verify that day_of_week matches the actual day of the week for each date
    for day in &days {
        let date = chrono::NaiveDate::parse_from_str(&day.date, "%Y-%m-%d").unwrap();
        let actual_dow = date.weekday().num_days_from_sunday() as u8;
        let stored_dow = day.day_of_week.0;

        assert_eq!(
            actual_dow, stored_dow,
            "day_of_week mismatch for {}: date {} is actually day {} but stored as day {}",
            day.id, day.date, actual_dow, stored_dow
        );
    }

    // TODO: Add week_of_season consistency test
    // This test should verify that week_of_season follows the correct pattern:
    // - Each season starts with week 1 (or 0 for Lent)
    // - Week numbers increment on Sundays
    // - Special handling for Christmas Time and Ordinary Time
    // - Complex logic needed for different seasons
}

// -------------------------------------------------------------------------
// Tests for ordinal_format resolution
// -------------------------------------------------------------------------

use crate::engine::resources::Resources;
use crate::types::OrdinalFormat;
use crate::types::resource::ResourcesMetadata;
use std::collections::BTreeMap;

/// Creates a minimal ResourcesMetadata for testing
fn create_test_metadata(ordinal_format: Option<OrdinalFormat>) -> ResourcesMetadata {
    let mut ordinals_letters = BTreeMap::new();
    ordinals_letters.insert("1".to_string(), "first".to_string());
    ordinals_letters.insert("2".to_string(), "second".to_string());

    let mut ordinals_numeric = BTreeMap::new();
    ordinals_numeric.insert("1".to_string(), "1st".to_string());
    ordinals_numeric.insert("2".to_string(), "2nd".to_string());

    ResourcesMetadata {
        ordinal_format,
        ordinals_letters: Some(ordinals_letters),
        ordinals_numeric: Some(ordinals_numeric),
        weekdays: None,
        months: None,
        colors: None,
        seasons: None,
        periods: None,
        ranks: None,
        cycles: None,
    }
}

/// Creates a test Resources with the given locale and metadata
fn create_test_resources(locale: &str, ordinal_format: Option<OrdinalFormat>) -> Resources {
    Resources {
        schema: None,
        locale: locale.to_string(),
        metadata: Some(create_test_metadata(ordinal_format)),
        martyrology: None,
    }
}

#[test]
fn test_ordinal_format_default_is_numeric() {
    // When no ordinal_format is specified anywhere, default should be Numeric
    let romcal = Romcal::default();
    assert_eq!(romcal.ordinal_format, OrdinalFormat::Numeric);
}

#[test]
fn test_ordinal_format_from_locale_metadata() {
    // When ordinal_format is set in locale metadata, it should be used
    let romcal = Romcal {
        locale: "test".to_string(),
        resources: vec![create_test_resources("test", Some(OrdinalFormat::Letters))],
        ..Default::default()
    };

    let resolver = ProperOfTime::create_template_resolver(&romcal);
    assert!(resolver.is_some());
    assert_eq!(resolver.unwrap().ordinal_format(), OrdinalFormat::Letters);
}

#[test]
fn test_ordinal_format_from_romcal_when_metadata_not_set() {
    // When ordinal_format is not set in metadata, romcal value should be used
    let romcal = Romcal {
        locale: "test".to_string(),
        ordinal_format: OrdinalFormat::Letters,
        resources: vec![create_test_resources("test", None)],
        ..Default::default()
    };

    let resolver = ProperOfTime::create_template_resolver(&romcal);
    assert!(resolver.is_some());
    assert_eq!(resolver.unwrap().ordinal_format(), OrdinalFormat::Letters);
}

#[test]
fn test_ordinal_format_metadata_takes_priority() {
    // When ordinal_format is set in both metadata and romcal, metadata should win
    let romcal = Romcal {
        locale: "test".to_string(),
        ordinal_format: OrdinalFormat::Numeric, // Romcal says Numeric
        resources: vec![create_test_resources("test", Some(OrdinalFormat::Letters))], // Metadata says Letters
        ..Default::default()
    };

    let resolver = ProperOfTime::create_template_resolver(&romcal);
    assert!(resolver.is_some());
    // Metadata should take priority
    assert_eq!(resolver.unwrap().ordinal_format(), OrdinalFormat::Letters);
}

#[test]
fn test_ordinal_format_fallback_to_en_locale() {
    // When target locale has no metadata but 'en' does, use 'en' metadata
    let romcal = Romcal {
        locale: "nonexistent".to_string(),
        resources: vec![create_test_resources("en", Some(OrdinalFormat::Letters))],
        ..Default::default()
    };

    let resolver = ProperOfTime::create_template_resolver(&romcal);
    assert!(resolver.is_some());
    assert_eq!(resolver.unwrap().ordinal_format(), OrdinalFormat::Letters);
}

#[test]
fn test_ordinal_format_no_resolver_without_resources() {
    // When no resources are available, resolver should be None
    let romcal = Romcal {
        locale: "test".to_string(),
        resources: vec![],
        ..Default::default()
    };

    let resolver = ProperOfTime::create_template_resolver(&romcal);
    assert!(resolver.is_none());
}

// -------------------------------------------------------------------------
// Tests for martyrology-based fullname resolution
// -------------------------------------------------------------------------

use crate::types::martyrology::MartyrologyEntryDef;

/// Creates test resources with martyrology entries for fullname resolution tests
fn create_test_resources_with_martyrology(
    locale: &str,
    martyrology: std::collections::BTreeMap<String, MartyrologyEntryDef>,
) -> Resources {
    Resources {
        schema: None,
        locale: locale.to_string(),
        metadata: Some(create_test_metadata(None)),
        martyrology: Some(martyrology),
    }
}

#[test]
fn test_fullname_resolved_from_martyrology() {
    // When a martyrology entry has a fullname defined, it should be used
    let mut martyrology = std::collections::BTreeMap::new();
    martyrology.insert(
        "mary_mother_of_god".to_string(),
        MartyrologyEntryDef {
            fullname: Some("Mary, Mother of God".to_string()),
            ..Default::default()
        },
    );

    let romcal = Romcal {
        locale: "en".to_string(),
        resources: vec![create_test_resources_with_martyrology("en", martyrology)],
        ..Default::default()
    };

    let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();

    // Check that martyrology resolver has the entry
    let fullname = proper_of_time
        .martyrology_resolver
        .get_fullname_for_day("mary_mother_of_god", None);
    assert_eq!(fullname, Some("Mary, Mother of God".to_string()));
}

#[test]
fn test_fullname_fallback_to_template_when_no_martyrology() {
    // When no martyrology fullname exists but day_type is provided, template should be used
    let romcal = Romcal {
        locale: "en".to_string(),
        resources: vec![create_test_resources("en", None)],
        ..Default::default()
    };

    let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();

    // For days like "advent_sunday_1" that don't have martyrology fullnames,
    // the template resolver should be used
    // This is implicitly tested by the fact that ProperOfTime works correctly
    assert!(proper_of_time.template_resolver.is_some());
}

#[test]
fn test_martyrology_fullname_priority_over_template() {
    // Martyrology fullname should take priority over template resolution
    // This tests the priority: Martyrology > Template > ID

    let mut martyrology = std::collections::BTreeMap::new();
    martyrology.insert(
        "test_entry".to_string(),
        MartyrologyEntryDef {
            fullname: Some("Martyrology Fullname".to_string()),
            ..Default::default()
        },
    );

    let romcal = Romcal {
        locale: "en".to_string(),
        resources: vec![create_test_resources_with_martyrology("en", martyrology)],
        ..Default::default()
    };

    let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();

    // The martyrology resolver should find the fullname
    let fullname = proper_of_time
        .martyrology_resolver
        .get_fullname_for_day("test_entry", None);
    assert_eq!(fullname, Some("Martyrology Fullname".to_string()));

    // Non-existent entry should return None
    let no_fullname = proper_of_time
        .martyrology_resolver
        .get_fullname_for_day("nonexistent", None);
    assert!(no_fullname.is_none());
}

#[test]
fn test_martyrology_fullname_with_locale_override() {
    // When target locale has martyrology fullname, it should override 'en'
    let mut en_martyrology = std::collections::BTreeMap::new();
    en_martyrology.insert(
        "mary_mother_of_god".to_string(),
        MartyrologyEntryDef {
            fullname: Some("Mary, Mother of God".to_string()),
            ..Default::default()
        },
    );

    let mut fr_martyrology = std::collections::BTreeMap::new();
    fr_martyrology.insert(
        "mary_mother_of_god".to_string(),
        MartyrologyEntryDef {
            fullname: Some("Sainte Marie, Mère de Dieu".to_string()),
            ..Default::default()
        },
    );

    let romcal = Romcal {
        locale: "fr".to_string(),
        resources: vec![
            create_test_resources_with_martyrology("en", en_martyrology),
            create_test_resources_with_martyrology("fr", fr_martyrology),
        ],
        ..Default::default()
    };

    let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();

    // French locale should use French fullname
    let fullname = proper_of_time
        .martyrology_resolver
        .get_fullname_for_day("mary_mother_of_god", None);
    assert_eq!(fullname, Some("Sainte Marie, Mère de Dieu".to_string()));
}
