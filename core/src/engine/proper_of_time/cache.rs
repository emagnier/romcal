//! Cache for Proper of Time date calculations.
//!
//! This module provides a cache for season start/end dates and liturgical
//! year boundaries to avoid repeated calculations during calendar generation.

use chrono::DateTime;
use chrono::Datelike;
use chrono::Utc;

use crate::engine::dates::LiturgicalDates;
use crate::error::RomcalResult;
use crate::romcal::Romcal;
use crate::types::liturgical::{Season, SundayCycle, WeekdayCycle};
use crate::types::{CalendarContext, EasterCalculationType};

/// Key for caching season data based on romcal configuration and year
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RomcalCacheKey {
    context: CalendarContext,
    easter_calculation_type: EasterCalculationType,
    epiphany_on_sunday: bool,
    ascension_on_sunday: bool,
    corpus_christi_on_sunday: bool,
    year: i32,
}

impl From<(&Romcal, i32)> for RomcalCacheKey {
    fn from((romcal, year): (&Romcal, i32)) -> Self {
        Self {
            context: romcal.context,
            easter_calculation_type: romcal.easter_calculation_type,
            epiphany_on_sunday: romcal.epiphany_on_sunday,
            ascension_on_sunday: romcal.ascension_on_sunday,
            corpus_christi_on_sunday: romcal.corpus_christi_on_sunday,
            year,
        }
    }
}

/// Cache for season start dates and liturgical years
/// Valid only for a specific romcal configuration and year
#[derive(Debug, Clone)]
pub struct ProperOfTimeCache {
    key: RomcalCacheKey,
    // Liturgical years
    advent_year: i32,
    christmas_year: i32,
    lent_year: i32,
    triduum_year: i32,
    easter_year: i32,
    ordinary_year: i32,

    /// Liturgical year start date when in context of the season of Christmas from January 1st (not before) to the end of Ordinary Time
    christmas_context_year_start: String,

    /// Liturgical year start date when the season is Advent or early Christmas (until December 31)
    advent_context_year_start: String,

    /// Liturgical year end date when in context of the season of Christmas from January 1st (not before) to the end of Ordinary Time
    christmas_context_year_end: String,

    /// Liturgical year end date when the season is Advent or early Christmas (until December 31)
    advent_context_year_end: String,

    // Season start dates
    advent_start: DateTime<Utc>,
    christmas_start: DateTime<Utc>,
    christmas_start_from_new_year: DateTime<Utc>,
    lent_start: DateTime<Utc>,
    triduum_start: DateTime<Utc>,
    easter_start: DateTime<Utc>,
    ordinary_start: DateTime<Utc>,

    // Season end dates
    advent_end: DateTime<Utc>,
    christmas_end: DateTime<Utc>,
    christmas_end_from_new_year: DateTime<Utc>,
    lent_end: DateTime<Utc>,
    triduum_end: DateTime<Utc>,
    easter_end: DateTime<Utc>,
    ordinary_early_end: DateTime<Utc>,
    ordinary_late_end: DateTime<Utc>,

    // Cycles
    sunday_cycle: SundayCycle,
    weekday_cycle: WeekdayCycle,
}

impl ProperOfTimeCache {
    /// Create a new season cache for the given romcal config and year
    pub fn new(romcal: &Romcal, year: i32) -> RomcalResult<Self> {
        let key = RomcalCacheKey::from((romcal, year));
        let dates = LiturgicalDates::new(romcal.clone(), year)?;

        // Calculate liturgical years
        let advent_year = if romcal.context == CalendarContext::Liturgical {
            year - 1
        } else {
            year
        };
        let christmas_year = if romcal.context == CalendarContext::Liturgical {
            year - 1
        } else {
            year
        };
        let lent_year = year;
        let easter_year = year;
        let triduum_year = year;
        let ordinary_year = year;

        let advent_context_year_start = dates
            .get_first_sunday_of_advent_date(Some(advent_year))
            .format("%Y-%m-%d")
            .to_string();
        let christmas_context_year_start = dates
            .get_first_sunday_of_advent_date(Some(year - 1))
            .format("%Y-%m-%d")
            .to_string();

        let advent_context_year_end = dates
            .get_first_sunday_of_advent_date(Some(advent_year + 1))
            .checked_sub_signed(chrono::Duration::days(1))
            .unwrap()
            .format("%Y-%m-%d")
            .to_string();
        let christmas_context_year_end = dates
            .get_first_sunday_of_advent_date(Some(year))
            .checked_sub_signed(chrono::Duration::days(1))
            .unwrap()
            .format("%Y-%m-%d")
            .to_string();

        // Calculate season start dates
        let advent_start = dates
            .get_sunday_of_advent_date(1, Some(advent_year))
            .ok_or(crate::error::RomcalError::CalculationError)?;

        let christmas_start = dates.get_christmas_date(Some(christmas_year));

        let christmas_start_from_new_year = if romcal.context == CalendarContext::Liturgical {
            christmas_start
        } else {
            dates.get_christmas_date(Some(christmas_year - 1))
        };

        let lent_start = dates.get_ash_wednesday_date(Some(lent_year));

        let triduum_start = dates.get_holy_thursday_date(Some(triduum_year));

        let easter_start = dates.get_easter_sunday_date(Some(easter_year))?;

        let ordinary_start = dates
            .get_baptism_of_the_lord_date(Some(ordinary_year))
            .checked_add_signed(chrono::Duration::days(1))
            .unwrap();

        // Calculate season end dates
        // Advent ends on December 24 (Christmas Eve)
        let advent_end = christmas_start
            .checked_sub_signed(chrono::Duration::days(1))
            .unwrap();

        // Christmas Time ends on the day of the Baptism of the Lord
        let christmas_end = dates.get_baptism_of_the_lord_date(Some(christmas_year + 1));
        let christmas_end_from_new_year = if romcal.context == CalendarContext::Liturgical {
            christmas_end
        } else {
            dates.get_baptism_of_the_lord_date(Some(christmas_year))
        };

        // Lent ends on Holy Thursday (the day before the evening Mass of the Lord's Supper)
        let lent_end = triduum_start;

        // Paschal Triduum ends on Easter Sunday evening (but Easter Sunday itself starts Easter Time)
        let triduum_end = easter_start;

        // Easter Time ends on Pentecost Sunday
        let easter_end = dates.get_pentecost_sunday_date(Some(easter_year));

        // Ordinary Time (early) ends on the day before Ash Wednesday
        let ordinary_early_end = lent_start
            .checked_sub_signed(chrono::Duration::days(1))
            .unwrap();

        // Ordinary Time (late) ends on the Saturday before the 1st Sunday of Advent
        let ordinary_late_end = dates
            .get_first_sunday_of_advent_date(Some(ordinary_year))
            .checked_sub_signed(chrono::Duration::days(1))
            .unwrap();

        Ok(Self {
            key,
            advent_year,
            christmas_year,
            lent_year,
            triduum_year,
            easter_year,
            ordinary_year,
            christmas_context_year_start,
            advent_context_year_start,
            christmas_context_year_end,
            advent_context_year_end,
            advent_start,
            christmas_start_from_new_year,
            christmas_start,
            lent_start,
            triduum_start,
            easter_start,
            ordinary_start,
            advent_end,
            christmas_end,
            christmas_end_from_new_year,
            lent_end,
            triduum_end,
            easter_end,
            ordinary_early_end,
            ordinary_late_end,
            sunday_cycle: SundayCycle::from_year(year),
            weekday_cycle: WeekdayCycle::from_year(year),
        })
    }

    /// Check if this cache is valid for the given romcal config and year
    pub fn is_valid_for(&self, romcal: &Romcal, year: i32) -> bool {
        self.key == RomcalCacheKey::from((romcal, year))
    }

    // Getters for liturgical years
    pub fn advent_year(&self) -> i32 {
        self.advent_year
    }
    pub fn christmas_year(&self) -> i32 {
        self.christmas_year
    }
    pub fn lent_year(&self) -> i32 {
        self.lent_year
    }
    pub fn easter_year(&self) -> i32 {
        self.easter_year
    }
    pub fn triduum_year(&self) -> i32 {
        self.triduum_year
    }
    pub fn ordinary_year(&self) -> i32 {
        self.ordinary_year
    }

    // Accessors for liturgical year start dates
    pub fn liturgical_year_start(&self, season: Season, date: DateTime<Utc>) -> String {
        if season == Season::Advent
            || (season == Season::ChristmasTime && [11u32, 12u32].contains(&date.month()))
        {
            self.advent_context_year_start.clone()
        } else {
            self.christmas_context_year_start.clone()
        }
    }

    // Accessors for liturgical year end dates
    pub fn liturgical_year_end(&self, season: Season, date: DateTime<Utc>) -> String {
        if season == Season::Advent
            || (season == Season::ChristmasTime && [11u32, 12u32].contains(&date.month()))
        {
            self.advent_context_year_end.clone()
        } else {
            self.christmas_context_year_end.clone()
        }
    }

    /// Determine season start date based on season
    pub fn start_of_seasons(&self, season: Season, date: DateTime<Utc>) -> String {
        let start_of_season = match season {
            Season::Advent => self.advent_start,
            Season::ChristmasTime => {
                if date.month() == 1 {
                    self.christmas_start_from_new_year
                } else {
                    self.christmas_start
                }
            }
            Season::Lent => self.lent_start,
            Season::EasterTime => self.easter_start,
            Season::PaschalTriduum => self.triduum_start,
            Season::OrdinaryTime => self.ordinary_start,
        };
        start_of_season.format("%Y-%m-%d").to_string()
    }

    /// Determine season end date based on season
    pub fn end_of_seasons(&self, season: Season, date: DateTime<Utc>) -> String {
        let end_of_season = match season {
            Season::Advent => self.advent_end,
            Season::ChristmasTime => {
                if date.month() == 1 {
                    self.christmas_end_from_new_year
                } else {
                    self.christmas_end
                }
            }
            Season::Lent => self.lent_end,
            Season::EasterTime => self.easter_end,
            Season::PaschalTriduum => self.triduum_end,
            Season::OrdinaryTime => {
                // Determine if we're in early or late Ordinary Time
                if date < self.lent_start {
                    self.ordinary_early_end
                } else {
                    self.ordinary_late_end
                }
            }
        };
        end_of_season.format("%Y-%m-%d").to_string()
    }

    // Getters for season start dates
    pub fn advent_start(&self) -> DateTime<Utc> {
        self.advent_start
    }
    pub fn christmas_start(&self) -> DateTime<Utc> {
        self.christmas_start
    }
    pub fn lent_start(&self) -> DateTime<Utc> {
        self.lent_start
    }
    pub fn easter_start(&self) -> DateTime<Utc> {
        self.easter_start
    }
    pub fn triduum_start(&self) -> DateTime<Utc> {
        self.triduum_start
    }

    // Getters for cycles
    pub fn sunday_cycle(&self) -> SundayCycle {
        self.sunday_cycle
    }
    pub fn weekday_cycle(&self) -> WeekdayCycle {
        self.weekday_cycle
    }
}
