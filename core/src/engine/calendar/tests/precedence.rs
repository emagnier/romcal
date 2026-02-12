use crate::engine::calendar::Calendar;
use crate::engine::liturgical_day::{LiturgicalDay, ParentOverride};
use crate::romcal::Romcal;
use crate::types::dates::DateDef;
use crate::types::liturgical::{Precedence, PsalterWeekCycle, Rank, SundayCycle, WeekdayCycle};
use crate::types::martyrology::{Title, TitlesDef};

#[test]
fn test_precedence_comparison() {
    let romcal = Romcal::empty();
    let calendar = Calendar::new(romcal, 2026).unwrap();

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
        Rank::Weekday,
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
        Rank::Weekday,
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
fn test_parent_override_structure() {
    // Test that ParentOverride can be created and checked for changes
    let mut override_empty = ParentOverride::new("test_calendar".to_string());
    assert!(!override_empty.has_changes());

    override_empty.precedence = Some(Precedence::GeneralMemorial_10);
    assert!(override_empty.has_changes());
}

#[test]
fn test_martyr_color_from_titles() {
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
