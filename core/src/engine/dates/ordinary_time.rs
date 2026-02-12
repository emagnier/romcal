//! Ordinary Time date calculations.

use chrono::{DateTime, Datelike, Utc, Weekday};

use super::LiturgicalDates;

impl LiturgicalDates {
    // =================================================================================
    // Ordinary Time calculations
    // =================================================================================

    /// Gets all dates occurring in Ordinary Time
    pub fn get_all_dates_of_ordinary_time(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let mut early = self.get_all_dates_of_early_ordinary_time(Some(year));
        let mut late = self.get_all_dates_of_late_ordinary_time(Some(year));
        early.append(&mut late);
        early
    }

    /// Gets all dates of early Ordinary Time
    /// Ordinary Time in the early part of the year begins
    /// the day after the Baptism of the Lord and concludes
    /// the day before Ash Wednesday
    pub fn get_all_dates_of_early_ordinary_time(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let start = Self::add_days(self.get_baptism_of_the_lord_date(Some(year)), 1);
        let end = Self::subtract_days(self.get_ash_wednesday_date(Some(year)), 1);
        Self::range_of_days(start, end)
    }

    /// Gets all Sundays that fall within the period of early Ordinary Time
    pub fn get_all_sundays_of_early_ordinary_time(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        self.get_all_dates_of_early_ordinary_time(Some(year))
            .into_iter()
            .filter(|d| d.weekday() == Weekday::Sun)
            .collect()
    }

    /// Gets all dates of late Ordinary Time
    /// Ordinary Time after Pentecost to the day before the First Sunday of Advent
    pub fn get_all_dates_of_late_ordinary_time(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let start = Self::add_days(self.get_pentecost_sunday_date(Some(year)), 1);
        let end = Self::subtract_days(self.get_first_sunday_of_advent_date(Some(year)), 1);
        Self::range_of_days(start, end)
    }

    /// Gets all Sundays that fall within the period of late Ordinary Time
    pub fn get_all_sundays_of_late_ordinary_time(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        self.get_all_dates_of_late_ordinary_time(Some(year))
            .into_iter()
            .filter(|d| d.weekday() == Weekday::Sun)
            .collect()
    }

    /// Gets a specific date of Ordinary Time by day of week and week number
    pub fn get_date_in_ordinary_time(
        &self,
        dow: u8,
        week: u8,
        year: Option<i32>,
    ) -> Option<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);

        if dow > 6 || !(1..=35).contains(&week) {
            return None;
        }

        let early_dates = self.get_all_dates_of_early_ordinary_time(Some(year));
        let late_dates = self.get_all_dates_of_late_ordinary_time(Some(year));

        // Calculate the starting week number for late Ordinary Time
        let late_ordinary_start_week = 35 - (late_dates.len() + 1) / 7;

        // Group dates by week and day of week
        let mut grouped_dates: std::collections::HashMap<(u8, u8), DateTime<Utc>> =
            std::collections::HashMap::new();

        // Process early Ordinary Time dates
        for (idx, date) in early_dates.iter().enumerate() {
            let week_number = (idx / 7) as u8 + 1;
            let day_of_week = date.weekday().num_days_from_sunday() as u8;
            grouped_dates.insert((week_number, day_of_week), *date);
        }

        // Process late Ordinary Time dates
        for (idx, date) in late_dates.iter().enumerate() {
            let week_number = if date.weekday() == Weekday::Sun {
                late_ordinary_start_week + (idx / 7) + 1
            } else {
                late_ordinary_start_week + (idx / 7)
            };
            let day_of_week = date.weekday().num_days_from_sunday() as u8;
            grouped_dates.insert((week_number as u8, day_of_week), *date);
        }

        grouped_dates.get(&(week, dow)).copied()
    }
}
