use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc, Weekday};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::easter::{calculate_gregorian_easter_date, calculate_julian_easter_date_to_gregorian};
use crate::error::{validate_year, RomcalResult};
use crate::{EasterCalculationType, LiturgicalConfig};

/// Liturgical seasons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Season {
    /// Advent
    Advent,
    /// Christmas Time
    ChristmasTime,
    /// Lent
    Lent,
    /// Paschal Triduum
    PaschalTriduum,
    /// Easter Time
    EasterTime,
    /// Ordinary Time
    OrdinaryTime,
}

/// Main structure for liturgical date calculations
pub struct LiturgicalDates {
    config: LiturgicalConfig,
    year: i32,
    is_liturgical_year: bool,
}

impl LiturgicalDates {
    /// Creates a new instance of LiturgicalDates
    ///
    /// # Errors
    ///
    /// Returns `RomcalError::InvalidYear` if the year is before 1583
    pub fn new(config: LiturgicalConfig, year: i32) -> RomcalResult<Self> {
        validate_year(year, 1583)?;
        let is_liturgical_year = config.scope == crate::CalendarScope::Liturgical;
        Ok(Self {
            config,
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
    // Advent calculations
    // =================================================================================

    /// Gets all dates of Advent
    pub fn all_dates_of_advent(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = self.effective_year(year);
        let start = self.first_sunday_of_advent(Some(year));
        let end = Self::subtract_days(self.christmas(Some(year)), 1);
        Self::range_of_days(start, end)
    }

    /// Gets all Sundays of Advent
    pub fn all_sundays_of_advent(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = self.effective_year(year);
        let first_sunday = self.first_sunday_of_advent(Some(year));

        vec![
            first_sunday,
            Self::add_days(first_sunday, 7),
            Self::add_days(first_sunday, 14),
            Self::add_days(first_sunday, 21),
        ]
    }

    /// Gets the date of the first Sunday of Advent
    pub fn first_sunday_of_advent(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = self.effective_year(year);
        Self::first_sunday_of_advent_static(year)
    }

    /// Static calculation of the first Sunday of Advent
    pub fn first_sunday_of_advent_static(year: i32) -> DateTime<Utc> {
        let christmas = Self::christmas_static(year);
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

        let first_sunday = self.first_sunday_of_advent(Some(year));
        let date = Self::add_days(first_sunday, (week - 1) as i64 * 7 + (dow - 1) as i64);

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
    pub fn sunday_of_advent(&self, week: u8, year: Option<i32>) -> Option<DateTime<Utc>> {
        let year = self.effective_year(year);

        if !(1..=4).contains(&week) {
            return None;
        }

        let first_sunday = self.first_sunday_of_advent(Some(year));
        Some(Self::add_days(first_sunday, (week - 1) as i64 * 7))
    }

    // =================================================================================
    // Christmas calculations
    // =================================================================================

    /// Gets the date of Christmas
    pub fn christmas(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = self.effective_year(year);
        Self::christmas_static(year)
    }

    /// Static calculation of Christmas
    pub fn christmas_static(year: i32) -> DateTime<Utc> {
        Self::get_utc_date(year, 12, 25)
    }

    /// Gets all dates in the octave of Christmas (from Christmas to Mary Mother of God, inclusive)
    pub fn all_dates_in_octave_of_christmas(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = self.effective_year(year);
        let christmas = self.christmas(Some(year));
        let mary_mother_of_god = self.mary_mother_of_god(Some(year));

        // Octave includes Christmas + 6 days + Mary Mother of God
        let mut dates = Self::range_of_days(christmas, Self::add_days(christmas, 6));
        dates.push(mary_mother_of_god);
        dates
    }

    /// Gets the date of the nth weekday within the Octave of the Nativity of the Lord
    /// Sundays and the feast of the Holy Family are excluded
    pub fn weekday_within_octave_of_christmas(
        &self,
        day_of_octave: u8,
        year: Option<i32>,
    ) -> Option<DateTime<Utc>> {
        let year = self.effective_year(year);

        if !(1..=8).contains(&day_of_octave) {
            return None;
        }

        let christmas = self.christmas(Some(year));
        let date = Self::add_days(christmas, (day_of_octave - 1) as i64);
        let holy_family = self.holy_family(Some(year));

        // If it's the same date as Holy Family, return None
        if Self::is_same_date(date, holy_family) {
            return None;
        }

        Some(date)
    }

    /// Gets the date of the Holy Family
    pub fn holy_family(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = self.effective_year(year);
        let christmas = self.christmas(Some(year));
        if christmas.weekday() == Weekday::Sun {
            // If Christmas is on a Sunday, Holy Family is on December 30
            Self::get_utc_date(year, 12, 30)
        } else {
            // Holy Family is 1 week after Christmas when Christmas is on a weekday
            Self::start_of_week(Self::add_days(christmas, 7))
        }
    }

    /// Gets all dates occurring in the season of Christmas
    pub fn all_dates_of_christmas_time(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let start = self.christmas(Some(year));
        let end = self.baptism_of_the_lord(Some(year));
        Self::range_of_days(start, end)
    }

    /// Gets the second Sunday after the Octave of the Nativity of the Lord,
    /// which is not the Epiphany or the Baptism of the Lord
    pub fn second_sunday_after_christmas(&self, year: Option<i32>) -> Option<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);

        if self.config.epiphany_on_sunday {
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
        let start = Self::add_days(self.mary_mother_of_god(Some(year)), 1);
        let epiphany = self.epiphany(Some(year));

        // If there are no days between Mary, Mother of God and Epiphany
        if Self::is_same_date(start, epiphany) {
            return Vec::new();
        }

        let end = Self::subtract_days(epiphany, 1);
        Self::range_of_days(start, end)
    }

    /// Gets the date of a weekday before Epiphany (and from January 2)
    pub fn weekday_before_epiphany(&self, day: u8, year: Option<i32>) -> Option<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);

        if !(2..=8).contains(&day) {
            return None;
        }

        self.all_dates_before_epiphany(Some(year))
            .iter()
            .find(|d| d.day() == day as u32)
            .copied()
    }

    /// Gets the date of Epiphany
    pub fn epiphany(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        let first_day = Self::get_utc_date(year, 1, 1);
        let mut date = Self::get_utc_date(year, 1, 6);

        if self.config.epiphany_on_sunday {
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
        let start = Self::add_days(self.epiphany(Some(year)), 1);
        let baptism_of_the_lord = self.baptism_of_the_lord(Some(year));

        // If there are no days between Epiphany and Baptism of the Lord
        if Self::is_same_date(start, baptism_of_the_lord) {
            return Vec::new();
        }

        let end = Self::subtract_days(baptism_of_the_lord, 1);
        Self::range_of_days(start, end)
    }

    /// Gets the date of a weekday after Epiphany (and before the Baptism of the Lord)
    pub fn weekday_after_epiphany(&self, dow: u8, year: Option<i32>) -> Option<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);

        if !(1..=6).contains(&dow) {
            return None;
        }

        self.all_dates_after_epiphany(Some(year))
            .iter()
            .find(|d| d.weekday().num_days_from_sunday() as u8 == dow)
            .copied()
    }

    // =================================================================================
    // Lent calculations
    // =================================================================================

    /// Gets the date of Ash Wednesday
    pub fn ash_wednesday(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::subtract_days(self.easter_sunday_unwrap(Some(year)), 46)
    }

    /// Gets all dates of Lent
    pub fn all_dates_of_lent(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let start = self.ash_wednesday(Some(year));
        let end = self.holy_thursday(Some(year));
        Self::range_of_days(start, end)
    }

    /// Gets all Sundays of Lent (from Ash Wednesday to the day before Holy Thursday)
    pub fn all_sundays_of_lent(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let first_sunday = Self::add_days(self.ash_wednesday(Some(year)), 4);

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
    pub fn palm_sunday(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::subtract_days(self.easter_sunday_unwrap(Some(year)), 7)
    }

    // =================================================================================
    // Holy Week calculations
    // =================================================================================

    /// Gets the date of Holy Thursday
    pub fn holy_thursday(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::subtract_days(self.easter_sunday_unwrap(Some(year)), 3)
    }

    /// Gets the date of Good Friday
    pub fn good_friday(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::subtract_days(self.easter_sunday_unwrap(Some(year)), 2)
    }

    /// Gets the date of Holy Saturday
    pub fn holy_saturday(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::subtract_days(self.easter_sunday_unwrap(Some(year)), 1)
    }

    /// Gets all dates of Holy Week
    pub fn all_dates_of_holy_week(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let start = self.palm_sunday(Some(year));
        let end = self.holy_saturday(Some(year));
        Self::range_of_days(start, end)
    }

    // =================================================================================
    // Paschal Triduum calculations
    // =================================================================================

    /// Gets all dates of the Paschal Triduum
    pub fn all_dates_of_paschal_triduum(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let start = self.holy_thursday(Some(year));
        let end = self.easter_sunday_unwrap(Some(year));
        Self::range_of_days(start, end)
    }

    // =================================================================================
    // Easter calculations
    // =================================================================================

    /// Gets the date of Easter Sunday
    ///
    /// # Errors
    ///
    /// Returns `RomcalError::InvalidYear` if the year is before 1583
    pub fn easter_sunday(&self, year: Option<i32>) -> RomcalResult<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);

        let easter_date = match self.config.easter_calculation_type {
            EasterCalculationType::Gregorian => calculate_gregorian_easter_date(year)?,
            EasterCalculationType::Julian => calculate_julian_easter_date_to_gregorian(year)?,
        };
        easter_date.to_utc_date()
    }

    /// Gets the date of Easter Sunday (compatibility method that panics on error)
    ///
    /// # Panics
    ///
    /// Panics if the year is invalid or if there's a calculation error
    pub fn easter_sunday_unwrap(&self, year: Option<i32>) -> DateTime<Utc> {
        self.easter_sunday(year)
            .expect("Invalid year or calculation error")
    }

    /// Gets all dates occurring during the octave of Easter
    /// from Easter Sunday until the Sunday following Easter (Divine Mercy Sunday), inclusive
    pub fn all_dates_in_octave_of_easter(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let start = self.easter_sunday_unwrap(Some(year));
        let end = self.divine_mercy_sunday(Some(year));
        Self::range_of_days(start, end)
    }

    /// Gets all Sundays of Easter
    /// Easter Time is the period of fifty days from Easter Sunday to Pentecost Sunday (inclusive).
    /// All Sundays in this period are counted as Sundays of Easter.
    pub fn all_sundays_of_easter(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let first_sunday = self.easter_sunday_unwrap(Some(year));

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
    pub fn weekday_or_sunday_of_easter_time(
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
            self.easter_sunday_unwrap(Some(year)),
            ((week - 1) * 7 + dow) as i64,
        );
        let ascension = self.ascension(Some(year));

        // If it's the same date as Ascension, return None
        if Self::is_same_date(date, ascension) {
            return None;
        }

        Some(date)
    }

    /// Gets all dates occurring in Easter Time
    /// Easter Time is the period of fifty days from Easter Sunday to Pentecost Sunday
    pub fn all_dates_of_easter_time(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let start = self.easter_sunday_unwrap(Some(year));
        let end = self.pentecost_sunday(Some(year));
        Self::range_of_days(start, end)
    }

    /// Gets the date of Divine Mercy Sunday
    pub fn divine_mercy_sunday(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::add_days(self.easter_sunday_unwrap(Some(year)), 7)
    }

    /// Gets the date of Ascension
    pub fn ascension(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        if self.config.ascension_on_sunday {
            // Ascension on the 7th Sunday of Easter (42 days after Easter)
            Self::add_days(self.easter_sunday_unwrap(Some(year)), 42)
        } else {
            // Ascension on Thursday (39 days after Easter)
            Self::add_days(self.easter_sunday_unwrap(Some(year)), 39)
        }
    }

    /// Gets the date of Pentecost
    pub fn pentecost_sunday(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::add_days(self.easter_sunday_unwrap(Some(year)), 49)
    }

    // =================================================================================
    // Ordinary Time calculations
    // =================================================================================

    /// Gets all dates occurring in Ordinary Time
    pub fn all_dates_of_ordinary_time(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let mut early = self.all_dates_of_early_ordinary_time(Some(year));
        let mut late = self.all_dates_of_late_ordinary_time(Some(year));
        early.append(&mut late);
        early
    }

    /// Gets all dates of early Ordinary Time
    /// Ordinary Time in the early part of the year begins
    /// the day after the Baptism of the Lord and concludes
    /// the day before Ash Wednesday
    pub fn all_dates_of_early_ordinary_time(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let start = Self::add_days(self.baptism_of_the_lord(Some(year)), 1);
        let end = Self::subtract_days(self.ash_wednesday(Some(year)), 1);
        Self::range_of_days(start, end)
    }

    /// Gets all Sundays that fall within the period of early Ordinary Time
    pub fn sundays_of_early_ordinary_time(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        self.all_dates_of_early_ordinary_time(Some(year))
            .into_iter()
            .filter(|d| d.weekday() == Weekday::Sun)
            .collect()
    }

    /// Gets all dates of late Ordinary Time
    /// Ordinary Time after Pentecost to the day before the First Sunday of Advent
    pub fn all_dates_of_late_ordinary_time(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let start = Self::add_days(self.pentecost_sunday(Some(year)), 1);
        let end = Self::subtract_days(self.first_sunday_of_advent(Some(year)), 1);
        Self::range_of_days(start, end)
    }

    /// Gets all Sundays that fall within the period of late Ordinary Time
    pub fn all_sundays_of_late_ordinary_time(&self, year: Option<i32>) -> Vec<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        self.all_dates_of_late_ordinary_time(Some(year))
            .into_iter()
            .filter(|d| d.weekday() == Weekday::Sun)
            .collect()
    }

    /// Gets a specific date of Ordinary Time by day of week and week number
    pub fn date_of_ordinary_time(
        &self,
        dow: u8,
        week: u8,
        year: Option<i32>,
    ) -> Option<DateTime<Utc>> {
        let year = year.unwrap_or(self.year);

        if dow > 6 || !(1..=35).contains(&week) {
            return None;
        }

        let early_dates = self.all_dates_of_early_ordinary_time(Some(year));
        let late_dates = self.all_dates_of_late_ordinary_time(Some(year));

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

    // =================================================================================
    // Fixed and movable Feasts and Solemnities
    // =================================================================================

    /// Gets the date of Mary, Mother of God (January 1)
    pub fn mary_mother_of_god(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::get_utc_date(year, 1, 1)
    }

    /// Gets the date of the Baptism of the Lord
    pub fn baptism_of_the_lord(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        let epiphany = self.epiphany(Some(year));

        if epiphany.day() == 6 {
            // If Epiphany is celebrated on January 6,
            // the Baptism of the Lord occurs on the Sunday following January 6
            Self::start_of_week(Self::add_days(epiphany, 7))
        } else if (epiphany.weekday() == Weekday::Sun && epiphany.day() == 7) || epiphany.day() == 8
        {
            // If Epiphany occurs on Sunday January 7 or January 8,
            // then the Baptism of the Lord is the next day (Monday)
            Self::add_days(epiphany, 1)
        } else {
            // If Epiphany occurs before January 6, the Sunday
            // following Epiphany is the Baptism of the Lord
            Self::start_of_week(Self::add_days(epiphany, 7))
        }
    }

    /// Gets the date of the Presentation of the Lord (February 2)
    pub fn presentation_of_the_lord(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::get_utc_date(year, 2, 2)
    }

    /// Gets the date of the Annunciation (March 25)
    pub fn annunciation(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        let mut date = Self::get_utc_date(year, 3, 25);

        // If it falls during Holy Week or the Octave of Easter,
        // it is transferred to the Monday of the 2nd week of Easter
        let palm_sunday = self.palm_sunday(Some(year));
        let divine_mercy_sunday = self.divine_mercy_sunday(Some(year));

        if date >= palm_sunday && date <= divine_mercy_sunday {
            date = Self::add_days(divine_mercy_sunday, 1);
        }

        date
    }

    /// Gets the date of Mary, Mother of the Church
    /// (occurs the day after Pentecost Sunday)
    pub fn mary_mother_of_the_church(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::add_days(self.easter_sunday_unwrap(Some(year)), 50)
    }

    /// Gets the date of Trinity Sunday
    pub fn trinity_sunday(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::add_days(self.easter_sunday_unwrap(Some(year)), 56)
    }

    /// Gets the date of Corpus Christi
    pub fn corpus_christi(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        if self.config.corpus_christi_on_sunday {
            // Corpus Christi on Sunday (63 days after Easter)
            Self::add_days(self.easter_sunday_unwrap(Some(year)), 63)
        } else {
            // Corpus Christi on Thursday (60 days after Easter)
            Self::add_days(self.easter_sunday_unwrap(Some(year)), 60)
        }
    }

    /// Gets the date of the Most Sacred Heart of Jesus
    pub fn most_sacred_heart_of_jesus(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::add_days(self.easter_sunday_unwrap(Some(year)), 68)
    }

    /// Gets the date of the Immaculate Heart of Mary
    pub fn immaculate_heart_of_mary(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::add_days(self.easter_sunday_unwrap(Some(year)), 69)
    }

    /// Gets the date of the Nativity of John the Baptist (June 24)
    pub fn nativity_of_john_the_baptist(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::get_utc_date(year, 6, 24)
    }

    /// Gets the date of Peter and Paul (June 29)
    pub fn peter_and_paul_apostles(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::get_utc_date(year, 6, 29)
    }

    /// Gets the date of the Transfiguration (August 6)
    pub fn transfiguration(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::get_utc_date(year, 8, 6)
    }

    /// Gets the date of the Assumption (August 15)
    pub fn assumption(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::get_utc_date(year, 8, 15)
    }

    /// Gets the date of the Exaltation of the Holy Cross (September 14)
    pub fn exaltation_of_the_holy_cross(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::get_utc_date(year, 9, 14)
    }

    /// Gets the date of All Saints (November 1)
    pub fn all_saints(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::get_utc_date(year, 11, 1)
    }

    /// Gets the date of Christ the King
    pub fn christ_the_king_sunday(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::subtract_days(self.first_sunday_of_advent(Some(year)), 7)
    }

    /// Gets the date of the Immaculate Conception (December 8)
    pub fn immaculate_conception_of_mary(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = self.effective_year(year);
        let mut date = Self::get_utc_date(year, 12, 8);

        // If this solemnity falls on a Sunday, it is transferred to the following Monday
        if date.weekday() == Weekday::Sun {
            date = Self::add_days(date, 1);
        }

        date
    }

    // =================================================================================
    // Season calculations
    // =================================================================================

    /// Gets the start of seasons for a given year
    pub fn start_of_seasons(&self, year: Option<i32>) -> HashMap<Season, DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let mut seasons = HashMap::new();

        seasons.insert(Season::Advent, self.first_sunday_of_advent(Some(year - 1)));
        seasons.insert(Season::ChristmasTime, self.christmas(Some(year - 1)));
        seasons.insert(Season::Lent, self.ash_wednesday(Some(year)));
        seasons.insert(Season::PaschalTriduum, self.holy_thursday(Some(year)));
        seasons.insert(Season::EasterTime, self.easter_sunday_unwrap(Some(year)));
        seasons.insert(
            Season::OrdinaryTime,
            Self::add_days(self.baptism_of_the_lord(Some(year)), 1),
        );

        seasons
    }

    /// Gets the end of seasons for a given year
    pub fn end_of_seasons(&self, year: Option<i32>) -> HashMap<Season, DateTime<Utc>> {
        let year = year.unwrap_or(self.year);
        let mut seasons = HashMap::new();

        seasons.insert(Season::Advent, Self::get_utc_date(year - 1, 12, 24));
        seasons.insert(Season::ChristmasTime, self.baptism_of_the_lord(Some(year)));
        seasons.insert(Season::Lent, self.holy_thursday(Some(year)));
        seasons.insert(
            Season::PaschalTriduum,
            self.easter_sunday_unwrap(Some(year)),
        );
        seasons.insert(Season::EasterTime, self.pentecost_sunday(Some(year)));
        seasons.insert(
            Season::OrdinaryTime,
            Self::add_days(self.christ_the_king_sunday(Some(year)), 6),
        );

        seasons
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_liturgical_dates_creation() {
        let config = LiturgicalConfig::default();
        let dates = LiturgicalDates::new(config, 2024).unwrap();

        assert_eq!(dates.year, 2024);
        assert!(!dates.is_liturgical_year);
    }

    #[test]
    fn test_liturgical_year_creation() {
        let config = LiturgicalConfig {
            scope: crate::CalendarScope::Liturgical,
            ..LiturgicalConfig::default()
        };
        let dates = LiturgicalDates::new(config, 2024).unwrap();

        assert_eq!(dates.year, 2024);
        assert!(dates.is_liturgical_year);
    }

    #[test]
    fn test_christmas_calculation() {
        let config = LiturgicalConfig::default();
        let dates = LiturgicalDates::new(config, 2024).unwrap();
        let christmas = dates.christmas(None);

        assert_eq!(christmas.day(), 25);
        assert_eq!(christmas.month(), 12);
        assert_eq!(christmas.year(), 2024);
    }

    #[test]
    fn test_easter_calculation() {
        let config = LiturgicalConfig::default();
        let dates = LiturgicalDates::new(config, 2024).unwrap();
        let easter = dates.easter_sunday_unwrap(None);

        // Easter 2024 is March 31
        assert_eq!(easter.day(), 31);
        assert_eq!(easter.month(), 3);
        assert_eq!(easter.year(), 2024);
    }

    #[test]
    fn test_ash_wednesday_calculation() {
        let config = LiturgicalConfig::default();
        let dates = LiturgicalDates::new(config, 2024).unwrap();
        let ash_wednesday = dates.ash_wednesday(None);

        // Ash Wednesday 2024 is February 14 (46 days before Easter)
        assert_eq!(ash_wednesday.day(), 14);
        assert_eq!(ash_wednesday.month(), 2);
        assert_eq!(ash_wednesday.year(), 2024);
    }

    #[test]
    fn test_utility_functions() {
        let date1 = LiturgicalDates::get_utc_date(2024, 3, 31);
        let date2 = LiturgicalDates::get_utc_date(2024, 3, 31);
        let date3 = LiturgicalDates::get_utc_date(2024, 4, 1);

        assert!(LiturgicalDates::is_same_date(date1, date2));
        assert!(!LiturgicalDates::is_same_date(date1, date3));

        let added_date = LiturgicalDates::add_days(date1, 1);
        assert!(LiturgicalDates::is_same_date(added_date, date3));
    }

    #[test]
    fn test_unprivileged_weekday_of_advent() {
        let config = LiturgicalConfig::default();
        let dates = LiturgicalDates::new(config, 2024).unwrap();

        // Test valid weekday
        let weekday = dates.unprivileged_weekday_of_advent(1, 1, None); // Monday, week 1
        assert!(weekday.is_some());

        // Test invalid parameters
        assert!(dates.unprivileged_weekday_of_advent(0, 1, None).is_none()); // Invalid dow
        assert!(dates.unprivileged_weekday_of_advent(1, 0, None).is_none()); // Invalid week
        assert!(dates.unprivileged_weekday_of_advent(7, 1, None).is_none()); // Invalid dow
        assert!(dates.unprivileged_weekday_of_advent(1, 5, None).is_none()); // Invalid week
    }

    #[test]
    fn test_privileged_weekday_of_advent() {
        let config = LiturgicalConfig::default();
        let dates = LiturgicalDates::new(config, 2024).unwrap();

        // Test valid day
        let weekday = dates.privileged_weekday_of_advent(17, None);
        assert!(weekday.is_some());

        // Test invalid parameters
        assert!(dates.privileged_weekday_of_advent(16, None).is_none()); // Too early
        assert!(dates.privileged_weekday_of_advent(25, None).is_none()); // Too late
    }

    #[test]
    fn test_sunday_of_advent() {
        let config = LiturgicalConfig::default();
        let dates = LiturgicalDates::new(config, 2024).unwrap();

        // Test valid week
        let sunday = dates.sunday_of_advent(1, None);
        assert!(sunday.is_some());
        assert_eq!(sunday.unwrap().weekday(), Weekday::Sun);

        // Test invalid parameters
        assert!(dates.sunday_of_advent(0, None).is_none()); // Invalid week
        assert!(dates.sunday_of_advent(5, None).is_none()); // Invalid week
    }

    #[test]
    fn test_all_dates_in_octave_of_christmas() {
        let config = LiturgicalConfig::default();
        let dates = LiturgicalDates::new(config, 2024).unwrap();
        let octave_dates = dates.all_dates_in_octave_of_christmas(None);

        // Should have 8 dates: Christmas + 6 days + Mary Mother of God
        assert_eq!(octave_dates.len(), 8);

        // First date should be Christmas
        let christmas = dates.christmas(None);
        assert_eq!(octave_dates[0], christmas);

        // Last date should be Mary Mother of God
        let mary_mother_of_god = dates.mary_mother_of_god(None);
        assert_eq!(octave_dates[7], mary_mother_of_god);
    }

    #[test]
    fn test_weekday_within_octave_of_christmas() {
        let config = LiturgicalConfig::default();
        let dates = LiturgicalDates::new(config, 2024).unwrap();

        // Test valid day of octave
        let weekday = dates.weekday_within_octave_of_christmas(1, None);
        assert!(weekday.is_some());

        // Test invalid parameters
        assert!(dates.weekday_within_octave_of_christmas(0, None).is_none()); // Invalid day
        assert!(dates.weekday_within_octave_of_christmas(9, None).is_none()); // Invalid day
    }

    #[test]
    fn test_all_dates_of_christmas_time() {
        let config = LiturgicalConfig::default();
        let dates = LiturgicalDates::new(config, 2024).unwrap();
        let christmas_time_dates = dates.all_dates_of_christmas_time(None);

        // Should have dates from Christmas to Baptism of the Lord
        assert!(!christmas_time_dates.is_empty());

        // First date should be Christmas
        let christmas = dates.christmas(None);
        assert_eq!(christmas_time_dates[0], christmas);
    }

    #[test]
    fn test_epiphany() {
        let config = LiturgicalConfig::default();
        let dates = LiturgicalDates::new(config, 2024).unwrap();
        let epiphany = dates.epiphany(None);

        // Epiphany should be in January
        assert_eq!(epiphany.month(), 1);
        assert!(epiphany.day() >= 2 && epiphany.day() <= 8);
    }

    #[test]
    fn test_all_dates_before_epiphany() {
        let config = LiturgicalConfig::default();
        let dates = LiturgicalDates::new(config, 2024).unwrap();
        let dates_before = dates.all_dates_before_epiphany(None);

        // Should start from January 2
        if !dates_before.is_empty() {
            assert!(dates_before[0].day() >= 2);
        }
    }

    #[test]
    fn test_weekday_before_epiphany() {
        let config = LiturgicalConfig::default();
        let dates = LiturgicalDates::new(config, 2024).unwrap();

        // Test valid day
        let weekday = dates.weekday_before_epiphany(2, None);
        // May or may not exist depending on the year
        if weekday.is_some() {
            assert_eq!(weekday.unwrap().day(), 2);
        }

        // Test invalid parameters
        assert!(dates.weekday_before_epiphany(1, None).is_none()); // Too early
        assert!(dates.weekday_before_epiphany(9, None).is_none()); // Too late
    }

    #[test]
    fn test_weekday_after_epiphany() {
        let config = LiturgicalConfig::default();
        let dates = LiturgicalDates::new(config, 2024).unwrap();

        // Test valid day of week
        let weekday = dates.weekday_after_epiphany(1, None); // Monday
                                                             // May or may not exist depending on the year
        if weekday.is_some() {
            assert_eq!(weekday.unwrap().weekday().num_days_from_sunday() as u8, 1);
        }

        // Test invalid parameters
        assert!(dates.weekday_after_epiphany(0, None).is_none()); // Invalid dow
        assert!(dates.weekday_after_epiphany(7, None).is_none()); // Invalid dow
    }

    #[test]
    fn test_invalid_year_creation() {
        let config = LiturgicalConfig::default();

        // Test invalid year
        assert!(LiturgicalDates::new(config.clone(), 1500).is_err());
        assert!(LiturgicalDates::new(config.clone(), 1582).is_err());

        // Test valid year
        assert!(LiturgicalDates::new(config.clone(), 1583).is_ok());
        assert!(LiturgicalDates::new(config, 2024).is_ok());
    }

    #[test]
    fn test_easter_error_handling() {
        let config = LiturgicalConfig::default();
        let dates = LiturgicalDates::new(config, 2024).unwrap();

        // Test valid year
        assert!(dates.easter_sunday(Some(2024)).is_ok());

        // Test invalid year
        assert!(dates.easter_sunday(Some(1500)).is_err());
    }
}
