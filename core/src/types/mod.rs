//! Type definitions for the Romcal liturgical calendar library.
//!
//! This module contains all the core types used throughout Romcal,
//! organized into submodules by domain: dates, calendar, martyrology,
//! liturgical, mass, preset, and resource.

pub mod calendar;
pub mod dates;
pub mod liturgical;
pub mod martyrology;
pub mod mass;
pub mod preset;
pub mod resource;

pub use calendar::*;
pub use dates::*;
pub use liturgical::*;
pub use martyrology::*;
pub use mass::*;
pub use preset::*;
pub use resource::*;
