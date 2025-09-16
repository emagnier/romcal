use clap::ValueEnum;
use romcal_core::CalendarContext;

/// Calendar context for CLI
#[derive(ValueEnum, Clone, Debug)]
pub enum CliCalendarContext {
    /// Civil year (January 1 to December 31)
    Gregorian,
    /// Liturgical year (first Sunday of Advent to the day before the first Sunday of Advent of the next year)
    Liturgical,
}

impl From<CliCalendarContext> for CalendarContext {
    fn from(context: CliCalendarContext) -> Self {
        match context {
            CliCalendarContext::Gregorian => CalendarContext::Gregorian,
            CliCalendarContext::Liturgical => CalendarContext::Liturgical,
        }
    }
}
