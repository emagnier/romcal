//! Entity matcher with fuzzy search support using Jaro-Winkler similarity.

use crate::types::entity::{Entity, Title};

use super::query::EntityQuery;
use super::result::EntitySearchResult;

/// Normalize a string for comparison: lowercase and remove diacritics.
fn normalize(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        let c = c.to_lowercase().next().unwrap_or(c);
        match c {
            // Ligatures → expand to multiple characters
            'æ' => result.push_str("ae"),
            'œ' => result.push_str("oe"),
            'ß' => result.push_str("ss"),
            // Accented characters → base form
            'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' | 'ā' | 'ă' | 'ą' => result.push('a'),
            'è' | 'é' | 'ê' | 'ë' | 'ē' | 'ĕ' | 'ė' | 'ę' | 'ě' => result.push('e'),
            'ì' | 'í' | 'î' | 'ï' | 'ĩ' | 'ī' | 'ĭ' | 'į' | 'ı' => result.push('i'),
            'ò' | 'ó' | 'ô' | 'õ' | 'ö' | 'ø' | 'ō' | 'ŏ' | 'ő' => result.push('o'),
            'ù' | 'ú' | 'û' | 'ü' | 'ũ' | 'ū' | 'ŭ' | 'ů' | 'ű' | 'ų' => result.push('u'),
            'ý' | 'ÿ' | 'ŷ' => result.push('y'),
            'ñ' | 'ń' | 'ņ' | 'ň' => result.push('n'),
            'ç' | 'ć' | 'ĉ' | 'ċ' | 'č' => result.push('c'),
            'ś' | 'ŝ' | 'ş' | 'š' => result.push('s'),
            'ź' | 'ż' | 'ž' => result.push('z'),
            'ð' | 'ď' | 'đ' => result.push('d'),
            'ł' | 'ĺ' | 'ļ' | 'ľ' => result.push('l'),
            'ŕ' | 'ř' => result.push('r'),
            'ť' | 'ţ' => result.push('t'),
            'ğ' | 'ĝ' | 'ġ' | 'ģ' => result.push('g'),
            'ĥ' => result.push('h'),
            'ĵ' => result.push('j'),
            'ķ' => result.push('k'),
            'ŵ' => result.push('w'),
            'þ' => result.push('t'),
            _ => result.push(c),
        }
    }
    result
}

/// Entity matcher that performs fuzzy search on entities.
#[derive(Default)]
pub struct EntityMatcher;

impl EntityMatcher {
    /// Create a new entity matcher.
    pub fn new() -> Self {
        Self
    }

    /// Search entities with the given query.
    ///
    /// Returns a list of matching entities sorted by score (highest first).
    pub fn search<'a>(
        &self,
        entities: impl Iterator<Item = &'a Entity>,
        query: &EntityQuery,
    ) -> Vec<EntitySearchResult> {
        let limit = query.effective_limit();
        let min_score = query.effective_min_score();

        // Collect all matching entities
        let mut results: Vec<EntitySearchResult> = entities
            .filter_map(|entity| self.match_entity(entity, query))
            .filter(|result| result.score >= min_score)
            .collect();

        // Sort by score (highest first), then by ID for stability
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.entity.id.cmp(&b.entity.id))
        });

        // Apply limit
        results.truncate(limit);

        results
    }

    /// Match a single entity against the query.
    ///
    /// Returns `None` if the entity doesn't match the query filters or text.
    fn match_entity(&self, entity: &Entity, query: &EntityQuery) -> Option<EntitySearchResult> {
        // Apply filters first (fast rejection)
        if !self.matches_filters(entity, query) {
            return None;
        }

        // If no text query, return as filter-only match
        if !query.has_text() {
            return Some(EntitySearchResult::filter_only(entity.clone()));
        }

        let search_text = query.text.as_ref().unwrap();

        // Check for exact ID match first
        if entity.id.eq_ignore_ascii_case(search_text) {
            return Some(EntitySearchResult::exact_id(entity.clone()));
        }

        // Perform fuzzy matching on text fields
        self.fuzzy_match(entity, search_text, query.effective_min_score())
    }

    /// Check if entity matches all query filters.
    fn matches_filters(&self, entity: &Entity, query: &EntityQuery) -> bool {
        // Filter by entity type
        if let Some(ref query_type) = query.entity_type
            && &entity.r#type != query_type
        {
            return false;
        }

        // Filter by canonization level
        if let Some(ref query_level) = query.canonization_level
            && entity.canonization_level.as_ref() != Some(query_level)
        {
            return false;
        }

        // Filter by sex
        if let Some(ref query_sex) = query.sex
            && entity.sex.as_ref() != Some(query_sex)
        {
            return false;
        }

        // Filter by titles (must have at least one matching title)
        if let Some(ref query_titles) = query.titles
            && !self.has_matching_title(entity, query_titles)
        {
            return false;
        }

        true
    }

    /// Check if entity has at least one of the specified titles.
    fn has_matching_title(&self, entity: &Entity, query_titles: &[Title]) -> bool {
        entity.titles.as_ref().is_some_and(|entity_titles| {
            entity_titles
                .iter()
                .any(|title| query_titles.contains(title))
        })
    }

    /// Perform fuzzy matching on entity text fields using Jaro-Winkler similarity.
    fn fuzzy_match(
        &self,
        entity: &Entity,
        search_text: &str,
        min_score: f64,
    ) -> Option<EntitySearchResult> {
        let search_normalized = normalize(search_text);
        let mut best_score: f64 = 0.0;
        let mut matched_fields = Vec::new();

        // Match against ID
        let score = strsim::jaro_winkler(&search_normalized, &normalize(&entity.id));
        if score > best_score {
            best_score = score;
        }
        if score >= min_score {
            matched_fields.push("id".to_string());
        }

        // Match against fullname
        if let Some(fullname) = &entity.fullname {
            let score = strsim::jaro_winkler(&search_normalized, &normalize(fullname));
            if score > best_score {
                best_score = score;
            }
            if score >= min_score {
                matched_fields.push("fullname".to_string());
            }
        }

        // Match against name
        if let Some(name) = &entity.name {
            let score = strsim::jaro_winkler(&search_normalized, &normalize(name));
            if score > best_score {
                best_score = score;
            }
            if score >= min_score {
                matched_fields.push("name".to_string());
            }
        }

        // Only return a result if we have a meaningful score
        // Cap at 0.99 since 1.0 is reserved for exact ID match
        if best_score > 0.0 {
            Some(EntitySearchResult::fuzzy(
                entity.clone(),
                best_score.min(0.99),
                matched_fields,
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity_search::MatchType;
    use crate::types::entity::{CanonizationLevel, EntityDefinition, EntityType, Sex, Title};

    fn create_test_entity(id: &str, name: &str, fullname: &str) -> Entity {
        let definition = EntityDefinition {
            name: Some(name.to_string()),
            fullname: Some(fullname.to_string()),
            r#type: Some(EntityType::Person),
            canonization_level: Some(CanonizationLevel::Saint),
            sex: Some(Sex::Male),
            ..Default::default()
        };
        Entity::new(id.to_string(), definition)
    }

    #[test]
    fn test_exact_id_match() {
        let matcher = EntityMatcher::new();
        let entities = vec![
            create_test_entity("francis_of_assisi", "Francis", "Saint Francis of Assisi"),
            create_test_entity("francis_xavier", "Francis Xavier", "Saint Francis Xavier"),
        ];

        let query = EntityQuery {
            text: Some("francis_of_assisi".into()),
            ..Default::default()
        };

        let results = matcher.search(entities.iter(), &query);

        assert!(!results.is_empty());
        assert_eq!(results[0].match_type, MatchType::ExactId);
        assert_eq!(results[0].score, 1.0);
        assert_eq!(&results[0].entity.id, "francis_of_assisi");
    }

    #[test]
    fn test_fuzzy_match() {
        let matcher = EntityMatcher::new();
        let entities = vec![create_test_entity(
            "francis_of_assisi",
            "Francis",
            "Saint Francis of Assisi",
        )];

        let query = EntityQuery {
            text: Some("franc".into()),
            ..Default::default()
        };

        let results = matcher.search(entities.iter(), &query);

        assert!(!results.is_empty());
        assert_eq!(results[0].match_type, MatchType::Fuzzy);
        assert!(results[0].score < 1.0);
        assert!(results[0].score > 0.5); // Jaro-Winkler gives good scores for prefix matches
    }

    #[test]
    fn test_fuzzy_match_with_accents() {
        let matcher = EntityMatcher::new();
        let entities = vec![create_test_entity(
            "francis_of_assisi",
            "Francis",
            "Saint Francis of Assisi",
        )];

        // Test French variant "François" matching "Francis"
        let query = EntityQuery {
            text: Some("françois".into()),
            ..Default::default()
        };

        let results = matcher.search(entities.iter(), &query);

        assert!(!results.is_empty());
        assert_eq!(results[0].match_type, MatchType::Fuzzy);
        assert!(results[0].score > 0.8); // Should have high similarity
    }

    #[test]
    fn test_fuzzy_match_variant_name() {
        let matcher = EntityMatcher::new();
        let entities = vec![create_test_entity("mary", "Mary", "Virgin Mary")];

        // Test French variant "Marie" matching "Mary"
        let query = EntityQuery {
            text: Some("marie".into()),
            ..Default::default()
        };

        let results = matcher.search(entities.iter(), &query);

        assert!(!results.is_empty());
        assert_eq!(results[0].match_type, MatchType::Fuzzy);
        assert!(results[0].score > 0.7); // Should find a reasonable match
    }

    #[test]
    fn test_filter_by_entity_type() {
        let matcher = EntityMatcher::new();
        let mut entity = create_test_entity("test", "Test", "Test Entity");
        entity.r#type = EntityType::Place;

        let entities = vec![entity];

        // Should not match when filtering for Person
        let query = EntityQuery {
            entity_type: Some(EntityType::Person),
            ..Default::default()
        };
        let results = matcher.search(entities.iter(), &query);
        assert!(results.is_empty());

        // Should match when filtering for Place
        let query = EntityQuery {
            entity_type: Some(EntityType::Place),
            ..Default::default()
        };
        let results = matcher.search(entities.iter(), &query);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].match_type, MatchType::FilterOnly);
    }

    #[test]
    fn test_filter_by_canonization_level() {
        let matcher = EntityMatcher::new();
        let entities = vec![create_test_entity("saint_test", "Test", "Saint Test")];

        // Should match Saint
        let query = EntityQuery {
            canonization_level: Some(CanonizationLevel::Saint),
            ..Default::default()
        };
        let results = matcher.search(entities.iter(), &query);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].match_type, MatchType::FilterOnly);

        // Should not match Blessed
        let query = EntityQuery {
            canonization_level: Some(CanonizationLevel::Blessed),
            ..Default::default()
        };
        let results = matcher.search(entities.iter(), &query);
        assert!(results.is_empty());
    }

    #[test]
    fn test_filter_by_sex() {
        let matcher = EntityMatcher::new();
        let entities = vec![create_test_entity("test", "Test", "Test Entity")];

        // Should match Male
        let query = EntityQuery {
            sex: Some(Sex::Male),
            ..Default::default()
        };
        let results = matcher.search(entities.iter(), &query);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].match_type, MatchType::FilterOnly);

        // Should not match Female
        let query = EntityQuery {
            sex: Some(Sex::Female),
            ..Default::default()
        };
        let results = matcher.search(entities.iter(), &query);
        assert!(results.is_empty());
    }

    #[test]
    fn test_filter_by_titles() {
        let matcher = EntityMatcher::new();

        // Create entities with different titles
        let mut abbot = create_test_entity("benedict", "Benedict", "Saint Benedict");
        abbot.titles = Some(vec![Title::Abbot]);

        let mut bishop = create_test_entity("augustine", "Augustine", "Saint Augustine");
        bishop.titles = Some(vec![Title::Bishop]);

        let mut martyr = create_test_entity("stephen", "Stephen", "Saint Stephen");
        martyr.titles = Some(vec![Title::Martyr]);

        let entities = vec![abbot, bishop, martyr];

        // Filter only Abbots
        let query = EntityQuery {
            titles: Some(vec![Title::Abbot]),
            ..Default::default()
        };
        let results = matcher.search(entities.iter(), &query);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].match_type, MatchType::FilterOnly);
        assert_eq!(&results[0].entity.id, "benedict");

        // Filter Abbots and Bishops
        let query = EntityQuery {
            titles: Some(vec![Title::Abbot, Title::Bishop]),
            ..Default::default()
        };
        let results = matcher.search(entities.iter(), &query);
        assert_eq!(results.len(), 2);
        let ids: Vec<&str> = results.iter().map(|r| r.entity.id.as_str()).collect();
        assert!(ids.contains(&"benedict"));
        assert!(ids.contains(&"augustine"));
        assert!(!ids.contains(&"stephen"));
    }

    #[test]
    fn test_combined_text_and_filters() {
        let matcher = EntityMatcher::new();
        let entities = vec![
            create_test_entity("francis_of_assisi", "Francis", "Saint Francis of Assisi"),
            create_test_entity("francis_xavier", "Francis Xavier", "Saint Francis Xavier"),
        ];

        let query = EntityQuery {
            text: Some("francis".into()),
            canonization_level: Some(CanonizationLevel::Saint),
            ..Default::default()
        };

        let results = matcher.search(entities.iter(), &query);
        assert_eq!(results.len(), 2);
        // Text search with filters should be Fuzzy, not FilterOnly
        assert_eq!(results[0].match_type, MatchType::Fuzzy);
        assert_eq!(results[1].match_type, MatchType::Fuzzy);
    }

    #[test]
    fn test_limit() {
        let matcher = EntityMatcher::new();
        let entities: Vec<Entity> = (0..50)
            .map(|i| create_test_entity(&format!("entity_{}", i), "Test", "Test Entity"))
            .collect();

        let query = EntityQuery {
            limit: Some(5),
            ..Default::default()
        };

        let results = matcher.search(entities.iter(), &query);
        assert_eq!(results.len(), 5);
        // No text search, only limit → FilterOnly
        for result in &results {
            assert_eq!(result.match_type, MatchType::FilterOnly);
        }
    }

    #[test]
    fn test_normalize() {
        // Accented characters
        assert_eq!(normalize("François"), "francois");
        assert_eq!(normalize("MARIE"), "marie");
        assert_eq!(normalize("Święty"), "swiety");
        assert_eq!(normalize("José"), "jose");
        assert_eq!(normalize("Thérèse"), "therese");

        // Ligatures expansion
        assert_eq!(normalize("Cæsar"), "caesar");
        assert_eq!(normalize("cœur"), "coeur");
        assert_eq!(normalize("Straße"), "strasse");
    }
}
