# Proper of Time

This module implements the generation of liturgical days for the Proper of Time (Temps Propre) for the Catholic liturgical calendar.

## Features

- Generation of Advent liturgical days
- Support for liturgical and Gregorian years
- Automatic date calculation based on Easter
- Appropriate liturgical color management
- Respect for liturgical precedences

## Usage

### Basic example

```rust
use romcal_core::Preset;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a default configuration
    let config = Preset::default();

    // Generate liturgical days of Advent for liturgical year 2026
    let advent_days = config.proper_of_time(2026)?;

    println!("Number of days generated: {}", advent_days.len());

    // Display Advent Sundays
    for day in advent_days.iter().filter(|d| d.id.contains("sunday")) {
        println!("- {}: {} ({})", day.id, day.fullname, day.date);
    }

    Ok(())
}
```

### With liturgical year

```rust
use romcal_core::{Preset, PresetPartial, CalendarContext};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configuration for liturgical year
    let config = Preset::new(PresetPartial {
        context: Some(CalendarContext::Liturgical),
        ..PresetPartial::default()
    });

    // For liturgical year 2026, Advent begins in 2025
    let advent_days = config.proper_of_time(2026)?;

    // Check that dates are in 2025
    if let Some(first_day) = advent_days.first() {
        let year = first_day.date.split('-').next().unwrap().parse::<i32>().unwrap();
        assert_eq!(year, 2025);
    }

    Ok(())
}
```

## Generated day structure

### Advent Sundays

- **ID**: `advent_1_sunday`, `advent_2_sunday`, etc.
- **Precedence**: `PrivilegedSunday_2`
- **Rank**: `Sunday`
- **Colors**: Purple (Rose for the 3rd Sunday - Gaudete)
- **Obligation**: Yes (holy days of obligation)

### Advent weekdays

- **ID**: `advent_{week}_{weekday}` (e.g., `advent_1_monday`)
- **Precedence**: `Weekday_13`
- **Rank**: `Weekday`
- **Colors**: Purple
- **Obligation**: No

### Privileged days (December 17-24)

- **ID**: `advent_december_{day}` (e.g., `advent_december_17`)
- **Precedence**: `PrivilegedWeekday_9`
- **Rank**: `Weekday`
- **Colors**: Purple
- **Obligation**: No

## Date calculation

Dates are calculated using methods from `LiturgicalDates`:

- First Sunday of Advent: calculated from Christmas
- Weekdays: calculated relative to the first Sunday
- Privileged days: fixed dates from December 17 to 24

## Seasons and periods

All Advent days belong to:

- **Season**: `Advent`
- **Period**: No specific period for Advent

## Tests

Run tests:

```bash
cargo test --package romcal-core proper_of_time
```

Run example:

```bash
cargo run --example simple_proper_of_time --package romcal-core
```

## Current limitations

- Only the Advent season is implemented
- Other seasons (Christmas, Lent, Easter, Ordinary Time) will be added in future versions
- Translations are in English (to be internationalized)
