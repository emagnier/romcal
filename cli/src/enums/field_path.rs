use indexmap::IndexMap;
use std::fmt;
use std::str::FromStr;

/// A path to a field in a JSON structure, supporting dot notation for nested fields.
///
/// Examples:
/// - `"id"` → access the `id` field
/// - `"colors.key"` → access the `key` field inside each element of `colors` array
/// - `"optional_celebrations.id"` → extract `id` from each element in the array
///
/// Note: Only single-level nesting is supported (e.g., `colors.key`).
/// Deeper paths like `a.b.c` will only extract field `b` from `a`, ignoring `c`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldPath {
    /// The segments of the path (e.g., ["colors", "key"])
    segments: Vec<String>,
    /// The original raw string (for display purposes)
    raw: String,
}

impl FieldPath {
    /// Returns the root (first) field name
    pub fn root_field(&self) -> &str {
        &self.segments[0]
    }

    /// Returns the nested field name (for single-level nesting).
    /// For "colors.key", returns Some("key").
    /// For "id", returns None.
    pub fn nested_field(&self) -> Option<&str> {
        if self.segments.len() > 1 {
            Some(&self.segments[1])
        } else {
            None
        }
    }
}

/// Extract filtered fields from a JSON value using multiple FieldPaths.
///
/// Groups filters by root field and merges nested field selections.
/// For nested paths, preserves object structure with only requested fields.
///
/// Example:
/// - Filters: `["colors.key", "colors.name"]`
/// - Input: `{colors: [{key: "WHITE", name: "Blanc", hex: "#FFF"}]}`
/// - Output: `{colors: [{key: "WHITE", name: "Blanc"}]}`
pub fn extract_filtered(
    value: &serde_json::Value,
    filters: &[FieldPath],
) -> serde_json::Map<String, serde_json::Value> {
    // Group filters by root field, preserving insertion order
    let mut groups: IndexMap<&str, Vec<&FieldPath>> = IndexMap::new();
    for filter in filters {
        groups.entry(filter.root_field()).or_default().push(filter);
    }

    let mut result = serde_json::Map::new();

    for (root, paths) in &groups {
        if let Some(root_value) = value.get(root) {
            // Collect nested field names (if any)
            let nested_fields: Vec<&str> = paths.iter().filter_map(|p| p.nested_field()).collect();

            if nested_fields.is_empty() {
                // No nested fields, copy value as-is
                result.insert(root.to_string(), root_value.clone());
            } else {
                // Filter nested fields from array/object
                if let Some(filtered) = filter_nested_fields(root_value, &nested_fields) {
                    result.insert(root.to_string(), filtered);
                }
            }
        }
    }

    result
}

/// Filter nested fields from an array or object.
fn filter_nested_fields(value: &serde_json::Value, fields: &[&str]) -> Option<serde_json::Value> {
    match value {
        serde_json::Value::Array(arr) => {
            let results: Vec<serde_json::Value> = arr
                .iter()
                .filter_map(|item| filter_object(item, fields))
                .collect();

            if results.is_empty() {
                None
            } else {
                Some(serde_json::Value::Array(results))
            }
        }
        serde_json::Value::Object(_) => filter_object(value, fields),
        _ => None,
    }
}

/// Filter an object to keep only specified fields.
fn filter_object(value: &serde_json::Value, fields: &[&str]) -> Option<serde_json::Value> {
    let obj = value.as_object()?;

    let mut result = serde_json::Map::new();
    for field in fields {
        if let Some(v) = obj.get(*field) {
            result.insert(field.to_string(), v.clone());
        }
    }

    if result.is_empty() {
        None
    } else {
        Some(serde_json::Value::Object(result))
    }
}

impl FromStr for FieldPath {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let normalized = s.replace('-', "_");
        let segments: Vec<String> = normalized.split('.').map(|s| s.to_string()).collect();

        if segments.is_empty() || segments.iter().any(|seg| seg.is_empty()) {
            return Err(format!("Invalid field path: {}", s));
        }

        Ok(FieldPath {
            segments,
            raw: s.to_string(),
        })
    }
}

impl fmt::Display for FieldPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_simple_field() {
        let path: FieldPath = "id".parse().unwrap();
        assert_eq!(path.root_field(), "id");
        assert_eq!(path.nested_field(), None);
    }

    #[test]
    fn test_nested_field() {
        let path: FieldPath = "colors.key".parse().unwrap();
        assert_eq!(path.root_field(), "colors");
        assert_eq!(path.nested_field(), Some("key"));
    }

    #[test]
    fn test_extract_simple_fields() {
        let filters: Vec<FieldPath> = vec!["id".parse().unwrap(), "fullname".parse().unwrap()];
        let value = json!({"id": "test_id", "fullname": "Test Name", "rank": "MEMORIAL"});
        let result = extract_filtered(&value, &filters);

        assert_eq!(result.get("id"), Some(&json!("test_id")));
        assert_eq!(result.get("fullname"), Some(&json!("Test Name")));
        assert_eq!(result.get("rank"), None);
    }

    #[test]
    fn test_extract_nested_single_field() {
        let filters: Vec<FieldPath> = vec!["colors.key".parse().unwrap()];
        let value = json!({
            "colors": [
                {"key": "WHITE", "name": "Blanc", "hex": "#FFF"},
                {"key": "RED", "name": "Rouge", "hex": "#F00"}
            ]
        });
        let result = extract_filtered(&value, &filters);

        assert_eq!(
            result.get("colors"),
            Some(&json!([{"key": "WHITE"}, {"key": "RED"}]))
        );
    }

    #[test]
    fn test_extract_nested_multiple_fields() {
        let filters: Vec<FieldPath> = vec![
            "colors.key".parse().unwrap(),
            "colors.name".parse().unwrap(),
        ];
        let value = json!({
            "colors": [
                {"key": "WHITE", "name": "Blanc", "hex": "#FFF"},
                {"key": "RED", "name": "Rouge", "hex": "#F00"}
            ]
        });
        let result = extract_filtered(&value, &filters);

        assert_eq!(
            result.get("colors"),
            Some(&json!([
                {"key": "WHITE", "name": "Blanc"},
                {"key": "RED", "name": "Rouge"}
            ]))
        );
    }

    #[test]
    fn test_extract_mixed_simple_and_nested() {
        let filters: Vec<FieldPath> = vec!["id".parse().unwrap(), "colors.key".parse().unwrap()];
        let value = json!({
            "id": "test",
            "colors": [{"key": "WHITE", "name": "Blanc"}]
        });
        let result = extract_filtered(&value, &filters);

        assert_eq!(result.get("id"), Some(&json!("test")));
        assert_eq!(result.get("colors"), Some(&json!([{"key": "WHITE"}])));
    }

    #[test]
    fn test_extract_nested_from_object() {
        let filters: Vec<FieldPath> = vec!["metadata.version".parse().unwrap()];
        let value = json!({
            "metadata": {"version": "1.0", "author": "test"}
        });
        let result = extract_filtered(&value, &filters);

        assert_eq!(result.get("metadata"), Some(&json!({"version": "1.0"})));
    }

    #[test]
    fn test_extract_missing_field() {
        let filters: Vec<FieldPath> = vec!["nonexistent".parse().unwrap()];
        let value = json!({"id": "test"});
        let result = extract_filtered(&value, &filters);

        assert!(result.is_empty());
    }

    #[test]
    fn test_extract_empty_array() {
        let filters: Vec<FieldPath> = vec!["colors.key".parse().unwrap()];
        let value = json!({"colors": []});
        let result = extract_filtered(&value, &filters);

        // Empty array after filtering returns None, so field is not included
        assert!(result.get("colors").is_none());
    }

    #[test]
    fn test_dash_to_underscore() {
        let path: FieldPath = "is-optional".parse().unwrap();
        assert_eq!(path.root_field(), "is_optional");
        // Display preserves original
        assert_eq!(path.to_string(), "is-optional");
    }
}
