use std::collections::HashSet;

use super::Calendar;
use crate::engine::calendar_definition::CalendarDefinition;
use crate::engine::liturgical_day::LiturgicalDay;
use crate::engine::proper_of_time::utils::PROPER_OF_TIME_ID;
use crate::error::{RomcalError, RomcalResult};
use crate::romcal::Romcal;
use crate::types::calendar::{DayDefinition, DayId};
use crate::types::dates::DateDefExceptions;
use std::collections::{BTreeMap, HashMap};

impl Calendar {
    /// Resolves the calendar hierarchy from root to target (general to specific)
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The target calendar doesn't exist in calendar_definitions
    /// - Any parent calendar in the hierarchy doesn't exist
    pub(super) fn resolve_calendar_hierarchy(
        romcal: &Romcal,
    ) -> RomcalResult<(Vec<CalendarDefinition>, HashMap<String, usize>)> {
        // The temporal cycle uses only the proper of time, no calendar definitions needed
        if romcal.calendar == crate::romcal::TEMPORAL_CYCLE {
            return Ok((Vec::new(), HashMap::new()));
        }

        let mut hierarchy = Vec::new();
        let mut visited_ids = HashSet::new();
        let mut current_path = HashSet::new();

        // Validate and load the target calendar and its parent chain
        let target = romcal
            .get_calendar_definition(&romcal.calendar)
            .ok_or_else(|| {
                RomcalError::ValidationError(format!(
                    "Calendar '{}' not found in calendar definitions.",
                    romcal.calendar
                ))
            })?;
        Self::collect_calendar_hierarchy(
            romcal,
            target,
            &mut hierarchy,
            &mut visited_ids,
            &mut current_path,
        )?;

        // Post-order DFS produces the correct order (general → specific), no reverse needed

        let mut calendar_priority = HashMap::new();
        for (idx, calendar) in hierarchy.iter().enumerate() {
            calendar_priority.entry(calendar.id.clone()).or_insert(idx);
        }

        Ok((hierarchy, calendar_priority))
    }

    /// Recursively collects calendar definitions in hierarchy
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - Any parent calendar doesn't exist
    /// - A circular reference is detected
    fn collect_calendar_hierarchy(
        romcal: &Romcal,
        calendar: &CalendarDefinition,
        hierarchy: &mut Vec<CalendarDefinition>,
        visited: &mut HashSet<String>,
        current_path: &mut HashSet<String>,
    ) -> RomcalResult<()> {
        // Check for circular reference (calendar in current recursion path)
        if current_path.contains(&calendar.id) {
            return Err(RomcalError::ValidationError(format!(
                "Circular reference detected in calendar hierarchy: '{}' references itself (directly or indirectly).",
                calendar.id
            )));
        }

        // Skip if already fully processed (diamond inheritance is OK)
        if visited.contains(&calendar.id) {
            return Ok(());
        }

        // Mark as being processed in current path
        current_path.insert(calendar.id.clone());

        // Process parent calendars FIRST (post-order DFS)
        for parent_id in &calendar.parent_calendar_ids {
            let parent = romcal.get_calendar_definition(parent_id).ok_or_else(|| {
                RomcalError::ValidationError(format!(
                    "Parent calendar '{}' (required by '{}') not found in calendar definitions.",
                    parent_id, calendar.id
                ))
            })?;
            Self::collect_calendar_hierarchy(romcal, parent, hierarchy, visited, current_path)?;
        }

        // Remove from current path, mark as fully visited
        current_path.remove(&calendar.id);
        visited.insert(calendar.id.clone());

        // Add this calendar AFTER processing all parents
        hierarchy.push(calendar.clone());
        Ok(())
    }

    /// Processes a calendar definition and adds its days to the index
    pub(super) fn process_calendar_definition(
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
                if let Some(existing_days) = by_ids.get(day_id)
                    && existing_days
                        .iter()
                        .any(|d| d.from_calendar_id == PROPER_OF_TIME_ID)
                {
                    return Err(RomcalError::ValidationError(format!(
                        "In the '{}' calendar, you can't drop a LiturgicalDay from the Proper of Time: '{}'.",
                        calendar_def.id, day_id
                    )));
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
            if effective_day_def.date_def.is_none()
                && let Some(existing) = existing_day
            {
                effective_day_def.date_def = Some(existing.date_def.clone());
            }
            // Also inherit date_exceptions if not defined
            if effective_day_def.date_exceptions.is_none()
                && let Some(existing) = existing_day
                && !existing.date_exceptions.is_empty()
            {
                effective_day_def.date_exceptions = Some(if existing.date_exceptions.len() == 1 {
                    DateDefExceptions::Single(existing.date_exceptions[0].clone())
                } else {
                    DateDefExceptions::Multiple(existing.date_exceptions.clone())
                });
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

    /// Computes the parent overrides (diff) for a day definition
    pub(super) fn compute_parent_overrides(
        &self,
        day_id: &DayId,
        day_def: &DayDefinition,
        calendar_def: &CalendarDefinition,
        by_ids: &BTreeMap<String, Vec<LiturgicalDay>>,
    ) -> RomcalResult<Vec<crate::engine::liturgical_day::ParentOverride>> {
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
                    let mut parent_override = crate::engine::liturgical_day::ParentOverride::new(
                        existing_day.from_calendar_id.clone(),
                    );

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
}
