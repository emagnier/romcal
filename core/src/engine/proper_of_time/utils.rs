//! Utility functions for Proper of Time generation.
//!
//! This module provides helper functions and constants used across
//! all season generators.

use crate::engine::liturgical_day::LiturgicalDay;

/// Calendar ID for the Proper of Time.
pub const PROPER_OF_TIME_ID: &str = "proper_of_time";

/// Weekday names for liturgical day generation
pub const WEEKDAY_NAMES: [&str; 7] = [
    "sunday",
    "monday",
    "tuesday",
    "wednesday",
    "thursday",
    "friday",
    "saturday",
];

/// Helper function to convert an enum to its string representation using Serde
pub fn enum_to_string<T>(value: &T) -> String
where
    T: serde::Serialize,
{
    serde_json::to_string(value)
        .unwrap_or_default()
        .trim_matches('"')
        .to_string()
}

/// Helper function to sort liturgical days by date in chronological order
pub fn sort_liturgical_days_by_date(days: &mut [LiturgicalDay]) {
    days.sort_by(|a, b| {
        // Parse dates and compare chronologically
        let date_a = chrono::NaiveDate::parse_from_str(&a.date, "%Y-%m-%d").unwrap_or_default();
        let date_b = chrono::NaiveDate::parse_from_str(&b.date, "%Y-%m-%d").unwrap_or_default();
        date_a.cmp(&date_b)
    });
}
