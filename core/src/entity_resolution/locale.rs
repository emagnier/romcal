//! Locale hierarchy utilities.
//!
//! This module provides functions for working with BCP 47 locale tags
//! and building locale hierarchies for resource merging.
//!
//! All locale operations are case-insensitive per BCP 47 (RFC 5646).

/// Normalize a locale to lowercase for case-insensitive comparison.
///
/// BCP 47 locale tags are case-insensitive, but have conventional formatting:
/// - Language: lowercase (e.g., `fr`, `en`)
/// - Script: Title case (e.g., `Hant`, `Latn`)
/// - Region: UPPERCASE (e.g., `CA`, `FR`)
///
/// This function normalizes to lowercase for internal storage and comparison.
///
/// # Examples
///
/// ```
/// use romcal::entity_resolution::locale::normalize_locale;
///
/// assert_eq!(normalize_locale("fr-CA"), "fr-ca");
/// assert_eq!(normalize_locale("zh-Hant-TW"), "zh-hant-tw");
/// assert_eq!(normalize_locale("EN"), "en");
/// ```
pub fn normalize_locale(locale: &str) -> String {
    locale.to_lowercase()
}

/// Get the parent locale from a BCP 47 locale tag.
///
/// # Examples
///
/// ```
/// use romcal::entity_resolution::locale::get_parent_locale;
///
/// assert_eq!(get_parent_locale("fr-FR"), Some("fr".to_string()));
/// assert_eq!(get_parent_locale("fr-CA-fonipa"), Some("fr-CA".to_string()));
/// assert_eq!(get_parent_locale("fr"), None);
/// assert_eq!(get_parent_locale("en"), None);
/// ```
pub fn get_parent_locale(locale: &str) -> Option<String> {
    locale.rfind('-').map(|pos| locale[..pos].to_string())
}

/// Get all parent locales from most specific to most general.
///
/// # Examples
///
/// ```
/// use romcal::entity_resolution::locale::get_all_parent_locales;
///
/// assert_eq!(get_all_parent_locales("fr-CA-fonipa"), vec!["fr-CA", "fr"]);
/// assert_eq!(get_all_parent_locales("zh-Hant-TW"), vec!["zh-Hant", "zh"]);
/// assert_eq!(get_all_parent_locales("fr"), Vec::<String>::new());
/// ```
pub fn get_all_parent_locales(locale: &str) -> Vec<String> {
    let parts: Vec<&str> = locale.split('-').collect();
    let mut parents = Vec::new();

    // Generate all possible parent locales by progressively removing the last part
    for i in 1..parts.len() {
        let parent = parts[..parts.len() - i].join("-");
        parents.push(parent);
    }

    parents
}

/// Build locale merge hierarchy from most general to most specific.
///
/// This function builds a list of locales to merge in order, starting with
/// the base locale "en" and progressing to the most specific locale.
///
/// # Examples
///
/// ```
/// use romcal::entity_resolution::locale::build_merge_hierarchy;
///
/// assert_eq!(build_merge_hierarchy("fr-FR"), vec!["en", "fr", "fr-FR"]);
/// assert_eq!(build_merge_hierarchy("fr"), vec!["en", "fr"]);
/// assert_eq!(build_merge_hierarchy("en"), vec!["en"]);
/// assert_eq!(build_merge_hierarchy("zh-Hant-TW"), vec!["en", "zh", "zh-Hant", "zh-Hant-TW"]);
/// ```
pub fn build_merge_hierarchy(locale: &str) -> Vec<String> {
    let mut hierarchy = vec!["en".to_string()];

    if locale != "en" {
        // Add parent locales in order (most general first)
        let parents = get_all_parent_locales(locale);
        for parent in parents.into_iter().rev() {
            if parent != "en" {
                hierarchy.push(parent);
            }
        }
        // Add the specific locale last
        hierarchy.push(locale.to_string());
    }

    hierarchy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_parent_locale() {
        assert_eq!(get_parent_locale("fr-FR"), Some("fr".to_string()));
        assert_eq!(get_parent_locale("fr-CA-fonipa"), Some("fr-CA".to_string()));
        assert_eq!(get_parent_locale("fr"), None);
        assert_eq!(get_parent_locale("en"), None);
        assert_eq!(get_parent_locale("zh-Hant-TW"), Some("zh-Hant".to_string()));
    }

    #[test]
    fn test_get_all_parent_locales() {
        assert_eq!(get_all_parent_locales("fr-CA-fonipa"), vec!["fr-CA", "fr"]);
        assert_eq!(get_all_parent_locales("zh-Hant-TW"), vec!["zh-Hant", "zh"]);
        assert_eq!(get_all_parent_locales("fr-FR"), vec!["fr"]);
        assert_eq!(get_all_parent_locales("fr"), Vec::<String>::new());
        assert_eq!(get_all_parent_locales("en"), Vec::<String>::new());
    }

    #[test]
    fn test_build_merge_hierarchy() {
        // Simple locale
        assert_eq!(build_merge_hierarchy("fr"), vec!["en", "fr"]);

        // Two-part locale
        assert_eq!(build_merge_hierarchy("fr-FR"), vec!["en", "fr", "fr-FR"]);

        // Three-part locale
        assert_eq!(
            build_merge_hierarchy("zh-Hant-TW"),
            vec!["en", "zh", "zh-Hant", "zh-Hant-TW"]
        );

        // English locale (base case)
        assert_eq!(build_merge_hierarchy("en"), vec!["en"]);

        // English variant
        assert_eq!(build_merge_hierarchy("en-GB"), vec!["en", "en-GB"]);
    }

    #[test]
    fn test_build_merge_hierarchy_order() {
        // Verify the order is from most general to most specific
        let hierarchy = build_merge_hierarchy("fr-CA-fonipa");

        // en should be first
        assert_eq!(hierarchy[0], "en");

        // fr should come before fr-CA
        let fr_idx = hierarchy.iter().position(|x| x == "fr").unwrap();
        let fr_ca_idx = hierarchy.iter().position(|x| x == "fr-CA").unwrap();
        assert!(fr_idx < fr_ca_idx);

        // fr-CA should come before fr-CA-fonipa
        let fr_ca_fonipa_idx = hierarchy.iter().position(|x| x == "fr-CA-fonipa").unwrap();
        assert!(fr_ca_idx < fr_ca_fonipa_idx);
    }

    #[test]
    fn test_normalize_locale() {
        assert_eq!(normalize_locale("fr-CA"), "fr-ca");
        assert_eq!(normalize_locale("zh-Hant-TW"), "zh-hant-tw");
        assert_eq!(normalize_locale("EN"), "en");
        assert_eq!(normalize_locale("FR-FR"), "fr-fr");
        assert_eq!(normalize_locale("en"), "en");
    }
}
