use crate::error::RomcalCliError;
use crate::utils::{
    combine_resources_by_locale, parse_calendar_definition_files, parse_resource_files,
};
use romcal_core::config::LiturgicalConfigPartial;
use romcal_core::{CalendarScope, EasterCalculationType, LiturgicalConfig};

/// Create a liturgical configuration from CLI parameters
#[allow(clippy::too_many_arguments)]
pub fn create_liturgical_config(
    calendar: Option<&str>,
    locale: Option<&str>,
    scope: Option<&str>,
    easter_calculation_type: Option<&str>,
    epiphany_on_sunday: Option<bool>,
    ascension_on_sunday: Option<bool>,
    corpus_christi_on_sunday: Option<bool>,
    calendar_definitions: &[String],
    resources: &[String],
) -> Result<LiturgicalConfig, RomcalCliError> {
    // Parse scope
    let scope = match scope.unwrap_or("gregorian") {
        "gregorian" => CalendarScope::Gregorian,
        "liturgical" => CalendarScope::Liturgical,
        _ => return Err(RomcalCliError::invalid_scope(scope.unwrap_or("unknown"))),
    };

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

    // Load custom calendar definitions and resources if provided
    let calendar_def_data = parse_calendar_definition_files(calendar_definitions)?;
    let resources_data = if !resources.is_empty() {
        // Parse resource files
        let parsed_resources = parse_resource_files(resources)?;
        // Combine resources by locale (deep merge metadata, concatenate entities)
        combine_resources_by_locale(parsed_resources)?
    } else {
        Vec::new()
    };

    // Create a romcal config
    let config = LiturgicalConfig::new(LiturgicalConfigPartial {
        calendar: calendar.map(|s| s.to_string()),
        locale: locale.map(|s| s.to_string()),
        scope: Some(scope),
        easter_calculation_type: Some(easter_calculation_type),
        epiphany_on_sunday,
        corpus_christi_on_sunday,
        ascension_on_sunday,
        calendar_definitions: Some(calendar_def_data),
        resources: Some(resources_data),
    });

    Ok(config)
}

#[cfg(test)]
mod tests {
    use crate::utils::validate_year;

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
