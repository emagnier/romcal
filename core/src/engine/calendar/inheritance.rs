use std::collections::BTreeMap;

use super::Calendar;
use crate::engine::calendar_definition::CalendarDefinition;
use crate::engine::liturgical_day::LiturgicalDay;
use crate::engine::proper_of_time::utils::{PROPER_OF_TIME_ID, enum_to_string};
use crate::error::{RomcalError, RomcalResult};
use crate::types::calendar::{DayDefinition, DayId};
use crate::types::dates::{DateDef, DateDefExceptions};
use crate::types::liturgical::{Color, ColorInfo, Precedence};
use crate::types::mass::MassInfo;

impl Calendar {
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
                if let Some(days) = by_ids.get(day_id)
                    && let Some(proper_day) = days
                        .iter()
                        .find(|d| d.from_calendar_id == PROPER_OF_TIME_ID && d.date == date_str)
                {
                    return Some(proper_day);
                }
            }
        }
        None
    }

    /// Inherits properties from the Proper of Time day to a particular calendar day
    pub(super) fn inherit_proper_of_time_properties(
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

    /// Creates a LiturgicalDay from a DayDefinition
    pub(super) fn create_liturgical_day_from_definition(
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
        let date = chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|e| {
            RomcalError::ValidationError(format!("Cannot parse date {}: {}", date_str, e))
        })?;
        let dow = chrono::Datelike::weekday(&date).num_days_from_sunday() as u8;

        // Get cycles from cache (we'd need to access proper_of_time cache here)
        // For now, use defaults - in full implementation, this would be calculated
        use crate::types::liturgical::{PsalterWeekCycle, SundayCycle, WeekdayCycle};

        let sunday_cycle = SundayCycle::from_year(self.year);
        let weekday_cycle = WeekdayCycle::from_year(self.year);
        let psalter_week = PsalterWeekCycle::Week_1; // Simplified

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

        // Resolve the fullname from the martyrology
        // If custom_locale_id is defined, use it for lookup, otherwise use day_id
        let custom_locale_id = day_def.custom_locale_id.as_deref();
        let fullname = self
            .martyrology_resolver
            .get_fullname_for_day(day_id, custom_locale_id)
            .unwrap_or_else(|| day_id.clone());

        let mut liturgical_day = LiturgicalDay::new(
            day_id.clone(),
            fullname, // Use resolved fullname from martyrology
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

        // Resolve martyrology entries for this day using the martyrology resolver
        // Priority: day_def.martyrology > fallback on day_id
        // Returns error if entry not found after locale fallback
        let resolved_martyrology = self
            .martyrology_resolver
            .resolve_martyrology_for_day(day_def, day_id)?;

        // Set martyrology entries on the liturgical day
        liturgical_day.martyrology = resolved_martyrology.clone();

        // Add titles - priority: day_def.titles > combined from martyrology > inherited from existing
        if let Some(titles) = &day_def.titles {
            // Explicitly defined titles in the calendar definition
            liturgical_day.titles = titles.clone();
        } else if !resolved_martyrology.is_empty() {
            // Combine titles from all resolved entries
            liturgical_day.titles = self
                .martyrology_resolver
                .combine_titles(&resolved_martyrology);
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

        // Set masses from DayDefinition.masses if defined, otherwise keep default
        if let Some(masses_def) = &day_def.masses {
            // Extract mass types from MassesDefinitions keys
            let masses: Vec<MassInfo> = masses_def
                .keys()
                .map(|mt| MassInfo::new(mt.clone()))
                .collect();
            if !masses.is_empty() {
                liturgical_day.masses = masses;
            }
        } else if let Some(existing) = existing_day {
            // Inherit masses from existing day if not defined
            if !existing.masses.is_empty() {
                liturgical_day.masses = existing.masses.clone();
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
}
