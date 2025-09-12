//! # Romcal - Liturgical Calendar Library
//!
//! A Rust library for calculating Catholic liturgical dates and seasons.
//!
//! ## Quick Start
//!
//! ```rust
//! use romcal_core::{LiturgicalConfig, LiturgicalDates};
//!
//! let config = LiturgicalConfig::default();
//! let dates = LiturgicalDates::new(config, 2024).unwrap();
//! let easter = dates.get_easter_sunday_date_unwrap(None);
//! ```

pub mod calendar_def;
pub mod config;
pub mod dates;
pub mod easter;
pub mod error;
pub mod generated_constants;
pub mod resources;
pub mod types;

pub use calendar_def::*;
pub use config::LiturgicalConfig;
pub use dates::LiturgicalDates;
pub use error::{validate_range, validate_year, RomcalError, RomcalResult, Validate};
pub use generated_constants::{CALENDAR_IDS, LOCALE_CODES};
pub use resources::{EntityDefinition, EntityId, LocaleId, ResourcesDefinition};
pub use types::entity::SaintCount;
pub use types::liturgical::Season;
pub use types::{CalendarScope, EasterCalculationType};
