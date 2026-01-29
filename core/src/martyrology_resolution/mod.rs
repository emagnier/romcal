//! Martyrology resolution module.
//!
//! This module provides functionality for resolving martyrology entries from resources
//! with locale-based fallback. It handles:
//!
//! - Merging entries across locales (en → parent → specific)
//! - Resolving martyrology pointers (ResourceId, Override)
//! - Combining titles from multiple entries
//!
//! # Locale Hierarchy
//!
//! For a locale like "fr-FR", entries are merged in this order:
//! 1. "en" (base locale)
//! 2. "fr" (parent locale)
//! 3. "fr-FR" (specific locale)
//!
//! Properties from more specific locales override those from more general locales.

pub mod locale;
mod merge;
mod pointer;
mod resolver;

pub use locale::{
    build_merge_hierarchy, get_all_parent_locales, get_parent_locale, normalize_locale,
};
pub use resolver::MartyrologyResolver;
