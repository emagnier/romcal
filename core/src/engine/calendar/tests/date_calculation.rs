use chrono::{Datelike, NaiveDate, Weekday};

use crate::engine::calendar::Calendar;
use crate::romcal::Romcal;
use crate::types::dates::DateDef;

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
