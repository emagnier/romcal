#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Saint date representation with different precision levels.
/// Supports year-only, year-month, or full date specifications.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(untagged)]
pub enum SaintDate {
    /// Year only (e.g., 1234)
    Year(u32),
    /// Year and month in "YYYY-MM" format (e.g., "1234-05")
    YearMonth(String),
    /// Full date in "YYYY-MM-DD" format (e.g., "1234-05-15")
    YearMonthDay(String),
}

/// Saint date definition supporting various date specifications.
/// Allows single dates, date ranges, multiple alternatives, or century specifications.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(untagged)]
pub enum SaintDateDef {
    /// Single date specification
    Date(SaintDate),
    /// Date range between two dates
    Between {
        /// The date range (start and end dates)
        between: [SaintDate; 2],
    },
    /// Multiple alternative dates (any one of them)
    Or {
        /// The list of alternative dates
        or: Vec<SaintDate>,
    },
    /// Century specification (e.g., 12 for 12th century)
    Century {
        /// The century number
        century: u32,
    },
}
