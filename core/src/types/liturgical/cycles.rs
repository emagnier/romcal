#[cfg(feature = "schema-gen")]
use crate::types::mass::mass_definition::LiturgicalCycle;
#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// A three-year cycle for Sunday Mass readings (and some solemnities), designated by A, B, or C.
/// Each cycle begins on the First Sunday of Advent of the previous civil year and ends on Saturday
/// after the Christ the King Solemnity. The cycles follow each other in alphabetical order.
/// C year is always divisible by 3, A has remainder of 1, and B remainder of 2.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SundayCycle {
    /// Year A
    YearA,
    /// Year B
    YearB,
    /// Year C
    YearC,
}

/// Combined Sunday cycle for cases where readings can apply to multiple years.
/// This allows for flexible configuration where the same readings can be used
/// across different combinations of Sunday cycles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SundayCycleCombined {
    /// Years A and B combined
    YearAB,
    /// Years A and C combined
    YearAC,
    /// Years B and C combined
    YearBC,
}

/// A two-year cycle for the weekday Mass readings (also called Cycle I and Cycle II).
/// Odd-numbered years are the Cycle I (year 1); even-numbered ones are the Cycle II (year 2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(non_camel_case_types)] // Intentionally using Year_1/Year_2 to produce YEAR_1/YEAR_2 in JSON
pub enum WeekdayCycle {
    /// Year 1 (Cycle I)
    Year_1,
    /// Year 2 (Cycle II)
    Year_2,
}

/// [GILH §133] The four-week cycle of the psalter is coordinated with the liturgical year in such a way that
/// on the First Sunday of Advent, the First Sunday in Ordinary Time, the First Sunday of Lent,
/// and Easter Sunday the cycle is always begun again with Week 1 (others being omitted when necessary).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, EnumIter)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(non_camel_case_types)] // Intentionally using Week_1/Week_2/etc. to produce WEEK_1/WEEK_2/etc. in JSON
pub enum PsalterWeekCycle {
    /// Week 1
    Week_1,
    /// Week 2
    Week_2,
    /// Week 3
    Week_3,
    /// Week 4
    Week_4,
}

impl SundayCycle {
    /// Determines the Sunday cycle for a given liturgical year.
    ///
    /// The Sunday cycle follows a 3-year pattern (A, B, C) where:
    /// - Year C: years divisible by 3
    /// - Year A: years with remainder 1 when divided by 3
    /// - Year B: years with remainder 2 when divided by 3
    ///
    /// Each cycle begins on the First Sunday of Advent of the previous civil year
    /// and ends on Saturday after the Christ the King Solemnity.
    ///
    /// # Arguments
    ///
    /// * `year` - The liturgical year
    ///
    /// # Returns
    ///
    /// The corresponding Sunday cycle for the given year
    ///
    /// # Examples
    ///
    /// ```rust
    /// use romcal::types::liturgical::cycles::SundayCycle;
    ///
    /// assert_eq!(SundayCycle::from_year(2023), SundayCycle::YearA); // 2023 % 3 = 1
    /// assert_eq!(SundayCycle::from_year(2024), SundayCycle::YearB); // 2024 % 3 = 2
    /// assert_eq!(SundayCycle::from_year(2025), SundayCycle::YearC); // 2025 % 3 = 0
    /// ```
    pub fn from_year(year: i32) -> Self {
        match (year + 2) % 3 {
            0 => SundayCycle::YearA,
            1 => SundayCycle::YearB,
            2 => SundayCycle::YearC,
            _ => unreachable!(), // This should never happen with modulo 3
        }
    }
}

impl WeekdayCycle {
    /// Determines the weekday cycle for a given liturgical year.
    ///
    /// The weekday cycle follows a 2-year pattern where:
    /// - Year 1 (Cycle I): odd-numbered years
    /// - Year 2 (Cycle II): even-numbered years
    ///
    /// # Arguments
    ///
    /// * `year` - The liturgical year
    ///
    /// # Returns
    ///
    /// The corresponding weekday cycle for the given year
    ///
    /// # Examples
    ///
    /// ```rust
    /// use romcal::types::liturgical::cycles::WeekdayCycle;
    ///
    /// assert_eq!(WeekdayCycle::from_year(2023), WeekdayCycle::Year_1); // odd year
    /// assert_eq!(WeekdayCycle::from_year(2024), WeekdayCycle::Year_2); // even year
    /// assert_eq!(WeekdayCycle::from_year(2025), WeekdayCycle::Year_1); // odd year
    /// ```
    pub fn from_year(year: i32) -> Self {
        if year % 2 == 0 {
            WeekdayCycle::Year_2
        } else {
            WeekdayCycle::Year_1
        }
    }
}

impl PsalterWeekCycle {
    /// Determines the psalter week cycle based on the week of the liturgical season.
    ///
    /// The psalter week cycle follows a 4-week pattern (Week1, Week2, Week3, Week4) that
    /// restarts at the beginning of each liturgical season. There are special exceptions:
    ///
    /// 1. During the first four days of Lent (Ash Wednesday to the next Saturday),
    ///    which are in week 4, to start on week 1 after the first Sunday of Lent.
    /// 2. According to GILH §133, the psalter week cycle should not restart for Christmas Time
    ///    due to the fact that December 25 does not always start on Sunday.
    ///
    /// # Arguments
    ///
    /// * `week_of_season` - The week number within the current liturgical season
    /// * `is_lent` - Whether this is during the Lent season
    /// * `is_christmas_time` - Whether this is during Christmas Time
    ///
    /// # Returns
    ///
    /// The corresponding psalter week cycle
    ///
    /// # Examples
    ///
    /// ```rust
    /// use romcal::types::liturgical::cycles::PsalterWeekCycle;
    ///
    /// // Normal case: week 1 of Advent
    /// assert_eq!(PsalterWeekCycle::from_week(1, false, false), PsalterWeekCycle::Week_1);
    ///
    /// // Lent exception: first week of Lent is week 4
    /// assert_eq!(PsalterWeekCycle::from_week(1, true, false), PsalterWeekCycle::Week_4);
    ///
    /// // Christmas Time: special calculation
    /// assert_eq!(PsalterWeekCycle::from_week(2, false, true), PsalterWeekCycle::Week_1);
    /// ```
    pub fn from_week(week_of_season: u32, is_lent: bool, is_christmas_time: bool) -> Self {
        if is_lent && week_of_season == 1 {
            // Special case for Lent: first week is week 4
            PsalterWeekCycle::Week_4
        } else if is_lent {
            // Lent: after week 1 (which is Week_4), continue with Week_1, Week_2, Week_3
            let week_index = (week_of_season + 2) % 4;
            match week_index {
                0 => PsalterWeekCycle::Week_1,
                1 => PsalterWeekCycle::Week_2,
                2 => PsalterWeekCycle::Week_3,
                3 => PsalterWeekCycle::Week_4,
                _ => unreachable!(), // This should never happen with modulo 4
            }
        } else if is_christmas_time {
            // Christmas Time: special calculation based on GILH §133
            let week_index = (2 + week_of_season) % 4;
            match week_index {
                0 => PsalterWeekCycle::Week_1,
                1 => PsalterWeekCycle::Week_2,
                2 => PsalterWeekCycle::Week_3,
                3 => PsalterWeekCycle::Week_4,
                _ => unreachable!(), // This should never happen with modulo 4
            }
        } else {
            // Normal case: cycle restarts at beginning of each season
            let week_index = (week_of_season + 3) % 4; // Equivalent to (week_of_season - 1) % 4 but safe
            match week_index {
                0 => PsalterWeekCycle::Week_1,
                1 => PsalterWeekCycle::Week_2,
                2 => PsalterWeekCycle::Week_3,
                3 => PsalterWeekCycle::Week_4,
                _ => unreachable!(), // This should never happen with modulo 4
            }
        }
    }
}

// Schema generation functions (only compiled when feature "schema-gen" is enabled)
#[cfg(feature = "schema-gen")]
pub fn get_liturgical_cycle_description(cycle: &LiturgicalCycle) -> &'static str {
    match cycle {
        LiturgicalCycle::Invariant => "Invariant content that applies to all cycles",
        LiturgicalCycle::YearA => "Year A of the Sunday cycle",
        LiturgicalCycle::YearB => "Year B of the Sunday cycle",
        LiturgicalCycle::YearC => "Year C of the Sunday cycle",
        LiturgicalCycle::YearAB => "Combined years A and B of the Sunday cycle",
        LiturgicalCycle::YearAC => "Combined years A and C of the Sunday cycle",
        LiturgicalCycle::YearBC => "Combined years B and C of the Sunday cycle",
        LiturgicalCycle::Year1 => "Year 1 of the weekday cycle (Cycle I)", // Note: LiturgicalCycle uses Year1, not Year_1
        LiturgicalCycle::Year2 => "Year 2 of the weekday cycle (Cycle II)", // Note: LiturgicalCycle uses Year2, not Year_2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sunday_cycle_from_year() {
        assert_eq!(SundayCycle::from_year(2020), SundayCycle::YearA); // (2020 + 2) % 3 = 1 -> YearA
        assert_eq!(SundayCycle::from_year(2021), SundayCycle::YearB); // (2021 + 2) % 3 = 2 -> YearB
        assert_eq!(SundayCycle::from_year(2022), SundayCycle::YearC); // (2022 + 2) % 3 = 0 -> YearC
        assert_eq!(SundayCycle::from_year(2023), SundayCycle::YearA); // (2023 + 2) % 3 = 1 -> YearA
        assert_eq!(SundayCycle::from_year(2024), SundayCycle::YearB); // (2024 + 2) % 3 = 2 -> YearB
        assert_eq!(SundayCycle::from_year(2025), SundayCycle::YearC); // (2025 + 2) % 3 = 0 -> YearC
        assert_eq!(SundayCycle::from_year(2026), SundayCycle::YearA); // (2026 + 2) % 3 = 1 -> YearA
        assert_eq!(SundayCycle::from_year(2027), SundayCycle::YearB); // (2027 + 2) % 3 = 2 -> YearB
        assert_eq!(SundayCycle::from_year(2028), SundayCycle::YearC); // (2028 + 2) % 3 = 0 -> YearC
    }

    #[test]
    fn test_weekday_cycle_from_year() {
        assert_eq!(WeekdayCycle::from_year(0), WeekdayCycle::Year_2); // even year -> Year_2
        assert_eq!(WeekdayCycle::from_year(1), WeekdayCycle::Year_1); // odd year -> Year_1
        assert_eq!(WeekdayCycle::from_year(-1), WeekdayCycle::Year_1); // odd year -> Year_1
        assert_eq!(WeekdayCycle::from_year(-2), WeekdayCycle::Year_2); // even year -> Year_2

        assert_eq!(WeekdayCycle::from_year(2020), WeekdayCycle::Year_2); // even year -> Year_2
        assert_eq!(WeekdayCycle::from_year(2021), WeekdayCycle::Year_1); // odd year -> Year_1
        assert_eq!(WeekdayCycle::from_year(2022), WeekdayCycle::Year_2); // even year -> Year_2
        assert_eq!(WeekdayCycle::from_year(2023), WeekdayCycle::Year_1); // odd year -> Year_1
        assert_eq!(WeekdayCycle::from_year(2024), WeekdayCycle::Year_2); // even year -> Year_2
        assert_eq!(WeekdayCycle::from_year(2025), WeekdayCycle::Year_1); // odd year -> Year_1
    }

    #[test]
    fn test_psalter_week_cycle_from_week() {
        // Test normal case (Advent, Easter Time, Ordinary Time)
        assert_eq!(
            PsalterWeekCycle::from_week(1, false, false),
            PsalterWeekCycle::Week_1
        );
        assert_eq!(
            PsalterWeekCycle::from_week(2, false, false),
            PsalterWeekCycle::Week_2
        );
        assert_eq!(
            PsalterWeekCycle::from_week(3, false, false),
            PsalterWeekCycle::Week_3
        );
        assert_eq!(
            PsalterWeekCycle::from_week(4, false, false),
            PsalterWeekCycle::Week_4
        );
        assert_eq!(
            PsalterWeekCycle::from_week(5, false, false),
            PsalterWeekCycle::Week_1
        ); // Cycle repeats

        // Test Lent exception: first week is week 4
        assert_eq!(
            PsalterWeekCycle::from_week(1, true, false),
            PsalterWeekCycle::Week_4
        );
        assert_eq!(
            PsalterWeekCycle::from_week(2, true, false),
            PsalterWeekCycle::Week_1
        );
        assert_eq!(
            PsalterWeekCycle::from_week(3, true, false),
            PsalterWeekCycle::Week_2
        );
        assert_eq!(
            PsalterWeekCycle::from_week(4, true, false),
            PsalterWeekCycle::Week_3
        );

        // Test Christmas Time: special calculation
        assert_eq!(
            PsalterWeekCycle::from_week(1, false, true),
            PsalterWeekCycle::Week_4
        ); // (2+1) % 4 = 3 -> Week_4
        assert_eq!(
            PsalterWeekCycle::from_week(2, false, true),
            PsalterWeekCycle::Week_1
        ); // (2+2) % 4 = 0 -> Week_1
        assert_eq!(
            PsalterWeekCycle::from_week(3, false, true),
            PsalterWeekCycle::Week_2
        ); // (2+3) % 4 = 1 -> Week_2
        assert_eq!(
            PsalterWeekCycle::from_week(4, false, true),
            PsalterWeekCycle::Week_3
        ); // (2+4) % 4 = 2 -> Week_3

        // Test edge cases
        assert_eq!(
            PsalterWeekCycle::from_week(0, false, false),
            PsalterWeekCycle::Week_4
        ); // (0-1) % 4 = 3
        assert_eq!(
            PsalterWeekCycle::from_week(8, false, false),
            PsalterWeekCycle::Week_4
        ); // (8-1) % 4 = 3
    }

    #[test]
    fn test_cycle_consistency() {
        // Test that cycles are consistent across multiple years
        for year in 2020..=2030 {
            let sunday_cycle = SundayCycle::from_year(year);
            let weekday_cycle = WeekdayCycle::from_year(year);

            // Verify that cycles are valid enum values
            match sunday_cycle {
                SundayCycle::YearA | SundayCycle::YearB | SundayCycle::YearC => {}
            }

            match weekday_cycle {
                WeekdayCycle::Year_1 | WeekdayCycle::Year_2 => {}
            }
        }

        // Test psalter week cycle consistency
        for week in 1..=10 {
            let psalter_cycle_normal = PsalterWeekCycle::from_week(week, false, false);
            let psalter_cycle_lent = PsalterWeekCycle::from_week(week, true, false);
            let psalter_cycle_christmas = PsalterWeekCycle::from_week(week, false, true);

            // Verify that cycles are valid enum values
            match psalter_cycle_normal {
                PsalterWeekCycle::Week_1
                | PsalterWeekCycle::Week_2
                | PsalterWeekCycle::Week_3
                | PsalterWeekCycle::Week_4 => {}
            }

            match psalter_cycle_lent {
                PsalterWeekCycle::Week_1
                | PsalterWeekCycle::Week_2
                | PsalterWeekCycle::Week_3
                | PsalterWeekCycle::Week_4 => {}
            }

            match psalter_cycle_christmas {
                PsalterWeekCycle::Week_1
                | PsalterWeekCycle::Week_2
                | PsalterWeekCycle::Week_3
                | PsalterWeekCycle::Week_4 => {}
            }
        }
    }

    // Tests for strum iteration order
    use strum::IntoEnumIterator;

    #[test]
    fn test_sunday_cycle_iteration_order() {
        let variants: Vec<SundayCycle> = SundayCycle::iter().collect();

        // Verify that the order is exactly the declaration order
        assert_eq!(variants[0], SundayCycle::YearA);
        assert_eq!(variants[1], SundayCycle::YearB);
        assert_eq!(variants[2], SundayCycle::YearC);

        // Verify that we have all variants
        assert_eq!(variants.len(), 3);
    }

    #[test]
    fn test_sunday_cycle_combined_iteration_order() {
        let variants: Vec<SundayCycleCombined> = SundayCycleCombined::iter().collect();

        // Verify that the order is exactly the declaration order
        assert_eq!(variants[0], SundayCycleCombined::YearAB);
        assert_eq!(variants[1], SundayCycleCombined::YearAC);
        assert_eq!(variants[2], SundayCycleCombined::YearBC);

        // Verify that we have all variants
        assert_eq!(variants.len(), 3);
    }

    #[test]
    fn test_weekday_cycle_iteration_order() {
        let variants: Vec<WeekdayCycle> = WeekdayCycle::iter().collect();

        // Verify that the order is exactly the declaration order
        assert_eq!(variants[0], WeekdayCycle::Year_1);
        assert_eq!(variants[1], WeekdayCycle::Year_2);

        // Verify that we have all variants
        assert_eq!(variants.len(), 2);
    }

    #[test]
    fn test_psalter_week_cycle_iteration_order() {
        let variants: Vec<PsalterWeekCycle> = PsalterWeekCycle::iter().collect();

        // Verify that the order is exactly the declaration order
        assert_eq!(variants[0], PsalterWeekCycle::Week_1);
        assert_eq!(variants[1], PsalterWeekCycle::Week_2);
        assert_eq!(variants[2], PsalterWeekCycle::Week_3);
        assert_eq!(variants[3], PsalterWeekCycle::Week_4);

        // Verify that we have all variants
        assert_eq!(variants.len(), 4);
    }

    #[test]
    fn test_sunday_cycle_iteration_consistency() {
        // Verify that the order is always the same across multiple iterations
        let first_iteration: Vec<SundayCycle> = SundayCycle::iter().collect();
        let second_iteration: Vec<SundayCycle> = SundayCycle::iter().collect();

        assert_eq!(first_iteration, second_iteration);
    }

    #[test]
    fn test_sunday_cycle_combined_iteration_consistency() {
        // Verify that the order is always the same across multiple iterations
        let first_iteration: Vec<SundayCycleCombined> = SundayCycleCombined::iter().collect();
        let second_iteration: Vec<SundayCycleCombined> = SundayCycleCombined::iter().collect();

        assert_eq!(first_iteration, second_iteration);
    }

    #[test]
    fn test_weekday_cycle_iteration_consistency() {
        // Verify that the order is always the same across multiple iterations
        let first_iteration: Vec<WeekdayCycle> = WeekdayCycle::iter().collect();
        let second_iteration: Vec<WeekdayCycle> = WeekdayCycle::iter().collect();

        assert_eq!(first_iteration, second_iteration);
    }

    #[test]
    fn test_psalter_week_cycle_iteration_consistency() {
        // Verify that the order is always the same across multiple iterations
        let first_iteration: Vec<PsalterWeekCycle> = PsalterWeekCycle::iter().collect();
        let second_iteration: Vec<PsalterWeekCycle> = PsalterWeekCycle::iter().collect();

        assert_eq!(first_iteration, second_iteration);
    }

    #[test]
    fn test_sunday_cycle_serialization() {
        // Verify that serialization works
        let cycle = SundayCycle::YearA;
        let json = serde_json::to_string(&cycle).unwrap();
        assert_eq!(json, "\"YEAR_A\"");

        let deserialized: SundayCycle = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, SundayCycle::YearA);
    }

    #[test]
    fn test_sunday_cycle_combined_serialization() {
        // Verify that serialization works
        let cycle = SundayCycleCombined::YearAB;
        let json = serde_json::to_string(&cycle).unwrap();
        assert_eq!(json, "\"YEAR_A_B\"");

        let deserialized: SundayCycleCombined = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, SundayCycleCombined::YearAB);
    }

    #[test]
    fn test_weekday_cycle_serialization() {
        // Verify that serialization works
        let cycle = WeekdayCycle::Year_1;
        let json = serde_json::to_string(&cycle).unwrap();
        assert_eq!(json, "\"YEAR_1\"");

        let deserialized: WeekdayCycle = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, WeekdayCycle::Year_1);
    }

    #[test]
    fn test_psalter_week_cycle_serialization() {
        // Verify that serialization works
        let cycle = PsalterWeekCycle::Week_1;
        let json = serde_json::to_string(&cycle).unwrap();
        assert_eq!(json, "\"WEEK_1\"");

        let deserialized: PsalterWeekCycle = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, PsalterWeekCycle::Week_1);
    }
}
