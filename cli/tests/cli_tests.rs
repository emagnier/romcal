use assert_cmd::Command;
use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

fn romcal() -> Command {
    cargo_bin_cmd!("romcal")
}

fn fixture_path(name: &str) -> String {
    format!("{}/tests/fixtures/{}", env!("CARGO_MANIFEST_DIR"), name)
}

// ============================================================================
// date command tests
// ============================================================================

#[test]
fn test_date_easter_2025() {
    romcal()
        .args(["date", "easter_sunday", "2025"])
        .assert()
        .success()
        .stdout(predicate::str::contains("2025-04-20"));
}

#[test]
fn test_date_all_saints_2025() {
    romcal()
        .args(["date", "all_saints", "2025"])
        .assert()
        .success()
        .stdout(predicate::str::contains("2025-11-01"));
}

#[test]
fn test_date_json_format() {
    romcal()
        .args(["date", "easter_sunday", "2025", "-f", "json"])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"2025-04-20\""));
}

// ============================================================================
// calendar command tests
// ============================================================================

#[test]
fn test_calendar_generates_output() {
    romcal()
        .args(["calendar", "2025"])
        .assert()
        .success()
        .stdout(predicate::str::contains("id:"));
}

#[test]
fn test_calendar_with_filter() {
    romcal()
        .args(["calendar", "2025", "--filter", "id,date"])
        .assert()
        .success()
        .stdout(predicate::str::contains("id:"))
        .stdout(predicate::str::contains("date:"))
        .stdout(predicate::str::contains("precedence:").not());
}

// ============================================================================
// list commands tests
// ============================================================================

#[test]
fn test_list_calendars() {
    romcal()
        .args(["list", "calendars"])
        .assert()
        .success()
        .stdout(predicate::str::contains("general_roman"));
}

#[test]
fn test_list_locales() {
    romcal()
        .args(["list", "locales"])
        .assert()
        .success()
        .stdout(predicate::str::contains("fr"));
}

#[test]
fn test_list_calendars_tree() {
    romcal()
        .args(["list", "calendars", "--tree"])
        .assert()
        .success()
        .stdout(predicate::str::contains("general_roman"));
}

// ============================================================================
// preset command tests
// ============================================================================

#[test]
fn test_preset_shows_config() {
    romcal()
        .args(["preset"])
        .assert()
        .success()
        .stdout(predicate::str::contains("calendar"));
}

// ============================================================================
// completions command tests
// ============================================================================

#[test]
fn test_completions_bash() {
    romcal()
        .args(["completions", "bash"])
        .assert()
        .success()
        .stdout(predicate::str::contains("complete"));
}

#[test]
fn test_completions_zsh() {
    romcal()
        .args(["completions", "zsh"])
        .assert()
        .success()
        .stdout(predicate::str::contains("#compdef"));
}

// ============================================================================
// help and version tests
// ============================================================================

#[test]
fn test_help() {
    romcal()
        .args(["--help"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Usage:"));
}

#[test]
fn test_version() {
    romcal()
        .args(["--version"])
        .assert()
        .success()
        .stdout(predicate::str::contains("romcal"));
}

// ============================================================================
// error handling tests
// ============================================================================

#[test]
fn test_invalid_command() {
    romcal().args(["invalid_command"]).assert().failure();
}

#[test]
fn test_invalid_date_name() {
    romcal()
        .args(["date", "invalid_date", "2025"])
        .assert()
        .failure();
}

#[test]
fn test_invalid_format() {
    romcal()
        .args(["date", "easter_sunday", "2025", "-f", "invalid"])
        .assert()
        .failure();
}

// ============================================================================
// bundled data tests
// ============================================================================

#[test]
fn test_bundled_data_default() {
    // CLI should work out-of-the-box with built-in data
    romcal()
        .args(["calendar", "2026"])
        .assert()
        .success()
        .stdout(predicate::str::contains("mary_mother_of_god"));
}

#[test]
fn test_bundled_data_with_calendar_and_locale() {
    // Specific calendar and locale should work with built-in data
    romcal()
        .args(["calendar", "2026", "-c", "france", "-l", "fr"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Sainte Marie"));
}

#[test]
fn test_replace_flag_without_custom_data_uses_builtin() {
    // --replace without -d/-r should be ignored and use built-in data
    romcal()
        .args(["calendar", "2026", "--replace"])
        .assert()
        .success()
        .stdout(predicate::str::contains("mary_mother_of_god"));
}

// ============================================================================
// custom data merge tests
// ============================================================================

#[test]
fn test_merge_custom_definitions() {
    // Custom definitions should be merged with built-in data
    romcal()
        .args([
            "calendar",
            "2026",
            "-c",
            "test_custom_calendar",
            "-d",
            &fixture_path("custom_calendar.json"),
            "-r",
            &fixture_path("custom_resource.json"),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("test_custom_saint"));
}

#[test]
fn test_merge_custom_resources() {
    // Custom resources should be merged with built-in data
    // The custom saint name should appear in output
    romcal()
        .args([
            "calendar",
            "2026",
            "-c",
            "test_custom_calendar",
            "-d",
            &fixture_path("custom_calendar.json"),
            "-r",
            &fixture_path("custom_resource.json"),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("Test Custom Saint"));
}

#[test]
fn test_merge_preserves_builtin_data() {
    // When merging, built-in data should still be available
    romcal()
        .args([
            "calendar",
            "2026",
            "-d",
            &fixture_path("custom_calendar.json"),
        ])
        .assert()
        .success()
        // Built-in general_roman entries should still work
        .stdout(predicate::str::contains("mary_mother_of_god"));
}

// ============================================================================
// custom data replace tests
// ============================================================================

#[test]
fn test_replace_with_custom_definitions() {
    // With --replace, only custom data should be used
    // Must include general_roman since test_custom_calendar depends on it
    let definitions = format!(
        "{},{}",
        fixture_path("general_roman_minimal.json"),
        fixture_path("custom_calendar.json")
    );
    romcal()
        .args([
            "calendar",
            "2026",
            "-c",
            "test_custom_calendar",
            "-d",
            &definitions,
            "-r",
            &fixture_path("custom_resource.json"),
            "--replace",
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("test_custom_saint"));
}

#[test]
fn test_replace_rejects_missing_calendar() {
    // With --replace, if a calendar doesn't exist in custom data, it should fail
    romcal()
        .args([
            "calendar",
            "2026",
            "-c",
            "france",
            "-d",
            &fixture_path("custom_calendar.json"),
            "-r",
            &fixture_path("custom_resource.json"),
            "--replace",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Calendar 'france' not found"));
}

#[test]
fn test_replace_rejects_missing_parent_calendar() {
    // With --replace, if a parent calendar doesn't exist in custom data, it should fail
    romcal()
        .args([
            "calendar",
            "2026",
            "-c",
            "test_custom_calendar",
            "-d",
            &fixture_path("custom_calendar.json"),
            "-r",
            &fixture_path("custom_resource.json"),
            "--replace",
        ])
        .assert()
        .failure()
        .stderr(predicate::str::contains("Parent calendar 'general_roman'"))
        .stderr(predicate::str::contains("not found"));
}
