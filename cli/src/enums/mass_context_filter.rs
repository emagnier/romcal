use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Available filters for MassContext properties
#[derive(Debug, Clone, PartialEq, Eq, ValueEnum, Serialize, Deserialize)]
pub enum MassContextFilter {
    /// The type of mass (e.g., DayMass, EasterVigil)
    MassTime,
    /// The localized name of the mass time
    MassTimeName,
    /// The civil calendar date (YYYY-MM-DD)
    CivilDate,
    /// The liturgical date (YYYY-MM-DD)
    LiturgicalDate,
    /// The liturgical season
    Season,
    /// The localized season name
    SeasonName,
    /// The Sunday cycle (Year A, B, or C)
    SundayCycle,
    /// The localized Sunday cycle name
    SundayCycleName,
    /// The weekday cycle (Year 1 or 2)
    WeekdayCycle,
    /// The localized weekday cycle name
    WeekdayCycleName,
    /// The psalter week cycle (Week 1-4)
    PsalterWeek,
    /// The localized psalter week name
    PsalterWeekName,
    /// The week number within the liturgical season
    WeekOfSeason,
    /// The day number within the liturgical season
    DayOfSeason,
    /// The day of the week
    DayOfWeek,
    /// The liturgical periods
    Periods,
    /// The first day of the current liturgical season
    StartOfSeason,
    /// The last day of the current liturgical season
    EndOfSeason,
    /// The first day of the liturgical year
    StartOfLiturgicalYear,
    /// The last day of the liturgical year
    EndOfLiturgicalYear,
    /// The unique identifier of the liturgical day
    Id,
    /// The full name of the liturgical day
    Fullname,
    /// The liturgical precedence
    Precedence,
    /// The liturgical rank
    Rank,
    /// The localized liturgical rank name
    RankName,
    /// The liturgical colors
    Colors,
    /// The common prayers/readings used
    Commons,
    /// The entities linked to this day
    Entities,
    /// The titles for this liturgical day
    Titles,
    /// Whether this is a holy day of obligation
    IsHolyDayOfObligation,
    /// Whether this liturgical day is optional
    IsOptional,
    /// The ID of the calendar where this day is defined
    FromCalendarId,
    /// Optional alternative celebrations
    OptionalCelebrations,
}

impl MassContextFilter {
    /// Get the field name as a string
    pub fn field_name(&self) -> &'static str {
        match self {
            MassContextFilter::MassTime => "mass_time",
            MassContextFilter::MassTimeName => "mass_time_name",
            MassContextFilter::CivilDate => "civil_date",
            MassContextFilter::LiturgicalDate => "liturgical_date",
            MassContextFilter::Season => "season",
            MassContextFilter::SeasonName => "season_name",
            MassContextFilter::SundayCycle => "sunday_cycle",
            MassContextFilter::SundayCycleName => "sunday_cycle_name",
            MassContextFilter::WeekdayCycle => "weekday_cycle",
            MassContextFilter::WeekdayCycleName => "weekday_cycle_name",
            MassContextFilter::PsalterWeek => "psalter_week",
            MassContextFilter::PsalterWeekName => "psalter_week_name",
            MassContextFilter::WeekOfSeason => "week_of_season",
            MassContextFilter::DayOfSeason => "day_of_season",
            MassContextFilter::DayOfWeek => "day_of_week",
            MassContextFilter::Periods => "periods",
            MassContextFilter::StartOfSeason => "start_of_season",
            MassContextFilter::EndOfSeason => "end_of_season",
            MassContextFilter::StartOfLiturgicalYear => "start_of_liturgical_year",
            MassContextFilter::EndOfLiturgicalYear => "end_of_liturgical_year",
            MassContextFilter::Id => "id",
            MassContextFilter::Fullname => "fullname",
            MassContextFilter::Precedence => "precedence",
            MassContextFilter::Rank => "rank",
            MassContextFilter::RankName => "rank_name",
            MassContextFilter::Colors => "colors",
            MassContextFilter::Commons => "commons",
            MassContextFilter::Entities => "entities",
            MassContextFilter::Titles => "titles",
            MassContextFilter::IsHolyDayOfObligation => "is_holy_day_of_obligation",
            MassContextFilter::IsOptional => "is_optional",
            MassContextFilter::FromCalendarId => "from_calendar_id",
            MassContextFilter::OptionalCelebrations => "optional_celebrations",
        }
    }
}

impl fmt::Display for MassContextFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.field_name())
    }
}

/// Helper function to parse a filter string into a MassContextFilter
fn parse_filter(s: &str) -> Result<MassContextFilter, String> {
    match s {
        "mass_time" => Ok(MassContextFilter::MassTime),
        "mass_time_name" => Ok(MassContextFilter::MassTimeName),
        "civil_date" => Ok(MassContextFilter::CivilDate),
        "liturgical_date" => Ok(MassContextFilter::LiturgicalDate),
        "season" => Ok(MassContextFilter::Season),
        "season_name" => Ok(MassContextFilter::SeasonName),
        "sunday_cycle" => Ok(MassContextFilter::SundayCycle),
        "sunday_cycle_name" => Ok(MassContextFilter::SundayCycleName),
        "weekday_cycle" => Ok(MassContextFilter::WeekdayCycle),
        "weekday_cycle_name" => Ok(MassContextFilter::WeekdayCycleName),
        "psalter_week" => Ok(MassContextFilter::PsalterWeek),
        "psalter_week_name" => Ok(MassContextFilter::PsalterWeekName),
        "week_of_season" => Ok(MassContextFilter::WeekOfSeason),
        "day_of_season" => Ok(MassContextFilter::DayOfSeason),
        "day_of_week" => Ok(MassContextFilter::DayOfWeek),
        "periods" => Ok(MassContextFilter::Periods),
        "start_of_season" => Ok(MassContextFilter::StartOfSeason),
        "end_of_season" => Ok(MassContextFilter::EndOfSeason),
        "start_of_liturgical_year" => Ok(MassContextFilter::StartOfLiturgicalYear),
        "end_of_liturgical_year" => Ok(MassContextFilter::EndOfLiturgicalYear),
        "id" => Ok(MassContextFilter::Id),
        "fullname" => Ok(MassContextFilter::Fullname),
        "precedence" => Ok(MassContextFilter::Precedence),
        "rank" => Ok(MassContextFilter::Rank),
        "rank_name" => Ok(MassContextFilter::RankName),
        "colors" => Ok(MassContextFilter::Colors),
        "commons" => Ok(MassContextFilter::Commons),
        "entities" => Ok(MassContextFilter::Entities),
        "titles" => Ok(MassContextFilter::Titles),
        "is_holy_day_of_obligation" => Ok(MassContextFilter::IsHolyDayOfObligation),
        "is_optional" => Ok(MassContextFilter::IsOptional),
        "from_calendar_id" => Ok(MassContextFilter::FromCalendarId),
        "optional_celebrations" => Ok(MassContextFilter::OptionalCelebrations),
        _ => Err(format!("Unknown filter: {}", s)),
    }
}

/// Wrapper type to handle both dash and underscore formats for MassContextFilter
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MassContextFilterWrapper(pub MassContextFilter);

impl FromStr for MassContextFilterWrapper {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Convert dashes to underscores for matching
        let normalized = s.replace('-', "_");
        let filter = parse_filter(&normalized)?;
        Ok(MassContextFilterWrapper(filter))
    }
}

impl fmt::Display for MassContextFilterWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.field_name())
    }
}

impl From<MassContextFilterWrapper> for MassContextFilter {
    fn from(wrapper: MassContextFilterWrapper) -> Self {
        wrapper.0
    }
}
