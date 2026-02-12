//! Easter Time date calculations.

use chrono::{DateTime, Utc};

use super::LiturgicalDates;
use crate::error::RomcalResult;
use crate::types::EasterCalculationType;

impl LiturgicalDates {
    // =================================================================================
    // Easter calculations
    // =================================================================================

    /// Gets the date of Easter Sunday
    ///
    /// # Errors
    ///
    /// Returns `RomcalError::InvalidYear` if the year is before 1583
    pub fn get_easter_sunday_date(&self, year: Option<i32>) -> RomcalResult<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);

        let easter_date = match self.romcal.easter_calculation_type {
            EasterCalculationType::Gregorian => {
                super::super::easter::calculate_gregorian_easter_date(year)?
            }
            EasterCalculationType::Julian => {
                super::super::easter::calculate_julian_easter_date_to_gregorian(year)?
            }
        };
        easter_date.to_utc_date()
    }

    /// Gets the date of Easter Sunday (compatibility method that panics on error)
    ///
    /// # Panics
    ///
    /// Panics if the year is invalid or if there's a calculation error
    pub fn get_easter_sunday_date_unwrap(&self, year: Option<i32>) -> DateTime<Utc> {
        self.get_easter_sunday_date(year)
            .expect("Invalid year or calculation error")
    }

    /// Gets all dates occurring during the octave of Easter
    /// from Easter Sunday until the Sunday following Easter (Divine Mercy Sunday), inclusive
    pub fn all_dates_in_octave_of_easter(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let start = self.get_easter_sunday_date_unwrap(Some(year));
        let end = self.get_divine_mercy_sunday_date(Some(year));
        Self::range_of_days(start, end)
    }

    /// Gets all Sundays of Easter
    /// Easter Time is the period of fifty days from Easter Sunday to Pentecost Sunday (inclusive).
    /// All Sundays in this period are counted as Sundays of Easter.
    pub fn get_all_sundays_of_easter(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let first_sunday = self.get_easter_sunday_date_unwrap(Some(year));

        vec![
            first_sunday,
            Self::add_days(first_sunday, 7),
            Self::add_days(first_sunday, 14),
            Self::add_days(first_sunday, 21),
            Self::add_days(first_sunday, 28),
            Self::add_days(first_sunday, 35),
            Self::add_days(first_sunday, 42),
            Self::add_days(first_sunday, 49),
        ]
    }

    /// Gets a weekday or Sunday of Easter Time
    pub fn get_date_in_easter_time(
        &self,
        dow: u8,
        week: u8,
        year: Option<i32>,
    ) -> Option<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);

        if !(1..=7).contains(&week) || dow > 6 {
            return None;
        }

        let date = Self::add_days(
            self.get_easter_sunday_date_unwrap(Some(year)),
            ((week - 1) * 7 + dow) as i64,
        );
        let ascension = self.get_ascension_date(Some(year));

        // If it's the same date as Ascension, return None
        if Self::is_same_date(date, ascension) {
            return None;
        }

        Some(date)
    }

    /// Gets all dates occurring in Easter Time
    /// Easter Time is the period of fifty days from Easter Sunday to Pentecost Sunday
    pub fn get_all_dates_of_easter_time(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let start = self.get_easter_sunday_date_unwrap(Some(year));
        let end = self.get_pentecost_sunday_date(Some(year));
        Self::range_of_days(start, end)
    }

    /// Gets the date of Divine Mercy Sunday
    pub fn get_divine_mercy_sunday_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::add_days(self.get_easter_sunday_date_unwrap(Some(year)), 7)
    }

    /// Gets the date of Ascension
    pub fn get_ascension_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        if self.romcal.ascension_on_sunday {
            // Ascension on the 7th Sunday of Easter (42 days after Easter)
            Self::add_days(self.get_easter_sunday_date_unwrap(Some(year)), 42)
        } else {
            // Ascension on Thursday (39 days after Easter)
            Self::add_days(self.get_easter_sunday_date_unwrap(Some(year)), 39)
        }
    }

    /// Gets the date of Pentecost
    pub fn get_pentecost_sunday_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::add_days(self.get_easter_sunday_date_unwrap(Some(year)), 49)
    }
}
