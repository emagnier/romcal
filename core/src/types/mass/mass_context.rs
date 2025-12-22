//! Mass calendar context types for the mass-centric calendar view.
//!
//! This module provides types for generating a calendar organized by masses
//! instead of liturgical days, with support for civil date shifting for
//! evening masses (Easter Vigil, Previous Evening Mass).

use std::collections::BTreeMap;

#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, Serializer};

use super::MassTime;
use crate::engine::calendar_definition::CalendarId;
use crate::engine::liturgical_day::{LiturgicalDay, LiturgicalDayId};
use crate::types::dates::DayOfWeek;
use crate::types::entity::{Entity, TitlesDef};
use crate::types::liturgical::Season;
use crate::types::{
    ColorInfo, CommonInfo, PeriodInfo, Precedence, PsalterWeekCycle, Rank, SundayCycle,
    WeekdayCycle,
};

/// A calendar organized by civil dates and mass times.
/// Each entry is a date (YYYY-MM-DD) mapping to a list of masses for that day.
pub type MassCalendar = BTreeMap<String, Vec<MassContext>>;

/// Summary of a celebration for use in optional celebrations list.
/// Contains the essential fields from a LiturgicalDay that identify a celebration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
pub struct CelebrationSummary {
    /// The unique identifier of the liturgical day
    pub id: LiturgicalDayId,
    /// The full name of the liturgical day
    pub fullname: String,
    /// The liturgical precedence for this liturgical day
    pub precedence: Precedence,
    /// The liturgical rank for this liturgical day
    pub rank: Rank,
    /// The localized liturgical rank for this liturgical day
    pub rank_name: String,
    /// The liturgical colors for this liturgical day
    pub colors: Vec<ColorInfo>,
    /// The common prayers/readings used for this celebration
    pub commons: Vec<CommonInfo>,
    /// The entities (Saints, Blessed, or Places) linked to this liturgical day
    pub entities: Vec<Entity>,
    /// The titles for this liturgical day
    pub titles: TitlesDef,
    /// Holy days of obligation
    pub is_holy_day_of_obligation: bool,
    /// Indicates if this liturgical day is optional
    pub is_optional: bool,
    /// The ID of the calendar where this liturgical day is defined
    pub from_calendar_id: CalendarId,
}

impl From<&LiturgicalDay> for CelebrationSummary {
    fn from(day: &LiturgicalDay) -> Self {
        Self {
            id: day.id.clone(),
            fullname: day.fullname.clone(),
            precedence: day.precedence.clone(),
            rank: day.rank.clone(),
            rank_name: day.rank_name.clone(),
            colors: day.colors.clone(),
            commons: day.commons.clone(),
            entities: day.entities.clone(),
            titles: day.titles.clone(),
            is_holy_day_of_obligation: day.is_holy_day_of_obligation,
            is_optional: day.is_optional,
            from_calendar_id: day.from_calendar_id.clone(),
        }
    }
}

/// Serializes MassTime to SCREAMING_SNAKE_CASE (e.g., DayMass -> "DAY_MASS")
fn serialize_mass_time_uppercase<S>(mass_time: &MassTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let snake_case = mass_time_to_key(mass_time);
    let screaming_snake_case = snake_case.to_uppercase();
    serializer.serialize_str(&screaming_snake_case)
}

/// Converts a MassTime enum variant to its snake_case key.
fn mass_time_to_key(mass_time: &MassTime) -> String {
    serde_json::to_string(mass_time)
        .unwrap_or_default()
        .trim_matches('"')
        .to_string()
}

/// A flat structure representing a single mass with its full liturgical context.
///
/// This is the main type for the mass-centric calendar view. It contains:
/// - Mass identification (type, name, civil/liturgical dates)
/// - Day-level context (season, cycles, periods)
/// - Primary celebration data (flattened from LiturgicalDay)
/// - Optional alternative celebrations
///
/// For evening masses (Easter Vigil, Previous Evening Mass), the `civil_date`
/// is shifted to the previous day while `liturgical_date` remains the original
/// liturgical celebration date.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
pub struct MassContext {
    // === Mass identification ===
    /// The type of mass (e.g., DayMass, EasterVigil, etc.)
    /// Serialized as SCREAMING_SNAKE_CASE (e.g., "DAY_MASS")
    #[serde(serialize_with = "serialize_mass_time_uppercase")]
    pub mass_time: MassTime,

    /// The localized name of the mass time (translation key in snake_case)
    pub mass_time_name: String,

    /// The civil calendar date when this mass is celebrated (YYYY-MM-DD).
    /// For evening masses (EasterVigil, PreviousEveningMass), this is the day
    /// BEFORE the liturgical date.
    pub civil_date: String,

    /// The liturgical date this mass belongs to (YYYY-MM-DD).
    /// This is the "theological" date of the celebration.
    pub liturgical_date: String,

    // === Day-level context (from the liturgical date) ===
    /// The liturgical season
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season: Option<Season>,

    /// The localized season name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub season_name: Option<String>,

    /// The Sunday cycle (Year A, B, or C)
    pub sunday_cycle: SundayCycle,

    /// The localized Sunday cycle name
    pub sunday_cycle_name: String,

    /// The weekday cycle (Year 1 or 2)
    pub weekday_cycle: WeekdayCycle,

    /// The localized weekday cycle name
    pub weekday_cycle_name: String,

    /// The psalter week cycle (Week 1-4)
    pub psalter_week: PsalterWeekCycle,

    /// The localized psalter week name
    pub psalter_week_name: String,

    /// The week number within the liturgical season
    #[serde(skip_serializing_if = "Option::is_none")]
    pub week_of_season: Option<u32>,

    /// The day number within the liturgical season
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_of_season: Option<u32>,

    /// The day of the week (0=Sunday to 6=Saturday)
    pub day_of_week: DayOfWeek,

    /// The liturgical periods this day belongs to
    pub periods: Vec<PeriodInfo>,

    /// The first day of the current liturgical season
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_of_season: Option<String>,

    /// The last day of the current liturgical season
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_of_season: Option<String>,

    /// The first day of the liturgical year (first Sunday of Advent)
    pub start_of_liturgical_year: String,

    /// The last day of the liturgical year
    pub end_of_liturgical_year: String,

    // === Primary celebration (flattened from LiturgicalDay) ===
    /// The unique identifier of the liturgical day
    pub id: LiturgicalDayId,

    /// The full name of the liturgical day
    pub fullname: String,

    /// The liturgical precedence
    pub precedence: Precedence,

    /// The liturgical rank
    pub rank: Rank,

    /// The localized liturgical rank name
    pub rank_name: String,

    /// The liturgical colors
    pub colors: Vec<ColorInfo>,

    /// The common prayers/readings used
    pub commons: Vec<CommonInfo>,

    /// The entities (Saints, Blessed, or Places) linked to this day
    pub entities: Vec<Entity>,

    /// The titles for this liturgical day
    pub titles: TitlesDef,

    /// Whether this is a holy day of obligation
    pub is_holy_day_of_obligation: bool,

    /// Whether this liturgical day is optional
    pub is_optional: bool,

    /// The ID of the calendar where this liturgical day is defined
    pub from_calendar_id: CalendarId,

    // === Alternative celebrations ===
    /// Optional alternative celebrations (e.g., optional memorials)
    /// that can be celebrated instead of the primary celebration.
    pub optional_celebrations: Vec<CelebrationSummary>,
}

impl MassContext {
    /// Creates a new MassContext from a LiturgicalDay and a specific MassTime.
    ///
    /// # Arguments
    /// * `day` - The liturgical day providing context and celebration data
    /// * `mass_time` - The specific mass time for this entry
    /// * `civil_date` - The civil date (may differ from liturgical date for evening masses)
    /// * `optional_celebrations` - Alternative celebrations for this mass
    pub fn new(
        day: &LiturgicalDay,
        mass_time: MassTime,
        civil_date: String,
        optional_celebrations: Vec<CelebrationSummary>,
    ) -> Self {
        Self {
            // Mass identification
            mass_time_name: mass_time_to_key(&mass_time),
            mass_time,
            civil_date,
            liturgical_date: day.date.clone(),

            // Day-level context
            season: day.season,
            season_name: day.season_name.clone(),
            sunday_cycle: day.sunday_cycle,
            sunday_cycle_name: day.sunday_cycle_name.clone(),
            weekday_cycle: day.weekday_cycle,
            weekday_cycle_name: day.weekday_cycle_name.clone(),
            psalter_week: day.psalter_week,
            psalter_week_name: day.psalter_week_name.clone(),
            week_of_season: day.week_of_season,
            day_of_season: day.day_of_season,
            day_of_week: day.day_of_week.clone(),
            periods: day.periods.clone(),
            start_of_season: day.start_of_season.clone(),
            end_of_season: day.end_of_season.clone(),
            start_of_liturgical_year: day.start_of_liturgical_year.clone(),
            end_of_liturgical_year: day.end_of_liturgical_year.clone(),

            // Primary celebration
            id: day.id.clone(),
            fullname: day.fullname.clone(),
            precedence: day.precedence.clone(),
            rank: day.rank.clone(),
            rank_name: day.rank_name.clone(),
            colors: day.colors.clone(),
            commons: day.commons.clone(),
            entities: day.entities.clone(),
            titles: day.titles.clone(),
            is_holy_day_of_obligation: day.is_holy_day_of_obligation,
            is_optional: day.is_optional,
            from_calendar_id: day.from_calendar_id.clone(),

            // Alternative celebrations
            optional_celebrations,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::dates::{DateDef, MonthIndex};

    fn create_test_day() -> LiturgicalDay {
        LiturgicalDay::new(
            "test_day".to_string(),
            "Test Day".to_string(),
            "2025-12-25".to_string(),
            DateDef::MonthDate {
                month: MonthIndex(12),
                date: 25,
                day_offset: None,
            },
            Precedence::ProperOfTimeSolemnity_2,
            Rank::Solemnity,
            "Solemnity".to_string(),
            SundayCycle::YearC,
            "Year C".to_string(),
            WeekdayCycle::Year_1,
            "Year 1".to_string(),
            PsalterWeekCycle::Week_1,
            "Week 1".to_string(),
            "general_roman".to_string(),
        )
    }

    #[test]
    fn test_celebration_summary_from_liturgical_day() {
        let day = create_test_day();
        let summary = CelebrationSummary::from(&day);

        assert_eq!(summary.id, "test_day");
        assert_eq!(summary.fullname, "Test Day");
        assert_eq!(summary.rank, Rank::Solemnity);
    }

    #[test]
    fn test_mass_context_new() {
        let day = create_test_day();
        let context = MassContext::new(&day, MassTime::DayMass, "2025-12-25".to_string(), vec![]);

        assert_eq!(context.mass_time, MassTime::DayMass);
        assert_eq!(context.mass_time_name, "day_mass");
        assert_eq!(context.civil_date, "2025-12-25");
        assert_eq!(context.liturgical_date, "2025-12-25");
        assert_eq!(context.id, "test_day");
        assert_eq!(context.fullname, "Test Day");
    }

    #[test]
    fn test_mass_context_evening_mass() {
        let day = create_test_day();
        let context = MassContext::new(
            &day,
            MassTime::PreviousEveningMass,
            "2025-12-24".to_string(), // Civil date is the day before
            vec![],
        );

        assert_eq!(context.mass_time, MassTime::PreviousEveningMass);
        assert_eq!(context.civil_date, "2025-12-24");
        assert_eq!(context.liturgical_date, "2025-12-25");
    }

    #[test]
    fn test_mass_context_serialization() {
        let day = create_test_day();
        let context = MassContext::new(&day, MassTime::DayMass, "2025-12-25".to_string(), vec![]);

        let json = serde_json::to_string(&context).unwrap();
        // mass_time is serialized as SCREAMING_SNAKE_CASE
        assert!(json.contains("\"mass_time\":\"DAY_MASS\""));
        // mass_time_name remains in snake_case
        assert!(json.contains("\"mass_time_name\":\"day_mass\""));
    }
}
