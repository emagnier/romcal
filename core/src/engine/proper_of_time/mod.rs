//! Proper of Time generation for the liturgical calendar.
//!
//! This module generates the temporal cycle of the liturgical year,
//! including Advent, Christmas Time, Ordinary Time, Lent, Paschal Triduum,
//! and Easter Time.

use chrono::{DateTime, Datelike, Utc};

pub mod advent;
pub mod cache;
pub mod christmas_time;
pub mod easter_time;
pub mod lent;
pub mod ordinary_time;
pub mod paschal_triduum;
pub mod utils;

use self::advent::Advent;
use self::cache::ProperOfTimeCache;
use self::christmas_time::ChristmasTime;
use self::easter_time::EasterTime;
use self::lent::Lent;
use self::ordinary_time::OrdinaryTime;
use self::paschal_triduum::PaschalTriduum;
use self::utils::{PROPER_OF_TIME_ID, enum_to_string, sort_liturgical_days_by_date};
use crate::engine::dates::LiturgicalDates;
use crate::engine::liturgical_day::LiturgicalDay;
use crate::engine::template_resolver::{ProperOfTimeDayType, TemplateResolver};
use crate::error::RomcalResult;
use crate::martyrology_resolution::MartyrologyResolver;
use crate::romcal::Romcal;
use crate::types::dates::{DateDef, DayOfWeek};
use crate::types::liturgical::{
    Color, ColorInfo, Period, PeriodInfo, Precedence, PsalterWeekCycle, Rank, Season,
};

/// Structure for generating liturgical days of the Proper of Time
pub struct ProperOfTime {
    romcal: Romcal,
    dates: LiturgicalDates,
    cache: ProperOfTimeCache,
    template_resolver: Option<TemplateResolver>,
    martyrology_resolver: MartyrologyResolver,
}

impl ProperOfTime {
    /// Creates a new instance of ProperOfTime
    ///
    /// # Arguments
    ///
    /// * `romcal` - Romcal instance
    /// * `year` - Liturgical year
    ///
    /// # Errors
    ///
    /// Returns an error if the year is invalid
    pub fn new(romcal: Romcal, year: i32) -> RomcalResult<Self> {
        use self::cache::ProperOfTimeCache;
        let liturgical_dates = LiturgicalDates::new(romcal.clone(), year)?;
        let cache = ProperOfTimeCache::new(&romcal, year)?;

        // Create template resolver from locale metadata
        let template_resolver = Self::create_template_resolver(&romcal);

        // Create martyrology resolver to resolve fullnames for martyrology-based days
        let martyrology_resolver = MartyrologyResolver::new(&romcal);

        Ok(Self {
            romcal,
            dates: liturgical_dates,
            cache,
            template_resolver,
            martyrology_resolver,
        })
    }

    /// Creates a TemplateResolver from the romcal's locale resources.
    ///
    /// Looks for metadata in the target locale first, then falls back to 'en'.
    ///
    /// Priority for ordinal_format:
    /// 1. `metadata.ordinal_format` (locale-specific setting)
    /// 2. `romcal.ordinal_format` (user-defined or default)
    fn create_template_resolver(romcal: &Romcal) -> Option<TemplateResolver> {
        let locale = &romcal.locale;

        // Try target locale first
        if let Some(resources) = romcal.get_resources(locale)
            && let Some(metadata) = resources.metadata.clone()
        {
            // Resolve ordinal_format: metadata > romcal
            let ordinal_format = metadata.ordinal_format.unwrap_or(romcal.ordinal_format);
            return Some(TemplateResolver::new(
                metadata,
                locale.clone(),
                ordinal_format,
            ));
        }

        // Fall back to 'en' if target locale has no metadata
        if locale != "en"
            && let Some(resources) = romcal.get_resources("en")
            && let Some(metadata) = resources.metadata.clone()
        {
            // Resolve ordinal_format: metadata > romcal
            let ordinal_format = metadata.ordinal_format.unwrap_or(romcal.ordinal_format);
            return Some(TemplateResolver::new(
                metadata,
                "en".to_string(),
                ordinal_format,
            ));
        }

        None
    }

    /// Creates a liturgical day with common properties
    fn create_liturgical_day_base(
        &self,
        id: &str,
        date: DateTime<Utc>,
        precedence: Precedence,
        season: Option<Season>,
        color: Color,
        day_type: Option<&ProperOfTimeDayType>,
    ) -> LiturgicalDay {
        let id = id.to_string();
        let date_str = date.format("%Y-%m-%d").to_string();
        let dow = date.weekday().num_days_from_sunday() as u8;
        let rank = precedence.to_rank();
        let sunday_cycle = self.cache.sunday_cycle();
        let weekday_cycle = self.cache.weekday_cycle();

        // Resolve fullname with priority: 1) Martyrology, 2) Template, 3) ID fallback
        let fullname = self
            .martyrology_resolver
            .get_fullname_for_day(&id, None)
            .or_else(|| {
                day_type.and_then(|dt| {
                    self.template_resolver
                        .as_ref()
                        .map(|r| r.resolve_proper_of_time_fullname(dt))
                })
            })
            .unwrap_or_else(|| id.clone());

        // Calculate season-related fields only if season is provided
        let (day_of_season, week_of_season, psalter_week_cycle) = if let Some(season) = season {
            let start_of_season = match season {
                Season::Advent => self.cache.advent_start(),
                Season::ChristmasTime => self.cache.christmas_start(),
                Season::Lent => self.cache.lent_start(),
                Season::EasterTime => self.cache.easter_start(),
                Season::PaschalTriduum => self.cache.triduum_start(),
                Season::OrdinaryTime => {
                    // For Ordinary Time, we need to determine if it's early or late
                    // This is a simplified approach - in practice, this should be determined by the calling function
                    self.cache.easter_start() // Default to Easter start for now
                }
            };

            // Calculate day_of_season and week_of_season automatically
            let days_since_start = (date.date_naive() - start_of_season.date_naive()).num_days();
            let day_of_season = if days_since_start < 0 {
                0
            } else {
                (days_since_start + 1) as u32
            };

            // Special logic for Lent: if day_of_season < 5, week_of_season starts at 0
            let week_of_season = if season == Season::Lent && day_of_season < 5 {
                0
            } else if day_of_season == 0 {
                // Should never happen if day_of_season is calculated correctly
                1
            } else {
                (day_of_season - 1) / 7 + 1
            };

            (
                Some(day_of_season),
                Some(week_of_season),
                PsalterWeekCycle::from_week(
                    week_of_season,
                    season == Season::Lent,
                    season == Season::ChristmasTime,
                ),
            )
        } else {
            (None, None, PsalterWeekCycle::Week_1)
        };

        // Resolve localized names using template resolver
        let rank_name = self
            .template_resolver
            .as_ref()
            .map(|r| r.get_rank(&enum_to_string(&rank)))
            .unwrap_or_else(|| enum_to_string(&rank));

        let sunday_cycle_name = self
            .template_resolver
            .as_ref()
            .map(|r| r.get_cycle(&enum_to_string(&sunday_cycle)))
            .unwrap_or_else(|| enum_to_string(&sunday_cycle));

        let weekday_cycle_name = self
            .template_resolver
            .as_ref()
            .map(|r| r.get_cycle(&enum_to_string(&weekday_cycle)))
            .unwrap_or_else(|| enum_to_string(&weekday_cycle));

        let psalter_week_name = self
            .template_resolver
            .as_ref()
            .map(|r| r.get_cycle(&enum_to_string(&psalter_week_cycle)))
            .unwrap_or_else(|| enum_to_string(&psalter_week_cycle));

        let mut liturgical_day = LiturgicalDay::new(
            id.clone(),
            fullname,
            date_str,
            DateDef::MonthDate {
                month: crate::types::dates::MonthIndex(1), // January
                date: 1,
                day_offset: None,
            },
            precedence,
            rank.clone(),
            rank_name,
            sunday_cycle,
            sunday_cycle_name,
            weekday_cycle,
            weekday_cycle_name,
            psalter_week_cycle,
            psalter_week_name,
            PROPER_OF_TIME_ID.to_string(),
        )
        .with_day_of_week(DayOfWeek(dow))
        .with_is_holy_day_of_obligation(dow == 0 && rank == Rank::Solemnity);

        // Set season-related fields if season is provided
        if let Some(season) = season {
            let season_name = self
                .template_resolver
                .as_ref()
                .map(|r| r.get_season_name(&enum_to_string(&season)))
                .unwrap_or_else(|| enum_to_string(&season));

            liturgical_day = liturgical_day
                .with_seasons(season)
                .with_season_name(season_name)
                .with_start_of_season(self.cache.start_of_seasons(season, date))
                .with_end_of_season(self.cache.end_of_seasons(season, date))
                .with_liturgical_year_boundaries(
                    self.cache.liturgical_year_start(season, date),
                    self.cache.liturgical_year_end(season, date),
                );
        }

        // Set season position if calculated
        if let (Some(week), Some(day)) = (week_of_season, day_of_season) {
            liturgical_day = liturgical_day.with_season_position(week, day);
        }

        // Color with localized name
        let color_name = self
            .template_resolver
            .as_ref()
            .map(|r| r.get_color(&enum_to_string(&color)))
            .unwrap_or_else(|| enum_to_string(&color));

        liturgical_day.colors = vec![ColorInfo {
            key: color.clone(),
            name: color_name,
        }];

        liturgical_day.date_def = DateDef::InheritedFromProperOfTime {};

        liturgical_day
    }

    /// Converts a list of Period enums to PeriodInfo with localized names.
    ///
    /// Uses the TemplateResolver to get localized names for each period.
    /// Falls back to the enum string representation if no translation is found.
    pub fn resolve_periods(&self, periods: Vec<Period>) -> Vec<PeriodInfo> {
        periods
            .into_iter()
            .map(|period| {
                let period_key = enum_to_string(&period);
                let name = self
                    .template_resolver
                    .as_ref()
                    .map(|r| r.get_period(&period_key))
                    .unwrap_or_else(|| period_key.clone());
                PeriodInfo { key: period, name }
            })
            .collect()
    }

    /// Generates all liturgical days of the Proper of Time for the liturgical year
    pub fn generate_all(&self) -> RomcalResult<Vec<LiturgicalDay>> {
        let mut days = Vec::new();

        let advent = Advent::new(self);
        let christmas_time = ChristmasTime::new(self);
        let ordinary_time = OrdinaryTime::new(self);
        let lent = Lent::new(self);
        let paschal_triduum = PaschalTriduum::new(self);
        let easter_time = EasterTime::new(self);

        if self.romcal.context == crate::CalendarContext::Liturgical {
            days.extend(advent.generate()?);
            days.extend(christmas_time.generate_early()?);
        }

        days.extend(christmas_time.generate_late()?);
        days.extend(ordinary_time.generate_early()?);
        days.extend(lent.generate()?);
        days.extend(paschal_triduum.generate()?);
        days.extend(easter_time.generate()?);
        days.extend(ordinary_time.generate_late()?);

        if self.romcal.context == crate::CalendarContext::Gregorian {
            days.extend(advent.generate()?);
            days.extend(christmas_time.generate_early()?);
        }

        // TODO: Temporary fix to sort days by date
        sort_liturgical_days_by_date(&mut days);

        Ok(days)
    }
}

#[cfg(test)]
mod tests;
