//! Martyrology types for entries in the Roman Martyrology.
//!
//! This module provides types for representing persons, places, and events
//! in the liturgical calendar, including their titles, dates, and canonization status.
//! The Roman Martyrology (Martyrologium Romanum) is the official catalog of saints,
//! blessed, church dedications, and events recognized by the Catholic Church.

pub mod canonization_level;
pub mod entry_def;
pub mod entry_override;
pub mod entry_type;
pub mod saint_count;
pub mod saint_date;
pub mod sex;
pub mod title;
pub mod with_id;

pub use canonization_level::*;
pub use entry_def::*;
pub use entry_override::*;
pub use entry_type::*;
pub use saint_count::*;
pub use saint_date::*;
pub use sex::*;
pub use title::*;
pub use with_id::*;
