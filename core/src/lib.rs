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
pub mod types;

pub use calendar_def::*;
pub use config::{CalendarScope, EasterCalculationType, LiturgicalConfig};
pub use dates::{LiturgicalDates, Season};
pub use error::{validate_range, validate_year, RomcalError, RomcalResult, Validate};
pub use resources::{
    CanonizationLevel, EntityDefinition, EntityId, EntityType, LocaleColors, LocaleId,
    ResourcesDefinition, ResourcesMetadata, SaintDate, SaintDateDef, Sex,
};
pub use types::SaintCount;

#[cfg(test)]
mod integration_tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_saint_count_optimization() {
        // Test avec JSON (format auto-descriptif)
        let json_with_many = r#""MANY""#;
        let result: SaintCount = serde_json::from_str(json_with_many).unwrap();
        assert!(matches!(result, SaintCount::Many));

        let json_with_number = r#"42"#;
        let result: SaintCount = serde_json::from_str(json_with_number).unwrap();
        assert!(matches!(result, SaintCount::Number(42)));

        // Test avec des valeurs invalides
        let json_invalid = r#""INVALID""#;
        let result: Result<SaintCount, _> = serde_json::from_str(json_invalid);
        assert!(result.is_err());

        let json_too_large = r#"4294967296"#; // u32::MAX + 1
        let result: Result<SaintCount, _> = serde_json::from_str(json_too_large);
        assert!(result.is_err());

        // Test de sérialisation
        let many = SaintCount::Many;
        let json = serde_json::to_string(&many).unwrap();
        assert_eq!(json, r#""MANY""#);

        let number = SaintCount::Number(42);
        let json = serde_json::to_string(&number).unwrap();
        assert_eq!(json, "42");
    }
}
