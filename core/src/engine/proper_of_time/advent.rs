//! Advent season generation.
//!
//! This module generates liturgical days for the Advent season,
//! from the first Sunday of Advent to December 24.

use chrono::{DateTime, Datelike, Utc};

use super::utils::{WEEKDAY_NAMES, enum_to_string, sort_liturgical_days_by_date};
use crate::engine::liturgical_day::LiturgicalDay;
use crate::engine::template_resolver::ProperOfTimeDayType;
use crate::error::RomcalResult;
use crate::types::liturgical::{Color, ColorInfo, Precedence, Season};
use crate::types::mass::{MassInfo, MassTime};

use super::ProperOfTime;

/// Structure for generating Advent liturgical days
/// This encapsulates all Advent-specific logic that was previously in ProperOfTime
pub struct Advent<'a> {
    proper_of_time: &'a ProperOfTime,
}

impl<'a> Advent<'a> {
    /// Creates a new Advent instance
    pub fn new(proper_of_time: &'a ProperOfTime) -> Self {
        Self { proper_of_time }
    }

    /// Generates liturgical days of Advent
    ///
    /// Advent begins on the first Sunday of Advent and ends on December 24.
    pub fn generate(&self) -> RomcalResult<Vec<LiturgicalDay>> {
        let mut days = Vec::new();

        // Use cached values
        let advent_year = self.proper_of_time.cache.advent_year();

        // ADVENT DAY TYPES:
        // 1. Advent Sundays (4 Sundays)
        for week in 1..=4 {
            if let Some(sunday_date) = self
                .proper_of_time
                .dates
                .get_sunday_of_advent_date(week, Some(advent_year))
            {
                let day = self.create_advent_sunday(week, sunday_date)?;
                days.push(day);
            }
        }

        // 2. Advent Weekdays (Monday-Saturday, weeks 1-3)
        for week in 1..=3 {
            for dow in 1..=6 {
                // Monday to Saturday
                if let Some(weekday_date) = self
                    .proper_of_time
                    .dates
                    .unprivileged_weekday_of_advent(dow, week, Some(advent_year))
                {
                    let day = self.create_advent_weekday(week, dow, weekday_date)?;
                    days.push(day);
                }
            }
        }

        // 3. Privileged Advent Weekdays (December 17-24)
        // Calculate the first Sunday once to avoid recalculating it for each day
        for day in 17..=24 {
            if let Some(privileged_date) = self
                .proper_of_time
                .dates
                .privileged_weekday_of_advent(day, Some(advent_year))
            {
                // Calculate the correct week based on the date
                // December 17-24 can span both week 3 and week 4 of Advent
                let liturgical_day = self.create_privileged_advent_weekday(day, privileged_date)?;
                days.push(liturgical_day);
            }
        }

        // TODO: Temporary fix to sort days by date
        sort_liturgical_days_by_date(&mut days);

        Ok(days)
    }

    // ---------------------------------------------------------------------------------
    // ADVENT DAY CREATION FUNCTIONS
    // ---------------------------------------------------------------------------------

    /// Creates an Advent Sunday
    fn create_advent_sunday(&self, week: u8, date: DateTime<Utc>) -> RomcalResult<LiturgicalDay> {
        let day_type = ProperOfTimeDayType::AdventSunday { week };
        let mut liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                &format!("advent_{}_sunday", week),
                date,
                Precedence::PrivilegedSunday_2,
                Some(Season::Advent),
                Color::Purple,
                Some(&day_type),
            )
            .with_is_holy_day_of_obligation(true);

        // Colors (rose for the 3rd Sunday - Gaudete)
        if week == 3 {
            liturgical_day.colors = vec![
                ColorInfo {
                    key: Color::Rose,
                    name: enum_to_string(&Color::Rose),
                },
                ColorInfo {
                    key: Color::Purple,
                    name: enum_to_string(&Color::Purple),
                },
            ];
        }

        Ok(liturgical_day)
    }

    /// Creates an Advent weekday
    fn create_advent_weekday(
        &self,
        week: u8,
        dow: u8,
        date: DateTime<Utc>,
    ) -> RomcalResult<LiturgicalDay> {
        let day_type = ProperOfTimeDayType::AdventWeekday { week, dow };
        let liturgical_day = self.proper_of_time.create_liturgical_day_base(
            &format!("advent_{}_{}", week, WEEKDAY_NAMES[dow as usize]),
            date,
            Precedence::Weekday_13,
            Some(Season::Advent),
            Color::Purple,
            Some(&day_type),
        );

        Ok(liturgical_day)
    }

    /// Creates a privileged Advent weekday (December 17-24)
    fn create_privileged_advent_weekday(
        &self,
        day: u8,
        date: DateTime<Utc>,
    ) -> RomcalResult<LiturgicalDay> {
        let dow = date.weekday().num_days_from_sunday() as u8;
        let day_type = ProperOfTimeDayType::AdventPrivilegedWeekday { day, dow };
        let mut liturgical_day = self.proper_of_time.create_liturgical_day_base(
            &format!("advent_december_{}", day),
            date,
            Precedence::PrivilegedWeekday_9,
            Some(Season::Advent),
            Color::Purple,
            Some(&day_type),
        );

        // December 24 only has a morning mass (evening is Christmas Eve)
        if day == 24 {
            liturgical_day = liturgical_day.with_masses(vec![MassInfo::new(MassTime::MorningMass)]);
        }

        Ok(liturgical_day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Romcal, romcal::Preset};

    #[test]
    fn test_advent_generation() {
        let romcal = Romcal::default();
        let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();
        let advent = Advent::new(&proper_of_time);
        let advent_days = advent.generate().unwrap();

        // Check that we have generated days
        assert!(!advent_days.is_empty());

        // Check that we have the 4 Sundays
        let sundays: Vec<_> = advent_days
            .iter()
            .filter(|day| day.id.contains("sunday"))
            .collect();
        assert_eq!(sundays.len(), 4);
    }

    #[test]
    fn test_liturgical_year_advent() {
        let romcal = Romcal::new(Preset {
            context: Some(crate::CalendarContext::Liturgical),
            ..Preset::default()
        });
        let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();
        let advent = Advent::new(&proper_of_time);
        // For liturgical year 2026, Advent begins in 2025
        let advent_days = advent.generate().unwrap();

        // For liturgical year 2026, Advent must begin in 2025
        assert!(!advent_days.is_empty());

        // Check that the dates are in 2025
        for day in &advent_days {
            let year = day.date.split('-').next().unwrap().parse::<i32>().unwrap();
            assert_eq!(year, 2025);
        }
    }
}
