use crate::config::create_liturgical_config;
use crate::error::RomcalCliError;

/// Parameters for the generate bundle command
pub struct GenerateBundleParams {
    pub calendar: Option<String>,
    pub locale: Option<String>,
    pub scope: Option<String>,
    pub easter_calculation_type: Option<String>,
    pub ascension_on_sunday: Option<bool>,
    pub epiphany_on_sunday: Option<bool>,
    pub corpus_christi_on_sunday: Option<bool>,
    pub output_file: Option<String>,
    pub calendar_definitions: Vec<String>,
    pub resources: Vec<String>,
}

/// Handle generate bundle command
pub fn handle_generate_bundle(params: GenerateBundleParams) -> Result<(), RomcalCliError> {
    // Create liturgical configuration
    let liturgical_config = create_liturgical_config(
        params.calendar.as_deref(),
        params.locale.as_deref(),
        params.scope.as_deref(),
        params.easter_calculation_type.as_deref(),
        params.ascension_on_sunday,
        params.corpus_christi_on_sunday,
        params.epiphany_on_sunday,
        &params.calendar_definitions,
        &params.resources,
    )?;

    // Generate the JSON bundle
    let json_bundle = liturgical_config.create_bundle()?;

    // Output the bundle
    match params.output_file {
        Some(file_path) => {
            // Write to file
            std::fs::write(&file_path, json_bundle)?;
            println!("Configuration bundle saved to: {}", file_path);
        }
        None => {
            // Print to stdout
            println!("{}", json_bundle);
        }
    }

    Ok(())
}
