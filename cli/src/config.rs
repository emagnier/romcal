use crate::error::RomcalCliError;
use chrono::Datelike;
use romcal_core::{CalendarScope, EasterCalculationType, LiturgicalConfig};

/// Create a liturgical configuration from CLI parameters
pub fn create_liturgical_config(
    calendar: Option<&str>,
    locale: Option<&str>,
    scope: Option<&str>,
    easter_calculation_type: Option<&str>,
    ascension_on_sunday: Option<bool>,
    corpus_christi_on_sunday: Option<bool>,
    epiphany_on_sunday: Option<bool>,
) -> Result<LiturgicalConfig, RomcalCliError> {
    // Parse Easter calculation type
    let easter_calculation_type = match easter_calculation_type.unwrap_or("gregorian") {
        "gregorian" => EasterCalculationType::Gregorian,
        "julian" => EasterCalculationType::Julian,
        _ => {
            return Err(RomcalCliError::invalid_calculation_type(
                easter_calculation_type.unwrap_or("unknown"),
            ))
        }
    };

    // Parse scope
    let scope = match scope.unwrap_or("gregorian") {
        "gregorian" => CalendarScope::Gregorian,
        "liturgical" => CalendarScope::Liturgical,
        _ => return Err(RomcalCliError::invalid_scope(scope.unwrap_or("unknown"))),
    };

    // Use core's with_optional_values method which handles all defaults and validation
    let config = LiturgicalConfig::with_optional_values(
        calendar,
        locale,
        easter_calculation_type,
        scope,
        epiphany_on_sunday,
        corpus_christi_on_sunday,
        ascension_on_sunday,
    );

    Ok(config)
}

/// Get current year
pub fn current_year() -> i32 {
    chrono::Utc::now().year()
}

/// Validate a year
pub fn validate_year(year: i32) -> Result<(), RomcalCliError> {
    if year < 1583 {
        Err(RomcalCliError::invalid_year(year))
    } else if year > 9999 {
        Err(RomcalCliError::invalid_year(year))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_year() {
        // Test valid years
        assert!(validate_year(1583).is_ok(), "1583 should be valid");
        assert!(validate_year(2024).is_ok(), "2024 should be valid");
        assert!(validate_year(3000).is_ok(), "3000 should be valid");
        assert!(validate_year(9999).is_ok(), "9999 should be valid");

        // Test invalid years (too small)
        assert!(validate_year(1582).is_err(), "1582 should be invalid");
        assert!(validate_year(1000).is_err(), "1000 should be invalid");
        assert!(validate_year(0).is_err(), "0 should be invalid");
        assert!(validate_year(-100).is_err(), "-100 should be invalid");

        // Test invalid years (too large)
        assert!(validate_year(10000).is_err(), "10000 should be invalid");
        assert!(validate_year(50000).is_err(), "50000 should be invalid");
    }

    #[test]
    fn test_validate_year_error_message() {
        let result = validate_year(1582);
        assert!(result.is_err());

        if let Err(RomcalCliError::InvalidYear(year)) = result {
            assert_eq!(year, 1582);
        } else {
            panic!("Expected InvalidYear error");
        }
    }

    #[test]
    fn test_validate_year_maximum_boundary() {
        // Test maximum valid year
        assert!(validate_year(9999).is_ok(), "9999 should be valid");

        // Test just over maximum
        let result = validate_year(10000);
        assert!(result.is_err(), "10000 should be invalid");

        if let Err(RomcalCliError::InvalidYear(year)) = result {
            assert_eq!(year, 10000);
        } else {
            panic!("Expected InvalidYear error");
        }
    }
}
