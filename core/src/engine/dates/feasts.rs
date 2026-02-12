//! Fixed and movable feasts and solemnities date calculations.

use chrono::{DateTime, Datelike, Utc, Weekday};

use super::LiturgicalDates;

impl LiturgicalDates {
    // =================================================================================
    // Fixed and movable Feasts and Solemnities
    // =================================================================================

    /// Gets the date of Mary, Mother of God (January 1)
    pub fn get_mary_mother_of_god_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::get_utc_date(year, 1, 1)
    }

    /// Gets the date of the Baptism of the Lord
    pub fn get_baptism_of_the_lord_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        let epiphany = self.get_epiphany_date(Some(year));

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
    pub fn get_presentation_of_the_lord_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::get_utc_date(year, 2, 2)
    }

    /// Gets the date of the Annunciation (March 25)
    pub fn get_annunciation_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        let mut date = Self::get_utc_date(year, 3, 25);

        // If it falls during Holy Week or the Octave of Easter,
        // it is transferred to the Monday of the 2nd week of Easter
        let palm_sunday = self.get_palm_sunday_date(Some(year));
        let divine_mercy_sunday = self.get_divine_mercy_sunday_date(Some(year));

        if date >= palm_sunday && date <= divine_mercy_sunday {
            date = Self::add_days(divine_mercy_sunday, 1);
        }

        date
    }

    /// Gets the date of Mary, Mother of the Church
    /// (occurs the day after Pentecost Sunday)
    pub fn get_mary_mother_of_the_church_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::add_days(self.get_easter_sunday_date_unwrap(Some(year)), 50)
    }

    /// Gets the date of Trinity Sunday
    pub fn get_trinity_sunday_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::add_days(self.get_easter_sunday_date_unwrap(Some(year)), 56)
    }

    /// Gets the date of Corpus Christi
    pub fn get_corpus_christi_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        if self.romcal.corpus_christi_on_sunday {
            // Corpus Christi on Sunday (63 days after Easter)
            Self::add_days(self.get_easter_sunday_date_unwrap(Some(year)), 63)
        } else {
            // Corpus Christi on Thursday (60 days after Easter)
            Self::add_days(self.get_easter_sunday_date_unwrap(Some(year)), 60)
        }
    }

    /// Gets the date of the Most Sacred Heart of Jesus
    pub fn get_most_sacred_heart_of_jesus_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::add_days(self.get_easter_sunday_date_unwrap(Some(year)), 68)
    }

    /// Gets the date of the Immaculate Heart of Mary
    pub fn get_immaculate_heart_of_mary_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::add_days(self.get_easter_sunday_date_unwrap(Some(year)), 69)
    }

    /// Gets the date of the Nativity of John the Baptist (June 24)
    pub fn get_nativity_of_john_the_baptist_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::get_utc_date(year, 6, 24)
    }

    /// Gets the date of Peter and Paul (June 29)
    pub fn get_peter_and_paul_apostles_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::get_utc_date(year, 6, 29)
    }

    /// Gets the date of the Transfiguration (August 6)
    pub fn get_transfiguration_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::get_utc_date(year, 8, 6)
    }

    /// Gets the date of the Assumption (August 15)
    pub fn get_assumption_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::get_utc_date(year, 8, 15)
    }

    /// Gets the date of the Exaltation of the Holy Cross (September 14)
    pub fn get_exaltation_of_the_holy_cross_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::get_utc_date(year, 9, 14)
    }

    /// Gets the date of All Saints (November 1)
    pub fn get_all_saints_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::get_utc_date(year, 11, 1)
    }

    /// Gets the date of Christ the King
    pub fn get_christ_the_king_sunday_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = year.unwrap_or(self.year);
        Self::subtract_days(self.get_first_sunday_of_advent_date(Some(year)), 7)
    }

    /// Gets the date of the Immaculate Conception (December 8)
    pub fn get_immaculate_conception_of_mary_date(&self, year: Option<i32>) -> DateTime<Utc> {
        let year = self.effective_year(year);
        let mut date = Self::get_utc_date(year, 12, 8);

        // If this solemnity falls on a Sunday, it is transferred to the following Monday
        if date.weekday() == Weekday::Sun {
            date = Self::add_days(date, 1);
        }

        date
    }
}
