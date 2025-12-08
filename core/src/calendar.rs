//! # Calendar Generation Module
//!
//! This module implements the liturgical calendar generation algorithm.
//! It combines the Proper of Time with particular calendars and applies
//! precedence rules according to UNLY #49.

use chrono::{Datelike, Duration, NaiveDate, Weekday};
use std::collections::{BTreeMap, HashMap, HashSet};

use crate::dates::LiturgicalDates;
use crate::error::{RomcalError, RomcalResult};
use crate::liturgical_day::{LiturgicalDay, ParentOverride};
use crate::preset::Preset;
use crate::proper_of_time::common::PROPER_OF_TIME_ID;
use crate::proper_of_time::ProperOfTime;
use crate::types::calendar::{DayDefinition, DayId};
use crate::types::dates::{DateDef, DateDefException, DateDefExceptions, ExceptionCondition};
use crate::types::liturgical::{Color, ColorInfo, Precedence, Rank, Season};
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
    /// Calendar hierarchy ordered from most general to most specific
    calendar_hierarchy: Vec<CalendarDefinition>,
    /// Mapping calendar_id -> priority (0 = general_roman, higher = more specific)
    calendar_priority: HashMap<String, usize>,
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

        let (calendar_hierarchy, calendar_priority) = Self::resolve_calendar_hierarchy(&preset);

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
            calendar_hierarchy,
            calendar_priority,
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
        for calendar_def in &self.calendar_hierarchy {
            self.process_calendar_definition(calendar_def, &mut by_ids, &mut dates_index)?;
        }

        Ok(BuiltData {
            by_ids,
            dates_index,
        })
    }

    /// Resolves the calendar hierarchy from root to target (general to specific)
    fn resolve_calendar_hierarchy(
        preset: &Preset,
    ) -> (Vec<CalendarDefinition>, HashMap<String, usize>) {
        let mut hierarchy = Vec::new();
        let mut visited_ids = HashSet::new();

        // Always start with general_roman as the base calendar (most general)
        // It should be processed first, even if not explicitly in parent chain
        if let Some(general_roman) = preset.get_calendar_definition("general_roman") {
            if !visited_ids.contains("general_roman") {
                hierarchy.push(general_roman.clone());
                visited_ids.insert("general_roman".to_string());
            }
        }

        // Then process the target calendar and its parent chain
        if let Some(target) = preset.get_calendar_definition(&preset.calendar) {
            Self::collect_calendar_hierarchy(preset, target, &mut hierarchy, &mut visited_ids);
        }

        // Post-order DFS produces the correct order (general → specific), no reverse needed

        let mut calendar_priority = HashMap::new();
        for (idx, calendar) in hierarchy.iter().enumerate() {
            calendar_priority.entry(calendar.id.clone()).or_insert(idx);
        }

        (hierarchy, calendar_priority)
    }

    /// Recursively collects calendar definitions in hierarchy
    fn collect_calendar_hierarchy(
        preset: &Preset,
        calendar: &CalendarDefinition,
        hierarchy: &mut Vec<CalendarDefinition>,
        visited: &mut HashSet<String>,
    ) {
        if visited.contains(&calendar.id) {
            return;
        }
        visited.insert(calendar.id.clone());

        // Process parent calendars FIRST (post-order DFS)
        for parent_id in &calendar.parent_calendar_ids {
            if let Some(parent) = preset.get_calendar_definition(parent_id) {
                Self::collect_calendar_hierarchy(preset, parent, hierarchy, visited);
            }
        }

        // Add this calendar AFTER processing all parents
        hierarchy.push(calendar.clone());
    }

    /// Processes a calendar definition and adds its days to the index
    fn process_calendar_definition(
        &self,
        calendar_def: &CalendarDefinition,
        by_ids: &mut BTreeMap<String, Vec<LiturgicalDay>>,
        dates_index: &mut BTreeMap<String, Vec<String>>,
    ) -> RomcalResult<()> {
        for (day_id, day_def) in &calendar_def.days_definitions {
            // Handle drop flag with validations
            if day_def.drop.unwrap_or(false) {
                // Validation 1: Verify the element exists before dropping
                if !by_ids.contains_key(day_id) {
                    return Err(RomcalError::ValidationError(format!(
                        "In the '{}' calendar, trying to drop a LiturgicalDay that doesn't exist: '{}'.",
                        calendar_def.id, day_id
                    )));
                }

                // Validation 2: Prevent dropping elements from Proper of Time
                if let Some(existing_days) = by_ids.get(day_id) {
                    if existing_days
                        .iter()
                        .any(|d| d.from_calendar_id == PROPER_OF_TIME_ID)
                    {
                        return Err(RomcalError::ValidationError(format!(
                            "In the '{}' calendar, you can't drop a LiturgicalDay from the Proper of Time: '{}'.",
                            calendar_def.id, day_id
                        )));
                    }
                }

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

            // Check if a day with the same ID already exists (for inheritance)
            let existing_day = by_ids.get(day_id).and_then(|days| days.first());

            // Build effective DayDefinition with inherited properties for date calculation
            // This ensures date_def is inherited before build_date is called
            let mut effective_day_def = day_def.clone();
            if effective_day_def.date_def.is_none() {
                if let Some(existing) = existing_day {
                    effective_day_def.date_def = Some(existing.date_def.clone());
                }
            }
            // Also inherit date_exceptions if not defined
            if effective_day_def.date_exceptions.is_none() {
                if let Some(existing) = existing_day {
                    if !existing.date_exceptions.is_empty() {
                        use crate::types::dates::DateDefExceptions;
                        effective_day_def.date_exceptions =
                            Some(if existing.date_exceptions.len() == 1 {
                                DateDefExceptions::Single(existing.date_exceptions[0].clone())
                            } else {
                                DateDefExceptions::Multiple(existing.date_exceptions.clone())
                            });
                    }
                }
            }

            // Calculate the date for this day definition using effective_day_def
            if let Some(date) = self.build_date(&effective_day_def, day_id, 0)? {
                // Check if date is within liturgical year
                if date < self.start_of_year || date > self.end_of_year {
                    continue;
                }

                let date_str = date.format("%Y-%m-%d").to_string();

                // Create or update LiturgicalDay with inherited properties
                // Use original day_def (not effective_day_def) so explicit values take precedence
                if let Some(mut liturgical_day) = self.create_liturgical_day_from_definition(
                    day_id,
                    day_def,
                    &date_str,
                    calendar_def,
                    by_ids,
                    dates_index,
                )? {
                    // Inherit properties from Proper of Time if this is not from Proper of Time
                    if calendar_def.id != PROPER_OF_TIME_ID {
                        self.inherit_proper_of_time_properties(
                            &mut liturgical_day,
                            &date_str,
                            day_def,
                            by_ids,
                            dates_index,
                        );
                    }

                    // Update indices - REPLACE existing day with same ID instead of adding
                    if let Some(old_days) = by_ids.get(day_id) {
                        // Remove old day from dates_index for its old date(s)
                        let old_dates: Vec<String> =
                            old_days.iter().map(|d| d.date.clone()).collect();
                        for old_date in old_dates {
                            if let Some(ids) = dates_index.get_mut(&old_date) {
                                ids.retain(|id| id != day_id);
                            }
                        }
                        // Replace in by_ids
                        by_ids.insert(day_id.clone(), vec![liturgical_day]);
                    } else {
                        // New day, just add it
                        by_ids
                            .entry(day_id.clone())
                            .or_default()
                            .push(liturgical_day);
                    }
                    // Add to new date in dates_index
                    dates_index
                        .entry(date_str)
                        .or_default()
                        .push(day_id.clone());
                }
            }
        }

        Ok(())
    }

    /// Gets the Proper of Time LiturgicalDay for a given date
    fn get_proper_of_time_day_for_date<'a>(
        &self,
        date_str: &str,
        by_ids: &'a BTreeMap<String, Vec<LiturgicalDay>>,
        dates_index: &BTreeMap<String, Vec<String>>,
    ) -> Option<&'a LiturgicalDay> {
        // Get all day IDs for this date
        if let Some(day_ids) = dates_index.get(date_str) {
            // Find the first day from Proper of Time
            for day_id in day_ids {
                if let Some(days) = by_ids.get(day_id) {
                    if let Some(proper_day) = days
                        .iter()
                        .find(|d| d.from_calendar_id == PROPER_OF_TIME_ID && d.date == date_str)
                    {
                        return Some(proper_day);
                    }
                }
            }
        }
        None
    }

    /// Inherits properties from the Proper of Time day to a particular calendar day
    fn inherit_proper_of_time_properties(
        &self,
        liturgical_day: &mut LiturgicalDay,
        date_str: &str,
        day_def: &DayDefinition,
        by_ids: &BTreeMap<String, Vec<LiturgicalDay>>,
        dates_index: &BTreeMap<String, Vec<String>>,
    ) {
        // Get the Proper of Time day for this date
        let proper_day = match self.get_proper_of_time_day_for_date(date_str, by_ids, dates_index) {
            Some(day) => day,
            None => return, // No Proper of Time day found for this date
        };

        // Copy season if not defined
        if liturgical_day.season.is_none() {
            liturgical_day.season = proper_day.season;
        }

        // Copy season name if not defined
        if liturgical_day.season_name.is_none() {
            liturgical_day.season_name = proper_day.season_name.clone();
        }

        // Copy periods if empty
        if liturgical_day.periods.is_empty() {
            liturgical_day.periods = proper_day.periods.clone();
        }

        // Handle colors: explicit calendar color > martyr rule > proper of time color
        if liturgical_day.colors.is_empty() {
            if day_def.colors.is_some() {
                // Use explicit color from calendar definition
                // This is already set in create_liturgical_day_from_definition
            } else if liturgical_day.titles.contains_martyr() {
                // Martyrs get red color
                use crate::proper_of_time::common::enum_to_string;
                liturgical_day.colors = vec![ColorInfo {
                    key: Color::Red,
                    name: enum_to_string(&Color::Red),
                }];
            } else {
                // Copy colors from Proper of Time
                liturgical_day.colors = proper_day.colors.clone();
            }
        }

        // Copy week_of_season if not defined
        if liturgical_day.week_of_season.is_none() {
            liturgical_day.week_of_season = proper_day.week_of_season;
        }

        // Copy day_of_season if not defined
        if liturgical_day.day_of_season.is_none() {
            liturgical_day.day_of_season = proper_day.day_of_season;
        }

        // day_of_week is always computed from the date, so just use proper_day's value
        liturgical_day.day_of_week = proper_day.day_of_week.clone();

        // Copy nth_day_of_week_in_month
        if liturgical_day.nth_day_of_week_in_month == 0 {
            liturgical_day.nth_day_of_week_in_month = proper_day.nth_day_of_week_in_month;
        }

        // Copy start_of_season if not defined
        if liturgical_day.start_of_season.is_none() {
            liturgical_day.start_of_season = proper_day.start_of_season.clone();
        }

        // Copy end_of_season if not defined
        if liturgical_day.end_of_season.is_none() {
            liturgical_day.end_of_season = proper_day.end_of_season.clone();
        }

        // Copy start_of_liturgical_year if empty
        if liturgical_day.start_of_liturgical_year.is_empty() {
            liturgical_day.start_of_liturgical_year = proper_day.start_of_liturgical_year.clone();
        }

        // Copy end_of_liturgical_year if empty
        if liturgical_day.end_of_liturgical_year.is_empty() {
            liturgical_day.end_of_liturgical_year = proper_day.end_of_liturgical_year.clone();
        }

        // Copy sunday_cycle
        liturgical_day.sunday_cycle = proper_day.sunday_cycle;
        liturgical_day.sunday_cycle_name = proper_day.sunday_cycle_name.clone();

        // Copy weekday_cycle
        liturgical_day.weekday_cycle = proper_day.weekday_cycle;
        liturgical_day.weekday_cycle_name = proper_day.weekday_cycle_name.clone();

        // Copy psalter_week
        liturgical_day.psalter_week = proper_day.psalter_week;
        liturgical_day.psalter_week_name = proper_day.psalter_week_name.clone();
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
        by_ids: &BTreeMap<String, Vec<LiturgicalDay>>,
        _dates_index: &BTreeMap<String, Vec<String>>,
    ) -> RomcalResult<Option<LiturgicalDay>> {
        // Check if a day with the same ID already exists (from a parent calendar)
        let existing_day = by_ids.get(day_id).and_then(|days| days.first());

        // Inherit properties from existing day if not defined in day_def
        // Precedence: use day_def if defined, otherwise inherit from existing, otherwise default
        let precedence = day_def
            .precedence
            .clone()
            .or_else(|| existing_day.map(|d| d.precedence.clone()))
            .unwrap_or(Precedence::Weekday_13);
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

        // Inherit date_def if not defined in day_def
        let date_def = day_def
            .date_def
            .clone()
            .or_else(|| existing_day.map(|d| d.date_def.clone()))
            .unwrap_or(DateDef::MonthDate {
                month: crate::types::dates::MonthIndex(1),
                date: 1,
                day_offset: None,
            });

        // Inherit other boolean properties
        let is_holy_day_of_obligation = day_def
            .is_holy_day_of_obligation
            .or_else(|| existing_day.map(|d| d.is_holy_day_of_obligation))
            .unwrap_or(false);
        let is_optional = day_def
            .is_optional
            .or_else(|| existing_day.map(|d| d.is_optional))
            .unwrap_or(false);
        let allow_similar_rank_items = day_def
            .allow_similar_rank_items
            .or_else(|| existing_day.map(|d| d.allow_similar_rank_items))
            .unwrap_or(false);

        let mut liturgical_day = LiturgicalDay::new(
            day_id.clone(),
            day_id.clone(), // fullname - would be localized in full implementation
            date_str.to_string(),
            date_def,
            precedence.clone(),
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
        .with_is_holy_day_of_obligation(is_holy_day_of_obligation)
        .with_is_optional(is_optional)
        .with_allow_similar_rank_items(allow_similar_rank_items);

        // Add date exceptions - inherit if not defined in day_def
        if let Some(exceptions) = &day_def.date_exceptions {
            liturgical_day.date_exceptions = match exceptions {
                DateDefExceptions::Single(e) => vec![e.clone()],
                DateDefExceptions::Multiple(list) => list.clone(),
            };
        } else if let Some(existing) = existing_day {
            // Inherit date_exceptions from existing day if not defined
            if !existing.date_exceptions.is_empty() {
                liturgical_day.date_exceptions = existing.date_exceptions.clone();
            }
        }

        // Add titles - inherit if not defined in day_def
        if let Some(titles) = &day_def.titles {
            liturgical_day.titles = titles.clone();
        } else if let Some(existing) = existing_day {
            // Inherit titles from existing day if not defined
            if !existing.titles.is_empty() {
                liturgical_day.titles = existing.titles.clone();
            }
        }

        // Add explicit colors if defined (priority over martyr rule)
        // Inherit if not defined in day_def
        if let Some(colors_def) = &day_def.colors {
            use crate::types::calendar::ColorsDef;
            let colors: Vec<Color> = match colors_def {
                ColorsDef::Single(c) => vec![c.clone()],
                ColorsDef::Multiple(list) => list.clone(),
            };
            liturgical_day.colors = colors
                .into_iter()
                .map(|c| ColorInfo {
                    key: c.clone(),
                    name: enum_to_string(&c),
                })
                .collect();
        } else if let Some(existing) = existing_day {
            // Inherit colors from existing day if not defined
            if !existing.colors.is_empty() {
                liturgical_day.colors = existing.colors.clone();
            }
        }

        // Calculate and store parent overrides (diff from parent definitions)
        let parent_overrides =
            self.compute_parent_overrides(day_id, day_def, calendar_def, by_ids)?;
        if !parent_overrides.is_empty() {
            liturgical_day.parent_overrides = parent_overrides;
        }

        Ok(Some(liturgical_day))
    }

    /// Computes the parent overrides (diff) for a day definition
    fn compute_parent_overrides(
        &self,
        day_id: &DayId,
        day_def: &DayDefinition,
        calendar_def: &CalendarDefinition,
        by_ids: &BTreeMap<String, Vec<LiturgicalDay>>,
    ) -> RomcalResult<Vec<ParentOverride>> {
        let mut overrides = Vec::new();

        // Check if this day already exists from a parent calendar
        if let Some(existing_days) = by_ids.get(day_id) {
            // Find existing days that are NOT from proper_of_time
            // and NOT from the current calendar
            for existing_day in existing_days {
                if existing_day.from_calendar_id != PROPER_OF_TIME_ID
                    && existing_day.from_calendar_id != calendar_def.id
                {
                    // Create a diff for this parent
                    let mut parent_override =
                        ParentOverride::new(existing_day.from_calendar_id.clone());

                    // Check what's different
                    if day_def.date_def.is_some() {
                        parent_override.date_def = Some(existing_day.date_def.clone());
                    }

                    if day_def.date_exceptions.is_some() && !existing_day.date_exceptions.is_empty()
                    {
                        parent_override.date_exceptions =
                            Some(existing_day.date_exceptions.clone());
                    }

                    if day_def.precedence.is_some() {
                        parent_override.precedence = Some(existing_day.precedence.clone());
                        parent_override.rank = Some(existing_day.rank.clone());
                    }

                    if day_def.colors.is_some() && !existing_day.colors.is_empty() {
                        parent_override.colors = Some(existing_day.colors.clone());
                    }

                    if day_def.titles.is_some() {
                        parent_override.titles = Some(existing_day.titles.clone());
                    }

                    if day_def.is_holy_day_of_obligation.is_some() {
                        parent_override.is_holy_day_of_obligation =
                            Some(existing_day.is_holy_day_of_obligation);
                    }

                    if day_def.is_optional.is_some() {
                        parent_override.is_optional = Some(existing_day.is_optional);
                    }

                    if day_def.allow_similar_rank_items.is_some() {
                        parent_override.allow_similar_rank_items =
                            Some(existing_day.allow_similar_rank_items);
                    }

                    if parent_override.has_changes() {
                        overrides.push(parent_override);
                    }
                }
            }
        }

        Ok(overrides)
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

        // Detect weekday_13 and optional memorials
        let weekday_13 = days
            .iter()
            .find(|d| d.precedence == Precedence::Weekday_13)
            .cloned();

        let mut optional_memorials: Vec<LiturgicalDay> = days
            .iter()
            .filter(|d| d.precedence == Precedence::OptionalMemorial_12)
            .cloned()
            .collect();

        // Sort optional memorials by calendar priority (more general first)
        optional_memorials.sort_by_key(|d| {
            self.calendar_priority
                .get(&d.from_calendar_id)
                .copied()
                .unwrap_or(usize::MAX)
        });

        // Base result starts with the highest precedence day
        let mut result = vec![highest.clone()];

        // Handle allowSimilarRankItems
        if highest.allow_similar_rank_items {
            for day in days.iter().skip(1) {
                if day.rank == highest.rank && !result.iter().any(|d| d.id == day.id) {
                    result.push(day.clone());
                }
            }
        }

        // During Lent, obligatory memorials become optional (UNLY #14)
        if let Some(Season::Lent) = highest.season {
            for day in days.iter().skip(1) {
                if day.rank == Rank::Memorial && !result.iter().any(|d| d.id == day.id) {
                    let mut optional_day = day.clone();
                    optional_day.is_optional = true;
                    optional_day.rank = Rank::OptionalMemorial;
                    result.push(optional_day);
                }
            }
        }

        // Optional memorial handling with weekday inclusion
        let highest_allows_optional = self.can_have_optional_memorials(highest);
        let highest_is_optional = highest.precedence == Precedence::OptionalMemorial_12;

        if (highest_allows_optional || highest_is_optional) && !optional_memorials.is_empty() {
            let mut ordered: Vec<LiturgicalDay> = Vec::new();

            if let Some(weekday) = weekday_13.clone() {
                if !ordered.iter().any(|d| d.id == weekday.id) {
                    ordered.push(weekday);
                }
            }

            if highest.precedence != Precedence::Weekday_13
                && highest.precedence != Precedence::OptionalMemorial_12
                && !ordered.iter().any(|d| d.id == highest.id)
            {
                ordered.push(highest.clone());
            }

            for day in optional_memorials {
                if !ordered.iter().any(|d| d.id == day.id) {
                    ordered.push(day);
                }
            }

            for day in result {
                if !ordered.iter().any(|d| d.id == day.id) {
                    ordered.push(day);
                }
            }

            return ordered;
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

    #[test]
    fn test_optional_memorials_keep_weekday_and_order_by_calendar() {
        use crate::types::liturgical::{PsalterWeekCycle, SundayCycle, WeekdayCycle};
        use std::collections::HashMap;

        let preset = Preset::default();
        let mut calendar = Calendar::new(preset, 2026).unwrap();

        // Simulate a hierarchy: general_roman (0) < france (1) < france__angers (2)
        calendar.calendar_priority = HashMap::from([
            ("general_roman".to_string(), 0),
            ("france".to_string(), 1),
            ("france__angers".to_string(), 2),
        ]);

        let mut days = vec![
            LiturgicalDay::new(
                "weekday".to_string(),
                "Weekday".to_string(),
                "2026-06-01".to_string(),
                DateDef::MonthDate {
                    month: crate::types::dates::MonthIndex(6),
                    date: 1,
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
                "general_roman".to_string(),
            ),
            LiturgicalDay::new(
                "optional_france".to_string(),
                "Optional Memorial France".to_string(),
                "2026-06-01".to_string(),
                DateDef::MonthDate {
                    month: crate::types::dates::MonthIndex(6),
                    date: 1,
                    day_offset: None,
                },
                Precedence::OptionalMemorial_12,
                Rank::OptionalMemorial,
                "Optional Memorial".to_string(),
                SundayCycle::YearA,
                "Year A".to_string(),
                WeekdayCycle::Year1,
                "Year I".to_string(),
                PsalterWeekCycle::Week1,
                "Week 1".to_string(),
                "france".to_string(),
            ),
            LiturgicalDay::new(
                "optional_general".to_string(),
                "Optional Memorial General".to_string(),
                "2026-06-01".to_string(),
                DateDef::MonthDate {
                    month: crate::types::dates::MonthIndex(6),
                    date: 1,
                    day_offset: None,
                },
                Precedence::OptionalMemorial_12,
                Rank::OptionalMemorial,
                "Optional Memorial".to_string(),
                SundayCycle::YearA,
                "Year A".to_string(),
                WeekdayCycle::Year1,
                "Year I".to_string(),
                PsalterWeekCycle::Week1,
                "Week 1".to_string(),
                "general_roman".to_string(),
            ),
        ];

        let result = calendar.apply_precedence_rules(&mut days);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0].id, "weekday");
        assert_eq!(result[1].id, "optional_general");
        assert_eq!(result[2].id, "optional_france");
    }

    #[test]
    fn test_proper_of_time_end_of_season_not_null() {
        let preset = Preset::default();
        let calendar = Calendar::new(preset, 2026).unwrap();
        let result = calendar.generate().unwrap();

        // Check that all days from Proper of Time have end_of_season defined
        for (_date, days) in &result {
            for day in days {
                if day.from_calendar_id == PROPER_OF_TIME_ID {
                    assert!(
                        day.end_of_season.is_some(),
                        "Day '{}' from Proper of Time on {} should have end_of_season defined",
                        day.id,
                        day.date
                    );
                }
            }
        }
    }

    #[test]
    fn test_parent_override_structure() {
        use crate::liturgical_day::ParentOverride;

        // Test that ParentOverride can be created and checked for changes
        let mut override_empty = ParentOverride::new("test_calendar".to_string());
        assert!(!override_empty.has_changes());

        override_empty.precedence = Some(Precedence::GeneralMemorial_10);
        assert!(override_empty.has_changes());
    }

    #[test]
    fn test_martyr_color_from_titles() {
        use crate::types::entity::{Title, TitlesDef};

        // Test with martyr title
        let martyr_titles = TitlesDef::Titles(vec![Title::Bishop, Title::Martyr]);
        assert!(martyr_titles.contains_martyr());

        // Test without martyr title
        let non_martyr_titles = TitlesDef::Titles(vec![Title::Bishop, Title::Virgin]);
        assert!(!non_martyr_titles.contains_martyr());

        // Test TheFirstMartyr
        let first_martyr_titles = TitlesDef::Titles(vec![Title::TheFirstMartyr]);
        assert!(first_martyr_titles.contains_martyr());

        // Test ProtoMartyrOfOceania
        let proto_martyr_titles = TitlesDef::Titles(vec![Title::ProtoMartyrOfOceania]);
        assert!(proto_martyr_titles.contains_martyr());
    }
}
