//! Christmas Time date calculations.

use chrono::{DateTime, Datelike, Utc, Weekday};

use super::LiturgicalDates;

impl LiturgicalDates {
    // =================================================================================
    // Christmas calculations
    // =================================================================================

    /// Gets the date of Christmas
    pub fn get_christmas_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = self.effective_year(year);
        Self::get_christmas_date_static(year)
    }

    /// Static calculation of Christmas
    pub fn get_christmas_date_static(year: i32) -> DateTime<Utc> {
        Self::get_utc_date(year, 12, 25)
    }

    /// Gets all dates in the octave of Christmas (from Christmas to Mary Mother of God, inclusive)
    pub fn all_dates_in_octave_of_christmas(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = self.effective_year(year);
        let christmas = self.get_christmas_date(Some(year));
        let mary_mother_of_god = self.get_mary_mother_of_god_date(Some(year));

        // Octave includes Christmas + 6 days + Mary Mother of God
        let mut dates = Self::range_of_days(christmas, Self::add_days(christmas, 6));
        dates.push(mary_mother_of_god);
        dates
    }

    /// Gets the date of the nth weekday within the Octave of the Nativity of the Lord
    /// Sundays and the feast of the Holy Family are excluded
    pub fn get_weekday_within_octave_of_christmas_date(
        &self,
        day_of_octave: u8,
        year: Option<i32>,
    ) -> Option<DateTime<Utc>> {
        let year = self.effective_year(year);

        if !(1..=8).contains(&day_of_octave) {
            return None;
        }

        let christmas = self.get_christmas_date(Some(year));
        let date = Self::add_days(christmas, (day_of_octave - 1) as i64);
        let holy_family = self.get_holy_family_date(Some(year));

        // If it's the same date as Holy Family, return None
        if Self::is_same_date(date, holy_family) {
            return None;
        }

        Some(date)
    }

    /// Gets the date of the Holy Family
    pub fn get_holy_family_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = self.effective_year(year);
        let christmas = self.get_christmas_date(Some(year));
        if christmas.weekday() == Weekday::Sun {
            // If Christmas is on a Sunday, Holy Family is on December 30
            Self::get_utc_date(year, 12, 30)
        } else {
            // Holy Family is 1 week after Christmas when Christmas is on a weekday
            Self::start_of_week(Self::add_days(christmas, 7))
        }
    }

    /// Gets all dates occurring in the season of Christmas
    pub fn get_all_dates_of_christmas_time(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let start = self.get_christmas_date(Some(year));
        let end = self.get_baptism_of_the_lord_date(Some(year));
        Self::range_of_days(start, end)
    }

    /// Gets the second Sunday after the Octave of the Nativity of the Lord,
    /// which is not the Epiphany or the Baptism of the Lord
    pub fn second_sunday_after_christmas(&self, year: Option<i32>) -> Option<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);

        if self.romcal.epiphany_on_sunday {
            return None;
        }

        // Find Sunday in dates before Epiphany
        let dates_before_epiphany = self.all_dates_before_epiphany(Some(year));
        if let Some(sunday) = dates_before_epiphany
            .iter()
            .find(|d| d.weekday() == Weekday::Sun)
        {
            return Some(*sunday);
        }

        // Find Sunday in dates after Epiphany
        let dates_after_epiphany = self.all_dates_after_epiphany(Some(year));
        dates_after_epiphany
            .iter()
            .find(|d| d.weekday() == Weekday::Sun)
            .copied()
    }

    /// Gets all dates before Epiphany (and from January 2)
    pub fn all_dates_before_epiphany(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let start = Self::add_days(self.get_mary_mother_of_god_date(Some(year)), 1);
        let epiphany = self.get_epiphany_date(Some(year));

        // If there are no days between Mary, Mother of God and Epiphany
        if Self::is_same_date(start, epiphany) {
            return Vec::new();
        }

        let end = Self::subtract_days(epiphany, 1);
        Self::range_of_days(start, end)
    }

    /// Gets the date of a weekday before Epiphany (and from January 2)
    /// Only returns weekdays (Monday-Saturday), ignoring Sundays
    pub fn get_weekday_before_epiphany_date(
        &self,
        day: u8,
        year: Option<i32>,
    ) -> Option<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);

        if !(2..=8).contains(&day) {
            return None;
        }

        self.all_dates_before_epiphany(Some(year))
            .iter()
            .filter(|d| d.weekday() != Weekday::Sun) // Ignore Sundays
            .find(|d| d.day() == day as u32)
            .copied()
    }

    /// Gets the date of Epiphany
    pub fn get_epiphany_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        let first_day = Self::get_utc_date(year, 1, 1);
        let mut date = Self::get_utc_date(year, 1, 6);

        if self.romcal.epiphany_on_sunday {
            match first_day.weekday() {
                Weekday::Sat => {
                    // If the first day of the year is a Saturday, Mary Mother of God is on that day
                    // and Epiphany is the next day
                    date = Self::add_days(first_day, 1);
                }
                Weekday::Sun => {
                    // If the first day of the year is a Sunday, Mary Mother of God is on that Sunday
                    // and the following Sunday will be Epiphany
                    date = Self::add_days(first_day, 7);
                }
                _ => {
                    // If the first day of the year is a weekday (Monday-Friday),
                    // Epiphany will be celebrated on the following Sunday
                    date = Self::start_of_week(Self::add_days(first_day, 7));
                }
            }
        }

        date
    }

    /// Gets all dates after Epiphany, until the day before the Baptism of the Lord
    pub fn all_dates_after_epiphany(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let start = Self::add_days(self.get_epiphany_date(Some(year)), 1);
        let baptism_of_the_lord = self.get_baptism_of_the_lord_date(Some(year));

        // If there are no days between Epiphany and Baptism of the Lord
        if Self::is_same_date(start, baptism_of_the_lord) {
            return Vec::new();
        }

        let end = Self::subtract_days(baptism_of_the_lord, 1);
        Self::range_of_days(start, end)
    }

    /// Gets the date of a weekday after Epiphany (and before the Baptism of the Lord)
    pub fn get_weekday_after_epiphany_date(
        &self,
        dow: u8,
        year: Option<i32>,
    ) -> Option<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);

        if !(1..=6).contains(&dow) {
            return None;
        }

        self.all_dates_after_epiphany(Some(year))
            .iter()
            .find(|d| d.weekday().num_days_from_sunday() as u8 == dow)
            .copied()
    }
}
