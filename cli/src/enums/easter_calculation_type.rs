use clap::ValueEnum;
use romcal_core::EasterCalculationType;

/// Easter calculation type for CLI
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
