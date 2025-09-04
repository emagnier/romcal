# Romcal - Liturgical Date Calculation Library in Rust

A Rust library for calculating liturgical dates of the Roman Catholic calendar.

## Features

- **Easter calculations**: Gregorian and Julian algorithms
- **Liturgical seasons**: Advent, Christmas, Lent, Easter Time, Ordinary Time
- **Fixed feasts**: Mary Mother of God, Annunciation, Assumption, All Saints, etc.
- **Movable feasts**: Ascension, Pentecost, Corpus Christi, etc.
- **Feasts of the Lord**: Holy Family, Baptism of the Lord, Epiphany, etc.
- **Flexible configuration**: Support for different liturgical practices

## Installation

Add this dependency to your `Cargo.toml`:

```toml
[dependencies]
romcal = "0.1.0"
```

## Usage

### Basic example

```rust
use romcal::{LiturgicalConfig, LiturgicalDates};

fn main() {
    // Default configuration
    let config = LiturgicalConfig::default();
    let dates = LiturgicalDates::new(config, 2024);

    // Basic calculations
    let easter = dates.easter_sunday(None);
    let christmas = dates.christmas(None);
    let first_advent = dates.first_sunday_of_advent(None);

    println!("Easter 2024: {}", easter.format("%d/%m/%Y"));
    println!("Christmas: {}", christmas.format("%d/%m/%Y"));
    println!("First Sunday of Advent: {}", first_advent.format("%d/%m/%Y"));
}
```

### Custom configuration

```rust
use romcal::{LiturgicalConfig, EasterCalculationType, CalendarScope};

let config = LiturgicalConfig::custom(
    EasterCalculationType::Gregorian,
    CalendarScope::Gregorian,
    false, // Epiphany on January 6
    true,  // Corpus Christi on Sunday
    false, // Ascension on Thursday
);
```

### Predefined configurations

```rust
// Standard Roman rite
let config = LiturgicalConfig::roman_rite();

// United States (Ascension and Corpus Christi on Sunday)
let config = LiturgicalConfig::united_states();

// Countries where Epiphany is celebrated on a Sunday
let config = LiturgicalConfig::epiphany_on_sunday();
```

## Main API

### Easter calculations

```rust
let easter = dates.easter_sunday(Some(2024));
let ash_wednesday = dates.ash_wednesday(Some(2024));
let palm_sunday = dates.palm_sunday(Some(2024));
let good_friday = dates.good_friday(Some(2024));
let holy_saturday = dates.holy_saturday(Some(2024));
```

### Liturgical seasons

```rust
// Advent
let first_advent = dates.first_sunday_of_advent(Some(2024));
let all_dates_advent = dates.all_dates_of_advent(Some(2024));

// Lent
let all_dates_lent = dates.all_dates_of_lent(Some(2024));

// Easter Time
let pentecost = dates.pentecost_sunday(Some(2024));
let ascension = dates.ascension(Some(2024));
```

### Fixed feasts

```rust
let christmas = dates.christmas(Some(2024));
let epiphany = dates.epiphany(Some(2024));
let mary_mother_god = dates.mary_mother_of_god(Some(2024));
let annunciation = dates.annunciation(Some(2024));
let assumption = dates.assumption(Some(2024));
let all_saints = dates.all_saints(Some(2024));
let immaculate_conception = dates.immaculate_conception_of_mary(Some(2024));
```

### Feasts of the Lord

```rust
let holy_family = dates.holy_family(Some(2024));
let baptism_lord = dates.baptism_of_the_lord(Some(2024));
let presentation_lord = dates.presentation_of_the_lord(Some(2024));
let christ_king = dates.christ_the_king_sunday(Some(2024));
```

### Seasons

```rust
let seasons_start = dates.start_of_seasons(Some(2024));
let seasons_end = dates.end_of_seasons(Some(2024));
```

## Examples

See the `examples/basic_usage.rs` file for a complete usage example.

## Tests

```bash
cargo test
```

## Performance

The library is optimized for typical use cases where the same dates are calculated multiple times. The simple implementation prioritizes code clarity over complex caching mechanisms.

## Differences from TypeScript version

This Rust version simplifies the configuration by removing the dependency on `RomcalConfig` and providing a more direct API. The caching system is also optimized for Rust.

## License

This library is licensed under the MIT License.
