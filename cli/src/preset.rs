use crate::error::RomcalCliError;
use crate::utils::{
    combine_resources_by_locale, parse_calendar_definition_files, parse_resource_files,
};
use romcal_core::{CalendarScope, EasterCalculationType, Preset};

/// Create a romcal preset from CLI parameters
#[allow(clippy::too_many_arguments)]
pub fn create_preset(
    calendar: Option<&str>,
    locale: Option<&str>,
    scope: Option<CalendarScope>,
    easter_calculation_type: Option<EasterCalculationType>,
    epiphany_on_sunday: Option<bool>,
    ascension_on_sunday: Option<bool>,
    corpus_christi_on_sunday: Option<bool>,
    calendar_definitions: &[String],
    resources: &[String],
) -> Result<Preset, RomcalCliError> {
    // Start with default preset from core
    let mut preset = Preset::default();

    // Override with CLI-provided values if specified
    if let Some(cal) = calendar {
        preset.calendar = cal.to_string();
    }
    if let Some(loc) = locale {
        preset.locale = loc.to_string();
    }
    if let Some(s) = scope {
        preset.scope = s;
    }
    if let Some(easter_type) = easter_calculation_type {
        preset.easter_calculation_type = easter_type;
    }
    if let Some(epiphany) = epiphany_on_sunday {
        preset.epiphany_on_sunday = epiphany;
    }
    if let Some(ascension) = ascension_on_sunday {
        preset.ascension_on_sunday = ascension;
    }
    if let Some(corpus_christi) = corpus_christi_on_sunday {
        preset.corpus_christi_on_sunday = corpus_christi;
    }

    // Load custom calendar definitions and resources if provided
    if !calendar_definitions.is_empty() {
        preset.calendar_definitions = parse_calendar_definition_files(calendar_definitions)?;
    }
    if !resources.is_empty() {
        let parsed_resources = parse_resource_files(resources)?;
        preset.resources = combine_resources_by_locale(parsed_resources)?;
    }

    Ok(preset)
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
