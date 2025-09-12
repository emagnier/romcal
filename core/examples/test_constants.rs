use romcal_core::{CALENDAR_IDS, LOCALE_CODES};

fn main() {
    println!("=== Romcal Constants Test ===");
    println!();

    println!("📅 Calendar IDs ({} total):", CALENDAR_IDS.len());
    for (i, calendar) in CALENDAR_IDS.iter().enumerate() {
        println!("  {}. {}", i + 1, calendar);
    }

    println!();
    println!("🌍 Locale Codes ({} total):", LOCALE_CODES.len());
    for (i, locale) in LOCALE_CODES.iter().enumerate() {
        println!("  {}. {}", i + 1, locale);
    }

    println!();
    println!("✅ Constants loaded successfully!");
}
