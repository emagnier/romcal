use chrono::{Datelike, Duration, NaiveDate, Weekday};

use super::Calendar;
use crate::engine::dates::LiturgicalDates;
use crate::error::{RomcalError, RomcalResult};
use crate::types::calendar::DayDefinition;
use crate::types::dates::{DateDef, DateDefException, DateDefExceptions, ExceptionCondition};

impl Calendar {
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
    pub(super) fn build_date(
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
    pub(super) fn calculate_date_from_def(
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

            DateDef::InheritedFromProperOfTime {} => {
                // This date comes from the Proper of Time and should not be calculated here
                // Return None to indicate that the date is already set from Proper of Time
                Ok(None)
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

    // ==================== Helper functions ====================

    /// Converts a DayOfWeek (0-6) to chrono::Weekday
    pub(super) fn day_of_week_to_weekday(dow: u8) -> Weekday {
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
    pub(super) fn last_day_of_month(year: i32, month: u32) -> RomcalResult<NaiveDate> {
        let next_month = if month == 12 { 1 } else { month + 1 };
        let next_year = if month == 12 { year + 1 } else { year };

        let first_of_next = NaiveDate::from_ymd_opt(next_year, next_month, 1).ok_or_else(|| {
            RomcalError::ValidationError(format!("Invalid month: {}/{}", year, month))
        })?;

        Ok(first_of_next - Duration::days(1))
    }
}
