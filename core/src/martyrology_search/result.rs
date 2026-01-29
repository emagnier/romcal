//! Search result types for martyrology searches.

use serde::{Deserialize, Serialize};
use strum::Display;

#[cfg(feature = "ts-bindings")]
use ts_rs::TS;

use crate::types::martyrology::MartyrologyEntry;

/// Type of match that was found for a search result.
#[derive(Debug, Clone, PartialEq, Eq, Display, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[cfg_attr(feature = "ts-bindings", derive(TS))]
#[cfg_attr(feature = "ts-bindings", ts(export))]
pub enum MatchType {
    /// Exact ID match (score = 1.0).
    ExactId,
    /// Fuzzy match on text fields (score < 1.0).
    Fuzzy,
    /// Match by filters only (no text query provided).
    FilterOnly,
}

/// Result of a martyrology search.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "ts-bindings", derive(TS))]
#[cfg_attr(feature = "ts-bindings", ts(export, rename_all = "snake_case"))]
pub struct MartyrologySearchResult {
    /// The matched martyrology entry.
    pub entry: MartyrologyEntry,
    /// Match score from 0.0 to 1.0, where 1.0 is a perfect match.
    pub score: f64,
    /// Type of match that was found.
    pub match_type: MatchType,
    /// Names of fields that matched the query.
    pub matched_fields: Vec<String>,
}

impl MartyrologySearchResult {
    /// Create a new search result with exact ID match.
    pub fn exact_id(entry: MartyrologyEntry) -> Self {
        Self {
            entry,
            score: 1.0,
            match_type: MatchType::ExactId,
            matched_fields: vec!["id".to_string()],
        }
    }

    /// Create a new search result with fuzzy match.
    pub fn fuzzy(entry: MartyrologyEntry, score: f64, matched_fields: Vec<String>) -> Self {
        Self {
            entry,
            score,
            match_type: MatchType::Fuzzy,
            matched_fields,
        }
    }

    /// Create a new search result matched by filters only.
    pub fn filter_only(entry: MartyrologyEntry) -> Self {
        Self {
            entry,
            score: 1.0,
            match_type: MatchType::FilterOnly,
            matched_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::martyrology::MartyrologyEntryDef;

    fn create_test_entry() -> MartyrologyEntry {
        MartyrologyEntry::new("test_id".to_string(), MartyrologyEntryDef::default())
    }

    #[test]
    fn test_exact_id_result() {
        let entry = create_test_entry();
        let result = MartyrologySearchResult::exact_id(entry);

        assert_eq!(result.score, 1.0);
        assert_eq!(result.match_type, MatchType::ExactId);
        assert_eq!(result.matched_fields, vec!["id".to_string()]);
    }

    #[test]
    fn test_fuzzy_result() {
        let entry = create_test_entry();
        let result = MartyrologySearchResult::fuzzy(
            entry,
            0.85,
            vec!["fullname".to_string(), "name".to_string()],
        );

        assert!((result.score - 0.85).abs() < f64::EPSILON);
        assert_eq!(result.match_type, MatchType::Fuzzy);
        assert_eq!(result.matched_fields.len(), 2);
    }

    #[test]
    fn test_filter_only_result() {
        let entry = create_test_entry();
        let result = MartyrologySearchResult::filter_only(entry);

        assert_eq!(result.score, 1.0);
        assert_eq!(result.match_type, MatchType::FilterOnly);
        assert!(result.matched_fields.is_empty());
    }

    #[test]
    fn test_match_type_display() {
        assert_eq!(MatchType::ExactId.to_string(), "exact_id");
        assert_eq!(MatchType::Fuzzy.to_string(), "fuzzy");
        assert_eq!(MatchType::FilterOnly.to_string(), "filter_only");
    }
}
