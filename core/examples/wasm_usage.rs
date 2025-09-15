//! # WASM-friendly usage example for Romcal
//!
//! This example demonstrates how to use the Romcal library with robust error handling,
//! optimized for use in WebAssembly.

use romcal_core::{LiturgicalDates, Preset, RomcalResult};

fn main() {
    println!("=== WASM-friendly usage example for Romcal ===\n");

    // Example 1: Creation with error handling
    match create_liturgical_dates(2024) {
        Ok(dates) => {
            println!("✅ Creation successful for year 2024");
            demonstrate_easter_calculation(&dates);
        }
        Err(e) => {
            println!("❌ Error during creation: {}", e);
        }
    }

    // Example 2: Test with invalid year
    match create_liturgical_dates(1500) {
        Ok(_) => {
            println!("✅ Creation successful (unexpected)");
        }
        Err(e) => {
            println!("❌ Expected error for year 1500: {}", e);
        }
    }

    // Example 3: Calculations with error handling
    if let Ok(dates) = create_liturgical_dates(2024) {
        demonstrate_error_handling(&dates);
    }

    // Example 4: Using the compatibility API
    demonstrate_compatibility_api();
}

/// Creates a LiturgicalDates instance with error handling
fn create_liturgical_dates(year: i32) -> RomcalResult<LiturgicalDates> {
    let config = Preset::default();
    LiturgicalDates::new(config, year)
}

/// Demonstrates Easter calculation with error handling
fn demonstrate_easter_calculation(dates: &LiturgicalDates) {
    println!("\n--- Easter calculation ---");

    // Using the new API with Result
    match dates.get_easter_sunday_date(Some(2024)) {
        Ok(easter) => {
            println!("✅ Easter 2024: {}", easter.format("%d/%m/%Y"));
        }
        Err(e) => {
            println!("❌ Error during Easter calculation: {}", e);
        }
    }

    // Test with invalid year
    match dates.get_easter_sunday_date(Some(1500)) {
        Ok(_) => {
            println!("✅ Easter 1500 calculated (unexpected)");
        }
        Err(e) => {
            println!("❌ Expected error for Easter 1500: {}", e);
        }
    }
}

/// Demonstrates error handling for different calculations
fn demonstrate_error_handling(dates: &LiturgicalDates) {
    println!("\n--- Error handling for different calculations ---");

    let test_years = vec![2024, 1500, 1583, 2025];

    for year in test_years {
        match dates.get_easter_sunday_date(Some(year)) {
            Ok(easter) => {
                println!("✅ Easter {}: {}", year, easter.format("%d/%m/%Y"));
            }
            Err(e) => {
                println!("❌ Error for year {}: {}", year, e);
            }
        }
    }
}

/// Demonstrates the use of the compatibility API
fn demonstrate_compatibility_api() {
    println!("\n--- Compatibility API ---");

    // The compatibility API uses unwrap() and can panic
    // It's useful for existing code that cannot handle errors
    let config = Preset::default();
    let dates = LiturgicalDates::new(config, 2024).unwrap();

    // Using the compatibility API (can panic)
    let easter = dates.get_easter_sunday_date_unwrap(None);
    println!(
        "✅ Easter 2024 (compatibility API): {}",
        easter.format("%d/%m/%Y")
    );

    // Feast calculations with the compatibility API
    let ash_wednesday = dates.get_ash_wednesday_date(None);
    let christmas = dates.get_christmas_date(None);

    println!("✅ Ash Wednesday: {}", ash_wednesday.format("%d/%m/%Y"));
    println!("✅ Christmas: {}", christmas.format("%d/%m/%Y"));
}

/// Utility function for WASM interface
/// This function could be exposed via wasm-bindgen
pub fn calculate_easter_wasm(year: i32) -> Result<String, String> {
    let config = Preset::default();
    let dates = LiturgicalDates::new(config, year).map_err(|e| e.to_string())?;

    let easter = dates
        .get_easter_sunday_date(Some(year))
        .map_err(|e| e.to_string())?;

    Ok(easter.format("%Y-%m-%d").to_string())
}

/// Utility function to validate a year
/// This function could be exposed via wasm-bindgen
pub fn validate_year_wasm(year: i32) -> Result<bool, String> {
    let config = Preset::default();
    match LiturgicalDates::new(config, year) {
        Ok(_) => Ok(true),
        Err(e) => Err(e.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_easter_wasm() {
        // Test with valid year
        let result = calculate_easter_wasm(2024);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "2024-03-31");

        // Test with invalid year
        let result = calculate_easter_wasm(1500);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_year_wasm() {
        // Test with valid year
        assert!(validate_year_wasm(2024).unwrap());
        assert!(validate_year_wasm(1583).unwrap());

        // Test with invalid year
        assert!(validate_year_wasm(1500).is_err());
        assert!(validate_year_wasm(1582).is_err());
    }
}
