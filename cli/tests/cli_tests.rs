use assert_cmd::Command;
use assert_cmd::cargo::cargo_bin_cmd;
use predicates::prelude::*;

fn romcal() -> Command {
    cargo_bin_cmd!("romcal")
}

// ============================================================================
// dates command tests
// ============================================================================

#[test]
fn test_dates_easter_2025() {
    romcal()
        .args(["dates", "easter_sunday", "2025"])
        .assert()
        .success()
        .stdout(predicate::str::contains("2025-04-20"));
}

#[test]
fn test_dates_all_saints_2025() {
    romcal()
        .args(["dates", "all_saints", "2025"])
        .assert()
        .success()
        .stdout(predicate::str::contains("2025-11-01"));
}

#[test]
fn test_dates_json_format() {
    romcal()
        .args(["dates", "easter_sunday", "2025", "-f", "json"])
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
        .args(["dates", "invalid_date", "2025"])
        .assert()
        .failure();
}

#[test]
fn test_invalid_format() {
    romcal()
        .args(["dates", "easter_sunday", "2025", "-f", "invalid"])
        .assert()
        .failure();
}
