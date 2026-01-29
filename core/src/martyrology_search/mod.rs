//! Martyrology search module with fuzzy matching support.
//!
//! This module provides functionality to search for martyrology entries (saints, blessed, places, events)
//! with support for fuzzy matching, accent-insensitive search, and various filters.

mod matcher;
mod query;
mod result;

pub use matcher::MartyrologyMatcher;
pub use query::MartyrologyQuery;
pub use result::{MartyrologySearchResult, MatchType};
