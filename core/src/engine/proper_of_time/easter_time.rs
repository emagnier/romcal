//! Easter Time generation.
//!
//! This module generates liturgical days for the Easter season,
//! from Easter Monday to Pentecost Sunday.

use chrono::{DateTime, Utc};

use super::utils::{WEEKDAY_NAMES, sort_liturgical_days_by_date};
use crate::engine::liturgical_day::LiturgicalDay;
use crate::engine::template_resolver::ProperOfTimeDayType;
use crate::error::RomcalResult;
use crate::types::liturgical::{Color, Precedence, Season};
use crate::types::mass::{MassInfo, MassTime};
use crate::types::{DateDef, DateFn, Period};

use super::ProperOfTime;

/// Structure for generating Easter Time liturgical days
/// This encapsulates all Easter Time-specific logic that was previously in ProperOfTime
pub struct EasterTime<'a> {
    proper_of_time: &'a ProperOfTime,
}

impl<'a> EasterTime<'a> {
    /// Creates a new EasterTime instance
    pub fn new(proper_of_time: &'a ProperOfTime) -> Self {
        Self { proper_of_time }
    }

    /// Generates liturgical days of Easter Time
    ///
    /// Easter Time includes:
    /// - Octave of Easter (Monday-Saturday after Easter Sunday)
    /// - Divine Mercy Sunday (Second Sunday of Easter)
    /// - Weekdays and Sundays of Easter Time (2nd Monday to 7th Saturday)
    /// - Ascension of the Lord (6th week, Thursday)
    /// - Pentecost Sunday
    pub fn generate(&self) -> RomcalResult<Vec<LiturgicalDay>> {
        let mut days = Vec::new();

        // Use cached values
        let easter_year = self.proper_of_time.cache.easter_year();

        // EASTER TIME DAY TYPES:
        // 1. Easter Sunday of the Resurrection of the Lord
        let easter_sunday_date = self
            .proper_of_time
            .dates
            .get_easter_sunday_date(Some(easter_year))?;
        let day = self.create_easter_sunday(easter_sunday_date)?;
        days.push(day);

        // 2. Octave of Easter (Monday-Saturday after Easter Sunday)
        let easter_sunday_date = self.proper_of_time.cache.easter_start();
        for dow in 1..=6 {
            let octave_date = easter_sunday_date + chrono::Duration::days(dow as i64);
            let liturgical_day = self.create_easter_octave_day(dow, octave_date)?;
            days.push(liturgical_day);
        }

        // 3. Divine Mercy Sunday (Second Sunday of Easter)
        let divine_mercy_date = self
            .proper_of_time
            .dates
            .get_divine_mercy_sunday_date(Some(easter_year));
        let day = self.create_divine_mercy_sunday(divine_mercy_date)?;
        days.push(day);

        // 4. All days from 2nd Monday to 7th Saturday of Easter Time
        for i in 8..49 {
            let week = (i / 7) + 1;
            let dow = i - (week - 1) * 7;

            let weekday_date = easter_sunday_date + chrono::Duration::days(i as i64);

            // Special case: Ascension of the Lord
            // If ascension_on_sunday is false: 6th week, Thursday (39 days after Easter)
            // If ascension_on_sunday is true: 7th week, Sunday (42 days after Easter)
            let ascension_date = self
                .proper_of_time
                .dates
                .get_ascension_date(Some(easter_year));
            let is_ascension_day = if self.proper_of_time.romcal.ascension_on_sunday {
                week == 7 && dow == 0 // 7th week, Sunday
            } else {
                week == 6 && dow == 4 // 6th week, Thursday
            };

            if is_ascension_day {
                let liturgical_day = self.create_ascension_of_the_lord(ascension_date)?;
                days.push(liturgical_day);
            } else {
                let liturgical_day = self.create_easter_time_weekday(week, dow, weekday_date)?;
                days.push(liturgical_day);
            }
        }

        // 5. Pentecost Sunday
        let pentecost_date = self
            .proper_of_time
            .dates
            .get_pentecost_sunday_date(Some(easter_year));
        let day = self.create_pentecost_sunday(pentecost_date)?;
        days.push(day);

        // TODO: Temporary fix to sort days by date
        sort_liturgical_days_by_date(&mut days);

        Ok(days)
    }

    // ---------------------------------------------------------------------------------
    // EASTER TIME DAY CREATION FUNCTIONS
    // ---------------------------------------------------------------------------------

    /// Creates Easter Sunday of the Resurrection of the Lord
    fn create_easter_sunday(&self, date: DateTime<Utc>) -> RomcalResult<LiturgicalDay> {
        let day_type = ProperOfTimeDayType::EasterSunday;
        let periods = self
            .proper_of_time
            .resolve_periods(vec![Period::PaschalTriduum, Period::EasterOctave]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                "easter_sunday",
                date,
                Precedence::Triduum_1,
                Some(Season::EasterTime),
                Color::White,
                Some(&day_type),
            )
            .with_periods(periods)
            .with_is_holy_day_of_obligation(true)
            .with_masses(vec![
                MassInfo::new(MassTime::EasterVigil),
                MassInfo::new(MassTime::DayMass),
            ]);

        Ok(liturgical_day)
    }

    /// Creates a day within the Octave of Easter
    fn create_easter_octave_day(
        &self,
        dow: u8,
        date: DateTime<Utc>,
    ) -> RomcalResult<LiturgicalDay> {
        let day_type = ProperOfTimeDayType::EasterOctave { dow };
        let periods = self
            .proper_of_time
            .resolve_periods(vec![Period::EasterOctave]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                &format!("easter_{}", WEEKDAY_NAMES[dow as usize]),
                date,
                Precedence::WeekdayOfEasterOctave_2,
                Some(Season::EasterTime),
                Color::White,
                Some(&day_type),
            )
            .with_periods(periods);

        Ok(liturgical_day)
    }

    /// Creates Divine Mercy Sunday (Second Sunday of Easter)
    fn create_divine_mercy_sunday(&self, date: DateTime<Utc>) -> RomcalResult<LiturgicalDay> {
        // Entity-based day, fullname comes from entity resolution
        let periods = self
            .proper_of_time
            .resolve_periods(vec![Period::EasterOctave]);
        let mut liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                "divine_mercy_sunday",
                date,
                Precedence::PrivilegedSunday_2,
                Some(Season::EasterTime),
                Color::White,
                None,
            )
            .with_periods(periods)
            .with_is_holy_day_of_obligation(true);

        // Override date definition with specific function
        liturgical_day.date_def = DateDef::DateFunction {
            date_fn: DateFn::DivineMercySunday,
            day_offset: None,
        };

        Ok(liturgical_day)
    }

    /// Creates a weekday or Sunday of Easter Time
    fn create_easter_time_weekday(
        &self,
        week: u8,
        dow: u8,
        date: DateTime<Utc>,
    ) -> RomcalResult<LiturgicalDay> {
        let day_type = if dow == 0 {
            ProperOfTimeDayType::EasterTimeSunday { week }
        } else {
            ProperOfTimeDayType::EasterTimeWeekday { week, dow }
        };
        let liturgical_day = self.proper_of_time.create_liturgical_day_base(
            &format!("easter_time_{}_{}", week, WEEKDAY_NAMES[dow as usize]),
            date,
            if dow == 0 {
                Precedence::PrivilegedSunday_2
            } else {
                Precedence::Weekday_13
            },
            Some(Season::EasterTime),
            Color::White,
            Some(&day_type),
        );

        Ok(liturgical_day)
    }

    /// Creates the Ascension of the Lord
    fn create_ascension_of_the_lord(&self, date: DateTime<Utc>) -> RomcalResult<LiturgicalDay> {
        // Entity-based day, fullname comes from entity resolution
        let liturgical_day = self.proper_of_time.create_liturgical_day_base(
            "ascension_of_the_lord",
            date,
            Precedence::ProperOfTimeSolemnity_2,
            Some(Season::EasterTime),
            Color::White,
            None,
        );

        Ok(liturgical_day)
    }

    /// Creates Pentecost Sunday
    fn create_pentecost_sunday(&self, date: DateTime<Utc>) -> RomcalResult<LiturgicalDay> {
        // Entity-based day, fullname comes from entity resolution
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                "pentecost_sunday",
                date,
                Precedence::ProperOfTimeSolemnity_2,
                Some(Season::EasterTime),
                Color::Red,
                None,
            )
            .with_masses(vec![
                MassInfo::new(MassTime::PreviousEveningMass),
                MassInfo::new(MassTime::DayMass),
            ]);

        Ok(liturgical_day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::romcal::{Preset, Romcal};

    #[test]
    fn test_easter_time_generation() {
        let romcal = Romcal::default();
        let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();
        let easter_time = EasterTime::new(&proper_of_time);

        let days = easter_time.generate().unwrap();

        // Should have: 1 easter sunday + 6 octave days + 1 divine mercy + 40 weekdays/sundays + 1 ascension + 1 pentecost = 50 days
        assert_eq!(days.len(), 50);

        // Check for Easter Sunday
        let easter_sunday = days.iter().find(|d| d.id == "easter_sunday");
        assert!(easter_sunday.is_some());

        // Check for Easter octave days (Monday-Saturday)
        for dow in 1..=6 {
            let octave_day = days
                .iter()
                .find(|d| d.id == format!("easter_{}", WEEKDAY_NAMES[dow as usize]));
            assert!(
                octave_day.is_some(),
                "Missing Easter octave day for dow {}",
                dow
            );
        }

        // Check for Divine Mercy Sunday
        let divine_mercy = days.iter().find(|d| d.id == "divine_mercy_sunday");
        assert!(divine_mercy.is_some());

        // Check for Ascension of the Lord
        let ascension = days.iter().find(|d| d.id == "ascension_of_the_lord");
        assert!(ascension.is_some());

        // Check for Pentecost Sunday
        let pentecost = days.iter().find(|d| d.id == "pentecost_sunday");
        assert!(pentecost.is_some());

        // Check for some Easter Time weekdays
        let easter_time_weekday = days.iter().find(|d| d.id.starts_with("easter_time_"));
        assert!(easter_time_weekday.is_some());
    }

    #[test]
    fn test_liturgical_year_easter_time() {
        let romcal = Romcal::new(Preset {
            context: Some(crate::CalendarContext::Liturgical),
            ..Preset::default()
        });
        let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();
        let easter_time = EasterTime::new(&proper_of_time);

        let days = easter_time.generate().unwrap();

        // Should have exactly 50 days
        assert_eq!(days.len(), 50);

        // Check for Easter Sunday
        let easter_sunday = days.iter().find(|d| d.id == "easter_sunday");
        assert!(easter_sunday.is_some());

        // Check for Divine Mercy Sunday
        let divine_mercy = days.iter().find(|d| d.id == "divine_mercy_sunday");
        assert!(divine_mercy.is_some());

        // Check for Pentecost Sunday
        let pentecost = days.iter().find(|d| d.id == "pentecost_sunday");
        assert!(pentecost.is_some());
    }
}
