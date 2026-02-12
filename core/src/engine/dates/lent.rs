//! Lent, Holy Week, and Paschal Triduum date calculations.

use chrono::{DateTime, Utc};

use super::LiturgicalDates;

impl LiturgicalDates {
    // =================================================================================
    // Lent calculations
    // =================================================================================

    /// Gets the date of Ash Wednesday
    pub fn get_ash_wednesday_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::subtract_days(self.get_easter_sunday_date_unwrap(Some(year)), 46)
    }

    /// Gets all dates of Lent
    pub fn get_all_dates_of_lent(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let start = self.get_ash_wednesday_date(Some(year));
        let end = self.get_holy_thursday_date(Some(year));
        Self::range_of_days(start, end)
    }

    /// Gets all Sundays of Lent (from Ash Wednesday to the day before Holy Thursday)
    pub fn get_all_sundays_of_lent(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let first_sunday = Self::add_days(self.get_ash_wednesday_date(Some(year)), 4);

        vec![
            first_sunday,
            Self::add_days(first_sunday, 7),
            Self::add_days(first_sunday, 14),
            Self::add_days(first_sunday, 21),
            Self::add_days(first_sunday, 28),
            Self::add_days(first_sunday, 35),
        ]
    }

    /// Gets the date of Palm Sunday
    pub fn get_palm_sunday_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::subtract_days(self.get_easter_sunday_date_unwrap(Some(year)), 7)
    }

    // =================================================================================
    // Holy Week calculations
    // =================================================================================

    /// Gets the date of Holy Thursday
    pub fn get_holy_thursday_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::subtract_days(self.get_easter_sunday_date_unwrap(Some(year)), 3)
    }

    /// Gets the date of Good Friday
    pub fn get_good_friday_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::subtract_days(self.get_easter_sunday_date_unwrap(Some(year)), 2)
    }

    /// Gets the date of Holy Saturday
    pub fn get_holy_saturday_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::subtract_days(self.get_easter_sunday_date_unwrap(Some(year)), 1)
    }

    /// Gets all dates of Holy Week
    pub fn get_all_dates_of_holy_week(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let start = self.get_palm_sunday_date(Some(year));
        let end = self.get_holy_saturday_date(Some(year));
        Self::range_of_days(start, end)
    }

    // =================================================================================
    // Paschal Triduum calculations
    // =================================================================================

    /// Gets all dates of the Paschal Triduum
    pub fn get_all_dates_of_paschal_triduum(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let start = self.get_holy_thursday_date(Some(year));
        let end = self.get_easter_sunday_date_unwrap(Some(year));
        Self::range_of_days(start, end)
    }
}
