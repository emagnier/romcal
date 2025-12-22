//! Lent season generation.
//!
//! This module generates liturgical days for the Lenten season,
//! from Ash Wednesday to Holy Saturday (excluding Paschal Triduum).

use chrono::{DateTime, Utc};

use super::utils::{WEEKDAY_NAMES, sort_liturgical_days_by_date};
use crate::engine::liturgical_day::LiturgicalDay;
use crate::engine::template_resolver::ProperOfTimeDayType;
use crate::error::RomcalResult;
use crate::types::liturgical::{Color, Period, Precedence, Season};
use crate::types::mass::{MassInfo, MassTime};

use super::ProperOfTime;

/// Structure for generating Lent liturgical days
/// This encapsulates all Lent-specific logic that was previously in ProperOfTime
pub struct Lent<'a> {
    proper_of_time: &'a ProperOfTime,
}

impl<'a> Lent<'a> {
    /// Creates a new Lent instance
    pub fn new(proper_of_time: &'a ProperOfTime) -> Self {
        Self { proper_of_time }
    }

    /// Generates liturgical days of Lent
    ///
    /// Lent includes:
    /// - Ash Wednesday
    /// - Days after Ash Wednesday (Thursday-Saturday)
    /// - All days from 1st Sunday of Lent to Saturday of 5th week of Lent
    /// - Palm Sunday of the Passion of the Lord
    /// - Holy Week (Monday-Thursday)
    pub fn generate(&self) -> RomcalResult<Vec<LiturgicalDay>> {
        let mut days = Vec::new();

        // Use cached values
        let lent_year = self.proper_of_time.cache.lent_year();
        let ash_wednesday_date = self.proper_of_time.cache.lent_start();

        // LENT DAY TYPES:
        // 1. Ash Wednesday
        let day = self.create_ash_wednesday(ash_wednesday_date)?;
        days.push(day);

        // 2. Days after Ash Wednesday (Thursday-Saturday)
        for dow in 4..=6 {
            let weekday_date = ash_wednesday_date + chrono::Duration::days((dow - 3) as i64);
            let liturgical_day = self.create_weekday_after_ash_wednesday(dow, weekday_date)?;
            days.push(liturgical_day);
        }

        // 3. All days from 1st Sunday of Lent to Saturday of 5th week of Lent
        for i in 0..35 {
            let week = (i / 7) + 1;
            let dow = (i - (week - 1) * 7) as u8;

            let weekday_date = ash_wednesday_date + chrono::Duration::days((i + 4) as i64);
            let liturgical_day = self.create_lent_weekday(week, dow, weekday_date)?;
            days.push(liturgical_day);
        }

        // 4. Palm Sunday of the Passion of the Lord
        let palm_sunday_date = self
            .proper_of_time
            .dates
            .get_palm_sunday_date(Some(lent_year));
        let day = self.create_palm_sunday(palm_sunday_date)?;
        days.push(day);

        // 5. Holy Week (Monday-Thursday)
        for dow in 1..=4 {
            let weekday_date = palm_sunday_date + chrono::Duration::days(dow as i64);
            let liturgical_day = self.create_holy_week_weekday(dow, weekday_date)?;
            days.push(liturgical_day);
        }

        // TODO: Temporary fix to sort days by date
        sort_liturgical_days_by_date(&mut days);

        Ok(days)
    }

    // ---------------------------------------------------------------------------------
    // LENT DAY CREATION FUNCTIONS
    // ---------------------------------------------------------------------------------

    /// Creates Ash Wednesday
    fn create_ash_wednesday(&self, date: DateTime<Utc>) -> RomcalResult<LiturgicalDay> {
        // Entity-based day, fullname comes from entity resolution
        let periods = self
            .proper_of_time
            .resolve_periods(vec![Period::PresentationOfTheLordToHolyThursday]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                "ash_wednesday",
                date,
                Precedence::AshWednesday_2,
                Some(Season::Lent),
                Color::Purple,
                None,
            )
            .with_periods(periods);

        Ok(liturgical_day)
    }

    /// Creates a weekday after Ash Wednesday
    fn create_weekday_after_ash_wednesday(
        &self,
        dow: u8,
        date: DateTime<Utc>,
    ) -> RomcalResult<LiturgicalDay> {
        let day_type = ProperOfTimeDayType::DayAfterAshWednesday { dow };
        let periods = self
            .proper_of_time
            .resolve_periods(vec![Period::PresentationOfTheLordToHolyThursday]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                &format!("{}_after_ash_wednesday", WEEKDAY_NAMES[dow as usize]),
                date,
                Precedence::PrivilegedWeekday_9,
                Some(Season::Lent),
                Color::Purple,
                Some(&day_type),
            )
            .with_periods(periods);

        Ok(liturgical_day)
    }

    /// Creates a Lent weekday
    fn create_lent_weekday(
        &self,
        week: u32,
        dow: u8,
        date: DateTime<Utc>,
    ) -> RomcalResult<LiturgicalDay> {
        let day_type = if dow == 0 {
            ProperOfTimeDayType::LentSunday { week: week as u8 }
        } else {
            ProperOfTimeDayType::LentWeekday {
                week: week as u8,
                dow,
            }
        };
        let periods = self
            .proper_of_time
            .resolve_periods(vec![Period::PresentationOfTheLordToHolyThursday]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                &format!("lent_{}_{}", week, WEEKDAY_NAMES[dow as usize]),
                date,
                if dow == 0 {
                    Precedence::PrivilegedSunday_2
                } else {
                    Precedence::PrivilegedWeekday_9
                },
                Some(Season::Lent),
                if week == 4 && dow == 0 {
                    Color::Rose
                } else {
                    Color::Purple
                },
                Some(&day_type),
            )
            .with_periods(periods);

        Ok(liturgical_day)
    }

    /// Creates Palm Sunday of the Passion of the Lord
    fn create_palm_sunday(&self, date: DateTime<Utc>) -> RomcalResult<LiturgicalDay> {
        // Entity-based day, fullname comes from entity resolution
        let periods = self.proper_of_time.resolve_periods(vec![
            Period::HolyWeek,
            Period::PresentationOfTheLordToHolyThursday,
        ]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                "palm_sunday_of_the_passion_of_the_lord",
                date,
                Precedence::PrivilegedSunday_2,
                Some(Season::Lent),
                Color::Red,
                None,
            )
            .with_periods(periods)
            .with_is_holy_day_of_obligation(true)
            .with_masses(vec![MassInfo::new(MassTime::MassOfThePassion)]);

        Ok(liturgical_day)
    }

    /// Creates a Holy Week weekday
    fn create_holy_week_weekday(
        &self,
        dow: u8,
        date: DateTime<Utc>,
    ) -> RomcalResult<LiturgicalDay> {
        let day_type = ProperOfTimeDayType::HolyWeekDay { dow };
        let periods = self.proper_of_time.resolve_periods(vec![
            Period::HolyWeek,
            Period::PresentationOfTheLordToHolyThursday,
        ]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                &format!("holy_{}", WEEKDAY_NAMES[dow as usize]),
                date,
                Precedence::PrivilegedWeekday_9,
                Some(Season::Lent),
                Color::Purple,
                Some(&day_type),
            )
            .with_periods(periods);

        Ok(liturgical_day)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Romcal, romcal::Preset};

    #[test]
    fn test_lent_generation() {
        let romcal = Romcal::default();
        let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();
        let lent = Lent::new(&proper_of_time);
        let lent_days = lent.generate().unwrap();

        // Should have at least Ash Wednesday, Palm Sunday, and Lent weekdays
        assert!(!lent_days.is_empty());

        // Check for Ash Wednesday
        let ash_wednesday = lent_days.iter().find(|d| d.id == "ash_wednesday");
        assert!(ash_wednesday.is_some());

        // Check for Palm Sunday
        let palm_sunday = lent_days
            .iter()
            .find(|d| d.id == "palm_sunday_of_the_passion_of_the_lord");
        assert!(palm_sunday.is_some());

        // Check for Lent weekdays
        let lent_weekdays: Vec<_> = lent_days
            .iter()
            .filter(|d| d.id.starts_with("lent_"))
            .collect();
        assert!(!lent_weekdays.is_empty());

        // Check for Holy Week weekdays
        let holy_week_days: Vec<_> = lent_days
            .iter()
            .filter(|d| d.id.starts_with("holy_"))
            .collect();
        assert!(!holy_week_days.is_empty());
    }

    #[test]
    fn test_liturgical_year_lent() {
        let romcal = Romcal::new(Preset {
            context: Some(crate::CalendarContext::Liturgical),
            ..Preset::default()
        });
        let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();
        let lent = Lent::new(&proper_of_time);

        let days = lent.generate().unwrap();

        // Should have at least Ash Wednesday, Palm Sunday, and Lent weekdays
        assert!(!days.is_empty());

        // Check for Ash Wednesday
        let ash_wednesday = days.iter().find(|d| d.id == "ash_wednesday");
        assert!(ash_wednesday.is_some());

        // Check for Palm Sunday
        let palm_sunday = days
            .iter()
            .find(|d| d.id == "palm_sunday_of_the_passion_of_the_lord");
        assert!(palm_sunday.is_some());
    }
}
