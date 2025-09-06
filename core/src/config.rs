use serde::{Deserialize, Serialize};

/// Easter calculation type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EasterCalculationType {
    /// Gregorian calculation (default)
    Gregorian,
    /// Julian calculation converted to Gregorian
    Julian,
}

/// Calendar scope
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CalendarScope {
    /// Civil year (January 1 to December 31)
    Gregorian,
    /// Liturgical year (first Sunday of Advent to the day before the first Sunday of Advent of the next year)
    Liturgical,
}

/// Simplified configuration for liturgical date calculations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiturgicalConfig {
    /// Easter calculation type
    pub easter_calculation_type: EasterCalculationType,
    /// Calendar scope
    pub scope: CalendarScope,
    /// Epiphany is celebrated on a Sunday (between January 2-8)
    pub epiphany_on_sunday: bool,
    /// Corpus Christi is celebrated on a Sunday
    pub corpus_christi_on_sunday: bool,
    /// Ascension is celebrated on a Sunday (7th Sunday of Easter)
    pub ascension_on_sunday: bool,
}

impl Default for LiturgicalConfig {
    fn default() -> Self {
        Self {
            easter_calculation_type: EasterCalculationType::Gregorian,
            scope: CalendarScope::Gregorian,
            epiphany_on_sunday: false,
            corpus_christi_on_sunday: true,
            ascension_on_sunday: false,
        }
    }
}

impl LiturgicalConfig {
    /// Creates a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a custom configuration
    pub fn custom(
        easter_calculation_type: EasterCalculationType,
        scope: CalendarScope,
        epiphany_on_sunday: bool,
        corpus_christi_on_sunday: bool,
        ascension_on_sunday: bool,
    ) -> Self {
        Self {
            easter_calculation_type,
            scope,
            epiphany_on_sunday,
            corpus_christi_on_sunday,
            ascension_on_sunday,
        }
    }

    /// Configuration for standard Roman rite
    pub fn roman_rite() -> Self {
        Self {
            easter_calculation_type: EasterCalculationType::Gregorian,
            scope: CalendarScope::Gregorian,
            epiphany_on_sunday: false,
            corpus_christi_on_sunday: true,
            ascension_on_sunday: false,
        }
    }

    /// Configuration for United States (Ascension and Corpus Christi on Sunday)
    pub fn united_states() -> Self {
        Self {
            easter_calculation_type: EasterCalculationType::Gregorian,
            scope: CalendarScope::Gregorian,
            epiphany_on_sunday: false,
            corpus_christi_on_sunday: true,
            ascension_on_sunday: true,
        }
    }

    /// Configuration for countries where Epiphany is celebrated on a Sunday
    pub fn epiphany_on_sunday() -> Self {
        Self {
            easter_calculation_type: EasterCalculationType::Gregorian,
            scope: CalendarScope::Gregorian,
            epiphany_on_sunday: true,
            corpus_christi_on_sunday: true,
            ascension_on_sunday: false,
        }
    }
}
