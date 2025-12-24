use clap::ValueEnum;
use romcal::EasterCalculationType;

/// Easter calculation type for CLI.
/// Defines the algorithm used to calculate the date of Easter.
#[derive(ValueEnum, Clone, Debug)]
pub enum CliEasterCalculationType {
    /// Gregorian calculation (default)
    Gregorian,
    /// Julian calculation converted to Gregorian
    Julian,
}

impl From<CliEasterCalculationType> for EasterCalculationType {
    fn from(easter_type: CliEasterCalculationType) -> Self {
        match easter_type {
            CliEasterCalculationType::Gregorian => EasterCalculationType::Gregorian,
            CliEasterCalculationType::Julian => EasterCalculationType::Julian,
        }
    }
}
