use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Union types using enums
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum SaintDate {
    Year(u32),
    YearMonth(String),    // Format: "YYYY-MM"
    YearMonthDay(String), // Format: "YYYY-MM-DD"
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum SaintDateDef {
    Date(SaintDate),
    Between { between: [SaintDate; 2] },
    Or { or: Vec<SaintDate> },
    Century { century: u32 },
}
