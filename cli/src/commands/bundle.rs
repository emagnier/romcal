use crate::error::RomcalCliError;
use romcal::Romcal;

/// Handle bundle command
pub fn handle(romcal: Romcal) -> Result<(), RomcalCliError> {
    let optimized_preset = romcal.optimize()?;
    println!("{}", optimized_preset);
    Ok(())
}
