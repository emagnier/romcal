# Romcal CLI

A CLI for calculating Catholic liturgical dates and generating calendars.

## Quick Start

```bash
# Get Easter date for 2025
romcal date easter_sunday 2025

# Generate liturgical calendar for current year
romcal calendar

# Generate calendar with specific locale and calendar
romcal calendar 2025 --calendar france --locale fr

# Generate mass-centric calendar
romcal masses 2025 --filter civil_date,id,fullname

# List available calendars
romcal list calendars
```

## Installation

### Requirements

- [Rust](https://rustup.rs/) 1.85 or later

### From Source

```bash
# Clone the repository
git clone https://github.com/romcal/romcal.git
cd romcal

# Build and install
cargo install --path cli

# Or run directly
cargo run -p romcal-cli -- date easter_sunday 2025
```

### Binary Location

After building, the binary is located at:

- Development: `target/debug/romcal`
- Release: `target/release/romcal`

## Commands

| Command                        | Description                                           |
| ------------------------------ | ----------------------------------------------------- |
| `date <DATE_NAME> [YEAR]`      | Calculate a specific liturgical date                  |
| `calendar [YEAR]`              | Generate liturgical calendar (by liturgical date)     |
| `masses [YEAR]`                | Generate mass-centric calendar (by civil date + mass) |
| `list calendars [--tree]`      | List available calendars                              |
| `list locales [--tree]`        | List available locales                                |
| `preset`                       | Display current configuration                         |
| `bundle`                       | Bundle required data (definitions + resources)        |
| `validate definitions <FILES>` | Validate calendar definition files                    |
| `validate resources <FILES>`   | Validate resource files                               |
| `martyrology <ID>`             | Lookup a single martyrology entry by its exact ID     |
| `search [TEXT]`                | Fuzzy search for martyrology entries with filtering   |
| `completions <SHELL>`          | Generate shell completion scripts                     |

### date

Calculate a specific liturgical date by its ID. Returns date in `YYYY-MM-DD` format.

```bash
romcal date easter_sunday 2025
romcal date pentecost_sunday          # Uses current year
romcal date easter_sunday 2025 --easter-calc julian
romcal date ordinary_time_5_monday 2025
```

Any date ID from the liturgical calendar can be used (e.g., `easter_sunday`, `christmas`, `ordinary_time_5_monday`).

### calendar

Generate liturgical calendar organized by liturgical date.

```bash
romcal calendar 2025
romcal calendar 2025 --filter id,fullname,date
romcal calendar 2025 --context liturgical --locale fr
romcal calendar 2025 -f json > calendar.json
```

**Filter options** (supports dot notation for nested fields like `colors.key`):

| Category   | Filters                                                                                           |
| ---------- | ------------------------------------------------------------------------------------------------- |
| Basic      | `id`, `fullname`, `date`                                                                          |
| Hierarchy  | `precedence`, `rank`, `rank_name`                                                                 |
| Temporal   | `season`, `periods`, `colors`                                                                     |
| Cycles     | `sunday_cycle`, `weekday_cycle`, `psalter_week`                                                   |
| Position   | `week_of_season`, `day_of_season`, `day_of_week`                                                  |
| Boundaries | `start_of_season`, `end_of_season`, `start_of_liturgical_year`, `end_of_liturgical_year`          |
| Metadata   | `commons`, `titles`, `martyrology`, `is_holy_day_of_obligation`, `is_optional`                    |
| Advanced   | `date_def`, `date_exceptions`, `from_calendar_id`, `allow_similar_rank_items`, `parent_overrides` |

### masses

Generate mass-centric calendar organized by civil date and mass time. Useful for planning liturgical celebrations.

```bash
romcal masses 2025
romcal masses 2025 --filter civil_date,id,fullname,rank
romcal masses 2025 --filter optional_celebrations.id,optional_celebrations.rank
romcal masses 2025 -f json > masses.json
```

**Key fields:**

| Field                   | Description                                      |
| ----------------------- | ------------------------------------------------ |
| `mass_time`             | Type of mass (DAY_MASS, VIGIL_MASS, etc.)        |
| `civil_date`            | Civil calendar date                              |
| `liturgical_date`       | Liturgical calendar date                         |
| `optional_celebrations` | Alternative celebrations available for this mass |

Nested fields can be filtered with dot notation: `optional_celebrations.id`, `colors.key`

### list

```bash
romcal list calendars              # List all calendars
romcal list calendars --tree       # Show calendar hierarchy
romcal list locales                # List all locales
romcal list locales -f json        # Output as JSON
```

### bundle

Generate an optimized JSON bundle containing only the calendar definitions and resources required for the current preset configuration.

```bash
romcal bundle > bundle.json
romcal bundle --calendar france --locale fr > france-fr.json
```

### validate

Validate JSON files against Romcal schemas.

```bash
romcal validate definitions path/to/calendar.json
romcal validate resources "data/resources/**/*.json"
```

### martyrology

Lookup a single martyrology entry by its exact ID. Returns entry details in the specified format.

```bash
romcal martyrology francis_of_assisi
romcal martyrology francis_of_assisi -f json
romcal martyrology our_lady_of_lourdes --locale fr
```

**Output fields (CSV/lines):**

| Field               | Description                  |
| ------------------- | ---------------------------- |
| `id`                | Martyrology entry identifier |
| `fullname`          | Full display name            |
| `name`              | Short name                   |
| `type`              | Entry type                   |
| `sex`               | Sex (MALE, FEMALE, etc.)     |
| `canonizationLevel` | Canonization level           |

### search

Fuzzy search for martyrology entries with advanced filtering capabilities. Returns ranked results based on relevance score.

```bash
romcal search "francis"
romcal search "saint john" --type SAINT --limit 10
romcal search --type SAINT --sex MALE --level CANONIZED
romcal search "mary" --title VIRGIN -f json
```

**Filter options:**

| Option        | Description                                    |
| ------------- | ---------------------------------------------- |
| `--type`      | Filter by entry type (e.g., SAINT, BLESSED)    |
| `--sex`       | Filter by sex (MALE, FEMALE)                   |
| `--level`     | Filter by canonization level                   |
| `--title`     | Filter by title(s) - can be repeated           |
| `--limit`     | Maximum number of results (default: 20)        |
| `--min_score` | Minimum score threshold 0.0-1.0 (default: 0.3) |

**Output fields:**

| Field               | Description                    |
| ------------------- | ------------------------------ |
| `id`                | Martyrology entry identifier   |
| `score`             | Relevance score (0.0-1.0)      |
| `matchType`         | Type of match                  |
| `matchedFields`     | Fields that matched the search |
| `fullname`          | Full display name              |
| `name`              | Short name                     |
| `type`              | Entry type                     |
| `sex`               | Sex                            |
| `canonizationLevel` | Canonization level             |

## Options

### Global Options

```
-C, --config <PATH>     Config file path
-h, --help              Print help
-V, --version           Print version
```

### Preset Options

Available on `date`, `calendar`, `masses`, `preset`, `bundle`, `martyrology`, and `search` commands:

```
-c, --calendar <NAME>           Calendar to use (default: general_roman)
-l, --locale <CODE>             Locale to use (default: en)
-t, --context <TYPE>            gregorian | liturgical (default: gregorian)
    --easter-calc <TYPE>        gregorian | julian (default: gregorian)
    --epiphany-on-sunday        Celebrate Epiphany on Sunday
    --ascension-on-sunday       Celebrate Ascension on Sunday
    --corpus-christi-on-sunday  Celebrate Corpus Christi on Sunday
-d, --definitions <PATHS>       Custom calendar definition files
-r, --resources <PATHS>         Custom resource files
```

### Output Options

```
-f, --format <FORMAT>   yaml | json | csv | lines (default: yaml)
-D, --debug             Show debug information
```

## Configuration

Romcal CLI loads configuration from (in priority order):

1. CLI flags (highest priority)
2. `--config` specified file
3. `.romcal.toml` in current directory
4. `~/.config/romcal/config.toml` (Linux/macOS) or `%APPDATA%\romcal\config.toml` (Windows)

### Example Configuration

```toml
# .romcal.toml
calendar = "france"
locale = "fr"
format = "yaml"
context = "gregorian"
easter_calculation_type = "gregorian"
epiphany_on_sunday = true
ascension_on_sunday = false
corpus_christi_on_sunday = true
```

## Output Formats

| Format  | Description                          | Use case          |
| ------- | ------------------------------------ | ----------------- |
| `yaml`  | Human-readable, structured (default) | Manual inspection |
| `json`  | Machine-readable, structured         | API integration   |
| `csv`   | Tabular data                         | Spreadsheets      |
| `lines` | One item per line                    | Shell scripting   |

### Examples

```bash
# JSON for API consumption
romcal calendar 2025 -f json > calendar.json
romcal masses 2025 -f json > masses.json

# CSV for spreadsheet
romcal calendar 2025 --filter id,fullname,date -f csv > calendar.csv

# Lines for shell scripting
romcal list locales -f lines | while read locale; do
  echo "Processing $locale"
done
```

## Shell Completion

Generate completion scripts for your shell:

```bash
# Bash
romcal completions bash > ~/.bash_completion.d/romcal

# Zsh
romcal completions zsh > ~/.zfunc/_romcal

# Fish
romcal completions fish > ~/.config/fish/completions/romcal.fish

# PowerShell
romcal completions powershell >> $PROFILE
```

Supported shells: `bash`, `zsh`, `fish`, `powershell`, `elvish`

## Exit Codes

| Code | Meaning                                                        |
| ---- | -------------------------------------------------------------- |
| 0    | Success                                                        |
| 1    | Error (invalid input, file not found, validation failed, etc.) |

## Development

```bash
# Run tests
cargo test -p romcal-cli

# Run quality checks
./scripts/check-cli.sh

# Build release
./scripts/build-cli.sh
```

## Related

- [romcal](https://github.com/romcal/romcal) - Main Romcal project
- [romcal](../core/) - Core Rust library
- [romcal (TypeScript)](../bindings/typescript/) - TypeScript/JavaScript binding
- [romcal (Python)](../bindings/python/) - Python binding

## License

Apache License 2.0. See [LICENSE](../LICENSE) for details.
