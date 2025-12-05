//! # Romcal - Liturgical Calendar Library
//!
//! A Rust library for calculating Catholic liturgical dates and seasons.
//!
//! ## Quick Start
//!
//! ```rust
//! use romcal_core::{Preset, LiturgicalDates};
//!
//! let preset = Preset::default();
//! let dates = LiturgicalDates::new(preset, 2024).unwrap();
//! let easter = dates.get_easter_sunday_date_unwrap(None);
//! ```

pub mod calendar;
pub mod calendar_definition;
pub mod dates;
pub mod easter;
pub mod error;
pub mod generated_constants;
pub mod liturgical_day;
pub mod optimize;
pub mod preset;
pub mod proper_of_time;
pub mod resources;
pub mod types;

pub use calendar::{Calendar, LiturgicalCalendar};
pub use calendar_definition::*;
pub use dates::LiturgicalDates;
pub use error::{validate_range, validate_year, RomcalError, RomcalResult, Validate};
pub use generated_constants::{CALENDAR_IDS, LOCALE_CODES};
pub use liturgical_day::*;
pub use preset::{Preset, PresetPartial};
pub use proper_of_time::ProperOfTime;
pub use resources::*;
pub use types::entity::SaintCount;
pub use types::entity::{Entity, EntityId};
pub use types::liturgical::Season;
pub use types::{CalendarContext, EasterCalculationType};
