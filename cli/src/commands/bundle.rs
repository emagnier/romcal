use crate::error::RomcalCliError;
use romcal::Romcal;

/// Handle bundle command
pub fn handle(romcal: Romcal) -> Result<(), RomcalCliError> {
    let bundle = romcal.create_bundle()?;
    println!("{}", bundle);
    Ok(())
}
