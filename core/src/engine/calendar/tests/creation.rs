use chrono::{Datelike, NaiveDate, Weekday};

use crate::engine::calendar::Calendar;
use crate::engine::proper_of_time::utils::PROPER_OF_TIME_ID;
use crate::romcal::Romcal;
use crate::types::liturgical::Precedence;

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
