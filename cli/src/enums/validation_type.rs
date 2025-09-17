use clap::ValueEnum;

/// Type of validation to perform.
/// Defines the different types of JSON files that can be validated by the CLI.
#[derive(ValueEnum, Clone, Debug)]
pub enum ValidationType {
    /// Validate calendar definition JSON file
    CalendarDef,
    /// Validate resource JSON file
    Resource,
}
