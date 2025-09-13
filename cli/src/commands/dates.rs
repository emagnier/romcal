use crate::config::create_liturgical_config;
use crate::error::RomcalCliError;
use crate::output::validate_format;
use crate::output::OutputFormat;
use crate::utils::{current_year, validate_year};
use romcal_core::LiturgicalDates;

/// Type alias for date calculation methods
type DateMethod = fn(&LiturgicalDates, Option<i32>) -> chrono::DateTime<chrono::Utc>;

/// Date type configuration
struct DateTypeConfig {
    name: &'static str,
    method: DateMethod,
}

/// Valid date types with their corresponding methods
const DATE_TYPES: &[DateTypeConfig] = &[
    DateTypeConfig {
        name: "mary_mother_of_the_church",
        method: LiturgicalDates::get_mary_mother_of_the_church_date,
    },
    DateTypeConfig {
        name: "epiphany_sunday",
        method: LiturgicalDates::get_epiphany_date,
    },
    DateTypeConfig {
        name: "presentation_of_the_lord",
        method: LiturgicalDates::get_presentation_of_the_lord_date,
    },
    DateTypeConfig {
        name: "annunciation",
        method: LiturgicalDates::get_annunciation_date,
    },
    DateTypeConfig {
        name: "palm_sunday",
        method: LiturgicalDates::get_palm_sunday_date,
    },
    DateTypeConfig {
        name: "easter_sunday",
        method: LiturgicalDates::get_easter_sunday_date_unwrap,
    },
    DateTypeConfig {
        name: "divine_mercy_sunday",
        method: LiturgicalDates::get_divine_mercy_sunday_date,
    },
    DateTypeConfig {
        name: "immaculate_heart_of_mary",
        method: LiturgicalDates::get_immaculate_heart_of_mary_date,
    },
    DateTypeConfig {
        name: "pentecost_sunday",
        method: LiturgicalDates::get_pentecost_sunday_date,
    },
    DateTypeConfig {
        name: "corpus_christi_sunday",
        method: LiturgicalDates::get_corpus_christi_date,
    },
    DateTypeConfig {
        name: "nativity_of_john_the_baptist",
        method: LiturgicalDates::get_nativity_of_john_the_baptist_date,
    },
    DateTypeConfig {
        name: "peter_and_paul_apostles",
        method: LiturgicalDates::get_peter_and_paul_apostles_date,
    },
    DateTypeConfig {
        name: "transfiguration",
        method: LiturgicalDates::get_transfiguration_date,
    },
    DateTypeConfig {
        name: "assumption",
        method: LiturgicalDates::get_assumption_date,
    },
    DateTypeConfig {
        name: "exaltation_of_the_holy_cross",
        method: LiturgicalDates::get_exaltation_of_the_holy_cross_date,
    },
    DateTypeConfig {
        name: "all_saints",
        method: LiturgicalDates::get_all_saints_date,
    },
    DateTypeConfig {
        name: "immaculate_conception_of_mary",
        method: LiturgicalDates::get_immaculate_conception_of_mary_date,
    },
];

/// Validate date type and return the corresponding method
fn validate_and_get_date_method(date_type: &str) -> Result<DateMethod, RomcalCliError> {
    DATE_TYPES
        .iter()
        .find(|config| config.name == date_type)
        .map(|config| config.method)
        .ok_or_else(|| {
            let valid_types: Vec<&str> = DATE_TYPES.iter().map(|config| config.name).collect();
            RomcalCliError::config_error(format!(
                "Invalid date type: '{}'. Valid types are: {}",
                date_type,
                valid_types.join(", ")
            ))
        })
}

/// Handle dates command
#[allow(clippy::too_many_arguments)]
pub fn handle(
    date_type: &str,
    year: Option<i32>,
    calendar: Option<&str>,
    locale: Option<&str>,
    format: &str,
    scope: Option<&str>,
    easter_calculation_type: Option<&str>,
    ascension_on_sunday: Option<bool>,
    epiphany_on_sunday: Option<bool>,
    corpus_christi_on_sunday: Option<bool>,
    calendar_definitions: &[String],
    resources: &[String],
) -> Result<(), RomcalCliError> {
    let year = year.unwrap_or_else(current_year);

    validate_year(year)?;

    validate_format(format)?;

    // Parse output format
    let output_format = match format.to_lowercase().as_str() {
        "json" => OutputFormat::Json,
        "csv" => OutputFormat::Csv,
        "yaml" => OutputFormat::Yaml,
        "lines" => OutputFormat::Lines,
        _ => unreachable!(), // Already validated above
    };

    // Get the date calculation method
    let date_method = validate_and_get_date_method(date_type)?;

    // Create liturgical configuration
    let liturgical_config = create_liturgical_config(
        calendar,
        locale,
        scope,
        easter_calculation_type,
        ascension_on_sunday,
        corpus_christi_on_sunday,
        epiphany_on_sunday,
        calendar_definitions,
        resources,
    )?;

    // Create liturgical dates instance
    let dates = LiturgicalDates::new(liturgical_config, year)?;

    // Calculate the requested date
    let date_result = date_method(&dates, None);

    // Output the result - only the date in YYYY-MM-DD format
    let date_string = date_result.format("%Y-%m-%d").to_string();
    output_format.print(&date_string)?;

    Ok(())
}
