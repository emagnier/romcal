//! # Calendar Generation Module
//!
//! This module implements the liturgical calendar generation algorithm.
//! It combines the Proper of Time with particular calendars and applies
//! precedence rules according to UNLY #49.

mod date_calculation;
mod hierarchy;
mod inheritance;
mod precedence;

#[cfg(test)]
mod tests;

use chrono::{Duration, NaiveDate};
use std::collections::{BTreeMap, HashMap};

use super::calendar_definition::CalendarDefinition;
use super::dates::LiturgicalDates;
use super::liturgical_day::LiturgicalDay;
use super::proper_of_time::ProperOfTime;
use crate::error::RomcalResult;
use crate::martyrology_resolution::MartyrologyResolver;
use crate::romcal::Romcal;
use crate::types::mass::{CelebrationSummary, MassCalendar, MassContext, MassTime};

/// Type alias for the liturgical calendar output
/// Maps date strings (YYYY-MM-DD) to vectors of LiturgicalDay objects
pub type LiturgicalCalendar = BTreeMap<String, Vec<LiturgicalDay>>;

/// Calendar generator that combines Proper of Time with particular calendars
/// and applies precedence rules according to UNLY #49.
pub struct Calendar {
    /// The romcal configuration
    romcal: Romcal,
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
    /// Martyrology resolver for resolving martyrology pointers to full entries
    martyrology_resolver: MartyrologyResolver,
}

/// Internal state during calendar building.
///
/// Holds the intermediate data structures used while constructing
/// the liturgical calendar from various calendar definitions.
struct CalendarBuildState {
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
    /// * `romcal` - Romcal configuration
    /// * `year` - Liturgical year (e.g., 2026 for liturgical year 2025-2026)
    ///
    /// # Errors
    ///
    /// Returns an error if the year is invalid
    pub fn new(romcal: Romcal, year: i32) -> RomcalResult<Self> {
        let dates = LiturgicalDates::new(romcal.clone(), year)?;

        let (calendar_hierarchy, calendar_priority) = Self::resolve_calendar_hierarchy(&romcal)?;

        // Calculate liturgical year boundaries
        // Start: First Sunday of Advent (previous calendar year)
        // End: Saturday before the next First Sunday of Advent
        let start_of_year =
            LiturgicalDates::get_first_sunday_of_advent_date_static(year - 1).date_naive();
        let end_of_year = LiturgicalDates::get_first_sunday_of_advent_date_static(year)
            .date_naive()
            - Duration::days(1);

        // Create martyrology resolver with locale-merged resources
        let martyrology_resolver = MartyrologyResolver::new(&romcal);

        Ok(Self {
            romcal,
            dates,
            year,
            start_of_year,
            end_of_year,
            calendar_hierarchy,
            calendar_priority,
            martyrology_resolver,
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

    /// Generates a mass-centric view of the liturgical calendar.
    ///
    /// Unlike `generate()` which groups by liturgical date, this function
    /// groups by civil date and mass time. Evening masses (EasterVigil,
    /// PreviousEveningMass) appear on the PREVIOUS civil day.
    ///
    /// # Returns
    ///
    /// A BTreeMap of civil date strings to vectors of MassContext objects
    ///
    /// # Errors
    ///
    /// Returns an error if calendar generation fails
    pub fn generate_mass_calendar(&self) -> RomcalResult<MassCalendar> {
        // Step 1: Generate the standard liturgical calendar
        let liturgical_calendar = self.generate()?;

        // Step 2: Transform into mass-centric view
        let mut mass_calendar: MassCalendar = BTreeMap::new();

        for (liturgical_date, days) in &liturgical_calendar {
            // Parse the liturgical date
            let lit_date = NaiveDate::parse_from_str(liturgical_date, "%Y-%m-%d").map_err(|e| {
                crate::error::RomcalError::ValidationError(format!(
                    "Cannot parse date {}: {}",
                    liturgical_date, e
                ))
            })?;

            // Skip empty days
            if days.is_empty() {
                continue;
            }

            // Separate primary celebration from optional alternatives
            // The first day is typically the primary (highest precedence)
            // Optional memorials appear after the primary
            let (primary_day, optional_days) = (&days[0], &days[1..]);

            // Convert optional days to CelebrationSummary
            let optional_celebrations: Vec<CelebrationSummary> = optional_days
                .iter()
                .filter(|d| {
                    d.is_optional || d.rank == crate::types::liturgical::Rank::OptionalMemorial
                })
                .map(CelebrationSummary::from)
                .collect();

            // Process each mass of the primary celebration
            for mass_info in &primary_day.masses {
                // Calculate the civil date (shift for evening masses)
                let civil_date = self.compute_civil_date(&lit_date, &mass_info.mass_type);
                let civil_date_str = civil_date.format("%Y-%m-%d").to_string();

                // Create MassContext
                let mass_context = MassContext::new(
                    primary_day,
                    mass_info.mass_type.clone(),
                    civil_date_str.clone(),
                    optional_celebrations.clone(),
                );

                // Add to mass calendar grouped by civil date
                mass_calendar
                    .entry(civil_date_str)
                    .or_default()
                    .push(mass_context);
            }
        }

        Ok(mass_calendar)
    }

    /// Computes the civil date for a mass based on its type.
    ///
    /// Evening masses (EasterVigil, PreviousEveningMass) are celebrated
    /// the evening before the liturgical date, so they appear on the
    /// previous civil day.
    fn compute_civil_date(&self, liturgical_date: &NaiveDate, mass_time: &MassTime) -> NaiveDate {
        match mass_time {
            MassTime::EasterVigil | MassTime::PreviousEveningMass => {
                *liturgical_date - Duration::days(1)
            }
            _ => *liturgical_date,
        }
    }

    /// Builds dates data from all calendar sources
    fn build_dates_data(&self) -> RomcalResult<CalendarBuildState> {
        let mut by_ids: BTreeMap<String, Vec<LiturgicalDay>> = BTreeMap::new();
        let mut dates_index: BTreeMap<String, Vec<String>> = BTreeMap::new();

        // Step 1: Generate Proper of Time days
        let proper_of_time = ProperOfTime::new(self.romcal.clone(), self.year)?;
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

        Ok(CalendarBuildState {
            by_ids,
            dates_index,
        })
    }
}
