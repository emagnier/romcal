//! Template resolver for liturgical day translations.
//!
//! This module provides functionality to resolve localized templates
//! for liturgical days using a simple `{variable}` substitution pattern.

use crate::types::OrdinalFormat;
use crate::types::resource::{
    AdventSeason, ChristmasTimeSeason, CyclesMetadata, EasterTimeSeason, LentSeason, LocaleColors,
    OrdinaryTimeSeason, PeriodsMetadata, RanksMetadata, ResourcesMetadata, SeasonsMetadata,
};
use std::collections::BTreeMap;

/// Grammatical gender for agreement in translations.
/// Used for name templates (e.g., "Saint" vs "Sainte" in French).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GrammaticalGender {
    /// Default/neutral form (no suffix)
    Default,
    /// Masculine form (_masculine suffix)
    Masculine,
    /// Feminine form (_feminine suffix)
    Feminine,
    /// Neuter form (_neuter suffix)
    Neuter,
}

impl GrammaticalGender {
    /// Returns the suffix to append to keys for this gender.
    pub fn suffix(&self) -> &'static str {
        match self {
            GrammaticalGender::Default => "",
            GrammaticalGender::Masculine => "_masculine",
            GrammaticalGender::Feminine => "_feminine",
            GrammaticalGender::Neuter => "_neuter",
        }
    }
}

/// Type of day in the Proper of Time, used for template resolution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProperOfTimeDayType {
    // Advent
    AdventSunday { week: u8 },
    AdventWeekday { week: u8, dow: u8 },
    AdventPrivilegedWeekday { day: u8, dow: u8 },

    // Christmas Time
    ChristmasOctave { count: u8 },
    ChristmasDay { dow: u8 },
    BeforeEpiphany { day: u8 },
    AfterEpiphany { dow: u8 },
    SecondSundayAfterChristmas,

    // Ordinary Time
    OrdinaryTimeSunday { week: u8 },
    OrdinaryTimeWeekday { week: u8, dow: u8 },

    // Lent
    LentSunday { week: u8 },
    LentWeekday { week: u8, dow: u8 },
    DayAfterAshWednesday { dow: u8 },
    HolyWeekDay { dow: u8 },

    // Paschal Triduum
    HolyThursday,
    GoodFriday,
    HolySaturday,

    // Easter Time
    EasterSunday,
    EasterOctave { dow: u8 },
    EasterTimeSunday { week: u8 },
    EasterTimeWeekday { week: u8, dow: u8 },
}

/// Template resolver for liturgical translations.
///
/// Uses `ResourcesMetadata` to resolve templates with `{variable}` placeholders
/// into localized strings.
#[derive(Debug, Clone)]
pub struct TemplateResolver {
    metadata: ResourcesMetadata,
    locale: String,
    ordinal_format: OrdinalFormat,
}

impl TemplateResolver {
    /// Creates a new TemplateResolver with the given metadata, locale, and ordinal format.
    pub fn new(metadata: ResourcesMetadata, locale: String, ordinal_format: OrdinalFormat) -> Self {
        Self {
            metadata,
            locale,
            ordinal_format,
        }
    }

    /// Creates a TemplateResolver from optional metadata.
    /// Returns None if metadata is not provided.
    pub fn from_option(
        metadata: Option<ResourcesMetadata>,
        locale: String,
        ordinal_format: OrdinalFormat,
    ) -> Option<Self> {
        metadata.map(|m| Self::new(m, locale, ordinal_format))
    }

    /// Returns the locale of this resolver.
    pub fn locale(&self) -> &str {
        &self.locale
    }

    /// Returns a reference to the metadata.
    pub fn metadata(&self) -> &ResourcesMetadata {
        &self.metadata
    }

    /// Returns the ordinal format setting.
    pub fn ordinal_format(&self) -> OrdinalFormat {
        self.ordinal_format
    }

    // -------------------------------------------------------------------------
    // Core template resolution
    // -------------------------------------------------------------------------

    /// Resolves a template string by substituting `{variable}` placeholders.
    ///
    /// # Arguments
    ///
    /// * `template` - The template string with `{variable}` placeholders
    /// * `vars` - A slice of (key, value) pairs for substitution
    ///
    /// # Example
    ///
    /// ```ignore
    /// let result = resolver.resolve("{weekday} of the {ordinal} week", &[
    ///     ("weekday", "Monday"),
    ///     ("ordinal", "first"),
    /// ]);
    /// assert_eq!(result, "Monday of the first week");
    /// ```
    pub fn resolve(&self, template: &str, vars: &[(&str, &str)]) -> String {
        let mut result = template.to_string();
        for (key, value) in vars {
            result = result.replace(&format!("{{{}}}", key), value);
        }
        result
    }

    // -------------------------------------------------------------------------
    // Ordinal numbers
    // -------------------------------------------------------------------------

    /// Gets an ordinal number in the locale language, using the configured format.
    ///
    /// - If `OrdinalFormat::Letters`, tries `ordinals_letters` first, then falls back to `ordinals_numeric`
    /// - If `OrdinalFormat::Numeric`, tries `ordinals_numeric` first, then falls back to `ordinals_letters`
    ///
    /// Falls back through: primary format -> alternate format -> numeric string
    ///
    /// # Arguments
    ///
    /// * `num` - The ordinal number (1, 2, 3, etc.)
    /// * `gender` - Optional grammatical gender for agreement
    pub fn get_ordinal(&self, num: u32, gender: Option<GrammaticalGender>) -> String {
        let (primary, fallback) = match self.ordinal_format {
            OrdinalFormat::Letters => (
                self.metadata.ordinals_letters.as_ref(),
                self.metadata.ordinals_numeric.as_ref(),
            ),
            OrdinalFormat::Numeric => (
                self.metadata.ordinals_numeric.as_ref(),
                self.metadata.ordinals_letters.as_ref(),
            ),
        };

        // Try primary format first
        if let Some(result) = self.try_get_ordinal_from_map(num, gender, primary) {
            return result;
        }

        // Fall back to alternate format
        if let Some(result) = self.try_get_ordinal_from_map(num, gender, fallback) {
            return result;
        }

        // Last resort: return the number
        num.to_string()
    }

    /// Tries to get an ordinal from a specific map (internal helper).
    /// Returns None if the map is None or doesn't contain the key.
    fn try_get_ordinal_from_map(
        &self,
        num: u32,
        gender: Option<GrammaticalGender>,
        ordinals: Option<&BTreeMap<String, String>>,
    ) -> Option<String> {
        let ordinals = ordinals?;

        // Try gender-specific key first
        if let Some(g) = gender
            && g != GrammaticalGender::Default
        {
            let key_with_gender = format!("{}{}", num, g.suffix());
            if let Some(value) = ordinals.get(&key_with_gender) {
                return Some(value.clone());
            }
        }

        // Fall back to default key
        ordinals.get(&num.to_string()).cloned()
    }

    // -------------------------------------------------------------------------
    // Weekdays
    // -------------------------------------------------------------------------

    /// Gets a weekday name in the locale language.
    ///
    /// # Arguments
    ///
    /// * `dow` - Day of week (0 = Sunday, 6 = Saturday)
    pub fn get_weekday(&self, dow: u8) -> String {
        self.metadata
            .weekdays
            .as_ref()
            .and_then(|w| w.get(&dow.to_string()))
            .cloned()
            .unwrap_or_else(|| self.default_weekday(dow))
    }

    /// Gets a weekday name with first letter capitalized.
    pub fn get_weekday_capitalized(&self, dow: u8) -> String {
        capitalize_first(&self.get_weekday(dow))
    }

    /// Default English weekday names.
    fn default_weekday(&self, dow: u8) -> String {
        match dow {
            0 => "Sunday",
            1 => "Monday",
            2 => "Tuesday",
            3 => "Wednesday",
            4 => "Thursday",
            5 => "Friday",
            6 => "Saturday",
            _ => "Unknown",
        }
        .to_string()
    }

    // -------------------------------------------------------------------------
    // Months
    // -------------------------------------------------------------------------

    /// Gets a month name in the locale language.
    ///
    /// # Arguments
    ///
    /// * `month` - Month index (0 = January, 11 = December)
    pub fn get_month(&self, month: u8) -> String {
        self.metadata
            .months
            .as_ref()
            .and_then(|m| m.get(&month.to_string()))
            .cloned()
            .unwrap_or_else(|| self.default_month(month))
    }

    /// Default English month names.
    fn default_month(&self, month: u8) -> String {
        match month {
            0 => "January",
            1 => "February",
            2 => "March",
            3 => "April",
            4 => "May",
            5 => "June",
            6 => "July",
            7 => "August",
            8 => "September",
            9 => "October",
            10 => "November",
            11 => "December",
            _ => "Unknown",
        }
        .to_string()
    }

    // -------------------------------------------------------------------------
    // Colors
    // -------------------------------------------------------------------------

    /// Gets a color name in the locale language.
    pub fn get_color(&self, color: &str) -> String {
        self.metadata
            .colors
            .as_ref()
            .and_then(|c| self.get_color_from_struct(c, color))
            .unwrap_or_else(|| color.to_string())
    }

    fn get_color_from_struct(&self, colors: &LocaleColors, color: &str) -> Option<String> {
        match color {
            "BLACK" => colors.black.clone(),
            "GOLD" => colors.gold.clone(),
            "GREEN" => colors.green.clone(),
            "PURPLE" => colors.purple.clone(),
            "RED" => colors.red.clone(),
            "ROSE" => colors.rose.clone(),
            "WHITE" => colors.white.clone(),
            _ => None,
        }
    }

    // -------------------------------------------------------------------------
    // Ranks
    // -------------------------------------------------------------------------

    /// Gets a rank name in the locale language.
    pub fn get_rank(&self, rank: &str) -> String {
        self.metadata
            .ranks
            .as_ref()
            .and_then(|r| self.get_rank_from_struct(r, rank))
            .unwrap_or_else(|| rank.to_string())
    }

    fn get_rank_from_struct(&self, ranks: &RanksMetadata, rank: &str) -> Option<String> {
        match rank {
            "SOLEMNITY" => ranks.solemnity.clone(),
            "SUNDAY" => ranks.sunday.clone(),
            "FEAST" => ranks.feast.clone(),
            "MEMORIAL" => ranks.memorial.clone(),
            "OPTIONAL_MEMORIAL" => ranks.optional_memorial.clone(),
            "WEEKDAY" => ranks.weekday.clone(),
            _ => None,
        }
    }

    // -------------------------------------------------------------------------
    // Periods
    // -------------------------------------------------------------------------

    /// Gets a period name in the locale language.
    pub fn get_period(&self, period: &str) -> String {
        self.metadata
            .periods
            .as_ref()
            .and_then(|p| self.get_period_from_struct(p, period))
            .unwrap_or_else(|| period.to_string())
    }

    fn get_period_from_struct(&self, periods: &PeriodsMetadata, period: &str) -> Option<String> {
        match period {
            "CHRISTMAS_OCTAVE" => periods.christmas_octave.clone(),
            "DAYS_BEFORE_EPIPHANY" => periods.days_before_epiphany.clone(),
            "DAYS_FROM_EPIPHANY" => periods.days_from_epiphany.clone(),
            "CHRISTMAS_TO_PRESENTATION_OF_THE_LORD" => {
                periods.christmas_to_presentation_of_the_lord.clone()
            }
            "PRESENTATION_OF_THE_LORD_TO_HOLY_THURSDAY" => {
                periods.presentation_of_the_lord_to_holy_thursday.clone()
            }
            "HOLY_WEEK" => periods.holy_week.clone(),
            "PASCHAL_TRIDUUM" => periods.paschal_triduum.clone(),
            "EASTER_OCTAVE" => periods.easter_octave.clone(),
            "EARLY_ORDINARY_TIME" => periods.early_ordinary_time.clone(),
            "LATE_ORDINARY_TIME" => periods.late_ordinary_time.clone(),
            _ => None,
        }
    }

    // -------------------------------------------------------------------------
    // Cycles
    // -------------------------------------------------------------------------

    /// Gets a cycle name in the locale language.
    pub fn get_cycle(&self, cycle: &str) -> String {
        self.metadata
            .cycles
            .as_ref()
            .and_then(|c| self.get_cycle_from_struct(c, cycle))
            .unwrap_or_else(|| cycle.to_string())
    }

    fn get_cycle_from_struct(&self, cycles: &CyclesMetadata, cycle: &str) -> Option<String> {
        match cycle {
            "PROPER_OF_TIME" => cycles.proper_of_time.clone(),
            "PROPER_OF_SAINTS" => cycles.proper_of_saints.clone(),
            "YEAR_A" => cycles.sunday_year_a.clone(),
            "YEAR_B" => cycles.sunday_year_b.clone(),
            "YEAR_C" => cycles.sunday_year_c.clone(),
            "YEAR_1" => cycles.weekday_year_1.clone(),
            "YEAR_2" => cycles.weekday_year_2.clone(),
            "WEEK_1" => cycles.psalter_week_1.clone(),
            "WEEK_2" => cycles.psalter_week_2.clone(),
            "WEEK_3" => cycles.psalter_week_3.clone(),
            "WEEK_4" => cycles.psalter_week_4.clone(),
            _ => None,
        }
    }

    // -------------------------------------------------------------------------
    // Season names
    // -------------------------------------------------------------------------

    /// Gets a season name in the locale language.
    pub fn get_season_name(&self, season: &str) -> String {
        self.metadata
            .seasons
            .as_ref()
            .and_then(|s| self.get_season_name_from_struct(s, season))
            .unwrap_or_else(|| season.to_string())
    }

    fn get_season_name_from_struct(
        &self,
        seasons: &SeasonsMetadata,
        season: &str,
    ) -> Option<String> {
        match season {
            "ADVENT" => seasons.advent.as_ref().and_then(|s| s.season.clone()),
            "CHRISTMAS_TIME" => seasons
                .christmas_time
                .as_ref()
                .and_then(|s| s.season.clone()),
            "ORDINARY_TIME" => seasons
                .ordinary_time
                .as_ref()
                .and_then(|s| s.season.clone()),
            "LENT" => seasons.lent.as_ref().and_then(|s| s.season.clone()),
            "PASCHAL_TRIDUUM" => seasons
                .paschal_triduum
                .as_ref()
                .and_then(|s| s.season.clone()),
            "EASTER_TIME" => seasons.easter_time.as_ref().and_then(|s| s.season.clone()),
            _ => None,
        }
    }

    // -------------------------------------------------------------------------
    // Proper of Time fullname resolution
    // -------------------------------------------------------------------------

    /// Resolves the fullname for a Proper of Time day type.
    ///
    /// This is the main entry point for resolving liturgical day names
    /// from the Proper of Time.
    pub fn resolve_proper_of_time_fullname(&self, day_type: &ProperOfTimeDayType) -> String {
        match day_type {
            // Advent
            ProperOfTimeDayType::AdventSunday { week } => self.resolve_advent_sunday(*week),
            ProperOfTimeDayType::AdventWeekday { week, dow } => {
                self.resolve_advent_weekday(*week, *dow)
            }
            ProperOfTimeDayType::AdventPrivilegedWeekday { day, dow } => {
                self.resolve_advent_privileged_weekday(*day, *dow)
            }

            // Christmas Time
            ProperOfTimeDayType::ChristmasOctave { count } => self.resolve_christmas_octave(*count),
            ProperOfTimeDayType::ChristmasDay { dow } => self.resolve_christmas_day(*dow),
            ProperOfTimeDayType::BeforeEpiphany { day } => self.resolve_before_epiphany(*day),
            ProperOfTimeDayType::AfterEpiphany { dow } => self.resolve_after_epiphany(*dow),
            ProperOfTimeDayType::SecondSundayAfterChristmas => {
                self.resolve_second_sunday_after_christmas()
            }

            // Ordinary Time
            ProperOfTimeDayType::OrdinaryTimeSunday { week } => {
                self.resolve_ordinary_time_sunday(*week)
            }
            ProperOfTimeDayType::OrdinaryTimeWeekday { week, dow } => {
                self.resolve_ordinary_time_weekday(*week, *dow)
            }

            // Lent
            ProperOfTimeDayType::LentSunday { week } => self.resolve_lent_sunday(*week),
            ProperOfTimeDayType::LentWeekday { week, dow } => {
                self.resolve_lent_weekday(*week, *dow)
            }
            ProperOfTimeDayType::DayAfterAshWednesday { dow } => {
                self.resolve_day_after_ash_wednesday(*dow)
            }
            ProperOfTimeDayType::HolyWeekDay { dow } => self.resolve_holy_week_day(*dow),

            // Paschal Triduum
            ProperOfTimeDayType::HolyThursday => self.resolve_triduum_day("holy_thursday"),
            ProperOfTimeDayType::GoodFriday => self.resolve_triduum_day("good_friday"),
            ProperOfTimeDayType::HolySaturday => self.resolve_triduum_day("holy_saturday"),

            // Easter Time
            ProperOfTimeDayType::EasterSunday => self.resolve_easter_sunday(),
            ProperOfTimeDayType::EasterOctave { dow } => self.resolve_easter_octave(*dow),
            ProperOfTimeDayType::EasterTimeSunday { week } => {
                self.resolve_easter_time_sunday(*week)
            }
            ProperOfTimeDayType::EasterTimeWeekday { week, dow } => {
                self.resolve_easter_time_weekday(*week, *dow)
            }
        }
    }

    // -------------------------------------------------------------------------
    // Advent helpers
    // -------------------------------------------------------------------------

    fn resolve_advent_sunday(&self, week: u8) -> String {
        let template = self.get_advent_template("sunday");
        let ordinal = self.get_ordinal(week as u32, Some(GrammaticalGender::Default));
        let ordinal_feminine = self.get_ordinal(week as u32, Some(GrammaticalGender::Feminine));
        let weekday = self.get_weekday_capitalized(0);
        self.resolve(
            &template,
            &[
                ("week", &week.to_string()),
                ("ordinal", &ordinal),
                ("ordinal_feminine", &ordinal_feminine),
                ("weekday", &weekday),
            ],
        )
    }

    fn resolve_advent_weekday(&self, week: u8, dow: u8) -> String {
        let template = self.get_advent_template("weekday");
        let ordinal = self.get_ordinal(week as u32, Some(GrammaticalGender::Feminine));
        let weekday = self.get_weekday_capitalized(dow);
        self.resolve(
            &template,
            &[
                ("week", &week.to_string()),
                ("dow", &dow.to_string()),
                ("ordinal", &ordinal),
                (
                    "ordinal_feminine",
                    &self.get_ordinal(week as u32, Some(GrammaticalGender::Feminine)),
                ),
                ("weekday", &weekday),
            ],
        )
    }

    fn resolve_advent_privileged_weekday(&self, day: u8, dow: u8) -> String {
        let template = self.get_advent_template("privileged_weekday");
        let month = self.get_month(11); // December
        let weekday = self.get_weekday_capitalized(dow);
        self.resolve(
            &template,
            &[
                ("day", &day.to_string()),
                ("dow", &dow.to_string()),
                ("month", &month),
                ("weekday", &weekday),
            ],
        )
    }

    fn get_advent_template(&self, key: &str) -> String {
        self.metadata
            .seasons
            .as_ref()
            .and_then(|s| s.advent.as_ref())
            .and_then(|a| self.get_advent_field(a, key))
            .unwrap_or_default()
    }

    fn get_advent_field(&self, advent: &AdventSeason, key: &str) -> Option<String> {
        match key {
            "season" => advent.season.clone(),
            "weekday" => advent.weekday.clone(),
            "sunday" => advent.sunday.clone(),
            "privileged_weekday" => advent.privileged_weekday.clone(),
            _ => None,
        }
    }

    // -------------------------------------------------------------------------
    // Christmas Time helpers
    // -------------------------------------------------------------------------

    fn resolve_christmas_octave(&self, count: u8) -> String {
        let template = self.get_christmas_template("octave");
        let ordinal = self.get_ordinal(count as u32, Some(GrammaticalGender::Default));
        self.resolve(
            &template,
            &[("count", &count.to_string()), ("ordinal", &ordinal)],
        )
    }

    fn resolve_christmas_day(&self, dow: u8) -> String {
        let template = self.get_christmas_template("day");
        let weekday = self.get_weekday_capitalized(dow);
        self.resolve(
            &template,
            &[("dow", &dow.to_string()), ("weekday", &weekday)],
        )
    }

    fn resolve_before_epiphany(&self, day: u8) -> String {
        let template = self.get_christmas_template("before_epiphany");
        let month = self.get_month(0); // January
        self.resolve(&template, &[("day", &day.to_string()), ("month", &month)])
    }

    fn resolve_after_epiphany(&self, dow: u8) -> String {
        let template = self.get_christmas_template("after_epiphany");
        let weekday = self.get_weekday_capitalized(dow);
        self.resolve(
            &template,
            &[("dow", &dow.to_string()), ("weekday", &weekday)],
        )
    }

    fn resolve_second_sunday_after_christmas(&self) -> String {
        self.get_christmas_template("second_sunday_after_christmas")
    }

    fn get_christmas_template(&self, key: &str) -> String {
        self.metadata
            .seasons
            .as_ref()
            .and_then(|s| s.christmas_time.as_ref())
            .and_then(|c| self.get_christmas_field(c, key))
            .unwrap_or_default()
    }

    fn get_christmas_field(&self, christmas: &ChristmasTimeSeason, key: &str) -> Option<String> {
        match key {
            "season" => christmas.season.clone(),
            "day" => christmas.day.clone(),
            "octave" => christmas.octave.clone(),
            "before_epiphany" => christmas.before_epiphany.clone(),
            "second_sunday_after_christmas" => christmas.second_sunday_after_christmas.clone(),
            "after_epiphany" => christmas.after_epiphany.clone(),
            _ => None,
        }
    }

    // -------------------------------------------------------------------------
    // Ordinary Time helpers
    // -------------------------------------------------------------------------

    fn resolve_ordinary_time_sunday(&self, week: u8) -> String {
        let template = self.get_ordinary_time_template("sunday");
        let ordinal = self.get_ordinal(week as u32, Some(GrammaticalGender::Default));
        let ordinal_feminine = self.get_ordinal(week as u32, Some(GrammaticalGender::Feminine));
        self.resolve(
            &template,
            &[
                ("week", &week.to_string()),
                ("ordinal", &ordinal),
                ("ordinal_feminine", &ordinal_feminine),
            ],
        )
    }

    fn resolve_ordinary_time_weekday(&self, week: u8, dow: u8) -> String {
        let template = self.get_ordinary_time_template("weekday");
        let ordinal = self.get_ordinal(week as u32, Some(GrammaticalGender::Feminine));
        let weekday = self.get_weekday_capitalized(dow);
        self.resolve(
            &template,
            &[
                ("week", &week.to_string()),
                ("dow", &dow.to_string()),
                ("ordinal", &ordinal),
                (
                    "ordinal_feminine",
                    &self.get_ordinal(week as u32, Some(GrammaticalGender::Feminine)),
                ),
                ("weekday", &weekday),
            ],
        )
    }

    fn get_ordinary_time_template(&self, key: &str) -> String {
        self.metadata
            .seasons
            .as_ref()
            .and_then(|s| s.ordinary_time.as_ref())
            .and_then(|o| self.get_ordinary_time_field(o, key))
            .unwrap_or_default()
    }

    fn get_ordinary_time_field(&self, ordinary: &OrdinaryTimeSeason, key: &str) -> Option<String> {
        match key {
            "season" => ordinary.season.clone(),
            "weekday" => ordinary.weekday.clone(),
            "sunday" => ordinary.sunday.clone(),
            _ => None,
        }
    }

    // -------------------------------------------------------------------------
    // Lent helpers
    // -------------------------------------------------------------------------

    fn resolve_lent_sunday(&self, week: u8) -> String {
        let template = self.get_lent_template("sunday");
        let ordinal = self.get_ordinal(week as u32, Some(GrammaticalGender::Default));
        let ordinal_feminine = self.get_ordinal(week as u32, Some(GrammaticalGender::Feminine));
        self.resolve(
            &template,
            &[
                ("week", &week.to_string()),
                ("ordinal", &ordinal),
                ("ordinal_feminine", &ordinal_feminine),
            ],
        )
    }

    fn resolve_lent_weekday(&self, week: u8, dow: u8) -> String {
        let template = self.get_lent_template("weekday");
        let ordinal = self.get_ordinal(week as u32, Some(GrammaticalGender::Feminine));
        let weekday = self.get_weekday_capitalized(dow);
        self.resolve(
            &template,
            &[
                ("week", &week.to_string()),
                ("dow", &dow.to_string()),
                ("ordinal", &ordinal),
                (
                    "ordinal_feminine",
                    &self.get_ordinal(week as u32, Some(GrammaticalGender::Feminine)),
                ),
                ("weekday", &weekday),
            ],
        )
    }

    fn resolve_day_after_ash_wednesday(&self, dow: u8) -> String {
        let template = self.get_lent_template("day_after_ash_wed");
        let weekday = self.get_weekday_capitalized(dow);
        self.resolve(
            &template,
            &[("dow", &dow.to_string()), ("weekday", &weekday)],
        )
    }

    fn resolve_holy_week_day(&self, dow: u8) -> String {
        let template = self.get_lent_template("holy_week_day");
        let weekday = self.get_weekday_capitalized(dow);
        self.resolve(
            &template,
            &[("dow", &dow.to_string()), ("weekday", &weekday)],
        )
    }

    fn get_lent_template(&self, key: &str) -> String {
        self.metadata
            .seasons
            .as_ref()
            .and_then(|s| s.lent.as_ref())
            .and_then(|l| self.get_lent_field(l, key))
            .unwrap_or_default()
    }

    fn get_lent_field(&self, lent: &LentSeason, key: &str) -> Option<String> {
        match key {
            "season" => lent.season.clone(),
            "weekday" => lent.weekday.clone(),
            "sunday" => lent.sunday.clone(),
            "day_after_ash_wed" => lent.day_after_ash_wed.clone(),
            "holy_week_day" => lent.holy_week_day.clone(),
            _ => None,
        }
    }

    // -------------------------------------------------------------------------
    // Paschal Triduum helpers
    // -------------------------------------------------------------------------

    fn resolve_triduum_day(&self, day_id: &str) -> String {
        // For Triduum days, we use the entity names directly
        // These are fixed celebrations, not templated
        match day_id {
            "holy_thursday" => "Holy Thursday".to_string(),
            "good_friday" => "Good Friday".to_string(),
            "holy_saturday" => "Holy Saturday".to_string(),
            _ => day_id.to_string(),
        }
    }

    // -------------------------------------------------------------------------
    // Easter Time helpers
    // -------------------------------------------------------------------------

    fn resolve_easter_sunday(&self) -> String {
        "Easter Sunday".to_string()
    }

    fn resolve_easter_octave(&self, dow: u8) -> String {
        let template = self.get_easter_time_template("octave");
        let weekday = self.get_weekday_capitalized(dow);
        self.resolve(
            &template,
            &[("dow", &dow.to_string()), ("weekday", &weekday)],
        )
    }

    fn resolve_easter_time_sunday(&self, week: u8) -> String {
        let template = self.get_easter_time_template("sunday");
        let ordinal = self.get_ordinal(week as u32, Some(GrammaticalGender::Default));
        let ordinal_feminine = self.get_ordinal(week as u32, Some(GrammaticalGender::Feminine));
        self.resolve(
            &template,
            &[
                ("week", &week.to_string()),
                ("ordinal", &ordinal),
                ("ordinal_feminine", &ordinal_feminine),
            ],
        )
    }

    fn resolve_easter_time_weekday(&self, week: u8, dow: u8) -> String {
        let template = self.get_easter_time_template("weekday");
        let ordinal = self.get_ordinal(week as u32, Some(GrammaticalGender::Feminine));
        let weekday = self.get_weekday_capitalized(dow);
        self.resolve(
            &template,
            &[
                ("week", &week.to_string()),
                ("dow", &dow.to_string()),
                ("ordinal", &ordinal),
                (
                    "ordinal_feminine",
                    &self.get_ordinal(week as u32, Some(GrammaticalGender::Feminine)),
                ),
                ("weekday", &weekday),
            ],
        )
    }

    fn get_easter_time_template(&self, key: &str) -> String {
        self.metadata
            .seasons
            .as_ref()
            .and_then(|s| s.easter_time.as_ref())
            .and_then(|e| self.get_easter_time_field(e, key))
            .unwrap_or_default()
    }

    fn get_easter_time_field(&self, easter: &EasterTimeSeason, key: &str) -> Option<String> {
        match key {
            "season" => easter.season.clone(),
            "weekday" => easter.weekday.clone(),
            "sunday" => easter.sunday.clone(),
            "octave" => easter.octave.clone(),
            _ => None,
        }
    }
}

// -------------------------------------------------------------------------
// Utility functions
// -------------------------------------------------------------------------

/// Capitalizes the first letter of a string.
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_simple_template() {
        let metadata = ResourcesMetadata {
            ordinal_format: None,
            ordinals_letters: None,
            ordinals_numeric: None,
            weekdays: None,
            months: None,
            colors: None,
            seasons: None,
            periods: None,
            ranks: None,
            cycles: None,
        };
        let resolver = TemplateResolver::new(metadata, "en".to_string(), OrdinalFormat::Letters);

        let result = resolver.resolve("{a} and {b}", &[("a", "Hello"), ("b", "World")]);
        assert_eq!(result, "Hello and World");
    }

    #[test]
    fn test_capitalize_first() {
        assert_eq!(capitalize_first("hello"), "Hello");
        assert_eq!(capitalize_first("HELLO"), "HELLO");
        assert_eq!(capitalize_first(""), "");
        assert_eq!(capitalize_first("h"), "H");
    }

    #[test]
    fn test_gender_suffix() {
        assert_eq!(GrammaticalGender::Default.suffix(), "");
        assert_eq!(GrammaticalGender::Masculine.suffix(), "_masculine");
        assert_eq!(GrammaticalGender::Feminine.suffix(), "_feminine");
        assert_eq!(GrammaticalGender::Neuter.suffix(), "_neuter");
    }

    #[test]
    fn test_get_ordinal_letters_with_gender() {
        let mut ordinals_letters = BTreeMap::new();
        ordinals_letters.insert("1".to_string(), "first".to_string());
        ordinals_letters.insert("1_feminine".to_string(), "première".to_string());
        ordinals_letters.insert("2".to_string(), "second".to_string());

        let metadata = ResourcesMetadata {
            ordinal_format: None,
            ordinals_letters: Some(ordinals_letters),
            ordinals_numeric: None,
            weekdays: None,
            months: None,
            colors: None,
            seasons: None,
            periods: None,
            ranks: None,
            cycles: None,
        };
        let resolver = TemplateResolver::new(metadata, "fr".to_string(), OrdinalFormat::Letters);

        // Default should return "first"
        assert_eq!(
            resolver.get_ordinal(1, Some(GrammaticalGender::Default)),
            "first"
        );
        // Feminine should return "première"
        assert_eq!(
            resolver.get_ordinal(1, Some(GrammaticalGender::Feminine)),
            "première"
        );
        // Masculine falls back to default "first" (no _masculine key)
        assert_eq!(
            resolver.get_ordinal(1, Some(GrammaticalGender::Masculine)),
            "first"
        );
        // Non-existent ordinal falls back to number
        assert_eq!(resolver.get_ordinal(99, None), "99");
    }

    #[test]
    fn test_get_ordinal_numeric_format() {
        let mut ordinals_numeric = BTreeMap::new();
        ordinals_numeric.insert("1".to_string(), "1er".to_string());
        ordinals_numeric.insert("1_feminine".to_string(), "1re".to_string());
        ordinals_numeric.insert("2".to_string(), "2e".to_string());

        let metadata = ResourcesMetadata {
            ordinal_format: None,
            ordinals_letters: None,
            ordinals_numeric: Some(ordinals_numeric),
            weekdays: None,
            months: None,
            colors: None,
            seasons: None,
            periods: None,
            ranks: None,
            cycles: None,
        };
        let resolver = TemplateResolver::new(metadata, "fr".to_string(), OrdinalFormat::Numeric);

        // Default should return "1er"
        assert_eq!(
            resolver.get_ordinal(1, Some(GrammaticalGender::Default)),
            "1er"
        );
        // Feminine should return "1re"
        assert_eq!(
            resolver.get_ordinal(1, Some(GrammaticalGender::Feminine)),
            "1re"
        );
        // Non-existent ordinal falls back to number
        assert_eq!(resolver.get_ordinal(99, None), "99");
    }

    #[test]
    fn test_ordinal_format_selection() {
        let mut ordinals_letters = BTreeMap::new();
        ordinals_letters.insert("1".to_string(), "first".to_string());

        let mut ordinals_numeric = BTreeMap::new();
        ordinals_numeric.insert("1".to_string(), "1st".to_string());

        let metadata = ResourcesMetadata {
            ordinal_format: None,
            ordinals_letters: Some(ordinals_letters),
            ordinals_numeric: Some(ordinals_numeric),
            weekdays: None,
            months: None,
            colors: None,
            seasons: None,
            periods: None,
            ranks: None,
            cycles: None,
        };

        // With Letters format
        let resolver_letters =
            TemplateResolver::new(metadata.clone(), "en".to_string(), OrdinalFormat::Letters);
        assert_eq!(resolver_letters.get_ordinal(1, None), "first");

        // With Numeric format
        let resolver_numeric =
            TemplateResolver::new(metadata, "en".to_string(), OrdinalFormat::Numeric);
        assert_eq!(resolver_numeric.get_ordinal(1, None), "1st");
    }

    #[test]
    fn test_ordinal_fallback_letters_to_numeric() {
        // Only ordinals_numeric is defined, but format is Letters
        let mut ordinals_numeric = BTreeMap::new();
        ordinals_numeric.insert("1".to_string(), "1st".to_string());
        ordinals_numeric.insert("2".to_string(), "2nd".to_string());

        let metadata = ResourcesMetadata {
            ordinal_format: None,
            ordinals_letters: None, // Not defined
            ordinals_numeric: Some(ordinals_numeric),
            weekdays: None,
            months: None,
            colors: None,
            seasons: None,
            periods: None,
            ranks: None,
            cycles: None,
        };

        // Format is Letters, but ordinals_letters is None
        // Should fall back to ordinals_numeric
        let resolver = TemplateResolver::new(metadata, "en".to_string(), OrdinalFormat::Letters);
        assert_eq!(resolver.get_ordinal(1, None), "1st");
        assert_eq!(resolver.get_ordinal(2, None), "2nd");
    }

    #[test]
    fn test_ordinal_fallback_numeric_to_letters() {
        // Only ordinals_letters is defined, but format is Numeric
        let mut ordinals_letters = BTreeMap::new();
        ordinals_letters.insert("1".to_string(), "first".to_string());
        ordinals_letters.insert("2".to_string(), "second".to_string());

        let metadata = ResourcesMetadata {
            ordinal_format: None,
            ordinals_letters: Some(ordinals_letters),
            ordinals_numeric: None, // Not defined
            weekdays: None,
            months: None,
            colors: None,
            seasons: None,
            periods: None,
            ranks: None,
            cycles: None,
        };

        // Format is Numeric, but ordinals_numeric is None
        // Should fall back to ordinals_letters
        let resolver = TemplateResolver::new(metadata, "en".to_string(), OrdinalFormat::Numeric);
        assert_eq!(resolver.get_ordinal(1, None), "first");
        assert_eq!(resolver.get_ordinal(2, None), "second");
    }

    #[test]
    fn test_ordinal_fallback_to_raw_number() {
        // Neither ordinals_letters nor ordinals_numeric defined
        let metadata = ResourcesMetadata {
            ordinal_format: None,
            ordinals_letters: None,
            ordinals_numeric: None,
            weekdays: None,
            months: None,
            colors: None,
            seasons: None,
            periods: None,
            ranks: None,
            cycles: None,
        };

        // Should fall back to raw number
        let resolver = TemplateResolver::new(metadata, "en".to_string(), OrdinalFormat::Letters);
        assert_eq!(resolver.get_ordinal(1, None), "1");
        assert_eq!(resolver.get_ordinal(42, None), "42");
    }

    #[test]
    fn test_ordinal_fallback_missing_key_in_primary() {
        // ordinals_letters has key "1" but not "2"
        // ordinals_numeric has key "2"
        let mut ordinals_letters = BTreeMap::new();
        ordinals_letters.insert("1".to_string(), "first".to_string());

        let mut ordinals_numeric = BTreeMap::new();
        ordinals_numeric.insert("2".to_string(), "2nd".to_string());

        let metadata = ResourcesMetadata {
            ordinal_format: None,
            ordinals_letters: Some(ordinals_letters),
            ordinals_numeric: Some(ordinals_numeric),
            weekdays: None,
            months: None,
            colors: None,
            seasons: None,
            periods: None,
            ranks: None,
            cycles: None,
        };

        // Format is Letters
        let resolver = TemplateResolver::new(metadata, "en".to_string(), OrdinalFormat::Letters);
        // Key "1" exists in ordinals_letters
        assert_eq!(resolver.get_ordinal(1, None), "first");
        // Key "2" doesn't exist in ordinals_letters, falls back to ordinals_numeric
        assert_eq!(resolver.get_ordinal(2, None), "2nd");
        // Key "3" doesn't exist in either, falls back to raw number
        assert_eq!(resolver.get_ordinal(3, None), "3");
    }
}
