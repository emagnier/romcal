//! Entity search module with fuzzy matching support.
//!
//! This module provides functionality to search for entities (saints, blessed, places, events)
//! with support for fuzzy matching, accent-insensitive search, and various filters.

mod matcher;
mod query;
mod result;

pub use matcher::EntityMatcher;
pub use query::EntityQuery;
pub use result::{EntitySearchResult, MatchType};
