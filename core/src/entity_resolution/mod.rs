//! Entity resolution module.
//!
//! This module provides functionality for resolving entities from resources
//! with locale-based fallback. It handles:
//!
//! - Merging entities across locales (en → parent → specific)
//! - Resolving entity pointers (ResourceId, Override)
//! - Combining titles from multiple entities
//!
//! # Locale Hierarchy
//!
//! For a locale like "fr-FR", entities are merged in this order:
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
pub use resolver::EntityResolver;
