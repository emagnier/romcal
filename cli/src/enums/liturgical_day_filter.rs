use clap::ValueEnum;
use serde::{Deserialize, Serialize};

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
