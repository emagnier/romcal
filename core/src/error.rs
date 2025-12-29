//! # Error management for Romcal
//!
//! This module defines all the error types used in the library,
//! optimized for use in WebAssembly.

use std::fmt;

/// Maximum year supported for calculations
pub const MAX_YEAR: i32 = 9999;

/// Main errors of the Romcal library
#[derive(Debug, Clone, PartialEq)]
pub enum RomcalError {
    /// Invalid year for calculations (year, min_year)
    InvalidYear(i32, i32),
    /// Invalid date
    InvalidDate,
    /// Liturgical calculation error
    CalculationError,
    /// Invalid configuration
    InvalidConfig,
    /// Date conversion error
    DateConversionError,
    /// Validation error
    ValidationError(String),
    /// Unknown date name
    InvalidDateName(String),
}

impl fmt::Display for RomcalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RomcalError::InvalidYear(year, min_year) => {
                write!(
                    f,
                    "Invalid year: {} (must be between {} and {})",
                    year, min_year, MAX_YEAR
                )
            }
            RomcalError::InvalidDate => {
                write!(f, "Invalid date")
            }
            RomcalError::CalculationError => {
                write!(f, "Liturgical calculation error")
            }
            RomcalError::InvalidConfig => {
                write!(f, "Invalid configuration")
            }
            RomcalError::DateConversionError => {
                write!(f, "Date conversion error")
            }
            RomcalError::ValidationError(msg) => {
                write!(f, "Validation error: {}", msg)
            }
            RomcalError::InvalidDateName(name) => {
                write!(f, "Unknown date name: {}", name)
            }
        }
    }
}

impl std::error::Error for RomcalError {}

/// Standard result type for the library
pub type RomcalResult<T> = Result<T, RomcalError>;

/// Trait for parameter validation
pub trait Validate {
    fn validate(&self) -> RomcalResult<()>;
}

/// Validation of years
pub fn validate_year(year: i32, min_year: i32) -> RomcalResult<()> {
    if year < min_year || year > MAX_YEAR {
        Err(RomcalError::InvalidYear(year, min_year))
    } else {
        Ok(())
    }
}

/// Validation of value ranges
pub fn validate_range(value: u8, min: u8, max: u8, field_name: &str) -> RomcalResult<()> {
    if value < min || value > max {
        Err(RomcalError::ValidationError(format!(
            "{} must be between {} and {} (got: {})",
            field_name, min, max, value
        )))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        // Test with Gregorian min year
        let error = RomcalError::InvalidYear(1500, 1583);
        assert_eq!(
            error.to_string(),
            "Invalid year: 1500 (must be between 1583 and 9999)"
        );

        // Test with Julian min year
        let error = RomcalError::InvalidYear(300, 326);
        assert_eq!(
            error.to_string(),
            "Invalid year: 300 (must be between 326 and 9999)"
        );
    }

    #[test]
    fn test_validate_year() {
        // Valid years
        assert!(validate_year(1583, 1583).is_ok());
        assert!(validate_year(2024, 1583).is_ok());
        assert!(validate_year(9999, 1583).is_ok());

        // Invalid years (below min)
        assert!(validate_year(1500, 1583).is_err());
        assert!(validate_year(1582, 1583).is_err());

        // Invalid years (above max)
        assert!(validate_year(10000, 1583).is_err());
    }

    #[test]
    fn test_validate_range() {
        assert!(validate_range(5, 1, 7, "week").is_ok());
        assert!(validate_range(0, 1, 7, "week").is_err());
        assert!(validate_range(8, 1, 7, "week").is_err());
    }
}
