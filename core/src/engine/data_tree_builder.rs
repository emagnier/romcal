// Data tree builder utilities.
//
// This module provides shared structs and functions for building hierarchical data trees
// (calendars and locales) for both build.rs and tests.

use std::collections::HashMap;

// Shared structs for build process
#[derive(Debug, Clone)]
pub struct CalendarInfo {
    pub id: String,
    pub parent_calendar_ids: Vec<String>,
}

// Calendar tree structure for representing calendar hierarchies
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CalendarTreeNode {
    /// The calendar ID.
    pub id: String,
    /// Child calendar nodes.
    pub children: Vec<CalendarTreeNode>,
}

impl CalendarTreeNode {
    /// Create a new calendar tree node.
    pub fn new(id: String) -> Self {
        Self {
            id,
            children: Vec::new(),
        }
    }

    /// Add a child node to this node.
    pub fn add_child(&mut self, child: CalendarTreeNode) {
        self.children.push(child);
    }

    /// Find a node by its ID in the tree.
    pub fn find_by_id(&self, id: &str) -> Option<&CalendarTreeNode> {
        if self.id == id {
            return Some(self);
        }

        for child in &self.children {
            if let Some(found) = child.find_by_id(id) {
                return Some(found);
            }
        }

        None
    }

    /// Get all calendar IDs in the tree as a flat list.
    pub fn get_all_ids(&self) -> Vec<String> {
        let mut ids = vec![self.id.clone()];
        for child in &self.children {
            ids.extend(child.get_all_ids());
        }
        ids
    }
}

// Locale tree structure for representing locale hierarchies following BCP 47
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocaleTreeNode {
    /// The locale code (e.g., "en", "en-gb", "fr").
    pub locale: String,
    /// Child locale nodes.
    pub children: Vec<LocaleTreeNode>,
}

impl LocaleTreeNode {
    /// Create a new locale tree node.
    pub fn new(locale: String) -> Self {
        Self {
            locale,
            children: Vec::new(),
        }
    }

    /// Add a child node to this node.
    pub fn add_child(&mut self, child: LocaleTreeNode) {
        self.children.push(child);
    }

    /// Find a node by its locale code in the tree.
    pub fn find_by_locale(&self, locale: &str) -> Option<&LocaleTreeNode> {
        if self.locale == locale {
            return Some(self);
        }

        for child in &self.children {
            if let Some(found) = child.find_by_locale(locale) {
                return Some(found);
            }
        }

        None
    }

    /// Get all locale codes in the tree as a flat list.
    pub fn get_all_locales(&self) -> Vec<String> {
        let mut locales = vec![self.locale.clone()];
        for child in &self.children {
            locales.extend(child.get_all_locales());
        }
        locales
    }

    /// Check if this node represents a base language (no hyphen in locale code).
    pub fn is_base_language(&self) -> bool {
        !self.locale.contains('-')
    }

    /// Get the base language part of the locale (e.g., "en" from "en-gb").
    pub fn get_base_language(&self) -> &str {
        self.locale.split('-').next().unwrap_or(&self.locale)
    }
}

// Shared functions for build process
pub fn build_calendar_tree(calendars: &[CalendarInfo]) -> CalendarTreeNode {
    let calendar_map: HashMap<String, &CalendarInfo> =
        calendars.iter().map(|cal| (cal.id.clone(), cal)).collect();

    let root_calendar = calendar_map
        .get("general_roman")
        .expect("general_roman calendar not found");

    build_tree_recursive(root_calendar, calendars, &calendar_map)
}

fn build_tree_recursive(
    current: &CalendarInfo,
    all_calendars: &[CalendarInfo],
    _calendar_map: &HashMap<String, &CalendarInfo>,
) -> CalendarTreeNode {
    let mut node = CalendarTreeNode::new(current.id.clone());

    for calendar in all_calendars {
        if let Some(last_parent) = calendar.parent_calendar_ids.last()
            && last_parent == &current.id
        {
            let child_node = build_tree_recursive(calendar, all_calendars, _calendar_map);
            node.add_child(child_node);
        }
    }

    node.children.sort_by(|a, b| a.id.cmp(&b.id));
    node
}

pub fn build_locale_tree(locales: &[String]) -> Vec<LocaleTreeNode> {
    let mut result = Vec::new();

    let mut sorted_locales = locales.to_vec();
    sorted_locales.sort_by_key(|locale| locale.matches('-').count());

    for locale in sorted_locales {
        add_locale_simple(&mut result, &locale);
    }

    result.sort_by(|a, b| a.locale.cmp(&b.locale));
    for node in &mut result {
        sort_locale_tree_children(node);
    }

    result
}

fn add_locale_simple(result: &mut Vec<LocaleTreeNode>, locale: &str) {
    let parts: Vec<&str> = locale.split('-').collect();

    if parts.len() == 1 {
        if !result.iter().any(|node| node.locale == locale) {
            result.push(LocaleTreeNode::new(locale.to_string()));
        }
        return;
    }

    let parent_locale = parts[..parts.len() - 1].join("-");
    let parent_index = find_or_create_parent_index(result, &parent_locale);
    result[parent_index].add_child(LocaleTreeNode::new(locale.to_string()));
}

fn find_or_create_parent_index(result: &mut Vec<LocaleTreeNode>, parent_locale: &str) -> usize {
    for (i, node) in result.iter().enumerate() {
        if node.locale == parent_locale {
            return i;
        }
    }

    result.push(LocaleTreeNode::new(parent_locale.to_string()));
    result.len() - 1
}

fn sort_locale_tree_children(node: &mut LocaleTreeNode) {
    node.children.sort_by(|a, b| a.locale.cmp(&b.locale));
    for child in &mut node.children {
        sort_locale_tree_children(child);
    }
}

pub fn generate_calendar_tree_constant(tree: &CalendarTreeNode) -> String {
    generate_tree_node_json(tree)
}

pub fn generate_locale_tree_constant(trees: &[LocaleTreeNode]) -> String {
    if trees.len() == 1 {
        generate_locale_tree_node_json(&trees[0])
    } else {
        let mut result = String::from("[");
        for (i, tree) in trees.iter().enumerate() {
            if i > 0 {
                result.push(',');
            }
            result.push_str(&generate_locale_tree_node_json(tree));
        }
        result.push(']');
        result
    }
}

fn generate_tree_node_json(node: &CalendarTreeNode) -> String {
    if node.children.is_empty() {
        format!(r#"{{"id":"{}"}}"#, node.id)
    } else {
        let children_json = node
            .children
            .iter()
            .map(generate_tree_node_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(r#"{{"id":"{}","children":[{}]}}"#, node.id, children_json)
    }
}

fn generate_locale_tree_node_json(node: &LocaleTreeNode) -> String {
    if node.children.is_empty() {
        format!(r#"{{"locale":"{}"}}"#, node.locale)
    } else {
        let children_json = node
            .children
            .iter()
            .map(generate_locale_tree_node_json)
            .collect::<Vec<_>>()
            .join(",");
        format!(
            r#"{{"locale":"{}","children":[{}]}}"#,
            node.locale, children_json
        )
    }
}

// Helper functions for testing
pub fn collect_all_children(node: &LocaleTreeNode) -> Vec<String> {
    let mut result = Vec::new();
    for child in &node.children {
        result.push(child.locale.clone());
        result.extend(collect_all_children(child));
    }
    result
}

// JSON validation helpers
pub fn is_valid_json(json: &str) -> bool {
    serde_json::from_str::<serde_json::Value>(json).is_ok()
}

pub fn validate_calendar_tree_json(json: &str) -> Result<(), String> {
    if !is_valid_json(json) {
        return Err("Invalid JSON format".to_string());
    }

    let value: serde_json::Value = serde_json::from_str(json).unwrap();

    if !value.is_object() {
        return Err("Root should be an object".to_string());
    }

    let obj = value.as_object().unwrap();
    if !obj.contains_key("id") {
        return Err("Missing 'id' field".to_string());
    }

    if obj.contains_key("children") {
        let children = &obj["children"];
        if !children.is_array() {
            return Err("'children' should be an array".to_string());
        }
    }

    Ok(())
}

pub fn validate_locale_tree_json(json: &str) -> Result<(), String> {
    if !is_valid_json(json) {
        return Err("Invalid JSON format".to_string());
    }

    let value: serde_json::Value = serde_json::from_str(json).unwrap();

    if !value.is_array() {
        return Err("Root should be an array".to_string());
    }

    let array = value.as_array().unwrap();
    for item in array {
        if !item.is_object() {
            return Err("Each item should be an object".to_string());
        }

        let obj = item.as_object().unwrap();
        if !obj.contains_key("locale") {
            return Err("Missing 'locale' field".to_string());
        }
    }

    Ok(())
}
