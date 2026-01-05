//! Entity query definition for searching entities.

use crate::types::entity::{CanonizationLevel, EntityType, Sex, Title};

/// Query parameters for searching entities.
///
/// All fields are optional. When a field is `None`, it is not used for filtering.
/// When `text` is provided, fuzzy matching is performed on entity ID, fullname, and name.
#[derive(Debug, Clone, Default)]
pub struct EntityQuery {
    /// Fuzzy text search on id, fullname, and name fields.
    pub text: Option<String>,

    /// Filter by entity type (Person, Place, Event).
    pub entity_type: Option<EntityType>,

    /// Filter by canonization level (Saint, Blessed).
    pub canonization_level: Option<CanonizationLevel>,

    /// Filter by sex (Male, Female).
    pub sex: Option<Sex>,

    /// Filter by titles. Entity must have at least one of the specified titles.
    pub titles: Option<Vec<Title>>,

    /// Maximum number of results to return.
    /// Default: 20 (applied in search logic).
    pub limit: Option<usize>,

    /// Minimum score threshold (0.0 to 1.0).
    /// Default: 0.3 (applied in search logic).
    pub min_score: Option<f64>,
}

impl EntityQuery {
    /// Returns the effective limit, defaulting to 20 if not specified.
    pub fn effective_limit(&self) -> usize {
        self.limit.unwrap_or(20)
    }

    /// Returns the effective minimum score, defaulting to 0.3 if not specified.
    pub fn effective_min_score(&self) -> f64 {
        self.min_score.unwrap_or(0.3)
    }

    /// Returns true if this query has any text to search for.
    pub fn has_text(&self) -> bool {
        self.text.as_ref().is_some_and(|t| !t.trim().is_empty())
    }

    /// Returns true if this query has any filters set.
    pub fn has_filters(&self) -> bool {
        self.entity_type.is_some()
            || self.canonization_level.is_some()
            || self.sex.is_some()
            || self.titles.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_query() {
        let query = EntityQuery::default();
        assert!(query.text.is_none());
        assert!(query.entity_type.is_none());
        assert!(query.canonization_level.is_none());
        assert!(query.sex.is_none());
        assert!(query.titles.is_none());
        assert!(query.limit.is_none());
        assert!(query.min_score.is_none());
    }

    #[test]
    fn test_effective_defaults() {
        let query = EntityQuery::default();
        assert_eq!(query.effective_limit(), 20);
        assert!((query.effective_min_score() - 0.3).abs() < f64::EPSILON);
    }

    #[test]
    fn test_has_text() {
        let query = EntityQuery::default();
        assert!(!query.has_text());

        let query = EntityQuery {
            text: Some("".into()),
            ..Default::default()
        };
        assert!(!query.has_text());

        let query = EntityQuery {
            text: Some("   ".into()),
            ..Default::default()
        };
        assert!(!query.has_text());

        let query = EntityQuery {
            text: Some("francis".into()),
            ..Default::default()
        };
        assert!(query.has_text());
    }

    #[test]
    fn test_has_filters() {
        let query = EntityQuery::default();
        assert!(!query.has_filters());

        let query = EntityQuery {
            entity_type: Some(EntityType::Person),
            ..Default::default()
        };
        assert!(query.has_filters());

        let query = EntityQuery {
            canonization_level: Some(CanonizationLevel::Saint),
            ..Default::default()
        };
        assert!(query.has_filters());
    }
}
