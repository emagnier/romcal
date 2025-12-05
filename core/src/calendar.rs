//! # Calendar Generation Module
//!
//! This module implements the liturgical calendar generation algorithm.
//! It combines the Proper of Time with particular calendars and applies
//! precedence rules according to UNLY #49.

use chrono::{Datelike, Duration, NaiveDate, Weekday};
use std::collections::BTreeMap;

use crate::dates::LiturgicalDates;
use crate::error::{RomcalError, RomcalResult};
use crate::liturgical_day::LiturgicalDay;
use crate::preset::Preset;
use crate::proper_of_time::ProperOfTime;
use crate::types::calendar::{DayDefinition, DayId};
use crate::types::dates::{DateDef, DateDefException, DateDefExceptions, ExceptionCondition};
use crate::types::liturgical::{Precedence, Rank, Season};
use crate::CalendarDefinition;

/// Type alias for the liturgical calendar output
/// Maps date strings (YYYY-MM-DD) to vectors of LiturgicalDay objects
pub type LiturgicalCalendar = BTreeMap<String, Vec<LiturgicalDay>>;

/// Calendar generator that combines Proper of Time with particular calendars
/// and applies precedence rules according to UNLY #49.
pub struct Calendar {
    /// The preset configuration
    preset: Preset,
    /// The liturgical dates calculator
    dates: LiturgicalDates,
    /// The liturgical year (e.g., 2026 for liturgical year 2025-2026)
    year: i32,
    /// Start date of the liturgical year
    start_of_year: NaiveDate,
    /// End date of the liturgical year
    end_of_year: NaiveDate,
}

/// Internal structure to hold built calendar data
struct BuiltData {
    /// Map of day IDs to their LiturgicalDay instances
    by_ids: BTreeMap<String, Vec<LiturgicalDay>>,
    /// Map of date strings to day IDs for that date
    dates_index: BTreeMap<String, Vec<String>>,
}

impl Calendar {
    /// Creates a new Calendar instance for a given year
    ///
    /// # Arguments
    ///
    /// * `preset` - Calendar configuration
    /// * `year` - Liturgical year (e.g., 2026 for liturgical year 2025-2026)
    ///
    /// # Errors
    ///
    /// Returns an error if the year is invalid
    pub fn new(preset: Preset, year: i32) -> RomcalResult<Self> {
        let dates = LiturgicalDates::new(preset.clone(), year)?;

        // Calculate liturgical year boundaries
        // Start: First Sunday of Advent (previous calendar year)
        // End: Saturday before the next First Sunday of Advent
        let start_of_year =
            LiturgicalDates::get_first_sunday_of_advent_date_static(year - 1).date_naive();
        let end_of_year = LiturgicalDates::get_first_sunday_of_advent_date_static(year)
            .date_naive()
            - Duration::days(1);

        Ok(Self {
            preset,
            dates,
            year,
            start_of_year,
            end_of_year,
        })
    }

    /// Generates the complete liturgical calendar
    ///
    /// # Returns
    ///
    /// A BTreeMap of date strings to vectors of LiturgicalDay objects
    ///
    /// # Errors
    ///
    /// Returns an error if calendar generation fails
    pub fn generate(&self) -> RomcalResult<LiturgicalCalendar> {
        // Step 1: Build dates data from Proper of Time and particular calendars
        let built_data = self.build_dates_data()?;

        // Step 2: Apply precedence rules for each date
        let mut calendar = LiturgicalCalendar::new();

        for (date, day_ids) in &built_data.dates_index {
            // Collect all LiturgicalDay objects for this date
            let mut days_for_date: Vec<LiturgicalDay> = day_ids
                .iter()
                .filter_map(|id| built_data.by_ids.get(id))
                .flatten()
                .filter(|day| day.date == *date)
                .cloned()
                .collect();

            // Apply precedence rules
            let processed_days = self.apply_precedence_rules(&mut days_for_date);

            if !processed_days.is_empty() {
                calendar.insert(date.clone(), processed_days);
            }
        }

        Ok(calendar)
    }

    /// Builds dates data from all calendar sources
    fn build_dates_data(&self) -> RomcalResult<BuiltData> {
        let mut by_ids: BTreeMap<String, Vec<LiturgicalDay>> = BTreeMap::new();
        let mut dates_index: BTreeMap<String, Vec<String>> = BTreeMap::new();

        // Step 1: Generate Proper of Time days
        let proper_of_time = ProperOfTime::new(self.preset.clone(), self.year)?;
        let proper_days = proper_of_time.generate_all()?;

        // Index Proper of Time days
        for day in proper_days {
            let date = day.date.clone();
            let id = day.id.clone();

            by_ids.entry(id.clone()).or_default().push(day);
            dates_index.entry(date).or_default().push(id);
        }

        // Step 2: Process particular calendars in hierarchy order
        // Start with the target calendar and traverse up to parent calendars
        let calendar_hierarchy = self.resolve_calendar_hierarchy();

        for calendar_def in calendar_hierarchy {
            self.process_calendar_definition(&calendar_def, &mut by_ids, &mut dates_index)?;
        }

        Ok(BuiltData {
            by_ids,
            dates_index,
        })
    }

    /// Resolves the calendar hierarchy from the target calendar to root
    fn resolve_calendar_hierarchy(&self) -> Vec<CalendarDefinition> {
        let mut hierarchy = Vec::new();
        let mut visited_ids = std::collections::HashSet::new();

        // Start with the target calendar
        if let Some(target) = self.preset.get_calendar_definition(&self.preset.calendar) {
            self.collect_calendar_hierarchy(target, &mut hierarchy, &mut visited_ids);
        }

        // Reverse to process from root to target (inheritance order)
        hierarchy.reverse();
        hierarchy
    }

    /// Recursively collects calendar definitions in hierarchy
    fn collect_calendar_hierarchy(
        &self,
        calendar: &CalendarDefinition,
        hierarchy: &mut Vec<CalendarDefinition>,
        visited: &mut std::collections::HashSet<String>,
    ) {
        if visited.contains(&calendar.id) {
            return;
        }
        visited.insert(calendar.id.clone());

        // Add this calendar
        hierarchy.push(calendar.clone());

        // Process parent calendars
        for parent_id in &calendar.parent_calendar_ids {
            if let Some(parent) = self.preset.get_calendar_definition(parent_id) {
                self.collect_calendar_hierarchy(parent, hierarchy, visited);
            }
        }
    }

    /// Processes a calendar definition and adds its days to the index
    fn process_calendar_definition(
        &self,
        calendar_def: &CalendarDefinition,
        by_ids: &mut BTreeMap<String, Vec<LiturgicalDay>>,
        dates_index: &mut BTreeMap<String, Vec<String>>,
    ) -> RomcalResult<()> {
        for (day_id, day_def) in &calendar_def.days_definitions {
            // Handle drop flag
            if day_def.drop.unwrap_or(false) {
                // Remove this day from all dates
                if let Some(days) = by_ids.remove(day_id) {
                    for day in &days {
                        if let Some(ids) = dates_index.get_mut(&day.date) {
                            ids.retain(|id| id != day_id);
                        }
                    }
                }
                continue;
            }

            // Calculate the date for this day definition
            if let Some(date) = self.build_date(day_def, day_id, 0)? {
                // Check if date is within liturgical year
                if date < self.start_of_year || date > self.end_of_year {
                    continue;
                }

                let date_str = date.format("%Y-%m-%d").to_string();

                // Create or update LiturgicalDay
                // For now, create a minimal LiturgicalDay from the definition
                // In a full implementation, this would merge with existing data
                if let Some(liturgical_day) = self.create_liturgical_day_from_definition(
                    day_id,
                    day_def,
                    &date_str,
                    calendar_def,
                )? {
                    // Update indices
                    by_ids
                        .entry(day_id.clone())
                        .or_default()
                        .push(liturgical_day);
                    dates_index
                        .entry(date_str)
                        .or_default()
                        .push(day_id.clone());
                }
            }
        }

        Ok(())
    }

    /// Builds a date from a DateDef with exception handling
    ///
    /// # Arguments
    ///
    /// * `day_def` - The day definition containing date information
    /// * `day_id` - The ID of the day (for error messages)
    /// * `year_offset` - Offset from the current year (e.g., -1 for previous year)
    ///
    /// # Returns
    ///
    /// The calculated date, or None if the day should not appear this year
    fn build_date(
        &self,
        day_def: &DayDefinition,
        _day_id: &str,
        year_offset: i32,
    ) -> RomcalResult<Option<NaiveDate>> {
        // Get the base date definition
        let date_def = match &day_def.date_def {
            Some(def) => def,
            None => return Ok(None), // No date definition means this day doesn't appear
        };

        // Calculate the base date
        let base_date = match self.calculate_date_from_def(date_def, year_offset)? {
            Some(date) => date,
            None => return Ok(None),
        };

        // Apply exceptions if any
        if let Some(exceptions) = &day_def.date_exceptions {
            return self.apply_date_exceptions(base_date, exceptions, year_offset);
        }

        Ok(Some(base_date))
    }

    /// Calculates a date from a DateDef
    fn calculate_date_from_def(
        &self,
        date_def: &DateDef,
        year_offset: i32,
    ) -> RomcalResult<Option<NaiveDate>> {
        let year = self.year + year_offset;

        match date_def {
            DateDef::MonthDate {
                month,
                date,
                day_offset,
            } => {
                let naive_date = NaiveDate::from_ymd_opt(year, month.0 as u32, *date as u32)
                    .ok_or_else(|| {
                        RomcalError::ValidationError(format!(
                            "Invalid date: {}-{:02}-{:02}",
                            year, month.0, date
                        ))
                    })?;

                let final_date = if let Some(offset) = day_offset {
                    naive_date + Duration::days(*offset as i64)
                } else {
                    naive_date
                };

                Ok(Some(final_date))
            }

            DateDef::DateFunction {
                date_fn,
                day_offset,
            } => {
                use crate::types::dates::DateFn;

                let base_date = match date_fn {
                    DateFn::EasterSunday => self.dates.get_easter_sunday_date(Some(year))?,
                    DateFn::EpiphanySunday => self.dates.get_epiphany_date(Some(year)),
                    DateFn::PresentationOfTheLord => {
                        self.dates.get_presentation_of_the_lord_date(Some(year))
                    }
                    DateFn::Annunciation => self.dates.get_annunciation_date(Some(year)),
                    DateFn::PalmSunday => self.dates.get_palm_sunday_date(Some(year)),
                    DateFn::DivineMercySunday => {
                        self.dates.get_divine_mercy_sunday_date(Some(year))
                    }
                    DateFn::MaryMotherOfTheChurch => {
                        self.dates.get_mary_mother_of_the_church_date(Some(year))
                    }
                    DateFn::ImmaculateHeartOfMary => {
                        self.dates.get_immaculate_heart_of_mary_date(Some(year))
                    }
                    DateFn::PentecostSunday => self.dates.get_pentecost_sunday_date(Some(year)),
                    DateFn::CorpusChristiSunday => self.dates.get_corpus_christi_date(Some(year)),
                    DateFn::NativityOfJohnTheBaptist => {
                        self.dates.get_nativity_of_john_the_baptist_date(Some(year))
                    }
                    DateFn::PeterAndPaulApostles => {
                        self.dates.get_peter_and_paul_apostles_date(Some(year))
                    }
                    DateFn::Transfiguration => self.dates.get_transfiguration_date(Some(year)),
                    DateFn::Assumption => self.dates.get_assumption_date(Some(year)),
                    DateFn::ExaltationOfTheHolyCross => {
                        self.dates.get_exaltation_of_the_holy_cross_date(Some(year))
                    }
                    DateFn::AllSaints => self.dates.get_all_saints_date(Some(year)),
                    DateFn::ImmaculateConceptionOfMary => self
                        .dates
                        .get_immaculate_conception_of_mary_date(Some(year)),
                };

                let final_date = if let Some(offset) = day_offset {
                    LiturgicalDates::add_days(base_date, *offset as i64)
                } else {
                    base_date
                };

                Ok(Some(final_date.date_naive()))
            }

            DateDef::WeekdayOfMonth {
                month,
                day_of_week,
                nth_week_in_month,
                day_offset,
            } => {
                // Find the nth occurrence of day_of_week in the given month
                let first_of_month =
                    NaiveDate::from_ymd_opt(year, month.0 as u32, 1).ok_or_else(|| {
                        RomcalError::ValidationError(format!("Invalid month: {}", month.0))
                    })?;

                let target_weekday = Self::day_of_week_to_weekday(day_of_week.0);
                let first_dow = first_of_month.weekday();
                let days_until_target = (target_weekday.num_days_from_sunday() as i64
                    - first_dow.num_days_from_sunday() as i64
                    + 7)
                    % 7;

                let first_occurrence = first_of_month + Duration::days(days_until_target);
                let nth_occurrence =
                    first_occurrence + Duration::weeks((*nth_week_in_month - 1) as i64);

                // Verify it's still in the same month
                if nth_occurrence.month() != month.0 as u32 {
                    return Ok(None);
                }

                let final_date = if let Some(offset) = day_offset {
                    nth_occurrence + Duration::days(*offset as i64)
                } else {
                    nth_occurrence
                };

                Ok(Some(final_date))
            }

            DateDef::LastWeekdayOfMonth {
                month,
                last_day_of_week_in_month,
                day_offset,
            } => {
                // Find the last occurrence of day_of_week in the given month
                let last_of_month = Self::last_day_of_month(year, month.0 as u32)?;
                let target_weekday = Self::day_of_week_to_weekday(last_day_of_week_in_month.0);
                let last_dow = last_of_month.weekday();

                let days_back = (last_dow.num_days_from_sunday() as i64
                    - target_weekday.num_days_from_sunday() as i64
                    + 7)
                    % 7;

                let last_occurrence = last_of_month - Duration::days(days_back);

                let final_date = if let Some(offset) = day_offset {
                    last_occurrence + Duration::days(*offset as i64)
                } else {
                    last_occurrence
                };

                Ok(Some(final_date))
            }
        }
    }

    /// Applies date exceptions to modify a base date
    fn apply_date_exceptions(
        &self,
        base_date: NaiveDate,
        exceptions: &DateDefExceptions,
        year_offset: i32,
    ) -> RomcalResult<Option<NaiveDate>> {
        let exception_list = match exceptions {
            DateDefExceptions::Single(e) => vec![e.clone()],
            DateDefExceptions::Multiple(list) => list.clone(),
        };

        for exception in exception_list {
            if self.check_exception_condition(&base_date, &exception.when, year_offset)? {
                // Apply the exception
                return self.apply_exception_result(base_date, &exception, year_offset);
            }
        }

        Ok(Some(base_date))
    }

    /// Checks if an exception condition is met
    fn check_exception_condition(
        &self,
        date: &NaiveDate,
        condition: &ExceptionCondition,
        year_offset: i32,
    ) -> RomcalResult<bool> {
        match condition {
            ExceptionCondition::IsBetween {
                from,
                to,
                inclusive,
            } => {
                let from_date = self
                    .calculate_date_from_def(from, year_offset)?
                    .ok_or_else(|| {
                        RomcalError::ValidationError("Cannot calculate 'from' date".to_string())
                    })?;
                let to_date = self
                    .calculate_date_from_def(to, year_offset)?
                    .ok_or_else(|| {
                        RomcalError::ValidationError("Cannot calculate 'to' date".to_string())
                    })?;

                if *inclusive {
                    Ok(*date >= from_date && *date <= to_date)
                } else {
                    Ok(*date > from_date && *date < to_date)
                }
            }

            ExceptionCondition::IsSameAsDate { date: target_date } => {
                let target = self
                    .calculate_date_from_def(target_date, year_offset)?
                    .ok_or_else(|| {
                        RomcalError::ValidationError("Cannot calculate target date".to_string())
                    })?;
                Ok(*date == target)
            }

            ExceptionCondition::IsDayOfWeek { day_of_week } => {
                let target_weekday = Self::day_of_week_to_weekday(day_of_week.0);
                Ok(date.weekday() == target_weekday)
            }
        }
    }

    /// Applies an exception result to get the new date
    fn apply_exception_result(
        &self,
        base_date: NaiveDate,
        exception: &DateDefException,
        year_offset: i32,
    ) -> RomcalResult<Option<NaiveDate>> {
        use crate::types::dates::DateDefExtended;

        match &exception.then {
            DateDefExtended::DateDef(date_def) => {
                self.calculate_date_from_def(date_def, year_offset)
            }
            DateDefExtended::WithOffset(offset) => {
                Ok(Some(base_date + Duration::days(offset.day_offset as i64)))
            }
        }
    }

    /// Creates a LiturgicalDay from a DayDefinition
    fn create_liturgical_day_from_definition(
        &self,
        day_id: &DayId,
        day_def: &DayDefinition,
        date_str: &str,
        calendar_def: &CalendarDefinition,
    ) -> RomcalResult<Option<LiturgicalDay>> {
        // Get precedence or default to Weekday
        let precedence = day_def.precedence.clone().unwrap_or(Precedence::Weekday_13);
        let rank = precedence.to_rank();

        // Parse the date to get day of week
        let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|e| {
            RomcalError::ValidationError(format!("Cannot parse date {}: {}", date_str, e))
        })?;
        let dow = date.weekday().num_days_from_sunday() as u8;

        // Get cycles from cache (we'd need to access proper_of_time cache here)
        // For now, use defaults - in full implementation, this would be calculated
        use crate::proper_of_time::common::enum_to_string;
        use crate::types::liturgical::{PsalterWeekCycle, SundayCycle, WeekdayCycle};

        let sunday_cycle = SundayCycle::from_year(self.year);
        let weekday_cycle = WeekdayCycle::from_year(self.year);
        let psalter_week = PsalterWeekCycle::Week1; // Simplified

        let mut liturgical_day = LiturgicalDay::new(
            day_id.clone(),
            day_id.clone(), // fullname - would be localized in full implementation
            date_str.to_string(),
            day_def.date_def.clone().unwrap_or(DateDef::MonthDate {
                month: crate::types::dates::MonthIndex(1),
                date: 1,
                day_offset: None,
            }),
            precedence,
            rank.clone(),
            enum_to_string(&rank),
            sunday_cycle,
            enum_to_string(&sunday_cycle),
            weekday_cycle,
            enum_to_string(&weekday_cycle),
            psalter_week,
            enum_to_string(&psalter_week),
            calendar_def.id.clone(),
        )
        .with_day_of_week(crate::types::dates::DayOfWeek(dow))
        .with_is_holy_day_of_obligation(day_def.is_holy_day_of_obligation.unwrap_or(false))
        .with_is_optional(day_def.is_optional.unwrap_or(false))
        .with_allow_similar_rank_items(day_def.allow_similar_rank_items.unwrap_or(false));

        // Add date exceptions
        if let Some(exceptions) = &day_def.date_exceptions {
            liturgical_day.date_exceptions = match exceptions {
                DateDefExceptions::Single(e) => vec![e.clone()],
                DateDefExceptions::Multiple(list) => list.clone(),
            };
        }

        Ok(Some(liturgical_day))
    }

    /// Applies precedence rules according to UNLY #49
    ///
    /// This function sorts and filters liturgical days for a single date
    /// based on their precedence, handling special cases like:
    /// - Holy Thursday (two liturgical days on the same date)
    /// - Optional memorials
    /// - allowSimilarRankItems flag
    fn apply_precedence_rules(&self, days: &mut Vec<LiturgicalDay>) -> Vec<LiturgicalDay> {
        if days.is_empty() {
            return Vec::new();
        }

        if days.len() == 1 {
            return days.clone();
        }

        // Sort by precedence (lower enum variant = higher precedence)
        days.sort_by(|a, b| self.compare_precedence(a, b));

        // Get the highest precedence day
        let highest = &days[0];

        // Check for special cases
        // Holy Thursday: both "holy_thursday" and "thursday_of_the_lords_supper" appear
        let is_holy_thursday = days.iter().any(|d| d.id == "holy_thursday")
            && days.iter().any(|d| d.id == "thursday_of_the_lords_supper");

        if is_holy_thursday {
            // Return both days for Holy Thursday
            return days
                .iter()
                .filter(|d| d.id == "holy_thursday" || d.id == "thursday_of_the_lords_supper")
                .cloned()
                .collect();
        }

        // Handle allowSimilarRankItems
        let mut result = vec![highest.clone()];

        if highest.allow_similar_rank_items {
            // Include other days with the same rank
            for day in days.iter().skip(1) {
                if day.rank == highest.rank {
                    result.push(day.clone());
                }
            }
        }

        // Handle optional memorials
        // Optional memorials can be added after the main celebration on certain days
        let can_have_optional_memorials = self.can_have_optional_memorials(highest);

        if can_have_optional_memorials {
            for day in days.iter().skip(1) {
                if day.is_optional || day.rank == Rank::OptionalMemorial {
                    if !result.iter().any(|d| d.id == day.id) {
                        result.push(day.clone());
                    }
                }
            }
        }

        // During Lent, obligatory memorials become optional (UNLY #14)
        if let Some(Season::Lent) = highest.season {
            for day in days.iter().skip(1) {
                if day.rank == Rank::Memorial {
                    let mut optional_day = day.clone();
                    optional_day.is_optional = true;
                    if !result.iter().any(|d| d.id == optional_day.id) {
                        result.push(optional_day);
                    }
                }
            }
        }

        result
    }

    /// Compares two LiturgicalDay objects by precedence
    fn compare_precedence(&self, a: &LiturgicalDay, b: &LiturgicalDay) -> std::cmp::Ordering {
        use strum::IntoEnumIterator;

        // Get the position of each precedence in the enum order
        let precedences: Vec<Precedence> = Precedence::iter().collect();

        let pos_a = precedences
            .iter()
            .position(|p| *p == a.precedence)
            .unwrap_or(usize::MAX);
        let pos_b = precedences
            .iter()
            .position(|p| *p == b.precedence)
            .unwrap_or(usize::MAX);

        // Lower position = higher precedence
        let cmp = pos_a.cmp(&pos_b);

        if cmp == std::cmp::Ordering::Equal {
            // If same precedence, non-optional comes before optional
            match (a.is_optional, b.is_optional) {
                (false, true) => std::cmp::Ordering::Less,
                (true, false) => std::cmp::Ordering::Greater,
                _ => std::cmp::Ordering::Equal,
            }
        } else {
            cmp
        }
    }

    /// Determines if a day can have optional memorials added to it
    ///
    /// According to UNLY #14 and GIRM #355:
    /// - On privileged weekdays (UNLY #59 9)
    /// - On ferias (UNLY #59 13)
    fn can_have_optional_memorials(&self, day: &LiturgicalDay) -> bool {
        matches!(
            day.precedence,
            Precedence::PrivilegedWeekday_9 | Precedence::Weekday_13
        )
    }

    // ==================== Helper functions ====================

    /// Converts a DayOfWeek (0-6) to chrono::Weekday
    fn day_of_week_to_weekday(dow: u8) -> Weekday {
        match dow % 7 {
            0 => Weekday::Sun,
            1 => Weekday::Mon,
            2 => Weekday::Tue,
            3 => Weekday::Wed,
            4 => Weekday::Thu,
            5 => Weekday::Fri,
            6 => Weekday::Sat,
            _ => unreachable!(),
        }
    }

    /// Gets the last day of a month
    fn last_day_of_month(year: i32, month: u32) -> RomcalResult<NaiveDate> {
        let next_month = if month == 12 { 1 } else { month + 1 };
        let next_year = if month == 12 { year + 1 } else { year };

        let first_of_next = NaiveDate::from_ymd_opt(next_year, next_month, 1).ok_or_else(|| {
            RomcalError::ValidationError(format!("Invalid month: {}/{}", year, month))
        })?;

        Ok(first_of_next - Duration::days(1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calendar_creation() {
        let preset = Preset::default();
        let calendar = Calendar::new(preset, 2026).unwrap();

        assert_eq!(calendar.year, 2026);
        // Liturgical year 2026 starts on November 30, 2025 (First Sunday of Advent)
        assert_eq!(calendar.start_of_year.month(), 11);
        assert_eq!(calendar.start_of_year.year(), 2025);
    }

    #[test]
    fn test_day_of_week_conversion() {
        assert_eq!(Calendar::day_of_week_to_weekday(0), Weekday::Sun);
        assert_eq!(Calendar::day_of_week_to_weekday(1), Weekday::Mon);
        assert_eq!(Calendar::day_of_week_to_weekday(6), Weekday::Sat);
    }

    #[test]
    fn test_last_day_of_month() {
        assert_eq!(
            Calendar::last_day_of_month(2024, 2).unwrap(),
            NaiveDate::from_ymd_opt(2024, 2, 29).unwrap() // Leap year
        );
        assert_eq!(
            Calendar::last_day_of_month(2025, 2).unwrap(),
            NaiveDate::from_ymd_opt(2025, 2, 28).unwrap() // Non-leap year
        );
        assert_eq!(
            Calendar::last_day_of_month(2024, 12).unwrap(),
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()
        );
    }

    #[test]
    fn test_precedence_comparison() {
        let preset = Preset::default();
        let calendar = Calendar::new(preset, 2026).unwrap();

        // Create mock days with different precedences
        use crate::types::liturgical::{PsalterWeekCycle, SundayCycle, WeekdayCycle};

        let triduum_day = LiturgicalDay::new(
            "triduum".to_string(),
            "Triduum".to_string(),
            "2026-04-09".to_string(),
            DateDef::MonthDate {
                month: crate::types::dates::MonthIndex(4),
                date: 9,
                day_offset: None,
            },
            Precedence::Triduum_1,
            Rank::Weekday,
            "Weekday".to_string(),
            SundayCycle::YearA,
            "Year A".to_string(),
            WeekdayCycle::Year1,
            "Year I".to_string(),
            PsalterWeekCycle::Week1,
            "Week 1".to_string(),
            "test".to_string(),
        );

        let weekday = LiturgicalDay::new(
            "weekday".to_string(),
            "Weekday".to_string(),
            "2026-04-09".to_string(),
            DateDef::MonthDate {
                month: crate::types::dates::MonthIndex(4),
                date: 9,
                day_offset: None,
            },
            Precedence::Weekday_13,
            Rank::Weekday,
            "Weekday".to_string(),
            SundayCycle::YearA,
            "Year A".to_string(),
            WeekdayCycle::Year1,
            "Year I".to_string(),
            PsalterWeekCycle::Week1,
            "Week 1".to_string(),
            "test".to_string(),
        );

        // Triduum should have higher precedence (less) than weekday
        assert_eq!(
            calendar.compare_precedence(&triduum_day, &weekday),
            std::cmp::Ordering::Less
        );
    }

    #[test]
    fn test_calculate_month_date() {
        let preset = Preset::default();
        let calendar = Calendar::new(preset, 2026).unwrap();

        // Test simple month/date
        let date_def = DateDef::MonthDate {
            month: crate::types::dates::MonthIndex(12),
            date: 25,
            day_offset: None,
        };
        let result = calendar.calculate_date_from_def(&date_def, 0).unwrap();
        assert_eq!(result, Some(NaiveDate::from_ymd_opt(2026, 12, 25).unwrap()));

        // Test with day offset
        let date_def_with_offset = DateDef::MonthDate {
            month: crate::types::dates::MonthIndex(12),
            date: 25,
            day_offset: Some(-1),
        };
        let result_offset = calendar
            .calculate_date_from_def(&date_def_with_offset, 0)
            .unwrap();
        assert_eq!(
            result_offset,
            Some(NaiveDate::from_ymd_opt(2026, 12, 24).unwrap())
        );
    }

    #[test]
    fn test_calculate_date_function() {
        let preset = Preset::default();
        let calendar = Calendar::new(preset, 2026).unwrap();

        // Test Easter Sunday
        use crate::types::dates::DateFn;
        let date_def = DateDef::DateFunction {
            date_fn: DateFn::EasterSunday,
            day_offset: None,
        };
        let result = calendar.calculate_date_from_def(&date_def, 0).unwrap();
        // Easter 2026 is April 5
        assert_eq!(result, Some(NaiveDate::from_ymd_opt(2026, 4, 5).unwrap()));

        // Test with offset (Pentecost = Easter + 49 days)
        let pentecost_def = DateDef::DateFunction {
            date_fn: DateFn::EasterSunday,
            day_offset: Some(49),
        };
        let pentecost = calendar.calculate_date_from_def(&pentecost_def, 0).unwrap();
        assert_eq!(
            pentecost,
            Some(NaiveDate::from_ymd_opt(2026, 5, 24).unwrap())
        );
    }

    #[test]
    fn test_calculate_weekday_of_month() {
        let preset = Preset::default();
        let calendar = Calendar::new(preset, 2026).unwrap();

        // Test 3rd Sunday of September (e.g., for Catechetical Sunday)
        let date_def = DateDef::WeekdayOfMonth {
            month: crate::types::dates::MonthIndex(9),
            day_of_week: crate::types::dates::DayOfWeek(0), // Sunday
            nth_week_in_month: 3,
            day_offset: None,
        };
        let result = calendar.calculate_date_from_def(&date_def, 0).unwrap();
        // 3rd Sunday of September 2026 is September 20
        let expected_date = result.unwrap();
        assert_eq!(expected_date.month(), 9);
        assert_eq!(expected_date.weekday(), Weekday::Sun);
    }

    #[test]
    fn test_calculate_last_weekday_of_month() {
        let preset = Preset::default();
        let calendar = Calendar::new(preset, 2026).unwrap();

        // Test last Sunday of November (Christ the King region)
        let date_def = DateDef::LastWeekdayOfMonth {
            month: crate::types::dates::MonthIndex(11),
            last_day_of_week_in_month: crate::types::dates::DayOfWeek(0), // Sunday
            day_offset: None,
        };
        let result = calendar.calculate_date_from_def(&date_def, 0).unwrap();
        let expected_date = result.unwrap();
        assert_eq!(expected_date.month(), 11);
        assert_eq!(expected_date.weekday(), Weekday::Sun);
        // Last Sunday of November 2026 is November 29
        assert_eq!(expected_date.day(), 29);
    }

    #[test]
    fn test_generate_calendar_basic() {
        let preset = Preset::default();
        let calendar = Calendar::new(preset, 2026).unwrap();

        let result = calendar.generate();
        assert!(result.is_ok());

        let liturgical_calendar = result.unwrap();

        // Should have entries for each day of the liturgical year
        // A liturgical year typically has 365-366 days
        assert!(
            !liturgical_calendar.is_empty(),
            "Calendar should not be empty"
        );

        // Verify we have a reasonable number of dates
        // At minimum, Proper of Time should generate ~365 days
        assert!(
            liturgical_calendar.len() >= 350,
            "Should have at least 350 dates, got {}",
            liturgical_calendar.len()
        );

        // Easter 2026 should be present (Easter 2026 is April 5)
        assert!(
            liturgical_calendar.contains_key("2026-04-05"),
            "Easter 2026 should be present"
        );

        // Pentecost 2026 should be present (49 days after Easter = May 24)
        assert!(
            liturgical_calendar.contains_key("2026-05-24"),
            "Pentecost 2026 should be present"
        );

        // Check that days have proper structure
        let easter_days = liturgical_calendar.get("2026-04-05").unwrap();
        assert!(
            !easter_days.is_empty(),
            "Easter should have at least one entry"
        );

        let easter = &easter_days[0];
        assert_eq!(
            easter.precedence,
            Precedence::Triduum_1,
            "Easter should have Triduum precedence"
        );
    }

    #[test]
    fn test_precedence_order_all_levels() {
        // Test that all precedence levels are in correct order
        use strum::IntoEnumIterator;

        let precedences: Vec<Precedence> = Precedence::iter().collect();

        // Verify first few are highest precedence
        assert_eq!(precedences[0], Precedence::Triduum_1);
        assert_eq!(precedences[1], Precedence::ProperOfTimeSolemnity_2);
        assert_eq!(precedences[2], Precedence::PrivilegedSunday_2);

        // Verify last is lowest precedence
        assert_eq!(precedences[precedences.len() - 1], Precedence::Weekday_13);
    }

    #[test]
    fn test_apply_precedence_rules_single_day() {
        let preset = Preset::default();
        let calendar = Calendar::new(preset, 2026).unwrap();

        use crate::types::liturgical::{PsalterWeekCycle, SundayCycle, WeekdayCycle};

        let mut days = vec![LiturgicalDay::new(
            "single_day".to_string(),
            "Single Day".to_string(),
            "2026-01-01".to_string(),
            DateDef::MonthDate {
                month: crate::types::dates::MonthIndex(1),
                date: 1,
                day_offset: None,
            },
            Precedence::GeneralSolemnity_3,
            Rank::Solemnity,
            "Solemnity".to_string(),
            SundayCycle::YearA,
            "Year A".to_string(),
            WeekdayCycle::Year1,
            "Year I".to_string(),
            PsalterWeekCycle::Week1,
            "Week 1".to_string(),
            "test".to_string(),
        )];

        let result = calendar.apply_precedence_rules(&mut days);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, "single_day");
    }

    #[test]
    fn test_apply_precedence_rules_multiple_days() {
        let preset = Preset::default();
        let calendar = Calendar::new(preset, 2026).unwrap();

        use crate::types::liturgical::{PsalterWeekCycle, SundayCycle, WeekdayCycle};

        let solemnity = LiturgicalDay::new(
            "solemnity".to_string(),
            "Solemnity".to_string(),
            "2026-01-01".to_string(),
            DateDef::MonthDate {
                month: crate::types::dates::MonthIndex(1),
                date: 1,
                day_offset: None,
            },
            Precedence::GeneralSolemnity_3,
            Rank::Solemnity,
            "Solemnity".to_string(),
            SundayCycle::YearA,
            "Year A".to_string(),
            WeekdayCycle::Year1,
            "Year I".to_string(),
            PsalterWeekCycle::Week1,
            "Week 1".to_string(),
            "test".to_string(),
        );

        let memorial = LiturgicalDay::new(
            "memorial".to_string(),
            "Memorial".to_string(),
            "2026-01-01".to_string(),
            DateDef::MonthDate {
                month: crate::types::dates::MonthIndex(1),
                date: 1,
                day_offset: None,
            },
            Precedence::GeneralMemorial_10,
            Rank::Memorial,
            "Memorial".to_string(),
            SundayCycle::YearA,
            "Year A".to_string(),
            WeekdayCycle::Year1,
            "Year I".to_string(),
            PsalterWeekCycle::Week1,
            "Week 1".to_string(),
            "test".to_string(),
        );

        let mut days = vec![memorial.clone(), solemnity.clone()];
        let result = calendar.apply_precedence_rules(&mut days);

        // Solemnity should win over Memorial
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].id, "solemnity");
    }
}
