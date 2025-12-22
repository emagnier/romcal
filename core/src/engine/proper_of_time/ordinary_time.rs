//! Ordinary Time generation.
//!
//! This module generates liturgical days for Ordinary Time,
//! covering both early (after Baptism of the Lord) and late
//! (after Pentecost) periods.

use chrono::{DateTime, Datelike, Utc};

use super::utils::{WEEKDAY_NAMES, sort_liturgical_days_by_date};
use crate::engine::liturgical_day::LiturgicalDay;
use crate::engine::template_resolver::ProperOfTimeDayType;
use crate::error::RomcalResult;
use crate::types::liturgical::{Color, Period, Precedence, Season};

use super::ProperOfTime;

/// Structure for generating Ordinary Time liturgical days
/// This encapsulates all Ordinary Time-specific logic that was previously in ProperOfTime
pub struct OrdinaryTime<'a> {
    proper_of_time: &'a ProperOfTime,
}

impl<'a> OrdinaryTime<'a> {
    /// Creates a new OrdinaryTime instance
    pub fn new(proper_of_time: &'a ProperOfTime) -> Self {
        Self { proper_of_time }
    }

    /// Generates liturgical days of early Ordinary Time
    ///
    /// Early Ordinary Time includes:
    /// - All Sundays and weekdays from the day after the Baptism of the Lord to the day before Ash Wednesday
    /// - Special days: Sunday of the Word of God (3rd week)
    ///
    /// Note: The first week of early Ordinary Time is incomplete (no Sunday, possibly no Monday)
    /// because the Sunday is either Epiphany or Baptism of the Lord, and Monday may be missing
    /// if Baptism of the Lord falls on Monday.
    pub fn generate_early(&self) -> RomcalResult<Vec<LiturgicalDay>> {
        let mut days = Vec::new();

        // Use cached values
        let ordinary_year = self.proper_of_time.cache.easter_year(); // Same as easter year for early ordinary time

        // All days of early Ordinary Time
        let early_ordinary_dates = self
            .proper_of_time
            .dates
            .get_all_dates_of_early_ordinary_time(Some(ordinary_year));

        // Find the first Sunday in early Ordinary Time to calculate weeks correctly
        let first_sunday = early_ordinary_dates
            .iter()
            .find(|date| date.weekday() == chrono::Weekday::Sun)
            .copied()
            .unwrap_or_else(|| early_ordinary_dates[0]);

        for ordinary_date in early_ordinary_dates.iter() {
            let dow = ordinary_date.weekday().num_days_from_sunday() as u8;

            // Calculate week number using the specialized function
            let week = self.calculate_ordinary_time_week(*ordinary_date, first_sunday, true) as u8;

            // Special cases for specific Sundays
            if week == 3 && dow == 0 {
                // Sunday of the Word of God (3rd week)
                let liturgical_day = self
                    .create_sunday_of_the_word_of_god(*ordinary_date, Period::EarlyOrdinaryTime)?;
                days.push(liturgical_day);
            } else {
                // Regular Ordinary Time day
                let liturgical_day = self.create_ordinary_time_day(
                    week,
                    dow,
                    *ordinary_date,
                    Period::EarlyOrdinaryTime,
                )?;
                days.push(liturgical_day);
            }
        }

        // TODO: Temporary fix to sort days by date
        sort_liturgical_days_by_date(&mut days);

        Ok(days)
    }

    /// Generates liturgical days of late Ordinary Time
    ///
    /// Late Ordinary Time includes:
    /// - The Most Holy Trinity (Trinity Sunday)
    /// - The Most Holy Body and Blood of Christ (Corpus Christi)
    /// - The Most Sacred Heart of Jesus
    /// - All Sundays and weekdays from the day after Pentecost to the day before the First Sunday of Advent
    /// - Special days: Christ the King (34th week)
    ///
    /// Note: The first week of late Ordinary Time is incomplete (Monday to Saturday only)
    /// because the Sunday is Pentecost Sunday. All subsequent weeks are complete.
    pub fn generate_late(&self) -> RomcalResult<Vec<LiturgicalDay>> {
        let mut days = Vec::new();

        // Use cached values
        let ordinary_year = self.proper_of_time.cache.easter_year(); // Same as easter year for late ordinary time

        // Get solemnity dates for later use
        let trinity_date = self
            .proper_of_time
            .dates
            .get_trinity_sunday_date(Some(ordinary_year));
        let corpus_christi_date = self
            .proper_of_time
            .dates
            .get_corpus_christi_date(Some(ordinary_year));
        let sacred_heart_date = self
            .proper_of_time
            .dates
            .get_most_sacred_heart_of_jesus_date(Some(ordinary_year));

        // 4. All days of late Ordinary Time
        let late_ordinary_dates = self
            .proper_of_time
            .dates
            .get_all_dates_of_late_ordinary_time(Some(ordinary_year));

        // In late Ordinary Time, the first week is incomplete (Monday to Saturday, no Sunday)
        // because the Sunday is Pentecost Sunday. Then all weeks are complete until the last week (34th)
        let first_sunday_idx = late_ordinary_dates
            .iter()
            .position(|date| date.weekday() == chrono::Weekday::Sun)
            .unwrap_or(0);

        // Calculate how many complete weeks we have after the next Sunday after the Pentecost Sunday
        let complete_weeks_after_first_sunday = (late_ordinary_dates.len() - first_sunday_idx) / 7;
        // We need to end at week 34, so calculate the starting week
        let late_start_week = 34 - complete_weeks_after_first_sunday;

        for (i, ordinary_date) in late_ordinary_dates.iter().enumerate() {
            let dow = ordinary_date.weekday().num_days_from_sunday() as u8;

            // Calculate week number based on the first Sunday
            let week = if i < first_sunday_idx {
                // Days before the first Sunday are in the first incomplete week
                late_start_week
            } else {
                // Calculate week from the next Sunday after the Pentecost Sunday (which starts the complete weeks of the late Ordinary Time)
                late_start_week + 1 + ((i - first_sunday_idx) / 7)
            } as u8;

            // Check if this date is a solemnity and create the appropriate liturgical day
            if *ordinary_date == trinity_date {
                // Trinity Sunday
                let liturgical_day =
                    self.create_most_holy_trinity(*ordinary_date, Period::LateOrdinaryTime)?;
                days.push(liturgical_day);
            } else if *ordinary_date == corpus_christi_date {
                // Corpus Christi
                let liturgical_day = self.create_most_holy_body_and_blood_of_christ(
                    *ordinary_date,
                    Period::LateOrdinaryTime,
                )?;
                days.push(liturgical_day);
            } else if *ordinary_date == sacred_heart_date {
                // Sacred Heart
                let liturgical_day = self
                    .create_most_sacred_heart_of_jesus(*ordinary_date, Period::LateOrdinaryTime)?;
                days.push(liturgical_day);
            } else if week == 34 && dow == 0 {
                // Christ the King (34th week)
                let liturgical_day = self.create_our_lord_jesus_christ_king_of_the_universe(
                    *ordinary_date,
                    Period::LateOrdinaryTime,
                )?;
                days.push(liturgical_day);
            } else {
                // Regular Ordinary Time day
                let liturgical_day = self.create_ordinary_time_day(
                    week,
                    dow,
                    *ordinary_date,
                    Period::LateOrdinaryTime,
                )?;
                days.push(liturgical_day);
            }
        }

        // TODO: Temporary fix to sort days by date
        sort_liturgical_days_by_date(&mut days);

        Ok(days)
    }

    // ---------------------------------------------------------------------------------
    // ORDINARY TIME DAY CREATION FUNCTIONS
    // ---------------------------------------------------------------------------------

    /// Calculates the week number for Ordinary Time based on the first Sunday
    /// Handles the complex logic for incomplete first weeks
    fn calculate_ordinary_time_week(
        &self,
        date: DateTime<Utc>,
        first_sunday: DateTime<Utc>,
        is_early: bool,
    ) -> u32 {
        let days_since_first_sunday = (date.date_naive() - first_sunday.date_naive()).num_days();

        if days_since_first_sunday < 0 {
            // Days before the first Sunday are in week 1 (incomplete week)
            1
        } else {
            // Calculate week from the first Sunday
            let week = (days_since_first_sunday / 7) + 1;

            if is_early {
                // Early Ordinary Time: first Sunday is week 2, so add 1 to the calculated week
                (week + 1) as u32
            } else {
                // Late Ordinary Time: first Sunday is week 1
                week as u32
            }
        }
    }

    /// Creates a regular Ordinary Time day
    fn create_ordinary_time_day(
        &self,
        week: u8,
        dow: u8,
        date: DateTime<Utc>,
        period: Period,
    ) -> RomcalResult<LiturgicalDay> {
        let day_type = if dow == 0 {
            ProperOfTimeDayType::OrdinaryTimeSunday { week }
        } else {
            ProperOfTimeDayType::OrdinaryTimeWeekday { week, dow }
        };
        let periods = self.proper_of_time.resolve_periods(vec![period]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                &format!("ordinary_time_{}_{}", week, WEEKDAY_NAMES[dow as usize]),
                date,
                if dow == 0 {
                    Precedence::UnprivilegedSunday_6
                } else {
                    Precedence::Weekday_13
                },
                Some(Season::OrdinaryTime),
                Color::Green,
                Some(&day_type),
            )
            .with_periods(periods);

        Ok(liturgical_day)
    }

    /// Creates the Sunday of the Word of God (3rd week of Ordinary Time)
    fn create_sunday_of_the_word_of_god(
        &self,
        date: DateTime<Utc>,
        period: Period,
    ) -> RomcalResult<LiturgicalDay> {
        // Entity-based day, fullname comes from entity resolution
        let periods = self.proper_of_time.resolve_periods(vec![period]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                "sunday_of_the_word_of_god",
                date,
                Precedence::UnprivilegedSunday_6,
                Some(Season::OrdinaryTime),
                Color::Green,
                None,
            )
            .with_periods(periods);

        Ok(liturgical_day)
    }

    /// Creates the Most Holy Trinity (Trinity Sunday)
    fn create_most_holy_trinity(
        &self,
        date: DateTime<Utc>,
        period: Period,
    ) -> RomcalResult<LiturgicalDay> {
        // Entity-based day, fullname comes from entity resolution
        let periods = self.proper_of_time.resolve_periods(vec![period]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                "most_holy_trinity",
                date,
                Precedence::GeneralSolemnity_3,
                Some(Season::OrdinaryTime),
                Color::White,
                None,
            )
            .with_periods(periods);

        Ok(liturgical_day)
    }

    /// Creates the Most Holy Body and Blood of Christ (Corpus Christi)
    fn create_most_holy_body_and_blood_of_christ(
        &self,
        date: DateTime<Utc>,
        period: Period,
    ) -> RomcalResult<LiturgicalDay> {
        // Entity-based day, fullname comes from entity resolution
        let periods = self.proper_of_time.resolve_periods(vec![period]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                "most_holy_body_and_blood_of_christ",
                date,
                Precedence::GeneralSolemnity_3,
                Some(Season::OrdinaryTime),
                Color::White,
                None,
            )
            .with_periods(periods);

        Ok(liturgical_day)
    }

    /// Creates the Most Sacred Heart of Jesus
    fn create_most_sacred_heart_of_jesus(
        &self,
        date: DateTime<Utc>,
        period: Period,
    ) -> RomcalResult<LiturgicalDay> {
        // Entity-based day, fullname comes from entity resolution
        let periods = self.proper_of_time.resolve_periods(vec![period]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                "most_sacred_heart_of_jesus",
                date,
                Precedence::GeneralSolemnity_3,
                Some(Season::OrdinaryTime),
                Color::White,
                None,
            )
            .with_periods(periods);

        Ok(liturgical_day)
    }

    /// Creates Our Lord Jesus Christ, King of the Universe (34th week of Ordinary Time)
    fn create_our_lord_jesus_christ_king_of_the_universe(
        &self,
        date: DateTime<Utc>,
        period: Period,
    ) -> RomcalResult<LiturgicalDay> {
        // Entity-based day, fullname comes from entity resolution
        let periods = self.proper_of_time.resolve_periods(vec![period]);
        let liturgical_day = self
            .proper_of_time
            .create_liturgical_day_base(
                "our_lord_jesus_christ_king_of_the_universe",
                date,
                Precedence::GeneralSolemnity_3,
                Some(Season::OrdinaryTime),
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
    use crate::{
        romcal::{Preset, Romcal},
        types::Rank,
    };

    #[test]
    fn test_early_ordinary_time_generation() {
        let romcal = Romcal::default();
        let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();
        let ordinary_time = OrdinaryTime::new(&proper_of_time);

        let days = ordinary_time.generate_early().unwrap();

        // Check for Sunday of the Word of God (3rd week)
        let word_of_god = days.iter().find(|d| d.id == "sunday_of_the_word_of_god");
        assert!(
            word_of_god.is_some(),
            "Sunday of the Word of God should be present in early ordinary time"
        );

        // Check for some regular Ordinary Time days
        let ordinary_weekday = days.iter().find(|d| d.season == Some(Season::OrdinaryTime));
        assert!(
            ordinary_weekday.is_some(),
            "Should have ordinary time weekdays"
        );

        // Check that all days are in Ordinary Time season
        for day in &days {
            assert!(
                day.season == Some(Season::OrdinaryTime),
                "All days should be in Ordinary Time season, but {} is not",
                day.id
            );
        }
    }

    #[test]
    fn test_early_ordinary_time_first_week_incomplete_baptism_sunday() {
        // Test with epiphany_on_sunday = false so Baptism of the Lord falls on Sunday
        // This means early Ordinary Time starts on Monday (no Sunday in first week)
        let romcal = Romcal::new(Preset {
            epiphany_on_sunday: Some(false),
            ..Preset::default()
        });
        let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();
        let ordinary_time = OrdinaryTime::new(&proper_of_time);

        let days = ordinary_time.generate_early().unwrap();

        // Should have some days
        assert!(!days.is_empty());

        // Check that all days are in Ordinary Time season
        for day in &days {
            assert!(
                day.season == Some(Season::OrdinaryTime),
                "All days should be in Ordinary Time season, but {} is not",
                day.id
            );
        }
    }

    #[test]
    fn test_early_ordinary_time_first_week_incomplete_baptism_monday() {
        // Test with epiphany_on_sunday = true so Baptism of the Lord falls on Monday
        // This means early Ordinary Time starts on Tuesday (no Sunday, no Monday in first week)
        let romcal = Romcal::new(Preset {
            epiphany_on_sunday: Some(true),
            ..Preset::default()
        });
        let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();
        let ordinary_time = OrdinaryTime::new(&proper_of_time);

        let days = ordinary_time.generate_early().unwrap();

        // Should have some days
        assert!(!days.is_empty());

        // Check that all days are in Ordinary Time season
        for day in &days {
            assert!(
                day.season == Some(Season::OrdinaryTime),
                "All days should be in Ordinary Time season, but {} is not",
                day.id
            );
        }
    }

    #[test]
    fn test_late_ordinary_time_generation() {
        let romcal = Romcal::default();
        let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();
        let ordinary_time = OrdinaryTime::new(&proper_of_time);

        let days = ordinary_time.generate_late().unwrap();

        // Check for the Most Holy Trinity
        let trinity = days.iter().find(|d| d.id == "most_holy_trinity");
        assert!(
            trinity.is_some(),
            "Most Holy Trinity should be present in late ordinary time"
        );

        // Check for Corpus Christi
        let corpus_christi = days
            .iter()
            .find(|d| d.id == "most_holy_body_and_blood_of_christ");
        assert!(
            corpus_christi.is_some(),
            "Corpus Christi should be present in late ordinary time"
        );

        // Check for the Most Sacred Heart of Jesus
        let sacred_heart = days.iter().find(|d| d.id == "most_sacred_heart_of_jesus");
        assert!(
            sacred_heart.is_some(),
            "Most Sacred Heart of Jesus should be present in late ordinary time"
        );

        // Check for Christ the King (34th week)
        let christ_king = days
            .iter()
            .find(|d| d.id == "our_lord_jesus_christ_king_of_the_universe");
        assert!(
            christ_king.is_some(),
            "Christ the King should be present in late ordinary time"
        );

        // Check for some regular Ordinary Time days
        let ordinary_weekday = days.iter().find(|d| d.season == Some(Season::OrdinaryTime));
        assert!(
            ordinary_weekday.is_some(),
            "Should have ordinary time weekdays"
        );

        // Check that all days are in Ordinary Time season
        for day in &days {
            assert!(
                day.season == Some(Season::OrdinaryTime),
                "All days should be in Ordinary Time season, but {} is not",
                day.id
            );
        }
    }

    #[test]
    fn test_late_ordinary_time_first_week_incomplete() {
        let romcal = Romcal::default();
        let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();
        let ordinary_time = OrdinaryTime::new(&proper_of_time);

        let days = ordinary_time.generate_late().unwrap();

        // Should have some days
        assert!(!days.is_empty());

        // Check that all days are in Ordinary Time season
        for day in &days {
            assert!(
                day.season == Some(Season::OrdinaryTime),
                "All days should be in Ordinary Time season, but {} is not",
                day.id
            );
        }
    }

    #[test]
    fn test_late_ordinary_time_34th_week_christ_king() {
        // Test that the last week of late Ordinary Time is always the 34th week
        // and that the Sunday of this week is Christ the King
        let romcal = Romcal::default();
        let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();
        let ordinary_time = OrdinaryTime::new(&proper_of_time);

        let days = ordinary_time.generate_late().unwrap();

        // Find all ordinary time days (excluding solemnities)
        let ordinary_days: Vec<_> = days
            .iter()
            .filter(|d| d.season == Some(Season::OrdinaryTime) && d.rank != Rank::Solemnity)
            .collect();

        // Group by week_of_season
        let mut weeks: std::collections::HashMap<u32, Vec<_>> = std::collections::HashMap::new();
        for day in &ordinary_days {
            weeks
                .entry(day.week_of_season.unwrap())
                .or_insert_with(Vec::new)
                .push(day);
        }

        // Find the last week (highest week number)
        let last_week_num = *weeks.keys().max().unwrap();
        assert_eq!(
            last_week_num, 34,
            "Last week should be 34th week, but found week {}",
            last_week_num
        );

        // Check that the Sunday of the 34th week is Christ the King
        let christ_king = days
            .iter()
            .find(|d| d.id == "our_lord_jesus_christ_king_of_the_universe");
        assert!(christ_king.is_some(), "Christ the King should be present");

        // Verify that Christ the King is indeed on a Sunday of week 34
        let christ_king_day = christ_king.unwrap();
        assert_eq!(
            christ_king_day.week_of_season,
            Some(34),
            "Christ the King should be in week 34"
        );
        assert_eq!(
            christ_king_day.day_of_week,
            crate::types::dates::DayOfWeek(0),
            "Christ the King should be on Sunday"
        );

        // Verify that the 34th week has exactly 7 days total (including Christ the King)
        let all_week_34_days: Vec<_> = days
            .iter()
            .filter(|d| d.week_of_season == Some(34))
            .collect();
        assert_eq!(
            all_week_34_days.len(),
            7,
            "34th week should have 7 days total (including Christ the King), but found {}",
            all_week_34_days.len()
        );

        // Check that we have all days from Sunday to Saturday in the 34th week
        for dow in 0..7 {
            assert!(
                all_week_34_days
                    .iter()
                    .any(|d| d.day_of_week == crate::types::dates::DayOfWeek(dow)),
                "34th week should have day of week {}, but found days with day_of_week: {:?}",
                dow,
                all_week_34_days
                    .iter()
                    .map(|d| d.day_of_week.clone())
                    .collect::<Vec<_>>()
            );
        }
    }
}
