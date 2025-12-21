# Romcal CLI

A command-line interface for calculating Catholic liturgical dates and generating liturgical calendars.

## Quick Start

```bash
# Get Easter date for 2025
romcal dates easter_sunday 2025

# Generate liturgical calendar for current year
romcal days

# Generate calendar with specific locale and calendar
romcal days 2025 --calendar france --locale fr

# List available calendars
romcal list calendars
```

## Installation

### Requirements

- [Rust](https://rustup.rs/) 1.70 or later

### From Source

```bash
# Clone the repository
git clone https://github.com/romcal/romcal.git
cd romcal

# Build and install
cargo install --path cli

# Or run directly
cargo run -p romcal-cli -- dates easter_sunday 2025
```

### Binary Location

After building, the binary is located at:

- Development: `target/debug/romcal`
- Release: `target/release/romcal`

## Commands

| Command                        | Description                             |
| ------------------------------ | --------------------------------------- |
| `dates <DATE_NAME> [YEAR]`     | Calculate a specific liturgical date    |
| `days [YEAR]`                  | Generate all liturgical days for a year |
| `list calendars [--tree]`      | List available calendars                |
| `list locales [--tree]`        | List available locales                  |
| `preset`                       | Display current configuration           |
| `optimize-preset`              | Generate optimized JSON bundle          |
| `validate definitions <FILES>` | Validate calendar definition files      |
| `validate resources <FILES>`   | Validate resource files                 |
| `completions <SHELL>`          | Generate shell completion scripts       |

### dates

Calculate specific liturgical dates. Returns date in `YYYY-MM-DD` format.

```bash
romcal dates easter_sunday 2025
romcal dates pentecost_sunday          # Uses current year
romcal dates easter_sunday 2025 --easter-calc julian
```

**Available dates:**

`mary_mother_of_the_church`, `epiphany_sunday`, `presentation_of_the_lord`, `annunciation`, `palm_sunday`, `easter_sunday`, `divine_mercy_sunday`, `ascension`, `pentecost_sunday`, `corpus_christi_sunday`, `immaculate_heart_of_mary`, `nativity_of_john_the_baptist`, `peter_and_paul_apostles`, `transfiguration`, `assumption`, `exaltation_of_the_holy_cross`, `all_saints`, `immaculate_conception_of_mary`

### days

Generate liturgical days for an entire year.

```bash
romcal days 2025
romcal days 2025 --filter id,fullname,date
romcal days 2025 --context liturgical --locale fr
romcal days 2025 -f json > calendar.json
```

**Filter options:**

| Category   | Filters                                                                                           |
| ---------- | ------------------------------------------------------------------------------------------------- |
| Basic      | `id`, `fullname`, `date`                                                                          |
| Hierarchy  | `precedence`, `rank`, `rank_name`                                                                 |
| Temporal   | `season`, `periods`, `colors`                                                                     |
| Cycles     | `sunday_cycle`, `weekday_cycle`, `psalter_week`                                                   |
| Position   | `week_of_season`, `day_of_season`, `day_of_week`                                                  |
| Boundaries | `start_of_season`, `end_of_season`, `start_of_liturgical_year`, `end_of_liturgical_year`          |
| Metadata   | `commons`, `titles`, `entities`, `is_holy_day_of_obligation`, `is_optional`                       |
| Advanced   | `date_def`, `date_exceptions`, `from_calendar_id`, `allow_similar_rank_items`, `parent_overrides` |

### list

```bash
romcal list calendars              # List all calendars
romcal list calendars --tree       # Show calendar hierarchy
romcal list locales                # List all locales
romcal list locales -f json        # Output as JSON
```

### validate

Validate JSON files against Romcal schemas.

```bash
romcal validate definitions path/to/calendar.json
romcal validate resources "data/resources/**/*.json"
```

## Options

### Global Options

```
-C, --config <PATH>     Config file path
-h, --help              Print help
-V, --version           Print version
```

### Preset Options

Available on `dates`, `days`, `preset`, and `optimize-preset` commands:

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
romcal days 2025 -f json > calendar.json

# CSV for spreadsheet
romcal days 2025 --filter id,fullname,date -f csv > calendar.csv

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
- [romcal-core](../core/) - Core library

## License

Apache-2.0
