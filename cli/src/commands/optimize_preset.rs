use crate::error::RomcalCliError;
use romcal_core::Romcal;

/// Handle optimize preset command
pub fn handle(romcal: Romcal, output_file: Option<String>) -> Result<(), RomcalCliError> {
    // Optimize the preset
    let optimized_preset = romcal.optimize()?;

    // Output the optimized preset
    match output_file {
        Some(file_path) => {
            // Write to file
            std::fs::write(&file_path, optimized_preset)?;
            println!("Preset file saved to: {}", file_path);
        }
        None => {
            // Print to stdout
            println!("{}", optimized_preset);
        }
    }

    Ok(())
}
