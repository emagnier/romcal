//! Paschal Triduum generation.
//!
//! This module generates liturgical days for the Paschal Triduum,
//! from Holy Thursday evening to Easter Sunday.

use chrono::{DateTime, Utc};

use super::utils::sort_liturgical_days_by_date;
use crate::engine::liturgical_day::LiturgicalDay;
use crate::engine::template_resolver::ProperOfTimeDayType;
use crate::error::RomcalResult;
use crate::types::liturgical::{Color, Period, Precedence, Season};
use crate::types::mass::{MassInfo, MassTime};

use super::ProperOfTime;

/// Structure for generating Paschal Triduum liturgical days
/// This encapsulates all Paschal Triduum-specific logic that was previously in ProperOfTime
pub struct PaschalTriduum<'a> {
    proper_of_time: &'a ProperOfTime,
}

impl<'a> PaschalTriduum<'a> {
    /// Creates a new PaschalTriduum instance
    pub fn new(proper_of_time: &'a ProperOfTime) -> Self {
        Self { proper_of_time }
    }

    /// Generates liturgical days of the Paschal Triduum
    ///
    /// The Paschal Triduum includes:
    /// - Thursday of the Lord's Supper (Holy Thursday)
    /// - Friday of the Passion of the Lord (Good Friday)
    /// - Holy Saturday
    /// - Easter Sunday of the Resurrection of the Lord
    pub fn generate(&self) -> RomcalResult<Vec<LiturgicalDay>> {
        let mut days = Vec::new();

        // Use cached values
        let triduum_year = self.proper_of_time.cache.triduum_year();
        let holy_thursday_date = self.proper_of_time.cache.triduum_start();

        // PASCHAL TRIDUUM DAY TYPES:
        // 1. Thursday of the Lord's Supper (Holy Thursday)
        let day = self.create_holy_thursday(holy_thursday_date)?;
        days.push(day);

        // 2. Friday of the Passion of the Lord (Good Friday)
        let good_friday_date = self
            .proper_of_time
            .dates
            .get_good_friday_date(Some(triduum_year));
        let day = self.create_good_friday(good_friday_date)?;
        days.push(day);

        // 3. Holy Saturday
        let holy_saturday_date = self
            .proper_of_time
            .dates
            .get_holy_saturday_date(Some(triduum_year));
        let day = self.create_holy_saturday(holy_saturday_date)?;
        days.push(day);

        // TODO: Temporary fix to sort days by date
        sort_liturgical_days_by_date(&mut days);

        Ok(days)
    }

    // ---------------------------------------------------------------------------------
    // PASCHAL TRIDUUM DAY CREATION FUNCTIONS
    // ---------------------------------------------------------------------------------

    /// Creates Holy Thursday (Thursday of the Lord's Supper)
    fn create_holy_thursday(&self, date: DateTime<Utc>) -> RomcalResult<LiturgicalDay> {
        let day_type = ProperOfTimeDayType::HolyThursday;
        let periods = self
            .proper_of_time
            .resolve_periods(vec![Period::PaschalTriduum, Period::HolyWeek]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                "thursday_of_the_lords_supper",
                date,
                Precedence::Triduum_1,
                Some(Season::PaschalTriduum),
                Color::White,
                Some(&day_type),
            )
            .with_periods(periods)
            .with_masses(vec![MassInfo::new(MassTime::EveningMassOfTheLordsSupper)]);

        Ok(liturgical_day)
    }

    /// Creates Good Friday (Friday of the Passion of the Lord)
    fn create_good_friday(&self, date: DateTime<Utc>) -> RomcalResult<LiturgicalDay> {
        let day_type = ProperOfTimeDayType::GoodFriday;
        let periods = self
            .proper_of_time
            .resolve_periods(vec![Period::PaschalTriduum, Period::HolyWeek]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                "friday_of_the_passion_of_the_lord",
                date,
                Precedence::Triduum_1,
                Some(Season::PaschalTriduum),
                Color::Red,
                Some(&day_type),
            )
            .with_periods(periods)
            .with_masses(vec![MassInfo::new(MassTime::CelebrationOfThePassion)]);

        Ok(liturgical_day)
    }

    /// Creates Holy Saturday
    fn create_holy_saturday(&self, date: DateTime<Utc>) -> RomcalResult<LiturgicalDay> {
        let day_type = ProperOfTimeDayType::HolySaturday;
        let periods = self
            .proper_of_time
            .resolve_periods(vec![Period::PaschalTriduum, Period::HolyWeek]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                "holy_saturday",
                date,
                Precedence::Triduum_1,
                Some(Season::PaschalTriduum),
                Color::White, // Using White as default, can be overridden if needed
                Some(&day_type),
            )
            .with_periods(periods)
            .with_masses(MassInfo::none()); // Aliturgical day - no mass

        Ok(liturgical_day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::romcal::{Preset, Romcal};

    #[test]
    fn test_paschal_triduum_generation() {
        let romcal = Romcal::default();
        let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();
        let paschal_triduum = PaschalTriduum::new(&proper_of_time);

        let days = paschal_triduum.generate().unwrap();

        // Should have exactly 3 days: Holy Thursday, Good Friday, Holy Saturday
        assert_eq!(days.len(), 3);

        // Check for Holy Thursday
        let holy_thursday = days.iter().find(|d| d.id == "thursday_of_the_lords_supper");
        assert!(holy_thursday.is_some());

        // Check for Good Friday
        let good_friday = days
            .iter()
            .find(|d| d.id == "friday_of_the_passion_of_the_lord");
        assert!(good_friday.is_some());

        // Check for Holy Saturday
        let holy_saturday = days.iter().find(|d| d.id == "holy_saturday");
        assert!(holy_saturday.is_some());
    }

    #[test]
    fn test_liturgical_year_paschal_triduum() {
        let romcal = Romcal::new(Preset {
            context: Some(crate::CalendarContext::Liturgical),
            ..Preset::default()
        });
        let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();
        let paschal_triduum = PaschalTriduum::new(&proper_of_time);

        let days = paschal_triduum.generate().unwrap();

        // Should have exactly 3 days: Holy Thursday, Good Friday, Holy Saturday
        assert_eq!(days.len(), 3);

        // Check for Holy Thursday
        let holy_thursday = days.iter().find(|d| d.id == "thursday_of_the_lords_supper");
        assert!(holy_thursday.is_some());
    }
}
