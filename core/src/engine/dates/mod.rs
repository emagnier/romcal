//! Liturgical date calculations.
//!
//! This module provides utilities for calculating dates of movable feasts
//! and fixed celebrations in the liturgical calendar.

mod advent;
mod christmas;
mod easter;
mod feasts;
mod lent;
mod ordinary_time;
#[cfg(test)]
mod tests;

use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};
use std::collections::HashMap;

use crate::error::{RomcalResult, validate_year};
use crate::romcal::Romcal;
use crate::types::liturgical::Season;

/// Main structure for liturgical date calculations
pub struct LiturgicalDates {
    romcal: Romcal,
    year: i32,
    is_liturgical_year: bool,
}

impl LiturgicalDates {
    /// Creates a new instance of LiturgicalDates
    ///
    /// # Errors
    ///
    /// Returns `RomcalError::InvalidYear` if the year is before 1583
    pub fn new(romcal: Romcal, year: i32) -> RomcalResult<Self> {
        validate_year(year, 1583)?;
        let is_liturgical_year = romcal.context == crate::CalendarContext::Liturgical;
        Ok(Self {
            romcal,
            year,
            is_liturgical_year,
        })
    }

    /// Gets the effective year for calculations
    ///
    /// For liturgical years, uses the previous year for Advent and Christmas calculations
    fn effective_year(&self, year: Option<i32>) -> i32 {
        year.unwrap_or(if self.is_liturgical_year {
            self.year - 1
        } else {
            self.year
        })
    }

    // =================================================================================
    // Utility functions
    // =================================================================================

    /// Creates a UTC date
    pub fn get_utc_date(year: i32, month: u32, day: u32) -> DateTime<Utc> {
        NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap()
            .and_utc()
    }

    /// Adds days to a date
    pub fn add_days(date: DateTime<Utc>, days: i64) -> DateTime<Utc> {
        date + Duration::days(days)
    }

    /// Subtracts days from a date
    pub fn subtract_days(date: DateTime<Utc>, days: i64) -> DateTime<Utc> {
        date - Duration::days(days)
    }

    /// Checks if two dates are identical
    pub fn is_same_date(date1: DateTime<Utc>, date2: DateTime<Utc>) -> bool {
        date1.year() == date2.year() && date1.month() == date2.month() && date1.day() == date2.day()
    }

    /// Calculates the difference in days between two dates
    pub fn date_difference(date1: DateTime<Utc>, date2: DateTime<Utc>) -> i64 {
        (date2 - date1).num_days().abs()
    }

    /// Gets the start of the week (Sunday)
    pub fn start_of_week(date: DateTime<Utc>) -> DateTime<Utc> {
        let days_since_sunday = date.weekday().num_days_from_sunday() as i64;
        Self::subtract_days(date, days_since_sunday)
    }

    /// Checks if a date is valid
    pub fn is_valid_date(_date: &DateTime<Utc>) -> bool {
        // In Rust, if we can create a DateTime<Utc>, it's valid
        true
    }

    /// Gets the number of days in a month
    pub fn days_in_month(date: DateTime<Utc>) -> u32 {
        let next_month = if date.month() == 12 {
            Self::get_utc_date(date.year() + 1, 1, 1)
        } else {
            Self::get_utc_date(date.year(), date.month() + 1, 1)
        };

        let last_day_of_month = next_month - Duration::days(1);
        last_day_of_month.day()
    }

    /// Gets the ISO week number
    pub fn get_week_number(date: DateTime<Utc>) -> u32 {
        // Simplified implementation of ISO week number
        let year = date.year();
        let jan_1 = Self::get_utc_date(year, 1, 1);
        let days_since_jan_1 = (date - jan_1).num_days();
        let week_number =
            (days_since_jan_1 + jan_1.weekday().num_days_from_monday() as i64 + 1) / 7;
        (week_number + 1) as u32
    }

    /// Generates a range of dates between two dates inclusive
    pub fn range_of_days(start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<DateTime<Utc>> {
        let days = Self::date_difference(start, end);
        (0..=days).map(|i| Self::add_days(start, i)).collect()
    }

    /// Checks if a date exists in a range of dates
    pub fn range_contains_date(range: &[DateTime<Utc>], date: DateTime<Utc>) -> bool {
        range.iter().any(|&d| Self::is_same_date(d, date))
    }

    // =================================================================================
    // Season calculations
    // =================================================================================

    /// Gets the start of seasons for a given year
    pub fn get_start_of_seasons_dates(&self, year: Option<i32>) -> HashMap<Season, DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let mut seasons = HashMap::new();

        seasons.insert(
            Season::Advent,
            self.get_first_sunday_of_advent_date(Some(year - 1)),
        );
        seasons.insert(
            Season::ChristmasTime,
            self.get_christmas_date(Some(year - 1)),
        );
        seasons.insert(Season::Lent, self.get_ash_wednesday_date(Some(year)));
        seasons.insert(
            Season::PaschalTriduum,
            self.get_holy_thursday_date(Some(year)),
        );
        seasons.insert(
            Season::EasterTime,
            self.get_easter_sunday_date_unwrap(Some(year)),
        );
        seasons.insert(
            Season::OrdinaryTime,
            Self::add_days(self.get_baptism_of_the_lord_date(Some(year)), 1),
        );

        seasons
    }

    /// Gets a liturgical date by its ID
    ///
    /// Returns `Some(date)` for known date IDs, `None` for unknown IDs.
    /// This is used internally by `Romcal::get_date()` for fast date calculation.
    pub fn get_date_by_id(&self, id: &str) -> Option<DateTime<Utc>> {
        match id {
            // Easter and related
            "easter_sunday" => self.get_easter_sunday_date(None).ok(),
            "palm_sunday" => Some(self.get_palm_sunday_date(None)),
            "ash_wednesday" => Some(self.get_ash_wednesday_date(None)),
            "holy_thursday" => Some(self.get_holy_thursday_date(None)),
            "good_friday" => Some(self.get_good_friday_date(None)),
            "holy_saturday" => Some(self.get_holy_saturday_date(None)),
            "divine_mercy_sunday" => Some(self.get_divine_mercy_sunday_date(None)),
            "ascension" => Some(self.get_ascension_date(None)),
            "pentecost_sunday" => Some(self.get_pentecost_sunday_date(None)),
            "trinity_sunday" => Some(self.get_trinity_sunday_date(None)),
            "corpus_christi_sunday" => Some(self.get_corpus_christi_date(None)),
            "most_sacred_heart_of_jesus" => Some(self.get_most_sacred_heart_of_jesus_date(None)),
            "immaculate_heart_of_mary" => Some(self.get_immaculate_heart_of_mary_date(None)),
            "mary_mother_of_the_church" => Some(self.get_mary_mother_of_the_church_date(None)),

            // Christmas and related
            "christmas" => Some(self.get_christmas_date(None)),
            "holy_family" => Some(self.get_holy_family_date(None)),
            "epiphany_sunday" => Some(self.get_epiphany_date(None)),
            "baptism_of_the_lord" => Some(self.get_baptism_of_the_lord_date(None)),

            // Advent
            "first_sunday_of_advent" => Some(self.get_first_sunday_of_advent_date(None)),
            "christ_the_king_sunday" => Some(self.get_christ_the_king_sunday_date(None)),

            // Fixed feasts
            "mary_mother_of_god" => Some(self.get_mary_mother_of_god_date(None)),
            "presentation_of_the_lord" => Some(self.get_presentation_of_the_lord_date(None)),
            "annunciation" => Some(self.get_annunciation_date(None)),
            "nativity_of_john_the_baptist" => {
                Some(self.get_nativity_of_john_the_baptist_date(None))
            }
            "peter_and_paul_apostles" => Some(self.get_peter_and_paul_apostles_date(None)),
            "transfiguration" => Some(self.get_transfiguration_date(None)),
            "assumption" => Some(self.get_assumption_date(None)),
            "exaltation_of_the_holy_cross" => {
                Some(self.get_exaltation_of_the_holy_cross_date(None))
            }
            "all_saints" => Some(self.get_all_saints_date(None)),
            "immaculate_conception_of_mary" => {
                Some(self.get_immaculate_conception_of_mary_date(None))
            }

            // Unknown ID
            _ => None,
        }
    }

    /// Gets the end of seasons for a given year
    pub fn get_end_of_seasons_dates(&self, year: Option<i32>) -> HashMap<Season, DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let mut seasons = HashMap::new();

        seasons.insert(Season::Advent, Self::get_utc_date(year - 1, 12, 24));
        seasons.insert(
            Season::ChristmasTime,
            self.get_baptism_of_the_lord_date(Some(year)),
        );
        seasons.insert(Season::Lent, self.get_holy_thursday_date(Some(year)));
        seasons.insert(
            Season::PaschalTriduum,
            self.get_easter_sunday_date_unwrap(Some(year)),
        );
        seasons.insert(
            Season::EasterTime,
            self.get_pentecost_sunday_date(Some(year)),
        );
        seasons.insert(
            Season::OrdinaryTime,
            Self::add_days(self.get_christ_the_king_sunday_date(Some(year)), 6),
        );

        seasons
    }
}
