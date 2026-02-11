use chrono::{Datelike, NaiveDate, Weekday};

use super::*;
use crate::engine::proper_of_time::utils::PROPER_OF_TIME_ID;
use crate::romcal::Romcal;
use crate::types::dates::DateDef;
use crate::types::liturgical::Precedence;
use crate::types::mass::MassTime;

// ============================================================================
// Calendar creation and utility tests
// ============================================================================

#[test]
fn test_calendar_creation() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();

    assert_eq!(calendar.year, 2026);
    // Liturgical year 2026 starts on November 30, 2025 (First Sunday of Advent)
    assert_eq!(calendar.start_of_year.month(), 11);
    assert_eq!(calendar.start_of_year.year(), 2025);
}

#[test]
fn test_day_of_week_conversion() {
    assert_eq!(Calendar::day_of_week_to_weekday(0), Weekday::Sun);
    assert_eq!(Calendar::day_of_week_to_weekday(1), Weekday::Mon);
    assert_eq!(Calendar::day_of_week_to_weekday(6), Weekday::Sat);
}

#[test]
fn test_last_day_of_month() {
    assert_eq!(
        Calendar::last_day_of_month(2024, 2).unwrap(),
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap() // Leap year
    );
    assert_eq!(
        Calendar::last_day_of_month(2025, 2).unwrap(),
        NaiveDate::from_ymd_opt(2025, 2, 28).unwrap() // Non-leap year
    );
    assert_eq!(
        Calendar::last_day_of_month(2024, 12).unwrap(),
        NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()
    );
}

// ============================================================================
// Precedence tests
// ============================================================================

#[test]
fn test_precedence_comparison() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();

    // Create mock days with different precedences
    use crate::types::liturgical::{PsalterWeekCycle, SundayCycle, WeekdayCycle};

    let triduum_day = LiturgicalDay::new(
        "triduum".to_string(),
        "Triduum".to_string(),
        "2026-04-09".to_string(),
        DateDef::MonthDate {
            month: crate::types::dates::MonthIndex(4),
            date: 9,
            day_offset: None,
        },
        Precedence::Triduum_1,
        crate::types::liturgical::Rank::Weekday,
        "Weekday".to_string(),
        SundayCycle::YearA,
        "Year A".to_string(),
        WeekdayCycle::Year_1,
        "Year I".to_string(),
        PsalterWeekCycle::Week_1,
        "Week 1".to_string(),
        "test".to_string(),
    );

    let weekday = LiturgicalDay::new(
        "weekday".to_string(),
        "Weekday".to_string(),
        "2026-04-09".to_string(),
        DateDef::MonthDate {
            month: crate::types::dates::MonthIndex(4),
            date: 9,
            day_offset: None,
        },
        Precedence::Weekday_13,
        crate::types::liturgical::Rank::Weekday,
        "Weekday".to_string(),
        SundayCycle::YearA,
        "Year A".to_string(),
        WeekdayCycle::Year_1,
        "Year I".to_string(),
        PsalterWeekCycle::Week_1,
        "Week 1".to_string(),
        "test".to_string(),
    );

    // Triduum should have higher precedence (less) than weekday
    assert_eq!(
        calendar.compare_precedence(&triduum_day, &weekday),
        std::cmp::Ordering::Less
    );
}

// ============================================================================
// Date calculation tests
// ============================================================================

#[test]
fn test_calculate_month_date() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();

    // Test simple month/date
    let date_def = DateDef::MonthDate {
        month: crate::types::dates::MonthIndex(12),
        date: 25,
        day_offset: None,
    };
    let result = calendar.calculate_date_from_def(&date_def, 0).unwrap();
    assert_eq!(result, Some(NaiveDate::from_ymd_opt(2026, 12, 25).unwrap()));

    // Test with day offset
    let date_def_with_offset = DateDef::MonthDate {
        month: crate::types::dates::MonthIndex(12),
        date: 25,
        day_offset: Some(-1),
    };
    let result_offset = calendar
        .calculate_date_from_def(&date_def_with_offset, 0)
        .unwrap();
    assert_eq!(
        result_offset,
        Some(NaiveDate::from_ymd_opt(2026, 12, 24).unwrap())
    );
}

#[test]
fn test_calculate_date_function() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();

    // Test Easter Sunday
    use crate::types::dates::DateFn;
    let date_def = DateDef::DateFunction {
        date_fn: DateFn::EasterSunday,
        day_offset: None,
    };
    let result = calendar.calculate_date_from_def(&date_def, 0).unwrap();
    // Easter 2026 is April 5
    assert_eq!(result, Some(NaiveDate::from_ymd_opt(2026, 4, 5).unwrap()));

    // Test with offset (Pentecost = Easter + 49 days)
    let pentecost_def = DateDef::DateFunction {
        date_fn: DateFn::EasterSunday,
        day_offset: Some(49),
    };
    let pentecost = calendar.calculate_date_from_def(&pentecost_def, 0).unwrap();
    assert_eq!(
        pentecost,
        Some(NaiveDate::from_ymd_opt(2026, 5, 24).unwrap())
    );
}

#[test]
fn test_calculate_weekday_of_month() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();

    // Test 3rd Sunday of September (e.g., for Catechetical Sunday)
    let date_def = DateDef::WeekdayOfMonth {
        month: crate::types::dates::MonthIndex(9),
        day_of_week: crate::types::dates::DayOfWeek(0), // Sunday
        nth_week_in_month: 3,
        day_offset: None,
    };
    let result = calendar.calculate_date_from_def(&date_def, 0).unwrap();
    // 3rd Sunday of September 2026 is September 20
    let expected_date = result.unwrap();
    assert_eq!(expected_date.month(), 9);
    assert_eq!(expected_date.weekday(), Weekday::Sun);
}

#[test]
fn test_calculate_last_weekday_of_month() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();

    // Test last Sunday of November (Christ the King region)
    let date_def = DateDef::LastWeekdayOfMonth {
        month: crate::types::dates::MonthIndex(11),
        last_day_of_week_in_month: crate::types::dates::DayOfWeek(0), // Sunday
        day_offset: None,
    };
    let result = calendar.calculate_date_from_def(&date_def, 0).unwrap();
    let expected_date = result.unwrap();
    assert_eq!(expected_date.month(), 11);
    assert_eq!(expected_date.weekday(), Weekday::Sun);
    // Last Sunday of November 2026 is November 29
    assert_eq!(expected_date.day(), 29);
}

// ============================================================================
// Calendar generation tests
// ============================================================================

#[test]
fn test_generate_calendar_basic() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();

    let result = calendar.generate();
    assert!(result.is_ok());

    let liturgical_calendar = result.unwrap();

    // Should have entries for each day of the liturgical year
    // A liturgical year typically has 365-366 days
    assert!(
        !liturgical_calendar.is_empty(),
        "Calendar should not be empty"
    );

    // Verify we have a reasonable number of dates
    // At minimum, Proper of Time should generate ~365 days
    assert!(
        liturgical_calendar.len() >= 350,
        "Should have at least 350 dates, got {}",
        liturgical_calendar.len()
    );

    // Easter 2026 should be present (Easter 2026 is April 5)
    assert!(
        liturgical_calendar.contains_key("2026-04-05"),
        "Easter 2026 should be present"
    );

    // Pentecost 2026 should be present (49 days after Easter = May 24)
    assert!(
        liturgical_calendar.contains_key("2026-05-24"),
        "Pentecost 2026 should be present"
    );

    // Check that days have proper structure
    let easter_days = liturgical_calendar.get("2026-04-05").unwrap();
    assert!(
        !easter_days.is_empty(),
        "Easter should have at least one entry"
    );

    let easter = &easter_days[0];
    assert_eq!(
        easter.precedence,
        Precedence::Triduum_1,
        "Easter should have Triduum precedence"
    );
}

#[test]
fn test_precedence_order_all_levels() {
    // Test that all precedence levels are in correct order
    use strum::IntoEnumIterator;

    let precedences: Vec<Precedence> = Precedence::iter().collect();

    // Verify first few are highest precedence
    assert_eq!(precedences[0], Precedence::Triduum_1);
    assert_eq!(precedences[1], Precedence::ProperOfTimeSolemnity_2);
    assert_eq!(precedences[2], Precedence::PrivilegedSunday_2);

    // Verify last is lowest precedence
    assert_eq!(precedences[precedences.len() - 1], Precedence::Weekday_13);
}

#[test]
fn test_apply_precedence_rules_single_day() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();

    use crate::types::liturgical::{PsalterWeekCycle, Rank, SundayCycle, WeekdayCycle};

    let mut days = vec![LiturgicalDay::new(
        "single_day".to_string(),
        "Single Day".to_string(),
        "2026-01-01".to_string(),
        DateDef::MonthDate {
            month: crate::types::dates::MonthIndex(1),
            date: 1,
            day_offset: None,
        },
        Precedence::GeneralSolemnity_3,
        Rank::Solemnity,
        "Solemnity".to_string(),
        SundayCycle::YearA,
        "Year A".to_string(),
        WeekdayCycle::Year_1,
        "Year I".to_string(),
        PsalterWeekCycle::Week_1,
        "Week 1".to_string(),
        "test".to_string(),
    )];

    let result = calendar.apply_precedence_rules(&mut days);
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, "single_day");
}

#[test]
fn test_apply_precedence_rules_multiple_days() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();

    use crate::types::liturgical::{PsalterWeekCycle, Rank, SundayCycle, WeekdayCycle};

    let solemnity = LiturgicalDay::new(
        "solemnity".to_string(),
        "Solemnity".to_string(),
        "2026-01-01".to_string(),
        DateDef::MonthDate {
            month: crate::types::dates::MonthIndex(1),
            date: 1,
            day_offset: None,
        },
        Precedence::GeneralSolemnity_3,
        Rank::Solemnity,
        "Solemnity".to_string(),
        SundayCycle::YearA,
        "Year A".to_string(),
        WeekdayCycle::Year_1,
        "Year I".to_string(),
        PsalterWeekCycle::Week_1,
        "Week 1".to_string(),
        "test".to_string(),
    );

    let memorial = LiturgicalDay::new(
        "memorial".to_string(),
        "Memorial".to_string(),
        "2026-01-01".to_string(),
        DateDef::MonthDate {
            month: crate::types::dates::MonthIndex(1),
            date: 1,
            day_offset: None,
        },
        Precedence::GeneralMemorial_10,
        Rank::Memorial,
        "Memorial".to_string(),
        SundayCycle::YearA,
        "Year A".to_string(),
        WeekdayCycle::Year_1,
        "Year I".to_string(),
        PsalterWeekCycle::Week_1,
        "Week 1".to_string(),
        "test".to_string(),
    );

    let mut days = vec![memorial.clone(), solemnity.clone()];
    let result = calendar.apply_precedence_rules(&mut days);

    // Solemnity should win over Memorial
    assert_eq!(result.len(), 1);
    assert_eq!(result[0].id, "solemnity");
}

#[test]
fn test_optional_memorials_keep_weekday_and_order_by_calendar() {
    use crate::types::liturgical::{PsalterWeekCycle, Rank, SundayCycle, WeekdayCycle};
    use std::collections::HashMap;

    let romcal = Romcal::empty();
    let mut calendar = Calendar::new(romcal, 2026).unwrap();

    // Simulate a hierarchy: general_roman (0) < france (1) < france__angers (2)
    calendar.calendar_priority = HashMap::from([
        ("general_roman".to_string(), 0),
        ("france".to_string(), 1),
        ("france__angers".to_string(), 2),
    ]);

    let mut days = vec![
        LiturgicalDay::new(
            "weekday".to_string(),
            "Weekday".to_string(),
            "2026-06-01".to_string(),
            DateDef::MonthDate {
                month: crate::types::dates::MonthIndex(6),
                date: 1,
                day_offset: None,
            },
            Precedence::Weekday_13,
            Rank::Weekday,
            "Weekday".to_string(),
            SundayCycle::YearA,
            "Year A".to_string(),
            WeekdayCycle::Year_1,
            "Year I".to_string(),
            PsalterWeekCycle::Week_1,
            "Week 1".to_string(),
            "general_roman".to_string(),
        ),
        LiturgicalDay::new(
            "optional_france".to_string(),
            "Optional Memorial France".to_string(),
            "2026-06-01".to_string(),
            DateDef::MonthDate {
                month: crate::types::dates::MonthIndex(6),
                date: 1,
                day_offset: None,
            },
            Precedence::OptionalMemorial_12,
            Rank::OptionalMemorial,
            "Optional Memorial".to_string(),
            SundayCycle::YearA,
            "Year A".to_string(),
            WeekdayCycle::Year_1,
            "Year I".to_string(),
            PsalterWeekCycle::Week_1,
            "Week 1".to_string(),
            "france".to_string(),
        ),
        LiturgicalDay::new(
            "optional_general".to_string(),
            "Optional Memorial General".to_string(),
            "2026-06-01".to_string(),
            DateDef::MonthDate {
                month: crate::types::dates::MonthIndex(6),
                date: 1,
                day_offset: None,
            },
            Precedence::OptionalMemorial_12,
            Rank::OptionalMemorial,
            "Optional Memorial".to_string(),
            SundayCycle::YearA,
            "Year A".to_string(),
            WeekdayCycle::Year_1,
            "Year I".to_string(),
            PsalterWeekCycle::Week_1,
            "Week 1".to_string(),
            "general_roman".to_string(),
        ),
    ];

    let result = calendar.apply_precedence_rules(&mut days);

    assert_eq!(result.len(), 3);
    assert_eq!(result[0].id, "weekday");
    assert_eq!(result[1].id, "optional_general");
    assert_eq!(result[2].id, "optional_france");
}

#[test]
fn test_proper_of_time_end_of_season_not_null() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let result = calendar.generate().unwrap();

    // Check that all days from Proper of Time have end_of_season defined
    for days in result.values() {
        for day in days {
            if day.from_calendar_id == PROPER_OF_TIME_ID {
                assert!(
                    day.end_of_season.is_some(),
                    "Day '{}' from Proper of Time on {} should have end_of_season defined",
                    day.id,
                    day.date
                );
            }
        }
    }
}

#[test]
fn test_parent_override_structure() {
    use crate::engine::liturgical_day::ParentOverride;

    // Test that ParentOverride can be created and checked for changes
    let mut override_empty = ParentOverride::new("test_calendar".to_string());
    assert!(!override_empty.has_changes());

    override_empty.precedence = Some(Precedence::GeneralMemorial_10);
    assert!(override_empty.has_changes());
}

#[test]
fn test_martyr_color_from_titles() {
    use crate::types::martyrology::{Title, TitlesDef};

    // Test with martyr title
    let martyr_titles = TitlesDef::Titles(vec![Title::Bishop, Title::Martyr]);
    assert!(martyr_titles.contains_martyr());

    // Test without martyr title
    let non_martyr_titles = TitlesDef::Titles(vec![Title::Bishop, Title::Virgin]);
    assert!(!non_martyr_titles.contains_martyr());

    // Test TheFirstMartyr
    let first_martyr_titles = TitlesDef::Titles(vec![Title::TheFirstMartyr]);
    assert!(first_martyr_titles.contains_martyr());

    // Test ProtoMartyrOfOceania
    let proto_martyr_titles = TitlesDef::Titles(vec![Title::ProtoMartyrOfOceania]);
    assert!(proto_martyr_titles.contains_martyr());
}

// ============================================================================
// Mass assignment tests
// ============================================================================

#[test]
fn test_masses_default_is_day_mass() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let result = calendar.generate().unwrap();

    // Regular weekday should have default DayMass
    // Pick a regular Advent weekday
    if let Some(days) = result.get("2025-12-01") {
        let day = &days[0];
        assert_eq!(day.masses.len(), 1);
        assert_eq!(day.masses[0].mass_type, MassTime::DayMass);
        assert_eq!(day.masses[0].name, "day_mass");
    }
}

#[test]
fn test_masses_easter_sunday() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let result = calendar.generate().unwrap();

    // Easter Sunday 2026 is April 5
    if let Some(days) = result.get("2026-04-05") {
        let easter = days.iter().find(|d| d.id == "easter_sunday").unwrap();
        assert_eq!(easter.masses.len(), 2);
        assert!(
            easter
                .masses
                .iter()
                .any(|m| m.mass_type == MassTime::EasterVigil)
        );
        assert!(
            easter
                .masses
                .iter()
                .any(|m| m.mass_type == MassTime::DayMass)
        );
    }
}

#[test]
fn test_masses_holy_saturday_is_aliturgical() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let result = calendar.generate().unwrap();

    // Holy Saturday 2026 is April 4
    if let Some(days) = result.get("2026-04-04") {
        let holy_saturday = days.iter().find(|d| d.id == "holy_saturday").unwrap();
        // Holy Saturday is aliturgical - no masses
        assert!(
            holy_saturday.masses.is_empty(),
            "Holy Saturday should have no masses (aliturgical day)"
        );
    }
}

#[test]
fn test_masses_nativity_of_the_lord() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let result = calendar.generate().unwrap();

    // Christmas is December 25
    if let Some(days) = result.get("2025-12-25") {
        let christmas = days
            .iter()
            .find(|d| d.id == "nativity_of_the_lord")
            .unwrap();
        // Christmas has 4 masses
        assert_eq!(christmas.masses.len(), 4);
        assert!(
            christmas
                .masses
                .iter()
                .any(|m| m.mass_type == MassTime::PreviousEveningMass)
        );
        assert!(
            christmas
                .masses
                .iter()
                .any(|m| m.mass_type == MassTime::NightMass)
        );
        assert!(
            christmas
                .masses
                .iter()
                .any(|m| m.mass_type == MassTime::MassAtDawn)
        );
        assert!(
            christmas
                .masses
                .iter()
                .any(|m| m.mass_type == MassTime::DayMass)
        );
    }
}

#[test]
fn test_masses_from_calendar_definition() {
    use crate::engine::calendar_definition::CalendarDefinition;
    use crate::engine::resources::Resources;
    use crate::types::CalendarMetadata;
    use crate::types::calendar::{CalendarJurisdiction, CalendarType, DayDefinition};
    use crate::types::martyrology::MartyrologyEntryDef;
    use crate::types::mass::{MassCycleDefinition, MassesDefinitions};

    // Create a test calendar definition with masses
    let mut day_def = DayDefinition {
        date_def: Some(DateDef::MonthDate {
            month: crate::types::dates::MonthIndex(7),
            date: 15,
            day_offset: None,
        }),
        precedence: Some(Precedence::GeneralSolemnity_3),
        masses: None,
        date_exceptions: None,
        commons_def: None,
        is_holy_day_of_obligation: None,
        allow_similar_rank_items: None,
        is_optional: None,
        custom_locale_id: None,
        martyrology: None,
        titles: None,
        drop: None,
        colors: None,
    };

    // Add masses to the definition
    let mut masses_def = MassesDefinitions::new();
    masses_def.insert(MassTime::PreviousEveningMass, MassCycleDefinition::new());
    masses_def.insert(MassTime::DayMass, MassCycleDefinition::new());
    day_def.masses = Some(masses_def);

    // Create a calendar definition with this day
    let calendar_def = CalendarDefinition {
        schema: None,
        id: "test_calendar".to_string(),
        metadata: CalendarMetadata {
            r#type: CalendarType::GeneralRoman,
            jurisdiction: CalendarJurisdiction::Ecclesiastical,
        },
        particular_config: None,
        parent_calendar_ids: vec![],
        days_definitions: std::collections::BTreeMap::from([(
            "test_solemnity".to_string(),
            day_def,
        )]),
    };

    // Create martyrology entry for test_solemnity (required for strict validation)
    let entry_def = MartyrologyEntryDef {
        fullname: Some("Test Solemnity".to_string()),
        ..Default::default()
    };
    let mut resources = Resources::new("en".to_string());
    resources.add_martyrology_entry("test_solemnity".to_string(), entry_def);

    // Create romcal with this calendar definition and resources
    let mut romcal = Romcal::empty();
    romcal.calendar = "test_calendar".to_string();
    romcal.calendar_definitions.push(calendar_def);
    romcal.add_resources(resources);

    let calendar = Calendar::new(romcal, 2026).unwrap();
    let result = calendar.generate().unwrap();

    // Check July 15
    let days = result.get("2026-07-15").expect("2026-07-15 should exist");
    let test_day = days
        .iter()
        .find(|d| d.id == "test_solemnity")
        .expect("test_solemnity should be on July 15");

    // Should have PreviousEveningMass and DayMass from calendar definition
    assert_eq!(test_day.masses.len(), 2);
    assert!(
        test_day
            .masses
            .iter()
            .any(|m| m.mass_type == MassTime::PreviousEveningMass),
        "test_solemnity should have PreviousEveningMass"
    );
    assert!(
        test_day
            .masses
            .iter()
            .any(|m| m.mass_type == MassTime::DayMass),
        "test_solemnity should have DayMass"
    );
}

#[test]
fn test_masses_pentecost_sunday() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let result = calendar.generate().unwrap();

    // Pentecost 2026 is May 24
    if let Some(days) = result.get("2026-05-24") {
        let pentecost = days
            .iter()
            .find(|d| d.id == "pentecost_sunday")
            .expect("pentecost_sunday should exist");
        assert_eq!(pentecost.masses.len(), 2);
        assert!(
            pentecost
                .masses
                .iter()
                .any(|m| m.mass_type == MassTime::PreviousEveningMass),
            "Pentecost should have PreviousEveningMass"
        );
        assert!(
            pentecost
                .masses
                .iter()
                .any(|m| m.mass_type == MassTime::DayMass),
            "Pentecost should have DayMass"
        );
    }
}

#[test]
fn test_masses_palm_sunday() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let result = calendar.generate().unwrap();

    // Palm Sunday 2026 is March 29
    if let Some(days) = result.get("2026-03-29") {
        let palm_sunday = days
            .iter()
            .find(|d| d.id == "palm_sunday_of_the_passion_of_the_lord")
            .expect("palm_sunday should exist");
        assert_eq!(palm_sunday.masses.len(), 1);
        assert_eq!(palm_sunday.masses[0].mass_type, MassTime::MassOfThePassion);
        assert_eq!(palm_sunday.masses[0].name, "mass_of_the_passion");
    }
}

#[test]
fn test_masses_good_friday() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let result = calendar.generate().unwrap();

    // Good Friday 2026 is April 3
    if let Some(days) = result.get("2026-04-03") {
        let good_friday = days
            .iter()
            .find(|d| d.id == "friday_of_the_passion_of_the_lord")
            .expect("good_friday should exist");
        assert_eq!(good_friday.masses.len(), 1);
        assert_eq!(
            good_friday.masses[0].mass_type,
            MassTime::CelebrationOfThePassion
        );
        assert_eq!(good_friday.masses[0].name, "celebration_of_the_passion");
    }
}

#[test]
fn test_masses_holy_thursday() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let result = calendar.generate().unwrap();

    // Holy Thursday 2026 is April 2
    if let Some(days) = result.get("2026-04-02") {
        let holy_thursday = days
            .iter()
            .find(|d| d.id == "thursday_of_the_lords_supper")
            .expect("holy_thursday should exist");
        assert_eq!(holy_thursday.masses.len(), 1);
        assert_eq!(
            holy_thursday.masses[0].mass_type,
            MassTime::EveningMassOfTheLordsSupper
        );
        assert_eq!(
            holy_thursday.masses[0].name,
            "evening_mass_of_the_lords_supper"
        );
    }
}

#[test]
fn test_masses_december_24() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let result = calendar.generate().unwrap();

    // December 24, 2025 (in liturgical year 2026)
    if let Some(days) = result.get("2025-12-24") {
        let dec_24 = days
            .iter()
            .find(|d| d.id == "advent_december_24")
            .expect("advent_december_24 should exist");
        assert_eq!(dec_24.masses.len(), 1);
        assert_eq!(dec_24.masses[0].mass_type, MassTime::MorningMass);
        assert_eq!(dec_24.masses[0].name, "morning_mass");
    }
}

// ============================================================================
// Mass calendar generation tests
// ============================================================================

#[test]
fn test_generate_mass_calendar_basic() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();

    let result = calendar.generate_mass_calendar();
    assert!(result.is_ok());

    let mass_calendar = result.unwrap();

    // Should have entries for each day of the liturgical year
    assert!(
        !mass_calendar.is_empty(),
        "Mass calendar should not be empty"
    );

    // Should have reasonable number of dates
    assert!(
        mass_calendar.len() >= 350,
        "Should have at least 350 dates, got {}",
        mass_calendar.len()
    );
}

#[test]
fn test_generate_mass_calendar_christmas_evening_mass_shifted() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let mass_calendar = calendar.generate_mass_calendar().unwrap();

    // Christmas PreviousEveningMass should appear on December 24 (civil date)
    // but the liturgical_date should be December 25
    if let Some(masses) = mass_calendar.get("2025-12-24") {
        let evening_mass = masses
            .iter()
            .find(|m| m.mass_time == MassTime::PreviousEveningMass);

        assert!(
            evening_mass.is_some(),
            "Christmas PreviousEveningMass should be on Dec 24"
        );

        let evening_mass = evening_mass.unwrap();
        assert_eq!(evening_mass.civil_date, "2025-12-24");
        assert_eq!(evening_mass.liturgical_date, "2025-12-25");
        assert_eq!(evening_mass.id, "nativity_of_the_lord");
    }

    // Christmas Day masses (NightMass, MassAtDawn, DayMass) should be on December 25
    if let Some(masses) = mass_calendar.get("2025-12-25") {
        let day_masses: Vec<_> = masses
            .iter()
            .filter(|m| m.id == "nativity_of_the_lord")
            .collect();

        assert!(
            day_masses.len() >= 3,
            "Christmas should have at least 3 masses on Dec 25"
        );

        // All should have liturgical_date = civil_date = 2025-12-25
        for mass in day_masses {
            assert_eq!(mass.civil_date, "2025-12-25");
            assert_eq!(mass.liturgical_date, "2025-12-25");
        }
    }
}

#[test]
fn test_generate_mass_calendar_easter_vigil_shifted() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let mass_calendar = calendar.generate_mass_calendar().unwrap();

    // Easter Vigil should appear on Holy Saturday (civil date April 4)
    // but the liturgical_date should be Easter Sunday (April 5)
    if let Some(masses) = mass_calendar.get("2026-04-04") {
        let vigil = masses.iter().find(|m| m.mass_time == MassTime::EasterVigil);

        assert!(vigil.is_some(), "Easter Vigil should be on April 4");

        let vigil = vigil.unwrap();
        assert_eq!(vigil.civil_date, "2026-04-04");
        assert_eq!(vigil.liturgical_date, "2026-04-05");
        assert_eq!(vigil.id, "easter_sunday");
    }
}

#[test]
fn test_generate_mass_calendar_context_from_liturgical_date() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let mass_calendar = calendar.generate_mass_calendar().unwrap();

    // Easter Vigil (on civil date April 4) should have Easter's context, not Holy Saturday's
    if let Some(masses) = mass_calendar.get("2026-04-04") {
        let vigil = masses
            .iter()
            .find(|m| m.mass_time == MassTime::EasterVigil)
            .expect("Easter Vigil should exist");

        // The season should be Easter Time (from Easter Sunday's context)
        // Not Paschal Triduum (from Holy Saturday's context)
        // Note: Easter Sunday is in Easter Time
        assert!(vigil.season.is_some(), "Easter Vigil should have a season");
    }
}

#[test]
fn test_generate_mass_calendar_flat_structure() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let mass_calendar = calendar.generate_mass_calendar().unwrap();

    // Pick any date and verify the flat structure
    if let Some(masses) = mass_calendar.get("2025-12-25") {
        let mass = &masses[0];

        // Mass identification should be present
        assert!(!mass.mass_time_name.is_empty());

        // Day-level context should be directly accessible (flat)
        assert!(!mass.sunday_cycle_name.is_empty());
        assert!(!mass.weekday_cycle_name.is_empty());

        // Celebration data should be directly accessible (flat)
        assert!(!mass.id.is_empty());
        assert!(!mass.fullname.is_empty());
        assert!(!mass.rank_name.is_empty());
    }
}

#[test]
fn test_generate_mass_calendar_serialization() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();
    let mass_calendar = calendar.generate_mass_calendar().unwrap();

    // Serialize to JSON
    let json = serde_json::to_string(&mass_calendar);
    assert!(json.is_ok(), "Mass calendar should serialize to JSON");

    let json_str = json.unwrap();
    // Check that mass_time is serialized as snake_case
    assert!(
        json_str.contains("\"mass_time\":\"day_mass\"")
            || json_str.contains("\"mass_time\":\"previous_evening_mass\"")
            || json_str.contains("\"mass_time\":\"easter_vigil\""),
        "mass_time should be serialized as snake_case"
    );
}

// ============================================================================
// Calendar validation tests
// ============================================================================

#[test]
fn test_calendar_not_found_error() {
    use crate::engine::calendar_definition::CalendarDefinition;
    use crate::types::CalendarMetadata;
    use crate::types::calendar::{CalendarJurisdiction, CalendarType};

    // Create romcal with a definition, but requesting a different calendar
    let some_calendar = CalendarDefinition {
        schema: None,
        id: "some_calendar".to_string(),
        metadata: CalendarMetadata {
            r#type: CalendarType::GeneralRoman,
            jurisdiction: CalendarJurisdiction::Ecclesiastical,
        },
        particular_config: None,
        parent_calendar_ids: vec![],
        days_definitions: std::collections::BTreeMap::new(),
    };

    let mut romcal = Romcal::empty();
    romcal.calendar = "non_existent_calendar".to_string();
    romcal.calendar_definitions.push(some_calendar);

    let result = Calendar::new(romcal, 2026);

    match result {
        Err(error) => {
            let error_str = error.to_string();
            assert!(
                error_str.contains("not found"),
                "Error should mention 'not found': {}",
                error_str
            );
            assert!(
                error_str.contains("non_existent_calendar"),
                "Error should mention the calendar name: {}",
                error_str
            );
        }
        Ok(_) => panic!("Expected an error for non-existent calendar"),
    }
}

#[test]
fn test_parent_calendar_not_found_error() {
    use crate::engine::calendar_definition::CalendarDefinition;
    use crate::types::CalendarMetadata;
    use crate::types::calendar::{CalendarJurisdiction, CalendarType};

    // Create a calendar that references a non-existent parent
    let child_calendar = CalendarDefinition {
        schema: None,
        id: "child_calendar".to_string(),
        metadata: CalendarMetadata {
            r#type: CalendarType::Country,
            jurisdiction: CalendarJurisdiction::Civil,
        },
        particular_config: None,
        parent_calendar_ids: vec!["non_existent_parent".to_string()],
        days_definitions: std::collections::BTreeMap::new(),
    };

    let mut romcal = Romcal::empty();
    romcal.calendar = "child_calendar".to_string();
    romcal.calendar_definitions.push(child_calendar);

    let result = Calendar::new(romcal, 2026);

    match result {
        Err(error) => {
            let error_str = error.to_string();
            assert!(
                error_str.contains("Parent calendar"),
                "Error should mention 'Parent calendar': {}",
                error_str
            );
            assert!(
                error_str.contains("non_existent_parent"),
                "Error should mention the parent calendar name: {}",
                error_str
            );
            assert!(
                error_str.contains("child_calendar"),
                "Error should mention which calendar requires the parent: {}",
                error_str
            );
        }
        Ok(_) => panic!("Expected an error for missing parent calendar"),
    }
}

#[test]
fn test_general_roman_calendar_with_definition() {
    use crate::engine::calendar_definition::CalendarDefinition;
    use crate::types::CalendarMetadata;
    use crate::types::calendar::{CalendarJurisdiction, CalendarType};

    let general_roman = CalendarDefinition {
        schema: None,
        id: "general_roman".to_string(),
        metadata: CalendarMetadata {
            r#type: CalendarType::GeneralRoman,
            jurisdiction: CalendarJurisdiction::Ecclesiastical,
        },
        particular_config: None,
        parent_calendar_ids: vec![],
        days_definitions: std::collections::BTreeMap::new(),
    };

    let mut romcal = Romcal::empty();
    romcal.calendar = "general_roman".to_string();
    romcal.calendar_definitions.push(general_roman);

    let result = Calendar::new(romcal, 2026);
    assert!(
        result.is_ok(),
        "general_roman with explicit definition should work"
    );
}

#[test]
fn test_grandparent_calendar_not_found_error() {
    use crate::engine::calendar_definition::CalendarDefinition;
    use crate::types::CalendarMetadata;
    use crate::types::calendar::{CalendarJurisdiction, CalendarType};

    // Create hierarchy: child → parent → grandparent (missing)
    let parent_calendar = CalendarDefinition {
        schema: None,
        id: "parent_calendar".to_string(),
        metadata: CalendarMetadata {
            r#type: CalendarType::Region,
            jurisdiction: CalendarJurisdiction::Civil,
        },
        particular_config: None,
        parent_calendar_ids: vec!["missing_grandparent".to_string()],
        days_definitions: std::collections::BTreeMap::new(),
    };

    let child_calendar = CalendarDefinition {
        schema: None,
        id: "child_calendar".to_string(),
        metadata: CalendarMetadata {
            r#type: CalendarType::Country,
            jurisdiction: CalendarJurisdiction::Civil,
        },
        particular_config: None,
        parent_calendar_ids: vec!["parent_calendar".to_string()],
        days_definitions: std::collections::BTreeMap::new(),
    };

    let mut romcal = Romcal::empty();
    romcal.calendar = "child_calendar".to_string();
    romcal.calendar_definitions.push(parent_calendar);
    romcal.calendar_definitions.push(child_calendar);

    let result = Calendar::new(romcal, 2026);

    match result {
        Err(error) => {
            let error_str = error.to_string();
            assert!(
                error_str.contains("missing_grandparent"),
                "Error should mention the missing grandparent: {}",
                error_str
            );
        }
        Ok(_) => panic!("Expected an error for missing grandparent calendar"),
    }
}

#[test]
fn test_multiple_parents_one_missing_error() {
    use crate::engine::calendar_definition::CalendarDefinition;
    use crate::types::CalendarMetadata;
    use crate::types::calendar::{CalendarJurisdiction, CalendarType};

    // Create calendar with multiple parents, one missing
    let existing_parent = CalendarDefinition {
        schema: None,
        id: "existing_parent".to_string(),
        metadata: CalendarMetadata {
            r#type: CalendarType::GeneralRoman,
            jurisdiction: CalendarJurisdiction::Ecclesiastical,
        },
        particular_config: None,
        parent_calendar_ids: vec![],
        days_definitions: std::collections::BTreeMap::new(),
    };

    let child_calendar = CalendarDefinition {
        schema: None,
        id: "child_calendar".to_string(),
        metadata: CalendarMetadata {
            r#type: CalendarType::Country,
            jurisdiction: CalendarJurisdiction::Civil,
        },
        particular_config: None,
        parent_calendar_ids: vec!["existing_parent".to_string(), "missing_parent".to_string()],
        days_definitions: std::collections::BTreeMap::new(),
    };

    let mut romcal = Romcal::empty();
    romcal.calendar = "child_calendar".to_string();
    romcal.calendar_definitions.push(existing_parent);
    romcal.calendar_definitions.push(child_calendar);

    let result = Calendar::new(romcal, 2026);

    match result {
        Err(error) => {
            let error_str = error.to_string();
            assert!(
                error_str.contains("missing_parent"),
                "Error should mention the missing parent: {}",
                error_str
            );
        }
        Ok(_) => panic!("Expected an error when one of multiple parents is missing"),
    }
}

#[test]
fn test_circular_reference_error() {
    use crate::engine::calendar_definition::CalendarDefinition;
    use crate::types::CalendarMetadata;
    use crate::types::calendar::{CalendarJurisdiction, CalendarType};

    // Create circular reference: A → B → A
    let calendar_a = CalendarDefinition {
        schema: None,
        id: "calendar_a".to_string(),
        metadata: CalendarMetadata {
            r#type: CalendarType::GeneralRoman,
            jurisdiction: CalendarJurisdiction::Ecclesiastical,
        },
        particular_config: None,
        parent_calendar_ids: vec!["calendar_b".to_string()],
        days_definitions: std::collections::BTreeMap::new(),
    };

    let calendar_b = CalendarDefinition {
        schema: None,
        id: "calendar_b".to_string(),
        metadata: CalendarMetadata {
            r#type: CalendarType::Region,
            jurisdiction: CalendarJurisdiction::Civil,
        },
        particular_config: None,
        parent_calendar_ids: vec!["calendar_a".to_string()],
        days_definitions: std::collections::BTreeMap::new(),
    };

    let mut romcal = Romcal::empty();
    romcal.calendar = "calendar_a".to_string();
    romcal.calendar_definitions.push(calendar_a);
    romcal.calendar_definitions.push(calendar_b);

    let result = Calendar::new(romcal, 2026);

    match result {
        Err(error) => {
            let error_str = error.to_string();
            assert!(
                error_str.contains("Circular reference"),
                "Error should mention circular reference: {}",
                error_str
            );
        }
        Ok(_) => panic!("Expected an error for circular reference in calendar hierarchy"),
    }
}
