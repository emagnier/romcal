use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Available filters for LiturgicalDay properties
#[derive(Debug, Clone, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
pub enum LiturgicalDayFilter {
    /// The unique identifier of the liturgical day
    Id,
    /// The full name of the liturgical day
    Fullname,
    /// The computed date of the liturgical day
    Date,
    /// The date definition for this liturgical day
    DateDef,
    /// The date definition exceptions for this liturgical day
    DateExceptions,
    /// The liturgical precedence for this liturgical day
    Precedence,
    /// The liturgical rank for this liturgical day
    Rank,
    /// The localized liturgical rank for this liturgical day
    RankName,
    /// Allows similar items with the same rank and same or lower precedence
    AllowSimilarRankItems,
    /// Holy days of obligation
    IsHolyDayOfObligation,
    /// Indicates if this liturgical day is optional
    IsOptional,
    /// The liturgical seasons to which this liturgical day belongs
    Seasons,
    /// The liturgical periods to which this liturgical day belongs
    Periods,
    /// The common prayers, readings, and chants
    Commons,
    /// The liturgical colors for this liturgical day
    Colors,
    /// The titles for this liturgical day
    Titles,
    /// The entities linked to this liturgical day
    Entities,
    /// The week number of the current liturgical season
    WeekOfSeason,
    /// The day number within the current liturgical season
    DayOfSeason,
    /// The day of the week for this liturgical day
    DayOfWeek,
    /// The nth occurrence of this day of the week within the current month
    NthDayOfWeekInMonth,
    /// The first day of the current liturgical season
    StartOfSeason,
    /// The last day of the current liturgical season
    EndOfSeason,
    /// The first day of the current liturgical year
    StartOfLiturgicalYear,
    /// The last day of the current liturgical year
    EndOfLiturgicalYear,
    /// The Sunday cycle to which this liturgical day belongs
    SundayCycle,
    /// The weekday cycle to which this liturgical day belongs
    WeekdayCycle,
    /// The psalter week cycle to which this liturgical day belongs
    PsalterWeek,
    /// The ID of the calendar where this liturgical day is defined
    FromCalendarId,
    /// The parent overrides for this liturgical day
    ParentOverrides,
}

impl LiturgicalDayFilter {
    /// Get the field name as a string
    pub fn field_name(&self) -> &'static str {
        match self {
            LiturgicalDayFilter::Id => "id",
            LiturgicalDayFilter::Fullname => "fullname",
            LiturgicalDayFilter::Date => "date",
            LiturgicalDayFilter::DateDef => "date_def",
            LiturgicalDayFilter::DateExceptions => "date_exceptions",
            LiturgicalDayFilter::Precedence => "precedence",
            LiturgicalDayFilter::Rank => "rank",
            LiturgicalDayFilter::RankName => "rank_name",
            LiturgicalDayFilter::AllowSimilarRankItems => "allow_similar_rank_items",
            LiturgicalDayFilter::IsHolyDayOfObligation => "is_holy_day_of_obligation",
            LiturgicalDayFilter::IsOptional => "is_optional",
            LiturgicalDayFilter::Seasons => "seasons",
            LiturgicalDayFilter::Periods => "periods",
            LiturgicalDayFilter::Commons => "commons",
            LiturgicalDayFilter::Colors => "colors",
            LiturgicalDayFilter::Titles => "titles",
            LiturgicalDayFilter::Entities => "entities",
            LiturgicalDayFilter::WeekOfSeason => "week_of_season",
            LiturgicalDayFilter::DayOfSeason => "day_of_season",
            LiturgicalDayFilter::DayOfWeek => "day_of_week",
            LiturgicalDayFilter::NthDayOfWeekInMonth => "nth_day_of_week_in_month",
            LiturgicalDayFilter::StartOfSeason => "start_of_season",
            LiturgicalDayFilter::EndOfSeason => "end_of_season",
            LiturgicalDayFilter::StartOfLiturgicalYear => "start_of_liturgical_year",
            LiturgicalDayFilter::EndOfLiturgicalYear => "end_of_liturgical_year",
            LiturgicalDayFilter::SundayCycle => "sunday_cycle",
            LiturgicalDayFilter::WeekdayCycle => "weekday_cycle",
            LiturgicalDayFilter::PsalterWeek => "psalter_week",
            LiturgicalDayFilter::FromCalendarId => "from_calendar_id",
            LiturgicalDayFilter::ParentOverrides => "parent_overrides",
        }
    }
}

impl fmt::Display for LiturgicalDayFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display with underscores (preferred format)
        write!(f, "{}", self.field_name())
    }
}

/// Helper function to parse a filter string into a LiturgicalDayFilter
fn parse_filter(s: &str) -> Result<LiturgicalDayFilter, String> {
    match s {
        "id" => Ok(LiturgicalDayFilter::Id),
        "fullname" => Ok(LiturgicalDayFilter::Fullname),
        "date" => Ok(LiturgicalDayFilter::Date),
        "date_def" => Ok(LiturgicalDayFilter::DateDef),
        "date_exceptions" => Ok(LiturgicalDayFilter::DateExceptions),
        "precedence" => Ok(LiturgicalDayFilter::Precedence),
        "rank" => Ok(LiturgicalDayFilter::Rank),
        "rank_name" => Ok(LiturgicalDayFilter::RankName),
        "allow_similar_rank_items" => Ok(LiturgicalDayFilter::AllowSimilarRankItems),
        "is_holy_day_of_obligation" => Ok(LiturgicalDayFilter::IsHolyDayOfObligation),
        "is_optional" => Ok(LiturgicalDayFilter::IsOptional),
        "seasons" => Ok(LiturgicalDayFilter::Seasons),
        "periods" => Ok(LiturgicalDayFilter::Periods),
        "commons" => Ok(LiturgicalDayFilter::Commons),
        "colors" => Ok(LiturgicalDayFilter::Colors),
        "titles" => Ok(LiturgicalDayFilter::Titles),
        "entities" => Ok(LiturgicalDayFilter::Entities),
        "week_of_season" => Ok(LiturgicalDayFilter::WeekOfSeason),
        "day_of_season" => Ok(LiturgicalDayFilter::DayOfSeason),
        "day_of_week" => Ok(LiturgicalDayFilter::DayOfWeek),
        "nth_day_of_week_in_month" => Ok(LiturgicalDayFilter::NthDayOfWeekInMonth),
        "start_of_season" => Ok(LiturgicalDayFilter::StartOfSeason),
        "end_of_season" => Ok(LiturgicalDayFilter::EndOfSeason),
        "start_of_liturgical_year" => Ok(LiturgicalDayFilter::StartOfLiturgicalYear),
        "end_of_liturgical_year" => Ok(LiturgicalDayFilter::EndOfLiturgicalYear),
        "sunday_cycle" => Ok(LiturgicalDayFilter::SundayCycle),
        "weekday_cycle" => Ok(LiturgicalDayFilter::WeekdayCycle),
        "psalter_week" => Ok(LiturgicalDayFilter::PsalterWeek),
        "from_calendar_id" => Ok(LiturgicalDayFilter::FromCalendarId),
        "parent_overrides" => Ok(LiturgicalDayFilter::ParentOverrides),
        _ => Err(format!("Unknown filter: {}", s)),
    }
}

/// Wrapper type to handle both dash and underscore formats for LiturgicalDayFilter
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiturgicalDayFilterWrapper(pub LiturgicalDayFilter);

impl FromStr for LiturgicalDayFilterWrapper {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Convert dashes to underscores for matching
        let normalized = s.replace('-', "_");
        let filter = parse_filter(&normalized)?;
        Ok(LiturgicalDayFilterWrapper(filter))
    }
}

impl fmt::Display for LiturgicalDayFilterWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.field_name())
    }
}

impl From<LiturgicalDayFilterWrapper> for LiturgicalDayFilter {
    fn from(wrapper: LiturgicalDayFilterWrapper) -> Self {
        wrapper.0
    }
}
