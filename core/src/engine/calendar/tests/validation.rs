use crate::engine::calendar::Calendar;
use crate::engine::calendar_definition::CalendarDefinition;
use crate::romcal::Romcal;
use crate::types::CalendarMetadata;
use crate::types::calendar::{CalendarJurisdiction, CalendarType};

#[test]
fn test_calendar_not_found_error() {
    // Create romcal with a definition, but requesting a different calendar
    let some_calendar = CalendarDefinition {
        schema: None,
        id: "some_calendar".to_string(),
        metadata: CalendarMetadata {
            r#type: CalendarType::GeneralRoman,
            jurisdiction: CalendarJurisdiction::Ecclesiastical,
        },
        particular_config: None,
        parent_calendar_ids: vec![],
        days_definitions: std::collections::BTreeMap::new(),
    };

    let mut romcal = Romcal::empty();
    romcal.calendar = "non_existent_calendar".to_string();
    romcal.calendar_definitions.push(some_calendar);

    let result = Calendar::new(romcal, 2026);

    match result {
        Err(error) => {
            let error_str = error.to_string();
            assert!(
                error_str.contains("not found"),
                "Error should mention 'not found': {}",
                error_str
            );
            assert!(
                error_str.contains("non_existent_calendar"),
                "Error should mention the calendar name: {}",
                error_str
            );
        }
        Ok(_) => panic!("Expected an error for non-existent calendar"),
    }
}

#[test]
fn test_parent_calendar_not_found_error() {
    // Create a calendar that references a non-existent parent
    let child_calendar = CalendarDefinition {
        schema: None,
        id: "child_calendar".to_string(),
        metadata: CalendarMetadata {
            r#type: CalendarType::Country,
            jurisdiction: CalendarJurisdiction::Civil,
        },
        particular_config: None,
        parent_calendar_ids: vec!["non_existent_parent".to_string()],
        days_definitions: std::collections::BTreeMap::new(),
    };

    let mut romcal = Romcal::empty();
    romcal.calendar = "child_calendar".to_string();
    romcal.calendar_definitions.push(child_calendar);

    let result = Calendar::new(romcal, 2026);

    match result {
        Err(error) => {
            let error_str = error.to_string();
            assert!(
                error_str.contains("Parent calendar"),
                "Error should mention 'Parent calendar': {}",
                error_str
            );
            assert!(
                error_str.contains("non_existent_parent"),
                "Error should mention the parent calendar name: {}",
                error_str
            );
            assert!(
                error_str.contains("child_calendar"),
                "Error should mention which calendar requires the parent: {}",
                error_str
            );
        }
        Ok(_) => panic!("Expected an error for missing parent calendar"),
    }
}

#[test]
fn test_general_roman_calendar_with_definition() {
    let general_roman = CalendarDefinition {
        schema: None,
        id: "general_roman".to_string(),
        metadata: CalendarMetadata {
            r#type: CalendarType::GeneralRoman,
            jurisdiction: CalendarJurisdiction::Ecclesiastical,
        },
        particular_config: None,
        parent_calendar_ids: vec![],
        days_definitions: std::collections::BTreeMap::new(),
    };

    let mut romcal = Romcal::empty();
    romcal.calendar = "general_roman".to_string();
    romcal.calendar_definitions.push(general_roman);

    let result = Calendar::new(romcal, 2026);
    assert!(
        result.is_ok(),
        "general_roman with explicit definition should work"
    );
}

#[test]
fn test_grandparent_calendar_not_found_error() {
    // Create hierarchy: child → parent → grandparent (missing)
    let parent_calendar = CalendarDefinition {
        schema: None,
        id: "parent_calendar".to_string(),
        metadata: CalendarMetadata {
            r#type: CalendarType::Region,
            jurisdiction: CalendarJurisdiction::Civil,
        },
        particular_config: None,
        parent_calendar_ids: vec!["missing_grandparent".to_string()],
        days_definitions: std::collections::BTreeMap::new(),
    };

    let child_calendar = CalendarDefinition {
        schema: None,
        id: "child_calendar".to_string(),
        metadata: CalendarMetadata {
            r#type: CalendarType::Country,
            jurisdiction: CalendarJurisdiction::Civil,
        },
        particular_config: None,
        parent_calendar_ids: vec!["parent_calendar".to_string()],
        days_definitions: std::collections::BTreeMap::new(),
    };

    let mut romcal = Romcal::empty();
    romcal.calendar = "child_calendar".to_string();
    romcal.calendar_definitions.push(parent_calendar);
    romcal.calendar_definitions.push(child_calendar);

    let result = Calendar::new(romcal, 2026);

    match result {
        Err(error) => {
            let error_str = error.to_string();
            assert!(
                error_str.contains("missing_grandparent"),
                "Error should mention the missing grandparent: {}",
                error_str
            );
        }
        Ok(_) => panic!("Expected an error for missing grandparent calendar"),
    }
}

#[test]
fn test_multiple_parents_one_missing_error() {
    // Create calendar with multiple parents, one missing
    let existing_parent = CalendarDefinition {
        schema: None,
        id: "existing_parent".to_string(),
        metadata: CalendarMetadata {
            r#type: CalendarType::GeneralRoman,
            jurisdiction: CalendarJurisdiction::Ecclesiastical,
        },
        particular_config: None,
        parent_calendar_ids: vec![],
        days_definitions: std::collections::BTreeMap::new(),
    };

    let child_calendar = CalendarDefinition {
        schema: None,
        id: "child_calendar".to_string(),
        metadata: CalendarMetadata {
            r#type: CalendarType::Country,
            jurisdiction: CalendarJurisdiction::Civil,
        },
        particular_config: None,
        parent_calendar_ids: vec!["existing_parent".to_string(), "missing_parent".to_string()],
        days_definitions: std::collections::BTreeMap::new(),
    };

    let mut romcal = Romcal::empty();
    romcal.calendar = "child_calendar".to_string();
    romcal.calendar_definitions.push(existing_parent);
    romcal.calendar_definitions.push(child_calendar);

    let result = Calendar::new(romcal, 2026);

    match result {
        Err(error) => {
            let error_str = error.to_string();
            assert!(
                error_str.contains("missing_parent"),
                "Error should mention the missing parent: {}",
                error_str
            );
        }
        Ok(_) => panic!("Expected an error when one of multiple parents is missing"),
    }
}

#[test]
fn test_circular_reference_error() {
    // Create circular reference: A → B → A
    let calendar_a = CalendarDefinition {
        schema: None,
        id: "calendar_a".to_string(),
        metadata: CalendarMetadata {
            r#type: CalendarType::GeneralRoman,
            jurisdiction: CalendarJurisdiction::Ecclesiastical,
        },
        particular_config: None,
        parent_calendar_ids: vec!["calendar_b".to_string()],
        days_definitions: std::collections::BTreeMap::new(),
    };

    let calendar_b = CalendarDefinition {
        schema: None,
        id: "calendar_b".to_string(),
        metadata: CalendarMetadata {
            r#type: CalendarType::Region,
            jurisdiction: CalendarJurisdiction::Civil,
        },
        particular_config: None,
        parent_calendar_ids: vec!["calendar_a".to_string()],
        days_definitions: std::collections::BTreeMap::new(),
    };

    let mut romcal = Romcal::empty();
    romcal.calendar = "calendar_a".to_string();
    romcal.calendar_definitions.push(calendar_a);
    romcal.calendar_definitions.push(calendar_b);

    let result = Calendar::new(romcal, 2026);

    match result {
        Err(error) => {
            let error_str = error.to_string();
            assert!(
                error_str.contains("Circular reference"),
                "Error should mention circular reference: {}",
                error_str
            );
        }
        Ok(_) => panic!("Expected an error for circular reference in calendar hierarchy"),
    }
}
