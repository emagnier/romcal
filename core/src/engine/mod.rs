//! Core engine for liturgical calendar generation.
//!
//! This module contains the main business logic for computing liturgical calendars,
//! resolving entities, and managing resources.

pub mod calendar;
pub mod calendar_definition;
pub mod data_tree_builder;
pub mod dates;
pub mod easter;
pub mod entity_resolver;
pub mod liturgical_day;
pub mod optimize;
pub mod proper_of_time;
pub mod resources;
pub mod template_resolver;

pub use calendar::*;
pub use calendar_definition::*;
pub use data_tree_builder::*;
pub use dates::*;
pub use easter::*;
pub use entity_resolver::*;
pub use liturgical_day::*;
pub use optimize::*;
pub use proper_of_time::*;
pub use resources::*;
pub use template_resolver::*;
