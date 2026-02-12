//! Tests for liturgical date calculations.

use chrono::{Datelike, Weekday};

use super::LiturgicalDates;
use crate::romcal::Preset;

#[test]
fn test_liturgical_dates_creation() {
    let config = crate::romcal::Romcal::default();
    let dates = LiturgicalDates::new(config, 2024).unwrap();

    assert_eq!(dates.year, 2024);
    assert!(!dates.is_liturgical_year);
}

#[test]
fn test_liturgical_year_creation() {
    let config = crate::romcal::Romcal::new(Preset {
        context: Some(crate::CalendarContext::Liturgical),
        ..Preset::default()
    })
    .unwrap();
    let dates = LiturgicalDates::new(config, 2024).unwrap();

    assert_eq!(dates.year, 2024);
    assert!(dates.is_liturgical_year);
}

#[test]
fn test_christmas_calculation() {
    let config = crate::romcal::Romcal::default();
    let dates = LiturgicalDates::new(config, 2024).unwrap();
    let christmas = dates.get_christmas_date(None);

    assert_eq!(christmas.day(), 25);
    assert_eq!(christmas.month(), 12);
    assert_eq!(christmas.year(), 2024);
}

#[test]
fn test_easter_calculation() {
    let config = crate::romcal::Romcal::default();
    let dates = LiturgicalDates::new(config, 2024).unwrap();
    let easter = dates.get_easter_sunday_date_unwrap(None);

    // Easter 2024 is March 31
    assert_eq!(easter.day(), 31);
    assert_eq!(easter.month(), 3);
    assert_eq!(easter.year(), 2024);
}

#[test]
fn test_ash_wednesday_calculation() {
    let config = crate::romcal::Romcal::default();
    let dates = LiturgicalDates::new(config, 2024).unwrap();
    let ash_wednesday = dates.get_ash_wednesday_date(None);

    // Ash Wednesday 2024 is February 14 (46 days before Easter)
    assert_eq!(ash_wednesday.day(), 14);
    assert_eq!(ash_wednesday.month(), 2);
    assert_eq!(ash_wednesday.year(), 2024);
}

#[test]
fn test_utility_functions() {
    let date1 = LiturgicalDates::get_utc_date(2024, 3, 31);
    let date2 = LiturgicalDates::get_utc_date(2024, 3, 31);
    let date3 = LiturgicalDates::get_utc_date(2024, 4, 1);

    assert!(LiturgicalDates::is_same_date(date1, date2));
    assert!(!LiturgicalDates::is_same_date(date1, date3));

    let added_date = LiturgicalDates::add_days(date1, 1);
    assert!(LiturgicalDates::is_same_date(added_date, date3));
}

#[test]
fn test_unprivileged_weekday_of_advent() {
    let config = crate::romcal::Romcal::default();
    let dates = LiturgicalDates::new(config, 2024).unwrap();

    // Test valid weekday
    let weekday = dates.unprivileged_weekday_of_advent(1, 1, None); // Monday, week 1
    assert!(weekday.is_some());

    // Test invalid parameters
    assert!(dates.unprivileged_weekday_of_advent(0, 1, None).is_none()); // Invalid dow
    assert!(dates.unprivileged_weekday_of_advent(1, 0, None).is_none()); // Invalid week
    assert!(dates.unprivileged_weekday_of_advent(7, 1, None).is_none()); // Invalid dow
    assert!(dates.unprivileged_weekday_of_advent(1, 5, None).is_none()); // Invalid week
}

#[test]
fn test_privileged_weekday_of_advent() {
    let config = crate::romcal::Romcal::default();
    let dates = LiturgicalDates::new(config, 2024).unwrap();

    // Test valid day
    let weekday = dates.privileged_weekday_of_advent(17, None);
    assert!(weekday.is_some());

    // Test invalid parameters
    assert!(dates.privileged_weekday_of_advent(16, None).is_none()); // Too early
    assert!(dates.privileged_weekday_of_advent(25, None).is_none()); // Too late
}

#[test]
fn test_sunday_of_advent() {
    let config = crate::romcal::Romcal::default();
    let dates = LiturgicalDates::new(config, 2024).unwrap();

    // Test valid week
    let sunday = dates.get_sunday_of_advent_date(1, None);
    assert!(sunday.is_some());
    assert_eq!(sunday.unwrap().weekday(), Weekday::Sun);

    // Test invalid parameters
    assert!(dates.get_sunday_of_advent_date(0, None).is_none()); // Invalid week
    assert!(dates.get_sunday_of_advent_date(5, None).is_none()); // Invalid week
}

#[test]
fn test_all_dates_in_octave_of_christmas() {
    let config = crate::romcal::Romcal::default();
    let dates = LiturgicalDates::new(config, 2024).unwrap();
    let octave_dates = dates.all_dates_in_octave_of_christmas(None);

    // Should have 8 dates: Christmas + 6 days + Mary Mother of God
    assert_eq!(octave_dates.len(), 8);

    // First date should be Christmas
    let christmas = dates.get_christmas_date(None);
    assert_eq!(octave_dates[0], christmas);

    // Last date should be Mary Mother of God
    let mary_mother_of_god = dates.get_mary_mother_of_god_date(None);
    assert_eq!(octave_dates[7], mary_mother_of_god);
}

#[test]
fn test_weekday_within_octave_of_christmas() {
    let config = crate::romcal::Romcal::default();
    let dates = LiturgicalDates::new(config, 2024).unwrap();

    // Test valid day of octave
    let weekday = dates.get_weekday_within_octave_of_christmas_date(1, None);
    assert!(weekday.is_some());

    // Test invalid parameters
    assert!(
        dates
            .get_weekday_within_octave_of_christmas_date(0, None)
            .is_none()
    ); // Invalid day
    assert!(
        dates
            .get_weekday_within_octave_of_christmas_date(9, None)
            .is_none()
    ); // Invalid day
}

#[test]
fn test_all_dates_of_christmas_time() {
    let config = crate::romcal::Romcal::default();
    let dates = LiturgicalDates::new(config, 2024).unwrap();
    let christmas_time_dates = dates.get_all_dates_of_christmas_time(None);

    // Should have dates from Christmas to Baptism of the Lord
    assert!(!christmas_time_dates.is_empty());

    // First date should be Christmas
    let christmas = dates.get_christmas_date(None);
    assert_eq!(christmas_time_dates[0], christmas);
}

#[test]
fn test_epiphany() {
    let config = crate::romcal::Romcal::default();
    let dates = LiturgicalDates::new(config, 2024).unwrap();
    let epiphany = dates.get_epiphany_date(None);

    // Epiphany should be in January
    assert_eq!(epiphany.month(), 1);
    assert!(epiphany.day() >= 2 && epiphany.day() <= 8);
}

#[test]
fn test_all_dates_before_epiphany() {
    let config = crate::romcal::Romcal::default();
    let dates = LiturgicalDates::new(config, 2024).unwrap();
    let dates_before = dates.all_dates_before_epiphany(None);

    // Should start from January 2
    if !dates_before.is_empty() {
        assert!(dates_before[0].day() >= 2);
    }
}

#[test]
fn test_weekday_before_epiphany() {
    let config = crate::romcal::Romcal::default();
    let dates = LiturgicalDates::new(config, 2024).unwrap();

    // Test valid day
    let weekday = dates.get_weekday_before_epiphany_date(2, None);
    // May or may not exist depending on the year
    if weekday.is_some() {
        assert_eq!(weekday.unwrap().day(), 2);
    }

    // Test invalid parameters
    assert!(dates.get_weekday_before_epiphany_date(1, None).is_none()); // Too early
    assert!(dates.get_weekday_before_epiphany_date(9, None).is_none()); // Too late
}

#[test]
fn test_weekday_before_epiphany_ignores_sundays() {
    let config = crate::romcal::Romcal::default();
    let dates = LiturgicalDates::new(config, 2026).unwrap();

    // Get all weekdays before epiphany for 2026
    let all_dates = dates.all_dates_before_epiphany(Some(2026));

    // Find all Sundays in the range
    let sundays: Vec<_> = all_dates
        .iter()
        .filter(|d| d.weekday() == Weekday::Sun)
        .collect();

    // For each Sunday, verify that get_weekday_before_epiphany_date returns None
    for sunday in sundays {
        let day = sunday.day() as u8;
        let result = dates.get_weekday_before_epiphany_date(day, Some(2026));
        assert!(
            result.is_none(),
            "get_weekday_before_epiphany_date should ignore Sunday {} (day {})",
            sunday.format("%Y-%m-%d"),
            day
        );
    }
}

#[test]
fn test_weekday_after_epiphany() {
    let config = crate::romcal::Romcal::default();
    let dates = LiturgicalDates::new(config, 2024).unwrap();

    // Test valid day of week
    let weekday = dates.get_weekday_after_epiphany_date(1, None); // Monday
    // May or may not exist depending on the year
    if weekday.is_some() {
        assert_eq!(weekday.unwrap().weekday().num_days_from_sunday() as u8, 1);
    }

    // Test invalid parameters
    assert!(dates.get_weekday_after_epiphany_date(0, None).is_none()); // Invalid dow
    assert!(dates.get_weekday_after_epiphany_date(7, None).is_none()); // Invalid dow
}

#[test]
fn test_invalid_year_creation() {
    let config = crate::romcal::Romcal::default();

    // Test invalid year
    assert!(LiturgicalDates::new(config.clone(), 1500).is_err());
    assert!(LiturgicalDates::new(config.clone(), 1582).is_err());

    // Test valid year
    assert!(LiturgicalDates::new(config.clone(), 1583).is_ok());
    assert!(LiturgicalDates::new(config, 2024).is_ok());
}

#[test]
fn test_easter_error_handling() {
    let config = crate::romcal::Romcal::default();
    let dates = LiturgicalDates::new(config, 2024).unwrap();

    // Test valid year
    assert!(dates.get_easter_sunday_date(Some(2024)).is_ok());

    // Test invalid year
    assert!(dates.get_easter_sunday_date(Some(1500)).is_err());
}
