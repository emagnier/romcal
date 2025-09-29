use chrono::{DateTime, Utc};

use crate::error::RomcalResult;
use crate::liturgical_day::LiturgicalDay;
use crate::proper_of_time::common::sort_liturgical_days_by_date;
use crate::types::liturgical::{Color, Precedence, Rank, Season};

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
        let liturgical_day = self.proper_of_time.create_liturgical_day_base(
            "thursday_of_the_lords_supper",
            date,
            Precedence::Triduum_1,
            Rank::Weekday,
            Season::PaschalTriduum,
            Color::White,
        );

        Ok(liturgical_day)
    }

    /// Creates Good Friday (Friday of the Passion of the Lord)
    fn create_good_friday(&self, date: DateTime<Utc>) -> RomcalResult<LiturgicalDay> {
        let liturgical_day = self.proper_of_time.create_liturgical_day_base(
            "friday_of_the_passion_of_the_lord",
            date,
            Precedence::Triduum_1,
            Rank::Weekday,
            Season::PaschalTriduum,
            Color::Red,
        );

        Ok(liturgical_day)
    }

    /// Creates Holy Saturday
    fn create_holy_saturday(&self, date: DateTime<Utc>) -> RomcalResult<LiturgicalDay> {
        let liturgical_day = self.proper_of_time.create_liturgical_day_base(
            "holy_saturday",
            date,
            Precedence::Triduum_1,
            Rank::Weekday,
            Season::PaschalTriduum,
            Color::White, // Using White as default, can be overridden if needed
        );

        Ok(liturgical_day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preset::{Preset, PresetPartial};

    #[test]
    fn test_paschal_triduum_generation() {
        let preset = Preset::default();
        let proper_of_time = ProperOfTime::new(preset, 2026).unwrap();
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
        let preset = Preset::new(PresetPartial {
            context: Some(crate::CalendarContext::Liturgical),
            ..PresetPartial::default()
        });
        let proper_of_time = ProperOfTime::new(preset, 2026).unwrap();
        let paschal_triduum = PaschalTriduum::new(&proper_of_time);

        let days = paschal_triduum.generate().unwrap();

        // Should have exactly 3 days: Holy Thursday, Good Friday, Holy Saturday
        assert_eq!(days.len(), 3);

        // Check for Holy Thursday
        let holy_thursday = days.iter().find(|d| d.id == "thursday_of_the_lords_supper");
        assert!(holy_thursday.is_some());
    }
}
