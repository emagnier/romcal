//! Advent date calculations.

use chrono::{DateTime, Datelike, Utc, Weekday};

use super::LiturgicalDates;

impl LiturgicalDates {
    // =================================================================================
    // Advent calculations
    // =================================================================================

    /// Gets all dates of Advent
    pub fn get_all_dates_of_advent(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = self.effective_year(year);
        let start = self.get_first_sunday_of_advent_date(Some(year));
        let end = Self::subtract_days(self.get_christmas_date(Some(year)), 1);
        Self::range_of_days(start, end)
    }

    /// Gets all Sundays of Advent
    pub fn get_all_sundays_of_advent(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = self.effective_year(year);
        let first_sunday = self.get_first_sunday_of_advent_date(Some(year));

        vec![
            first_sunday,
            Self::add_days(first_sunday, 7),
            Self::add_days(first_sunday, 14),
            Self::add_days(first_sunday, 21),
        ]
    }

    /// Gets the date of the first Sunday of Advent
    pub fn get_first_sunday_of_advent_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = self.effective_year(year);
        Self::get_first_sunday_of_advent_date_static(year)
    }

    /// Static calculation of the first Sunday of Advent
    pub fn get_first_sunday_of_advent_date_static(year: i32) -> DateTime<Utc> {
        let christmas = Self::get_christmas_date_static(year);
        match christmas.weekday() {
            Weekday::Sun => Self::get_utc_date(year, 11, 27),
            Weekday::Mon => Self::get_utc_date(year, 12, 3),
            Weekday::Tue => Self::get_utc_date(year, 12, 2),
            Weekday::Wed => Self::get_utc_date(year, 12, 1),
            Weekday::Thu => Self::get_utc_date(year, 11, 30),
            Weekday::Fri => Self::get_utc_date(year, 11, 29),
            Weekday::Sat => Self::get_utc_date(year, 11, 28),
        }
    }

    /// Gets the date of an unprivileged weekday of Advent (until 16 December)
    pub fn unprivileged_weekday_of_advent(
        &self,
        dow: u8,
        week: u8,
        year: Option<i32>,
    ) -> Option<DateTime<Utc>> {
        let year = self.effective_year(year);

        if !(1..=6).contains(&dow) || !(1..=4).contains(&week) {
            return None;
        }

        let first_sunday = self.get_first_sunday_of_advent_date(Some(year));
        let date = Self::add_days(first_sunday, (week - 1) as i64 * 7 + dow as i64);

        // If the date is on or after December 17 and it's not a Sunday, return None
        if date.month() == 12 && date.day() >= 17 && date.weekday() != Weekday::Sun {
            return None;
        }

        Some(date)
    }

    /// Gets the date of a privileged weekday within Advent, from 17 to 24 December, Sundays excluded
    pub fn privileged_weekday_of_advent(
        &self,
        day: u8,
        year: Option<i32>,
    ) -> Option<DateTime<Utc>> {
        let year = self.effective_year(year);

        if !(17..=24).contains(&day) {
            return None;
        }

        let date = Self::get_utc_date(year, 12, day as u32);

        // If it's a Sunday, return None
        if date.weekday() == Weekday::Sun {
            return None;
        }

        Some(date)
    }

    /// Gets the date of a Sunday of Advent (1st to 4th)
    pub fn get_sunday_of_advent_date(&self, week: u8, year: Option<i32>) -> Option<DateTime<Utc>> {
        let year = self.effective_year(year);

        if !(1..=4).contains(&week) {
            return None;
        }

        let first_sunday = self.get_first_sunday_of_advent_date(Some(year));
        Some(Self::add_days(first_sunday, (week - 1) as i64 * 7))
    }
}
