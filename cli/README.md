# Romcal CLI

A command-line interface for the Romcal liturgical calendar library, allowing you to calculate Catholic liturgical dates and calendars.

## Installation

### From Source

```bash
# Build and install from source
cargo install --path .

# Or run directly during development
cargo run -- dates 2024 easter_sunday
```

### Using Build Scripts

```bash
# Build CLI only
./scripts/build-cli.sh

# Build entire project (includes CLI)
./scripts/build-all.sh

# Run quality checks on CLI
./scripts/check-cli.sh
```

### Binary Locations

After building, the CLI binary is located at:

- **Development**: `target/debug/romcal`
- **Release**: `target/release/romcal`

```bash
# Test the built binary
./target/release/romcal --help
./target/release/romcal list-locales
```

## Quick Start

```bash
# Calculate Easter for 2024
romcal dates 2024 easter_sunday

# Calculate Christmas (current year)
romcal dates christmas

# Calculate Pentecost with options
romcal dates --corpus-christi-on-sunday 2024 pentecost-sunday

# List available calendars and locales
romcal list-calendars
romcal list-locales
```

## Commands

### Date Calculations

The `dates` command allows you to calculate specific liturgical dates. All date commands return the date in YYYY-MM-DD format.

#### Available Date Commands

| Command                         | Description                   | Example                                              |
| ------------------------------- | ----------------------------- | ---------------------------------------------------- |
| `mary_mother_of_the_church`     | Mary Mother of the Church     | `romcal dates mary_mother_of_the_church -y 2024`     |
| `epiphany_sunday`               | Epiphany Sunday               | `romcal dates epiphany_sunday -y 2024`               |
| `presentation_of_the_lord`      | Presentation of the Lord      | `romcal dates presentation_of_the_lord -y 2024`      |
| `annunciation`                  | Annunciation                  | `romcal dates annunciation -y 2024`                  |
| `palm_sunday`                   | Palm Sunday                   | `romcal dates palm_sunday -y 2024`                   |
| `easter_sunday`                 | Easter Sunday                 | `romcal dates easter_sunday -y 2024`                 |
| `divine_mercy_sunday`           | Divine Mercy Sunday           | `romcal dates divine_mercy_sunday -y 2024`           |
| `immaculate_heart_of_mary`      | Immaculate Heart of Mary      | `romcal dates immaculate_heart_of_mary -y 2024`      |
| `pentecost_sunday`              | Pentecost Sunday              | `romcal dates pentecost_sunday -y 2024`              |
| `corpus_christi_sunday`         | Corpus Christi Sunday         | `romcal dates corpus_christi_sunday -y 2024`         |
| `nativity_of_john_the_baptist`  | Nativity of John the Baptist  | `romcal dates nativity_of_john_the_baptist -y 2024`  |
| `peter_and_paul_apostles`       | Peter and Paul Apostles       | `romcal dates peter_and_paul_apostles -y 2024`       |
| `transfiguration`               | Transfiguration               | `romcal dates transfiguration -y 2024`               |
| `assumption`                    | Assumption                    | `romcal dates assumption -y 2024`                    |
| `exaltation_of_the_holy_cross`  | Exaltation of the Holy Cross  | `romcal dates exaltation_of_the_holy_cross -y 2024`  |
| `all_saints`                    | All Saints                    | `romcal dates all_saints -y 2024`                    |
| `immaculate_conception_of_mary` | Immaculate Conception of Mary | `romcal dates immaculate_conception_of_mary -y 2024` |

### Information

| Command          | Description              |
| ---------------- | ------------------------ |
| `list-calendars` | List available calendars |
| `list-locales`   | List supported locales   |
| `config`         | Show configuration info  |

## Options

### Global Options

These options are available for all commands:

- `-d, --debug` - Show debug information
- `-f, --format <FORMAT>` - Output format (json, csv, yaml, lines) [default: yaml]
- `--easter-calculation-type <TYPE>` - Easter calculation type (gregorian, julian) [default: gregorian]
- `--scope <SCOPE>` - Calendar scope (gregorian, liturgical) [default: gregorian]
- `--ascension-on-sunday` - Celebrate Ascension on Sunday
- `--epiphany-on-sunday` - Celebrate Epiphany on Sunday
- `--corpus-christi-on-sunday` - Celebrate Corpus Christi on Sunday

### Date Command Specific Options

These options are only available for the `dates` command:

- `-y, --year <YEAR>` - Year for date calculations (default: current year)

### Format Options

All commands support the following output formats:

- `json` - JSON format
- `csv` - Comma-separated values
- `yaml` - YAML format (default)
- `lines` - One value per line (perfect for CLI automation)

## Examples

```bash
# Basic date calculations (uses current year)
romcal dates easter_sunday
romcal dates all_saints

# With specific year (using short form)
romcal dates easter_sunday -y 2024
romcal dates all_saints -y 2024

# With specific year (using long form)
romcal dates easter_sunday --year 2024
romcal dates all_saints --year 2024

# Easter with Julian calculation
romcal dates easter_sunday -y 2024 --easter-calculation-type julian

# JSON output format
romcal dates easter_sunday -y 2024 -f json

# CSV output format
romcal dates pentecost_sunday -y 2024 -f csv

# YAML output format (default)
romcal dates assumption -y 2024

# Special celebration options
romcal dates easter_sunday -y 2024 --ascension-on-sunday
romcal dates epiphany_sunday -y 2024 --epiphany-on-sunday
romcal dates corpus_christi_sunday -y 2024 --corpus-christi-on-sunday

# Liturgical year scope
romcal dates easter_sunday -y 2024 --scope liturgical

# Multiple options combined
romcal dates easter_sunday -y 2024 -f json --easter-calculation-type julian --ascension-on-sunday

# List available locales and calendars
romcal list-locales
romcal list-calendars

# List commands with different formats
romcal list-locales -f json
romcal list-calendars -f csv
romcal list-locales -f lines

# CLI automation examples
romcal list-locales -f lines | while read locale; do
  echo "Processing $locale"
done

romcal list-calendars -f csv | tr ',' '\n' | head -5
```

## Output Formats

### YAML Format (default)

**Date calculations:**

```
2024-03-31
```

**List commands:**

```yaml
- en
- fr
- es
- de
- it
- la
- pl
- pt-br
- sk
- ta
- cs
- en-gb
- en-ie
```

### JSON Format

**Date calculations:**

```json
"2024-03-31"
```

**List commands:**

```json
["en", "fr", "es", "de"]
```

### CSV Format

**Date calculations:**

```
2024-03-31
```

**List commands:**

```
en,fr,es,de,it,la,pl,pt-br,sk,ta,cs,en-gb,en-ie
```

### Lines Format

**Date calculations:**

```
2024-03-31
```

**List commands:**

```
en
fr
es
de
it
la
pl
pt-br
sk
ta
cs
en-gb
en-ie
```

## Usage Notes

- All date calculations return the date in YYYY-MM-DD format
- The `-y, --year` parameter is only available for the `dates` command and defaults to the current year
- Most options are global and available for all commands
- All commands support format validation with helpful error messages
- Invalid formats will show: `Error: Configuration error: Invalid format. Must be 'json', 'csv', 'yaml', or 'lines'`
- Use short forms (`-y`, `-f`, `-d`) for brevity or long forms (`--year`, `--format`, `--debug`) for clarity
- Multiple options can be combined for complex calculations

## License

MIT
