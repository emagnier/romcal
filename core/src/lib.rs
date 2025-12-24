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
pub mod error;
pub mod generated;
pub mod romcal;
pub mod types;

pub use engine::calendar::{Calendar, LiturgicalCalendar};
pub use engine::calendar_definition::*;
pub use engine::dates::LiturgicalDates;
pub use engine::entity_resolver::EntityResolver;
pub use engine::liturgical_day::*;
pub use engine::proper_of_time::ProperOfTime;
pub use engine::resources::*;
pub use engine::template_resolver::{GrammaticalGender, ProperOfTimeDayType, TemplateResolver};
pub use error::{RomcalError, RomcalResult, Validate, validate_range, validate_year};
pub use generated::calendar_ids::CALENDAR_IDS;
pub use generated::locale_ids::LOCALE_CODES;
pub use generated::schemas;
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
