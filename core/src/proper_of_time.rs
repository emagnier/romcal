use chrono::{DateTime, Datelike, Utc};

use crate::dates::LiturgicalDates;
use crate::error::RomcalResult;
use crate::liturgical_day::LiturgicalDay;
use crate::preset::Preset;
use crate::proper_of_time_cache::ProperOfTimeCache;
use crate::types::dates::{DateDef, DateFn, DayOfWeek};
use crate::types::liturgical::{
    Color, ColorInfo, Period, PeriodInfo, Precedence, Rank, Season, SeasonInfo,
};

/// Helper function to convert an enum to its string representation using Serde
fn enum_to_string<T>(value: &T) -> String
where
    T: serde::Serialize,
{
    serde_json::to_string(value)
        .unwrap_or_default()
        .trim_matches('"')
        .to_string()
}

/// Macro to create a liturgical day with common properties
/// Reduces code duplication in create_* functions
macro_rules! create_liturgical_day_base {
    ($id:expr, $date:expr, $precedence:expr, $rank:expr, $season:expr, $color:expr, $cache:expr) => {{
        let id = $id.to_string();
        let date_str = $date.format("%Y-%m-%d").to_string();
        let dow = $date.weekday().num_days_from_sunday() as u8;

        // Determine season start date based on season
        let start_of_season = match $season {
            Season::Advent => $cache.advent_start(),
            Season::ChristmasTime => $cache.christmas_start(),
            Season::Lent => $cache.lent_start(),
            Season::EasterTime => $cache.easter_start(),
            Season::PaschalTriduum => $cache.triduum_start(),
            Season::OrdinaryTime => {
                // For Ordinary Time, we need to determine if it's early or late
                // This is a simplified approach - in practice, this should be determined by the calling function
                $cache.easter_start() // Default to Easter start for now
            }
        };

        // Calculate day_of_season and week_of_season automatically
        let days_since_start = ($date.date_naive() - start_of_season.date_naive()).num_days();
        let day_of_season = if days_since_start < 0 {
            0
        } else {
            (days_since_start + 1) as u32
        };

        // Special logic for Lent: if day_of_season < 5, week_of_season starts at 0
        let week_of_season = if $season == Season::Lent && day_of_season < 5 {
            0
        } else if day_of_season == 0 { // Should never happen if day_of_season is calculated correctly
            1
        } else {
            ((day_of_season - 1) / 7 + 1) as u32
        };

        let mut liturgical_day = LiturgicalDay::new(
            id.clone(),
            id.clone(),
            date_str,
            PROPER_OF_TIME_ID.to_string(),
        );

        // Common configuration
        liturgical_day.precedence = $precedence;
        liturgical_day.rank = $rank;
        liturgical_day.rank_name = enum_to_string(&$rank);
        liturgical_day.is_holy_day_of_obligation = dow == 0 && $rank == Rank::Solemnity;
        liturgical_day.day_of_week = DayOfWeek(dow);
        liturgical_day.week_of_season = week_of_season;
        liturgical_day.day_of_season = day_of_season;
        liturgical_day.start_of_season = $cache.start_of_seasons($season, $date);
        liturgical_day.start_of_liturgical_year = $cache.liturgical_year_start($season, $date);
        liturgical_day.end_of_liturgical_year = $cache.liturgical_year_end($season, $date);

        // Season
        liturgical_day.seasons = vec![SeasonInfo {
            key: $season,
            name: enum_to_string(&$season),
        }];

        // Color
        liturgical_day.colors = vec![ColorInfo {
            key: $color,
            name: enum_to_string(&$color),
        }];

        // Date definition (placeholder for now)
        liturgical_day.date_def = DateDef::DateFunction {
            date_fn: DateFn::EasterSunday, // TODO: Calculate proper DateFn
            day_offset: None,
        };

        liturgical_day
    }};
}

// =================================================================================
// UTILITY FUNCTIONS
// =================================================================================

/// Helper function to sort liturgical days by date in chronological order
fn sort_liturgical_days_by_date(days: &mut [LiturgicalDay]) {
    days.sort_by(|a, b| {
        // Parse dates and compare chronologically
        let date_a = chrono::NaiveDate::parse_from_str(&a.date, "%Y-%m-%d").unwrap_or_default();
        let date_b = chrono::NaiveDate::parse_from_str(&b.date, "%Y-%m-%d").unwrap_or_default();
        date_a.cmp(&date_b)
    });
}

/// Calendar ID for the Proper of Time
const PROPER_OF_TIME_ID: &str = "proper_of_time";

/// Weekday names for liturgical day generation
const WEEKDAY_NAMES: [&str; 7] = [
    "sunday",
    "monday",
    "tuesday",
    "wednesday",
    "thursday",
    "friday",
    "saturday",
];

/// Structure for generating liturgical days of the Proper of Time
pub struct ProperOfTime {
    preset: Preset,
    dates: LiturgicalDates,
    cache: ProperOfTimeCache,
}

impl ProperOfTime {
    /// Creates a new instance of ProperOfTime
    ///
    /// # Arguments
    ///
    /// * `config` - Calendar configuration
    /// * `year` - Liturgical year
    ///
    /// # Errors
    ///
    /// Returns an error if the year is invalid
    pub fn new(config: Preset, year: i32) -> RomcalResult<Self> {
        let liturgical_dates = LiturgicalDates::new(config.clone(), year)?;
        let cache = ProperOfTimeCache::new(&config, year)?;
        Ok(Self {
            preset: config,
            dates: liturgical_dates,
            cache,
        })
    }

    // =================================================================================
    // MAIN GENERATION FUNCTION
    // =================================================================================

    /// Generates all liturgical days of the Proper of Time for the liturgical year
    pub fn generate_all(&self) -> RomcalResult<Vec<LiturgicalDay>> {
        let mut days = Vec::new();

        if self.preset.context == crate::CalendarContext::Liturgical {
            days.extend(self.advent()?);
            days.extend(self.early_christmas_time()?);
        }

        days.extend(self.late_christmas_time()?);
        days.extend(self.early_ordinary_time()?);
        days.extend(self.lent()?);
        days.extend(self.paschal_triduum()?);
        days.extend(self.easter_time()?);
        days.extend(self.late_ordinary_time()?);

        if self.preset.context == crate::CalendarContext::Gregorian {
            days.extend(self.advent()?);
            days.extend(self.early_christmas_time()?);
        }

        // TODO: Temporary fix to sort days by date
        sort_liturgical_days_by_date(&mut days);

        Ok(days)
    }

    // =================================================================================
    // SEASON GENERATION FUNCTIONS
    // =================================================================================

    /// Generates liturgical days of Advent
    ///
    /// Advent begins on the first Sunday of Advent and ends on December 24.
    pub fn advent(&self) -> RomcalResult<Vec<LiturgicalDay>> {
        let mut days = Vec::new();

        // Use cached values
        let advent_year = self.cache.advent_year();

        // ADVENT DAY TYPES:
        // 1. Advent Sundays (4 Sundays)
        for week in 1..=4 {
            if let Some(sunday_date) = self
                .dates
                .get_sunday_of_advent_date(week, Some(advent_year))
            {
                let day = self.create_advent_sunday(week, sunday_date, &self.cache)?;
                days.push(day);
            }
        }

        // 2. Advent Weekdays (Monday-Saturday, weeks 1-3)
        for week in 1..=3 {
            for dow in 1..=6 {
                // Monday to Saturday
                if let Some(weekday_date) =
                    self.dates
                        .unprivileged_weekday_of_advent(dow, week, Some(advent_year))
                {
                    let day = self.create_advent_weekday(week, dow, weekday_date, &self.cache)?;
                    days.push(day);
                }
            }
        }

        // 3. Privileged Advent Weekdays (December 17-24)
        // Calculate the first Sunday once to avoid recalculating it for each day
        for day in 17..=24 {
            if let Some(privileged_date) = self
                .dates
                .privileged_weekday_of_advent(day, Some(advent_year))
            {
                // Calculate the correct week based on the date
                // December 17-24 can span both week 3 and week 4 of Advent
                let liturgical_day =
                    self.create_privileged_advent_weekday(day, privileged_date, &self.cache)?;
                days.push(liturgical_day);
            }
        }

        Ok(days)
    }

    /// Generates liturgical days of early Christmas Time
    ///
    /// Early Christmas Time includes:
    /// - The Nativity of the Lord (December 25)
    /// - Octave of Christmas (December 26-31, excluding December 25 and January 1)
    /// - The Holy Family (Sunday within the Octave)
    ///
    /// # Arguments
    ///
    /// * `christmas_year` - The effective year for Christmas (already calculated based on context)
    pub fn early_christmas_time(&self) -> RomcalResult<Vec<LiturgicalDay>> {
        let mut days = Vec::new();

        // Use cached values
        let christmas_year = self.cache.christmas_year();
        let christmas_date = self.cache.christmas_start();

        // EARLY CHRISTMAS TIME DAY TYPES:
        // 1. The Nativity of the Lord (December 25)
        let day = self.create_nativity_of_the_lord(christmas_date, &self.cache)?;
        days.push(day);

        // 2. Octave of Christmas (December 26-31, excluding December 25 and January 1)
        for count in 2..=7 {
            if let Some(octave_date) = self
                .dates
                .get_weekday_within_octave_of_christmas_date(count, Some(christmas_year))
            {
                let day = self.create_christmas_octave_day(count, octave_date, &self.cache)?;
                days.push(day);
            }
        }

        // 3. The Holy Family (Sunday within the Octave)
        let holy_family_date = self.dates.get_holy_family_date(Some(christmas_year));

        let day = self.create_holy_family(holy_family_date, &self.cache)?;
        days.push(day);

        Ok(days)
    }

    /// Generates liturgical days of late Christmas Time
    ///
    /// Late Christmas Time includes:
    /// - Mary, Mother of God (January 1)
    /// - Second Sunday after Christmas (if it exists)
    /// - Weekdays before Epiphany (January 2-8)
    /// - The Epiphany of the Lord
    /// - Weekdays after Epiphany
    /// - The Baptism of the Lord
    ///
    /// # Arguments
    ///
    /// * `christmas_year` - The effective year for late Christmas Time (current year for liturgical context)
    pub fn late_christmas_time(&self) -> RomcalResult<Vec<LiturgicalDay>> {
        let mut days = Vec::new();

        // Use cached values
        let christmas_year = if self.preset.context == crate::CalendarContext::Liturgical {
            self.cache.christmas_year() + 1
        } else {
            self.cache.christmas_year()
        };

        // LATE CHRISTMAS TIME DAY TYPES:
        // 1. Mary, Mother of God (January 1)
        let mary_mother_of_god_date = self.dates.get_mary_mother_of_god_date(Some(christmas_year));

        let day = self.create_mary_mother_of_god(mary_mother_of_god_date, &self.cache)?;
        days.push(day);

        // 2. Second Sunday after Christmas (if it exists)
        let second_sunday_date = self
            .dates
            .second_sunday_after_christmas(Some(christmas_year));
        if let Some(second_sunday_date) = second_sunday_date {
            let day = self.create_second_sunday_after_christmas(second_sunday_date, &self.cache)?;
            days.push(day);
        }

        // 3. Weekdays before Epiphany (January 2-8)
        for day_num in 2..=8 {
            if let Some(weekday_date) = self
                .dates
                .get_weekday_before_epiphany_date(day_num, Some(christmas_year))
            {
                let liturgical_day =
                    self.create_weekday_before_epiphany(day_num, weekday_date, &self.cache)?;
                days.push(liturgical_day);
            }
        }

        // 4. The Epiphany of the Lord
        let epiphany_date = self.dates.get_epiphany_date(Some(christmas_year));

        let day = self.create_epiphany_of_the_lord(epiphany_date, &self.cache)?;
        days.push(day);

        // 5. Weekdays after Epiphany
        for dow in 1..=6 {
            if let Some(weekday_date) = self
                .dates
                .get_weekday_after_epiphany_date(dow, Some(christmas_year))
            {
                let liturgical_day =
                    self.create_weekday_after_epiphany(dow, weekday_date, &self.cache)?;
                days.push(liturgical_day);
            }
        }

        // 6. The Baptism of the Lord
        let baptism_date = self
            .dates
            .get_baptism_of_the_lord_date(Some(christmas_year));

        let day = self.create_baptism_of_the_lord(baptism_date, &self.cache)?;
        days.push(day);

        Ok(days)
    }

    /// Generates liturgical days of Lent
    ///
    /// Lent includes:
    /// - Ash Wednesday
    /// - Days after Ash Wednesday (Thursday-Saturday)
    /// - All days from 1st Sunday of Lent to Saturday of 5th week of Lent
    /// - Palm Sunday of the Passion of the Lord
    /// - Holy Week (Monday-Thursday)
    ///
    /// # Arguments
    ///
    /// * `lent_year` - The effective year for Lent (current year for liturgical context)
    pub fn lent(&self) -> RomcalResult<Vec<LiturgicalDay>> {
        let mut days = Vec::new();

        // Use cached values
        let lent_year = self.cache.lent_year();
        let ash_wednesday_date = self.cache.lent_start();

        // LENT DAY TYPES:
        // 1. Ash Wednesday
        let day = self.create_ash_wednesday(ash_wednesday_date, &self.cache)?;
        days.push(day);

        // 2. Days after Ash Wednesday (Thursday-Saturday)
        for dow in 4..=6 {
            let weekday_date = ash_wednesday_date + chrono::Duration::days((dow - 3) as i64);
            let liturgical_day =
                self.create_weekday_after_ash_wednesday(dow, weekday_date, &self.cache)?;
            days.push(liturgical_day);
        }

        // 3. All days from 1st Sunday of Lent to Saturday of 5th week of Lent
        for i in 0..35 {
            let week = (i / 7) + 1;
            let dow = (i - (week - 1) * 7) as u8;

            let weekday_date = ash_wednesday_date + chrono::Duration::days((i + 4) as i64);
            let liturgical_day = self.create_lent_weekday(week, dow, weekday_date, &self.cache)?;
            days.push(liturgical_day);
        }

        // 4. Palm Sunday of the Passion of the Lord
        let palm_sunday_date = self.dates.get_palm_sunday_date(Some(lent_year));
        let day = self.create_palm_sunday(palm_sunday_date, &self.cache)?;
        days.push(day);

        // 5. Holy Week (Monday-Thursday)
        for dow in 1..=4 {
            let weekday_date = palm_sunday_date + chrono::Duration::days(dow as i64);
            let liturgical_day = self.create_holy_week_weekday(dow, weekday_date, &self.cache)?;
            days.push(liturgical_day);
        }

        Ok(days)
    }

    /// Generates liturgical days of the Paschal Triduum
    ///
    /// The Paschal Triduum includes:
    /// - Thursday of the Lord's Supper (Holy Thursday)
    /// - Friday of the Passion of the Lord (Good Friday)
    /// - Holy Saturday
    /// - Easter Sunday of the Resurrection of the Lord
    ///
    /// # Arguments
    ///
    /// * `triduum_year` - The effective year for the Paschal Triduum (current year for liturgical context)
    pub fn paschal_triduum(&self) -> RomcalResult<Vec<LiturgicalDay>> {
        let mut days = Vec::new();

        // Use cached values
        let triduum_year = self.cache.triduum_year();
        let holy_thursday_date = self.cache.triduum_start();

        // PASCHAL TRIDUUM DAY TYPES:
        // 1. Thursday of the Lord's Supper (Holy Thursday)
        let day = self.create_holy_thursday(holy_thursday_date, &self.cache)?;
        days.push(day);

        // 2. Friday of the Passion of the Lord (Good Friday)
        let good_friday_date = self.dates.get_good_friday_date(Some(triduum_year));
        let day = self.create_good_friday(good_friday_date, &self.cache)?;
        days.push(day);

        // 3. Holy Saturday
        let holy_saturday_date = self.dates.get_holy_saturday_date(Some(triduum_year));
        let day = self.create_holy_saturday(holy_saturday_date, &self.cache)?;
        days.push(day);

        // 4. Easter Sunday of the Resurrection of the Lord
        let easter_sunday_date = self.dates.get_easter_sunday_date(Some(triduum_year))?;
        let day = self.create_easter_sunday(easter_sunday_date, &self.cache)?;
        days.push(day);

        Ok(days)
    }

    /// Generates liturgical days of Easter Time
    ///
    /// Easter Time includes:
    /// - Octave of Easter (Monday-Saturday after Easter Sunday)
    /// - Divine Mercy Sunday (Second Sunday of Easter)
    /// - Weekdays and Sundays of Easter Time (2nd Monday to 7th Saturday)
    /// - Ascension of the Lord (6th week, Thursday)
    /// - Pentecost Sunday
    pub fn easter_time(&self) -> RomcalResult<Vec<LiturgicalDay>> {
        let mut days = Vec::new();

        // Use cached values
        let easter_year = self.cache.easter_year();

        // EASTER TIME DAY TYPES:
        // 1. Octave of Easter (Monday-Saturday after Easter Sunday)
        let easter_sunday_date = self.cache.easter_start();
        for dow in 1..=6 {
            let octave_date = easter_sunday_date + chrono::Duration::days(dow as i64);
            let liturgical_day = self.create_easter_octave_day(dow, octave_date, &self.cache)?;
            days.push(liturgical_day);
        }

        // 2. Divine Mercy Sunday (Second Sunday of Easter)
        let divine_mercy_date = self.dates.get_divine_mercy_sunday_date(Some(easter_year));
        let day = self.create_divine_mercy_sunday(divine_mercy_date, &self.cache)?;
        days.push(day);

        // 3. All days from 2nd Monday to 7th Saturday of Easter Time
        for i in 8..49 {
            let week = (i / 7) + 1;
            let dow = i - (week - 1) * 7;

            let weekday_date = easter_sunday_date + chrono::Duration::days(i as i64);

            // Special case: Ascension of the Lord
            // If ascension_on_sunday is false: 6th week, Thursday (39 days after Easter)
            // If ascension_on_sunday is true: 7th week, Sunday (42 days after Easter)
            let ascension_date = self.dates.get_ascension_date(Some(easter_year));
            let is_ascension_day = if self.preset.ascension_on_sunday {
                week == 7 && dow == 0 // 7th week, Sunday
            } else {
                week == 6 && dow == 4 // 6th week, Thursday
            };

            if is_ascension_day {
                let liturgical_day =
                    self.create_ascension_of_the_lord(ascension_date, &self.cache)?;
                days.push(liturgical_day);
            } else {
                let liturgical_day =
                    self.create_easter_time_weekday(week, dow, weekday_date, &self.cache)?;
                days.push(liturgical_day);
            }
        }

        // 4. Pentecost Sunday
        let pentecost_date = self.dates.get_pentecost_sunday_date(Some(easter_year));
        let day = self.create_pentecost_sunday(pentecost_date, &self.cache)?;
        days.push(day);

        Ok(days)
    }

    /// Generates liturgical days of early Ordinary Time
    ///
    /// Early Ordinary Time includes:
    /// - All Sundays and weekdays from the day after the Baptism of the Lord to the day before Ash Wednesday
    /// - Special days: Sunday of the Word of God (3rd week)
    ///
    /// Note: The first week of early Ordinary Time may be incomplete (no Sunday, possibly no Monday)
    /// because the Sunday is either Epiphany or Baptism of the Lord, and Monday may be missing
    /// if Baptism of the Lord falls on Monday.
    ///
    /// # Arguments
    ///
    /// * `ordinary_year` - The effective year for Ordinary Time (current year for liturgical context)
    pub fn early_ordinary_time(&self) -> RomcalResult<Vec<LiturgicalDay>> {
        let mut days = Vec::new();

        // Use cached values
        let ordinary_year = self.cache.easter_year(); // Same as easter year for early ordinary time

        // All days of early Ordinary Time
        let early_ordinary_dates = self
            .dates
            .get_all_dates_of_early_ordinary_time(Some(ordinary_year));

        // Find the first Sunday in early Ordinary Time to calculate weeks correctly
        let first_sunday = early_ordinary_dates
            .iter()
            .find(|date| date.weekday() == chrono::Weekday::Sun)
            .copied()
            .unwrap_or_else(|| early_ordinary_dates[0]);

        for ordinary_date in early_ordinary_dates.iter() {
            let dow = ordinary_date.weekday().num_days_from_sunday() as u8;

            // Calculate week number using the specialized function
            let week = self.calculate_ordinary_time_week(*ordinary_date, first_sunday, true) as u8;

            // Special cases for specific Sundays
            if week == 3 && dow == 0 {
                // Sunday of the Word of God (3rd week)
                let liturgical_day =
                    self.create_sunday_of_the_word_of_god(*ordinary_date, &self.cache)?;
                days.push(liturgical_day);
            } else {
                // Regular Ordinary Time day
                let liturgical_day =
                    self.create_ordinary_time_day(week, dow, *ordinary_date, &self.cache)?;
                days.push(liturgical_day);
            }
        }

        Ok(days)
    }

    /// Generates liturgical days of late Ordinary Time
    ///
    /// Late Ordinary Time includes:
    /// - The Most Holy Trinity (Trinity Sunday)
    /// - The Most Holy Body and Blood of Christ (Corpus Christi)
    /// - The Most Sacred Heart of Jesus
    /// - All Sundays and weekdays from the day after Pentecost to the day before the First Sunday of Advent
    /// - Special days: Christ the King (34th week)
    ///
    /// Note: The first week of late Ordinary Time is incomplete (Monday to Saturday only)
    /// because the Sunday is Pentecost Sunday. All subsequent weeks are complete.
    ///
    /// # Arguments
    ///
    /// * `ordinary_year` - The effective year for Ordinary Time (current year for liturgical context)
    pub fn late_ordinary_time(&self) -> RomcalResult<Vec<LiturgicalDay>> {
        let mut days = Vec::new();

        // Use cached values
        let ordinary_year = self.cache.easter_year(); // Same as easter year for late ordinary time

        // Get solemnity dates for later use
        let trinity_date = self.dates.get_trinity_sunday_date(Some(ordinary_year));
        let corpus_christi_date = self.dates.get_corpus_christi_date(Some(ordinary_year));
        let sacred_heart_date = self
            .dates
            .get_most_sacred_heart_of_jesus_date(Some(ordinary_year));

        // 4. All days of late Ordinary Time
        let late_ordinary_dates = self
            .dates
            .get_all_dates_of_late_ordinary_time(Some(ordinary_year));

        // In late Ordinary Time, the first week is incomplete (Monday to Saturday, no Sunday)
        // because the Sunday is Pentecost Sunday. Then all weeks are complete until the last week (34th)
        let first_sunday_idx = late_ordinary_dates
            .iter()
            .position(|date| date.weekday() == chrono::Weekday::Sun)
            .unwrap_or(0);

        // Calculate how many complete weeks we have after the next Sunday after the Pentecost Sunday
        let complete_weeks_after_first_sunday = (late_ordinary_dates.len() - first_sunday_idx) / 7;
        // We need to end at week 34, so calculate the starting week
        let late_start_week = 34 - complete_weeks_after_first_sunday;

        for (i, ordinary_date) in late_ordinary_dates.iter().enumerate() {
            let dow = ordinary_date.weekday().num_days_from_sunday() as u8;

            // Calculate week number based on the first Sunday
            let week = if i < first_sunday_idx {
                // Days before the first Sunday are in the first incomplete week
                late_start_week
            } else {
                // Calculate week from the next Sunday after the Pentecost Sunday (which starts the complete weeks of the late Ordinary Time)
                late_start_week + 1 + ((i - first_sunday_idx) / 7)
            } as u8;

            // Check if this date is a solemnity and create the appropriate liturgical day
            if *ordinary_date == trinity_date {
                // Trinity Sunday
                let liturgical_day = self.create_most_holy_trinity(*ordinary_date, &self.cache)?;
                days.push(liturgical_day);
            } else if *ordinary_date == corpus_christi_date {
                // Corpus Christi
                let liturgical_day =
                    self.create_most_holy_body_and_blood_of_christ(*ordinary_date, &self.cache)?;
                days.push(liturgical_day);
            } else if *ordinary_date == sacred_heart_date {
                // Sacred Heart
                let liturgical_day =
                    self.create_most_sacred_heart_of_jesus(*ordinary_date, &self.cache)?;
                days.push(liturgical_day);
            } else if week == 34 && dow == 0 {
                // Christ the King (34th week)
                let liturgical_day = self.create_our_lord_jesus_christ_king_of_the_universe(
                    *ordinary_date,
                    &self.cache,
                )?;
                days.push(liturgical_day);
            } else {
                // Regular Ordinary Time day
                let liturgical_day =
                    self.create_ordinary_time_day(week, dow, *ordinary_date, &self.cache)?;
                days.push(liturgical_day);
            }
        }

        Ok(days)
    }

    // =================================================================================
    // LITURGICAL DAY CREATION FUNCTIONS
    // =================================================================================

    // ---------------------------------------------------------------------------------
    // ADVENT DAY CREATION FUNCTIONS
    // ---------------------------------------------------------------------------------

    /// Calculates the week number for Ordinary Time based on the first Sunday
    /// Handles the complex logic for incomplete first weeks
    fn calculate_ordinary_time_week(
        &self,
        date: DateTime<Utc>,
        first_sunday: DateTime<Utc>,
        is_early: bool,
    ) -> u32 {
        let days_since_first_sunday = (date.date_naive() - first_sunday.date_naive()).num_days();

        if days_since_first_sunday < 0 {
            // Days before the first Sunday are in week 1 (incomplete week)
            1
        } else {
            // Calculate week from the first Sunday
            let week = (days_since_first_sunday / 7) + 1;

            if is_early {
                // Early Ordinary Time: first Sunday is week 2, so add 1 to the calculated week
                (week + 1) as u32
            } else {
                // Late Ordinary Time: first Sunday is week 1
                week as u32
            }
        }
    }

    /// Creates an Advent Sunday
    fn create_advent_sunday(
        &self,
        week: u8,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let mut liturgical_day = create_liturgical_day_base!(
            format!("advent_{}_sunday", week),
            date,
            Precedence::PrivilegedSunday_2,
            Rank::Sunday,
            Season::Advent,
            Color::Purple,
            cache
        );

        // Override specific properties for Advent Sunday
        liturgical_day.is_holy_day_of_obligation = true;

        // Colors (rose for the 3rd Sunday - Gaudete)
        if week == 3 {
            liturgical_day.colors = vec![
                ColorInfo {
                    key: Color::Rose,
                    name: enum_to_string(&Color::Rose),
                },
                ColorInfo {
                    key: Color::Purple,
                    name: enum_to_string(&Color::Purple),
                },
            ];
        }

        Ok(liturgical_day)
    }

    /// Creates an Advent weekday
    fn create_advent_weekday(
        &self,
        week: u8,
        dow: u8,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let id = format!("advent_{}_{}", week, WEEKDAY_NAMES[dow as usize]);
        let liturgical_day = create_liturgical_day_base!(
            id,
            date,
            Precedence::Weekday_13,
            Rank::Weekday,
            Season::Advent,
            Color::Purple,
            cache
        );

        Ok(liturgical_day)
    }

    /// Creates a privileged Advent weekday (December 17-24)
    fn create_privileged_advent_weekday(
        &self,
        day: u8,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let id = format!("advent_december_{}", day);
        let liturgical_day = create_liturgical_day_base!(
            id,
            date,
            Precedence::PrivilegedWeekday_9,
            Rank::Weekday,
            Season::Advent,
            Color::Purple,
            cache
        );

        Ok(liturgical_day)
    }

    // ---------------------------------------------------------------------------------
    // CHRISTMAS TIME DAY CREATION FUNCTIONS
    // ---------------------------------------------------------------------------------

    /// Creates the Nativity of the Lord (December 25)
    fn create_nativity_of_the_lord(
        &self,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let liturgical_day = create_liturgical_day_base!(
            "nativity_of_the_lord",
            date,
            Precedence::ProperOfTimeSolemnity_2,
            Rank::Solemnity,
            Season::ChristmasTime,
            Color::White,
            cache
        );

        Ok(liturgical_day)
    }

    /// Creates a day within the Octave of Christmas
    fn create_christmas_octave_day(
        &self,
        count: u8,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let id = format!("christmas_octave_day_{}", count);
        let liturgical_day = create_liturgical_day_base!(
            id,
            date,
            Precedence::PrivilegedWeekday_9,
            Rank::Weekday,
            Season::ChristmasTime,
            Color::White,
            cache
        );

        Ok(liturgical_day)
    }

    /// Creates the Holy Family
    fn create_holy_family(
        &self,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let liturgical_day = create_liturgical_day_base!(
            "holy_family_of_jesus_mary_and_joseph",
            date,
            Precedence::GeneralLordFeast_5,
            Rank::Feast,
            Season::ChristmasTime,
            Color::White,
            cache
        );

        Ok(liturgical_day)
    }

    // ---------------------------------------------------------------------------------
    // LATE CHRISTMAS TIME DAY CREATION FUNCTIONS
    // ---------------------------------------------------------------------------------

    /// Creates Mary, Mother of God (January 1)
    fn create_mary_mother_of_god(
        &self,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let liturgical_day = create_liturgical_day_base!(
            "mary_mother_of_god",
            date,
            Precedence::GeneralSolemnity_3,
            Rank::Solemnity,
            Season::ChristmasTime,
            Color::White,
            cache
        );

        Ok(liturgical_day)
    }

    /// Creates the Second Sunday after Christmas
    fn create_second_sunday_after_christmas(
        &self,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let liturgical_day = create_liturgical_day_base!(
            "second_sunday_after_christmas",
            date,
            Precedence::UnprivilegedSunday_6,
            Rank::Sunday,
            Season::ChristmasTime,
            Color::White,
            cache
        );

        Ok(liturgical_day)
    }

    /// Creates a weekday before Epiphany
    fn create_weekday_before_epiphany(
        &self,
        day: u8,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let id = format!("christmas_time_january_{}", day);
        let liturgical_day = create_liturgical_day_base!(
            id,
            date,
            Precedence::Weekday_13,
            Rank::Weekday,
            Season::ChristmasTime,
            Color::White,
            cache
        );

        Ok(liturgical_day)
    }

    /// Creates the Epiphany of the Lord
    fn create_epiphany_of_the_lord(
        &self,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let liturgical_day = create_liturgical_day_base!(
            "epiphany_of_the_lord",
            date,
            Precedence::ProperOfTimeSolemnity_2,
            Rank::Solemnity,
            Season::ChristmasTime,
            Color::White,
            cache
        );

        Ok(liturgical_day)
    }

    /// Creates a weekday after Epiphany
    fn create_weekday_after_epiphany(
        &self,
        dow: u8,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let id = format!("{}_after_epiphany", WEEKDAY_NAMES[dow as usize]);
        let liturgical_day = create_liturgical_day_base!(
            id,
            date,
            Precedence::Weekday_13,
            Rank::Weekday,
            Season::ChristmasTime,
            Color::White,
            cache
        );

        Ok(liturgical_day)
    }

    /// Creates the Baptism of the Lord
    fn create_baptism_of_the_lord(
        &self,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let liturgical_day = create_liturgical_day_base!(
            "baptism_of_the_lord",
            date,
            Precedence::ProperOfTimeSolemnity_2,
            Rank::Solemnity,
            Season::ChristmasTime,
            Color::White,
            cache
        );

        Ok(liturgical_day)
    }

    // ---------------------------------------------------------------------------------
    // LENT DAY CREATION FUNCTIONS
    // ---------------------------------------------------------------------------------

    /// Creates Ash Wednesday
    fn create_ash_wednesday(
        &self,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let liturgical_day = create_liturgical_day_base!(
            "ash_wednesday",
            date,
            Precedence::AshWednesday_2,
            Rank::Weekday,
            Season::Lent,
            Color::Purple,
            cache
        );

        Ok(liturgical_day)
    }

    /// Creates a weekday after Ash Wednesday
    fn create_weekday_after_ash_wednesday(
        &self,
        dow: u8,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let liturgical_day = create_liturgical_day_base!(
            format!("{}_after_ash_wednesday", WEEKDAY_NAMES[dow as usize]),
            date,
            Precedence::PrivilegedWeekday_9,
            Rank::Weekday,
            Season::Lent,
            Color::Purple,
            cache
        );

        Ok(liturgical_day)
    }

    /// Creates a Lent weekday
    fn create_lent_weekday(
        &self,
        week: u32,
        dow: u8,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let id = format!("lent_{}_{}", week, WEEKDAY_NAMES[dow as usize]);
        let liturgical_day = create_liturgical_day_base!(
            id,
            date,
            if dow == 0 {
                Precedence::PrivilegedSunday_2
            } else {
                Precedence::PrivilegedWeekday_9
            },
            if dow == 0 {
                Rank::Sunday
            } else {
                Rank::Weekday
            },
            Season::Lent,
            if week == 4 && dow == 0 {
                Color::Rose
            } else {
                Color::Purple
            },
            cache
        );

        Ok(liturgical_day)
    }

    /// Creates Palm Sunday of the Passion of the Lord
    fn create_palm_sunday(
        &self,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let mut liturgical_day = create_liturgical_day_base!(
            "palm_sunday_of_the_passion_of_the_lord",
            date,
            Precedence::PrivilegedSunday_2,
            Rank::Sunday,
            Season::Lent,
            Color::Red,
            cache
        );

        // Override specific properties for Palm Sunday
        liturgical_day.is_holy_day_of_obligation = true;

        Ok(liturgical_day)
    }

    /// Creates a Holy Week weekday
    fn create_holy_week_weekday(
        &self,
        dow: u8,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let id = format!("holy_{}", WEEKDAY_NAMES[dow as usize]);
        let liturgical_day = create_liturgical_day_base!(
            id,
            date,
            Precedence::PrivilegedWeekday_9,
            Rank::Weekday,
            Season::Lent,
            Color::Purple,
            cache
        );

        Ok(liturgical_day)
    }

    // ---------------------------------------------------------------------------------
    // PASCHAL TRIDUUM DAY CREATION FUNCTIONS
    // ---------------------------------------------------------------------------------

    /// Creates Holy Thursday (Thursday of the Lord's Supper)
    fn create_holy_thursday(
        &self,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let liturgical_day = create_liturgical_day_base!(
            "thursday_of_the_lords_supper",
            date,
            Precedence::Triduum_1,
            Rank::Weekday,
            Season::PaschalTriduum,
            Color::White,
            cache
        );

        Ok(liturgical_day)
    }

    /// Creates Good Friday (Friday of the Passion of the Lord)
    fn create_good_friday(
        &self,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let liturgical_day = create_liturgical_day_base!(
            "friday_of_the_passion_of_the_lord",
            date,
            Precedence::Triduum_1,
            Rank::Weekday,
            Season::PaschalTriduum,
            Color::Red,
            cache
        );

        Ok(liturgical_day)
    }

    /// Creates Holy Saturday
    fn create_holy_saturday(
        &self,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let liturgical_day = create_liturgical_day_base!(
            "holy_saturday",
            date,
            Precedence::Triduum_1,
            Rank::Weekday,
            Season::PaschalTriduum,
            Color::White, // Using White as default, can be overridden if needed
            cache
        );

        Ok(liturgical_day)
    }

    /// Creates Easter Sunday of the Resurrection of the Lord
    fn create_easter_sunday(
        &self,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let mut liturgical_day = create_liturgical_day_base!(
            "easter_sunday",
            date,
            Precedence::Triduum_1,
            Rank::Sunday,
            Season::PaschalTriduum, // Primary season
            Color::White,
            cache
        );

        // Override specific properties for Easter Sunday
        liturgical_day.is_holy_day_of_obligation = true;

        // Easter Sunday belongs to both PaschalTriduum and EasterTime
        liturgical_day.seasons = vec![
            SeasonInfo {
                key: Season::PaschalTriduum,
                name: enum_to_string(&Season::PaschalTriduum),
            },
            SeasonInfo {
                key: Season::EasterTime,
                name: enum_to_string(&Season::EasterTime),
            },
        ];

        Ok(liturgical_day)
    }

    // ---------------------------------------------------------------------------------
    // EASTER TIME DAY CREATION FUNCTIONS
    // ---------------------------------------------------------------------------------

    /// Creates a day within the Octave of Easter
    fn create_easter_octave_day(
        &self,
        dow: u8,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let id = format!("easter_{}", WEEKDAY_NAMES[dow as usize]);
        let liturgical_day = create_liturgical_day_base!(
            id,
            date,
            Precedence::WeekdayOfEasterOctave_2,
            Rank::Weekday,
            Season::EasterTime,
            Color::White,
            cache
        );

        Ok(liturgical_day)
    }

    /// Creates Divine Mercy Sunday (Second Sunday of Easter)
    fn create_divine_mercy_sunday(
        &self,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let mut liturgical_day = create_liturgical_day_base!(
            "divine_mercy_sunday",
            date,
            Precedence::PrivilegedSunday_2,
            Rank::Sunday,
            Season::EasterTime,
            Color::White,
            cache
        );

        // Override specific properties for Divine Mercy Sunday
        liturgical_day.is_holy_day_of_obligation = true;

        // Add Easter Octave period
        liturgical_day.periods = vec![PeriodInfo {
            key: Period::EasterOctave,
            name: enum_to_string(&Period::EasterOctave),
        }];

        // Override date definition with specific function
        liturgical_day.date_def = DateDef::DateFunction {
            date_fn: DateFn::DivineMercySunday,
            day_offset: None,
        };

        Ok(liturgical_day)
    }

    /// Creates a weekday or Sunday of Easter Time
    fn create_easter_time_weekday(
        &self,
        week: u8,
        dow: u8,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let id = format!("easter_time_{}_{}", week, WEEKDAY_NAMES[dow as usize]);
        let liturgical_day = create_liturgical_day_base!(
            id,
            date,
            if dow == 0 {
                Precedence::PrivilegedSunday_2
            } else {
                Precedence::Weekday_13
            },
            if dow == 0 {
                Rank::Sunday
            } else {
                Rank::Weekday
            },
            Season::EasterTime,
            Color::White,
            cache
        );

        Ok(liturgical_day)
    }

    /// Creates the Ascension of the Lord
    fn create_ascension_of_the_lord(
        &self,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let liturgical_day = create_liturgical_day_base!(
            "ascension_of_the_lord",
            date,
            Precedence::ProperOfTimeSolemnity_2,
            Rank::Solemnity,
            Season::EasterTime,
            Color::White,
            cache
        );

        Ok(liturgical_day)
    }

    /// Creates Pentecost Sunday
    fn create_pentecost_sunday(
        &self,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let liturgical_day = create_liturgical_day_base!(
            "pentecost_sunday",
            date,
            Precedence::ProperOfTimeSolemnity_2,
            Rank::Solemnity,
            Season::EasterTime,
            Color::Red,
            cache
        );

        Ok(liturgical_day)
    }

    // ---------------------------------------------------------------------------------
    // ORDINARY TIME DAY CREATION FUNCTIONS
    // ---------------------------------------------------------------------------------

    /// Creates the Most Holy Trinity (Trinity Sunday)
    fn create_most_holy_trinity(
        &self,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let liturgical_day = create_liturgical_day_base!(
            "most_holy_trinity",
            date,
            Precedence::GeneralSolemnity_3,
            Rank::Solemnity,
            Season::OrdinaryTime,
            Color::White,
            cache
        );

        Ok(liturgical_day)
    }

    /// Creates the Most Holy Body and Blood of Christ (Corpus Christi)
    fn create_most_holy_body_and_blood_of_christ(
        &self,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let liturgical_day = create_liturgical_day_base!(
            "most_holy_body_and_blood_of_christ",
            date,
            Precedence::GeneralSolemnity_3,
            Rank::Solemnity,
            Season::OrdinaryTime,
            Color::White,
            cache
        );

        Ok(liturgical_day)
    }

    /// Creates the Most Sacred Heart of Jesus
    fn create_most_sacred_heart_of_jesus(
        &self,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let liturgical_day = create_liturgical_day_base!(
            "most_sacred_heart_of_jesus",
            date,
            Precedence::GeneralSolemnity_3,
            Rank::Solemnity,
            Season::OrdinaryTime,
            Color::White,
            cache
        );

        Ok(liturgical_day)
    }

    /// Creates a regular Ordinary Time day
    fn create_ordinary_time_day(
        &self,
        week: u8,
        dow: u8,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let id = format!("ordinary_time_{}_{}", week, WEEKDAY_NAMES[dow as usize]);
        let mut liturgical_day = create_liturgical_day_base!(
            id,
            date,
            if dow == 0 {
                Precedence::UnprivilegedSunday_6
            } else {
                Precedence::Weekday_13
            },
            if dow == 0 {
                Rank::Sunday
            } else {
                Rank::Weekday
            },
            Season::OrdinaryTime,
            Color::Green,
            cache
        );

        // TODO: Override week_of_season with the calculated week
        liturgical_day.week_of_season = week as u32;

        Ok(liturgical_day)
    }

    /// Creates the Sunday of the Word of God (3rd week of Ordinary Time)
    fn create_sunday_of_the_word_of_god(
        &self,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let mut liturgical_day = create_liturgical_day_base!(
            "sunday_of_the_word_of_god",
            date,
            Precedence::UnprivilegedSunday_6,
            Rank::Sunday,
            Season::OrdinaryTime,
            Color::Green,
            cache
        );

        // TODO: Sunday of the Word of God is always in week 3
        liturgical_day.week_of_season = 3;

        Ok(liturgical_day)
    }

    /// Creates Our Lord Jesus Christ, King of the Universe (34th week of Ordinary Time)
    fn create_our_lord_jesus_christ_king_of_the_universe(
        &self,
        date: DateTime<Utc>,
        cache: &ProperOfTimeCache,
    ) -> RomcalResult<LiturgicalDay> {
        let liturgical_day = create_liturgical_day_base!(
            "our_lord_jesus_christ_king_of_the_universe",
            date,
            Precedence::GeneralSolemnity_3,
            Rank::Solemnity,
            Season::OrdinaryTime,
            Color::White,
            cache
        );

        Ok(liturgical_day)
    }
}

// =================================================================================
// TESTS
// =================================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::preset::PresetPartial;

    #[test]
    fn test_proper_of_time_creation() {
        let config = Preset::default();
        let proper_of_time = ProperOfTime::new(config, 2026).unwrap();

        assert_eq!(proper_of_time.cache.advent_year(), 2026);
        assert_eq!(proper_of_time.cache.easter_year(), 2026);
    }

    #[test]
    fn test_advent_generation() {
        let config = Preset::default();
        let proper_of_time = ProperOfTime::new(config, 2026).unwrap();
        let advent_days = proper_of_time.advent().unwrap();

        // Check that we have generated days
        assert!(!advent_days.is_empty());

        // Check that we have the 4 Sundays
        let sundays: Vec<_> = advent_days
            .iter()
            .filter(|day| day.id.contains("sunday"))
            .collect();
        assert_eq!(sundays.len(), 4);
    }

    #[test]
    fn test_liturgical_year_advent() {
        let config = Preset::new(PresetPartial {
            context: Some(crate::CalendarContext::Liturgical),
            ..PresetPartial::default()
        });
        let proper_of_time = ProperOfTime::new(config, 2026).unwrap();
        // For liturgical year 2026, Advent begins in 2025
        let advent_days = proper_of_time.advent().unwrap();

        // For liturgical year 2026, Advent must begin in 2025
        assert!(!advent_days.is_empty());

        // Check that the dates are in 2025
        for day in &advent_days {
            let year = day.date.split('-').next().unwrap().parse::<i32>().unwrap();
            assert_eq!(year, 2025);
        }
    }

    #[test]
    fn test_early_christmas_time_generation() {
        let config = Preset::default();
        let proper_of_time = ProperOfTime::new(config, 2026).unwrap();
        let christmas_days = proper_of_time.early_christmas_time().unwrap();

        // Check that we have generated days
        assert!(!christmas_days.is_empty());

        // Check that we have the Nativity of the Lord
        let nativity: Vec<_> = christmas_days
            .iter()
            .filter(|day| day.id == "nativity_of_the_lord")
            .collect();
        assert_eq!(nativity.len(), 1);

        // Check that we have the Holy Family
        let holy_family: Vec<_> = christmas_days
            .iter()
            .filter(|day| day.id == "holy_family_of_jesus_mary_and_joseph")
            .collect();
        assert_eq!(holy_family.len(), 1);

        // Check that we have Octave days
        let octave_days: Vec<_> = christmas_days
            .iter()
            .filter(|day| day.id.starts_with("christmas_octave_day_"))
            .collect();
        assert!(!octave_days.is_empty());
    }

    #[test]
    fn test_liturgical_year_early_christmas_time() {
        let config = Preset::new(PresetPartial {
            context: Some(crate::CalendarContext::Liturgical),
            ..PresetPartial::default()
        });
        let proper_of_time = ProperOfTime::new(config, 2026).unwrap();
        // For liturgical year 2026, Christmas is in 2025
        let christmas_days = proper_of_time.early_christmas_time().unwrap();

        // Check that we have generated days
        assert!(!christmas_days.is_empty());

        // Check that the dates are in 2025
        for day in &christmas_days {
            let year = day.date.split('-').next().unwrap().parse::<i32>().unwrap();
            assert_eq!(year, 2025);
        }
    }

    #[test]
    fn test_no_duplicate_dates() {
        let config = Preset::default();
        let proper_of_time = ProperOfTime::new(config, 2026).unwrap();
        let all_days = proper_of_time.generate_all().unwrap();

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
            actual_duplicates,
            expected_duplicates,
            "Expected {} duplicate date (Holy Thursday), but found {} duplicates. Original: {}, Unique: {}",
            expected_duplicates,
            actual_duplicates,
            original_count,
            unique_count
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
        assert!(holy_thursday_days
            .iter()
            .any(|d| d.id == "thursday_of_the_lords_supper"));
    }

    #[test]
    fn test_no_duplicate_dates_liturgical_context() {
        let config = Preset::new(PresetPartial {
            context: Some(crate::CalendarContext::Liturgical),
            ..PresetPartial::default()
        });
        let proper_of_time = ProperOfTime::new(config, 2026).unwrap();
        let all_days = proper_of_time.generate_all().unwrap();

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
            actual_duplicates,
            expected_duplicates,
            "Expected {} duplicate date (Holy Thursday), but found {} duplicates (liturgical context). Original: {}, Unique: {}",
            expected_duplicates,
            actual_duplicates,
            original_count,
            unique_count
        );
    }

    #[test]
    fn test_sort_liturgical_days_by_date() {
        let config = Preset::default();
        let proper_of_time = ProperOfTime::new(config, 2026).unwrap();
        let mut all_days = proper_of_time.generate_all().unwrap();

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
        let config = Preset::default();
        let proper_of_time = ProperOfTime::new(config, 2026).unwrap();

        // Get all liturgical days
        let mut days = proper_of_time.generate_all().unwrap();

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

    #[test]
    fn test_late_christmas_time_generation() {
        let config = Preset::default();
        let proper_of_time = ProperOfTime::new(config, 2026).unwrap();

        let days = proper_of_time.late_christmas_time().unwrap();

        // Should have at least Mary Mother of God, Epiphany, and Baptism of the Lord
        assert!(!days.is_empty());

        // Check for Mary Mother of God
        let mary_mother_of_god = days.iter().find(|d| d.id == "mary_mother_of_god");
        assert!(mary_mother_of_god.is_some());
        assert_eq!(mary_mother_of_god.unwrap().date, "2026-01-01");

        // Check for Epiphany of the Lord
        let epiphany = days.iter().find(|d| d.id == "epiphany_of_the_lord");
        assert!(epiphany.is_some());

        // Check for Baptism of the Lord
        let baptism = days.iter().find(|d| d.id == "baptism_of_the_lord");
        assert!(baptism.is_some());
    }

    #[test]
    fn test_liturgical_year_late_christmas_time() {
        let config = Preset::new(PresetPartial {
            context: Some(crate::CalendarContext::Liturgical),
            ..PresetPartial::default()
        });
        let proper_of_time = ProperOfTime::new(config, 2026).unwrap();

        let days = proper_of_time.late_christmas_time().unwrap();

        // Should have at least Mary Mother of God, Epiphany, and Baptism of the Lord
        assert!(!days.is_empty());

        // Check for Mary Mother of God
        let mary_mother_of_god = days.iter().find(|d| d.id == "mary_mother_of_god");
        assert!(mary_mother_of_god.is_some());
        assert_eq!(mary_mother_of_god.unwrap().date, "2026-01-01");
    }

    #[test]
    fn test_lent_generation() {
        let config = Preset::default();
        let proper_of_time = ProperOfTime::new(config, 2026).unwrap();

        let days = proper_of_time.lent().unwrap();

        // Should have at least Ash Wednesday, Palm Sunday, and Lent weekdays
        assert!(!days.is_empty());

        // Check for Ash Wednesday
        let ash_wednesday = days.iter().find(|d| d.id == "ash_wednesday");
        assert!(ash_wednesday.is_some());

        // Check for Palm Sunday
        let palm_sunday = days
            .iter()
            .find(|d| d.id == "palm_sunday_of_the_passion_of_the_lord");
        assert!(palm_sunday.is_some());

        // Check for Lent weekdays
        let lent_weekdays: Vec<_> = days.iter().filter(|d| d.id.starts_with("lent_")).collect();
        assert!(!lent_weekdays.is_empty());

        // Check for Holy Week weekdays
        let holy_week_days: Vec<_> = days.iter().filter(|d| d.id.starts_with("holy_")).collect();
        assert!(!holy_week_days.is_empty());
    }

    #[test]
    fn test_liturgical_year_lent() {
        let config = Preset::new(PresetPartial {
            context: Some(crate::CalendarContext::Liturgical),
            ..PresetPartial::default()
        });
        let proper_of_time = ProperOfTime::new(config, 2026).unwrap();

        let days = proper_of_time.lent().unwrap();

        // Should have at least Ash Wednesday, Palm Sunday, and Lent weekdays
        assert!(!days.is_empty());

        // Check for Ash Wednesday
        let ash_wednesday = days.iter().find(|d| d.id == "ash_wednesday");
        assert!(ash_wednesday.is_some());

        // Check for Palm Sunday
        let palm_sunday = days
            .iter()
            .find(|d| d.id == "palm_sunday_of_the_passion_of_the_lord");
        assert!(palm_sunday.is_some());
    }

    #[test]
    fn test_paschal_triduum_generation() {
        let config = Preset::default();
        let proper_of_time = ProperOfTime::new(config, 2026).unwrap();

        let days = proper_of_time.paschal_triduum().unwrap();

        // Should have exactly 4 days: Holy Thursday, Good Friday, Holy Saturday, Easter Sunday
        assert_eq!(days.len(), 4);

        // Check for Holy Thursday
        let holy_thursday = days.iter().find(|d| d.id == "thursday_of_the_lords_supper");
        assert!(holy_thursday.is_some());

        // Check for Good Friday
        let good_friday = days
            .iter()
            .find(|d| d.id == "friday_of_the_passion_of_the_lord");
        assert!(good_friday.is_some());

        // Check for Holy Saturday
        let holy_saturday = days.iter().find(|d| d.id == "holy_saturday");
        assert!(holy_saturday.is_some());

        // Check for Easter Sunday
        let easter_sunday = days.iter().find(|d| d.id == "easter_sunday");
        assert!(easter_sunday.is_some());
    }

    #[test]
    fn test_liturgical_year_paschal_triduum() {
        let config = Preset::new(PresetPartial {
            context: Some(crate::CalendarContext::Liturgical),
            ..PresetPartial::default()
        });
        let proper_of_time = ProperOfTime::new(config, 2026).unwrap();

        let days = proper_of_time.paschal_triduum().unwrap();

        // Should have exactly 4 days: Holy Thursday, Good Friday, Holy Saturday, Easter Sunday
        assert_eq!(days.len(), 4);

        // Check for Holy Thursday
        let holy_thursday = days.iter().find(|d| d.id == "thursday_of_the_lords_supper");
        assert!(holy_thursday.is_some());

        // Check for Easter Sunday
        let easter_sunday = days.iter().find(|d| d.id == "easter_sunday");
        assert!(easter_sunday.is_some());
    }

    #[test]
    fn test_easter_time_generation() {
        let config = Preset::default();
        let proper_of_time = ProperOfTime::new(config, 2026).unwrap();

        let days = proper_of_time.easter_time().unwrap();

        // Should have: 6 octave days + 1 divine mercy + 40 weekdays/sundays + 1 ascension + 1 pentecost = 49 days
        // Note: Easter Sunday is not included here as it's already generated in paschal_triduum
        assert_eq!(days.len(), 49);

        // Check for Easter octave days (Monday-Saturday)
        for dow in 1..=6 {
            let octave_day = days
                .iter()
                .find(|d| d.id == format!("easter_{}", WEEKDAY_NAMES[dow]));
            assert!(
                octave_day.is_some(),
                "Missing Easter octave day for dow {}",
                dow
            );
        }

        // Check for Divine Mercy Sunday
        let divine_mercy = days.iter().find(|d| d.id == "divine_mercy_sunday");
        assert!(divine_mercy.is_some());

        // Check for Ascension of the Lord
        let ascension = days.iter().find(|d| d.id == "ascension_of_the_lord");
        assert!(ascension.is_some());

        // Check for Pentecost Sunday
        let pentecost = days.iter().find(|d| d.id == "pentecost_sunday");
        assert!(pentecost.is_some());

        // Check for some Easter Time weekdays
        let easter_time_weekday = days.iter().find(|d| d.id.starts_with("easter_time_"));
        assert!(easter_time_weekday.is_some());
    }

    #[test]
    fn test_liturgical_year_easter_time() {
        let config = Preset::new(PresetPartial {
            context: Some(crate::CalendarContext::Liturgical),
            ..PresetPartial::default()
        });
        let proper_of_time = ProperOfTime::new(config, 2026).unwrap();

        let days = proper_of_time.easter_time().unwrap();

        // Should have exactly 49 days
        assert_eq!(days.len(), 49);

        // Check for Divine Mercy Sunday
        let divine_mercy = days.iter().find(|d| d.id == "divine_mercy_sunday");
        assert!(divine_mercy.is_some());

        // Check for Pentecost Sunday
        let pentecost = days.iter().find(|d| d.id == "pentecost_sunday");
        assert!(pentecost.is_some());
    }

    #[test]
    fn test_early_ordinary_time_generation() {
        let config = Preset::default();
        let proper_of_time = ProperOfTime::new(config, 2026).unwrap();

        let days = proper_of_time.early_ordinary_time().unwrap();

        // Check for Sunday of the Word of God (3rd week)
        let word_of_god = days.iter().find(|d| d.id == "sunday_of_the_word_of_god");
        assert!(
            word_of_god.is_some(),
            "Sunday of the Word of God should be present in early ordinary time"
        );

        // Check for some regular Ordinary Time days
        let ordinary_weekday = days
            .iter()
            .find(|d| d.seasons.iter().any(|s| s.key == Season::OrdinaryTime));
        assert!(
            ordinary_weekday.is_some(),
            "Should have ordinary time weekdays"
        );

        // Check that all days are in Ordinary Time season
        for day in &days {
            assert!(
                day.seasons.iter().any(|s| s.key == Season::OrdinaryTime),
                "All days should be in Ordinary Time season, but {} is not",
                day.id
            );
        }
    }

    #[test]
    fn test_late_ordinary_time_generation() {
        let config = Preset::default();
        let proper_of_time = ProperOfTime::new(config, 2026).unwrap();

        let days = proper_of_time.late_ordinary_time().unwrap();

        // Check for the Most Holy Trinity
        let trinity = days.iter().find(|d| d.id == "most_holy_trinity");
        assert!(
            trinity.is_some(),
            "Most Holy Trinity should be present in late ordinary time"
        );

        // Check for Corpus Christi
        let corpus_christi = days
            .iter()
            .find(|d| d.id == "most_holy_body_and_blood_of_christ");
        assert!(
            corpus_christi.is_some(),
            "Corpus Christi should be present in late ordinary time"
        );

        // Check for the Most Sacred Heart of Jesus
        let sacred_heart = days.iter().find(|d| d.id == "most_sacred_heart_of_jesus");
        assert!(
            sacred_heart.is_some(),
            "Most Sacred Heart of Jesus should be present in late ordinary time"
        );

        // Check for Christ the King (34th week)
        let christ_king = days
            .iter()
            .find(|d| d.id == "our_lord_jesus_christ_king_of_the_universe");
        assert!(
            christ_king.is_some(),
            "Christ the King should be present in late ordinary time"
        );

        // Check for some regular Ordinary Time days
        let ordinary_weekday = days
            .iter()
            .find(|d| d.seasons.iter().any(|s| s.key == Season::OrdinaryTime));
        assert!(
            ordinary_weekday.is_some(),
            "Should have ordinary time weekdays"
        );

        // Check that all days are in Ordinary Time season
        for day in &days {
            assert!(
                day.seasons.iter().any(|s| s.key == Season::OrdinaryTime),
                "All days should be in Ordinary Time season, but {} is not",
                day.id
            );
        }
    }

    #[test]
    fn test_early_ordinary_time_first_week_incomplete_baptism_monday() {
        // Test with epiphany_on_sunday = true so Baptism of the Lord falls on Monday
        // This means early Ordinary Time starts on Tuesday (no Sunday, no Monday in first week)
        let config = Preset::new(PresetPartial {
            epiphany_on_sunday: Some(true),
            ..PresetPartial::default()
        });
        let proper_of_time = ProperOfTime::new(config, 2029).unwrap(); // 2029: Baptism on Monday

        let days = proper_of_time.early_ordinary_time().unwrap();

        // Check that first week has no Sunday and no Monday
        let first_week_days: Vec<_> = days.iter().filter(|d| d.week_of_season == 1).collect();

        // Should have Tuesday to Saturday only (5 days)
        assert_eq!(first_week_days.len(), 5);

        // Check specific days
        assert!(first_week_days
            .iter()
            .any(|d| d.id == "ordinary_time_1_tuesday"));
        assert!(first_week_days
            .iter()
            .any(|d| d.id == "ordinary_time_1_saturday"));

        // Should NOT have Sunday or Monday
        assert!(!first_week_days
            .iter()
            .any(|d| d.id == "ordinary_time_1_sunday"));
        assert!(!first_week_days
            .iter()
            .any(|d| d.id == "ordinary_time_1_monday"));

        // Second week should start with Sunday
        let second_week_sunday = days.iter().find(|d| d.id == "ordinary_time_2_sunday");
        assert!(second_week_sunday.is_some());

        // Verify that the Sunday after the incomplete first week is indeed week 2
        let second_week_days: Vec<_> = days.iter().filter(|d| d.week_of_season == 2).collect();

        // Should have Sunday to Saturday (7 days - complete week)
        assert_eq!(second_week_days.len(), 7);

        // Check that we have all days from Sunday to Saturday using day_of_week property
        for dow in 0..7 {
            assert!(
                second_week_days
                    .iter()
                    .any(|d| d.day_of_week == DayOfWeek(dow)),
                "Missing day of week {} in second week",
                dow
            );
        }

        // Verify that all days in the first week (incomplete) have week_of_season == 1
        for day in first_week_days {
            assert_eq!(
                day.week_of_season, 1,
                "All days in first week should have week_of_season == 1, but {} has week_of_season == {}",
                day.id, day.week_of_season
            );
        }

        // Verify that all days in the second week have week_of_season == 2
        for day in second_week_days {
            assert_eq!(
                day.week_of_season, 2,
                "All days in second week should have week_of_season == 2, but {} has week_of_season == {}",
                day.id, day.week_of_season
            );
        }
    }

    #[test]
    fn test_early_ordinary_time_first_week_incomplete_baptism_sunday() {
        // Test with epiphany_on_sunday = false so Baptism of the Lord falls on Sunday
        // This means early Ordinary Time starts on Monday (no Sunday in first week)
        let config = Preset::new(PresetPartial {
            epiphany_on_sunday: Some(false),
            ..PresetPartial::default()
        });
        let proper_of_time = ProperOfTime::new(config, 2030).unwrap(); // 2030: Baptism on Sunday

        let days = proper_of_time.early_ordinary_time().unwrap();

        // Check that first week has no Sunday but has Monday
        let first_week_days: Vec<_> = days.iter().filter(|d| d.week_of_season == 1).collect();

        // Should have Monday to Saturday (6 days)
        assert_eq!(first_week_days.len(), 6);

        // Check specific days
        assert!(first_week_days
            .iter()
            .any(|d| d.id == "ordinary_time_1_monday"));
        assert!(first_week_days
            .iter()
            .any(|d| d.id == "ordinary_time_1_saturday"));

        // Should NOT have Sunday
        assert!(!first_week_days
            .iter()
            .any(|d| d.id == "ordinary_time_1_sunday"));

        // Second week should start with Sunday
        let second_week_sunday = days.iter().find(|d| d.id == "ordinary_time_2_sunday");
        assert!(second_week_sunday.is_some());

        // Verify that the Sunday after the incomplete first week is indeed week 2
        let second_week_days: Vec<_> = days.iter().filter(|d| d.week_of_season == 2).collect();

        // Should have Sunday to Saturday (7 days - complete week)
        assert_eq!(second_week_days.len(), 7);

        // Check that we have all days from Sunday to Saturday using day_of_week property
        for dow in 0..7 {
            assert!(
                second_week_days
                    .iter()
                    .any(|d| d.day_of_week == DayOfWeek(dow)),
                "Missing day of week {} in second week",
                dow
            );
        }

        // Verify that all days in the first week (incomplete) have week_of_season == 1
        for day in first_week_days {
            assert_eq!(
                day.week_of_season, 1,
                "All days in first week should have week_of_season == 1, but {} has week_of_season == {}",
                day.id, day.week_of_season
            );
        }

        // Verify that all days in the second week have week_of_season == 2
        for day in second_week_days {
            assert_eq!(
                day.week_of_season, 2,
                "All days in second week should have week_of_season == 2, but {} has week_of_season == {}",
                day.id, day.week_of_season
            );
        }
    }

    #[test]
    fn test_late_ordinary_time_first_week_incomplete() {
        // Test that late Ordinary Time first week is always incomplete (Monday to Saturday only)
        // because the Sunday is Pentecost Sunday
        let config = Preset::default();
        let proper_of_time = ProperOfTime::new(config, 2026).unwrap();

        let days = proper_of_time.late_ordinary_time().unwrap();

        // Group by week_of_season
        let mut weeks: std::collections::HashMap<u32, Vec<_>> = std::collections::HashMap::new();
        for day in &days {
            weeks
                .entry(day.week_of_season)
                .or_insert_with(Vec::new)
                .push(day);
        }

        // Find the first week (lowest week number)
        let first_week_num = *weeks.keys().min().unwrap();
        let first_week_days = &weeks[&first_week_num];

        // First week should have Monday to Saturday only (6 days)
        assert_eq!(first_week_days.len(), 6);

        // Check specific days using day_of_week property
        assert!(first_week_days
            .iter()
            .any(|d| d.day_of_week == DayOfWeek(1))); // Monday
        assert!(first_week_days
            .iter()
            .any(|d| d.day_of_week == DayOfWeek(6))); // Saturday

        // Should NOT have Sunday
        assert!(!first_week_days
            .iter()
            .any(|d| d.day_of_week == DayOfWeek(0))); // Sunday

        // Second week should have Sunday
        let second_week_num = first_week_num + 1;
        let all_second_week_days: Vec<_> = days
            .iter()
            .filter(|d| d.week_of_season == second_week_num)
            .collect();

        // Verify that the second week is complete (7 days)
        assert_eq!(all_second_week_days.len(), 7);

        // Check that we have all days from Sunday to Saturday using day_of_week property
        for dow in 0..7 {
            assert!(
                all_second_week_days
                    .iter()
                    .any(|d| d.day_of_week == DayOfWeek(dow)),
                "Missing day of week {} in second week",
                dow
            );
        }
    }

    #[test]
    fn test_late_ordinary_time_34th_week_christ_king() {
        // Test that the last week of late Ordinary Time is always the 34th week
        // and that the Sunday of this week is Christ the King
        let config = Preset::default();
        let proper_of_time = ProperOfTime::new(config, 2026).unwrap();

        let days = proper_of_time.late_ordinary_time().unwrap();

        // Find all ordinary time days (excluding solemnities)
        let ordinary_days: Vec<_> = days
            .iter()
            .filter(|d| {
                d.seasons.iter().any(|s| s.key == Season::OrdinaryTime) && d.rank != Rank::Solemnity
            })
            .collect();

        // Group by week_of_season
        let mut weeks: std::collections::HashMap<u32, Vec<_>> = std::collections::HashMap::new();
        for day in &ordinary_days {
            weeks
                .entry(day.week_of_season)
                .or_insert_with(Vec::new)
                .push(day);
        }

        // Find the last week (highest week number)
        let last_week_num = *weeks.keys().max().unwrap();
        assert_eq!(
            last_week_num, 34,
            "Last week should be 34th week, but found week {}",
            last_week_num
        );

        // Check that the Sunday of the 34th week is Christ the King
        let christ_king = days
            .iter()
            .find(|d| d.id == "our_lord_jesus_christ_king_of_the_universe");
        assert!(christ_king.is_some(), "Christ the King should be present");

        // Verify that Christ the King is indeed on a Sunday of week 34
        let christ_king_day = christ_king.unwrap();
        assert_eq!(
            christ_king_day.week_of_season, 34,
            "Christ the King should be in week 34"
        );
        assert_eq!(
            christ_king_day.day_of_week,
            DayOfWeek(0),
            "Christ the King should be on Sunday"
        );

        // Verify that the 34th week has exactly 7 days total (including Christ the King)
        let all_week_34_days: Vec<_> = days.iter().filter(|d| d.week_of_season == 34).collect();
        assert_eq!(
            all_week_34_days.len(),
            7,
            "34th week should have 7 days total (including Christ the King), but found {}",
            all_week_34_days.len()
        );

        // Check that we have all days from Sunday to Saturday in the 34th week
        for dow in 0..7 {
            assert!(
                all_week_34_days
                    .iter()
                    .any(|d| d.day_of_week == DayOfWeek(dow)),
                "34th week should have day of week {}, but found days with day_of_week: {:?}",
                dow,
                all_week_34_days
                    .iter()
                    .map(|d| d.day_of_week.0)
                    .collect::<Vec<_>>()
            );
        }

        // Verify that all liturgical days in the 34th week have week_of_season == 34
        for day in all_week_34_days {
            assert_eq!(
                day.week_of_season, 34,
                "All days in 34th week should have week_of_season == 34, but {} has week_of_season == {}",
                day.id, day.week_of_season
            );
        }

        // Also verify that Christ the King has week_of_season == 34
        let christ_king_day = christ_king.unwrap();
        assert_eq!(
            christ_king_day.week_of_season, 34,
            "Christ the King should have week_of_season == 34, but found {}",
            christ_king_day.week_of_season
        );
    }
}
