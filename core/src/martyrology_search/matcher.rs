//! Martyrology matcher with fuzzy search support using Jaro-Winkler similarity.

use crate::types::martyrology::{MartyrologyEntry, Title};

use super::query::MartyrologyQuery;
use super::result::MartyrologySearchResult;

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

/// Martyrology matcher that performs fuzzy search on martyrology entries.
#[derive(Default)]
pub struct MartyrologyMatcher;

impl MartyrologyMatcher {
    /// Create a new martyrology matcher.
    pub fn new() -> Self {
        Self
    }

    /// Search martyrology entries with the given query.
    ///
    /// Returns a list of matching entries sorted by score (highest first).
    pub fn search<'a>(
        &self,
        entries: impl Iterator<Item = &'a MartyrologyEntry>,
        query: &MartyrologyQuery,
    ) -> Vec<MartyrologySearchResult> {
        let limit = query.effective_limit();
        let min_score = query.effective_min_score();

        // Collect all matching entries
        let mut results: Vec<MartyrologySearchResult> = entries
            .filter_map(|entry| self.match_entry(entry, query))
            .filter(|result| result.score >= min_score)
            .collect();

        // Sort by score (highest first), then by ID for stability
        results.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| a.entry.id.cmp(&b.entry.id))
        });

        // Apply limit
        results.truncate(limit);

        results
    }

    /// Match a single entry against the query.
    ///
    /// Returns `None` if the entry doesn't match the query filters or text.
    fn match_entry(
        &self,
        entry: &MartyrologyEntry,
        query: &MartyrologyQuery,
    ) -> Option<MartyrologySearchResult> {
        // Apply filters first (fast rejection)
        if !self.matches_filters(entry, query) {
            return None;
        }

        // If no text query, return as filter-only match
        if !query.has_text() {
            return Some(MartyrologySearchResult::filter_only(entry.clone()));
        }

        let search_text = query.text.as_ref().unwrap();

        // Check for exact ID match first
        if entry.id.eq_ignore_ascii_case(search_text) {
            return Some(MartyrologySearchResult::exact_id(entry.clone()));
        }

        // Perform fuzzy matching on text fields
        self.fuzzy_match(entry, search_text, query.effective_min_score())
    }

    /// Check if entry matches all query filters.
    fn matches_filters(&self, entry: &MartyrologyEntry, query: &MartyrologyQuery) -> bool {
        // Filter by entry type
        if let Some(ref query_type) = query.entry_type
            && &entry.r#type != query_type
        {
            return false;
        }

        // Filter by canonization level
        if let Some(ref query_level) = query.canonization_level
            && entry.canonization_level.as_ref() != Some(query_level)
        {
            return false;
        }

        // Filter by sex
        if let Some(ref query_sex) = query.sex
            && entry.sex.as_ref() != Some(query_sex)
        {
            return false;
        }

        // Filter by titles (must have at least one matching title)
        if let Some(ref query_titles) = query.titles
            && !self.has_matching_title(entry, query_titles)
        {
            return false;
        }

        true
    }

    /// Check if entry has at least one of the specified titles.
    fn has_matching_title(&self, entry: &MartyrologyEntry, query_titles: &[Title]) -> bool {
        entry.titles.as_ref().is_some_and(|entry_titles| {
            entry_titles
                .iter()
                .any(|title| query_titles.contains(title))
        })
    }

    /// Perform fuzzy matching on entry text fields using Jaro-Winkler similarity.
    fn fuzzy_match(
        &self,
        entry: &MartyrologyEntry,
        search_text: &str,
        min_score: f64,
    ) -> Option<MartyrologySearchResult> {
        let search_normalized = normalize(search_text);
        let mut best_score: f64 = 0.0;
        let mut matched_fields = Vec::new();

        // Match against ID
        let score = strsim::jaro_winkler(&search_normalized, &normalize(&entry.id));
        if score > best_score {
            best_score = score;
        }
        if score >= min_score {
            matched_fields.push("id".to_string());
        }

        // Match against fullname
        if let Some(fullname) = &entry.fullname {
            let score = strsim::jaro_winkler(&search_normalized, &normalize(fullname));
            if score > best_score {
                best_score = score;
            }
            if score >= min_score {
                matched_fields.push("fullname".to_string());
            }
        }

        // Match against name
        if let Some(name) = &entry.name {
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
            Some(MartyrologySearchResult::fuzzy(
                entry.clone(),
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
    use crate::martyrology_search::MatchType;
    use crate::types::martyrology::{
        CanonizationLevel, MartyrologyEntryDef, MartyrologyEntryType, Sex, Title,
    };

    fn create_test_entry(id: &str, name: &str, fullname: &str) -> MartyrologyEntry {
        let definition = MartyrologyEntryDef {
            name: Some(name.to_string()),
            fullname: Some(fullname.to_string()),
            r#type: Some(MartyrologyEntryType::Person),
            canonization_level: Some(CanonizationLevel::Saint),
            sex: Some(Sex::Male),
            ..Default::default()
        };
        MartyrologyEntry::new(id.to_string(), definition)
    }

    #[test]
    fn test_exact_id_match() {
        let matcher = MartyrologyMatcher::new();
        let entries = vec![
            create_test_entry("francis_of_assisi", "Francis", "Saint Francis of Assisi"),
            create_test_entry("francis_xavier", "Francis Xavier", "Saint Francis Xavier"),
        ];

        let query = MartyrologyQuery {
            text: Some("francis_of_assisi".into()),
            ..Default::default()
        };

        let results = matcher.search(entries.iter(), &query);

        assert!(!results.is_empty());
        assert_eq!(results[0].match_type, MatchType::ExactId);
        assert_eq!(results[0].score, 1.0);
        assert_eq!(&results[0].entry.id, "francis_of_assisi");
    }

    #[test]
    fn test_fuzzy_match() {
        let matcher = MartyrologyMatcher::new();
        let entries = vec![create_test_entry(
            "francis_of_assisi",
            "Francis",
            "Saint Francis of Assisi",
        )];

        let query = MartyrologyQuery {
            text: Some("franc".into()),
            ..Default::default()
        };

        let results = matcher.search(entries.iter(), &query);

        assert!(!results.is_empty());
        assert_eq!(results[0].match_type, MatchType::Fuzzy);
        assert!(results[0].score < 1.0);
        assert!(results[0].score > 0.5); // Jaro-Winkler gives good scores for prefix matches
    }

    #[test]
    fn test_fuzzy_match_with_accents() {
        let matcher = MartyrologyMatcher::new();
        let entries = vec![create_test_entry(
            "francis_of_assisi",
            "Francis",
            "Saint Francis of Assisi",
        )];

        // Test French variant "François" matching "Francis"
        let query = MartyrologyQuery {
            text: Some("françois".into()),
            ..Default::default()
        };

        let results = matcher.search(entries.iter(), &query);

        assert!(!results.is_empty());
        assert_eq!(results[0].match_type, MatchType::Fuzzy);
        assert!(results[0].score > 0.8); // Should have high similarity
    }

    #[test]
    fn test_fuzzy_match_variant_name() {
        let matcher = MartyrologyMatcher::new();
        let entries = vec![create_test_entry("mary", "Mary", "Virgin Mary")];

        // Test French variant "Marie" matching "Mary"
        let query = MartyrologyQuery {
            text: Some("marie".into()),
            ..Default::default()
        };

        let results = matcher.search(entries.iter(), &query);

        assert!(!results.is_empty());
        assert_eq!(results[0].match_type, MatchType::Fuzzy);
        assert!(results[0].score > 0.7); // Should find a reasonable match
    }

    #[test]
    fn test_filter_by_entry_type() {
        let matcher = MartyrologyMatcher::new();
        let mut entry = create_test_entry("test", "Test", "Test Entry");
        entry.r#type = MartyrologyEntryType::Place;

        let entries = vec![entry];

        // Should not match when filtering for Person
        let query = MartyrologyQuery {
            entry_type: Some(MartyrologyEntryType::Person),
            ..Default::default()
        };
        let results = matcher.search(entries.iter(), &query);
        assert!(results.is_empty());

        // Should match when filtering for Place
        let query = MartyrologyQuery {
            entry_type: Some(MartyrologyEntryType::Place),
            ..Default::default()
        };
        let results = matcher.search(entries.iter(), &query);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].match_type, MatchType::FilterOnly);
    }

    #[test]
    fn test_filter_by_canonization_level() {
        let matcher = MartyrologyMatcher::new();
        let entries = vec![create_test_entry("saint_test", "Test", "Saint Test")];

        // Should match Saint
        let query = MartyrologyQuery {
            canonization_level: Some(CanonizationLevel::Saint),
            ..Default::default()
        };
        let results = matcher.search(entries.iter(), &query);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].match_type, MatchType::FilterOnly);

        // Should not match Blessed
        let query = MartyrologyQuery {
            canonization_level: Some(CanonizationLevel::Blessed),
            ..Default::default()
        };
        let results = matcher.search(entries.iter(), &query);
        assert!(results.is_empty());
    }

    #[test]
    fn test_filter_by_sex() {
        let matcher = MartyrologyMatcher::new();
        let entries = vec![create_test_entry("test", "Test", "Test Entry")];

        // Should match Male
        let query = MartyrologyQuery {
            sex: Some(Sex::Male),
            ..Default::default()
        };
        let results = matcher.search(entries.iter(), &query);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].match_type, MatchType::FilterOnly);

        // Should not match Female
        let query = MartyrologyQuery {
            sex: Some(Sex::Female),
            ..Default::default()
        };
        let results = matcher.search(entries.iter(), &query);
        assert!(results.is_empty());
    }

    #[test]
    fn test_filter_by_titles() {
        let matcher = MartyrologyMatcher::new();

        // Create entries with different titles
        let mut abbot = create_test_entry("benedict", "Benedict", "Saint Benedict");
        abbot.titles = Some(vec![Title::Abbot]);

        let mut bishop = create_test_entry("augustine", "Augustine", "Saint Augustine");
        bishop.titles = Some(vec![Title::Bishop]);

        let mut martyr = create_test_entry("stephen", "Stephen", "Saint Stephen");
        martyr.titles = Some(vec![Title::Martyr]);

        let entries = vec![abbot, bishop, martyr];

        // Filter only Abbots
        let query = MartyrologyQuery {
            titles: Some(vec![Title::Abbot]),
            ..Default::default()
        };
        let results = matcher.search(entries.iter(), &query);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].match_type, MatchType::FilterOnly);
        assert_eq!(&results[0].entry.id, "benedict");

        // Filter Abbots and Bishops
        let query = MartyrologyQuery {
            titles: Some(vec![Title::Abbot, Title::Bishop]),
            ..Default::default()
        };
        let results = matcher.search(entries.iter(), &query);
        assert_eq!(results.len(), 2);
        let ids: Vec<&str> = results.iter().map(|r| r.entry.id.as_str()).collect();
        assert!(ids.contains(&"benedict"));
        assert!(ids.contains(&"augustine"));
        assert!(!ids.contains(&"stephen"));
    }

    #[test]
    fn test_combined_text_and_filters() {
        let matcher = MartyrologyMatcher::new();
        let entries = vec![
            create_test_entry("francis_of_assisi", "Francis", "Saint Francis of Assisi"),
            create_test_entry("francis_xavier", "Francis Xavier", "Saint Francis Xavier"),
        ];

        let query = MartyrologyQuery {
            text: Some("francis".into()),
            canonization_level: Some(CanonizationLevel::Saint),
            ..Default::default()
        };

        let results = matcher.search(entries.iter(), &query);
        assert_eq!(results.len(), 2);
        // Text search with filters should be Fuzzy, not FilterOnly
        assert_eq!(results[0].match_type, MatchType::Fuzzy);
        assert_eq!(results[1].match_type, MatchType::Fuzzy);
    }

    #[test]
    fn test_limit() {
        let matcher = MartyrologyMatcher::new();
        let entries: Vec<MartyrologyEntry> = (0..50)
            .map(|i| create_test_entry(&format!("entry_{}", i), "Test", "Test Entry"))
            .collect();

        let query = MartyrologyQuery {
            limit: Some(5),
            ..Default::default()
        };

        let results = matcher.search(entries.iter(), &query);
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
