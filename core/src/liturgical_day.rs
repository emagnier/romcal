#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::dates::{DateDef, DateDefException, DayOfWeek};
use crate::types::entity::{Entity, TitlesDef};
use crate::types::mass::MassInfo;
use crate::types::{
    ColorInfo, CommonDefinition, CommonInfo, PeriodInfo, Precedence, PsalterWeekCycle, Rank,
    SundayCycle, WeekdayCycle,
};
use crate::{CalendarId, Season};

/// Unique identifier for a liturgical day
pub type LiturgicalDayId = String;

/// Represents the differences between a liturgical day definition and its parent definition.
/// This is a lightweight structure that only contains fields that can be overridden.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
pub struct ParentOverride {
    /// The ID of the calendar from which this override originates
    pub from_calendar_id: CalendarId,

    /// The date definition if it was changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_def: Option<DateDef>,

    /// The date exceptions if they were changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_exceptions: Option<Vec<DateDefException>>,

    /// The precedence if it was changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub precedence: Option<Precedence>,

    /// The rank if it was changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<Rank>,

    /// The colors if they were changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<ColorInfo>>,

    /// The titles if they were changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub titles: Option<TitlesDef>,

    /// The commons definition if it was changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commons_def: Option<Vec<CommonDefinition>>,

    /// The is_holy_day_of_obligation flag if it was changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_holy_day_of_obligation: Option<bool>,

    /// The is_optional flag if it was changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_optional: Option<bool>,

    /// The allow_similar_rank_items flag if it was changed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_similar_rank_items: Option<bool>,
}

impl ParentOverride {
    /// Creates a new ParentOverride with only the from_calendar_id set.
    pub fn new(from_calendar_id: CalendarId) -> Self {
        Self {
            from_calendar_id,
            date_def: None,
            date_exceptions: None,
            precedence: None,
            rank: None,
            colors: None,
            titles: None,
            commons_def: None,
            is_holy_day_of_obligation: None,
            is_optional: None,
            allow_similar_rank_items: None,
        }
    }

    /// Returns true if this override has any changes
    pub fn has_changes(&self) -> bool {
        self.date_def.is_some()
            || self.date_exceptions.is_some()
            || self.precedence.is_some()
            || self.rank.is_some()
            || self.colors.is_some()
            || self.titles.is_some()
            || self.commons_def.is_some()
            || self.is_holy_day_of_obligation.is_some()
            || self.is_optional.is_some()
            || self.allow_similar_rank_items.is_some()
    }
}

/// A single day in the liturgical calendar with computed values and inheritance information.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
pub struct LiturgicalDay {
    /// The unique identifier of the liturgical day
    pub id: LiturgicalDayId,
    /// The full name of the liturgical day
    pub fullname: String,

    /// The computed date of the liturgical day.
    pub date: String, // in ISO 8601 format: YYYY-MM-DD

    /// The date definition for this liturgical day.
    pub date_def: DateDef, // Use Struct DateDef

    /// The date definition exceptions for this liturgical day.
    pub date_exceptions: Vec<DateDefException>, // Use Struct DateDefException

    /// The liturgical precedence for this liturgical day.
    pub precedence: Precedence, // Use Enum Precedence

    /// The liturgical rank for this liturgical day.
    pub rank: Rank, // Use Enum Rank

    /// The localized liturgical rank for this liturgical day.
    pub rank_name: String,

    /// Allows similar items with the same rank and same or lower precedence
    /// to coexist without this liturgical day overwriting them.
    pub allow_similar_rank_items: bool,

    /// Holy days of obligation are days on which the faithful are expected to attend Mass,
    /// and engage in rest from work and recreation.
    pub is_holy_day_of_obligation: bool,

    /// Indicates if this liturgical day is optional within a specific liturgical calendar.
    pub is_optional: bool,

    /// The liturgical seasons to which this liturgical day belongs.
    pub season: Option<Season>,

    /// The liturgical season name.
    pub season_name: Option<String>,

    /// The liturgical periods to which this liturgical day belongs.
    pub periods: Vec<PeriodInfo>, // Use Enum Period

    /// The common prayers, readings, and chants used for celebrating saints or
    /// feasts that belong to a specific category, such as martyrs, virgins, pastors, or the Blessed
    /// Virgin Mary.
    pub commons: Vec<CommonInfo>, // Use Enum Common

    /// The liturgical colors for this liturgical day.
    pub colors: Vec<ColorInfo>, // Use Enum Color

    /// The masses celebrated on this liturgical day.
    /// Most days have a single DayMass, but some have multiple masses
    /// (e.g., Christmas: PreviousEveningMass, NightMass, MassAtDawn, DayMass).
    /// Aliturgical days like Holy Saturday have an empty list.
    pub masses: Vec<MassInfo>,

    /// The titles for this liturgical day.
    pub titles: TitlesDef, // Use Enum Title

    /// The entities (Saints, Blessed, or Places) linked to this liturgical day.
    pub entities: Vec<Entity>, // Use Struct Entity

    /// The week number of the current liturgical season.
    /// Starts from `1`, except in the seasons of lent,
    /// the week of Ash Wednesday to the next Saturday is counted as `0`.
    pub week_of_season: Option<u32>,

    /// The day number within the current liturgical season.
    pub day_of_season: Option<u32>,

    /// The day of the week for this liturgical day.
    /// Returns a number from 0 (Sunday) to 6 (Saturday).
    pub day_of_week: DayOfWeek, // Use Struct DayOfWeek

    /// The nth occurrence of this day of the week within the current month.
    /// For example, the 3rd Sunday of the month would have nth_day_of_week_in_month = 3.
    pub nth_day_of_week_in_month: u8,

    /// The first day of the current liturgical season for this liturgical day.
    pub start_of_season: Option<String>, // in ISO 8601 format: YYYY-MM-DD

    /// The last day of the current liturgical season for this liturgical day.
    pub end_of_season: Option<String>, // in ISO 8601 format: YYYY-MM-DD

    /// The first day of the current liturgical year for this liturgical day,
    /// i.e. the first Sunday of Advent.
    pub start_of_liturgical_year: String, // in ISO 8601 format: YYYY-MM-DD

    /// The last day of the current liturgical year for this liturgical day,
    /// i.e. the last Saturday of Ordinary Time, in the 34th week.
    pub end_of_liturgical_year: String, // in ISO 8601 format: YYYY-MM-DD

    /// The Sunday cycle to which this liturgical day belongs.
    pub sunday_cycle: SundayCycle,

    /// The localized name of the Sunday cycle to which this liturgical day belongs.
    pub sunday_cycle_name: String,

    /// The weekday cycle to which this liturgical day belongs.
    pub weekday_cycle: WeekdayCycle,

    /// The localized name of the weekday cycle to which this liturgical day belongs.
    pub weekday_cycle_name: String,

    /// The psalter week cycle to which this liturgical day belongs.
    pub psalter_week: PsalterWeekCycle,

    /// The localized name of the psalter week cycle to which this liturgical day belongs.
    pub psalter_week_name: String,

    /// The ID of the calendar where this liturgical day is defined.
    /// Indicates the source calendar in the inheritance chain.
    pub from_calendar_id: CalendarId,

    /// Contains the differences between this liturgical day and its parent definitions.
    /// Each element in the array represents the diff from a parent calendar definition.
    /// The array is ordered from most general (e.g., general_roman) to most specific.
    pub parent_overrides: Vec<ParentOverride>,
}

impl LiturgicalDay {
    /// Creates a new LiturgicalDay.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the liturgical day
    /// * `fullname` - The full name of the liturgical day
    /// * `date` - The computed date in ISO 8601 format
    /// * `date_def` - The date definition for this liturgical day
    /// * `precedence` - The liturgical precedence for this liturgical day
    /// * `rank` - The liturgical rank for this liturgical day
    /// * `rank_name` - The localized liturgical rank for this liturgical day
    /// * `from_calendar_id` - The ID of the calendar where this liturgical day is defined
    ///
    /// # Returns
    ///
    /// A new LiturgicalDay instance with the specified required fields and default values for optional fields.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: LiturgicalDayId,
        fullname: String,
        date: String,
        date_def: DateDef,
        precedence: Precedence,
        rank: Rank,
        rank_name: String,
        sunday_cycle: SundayCycle,
        sunday_cycle_name: String,
        weekday_cycle: WeekdayCycle,
        weekday_cycle_name: String,
        psalter_week: PsalterWeekCycle,
        psalter_week_name: String,
        from_calendar_id: CalendarId,
    ) -> Self {
        Self {
            id,
            fullname,
            date,
            date_def,
            date_exceptions: Vec::new(),
            precedence,
            rank,
            rank_name,
            allow_similar_rank_items: false,
            is_holy_day_of_obligation: false,
            is_optional: false,
            season: None,
            season_name: None,
            periods: Vec::new(),
            commons: Vec::new(),
            colors: Vec::new(),
            masses: MassInfo::default_day_mass(),
            titles: TitlesDef::Titles(Vec::new()),
            entities: Vec::new(),
            week_of_season: None,
            day_of_season: None,
            day_of_week: DayOfWeek(0), // Sunday
            nth_day_of_week_in_month: 0,
            start_of_season: None,
            end_of_season: None,
            start_of_liturgical_year: String::new(),
            end_of_liturgical_year: String::new(),
            sunday_cycle,
            sunday_cycle_name,
            weekday_cycle,
            weekday_cycle_name,
            psalter_week,
            psalter_week_name,
            from_calendar_id,
            parent_overrides: Vec::new(),
        }
    }

    /// Sets the liturgical seasons for this liturgical day.
    pub fn with_seasons(mut self, season: Season) -> Self {
        self.season = Some(season);
        self
    }

    /// Sets the liturgical season name for this liturgical day.
    pub fn with_season_name(mut self, season_name: String) -> Self {
        self.season_name = Some(season_name);
        self
    }

    /// Sets the week number within the liturgical season for this liturgical day.
    pub fn with_week_of_season(mut self, week_of_season: u32) -> Self {
        self.week_of_season = Some(week_of_season);
        self
    }

    /// Sets the day number within the liturgical season for this liturgical day.
    pub fn with_day_of_season(mut self, day_of_season: u32) -> Self {
        self.day_of_season = Some(day_of_season);
        self
    }

    /// Sets the start date of the liturgical season for this liturgical day.
    pub fn with_start_of_season(mut self, start_of_season: String) -> Self {
        self.start_of_season = Some(start_of_season);
        self
    }

    /// Sets the end date of the liturgical season for this liturgical day.
    pub fn with_end_of_season(mut self, end_of_season: String) -> Self {
        self.end_of_season = Some(end_of_season);
        self
    }

    /// Sets the liturgical periods for this liturgical day.
    pub fn with_periods(mut self, periods: Vec<PeriodInfo>) -> Self {
        self.periods = periods;
        self
    }

    /// Sets the liturgical colors for this liturgical day.
    pub fn with_colors(mut self, colors: Vec<ColorInfo>) -> Self {
        self.colors = colors;
        self
    }

    /// Sets the masses for this liturgical day.
    pub fn with_masses(mut self, masses: Vec<MassInfo>) -> Self {
        self.masses = masses;
        self
    }

    /// Sets the common prayers for this liturgical day.
    pub fn with_commons(mut self, commons: Vec<CommonInfo>) -> Self {
        self.commons = commons;
        self
    }

    /// Sets the entities linked to this liturgical day.
    pub fn with_entities(mut self, entities: Vec<Entity>) -> Self {
        self.entities = entities;
        self
    }

    /// Sets the titles for this liturgical day.
    pub fn with_titles(mut self, titles: TitlesDef) -> Self {
        self.titles = titles;
        self
    }

    /// Sets the day of the week for this liturgical day.
    pub fn with_day_of_week(mut self, day_of_week: DayOfWeek) -> Self {
        self.day_of_week = day_of_week;
        self
    }

    /// Sets the week and day numbers within the liturgical season.
    pub fn with_season_position(mut self, week_of_season: u32, day_of_season: u32) -> Self {
        self.week_of_season = Some(week_of_season);
        self.day_of_season = Some(day_of_season);
        self
    }

    /// Sets the nth occurrence of this day of the week within the current month.
    pub fn with_nth_day_of_week_in_month(mut self, nth: u8) -> Self {
        self.nth_day_of_week_in_month = nth;
        self
    }

    /// Sets the liturgical year boundaries for this liturgical day.
    pub fn with_liturgical_year_boundaries(
        mut self,
        start_of_liturgical_year: String,
        end_of_liturgical_year: String,
    ) -> Self {
        self.start_of_liturgical_year = start_of_liturgical_year;
        self.end_of_liturgical_year = end_of_liturgical_year;
        self
    }

    /// Sets the current liturgical season boundaries for this liturgical day.
    pub fn with_season_boundaries(
        mut self,
        start_of_season: String,
        end_of_season: String,
    ) -> Self {
        self.start_of_season = Some(start_of_season);
        self.end_of_season = Some(end_of_season);
        self
    }

    /// Sets the boolean flag for holy day of obligation.
    pub fn with_is_holy_day_of_obligation(mut self, is_holy_day_of_obligation: bool) -> Self {
        self.is_holy_day_of_obligation = is_holy_day_of_obligation;
        self
    }

    /// Sets the boolean flag for optional.
    pub fn with_is_optional(mut self, is_optional: bool) -> Self {
        self.is_optional = is_optional;
        self
    }

    /// Sets the boolean flag for allowing similar rank items.
    pub fn with_allow_similar_rank_items(mut self, allow_similar_rank_items: bool) -> Self {
        self.allow_similar_rank_items = allow_similar_rank_items;
        self
    }

    /// Sets the parent overrides for this liturgical day.
    pub fn with_parent_overrides(mut self, parent_overrides: Vec<ParentOverride>) -> Self {
        self.parent_overrides = parent_overrides;
        self
    }

    /// Adds a parent override to this liturgical day.
    pub fn add_parent_override(&mut self, parent_override: ParentOverride) {
        self.parent_overrides.push(parent_override);
    }

    /// Gets the localized name of the liturgical day.
    pub fn get_display_name(&self) -> &str {
        &self.fullname
    }

    /// Gets the date of the liturgical day.
    pub fn get_date(&self) -> &str {
        &self.date
    }

    /// Checks if this liturgical day is a holy day of obligation.
    pub fn is_holy_day(&self) -> bool {
        self.is_holy_day_of_obligation
    }

    /// Checks if this liturgical day is optional.
    pub fn is_optional_day(&self) -> bool {
        self.is_optional
    }

    /// Gets the number of parent overrides for this liturgical day.
    pub fn parent_override_count(&self) -> usize {
        self.parent_overrides.len()
    }
}
