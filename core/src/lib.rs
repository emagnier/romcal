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
pub mod resources;
pub mod wasm;

pub use calendar_def::*;
pub use config::{CalendarScope, EasterCalculationType, LiturgicalConfig};
pub use dates::{LiturgicalDates, Season};
pub use error::{validate_range, validate_year, RomcalError, RomcalResult, Validate};
pub use resources::{
    CanonizationLevel, EntityDefinition, EntityId, EntityType, LocaleColors, LocaleId,
    ResourcesDefinition, ResourcesMetadata, SaintDate, SaintDateDef, Sex,
};

// Re-export WASM types for easier access
pub use wasm::{
    romcal, romcal_with_config, romcal_with_config_object, romcal_with_partial_config,
    PartialRomcalConfig, Romcal, RomcalConfig,
};
