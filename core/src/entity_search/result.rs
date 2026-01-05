//! Search result types for entity searches.

use crate::types::entity::Entity;
use strum::Display;

/// Type of match that was found for a search result.
#[derive(Debug, Clone, PartialEq, Eq, Display)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum MatchType {
    /// Exact ID match (score = 1.0).
    ExactId,
    /// Fuzzy match on text fields (score < 1.0).
    Fuzzy,
    /// Match by filters only (no text query provided).
    FilterOnly,
}

/// Result of an entity search.
#[derive(Debug, Clone)]
pub struct EntitySearchResult {
    /// The matched entity.
    pub entity: Entity,
    /// Match score from 0.0 to 1.0, where 1.0 is a perfect match.
    pub score: f64,
    /// Type of match that was found.
    pub match_type: MatchType,
    /// Names of fields that matched the query.
    pub matched_fields: Vec<String>,
}

impl EntitySearchResult {
    /// Create a new search result with exact ID match.
    pub fn exact_id(entity: Entity) -> Self {
        Self {
            entity,
            score: 1.0,
            match_type: MatchType::ExactId,
            matched_fields: vec!["id".to_string()],
        }
    }

    /// Create a new search result with fuzzy match.
    pub fn fuzzy(entity: Entity, score: f64, matched_fields: Vec<String>) -> Self {
        Self {
            entity,
            score,
            match_type: MatchType::Fuzzy,
            matched_fields,
        }
    }

    /// Create a new search result matched by filters only.
    pub fn filter_only(entity: Entity) -> Self {
        Self {
            entity,
            score: 1.0,
            match_type: MatchType::FilterOnly,
            matched_fields: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::entity::EntityDefinition;

    fn create_test_entity() -> Entity {
        Entity::new("test_id".to_string(), EntityDefinition::default())
    }

    #[test]
    fn test_exact_id_result() {
        let entity = create_test_entity();
        let result = EntitySearchResult::exact_id(entity);

        assert_eq!(result.score, 1.0);
        assert_eq!(result.match_type, MatchType::ExactId);
        assert_eq!(result.matched_fields, vec!["id".to_string()]);
    }

    #[test]
    fn test_fuzzy_result() {
        let entity = create_test_entity();
        let result = EntitySearchResult::fuzzy(
            entity,
            0.85,
            vec!["fullname".to_string(), "name".to_string()],
        );

        assert!((result.score - 0.85).abs() < f64::EPSILON);
        assert_eq!(result.match_type, MatchType::Fuzzy);
        assert_eq!(result.matched_fields.len(), 2);
    }

    #[test]
    fn test_filter_only_result() {
        let entity = create_test_entity();
        let result = EntitySearchResult::filter_only(entity);

        assert_eq!(result.score, 1.0);
        assert_eq!(result.match_type, MatchType::FilterOnly);
        assert!(result.matched_fields.is_empty());
    }

    #[test]
    fn test_match_type_display() {
        assert_eq!(MatchType::ExactId.to_string(), "EXACT_ID");
        assert_eq!(MatchType::Fuzzy.to_string(), "FUZZY");
        assert_eq!(MatchType::FilterOnly.to_string(), "FILTER_ONLY");
    }
}
