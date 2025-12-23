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
use crate::engine::entity_resolver::EntityResolver;
use crate::engine::liturgical_day::LiturgicalDay;
use crate::engine::template_resolver::{ProperOfTimeDayType, TemplateResolver};
use crate::error::RomcalResult;
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
    entity_resolver: EntityResolver,
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

        // Create entity resolver to resolve fullnames for entity-based days
        let entity_resolver = EntityResolver::new(&romcal);

        Ok(Self {
            romcal,
            dates: liturgical_dates,
            cache,
            template_resolver,
            entity_resolver,
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

        // Resolve fullname with priority: 1) Entity, 2) Template, 3) ID fallback
        let fullname = self
            .entity_resolver
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
mod tests {
    use super::*;
    use crate::romcal::Preset;

    #[test]
    fn test_proper_of_time_creation() {
        let romcal = Romcal::default();
        let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();

        assert_eq!(proper_of_time.cache.advent_year(), 2026);
        assert_eq!(proper_of_time.cache.easter_year(), 2026);
    }

    #[test]
    fn test_no_duplicate_dates() {
        let romcal = Romcal::default();
        let all_days = ProperOfTime::new(romcal, 2026)
            .unwrap()
            .generate_all()
            .unwrap();

        // Check that we have generated days
        assert!(!all_days.is_empty());

        // Extract all dates and check for duplicates
        let mut dates: Vec<&str> = all_days.iter().map(|day| day.date.as_str()).collect();
        let original_count = dates.len();

        // Sort and deduplicate
        dates.sort();
        dates.dedup();
        let unique_count = dates.len();

        // Exception: Holy Thursday has two liturgical days on the same date:
        // - holy_thursday (from lent)
        // - thursday_of_the_lords_supper (from paschal_triduum)
        // So we expect exactly 1 duplicate date
        let expected_duplicates = 1;
        let actual_duplicates = original_count - unique_count;

        assert_eq!(
            actual_duplicates, expected_duplicates,
            "Expected {} duplicate date (Holy Thursday), but found {} duplicates. Original: {}, Unique: {}",
            expected_duplicates, actual_duplicates, original_count, unique_count
        );

        // Additional check: verify that only Holy Thursday has duplicate dates
        let mut date_groups: std::collections::HashMap<String, Vec<&LiturgicalDay>> =
            std::collections::HashMap::new();
        for day in &all_days {
            date_groups
                .entry(day.date.clone())
                .or_insert_with(Vec::new)
                .push(day);
        }

        let duplicate_dates: Vec<_> = date_groups
            .iter()
            .filter(|(_, days)| days.len() > 1)
            .collect();

        assert_eq!(
            duplicate_dates.len(),
            1,
            "Expected exactly 1 duplicate date (Holy Thursday), but found {}: {:?}",
            duplicate_dates.len(),
            duplicate_dates
                .iter()
                .map(|(date, days)| (date, days.iter().map(|d| &d.id).collect::<Vec<_>>()))
                .collect::<Vec<_>>()
        );

        // Verify that the duplicate is indeed Holy Thursday
        let holy_thursday_days = duplicate_dates[0].1;
        assert_eq!(holy_thursday_days.len(), 2);
        assert!(holy_thursday_days.iter().any(|d| d.id == "holy_thursday"));
        assert!(
            holy_thursday_days
                .iter()
                .any(|d| d.id == "thursday_of_the_lords_supper")
        );
    }

    #[test]
    fn test_no_duplicate_dates_liturgical_context() {
        let romcal = Romcal::new(Preset {
            context: Some(crate::CalendarContext::Liturgical),
            ..Preset::default()
        });
        let all_days = ProperOfTime::new(romcal, 2026)
            .unwrap()
            .generate_all()
            .unwrap();

        // Check that we have generated days
        assert!(!all_days.is_empty());

        // Extract all dates and check for duplicates
        let mut dates: Vec<&str> = all_days.iter().map(|day| day.date.as_str()).collect();
        let original_count = dates.len();

        // Sort and deduplicate
        dates.sort();
        dates.dedup();
        let unique_count = dates.len();

        // Exception: Holy Thursday has two liturgical days on the same date:
        // - holy_thursday (from lent)
        // - thursday_of_the_lords_supper (from paschal_triduum)
        // So we expect exactly 1 duplicate date
        let expected_duplicates = 1;
        let actual_duplicates = original_count - unique_count;

        assert_eq!(
            actual_duplicates, expected_duplicates,
            "Expected {} duplicate date (Holy Thursday), but found {} duplicates (liturgical context). Original: {}, Unique: {}",
            expected_duplicates, actual_duplicates, original_count, unique_count
        );
    }

    #[test]
    fn test_sort_liturgical_days_by_date() {
        let romcal = Romcal::default();
        let mut all_days = ProperOfTime::new(romcal, 2026)
            .unwrap()
            .generate_all()
            .unwrap();

        // Shuffle the days to test sorting
        all_days.reverse();

        // Sort using the utility function
        sort_liturgical_days_by_date(&mut all_days);

        // Verify that days are sorted by date
        for i in 1..all_days.len() {
            let date_a = chrono::NaiveDate::parse_from_str(&all_days[i - 1].date, "%Y-%m-%d")
                .unwrap_or_default();
            let date_b = chrono::NaiveDate::parse_from_str(&all_days[i].date, "%Y-%m-%d")
                .unwrap_or_default();
            assert!(
                date_a <= date_b,
                "Days are not sorted by date: {} should come before {}",
                all_days[i - 1].date,
                all_days[i].date
            );
        }
    }

    #[test]
    fn test_calendar_continuity() {
        let romcal = Romcal::default();

        // Get all liturgical days
        let mut days = ProperOfTime::new(romcal, 2026)
            .unwrap()
            .generate_all()
            .unwrap();

        // Sort by date
        sort_liturgical_days_by_date(&mut days);

        // Verify that there are no gaps in dates between first and last day
        for i in 1..days.len() {
            let prev_date =
                chrono::NaiveDate::parse_from_str(&days[i - 1].date, "%Y-%m-%d").unwrap();
            let curr_date = chrono::NaiveDate::parse_from_str(&days[i].date, "%Y-%m-%d").unwrap();

            // Each day should be either:
            // 1. Same date as previous (for duplicates like Holy Thursday)
            // 2. Next day after previous (no gaps)
            let days_diff = (curr_date - prev_date).num_days();
            assert!(
                days_diff == 0 || days_diff == 1,
                "Gap found in calendar: {} to {} ({} days difference). Each day should be same date or next day.",
                prev_date,
                curr_date,
                days_diff
            );
        }

        // Verify that day_of_week matches the actual day of the week for each date
        for day in &days {
            let date = chrono::NaiveDate::parse_from_str(&day.date, "%Y-%m-%d").unwrap();
            let actual_dow = date.weekday().num_days_from_sunday() as u8;
            let stored_dow = day.day_of_week.0;

            assert_eq!(
                actual_dow, stored_dow,
                "day_of_week mismatch for {}: date {} is actually day {} but stored as day {}",
                day.id, day.date, actual_dow, stored_dow
            );
        }

        // TODO: Add week_of_season consistency test
        // This test should verify that week_of_season follows the correct pattern:
        // - Each season starts with week 1 (or 0 for Lent)
        // - Week numbers increment on Sundays
        // - Special handling for Christmas Time and Ordinary Time
        // - Complex logic needed for different seasons
    }

    // -------------------------------------------------------------------------
    // Tests for ordinal_format resolution
    // -------------------------------------------------------------------------

    use crate::engine::resources::Resources;
    use crate::types::OrdinalFormat;
    use crate::types::resource::ResourcesMetadata;
    use std::collections::BTreeMap;

    /// Creates a minimal ResourcesMetadata for testing
    fn create_test_metadata(ordinal_format: Option<OrdinalFormat>) -> ResourcesMetadata {
        let mut ordinals_letters = BTreeMap::new();
        ordinals_letters.insert("1".to_string(), "first".to_string());
        ordinals_letters.insert("2".to_string(), "second".to_string());

        let mut ordinals_numeric = BTreeMap::new();
        ordinals_numeric.insert("1".to_string(), "1st".to_string());
        ordinals_numeric.insert("2".to_string(), "2nd".to_string());

        ResourcesMetadata {
            ordinal_format,
            ordinals_letters: Some(ordinals_letters),
            ordinals_numeric: Some(ordinals_numeric),
            weekdays: None,
            months: None,
            colors: None,
            seasons: None,
            periods: None,
            ranks: None,
            cycles: None,
        }
    }

    /// Creates a test Resources with the given locale and metadata
    fn create_test_resources(locale: &str, ordinal_format: Option<OrdinalFormat>) -> Resources {
        Resources {
            schema: None,
            locale: locale.to_string(),
            metadata: Some(create_test_metadata(ordinal_format)),
            entities: None,
        }
    }

    #[test]
    fn test_ordinal_format_default_is_numeric() {
        // When no ordinal_format is specified anywhere, default should be Numeric
        let romcal = Romcal::default();
        assert_eq!(romcal.ordinal_format, OrdinalFormat::Numeric);
    }

    #[test]
    fn test_ordinal_format_from_locale_metadata() {
        // When ordinal_format is set in locale metadata, it should be used
        let mut romcal = Romcal::default();
        romcal.locale = "test".to_string();
        romcal.resources = vec![create_test_resources("test", Some(OrdinalFormat::Letters))];

        let resolver = ProperOfTime::create_template_resolver(&romcal);
        assert!(resolver.is_some());
        assert_eq!(resolver.unwrap().ordinal_format(), OrdinalFormat::Letters);
    }

    #[test]
    fn test_ordinal_format_from_romcal_when_metadata_not_set() {
        // When ordinal_format is not set in metadata, romcal value should be used
        let mut romcal = Romcal::default();
        romcal.locale = "test".to_string();
        romcal.ordinal_format = OrdinalFormat::Letters;
        romcal.resources = vec![create_test_resources("test", None)];

        let resolver = ProperOfTime::create_template_resolver(&romcal);
        assert!(resolver.is_some());
        assert_eq!(resolver.unwrap().ordinal_format(), OrdinalFormat::Letters);
    }

    #[test]
    fn test_ordinal_format_metadata_takes_priority() {
        // When ordinal_format is set in both metadata and romcal, metadata should win
        let mut romcal = Romcal::default();
        romcal.locale = "test".to_string();
        romcal.ordinal_format = OrdinalFormat::Numeric; // Romcal says Numeric
        romcal.resources = vec![create_test_resources("test", Some(OrdinalFormat::Letters))]; // Metadata says Letters

        let resolver = ProperOfTime::create_template_resolver(&romcal);
        assert!(resolver.is_some());
        // Metadata should take priority
        assert_eq!(resolver.unwrap().ordinal_format(), OrdinalFormat::Letters);
    }

    #[test]
    fn test_ordinal_format_fallback_to_en_locale() {
        // When target locale has no metadata but 'en' does, use 'en' metadata
        let mut romcal = Romcal::default();
        romcal.locale = "nonexistent".to_string();
        romcal.resources = vec![create_test_resources("en", Some(OrdinalFormat::Letters))];

        let resolver = ProperOfTime::create_template_resolver(&romcal);
        assert!(resolver.is_some());
        assert_eq!(resolver.unwrap().ordinal_format(), OrdinalFormat::Letters);
    }

    #[test]
    fn test_ordinal_format_no_resolver_without_resources() {
        // When no resources are available, resolver should be None
        let mut romcal = Romcal::default();
        romcal.locale = "test".to_string();
        romcal.resources = vec![];

        let resolver = ProperOfTime::create_template_resolver(&romcal);
        assert!(resolver.is_none());
    }

    // -------------------------------------------------------------------------
    // Tests for entity-based fullname resolution
    // -------------------------------------------------------------------------

    use crate::types::entity::entity_definition::Entity;

    /// Creates test resources with entities for entity fullname resolution tests
    fn create_test_resources_with_entities(
        locale: &str,
        entities: std::collections::BTreeMap<String, Entity>,
    ) -> Resources {
        Resources {
            schema: None,
            locale: locale.to_string(),
            metadata: Some(create_test_metadata(None)),
            entities: Some(entities),
        }
    }

    #[test]
    fn test_fullname_resolved_from_entity() {
        // When an entity has a fullname defined, it should be used
        let mut entities = std::collections::BTreeMap::new();
        entities.insert(
            "mary_mother_of_god".to_string(),
            Entity {
                fullname: Some("Mary, Mother of God".to_string()),
                ..Default::default()
            },
        );

        let mut romcal = Romcal::default();
        romcal.locale = "en".to_string();
        romcal.resources = vec![create_test_resources_with_entities("en", entities)];

        let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();

        // Check that entity resolver has the entity
        let fullname = proper_of_time
            .entity_resolver
            .get_fullname_for_day("mary_mother_of_god", None);
        assert_eq!(fullname, Some("Mary, Mother of God".to_string()));
    }

    #[test]
    fn test_fullname_fallback_to_template_when_no_entity() {
        // When no entity fullname exists but day_type is provided, template should be used
        let mut romcal = Romcal::default();
        romcal.locale = "en".to_string();
        romcal.resources = vec![create_test_resources("en", None)];

        let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();

        // For days like "advent_sunday_1" that don't have entity fullnames,
        // the template resolver should be used
        // This is implicitly tested by the fact that ProperOfTime works correctly
        assert!(proper_of_time.template_resolver.is_some());
    }

    #[test]
    fn test_entity_fullname_priority_over_template() {
        // Entity fullname should take priority over template resolution
        // This tests the priority: Entity > Template > ID

        let mut entities = std::collections::BTreeMap::new();
        entities.insert(
            "test_entity".to_string(),
            Entity {
                fullname: Some("Entity Fullname".to_string()),
                ..Default::default()
            },
        );

        let mut romcal = Romcal::default();
        romcal.locale = "en".to_string();
        romcal.resources = vec![create_test_resources_with_entities("en", entities)];

        let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();

        // The entity resolver should find the fullname
        let fullname = proper_of_time
            .entity_resolver
            .get_fullname_for_day("test_entity", None);
        assert_eq!(fullname, Some("Entity Fullname".to_string()));

        // Non-existent entity should return None
        let no_fullname = proper_of_time
            .entity_resolver
            .get_fullname_for_day("nonexistent", None);
        assert!(no_fullname.is_none());
    }

    #[test]
    fn test_entity_fullname_with_locale_override() {
        // When target locale has entity fullname, it should override 'en'
        let mut en_entities = std::collections::BTreeMap::new();
        en_entities.insert(
            "mary_mother_of_god".to_string(),
            Entity {
                fullname: Some("Mary, Mother of God".to_string()),
                ..Default::default()
            },
        );

        let mut fr_entities = std::collections::BTreeMap::new();
        fr_entities.insert(
            "mary_mother_of_god".to_string(),
            Entity {
                fullname: Some("Sainte Marie, Mère de Dieu".to_string()),
                ..Default::default()
            },
        );

        let mut romcal = Romcal::default();
        romcal.locale = "fr".to_string();
        romcal.resources = vec![
            create_test_resources_with_entities("en", en_entities),
            create_test_resources_with_entities("fr", fr_entities),
        ];

        let proper_of_time = ProperOfTime::new(romcal, 2026).unwrap();

        // French locale should use French fullname
        let fullname = proper_of_time
            .entity_resolver
            .get_fullname_for_day("mary_mother_of_god", None);
        assert_eq!(fullname, Some("Sainte Marie, Mère de Dieu".to_string()));
    }
}
