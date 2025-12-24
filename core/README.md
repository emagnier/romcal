# Romcal

A Rust library for calculating Catholic liturgical dates and generating liturgical calendars.

For command-line usage, see the [CLI documentation](../cli/README.md).

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
romcal = "4.0"
```

## Quick Start

```rust
use romcal::Romcal;

fn main() -> romcal::RomcalResult<()> {
    // Create a default configuration
    let romcal = Romcal::default();

    // Get a specific liturgical date
    let easter = romcal.get_date("easter_sunday", 2026)?;
    println!("Easter 2026: {}", easter);  // 2026-04-05

    // Generate the liturgical calendar for year 2026
    let calendar = romcal.generate_liturgical_calendar(2026)?;

    // Access a specific date
    if let Some(days) = calendar.get("2025-12-25") {
        for day in days {
            println!("{}: {}", day.date, day.fullname);
        }
    }

    Ok(())
}
```

## Configuration

### Using Preset

`Preset` is a configuration builder with optional fields. Use `Romcal::new(preset)` to create an instance:

```rust
use romcal::{Preset, Romcal, CalendarContext, EasterCalculationType};

let preset = Preset {
    calendar: Some("france".to_string()),
    locale: Some("fr".to_string()),
    context: Some(CalendarContext::Liturgical),
    epiphany_on_sunday: Some(true),
    ascension_on_sunday: Some(true),
    corpus_christi_on_sunday: Some(true),
    ..Preset::default()
};

let romcal = Romcal::new(preset);
```

### Configuration Options

| Option                     | Type                    | Default           | Description                                              |
| -------------------------- | ----------------------- | ----------------- | -------------------------------------------------------- |
| `calendar`                 | `String`                | `"general_roman"` | Calendar ID (e.g., `"france"`, `"united_states"`)        |
| `locale`                   | `String`                | `"en"`            | Locale code (e.g., `"fr"`, `"es"`)                       |
| `context`                  | `CalendarContext`       | `Gregorian`       | `Gregorian` (Jan-Dec) or `Liturgical` (Advent to Advent) |
| `epiphany_on_sunday`       | `bool`                  | `false`           | Celebrate Epiphany on Sunday (Jan 2-8) instead of Jan 6  |
| `ascension_on_sunday`      | `bool`                  | `false`           | Celebrate Ascension on Sunday instead of Thursday        |
| `corpus_christi_on_sunday` | `bool`                  | `true`            | Celebrate Corpus Christi on Sunday instead of Thursday   |
| `easter_calculation_type`  | `EasterCalculationType` | `Gregorian`       | `Gregorian` or `Julian` Easter calculation               |
| `ordinal_format`           | `OrdinalFormat`         | `Numeric`         | `Numeric` ("1st") or `Letters` ("first")                 |

### Available Calendars and Locales

```rust
use romcal::{CALENDAR_IDS, LOCALE_CODES};

// List all available calendar IDs
for id in CALENDAR_IDS {
    println!("{}", id);
}

// List all available locale codes
for code in LOCALE_CODES {
    println!("{}", code);
}
```

### Loading Calendar Data

For calendar generation, you need to load calendar definitions and resources:

```rust
use romcal::{Preset, Romcal};

// Load from JSON files or embedded data
let calendar_definitions = load_calendar_definitions(); // Your loading logic
let resources = load_resources();                       // Your loading logic

let preset = Preset {
    calendar: Some("france".to_string()),
    locale: Some("fr".to_string()),
    calendar_definitions: Some(calendar_definitions),
    resources: Some(resources),
    ..Preset::default()
};

let romcal = Romcal::new(preset);
```

## Getting a Liturgical Date by ID

The `get_date` method calculates a liturgical date by its ID:

```rust
use romcal::Romcal;

let romcal = Romcal::default();

// Easter-related dates
let easter = romcal.get_date("easter_sunday", 2026)?;       // 2026-04-05
let ash_wed = romcal.get_date("ash_wednesday", 2026)?;      // 2026-02-18
let pentecost = romcal.get_date("pentecost_sunday", 2026)?; // 2026-05-24

// Fixed feasts
let christmas = romcal.get_date("christmas", 2026)?;        // 2026-12-25
let all_saints = romcal.get_date("all_saints", 2026)?;      // 2026-11-01

// Any date from the calendar
let monday = romcal.get_date("ordinary_time_5_monday", 2026)?;
```

Any date ID from the liturgical calendar can be used (e.g., `easter_sunday`, `christmas`, `ordinary_time_5_monday`).

## Generating a Liturgical Calendar

Generate a complete liturgical calendar with all celebrations:

```rust
use romcal::Romcal;

let romcal = Romcal::default();

// Year parameter is the liturgical year end (2026 = liturgical year 2025-2026)
let calendar = romcal.generate_liturgical_calendar(2026)?;

// calendar is BTreeMap<String, Vec<LiturgicalDay>>
// Keys are dates in "YYYY-MM-DD" format
for (date, days) in &calendar {
    for day in days {
        println!("{}: {} ({:?})", date, day.fullname, day.rank);
    }
}
```

### LiturgicalDay Structure

Each `LiturgicalDay` contains:

| Field                       | Type               | Description                             |
| --------------------------- | ------------------ | --------------------------------------- |
| `id`                        | `String`           | Unique identifier                       |
| `fullname`                  | `String`           | Localized display name                  |
| `date`                      | `String`           | Date in YYYY-MM-DD format               |
| `precedence`                | `Precedence`       | Liturgical precedence level             |
| `rank`                      | `Rank`             | Rank (Solemnity, Feast, Memorial, etc.) |
| `rank_name`                 | `String`           | Localized rank name                     |
| `season`                    | `Option<Season>`   | Liturgical season                       |
| `season_name`               | `Option<String>`   | Localized season name                   |
| `colors`                    | `Vec<ColorInfo>`   | Liturgical colors                       |
| `entities`                  | `Vec<Entity>`      | Saints, Blessed, or Places              |
| `sunday_cycle`              | `SundayCycle`      | Year A, B, or C                         |
| `weekday_cycle`             | `WeekdayCycle`     | Year 1 or 2                             |
| `psalter_week`              | `PsalterWeekCycle` | Week 1-4                                |
| `is_holy_day_of_obligation` | `bool`             | Holy day of obligation                  |
| `is_optional`               | `bool`             | Optional celebration                    |

## Generating a Mass-Centric Calendar

The mass-centric calendar organizes by civil date and mass time, useful for scheduling:

```rust
use romcal::Romcal;

let romcal = Romcal::default();
let mass_calendar = romcal.generate_mass_calendar(2026)?;

// mass_calendar is BTreeMap<String, Vec<MassContext>>
// Keys are civil dates (not liturgical dates)
for (civil_date, masses) in &mass_calendar {
    for mass in masses {
        println!("{} - {:?}: {} (liturgical: {})",
            mass.civil_date,
            mass.mass_time,
            mass.fullname,
            mass.liturgical_date
        );
    }
}
```

### Key Difference from Liturgical Calendar

Evening masses appear on the **previous civil day**:

- Easter Vigil (April 19) has `civil_date: "2025-04-19"` but `liturgical_date: "2025-04-20"`
- Christmas Vigil Mass has `civil_date: "2025-12-24"` but `liturgical_date: "2025-12-25"`

### MassContext Structure

Each `MassContext` is a flat structure containing:

| Field                   | Type                      | Description                                   |
| ----------------------- | ------------------------- | --------------------------------------------- |
| `mass_time`             | `MassTime`                | Type of mass (DayMass, EasterVigil, etc.)     |
| `mass_time_name`        | `String`                  | Localized mass time name                      |
| `civil_date`            | `String`                  | Calendar date (YYYY-MM-DD)                    |
| `liturgical_date`       | `String`                  | Theological celebration date                  |
| `id`                    | `String`                  | Unique identifier                             |
| `fullname`              | `String`                  | Localized display name                        |
| `precedence`            | `Precedence`              | Liturgical precedence level                   |
| `rank`                  | `Rank`                    | Liturgical rank                               |
| `rank_name`             | `String`                  | Localized rank name                           |
| `season`                | `Option<Season>`          | Liturgical season                             |
| `season_name`           | `Option<String>`          | Localized season name                         |
| `colors`                | `Vec<ColorInfo>`          | Liturgical colors                             |
| `entities`              | `Vec<Entity>`             | Saints, Blessed, or Places                    |
| `sunday_cycle`          | `SundayCycle`             | Year A, B, or C                               |
| `weekday_cycle`         | `WeekdayCycle`            | Year 1 or 2                                   |
| `psalter_week`          | `PsalterWeekCycle`        | Week 1-4                                      |
| `periods`               | `Vec<PeriodInfo>`         | Liturgical periods                            |
| `week_of_season`        | `Option<u32>`             | Week number within the season                 |
| `day_of_season`         | `Option<u32>`             | Day number within the season                  |
| `optional_celebrations` | `Vec<CelebrationSummary>` | Alternative celebrations (optional memorials) |

## Creating an Optimized Bundle

Generate a minimal JSON bundle for deployment (useful for web/mobile apps):

```rust
use romcal::{Preset, Romcal};

let preset = Preset {
    calendar: Some("france".to_string()),
    locale: Some("fr".to_string()),
    calendar_definitions: Some(all_definitions),
    resources: Some(all_resources),
    ..Preset::default()
};

let romcal = Romcal::new(preset);
let json_bundle = romcal.optimize()?;

// json_bundle contains only:
// - Target calendar and its parent calendars
// - Target locale and parent locales
// - Entities actually used in the calendar
```

## Key Types

### Seasons

The liturgical year is divided into five seasons:

| Season          | Period                                          |
| --------------- | ----------------------------------------------- |
| `Advent`        | Four weeks before Christmas                     |
| `ChristmasTime` | Christmas to Baptism of the Lord                |
| `Lent`          | Ash Wednesday to Holy Thursday                  |
| `EasterTime`    | Easter Sunday to Pentecost (50 days)            |
| `OrdinaryTime`  | Two periods: after Epiphany and after Pentecost |

### Ranks

Celebrations are classified by rank, from highest to lowest (GNLY #11-16):

| Rank               | Description                                                                          |
| ------------------ | ------------------------------------------------------------------------------------ |
| `Solemnity`        | Most important days; begins at First Vespers on the preceding day                    |
| `Sunday`           | The Lord's Day; the primordial feast day celebrating the Paschal Mystery             |
| `Feast`            | Celebrated within the natural day; no First Vespers (except Lord's feasts on Sunday) |
| `Memorial`         | Obligatory commemoration; becomes optional during Lent and Advent privileged days    |
| `OptionalMemorial` | Non-obligatory commemoration; only one may be chosen if multiple fall on same day    |
| `Weekday`          | Ordinary weekdays; some (Ash Wednesday, Holy Week, Dec 17-24) take precedence        |

### Precedence

Precedence levels are essential for determining which celebration takes priority when multiple celebrations fall on the same day. Romcal implements the 13 levels defined in the _General Norms for the Liturgical Year and the Calendar_ (GNLY #49):

| Level | Description                                                                                                      |
| ----- | ---------------------------------------------------------------------------------------------------------------- |
| 1     | Paschal Triduum                                                                                                  |
| 2     | Nativity, Epiphany, Ascension, Pentecost; Sundays of Advent/Lent/Easter; Ash Wednesday; Holy Week; Easter Octave |
| 3     | Solemnities in the General Calendar; All Souls                                                                   |
| 4     | Proper Solemnities (patron, dedication, title, founder)                                                          |
| 5     | Feasts of the Lord in the General Calendar                                                                       |
| 6     | Sundays of Christmas Time and Ordinary Time                                                                      |
| 7     | Feasts of Mary and Saints in the General Calendar                                                                |
| 8     | Proper Feasts (diocese, region, religious order)                                                                 |
| 9     | Privileged weekdays (Advent Dec 17-24, Lent)                                                                     |
| 10    | Obligatory Memorials in the General Calendar                                                                     |
| 11    | Proper Obligatory Memorials                                                                                      |
| 12    | Optional Memorials                                                                                               |
| 13    | Weekdays                                                                                                         |

### Liturgical Colors

Colors are automatically computed based on the season and celebration. For memorials of martyrs, red is automatically applied.

| Color    | Usage                                                             |
| -------- | ----------------------------------------------------------------- |
| `White`  | Christmas, Easter, feasts of the Lord, Mary, Saints (non-martyrs) |
| `Red`    | Martyrs, Pentecost, Palm Sunday, Good Friday                      |
| `Purple` | Advent, Lent                                                      |
| `Green`  | Ordinary Time                                                     |
| `Rose`   | Gaudete Sunday (3rd Advent), Laetare Sunday (4th Lent)            |
| `Gold`   | Solemn celebrations (alternative to white)                        |
| `Black`  | Funerals, All Souls (optional)                                    |

### Liturgical Periods

Periods are sub-divisions within liturgical seasons, traditionally used in monastic and religious communities. They help determine specific elements such as the antiphon to the Blessed Virgin Mary (Alma Redemptoris Mater, Ave Regina Caelorum, Regina Caeli, Salve Regina).

| Period                                | Description                                    |
| ------------------------------------- | ---------------------------------------------- |
| `ChristmasOctave`                     | December 25 to January 1                       |
| `DaysBeforeEpiphany`                  | January 2 to the day before Epiphany           |
| `DaysFromEpiphany`                    | Epiphany to the day before the Presentation    |
| `ChristmasToPresentationOfTheLord`    | Christmas to Presentation (Feb 2)              |
| `PresentationOfTheLordToHolyThursday` | Presentation to Holy Thursday                  |
| `HolyWeek`                            | Palm Sunday to Holy Saturday                   |
| `PaschalTriduum`                      | Holy Thursday evening to Easter Sunday Vespers |
| `EasterOctave`                        | Easter Sunday to the following Sunday          |
| `EarlyOrdinaryTime`                   | After Presentation to Ash Wednesday            |
| `LateOrdinaryTime`                    | After Pentecost to first Sunday of Advent      |

### Cycles

Cycles determine which readings and psalms are used in the liturgy.

**Sunday Cycle** (`SundayCycle`): A three-year cycle for Sunday and solemnity readings.

| Cycle   | Years (examples)    | Gospel focus |
| ------- | ------------------- | ------------ |
| `YearA` | 2023, 2026, 2029... | Matthew      |
| `YearB` | 2024, 2027, 2030... | Mark         |
| `YearC` | 2025, 2028, 2031... | Luke         |

**Weekday Cycle** (`WeekdayCycle`): A two-year cycle for weekday readings (first reading only; Gospel follows its own sequence).

| Cycle    | Years (examples)                 |
| -------- | -------------------------------- |
| `Year_1` | Odd years (2025, 2027, 2029...)  |
| `Year_2` | Even years (2024, 2026, 2028...) |

**Psalter Week** (`PsalterWeekCycle`): A four-week cycle for the Liturgy of the Hours (Divine Office).

| Cycle    | Usage                              |
| -------- | ---------------------------------- |
| `Week_1` | Week 1 of the psalter              |
| `Week_2` | Week 2 of the psalter              |
| `Week_3` | Week 3 of the psalter              |
| `Week_4` | Week 4 of the psalter, then repeat |

### Mass Times

`MassTime` variants:

| Variant                       | Description                                               |
| ----------------------------- | --------------------------------------------------------- |
| `EasterVigil`                 | Easter Vigil on Holy Saturday night                       |
| `PreviousEveningMass`         | Mass the evening before a major feast                     |
| `NightMass`                   | Night Mass of the Nativity of the Lord (Christmas)        |
| `MassAtDawn`                  | Mass at Dawn of the Nativity of the Lord (Christmas)      |
| `MorningMass`                 | Morning Mass on December 24                               |
| `MassOfThePassion`            | Palm Sunday Mass with procession                          |
| `CelebrationOfThePassion`     | Good Friday celebration                                   |
| `DayMass`                     | Regular daytime Mass                                      |
| `ChrismMass`                  | Chrism Mass (typically Tuesday or Wednesday of Holy Week) |
| `EveningMassOfTheLordsSupper` | Holy Thursday evening                                     |

## Error Handling

All fallible operations return `RomcalResult<T>`, which is an alias for `Result<T, RomcalError>`.

```rust
use romcal::{Romcal, RomcalResult, RomcalError};

fn generate_calendar() -> RomcalResult<()> {
    let romcal = Romcal::default();

    // This will fail: year must be >= 1583 (Gregorian calendar adoption)
    match romcal.generate_liturgical_calendar(1500) {
        Ok(calendar) => { /* use calendar */ }
        Err(RomcalError::InvalidYear(year)) => {
            eprintln!("Invalid year: {}", year);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }

    Ok(())
}
```

### Error Types

| Error                     | Description                           |
| ------------------------- | ------------------------------------- |
| `InvalidYear(i32)`        | Year is before 1583 or after 9999     |
| `InvalidDate`             | Invalid date encountered              |
| `CalculationError`        | Error during liturgical calculations  |
| `InvalidConfig`           | Invalid configuration provided        |
| `DateConversionError`     | Error converting between date formats |
| `ValidationError(String)` | Validation failed with message        |
| `InvalidDateName(String)` | Unknown date ID passed to `get_date`  |

## Development

```bash
# Run tests
cargo test -p romcal

# Run quality checks
./scripts/check-core.sh

# Build release
cargo build -p romcal --release
```

## Related

- [romcal](https://github.com/romcal/romcal) - Main Romcal project
- [romcal-cli](../cli/) - Command-line interface
- [romcal (TypeScript)](../bindings/typescript/) - TypeScript/JavaScript binding
- [romcal (Python)](../bindings/python/) - Python binding

## License

Apache License 2.0. See [LICENSE](../LICENSE) for details.
