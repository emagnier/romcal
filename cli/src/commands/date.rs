use crate::enums::OutputFormat;
use crate::error::RomcalCliError;
use crate::utils::{current_year, validate_year};
use romcal::Romcal;

/// Handle date command
pub fn handle(
    id: &str,
    year: Option<i32>,
    output_format: OutputFormat,
    romcal: Romcal,
) -> Result<(), RomcalCliError> {
    let year = year.unwrap_or_else(current_year);
    validate_year(year)?;

    let date_string = romcal.get_date(id, year)?;
    output_format.print(&date_string)?;

    Ok(())
}
