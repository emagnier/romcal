use crate::error::RomcalCliError;
use romcal_core::LiturgicalConfig;

/// Handle generate bundle command
pub fn handle_generate_bundle(
    liturgical_config: LiturgicalConfig,
    output_file: Option<String>,
) -> Result<(), RomcalCliError> {
    // Generate the JSON bundle
    let json_bundle = liturgical_config.create_bundle()?;

    // Output the bundle
    match output_file {
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
