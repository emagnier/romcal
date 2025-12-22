//! Preset configuration types for the Romcal library.
//!
//! This module provides types for configuring calendar generation,
//! including Easter calculation methods, calendar contexts, and formatting options.

pub mod calendar_context;
pub mod easter_calculation_type;
pub mod ordinal_format;
pub mod particular_config;

pub use calendar_context::*;
pub use easter_calculation_type::*;
pub use ordinal_format::*;
pub use particular_config::*;
