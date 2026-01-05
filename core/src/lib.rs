//! # Romcal - Liturgical Calendar Library
//!
//! A Rust library for calculating Catholic liturgical dates and seasons.
//!
//! ## Quick Start
//!
//! ```rust
//! use romcal::{Romcal, LiturgicalDates};
//!
//! let romcal = Romcal::default();
//! let dates = LiturgicalDates::new(romcal, 2024).unwrap();
//! let easter = dates.get_easter_sunday_date_unwrap(None);
//! ```

pub mod engine;
pub mod entity_resolution;
pub mod entity_search;
pub mod error;
pub mod generated;
pub mod helpers;
pub mod romcal;
pub mod types;

/// The version of the romcal library.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub use engine::calendar::{Calendar, LiturgicalCalendar};
pub use engine::calendar_definition::*;
pub use engine::dates::LiturgicalDates;
pub use engine::liturgical_day::*;
pub use engine::proper_of_time::ProperOfTime;
pub use engine::resources::*;
pub use engine::template_resolver::{GrammaticalGender, ProperOfTimeDayType, TemplateResolver};
pub use entity_resolution::EntityResolver;
pub use error::{RomcalError, RomcalResult, Validate, validate_range, validate_year};
pub use generated::calendar_ids::CALENDAR_IDS;
pub use generated::locale_ids::LOCALE_CODES;
pub use generated::schemas;
pub use helpers::{merge_calendar_definitions, merge_resource_files};
pub use romcal::{Preset, Romcal};
pub use types::entity::SaintCount;
pub use types::entity::{Entity, EntityId};
pub use types::liturgical::Season;
pub use types::mass::{CelebrationSummary, MassCalendar, MassContext, MassInfo, MassTime};
pub use types::{CalendarContext, EasterCalculationType};

// Additional types for schema generation
pub use types::dates::{DateDefWithOffset, DayOfWeek, MonthIndex};
pub use types::liturgical::SundayCycleCombined;
pub use types::mass::{Acclamation, BibleBook, LiturgicalCycle, MassPart};

// Entity search types
pub use entity_search::{EntityMatcher, EntityQuery, EntitySearchResult, MatchType};
