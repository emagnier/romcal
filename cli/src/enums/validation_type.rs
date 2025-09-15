use clap::ValueEnum;

/// Type of validation to perform
#[derive(ValueEnum, Clone, Debug)]
pub enum ValidationType {
    /// Validate calendar definition JSON file
    CalendarDef,
    /// Validate resource JSON file
    Resource,
}
