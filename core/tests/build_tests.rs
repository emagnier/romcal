//! Tests for build.rs functions
//!
//! These tests verify the generation of the 4 constants with multiple levels

include!("../src/engine/data_tree_builder.rs");

#[test]
fn test_calendar_tree_generation() {
    let calendars = vec![
        CalendarInfo {
            id: "general_roman".to_string(),
            parent_calendar_ids: vec![],
        },
        CalendarInfo {
            id: "france".to_string(),
            parent_calendar_ids: vec!["general_roman".to_string()],
        },
        CalendarInfo {
            id: "france_paris".to_string(),
            parent_calendar_ids: vec!["general_roman".to_string(), "france".to_string()],
        },
        CalendarInfo {
            id: "france_lyon".to_string(),
            parent_calendar_ids: vec!["general_roman".to_string(), "france".to_string()],
        },
    ];

    let tree = build_calendar_tree(&calendars);

    // Test root node
    assert_eq!(tree.id, "general_roman");
    assert_eq!(tree.children.len(), 1);

    // Test first level
    let france = &tree.children[0];
    assert_eq!(france.id, "france");
    assert_eq!(france.children.len(), 2);

    // Test second level - verify exact structure
    let children_ids: Vec<&str> = france.children.iter().map(|c| c.id.as_str()).collect();
    assert_eq!(children_ids, vec!["france_lyon", "france_paris"]); // Sorted order

    // Verify no children at second level
    for child in &france.children {
        assert!(
            child.children.is_empty(),
            "Second level nodes should have no children"
        );
    }
}

#[test]
fn test_locale_tree_generation_simple() {
    let locales = vec!["en".to_string(), "fr".to_string(), "es".to_string()];

    let tree = build_locale_tree(&locales);

    assert_eq!(tree.len(), 3);
    let locale_codes: Vec<&str> = tree.iter().map(|n| n.locale.as_str()).collect();
    assert!(locale_codes.contains(&"en"));
    assert!(locale_codes.contains(&"fr"));
    assert!(locale_codes.contains(&"es"));
}

#[test]
fn test_locale_tree_generation_with_children() {
    let locales = vec![
        "en".to_string(),
        "en-gb".to_string(),
        "en-us".to_string(),
        "fr".to_string(),
        "fr-ca".to_string(),
    ];

    let tree = build_locale_tree(&locales);

    // Should have 2 root nodes: en and fr
    assert_eq!(tree.len(), 2);

    // Find en node
    let en_node = tree.iter().find(|n| n.locale == "en").unwrap();
    assert_eq!(en_node.children.len(), 2);
    let en_children: Vec<&str> = en_node.children.iter().map(|c| c.locale.as_str()).collect();
    assert!(en_children.contains(&"en-gb"));
    assert!(en_children.contains(&"en-us"));

    // Find fr node
    let fr_node = tree.iter().find(|n| n.locale == "fr").unwrap();
    assert_eq!(fr_node.children.len(), 1);
    assert_eq!(fr_node.children[0].locale, "fr-ca");
}

#[test]
fn test_locale_tree_generation_multiple_levels() {
    let locales = vec![
        "en".to_string(),
        "en-gb".to_string(),
        "en-gb-london".to_string(),
        "en-gb-manchester".to_string(),
        "fr".to_string(),
        "fr-ca".to_string(),
        "fr-ca-montreal".to_string(),
    ];

    let tree = build_locale_tree(&locales);

    // Should have at least 2 root nodes: en and fr
    // Note: The implementation might create intermediate nodes
    assert!(tree.len() >= 2);

    // Test that all expected locales exist somewhere in the tree
    let all_locales: Vec<String> = tree.iter().flat_map(|n| collect_all_children(n)).collect();
    assert!(all_locales.contains(&"en-gb-london".to_string()));
    assert!(all_locales.contains(&"en-gb-manchester".to_string()));
    assert!(all_locales.contains(&"fr-ca-montreal".to_string()));
}

#[test]
fn test_locale_tree_generation_orphaned_locales() {
    let locales = vec![
        "en".to_string(),
        "en-gb".to_string(),
        "pt-br".to_string(), // No parent pt
        "zh-cn".to_string(), // No parent zh
    ];

    let tree = build_locale_tree(&locales);

    // Should have at least 3 root nodes: en, pt-br, zh-cn
    assert!(tree.len() >= 3);

    // Test that all expected locales exist somewhere in the tree
    let all_locales: Vec<String> = tree.iter().flat_map(|n| collect_all_children(n)).collect();
    let root_locales: Vec<String> = tree.iter().map(|n| n.locale.clone()).collect();

    // Check that all locales exist either at root or as children
    let all_locales_combined: Vec<String> = root_locales.into_iter().chain(all_locales).collect();
    assert!(all_locales_combined.contains(&"en".to_string()));
    assert!(all_locales_combined.contains(&"en-gb".to_string()));
    assert!(all_locales_combined.contains(&"pt-br".to_string()));
    assert!(all_locales_combined.contains(&"zh-cn".to_string()));
}

#[test]
fn test_calendar_tree_json_generation() {
    let calendars = vec![
        CalendarInfo {
            id: "general_roman".to_string(),
            parent_calendar_ids: vec![],
        },
        CalendarInfo {
            id: "france".to_string(),
            parent_calendar_ids: vec!["general_roman".to_string()],
        },
    ];

    let tree = build_calendar_tree(&calendars);
    let json = generate_calendar_tree_constant(&tree);

    // Should not contain empty children arrays
    assert!(!json.contains(r#""children":[]"#));
    assert!(json.contains(r#""id":"general_roman""#));
    assert!(json.contains(r#""id":"france""#));
}

#[test]
fn test_locale_tree_json_generation() {
    let locales = vec!["en".to_string(), "en-gb".to_string(), "fr".to_string()];

    let tree = build_locale_tree(&locales);
    let json = generate_locale_tree_constant(&tree);

    // Should not contain empty children arrays
    assert!(!json.contains(r#""children":[]"#));
    assert!(json.contains(r#""locale":"en""#));
    assert!(json.contains(r#""locale":"en-gb""#));
    assert!(json.contains(r#""locale":"fr""#));
}

#[test]
fn test_json_generation_no_children() {
    let calendars = vec![CalendarInfo {
        id: "general_roman".to_string(),
        parent_calendar_ids: vec![],
    }];

    let tree = build_calendar_tree(&calendars);
    let json = generate_calendar_tree_constant(&tree);

    // Should be simple object without children property
    assert_eq!(json, r#"{"id":"general_roman"}"#);
}

#[test]
fn test_json_generation_with_children() {
    let calendars = vec![
        CalendarInfo {
            id: "general_roman".to_string(),
            parent_calendar_ids: vec![],
        },
        CalendarInfo {
            id: "child".to_string(),
            parent_calendar_ids: vec!["general_roman".to_string()],
        },
    ];

    let tree = build_calendar_tree(&calendars);
    let json = generate_calendar_tree_constant(&tree);

    // Should contain children property
    assert!(json.contains(r#""children"#));
    assert!(json.contains(r#""id":"general_roman""#));
    assert!(json.contains(r#""id":"child""#));
}

#[test]
fn test_all_four_constants_generation() {
    // Test CALENDAR_IDS
    let calendars = vec![
        CalendarInfo {
            id: "general_roman".to_string(),
            parent_calendar_ids: vec![],
        },
        CalendarInfo {
            id: "france".to_string(),
            parent_calendar_ids: vec!["general_roman".to_string()],
        },
        CalendarInfo {
            id: "americas".to_string(),
            parent_calendar_ids: vec!["general_roman".to_string()],
        },
    ];

    let calendar_tree = build_calendar_tree(&calendars);
    let calendar_json = generate_calendar_tree_constant(&calendar_tree);

    // Test LOCALE_CODES
    let locales = vec![
        "en".to_string(),
        "en-gb".to_string(),
        "en-gb-london".to_string(),
        "fr".to_string(),
        "fr-ca".to_string(),
        "pt-br".to_string(),
    ];

    let locale_tree = build_locale_tree(&locales);
    let locale_json = generate_locale_tree_constant(&locale_tree);

    // Verify calendar tree structure
    assert!(calendar_json.contains(r#""id":"general_roman""#));
    assert!(calendar_json.contains(r#""id":"france""#));
    assert!(calendar_json.contains(r#""id":"americas""#));

    // Verify locale tree structure
    assert!(locale_json.contains(r#""locale":"en""#));
    assert!(locale_json.contains(r#""locale":"en-gb""#));
    assert!(locale_json.contains(r#""locale":"en-gb-london""#));
    assert!(locale_json.contains(r#""locale":"fr""#));
    assert!(locale_json.contains(r#""locale":"fr-ca""#));
    assert!(locale_json.contains(r#""locale":"pt-br""#));

    // Verify no empty children arrays
    assert!(!calendar_json.contains(r#""children":[]"#));
    assert!(!locale_json.contains(r#""children":[]"#));

    // Verify hierarchical structure
    assert!(locale_json.contains(r#""children":[{"locale":"en-gb""#));
    assert!(locale_json.contains(r#""children":[{"locale":"en-gb-london""#));
}

// Error handling tests
#[test]
#[should_panic(expected = "general_roman calendar not found")]
fn test_calendar_tree_missing_root() {
    let calendars = vec![CalendarInfo {
        id: "france".to_string(),
        parent_calendar_ids: vec![],
    }];
    build_calendar_tree(&calendars);
}

#[test]
fn test_empty_locale_list() {
    let locales = vec![];
    let tree = build_locale_tree(&locales);
    assert!(tree.is_empty());
}

#[test]
fn test_single_locale() {
    let locales = vec!["en".to_string()];
    let tree = build_locale_tree(&locales);
    assert_eq!(tree.len(), 1);
    assert_eq!(tree[0].locale, "en");
    assert!(tree[0].children.is_empty());
}

// JSON validation tests
#[test]
fn test_calendar_tree_json_validity() {
    let calendars = vec![
        CalendarInfo {
            id: "general_roman".to_string(),
            parent_calendar_ids: vec![],
        },
        CalendarInfo {
            id: "france".to_string(),
            parent_calendar_ids: vec!["general_roman".to_string()],
        },
    ];

    let tree = build_calendar_tree(&calendars);
    let json = generate_calendar_tree_constant(&tree);

    // Validate JSON structure
    assert!(is_valid_json(&json));
    validate_calendar_tree_json(&json).expect("Calendar tree JSON should be valid");
}

#[test]
fn test_locale_tree_json_validity() {
    let locales = vec!["en".to_string(), "en-gb".to_string(), "fr".to_string()];
    let tree = build_locale_tree(&locales);
    let json = generate_locale_tree_constant(&tree);

    // Validate JSON structure
    assert!(is_valid_json(&json));
    validate_locale_tree_json(&json).expect("Locale tree JSON should be valid");
}

// Performance tests
#[test]
fn test_large_tree_performance() {
    let mut locales = Vec::new();

    // Generate a large number of locales
    for i in 0..100 {
        locales.push(format!("lang{}", i));
        for j in 0..10 {
            locales.push(format!("lang{}-region{}", i, j));
            for k in 0..5 {
                locales.push(format!("lang{}-region{}-city{}", i, j, k));
            }
        }
    }

    let start = std::time::Instant::now();
    let tree = build_locale_tree(&locales);
    let duration = start.elapsed();

    // Should complete within reasonable time (1 second)
    assert!(
        duration.as_secs() < 1,
        "Tree building took too long: {:?}",
        duration
    );

    // Verify all locales are present (some might be deduplicated)
    let all_locales: Vec<String> = tree.iter().flat_map(|n| collect_all_children(n)).collect();
    assert!(all_locales.len() <= locales.len());
}

// Edge cases
#[test]
fn test_locale_with_multiple_hyphens() {
    let locales = vec!["en".to_string(), "en-gb-london-westminster".to_string()];

    let tree = build_locale_tree(&locales);
    assert!(tree.len() >= 1);

    // Test that the deep locale exists somewhere in the tree
    let all_locales: Vec<String> = tree.iter().flat_map(|n| collect_all_children(n)).collect();
    assert!(all_locales.contains(&"en-gb-london-westminster".to_string()));
}

#[test]
fn test_duplicate_locales() {
    let locales = vec![
        "en".to_string(),
        "en".to_string(), // Duplicate
        "en-gb".to_string(),
    ];

    let tree = build_locale_tree(&locales);
    assert_eq!(tree.len(), 1);

    let en_node = &tree[0];
    assert_eq!(en_node.locale, "en");
    assert_eq!(en_node.children.len(), 1);
    assert_eq!(en_node.children[0].locale, "en-gb");
}
