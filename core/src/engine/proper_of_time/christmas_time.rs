//! Christmas Time generation.
//!
//! This module generates liturgical days for Christmas Time,
//! from December 25 to the Baptism of the Lord.

use chrono::{DateTime, Utc};

use super::utils::{WEEKDAY_NAMES, sort_liturgical_days_by_date};
use crate::engine::liturgical_day::LiturgicalDay;
use crate::engine::template_resolver::ProperOfTimeDayType;
use crate::error::RomcalResult;
use crate::types::liturgical::{Color, Period, Precedence, Season};
use crate::types::mass::{MassInfo, MassTime};

use super::ProperOfTime;

/// Structure for generating Christmas Time liturgical days
/// This encapsulates all Christmas Time-specific logic that was previously in ProperOfTime
pub struct ChristmasTime<'a> {
    proper_of_time: &'a ProperOfTime,
}

impl<'a> ChristmasTime<'a> {
    /// Creates a new ChristmasTime instance
    pub fn new(proper_of_time: &'a ProperOfTime) -> Self {
        Self { proper_of_time }
    }

    /// Generates liturgical days of early Christmas Time
    ///
    /// Early Christmas Time includes:
    /// - The Nativity of the Lord (December 25)
    /// - Octave of Christmas (December 26-31, excluding December 25 and January 1)
    /// - The Holy Family (Sunday within the Octave)
    pub fn generate_early(&self) -> RomcalResult<Vec<LiturgicalDay>> {
        let mut days = Vec::new();

        // Use cached values
        let christmas_year = self.proper_of_time.cache.christmas_year();
        let christmas_date = self.proper_of_time.cache.christmas_start();

        // EARLY CHRISTMAS TIME DAY TYPES:
        // 1. The Nativity of the Lord (December 25)
        let day = self.create_nativity_of_the_lord(christmas_date)?;
        days.push(day);

        // 2. Octave of Christmas (December 26-31, excluding December 25 and January 1)
        for count in 2..=7 {
            if let Some(octave_date) = self
                .proper_of_time
                .dates
                .get_weekday_within_octave_of_christmas_date(count, Some(christmas_year))
            {
                let day = self.create_christmas_octave_day(count, octave_date)?;
                days.push(day);
            }
        }

        // 3. The Holy Family (Sunday within the Octave)
        let holy_family_date = self
            .proper_of_time
            .dates
            .get_holy_family_date(Some(christmas_year));

        let day = self.create_holy_family(holy_family_date)?;
        days.push(day);

        // TODO: Temporary fix to sort days by date
        sort_liturgical_days_by_date(&mut days);

        Ok(days)
    }

    /// Generates liturgical days of late Christmas Time
    ///
    /// Late Christmas Time includes:
    /// - Mary, Mother of God (January 1)
    /// - Second Sunday after Christmas (if it exists)
    /// - Weekdays before Epiphany (January 2-8)
    /// - The Epiphany of the Lord
    /// - Weekdays after Epiphany
    /// - The Baptism of the Lord
    pub fn generate_late(&self) -> RomcalResult<Vec<LiturgicalDay>> {
        let mut days = Vec::new();

        // Use cached values
        let christmas_year =
            if self.proper_of_time.romcal.context == crate::CalendarContext::Liturgical {
                self.proper_of_time.cache.christmas_year() + 1
            } else {
                self.proper_of_time.cache.christmas_year()
            };

        // LATE CHRISTMAS TIME DAY TYPES:
        // 1. Mary, Mother of God (January 1)
        let mary_mother_of_god_date = self
            .proper_of_time
            .dates
            .get_mary_mother_of_god_date(Some(christmas_year));

        let day = self.create_mary_mother_of_god(mary_mother_of_god_date)?;
        days.push(day);

        // 2. Second Sunday after Christmas (if it exists)
        let epiphany_date = self
            .proper_of_time
            .dates
            .get_epiphany_date(Some(christmas_year));
        let second_sunday_date = self
            .proper_of_time
            .dates
            .second_sunday_after_christmas(Some(christmas_year));
        if let Some(second_sunday_date) = second_sunday_date {
            let day =
                self.create_second_sunday_after_christmas(second_sunday_date, epiphany_date)?;
            days.push(day);
        }

        // 3. Weekdays before Epiphany (January 2-8)
        for day_num in 2..=8 {
            if let Some(weekday_date) = self
                .proper_of_time
                .dates
                .get_weekday_before_epiphany_date(day_num, Some(christmas_year))
            {
                let liturgical_day = self.create_weekday_before_epiphany(day_num, weekday_date)?;
                days.push(liturgical_day);
            }
        }

        // 4. The Epiphany of the Lord
        let epiphany_date = self
            .proper_of_time
            .dates
            .get_epiphany_date(Some(christmas_year));

        let day = self.create_epiphany_of_the_lord(epiphany_date)?;
        days.push(day);

        // 5. Weekdays after Epiphany
        for dow in 1..=6 {
            if let Some(weekday_date) = self
                .proper_of_time
                .dates
                .get_weekday_after_epiphany_date(dow, Some(christmas_year))
            {
                let liturgical_day = self.create_weekday_after_epiphany(dow, weekday_date)?;
                days.push(liturgical_day);
            }
        }

        // 6. The Baptism of the Lord
        let baptism_date = self
            .proper_of_time
            .dates
            .get_baptism_of_the_lord_date(Some(christmas_year));

        let day = self.create_baptism_of_the_lord(baptism_date)?;
        days.push(day);

        // TODO: Temporary fix to sort days by date
        sort_liturgical_days_by_date(&mut days);

        Ok(days)
    }

    // ---------------------------------------------------------------------------------
    // CHRISTMAS TIME DAY CREATION FUNCTIONS
    // ---------------------------------------------------------------------------------

    /// Creates the Nativity of the Lord (December 25)
    fn create_nativity_of_the_lord(&self, date: DateTime<Utc>) -> RomcalResult<LiturgicalDay> {
        // Entity-based day, fullname comes from entity resolution
        let periods = self.proper_of_time.resolve_periods(vec![
            Period::ChristmasOctave,
            Period::ChristmasToPresentationOfTheLord,
        ]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                "nativity_of_the_lord",
                date,
                Precedence::ProperOfTimeSolemnity_2,
                Some(Season::ChristmasTime),
                Color::White,
                None,
            )
            .with_periods(periods)
            .with_masses(vec![
                MassInfo::new(MassTime::PreviousEveningMass),
                MassInfo::new(MassTime::NightMass),
                MassInfo::new(MassTime::MassAtDawn),
                MassInfo::new(MassTime::DayMass),
            ]);

        Ok(liturgical_day)
    }

    /// Creates a day within the Octave of Christmas
    fn create_christmas_octave_day(
        &self,
        count: u8,
        date: DateTime<Utc>,
    ) -> RomcalResult<LiturgicalDay> {
        let day_type = ProperOfTimeDayType::ChristmasOctave { count };
        let periods = self.proper_of_time.resolve_periods(vec![
            Period::ChristmasOctave,
            Period::ChristmasToPresentationOfTheLord,
        ]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                &format!("christmas_octave_day_{}", count),
                date,
                Precedence::PrivilegedWeekday_9,
                Some(Season::ChristmasTime),
                Color::White,
                Some(&day_type),
            )
            .with_periods(periods);

        Ok(liturgical_day)
    }

    /// Creates the Holy Family
    fn create_holy_family(&self, date: DateTime<Utc>) -> RomcalResult<LiturgicalDay> {
        // Entity-based day, fullname comes from entity resolution
        let periods = self.proper_of_time.resolve_periods(vec![
            Period::ChristmasOctave,
            Period::ChristmasToPresentationOfTheLord,
        ]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                "holy_family_of_jesus_mary_and_joseph",
                date,
                Precedence::GeneralLordFeast_5,
                Some(Season::ChristmasTime),
                Color::White,
                None,
            )
            .with_periods(periods);

        Ok(liturgical_day)
    }

    // ---------------------------------------------------------------------------------
    // LATE CHRISTMAS TIME DAY CREATION FUNCTIONS
    // ---------------------------------------------------------------------------------

    /// Creates Mary, Mother of God (January 1)
    fn create_mary_mother_of_god(&self, date: DateTime<Utc>) -> RomcalResult<LiturgicalDay> {
        // Entity-based day, fullname comes from entity resolution
        let periods = self.proper_of_time.resolve_periods(vec![
            Period::ChristmasOctave,
            Period::ChristmasToPresentationOfTheLord,
        ]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                "mary_mother_of_god",
                date,
                Precedence::GeneralSolemnity_3,
                Some(Season::ChristmasTime),
                Color::White,
                None,
            )
            .with_periods(periods);

        Ok(liturgical_day)
    }

    /// Creates the Second Sunday after Christmas
    fn create_second_sunday_after_christmas(
        &self,
        date: DateTime<Utc>,
        epiphany_date: DateTime<Utc>,
    ) -> RomcalResult<LiturgicalDay> {
        let day_type = ProperOfTimeDayType::SecondSundayAfterChristmas;
        // Determine if before or after Epiphany
        let epiphany_period = if date < epiphany_date {
            Period::DaysBeforeEpiphany
        } else {
            Period::DaysFromEpiphany
        };
        let periods = self.proper_of_time.resolve_periods(vec![
            epiphany_period,
            Period::ChristmasToPresentationOfTheLord,
        ]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                "second_sunday_after_christmas",
                date,
                Precedence::UnprivilegedSunday_6,
                Some(Season::ChristmasTime),
                Color::White,
                Some(&day_type),
            )
            .with_periods(periods);

        Ok(liturgical_day)
    }

    /// Creates a weekday before Epiphany
    fn create_weekday_before_epiphany(
        &self,
        day: u8,
        date: DateTime<Utc>,
    ) -> RomcalResult<LiturgicalDay> {
        let day_type = ProperOfTimeDayType::BeforeEpiphany { day };
        let periods = self.proper_of_time.resolve_periods(vec![
            Period::DaysBeforeEpiphany,
            Period::ChristmasToPresentationOfTheLord,
        ]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                &format!("christmas_time_january_{}", day),
                date,
                Precedence::Weekday_13,
                Some(Season::ChristmasTime),
                Color::White,
                Some(&day_type),
            )
            .with_periods(periods);

        Ok(liturgical_day)
    }

    /// Creates the Epiphany of the Lord
    fn create_epiphany_of_the_lord(&self, date: DateTime<Utc>) -> RomcalResult<LiturgicalDay> {
        // Entity-based day, fullname comes from entity resolution
        let periods = self.proper_of_time.resolve_periods(vec![
            Period::DaysFromEpiphany,
            Period::ChristmasToPresentationOfTheLord,
        ]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                "epiphany_of_the_lord",
                date,
                Precedence::ProperOfTimeSolemnity_2,
                Some(Season::ChristmasTime),
                Color::White,
                None,
            )
            .with_periods(periods);

        Ok(liturgical_day)
    }

    /// Creates a weekday after Epiphany
    fn create_weekday_after_epiphany(
        &self,
        dow: u8,
        date: DateTime<Utc>,
    ) -> RomcalResult<LiturgicalDay> {
        let day_type = ProperOfTimeDayType::AfterEpiphany { dow };
        let periods = self.proper_of_time.resolve_periods(vec![
            Period::DaysFromEpiphany,
            Period::ChristmasToPresentationOfTheLord,
        ]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                &format!("{}_after_epiphany", WEEKDAY_NAMES[dow as usize]),
                date,
                Precedence::Weekday_13,
                Some(Season::ChristmasTime),
                Color::White,
                Some(&day_type),
            )
            .with_periods(periods);

        Ok(liturgical_day)
    }

    /// Creates the Baptism of the Lord
    fn create_baptism_of_the_lord(&self, date: DateTime<Utc>) -> RomcalResult<LiturgicalDay> {
        // Entity-based day, fullname comes from entity resolution
        let periods = self.proper_of_time.resolve_periods(vec![
            Period::DaysFromEpiphany,
            Period::ChristmasToPresentationOfTheLord,
        ]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                "baptism_of_the_lord",
                date,
                Precedence::ProperOfTimeSolemnity_2,
                Some(Season::ChristmasTime),
                Color::White,
                None,
            )
            .with_periods(periods);

        Ok(liturgical_day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::romcal::{Preset, Romcal};

    #[test]
    fn test_early_christmas_time_generation() {
        let romcal = Romcal::default();
        let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();
        let christmas_time = ChristmasTime::new(&proper_of_time);
        let christmas_days = christmas_time.generate_early().unwrap();

        // Check that we have generated days
        assert!(!christmas_days.is_empty());

        // Check that we have the Nativity of the Lord
        let nativity: Vec<_> = christmas_days
            .iter()
            .filter(|day| day.id == "nativity_of_the_lord")
            .collect();
        assert_eq!(nativity.len(), 1);

        // Check that we have the Holy Family
        let holy_family: Vec<_> = christmas_days
            .iter()
            .filter(|day| day.id == "holy_family_of_jesus_mary_and_joseph")
            .collect();
        assert_eq!(holy_family.len(), 1);

        // Check that we have Octave days
        let octave_days: Vec<_> = christmas_days
            .iter()
            .filter(|day| day.id.starts_with("christmas_octave_day_"))
            .collect();
        assert!(!octave_days.is_empty());
    }

    #[test]
    fn test_liturgical_year_early_christmas_time() {
        let romcal = Romcal::new(Preset {
            context: Some(crate::CalendarContext::Liturgical),
            ..Preset::default()
        });
        let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();
        let christmas_time = ChristmasTime::new(&proper_of_time);
        // For liturgical year 2026, Christmas is in 2025
        let christmas_days = christmas_time.generate_early().unwrap();

        // For liturgical year 2026, Christmas must be in 2025
        assert!(!christmas_days.is_empty());

        // Check that the dates are in 2025
        for day in &christmas_days {
            let year = day.date.split('-').next().unwrap().parse::<i32>().unwrap();
            assert_eq!(year, 2025);
        }
    }

    #[test]
    fn test_late_christmas_time_generation() {
        let romcal = Romcal::default();
        let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();
        let christmas_time = ChristmasTime::new(&proper_of_time);
        let christmas_days = christmas_time.generate_late().unwrap();

        // Check that we have generated days
        assert!(!christmas_days.is_empty());

        // Check that we have Mary, Mother of God
        let mary_mother_of_god: Vec<_> = christmas_days
            .iter()
            .filter(|day| day.id == "mary_mother_of_god")
            .collect();
        assert_eq!(mary_mother_of_god.len(), 1);
        assert_eq!(mary_mother_of_god[0].date, "2026-01-01");

        // Check that we have the Epiphany of the Lord
        let epiphany: Vec<_> = christmas_days
            .iter()
            .filter(|day| day.id == "epiphany_of_the_lord")
            .collect();
        assert_eq!(epiphany.len(), 1);

        // Check that we have the Baptism of the Lord
        let baptism: Vec<_> = christmas_days
            .iter()
            .filter(|day| day.id == "baptism_of_the_lord")
            .collect();
        assert_eq!(baptism.len(), 1);
    }

    #[test]
    fn test_liturgical_year_late_christmas_time() {
        let romcal = Romcal::new(Preset {
            context: Some(crate::CalendarContext::Liturgical),
            ..Preset::default()
        });
        let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();
        let christmas_time = ChristmasTime::new(&proper_of_time);

        let days = christmas_time.generate_late().unwrap();

        // Check that we have Mary, Mother of God
        let mary_mother_of_god = days.iter().find(|d| d.id == "mary_mother_of_god");
        assert!(mary_mother_of_god.is_some());
        assert_eq!(mary_mother_of_god.unwrap().date, "2026-01-01");
    }
}
