//! # Romcal - Liturgical Calendar Library
//!
//! A Rust library for calculating Catholic liturgical dates and seasons.
//!
//! ## Quick Start
//!
//! ```rust
//! use romcal::{LiturgicalConfig, LiturgicalDates};
//!
//! let config = LiturgicalConfig::default();
//! let dates = LiturgicalDates::new(config, 2024).unwrap();
//! let easter = dates.get_easter_sunday_date_unwrap(None);
//! ```

pub mod config;
pub mod dates;
pub mod easter;
pub mod error;

pub use config::{CalendarScope, EasterCalculationType, LiturgicalConfig};
pub use dates::{LiturgicalDates, Season};
pub use error::{validate_range, validate_year, RomcalError, RomcalResult, Validate};
