#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::{DateFn, DayOfWeek, MonthIndex};

/// Date definition supporting various date calculation methods.
/// Provides flexible ways to specify liturgical dates using different approaches.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(untagged)]
pub enum DateDef {
    /// Simple month/day specification
    MonthDate {
        /// The month (1-12)
        month: MonthIndex,
        /// The day of the month (1-31)
        date: u8,
        /// Optional day offset for adjustments
        #[serde(skip_serializing_if = "Option::is_none")]
        day_offset: Option<i32>,
    },
    /// Date function calculation (Easter, Epiphany, etc.)
    DateFunction {
        /// The date function to calculate the base date
        date_fn: DateFn,
        /// Optional day offset for adjustments
        #[serde(skip_serializing_if = "Option::is_none")]
        day_offset: Option<i32>,
    },
    /// Nth weekday of a specific month
    WeekdayOfMonth {
        /// The month (1-12)
        month: MonthIndex,
        /// The day of the week (0=Sunday, 1=Monday, etc.)
        day_of_week: DayOfWeek,
        /// Which occurrence of the weekday (1st, 2nd, 3rd, etc.)
        nth_week_in_month: u8,
        /// Optional day offset for adjustments
        #[serde(skip_serializing_if = "Option::is_none")]
        day_offset: Option<i32>,
    },
    /// Last weekday of a specific month
    LastWeekdayOfMonth {
        /// The month (1-12)
        month: MonthIndex,
        /// The day of the week to find the last occurrence of
        last_day_of_week_in_month: DayOfWeek,
        /// Optional day offset for adjustments
        #[serde(skip_serializing_if = "Option::is_none")]
        day_offset: Option<i32>,
    },
    /// Inherited from the proper of time
    InheritedFromProperOfTime {},
}

/// Date definition with offset for adjustments.
/// Used when a date needs to be shifted by a specific number of days.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
pub struct DateDefWithOffset {
    /// The number of days to offset the date
    pub day_offset: i32,
}

/// Extended date definition supporting both regular dates and offset dates.
/// Provides flexibility for date calculations with optional adjustments.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(untagged)]
pub enum DateDefExtended {
    /// Regular date definition
    DateDef(DateDef),
    /// Date definition with offset
    WithOffset(DateDefWithOffset),
}

/// The liturgical day date exception.
/// Represents a condition and the date to set when that condition is met.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
pub struct DateDefException {
    /// The condition that triggers the exception
    pub when: ExceptionCondition,
    /// The date to set when the condition is met
    pub then: DateDefExtended,
}

/// Exception conditions that can trigger a date change.
/// Defines various conditions under which a date exception applies.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(untagged)]
pub enum ExceptionCondition {
    /// If the date is between two specified dates
    IsBetween {
        /// The start date of the range
        from: Box<DateDef>,
        /// The end date of the range
        to: Box<DateDef>,
        /// Whether the range is inclusive of the start date and the end date
        inclusive: bool,
    },
    /// If the date is the same as another specified date
    IsSameAsDate {
        /// The date to compare against
        date: Box<DateDef>,
    },
    /// If the date falls on a specific day of the week
    IsDayOfWeek {
        /// The day of the week to match
        day_of_week: DayOfWeek,
    },
}

/// Date exceptions that can be either a single exception or multiple exceptions.
/// Supports both simple single exceptions and complex multiple exception scenarios.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(untagged)]
pub enum DateDefExceptions {
    /// Single date exception
    Single(DateDefException),
    /// Multiple date exceptions
    Multiple(Vec<DateDefException>),
}
