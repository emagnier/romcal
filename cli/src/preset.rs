use crate::error::RomcalCliError;
use crate::utils::{
    combine_resources_by_locale, parse_calendar_definition_files, parse_resource_files,
};
use romcal::engine::{CalendarDefinition, Resources};
use romcal::{CalendarContext, EasterCalculationType, Romcal};

/// Load bundled calendar definitions and resources.
fn load_bundled_data() -> Result<(Vec<CalendarDefinition>, Vec<Resources>), RomcalCliError> {
    let definitions = romcal::bundled_data::get_all_calendar_definitions().map_err(|e| {
        RomcalCliError::ConfigError(format!("Failed to load bundled definitions: {}", e))
    })?;
    let resources = romcal::bundled_data::get_all_resources().map_err(|e| {
        RomcalCliError::ConfigError(format!("Failed to load bundled resources: {}", e))
    })?;
    Ok((definitions, resources))
}

/// Merge custom calendar definitions into bundled ones.
/// Custom definitions with same ID override bundled ones.
fn merge_calendar_definitions(base: &mut Vec<CalendarDefinition>, custom: Vec<CalendarDefinition>) {
    for custom_def in custom {
        base.retain(|def| def.id != custom_def.id);
        base.push(custom_def);
    }
}

/// Create a romcal instance from CLI parameters
#[allow(clippy::too_many_arguments)]
pub fn create_romcal(
    calendar: Option<&str>,
    locale: Option<&str>,
    context: Option<CalendarContext>,
    easter_calculation_type: Option<EasterCalculationType>,
    epiphany_on_sunday: Option<bool>,
    ascension_on_sunday: Option<bool>,
    corpus_christi_on_sunday: Option<bool>,
    calendar_definitions: &[String],
    resources: &[String],
    replace: bool,
) -> Result<Romcal, RomcalCliError> {
    let has_custom_data = !calendar_definitions.is_empty() || !resources.is_empty();

    // 1. Determine base data
    let (mut all_definitions, mut all_resources) = if replace && has_custom_data {
        // Replace mode: start with empty vectors
        (Vec::new(), Vec::new())
    } else {
        // Merge mode (default): load bundled data
        load_bundled_data()?
    };

    // 2. Add/replace custom definitions if provided
    if !calendar_definitions.is_empty() {
        let custom_definitions = parse_calendar_definition_files(calendar_definitions)?;
        if replace {
            all_definitions = custom_definitions;
        } else {
            merge_calendar_definitions(&mut all_definitions, custom_definitions);
        }
    }

    // 3. Add/replace custom resources if provided
    if !resources.is_empty() {
        let custom_resources = parse_resource_files(resources)?;
        if replace {
            all_resources = combine_resources_by_locale(custom_resources)?;
        } else {
            // Merge: combine bundled + custom resources by locale
            all_resources.extend(custom_resources);
            all_resources = combine_resources_by_locale(all_resources)?;
        }
    }

    // 4. Create romcal instance with loaded data
    let mut romcal = Romcal {
        calendar_definitions: all_definitions,
        resources: all_resources,
        ..Romcal::default()
    };

    // 5. Override with CLI-provided values if specified
    if let Some(cal) = calendar {
        romcal.calendar = cal.to_string();
    }
    if let Some(loc) = locale {
        romcal.locale = loc.to_string();
    }
    if let Some(c) = context {
        romcal.context = c;
    }
    if let Some(easter_type) = easter_calculation_type {
        romcal.easter_calculation_type = easter_type;
    }
    if let Some(epiphany) = epiphany_on_sunday {
        romcal.epiphany_on_sunday = epiphany;
    }
    if let Some(ascension) = ascension_on_sunday {
        romcal.ascension_on_sunday = ascension;
    }
    if let Some(corpus_christi) = corpus_christi_on_sunday {
        romcal.corpus_christi_on_sunday = corpus_christi;
    }

    Ok(romcal)
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
