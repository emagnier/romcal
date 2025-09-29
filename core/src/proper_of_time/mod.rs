use chrono::{DateTime, Datelike, Utc};

pub mod advent;
pub mod cache;
pub mod christmas_time;
pub mod common;
pub mod easter_time;
pub mod lent;
pub mod ordinary_time;
pub mod paschal_triduum;

use crate::dates::LiturgicalDates;
use crate::error::RomcalResult;
use crate::liturgical_day::LiturgicalDay;
use crate::preset::Preset;
use crate::proper_of_time::advent::Advent;
use crate::proper_of_time::cache::ProperOfTimeCache;
use crate::proper_of_time::christmas_time::ChristmasTime;
use crate::proper_of_time::common::{
    enum_to_string, sort_liturgical_days_by_date, PROPER_OF_TIME_ID,
};
use crate::proper_of_time::easter_time::EasterTime;
use crate::proper_of_time::lent::Lent;
use crate::proper_of_time::ordinary_time::OrdinaryTime;
use crate::proper_of_time::paschal_triduum::PaschalTriduum;
use crate::types::dates::{DateDef, DateFn, DayOfWeek};
use crate::types::liturgical::{Color, ColorInfo, Precedence, PsalterWeekCycle, Rank, Season};

/// Structure for generating liturgical days of the Proper of Time
pub struct ProperOfTime {
    preset: Preset,
    dates: LiturgicalDates,
    cache: ProperOfTimeCache,
}

impl ProperOfTime {
    /// Creates a new instance of ProperOfTime
    ///
    /// # Arguments
    ///
    /// * `preset` - Calendar preset
    /// * `year` - Liturgical year
    ///
    /// # Errors
    ///
    /// Returns an error if the year is invalid
    pub fn new(preset: Preset, year: i32) -> RomcalResult<Self> {
        use crate::proper_of_time::cache::ProperOfTimeCache;
        let liturgical_dates = LiturgicalDates::new(preset.clone(), year)?;
        let cache = ProperOfTimeCache::new(&preset, year)?;
        Ok(Self {
            preset,
            dates: liturgical_dates,
            cache,
        })
    }

    /// Creates a liturgical day with common properties
    fn create_liturgical_day_base(
        &self,
        id: &str,
        date: DateTime<Utc>,
        precedence: Precedence,
        season: Option<Season>,
        color: Color,
    ) -> LiturgicalDay {
        let id = id.to_string();
        let date_str = date.format("%Y-%m-%d").to_string();
        let dow = date.weekday().num_days_from_sunday() as u8;
        let rank = precedence.to_rank();
        let sunday_cycle = self.cache.sunday_cycle();
        let weekday_cycle = self.cache.weekday_cycle();

        // Calculate season-related fields only if season is provided
        let (day_of_season, week_of_season, psalter_week_cycle) = if let Some(season) = season {
            let start_of_season = match season {
                Season::Advent => self.cache.advent_start(),
                Season::ChristmasTime => self.cache.christmas_start(),
                Season::Lent => self.cache.lent_start(),
                Season::EasterTime => self.cache.easter_start(),
                Season::PaschalTriduum => self.cache.triduum_start(),
                Season::OrdinaryTime => {
                    // For Ordinary Time, we need to determine if it's early or late
                    // This is a simplified approach - in practice, this should be determined by the calling function
                    self.cache.easter_start() // Default to Easter start for now
                }
            };

            // Calculate day_of_season and week_of_season automatically
            let days_since_start = (date.date_naive() - start_of_season.date_naive()).num_days();
            let day_of_season = if days_since_start < 0 {
                0
            } else {
                (days_since_start + 1) as u32
            };

            // Special logic for Lent: if day_of_season < 5, week_of_season starts at 0
            let week_of_season = if season == Season::Lent && day_of_season < 5 {
                0
            } else if day_of_season == 0 {
                // Should never happen if day_of_season is calculated correctly
                1
            } else {
                (day_of_season - 1) / 7 + 1
            };

            (
                Some(day_of_season),
                Some(week_of_season),
                PsalterWeekCycle::from_week(
                    week_of_season,
                    season == Season::Lent,
                    season == Season::ChristmasTime,
                ),
            )
        } else {
            (None, None, PsalterWeekCycle::Week1)
        };

        let mut liturgical_day = LiturgicalDay::new(
            id.clone(),
            id.clone(),
            date_str,
            DateDef::MonthDate {
                month: crate::types::dates::MonthIndex(1), // January
                date: 1,
                day_offset: None,
            },
            precedence,
            rank.clone(),
            enum_to_string(&rank),
            sunday_cycle,
            enum_to_string(&sunday_cycle),
            weekday_cycle,
            enum_to_string(&weekday_cycle),
            psalter_week_cycle,
            enum_to_string(&psalter_week_cycle),
            PROPER_OF_TIME_ID.to_string(),
        )
        .with_day_of_week(DayOfWeek(dow))
        .with_is_holy_day_of_obligation(dow == 0 && rank == Rank::Solemnity);

        // Set season-related fields if season is provided
        if let Some(season) = season {
            liturgical_day = liturgical_day
                .with_seasons(season)
                .with_season_name(enum_to_string(&season))
                .with_start_of_season(self.cache.start_of_seasons(season, date))
                .with_liturgical_year_boundaries(
                    self.cache.liturgical_year_start(season, date),
                    self.cache.liturgical_year_end(season, date),
                );
        }

        // Set season position if calculated
        if let (Some(week), Some(day)) = (week_of_season, day_of_season) {
            liturgical_day = liturgical_day.with_season_position(week, day);
        }

        // Color
        liturgical_day.colors = vec![ColorInfo {
            key: color.clone(),
            name: enum_to_string(&color),
        }];

        // Date definition (placeholder for now)
        liturgical_day.date_def = DateDef::DateFunction {
            date_fn: DateFn::EasterSunday, // TODO: Calculate proper DateFn
            day_offset: None,
        };

        liturgical_day
    }

    /// Generates all liturgical days of the Proper of Time for the liturgical year
    pub fn generate_all(&self) -> RomcalResult<Vec<LiturgicalDay>> {
        let mut days = Vec::new();

        let advent = Advent::new(self);
        let christmas_time = ChristmasTime::new(self);
        let ordinary_time = OrdinaryTime::new(self);
        let lent = Lent::new(self);
        let paschal_triduum = PaschalTriduum::new(self);
        let easter_time = EasterTime::new(self);

        if self.preset.context == crate::CalendarContext::Liturgical {
            days.extend(advent.generate()?);
            days.extend(christmas_time.generate_early()?);
        }

        days.extend(christmas_time.generate_late()?);
        days.extend(ordinary_time.generate_early()?);
        days.extend(lent.generate()?);
        days.extend(paschal_triduum.generate()?);
        days.extend(easter_time.generate()?);
        days.extend(ordinary_time.generate_late()?);

        if self.preset.context == crate::CalendarContext::Gregorian {
            days.extend(advent.generate()?);
            days.extend(christmas_time.generate_early()?);
        }

        // TODO: Temporary fix to sort days by date
        sort_liturgical_days_by_date(&mut days);

        Ok(days)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preset::PresetPartial;

    #[test]
    fn test_proper_of_time_creation() {
        let preset = Preset::default();
        let proper_of_time = ProperOfTime::new(preset, 2026).unwrap();

        assert_eq!(proper_of_time.cache.advent_year(), 2026);
        assert_eq!(proper_of_time.cache.easter_year(), 2026);
    }

    #[test]
    fn test_no_duplicate_dates() {
        let preset = Preset::default();
        let all_days = preset.proper_of_time(2026).unwrap();

        // Check that we have generated days
        assert!(!all_days.is_empty());

        // Extract all dates and check for duplicates
        let mut dates: Vec<&str> = all_days.iter().map(|day| day.date.as_str()).collect();
        let original_count = dates.len();

        // Sort and deduplicate
        dates.sort();
        dates.dedup();
        let unique_count = dates.len();

        // Exception: Holy Thursday has two liturgical days on the same date:
        // - holy_thursday (from lent)
        // - thursday_of_the_lords_supper (from paschal_triduum)
        // So we expect exactly 1 duplicate date
        let expected_duplicates = 1;
        let actual_duplicates = original_count - unique_count;

        assert_eq!(
            actual_duplicates,
            expected_duplicates,
            "Expected {} duplicate date (Holy Thursday), but found {} duplicates. Original: {}, Unique: {}",
            expected_duplicates,
            actual_duplicates,
            original_count,
            unique_count
        );

        // Additional check: verify that only Holy Thursday has duplicate dates
        let mut date_groups: std::collections::HashMap<String, Vec<&LiturgicalDay>> =
            std::collections::HashMap::new();
        for day in &all_days {
            date_groups
                .entry(day.date.clone())
                .or_insert_with(Vec::new)
                .push(day);
        }

        let duplicate_dates: Vec<_> = date_groups
            .iter()
            .filter(|(_, days)| days.len() > 1)
            .collect();

        assert_eq!(
            duplicate_dates.len(),
            1,
            "Expected exactly 1 duplicate date (Holy Thursday), but found {}: {:?}",
            duplicate_dates.len(),
            duplicate_dates
                .iter()
                .map(|(date, days)| (date, days.iter().map(|d| &d.id).collect::<Vec<_>>()))
                .collect::<Vec<_>>()
        );

        // Verify that the duplicate is indeed Holy Thursday
        let holy_thursday_days = duplicate_dates[0].1;
        assert_eq!(holy_thursday_days.len(), 2);
        assert!(holy_thursday_days.iter().any(|d| d.id == "holy_thursday"));
        assert!(holy_thursday_days
            .iter()
            .any(|d| d.id == "thursday_of_the_lords_supper"));
    }

    #[test]
    fn test_no_duplicate_dates_liturgical_context() {
        let preset = Preset::new(PresetPartial {
            context: Some(crate::CalendarContext::Liturgical),
            ..PresetPartial::default()
        });
        let all_days = preset.proper_of_time(2026).unwrap();

        // Check that we have generated days
        assert!(!all_days.is_empty());

        // Extract all dates and check for duplicates
        let mut dates: Vec<&str> = all_days.iter().map(|day| day.date.as_str()).collect();
        let original_count = dates.len();

        // Sort and deduplicate
        dates.sort();
        dates.dedup();
        let unique_count = dates.len();

        // Exception: Holy Thursday has two liturgical days on the same date:
        // - holy_thursday (from lent)
        // - thursday_of_the_lords_supper (from paschal_triduum)
        // So we expect exactly 1 duplicate date
        let expected_duplicates = 1;
        let actual_duplicates = original_count - unique_count;

        assert_eq!(
            actual_duplicates,
            expected_duplicates,
            "Expected {} duplicate date (Holy Thursday), but found {} duplicates (liturgical context). Original: {}, Unique: {}",
            expected_duplicates,
            actual_duplicates,
            original_count,
            unique_count
        );
    }

    #[test]
    fn test_sort_liturgical_days_by_date() {
        let preset = Preset::default();
        let mut all_days = preset.proper_of_time(2026).unwrap();

        // Shuffle the days to test sorting
        all_days.reverse();

        // Sort using the utility function
        sort_liturgical_days_by_date(&mut all_days);

        // Verify that days are sorted by date
        for i in 1..all_days.len() {
            let date_a = chrono::NaiveDate::parse_from_str(&all_days[i - 1].date, "%Y-%m-%d")
                .unwrap_or_default();
            let date_b = chrono::NaiveDate::parse_from_str(&all_days[i].date, "%Y-%m-%d")
                .unwrap_or_default();
            assert!(
                date_a <= date_b,
                "Days are not sorted by date: {} should come before {}",
                all_days[i - 1].date,
                all_days[i].date
            );
        }
    }

    #[test]
    fn test_calendar_continuity() {
        let preset = Preset::default();

        // Get all liturgical days
        let mut days = preset.proper_of_time(2026).unwrap();

        // Sort by date
        sort_liturgical_days_by_date(&mut days);

        // Verify that there are no gaps in dates between first and last day
        for i in 1..days.len() {
            let prev_date =
                chrono::NaiveDate::parse_from_str(&days[i - 1].date, "%Y-%m-%d").unwrap();
            let curr_date = chrono::NaiveDate::parse_from_str(&days[i].date, "%Y-%m-%d").unwrap();

            // Each day should be either:
            // 1. Same date as previous (for duplicates like Holy Thursday)
            // 2. Next day after previous (no gaps)
            let days_diff = (curr_date - prev_date).num_days();
            assert!(
                days_diff == 0 || days_diff == 1,
                "Gap found in calendar: {} to {} ({} days difference). Each day should be same date or next day.",
                prev_date,
                curr_date,
                days_diff
            );
        }

        // Verify that day_of_week matches the actual day of the week for each date
        for day in &days {
            let date = chrono::NaiveDate::parse_from_str(&day.date, "%Y-%m-%d").unwrap();
            let actual_dow = date.weekday().num_days_from_sunday() as u8;
            let stored_dow = day.day_of_week.0;

            assert_eq!(
                actual_dow, stored_dow,
                "day_of_week mismatch for {}: date {} is actually day {} but stored as day {}",
                day.id, day.date, actual_dow, stored_dow
            );
        }

        // TODO: Add week_of_season consistency test
        // This test should verify that week_of_season follows the correct pattern:
        // - Each season starts with week 1 (or 0 for Lent)
        // - Week numbers increment on Sundays
        // - Special handling for Christmas Time and Ordinary Time
        // - Complex logic needed for different seasons
    }
}
