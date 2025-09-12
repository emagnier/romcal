use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::{DateFn, DayOfWeek, MonthIndex};

// Union types using enums
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum DateDef {
    /// Simple month/day
    MonthDate {
        month: MonthIndex,
        date: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        day_offset: Option<i32>,
    },
    /// Date function (Easter, Epiphany, etc.)
    DateFunction {
        date_fn: DateFn,
        #[serde(skip_serializing_if = "Option::is_none")]
        day_offset: Option<i32>,
    },
    /// Nth weekday of month
    WeekdayOfMonth {
        month: MonthIndex,
        day_of_week: DayOfWeek,
        nth_week_in_month: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        day_offset: Option<i32>,
    },
    /// Last weekday of month
    LastWeekdayOfMonth {
        month: MonthIndex,
        last_day_of_week_in_month: DayOfWeek,
        #[serde(skip_serializing_if = "Option::is_none")]
        day_offset: Option<i32>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct DateDefWithOffset {
    pub day_offset: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum DateDefExtended {
    DateDef(DateDef),
    WithOffset(DateDefWithOffset),
}

/// The liturgical day date exception
/// Represents a condition and the date to set when that condition is met
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct DateDefException {
    /// Condition that triggers the exception
    pub when: ExceptionCondition,
    /// Date to set when condition is met
    pub then: DateDefExtended,
}

/// Exception conditions that can trigger a date change
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ExceptionCondition {
    /// If date is between two dates
    IsBetween {
        from: Box<DateDef>,
        to: Box<DateDef>,
        inclusive: bool,
    },
    /// If date is same as another date
    IsSameAsDate { date: Box<DateDef> },
    /// If date is a specific day of week
    IsDayOfWeek { day_of_week: DayOfWeek },
}

/// Date exceptions that can be either a single exception or an array of exceptions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum DateDefExceptions {
    Single(DateDefException),
    Multiple(Vec<DateDefException>),
}
