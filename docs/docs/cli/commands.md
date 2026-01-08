---
sidebar_position: 3
---

# Commands Reference

Detailed documentation for all CLI commands.

## Global Options

These options are available for all commands:

```
-C, --config <PATH>     Config file path
-h, --help              Print help
-V, --version           Print version
```

## Preset Options

Available on `date`, `calendar`, `masses`, `preset`, `bundle`, `entity`, and `search` commands:

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

## Output Options

```
-f, --format <FORMAT>   yaml | json | csv | lines (default: yaml)
-D, --debug             Show debug information
```

---

## date

Calculate a specific liturgical date by its ID.

```bash
romcal date <DATE_NAME> [YEAR]
```

**Examples:**

```bash
romcal date easter_sunday 2025
romcal date pentecost_sunday              # Uses current year
romcal date easter_sunday 2025 --easter-calc julian
romcal date ordinary_time_5_monday 2025
```

**Output:** Date in `YYYY-MM-DD` format.

---

## calendar

Generate liturgical calendar organized by liturgical date.

```bash
romcal calendar [YEAR] [OPTIONS]
```

**Examples:**

```bash
romcal calendar 2025
romcal calendar 2025 --filter id,fullname,date
romcal calendar 2025 --context liturgical --locale fr
romcal calendar 2025 -f json > calendar.json
```

**Filter options:**

| Category   | Filters                                                                                  |
| ---------- | ---------------------------------------------------------------------------------------- |
| Basic      | `id`, `fullname`, `date`                                                                 |
| Hierarchy  | `precedence`, `rank`, `rank_name`                                                        |
| Temporal   | `season`, `periods`, `colors`                                                            |
| Cycles     | `sunday_cycle`, `weekday_cycle`, `psalter_week`                                          |
| Position   | `week_of_season`, `day_of_season`, `day_of_week`                                         |
| Boundaries | `start_of_season`, `end_of_season`, `start_of_liturgical_year`, `end_of_liturgical_year` |
| Metadata   | `commons`, `titles`, `entities`, `is_holy_day_of_obligation`, `is_optional`              |

---

## masses

Generate mass-centric calendar organized by civil date and mass time.

```bash
romcal masses [YEAR] [OPTIONS]
```

**Examples:**

```bash
romcal masses 2025
romcal masses 2025 --filter civil_date,id,fullname,rank
romcal masses 2025 -f json > masses.json
```

**Key fields:**

| Field                   | Description                               |
| ----------------------- | ----------------------------------------- |
| `mass_time`             | Type of mass (DAY_MASS, VIGIL_MASS, etc.) |
| `civil_date`            | Civil calendar date                       |
| `liturgical_date`       | Liturgical calendar date                  |
| `optional_celebrations` | Alternative celebrations available        |

---

## list

List available calendars or locales.

```bash
romcal list calendars [--tree]
romcal list locales [--tree]
```

**Examples:**

```bash
romcal list calendars              # List all calendars
romcal list calendars --tree       # Show calendar hierarchy
romcal list locales -f json        # Output as JSON
```

---

## entity

Lookup a single entity by its exact ID.

```bash
romcal entity <ID> [OPTIONS]
```

**Examples:**

```bash
romcal entity francis_of_assisi
romcal entity francis_of_assisi -f json
romcal entity our_lady_of_lourdes --locale fr
```

---

## search

Fuzzy search for entities with filtering capabilities.

```bash
romcal search [TEXT] [OPTIONS]
```

**Options:**

| Option        | Description                                    |
| ------------- | ---------------------------------------------- |
| `--type`      | Filter by entity type (SAINT, BLESSED)         |
| `--sex`       | Filter by sex (MALE, FEMALE)                   |
| `--level`     | Filter by canonization level                   |
| `--title`     | Filter by title(s) - can be repeated           |
| `--limit`     | Maximum number of results (default: 20)        |
| `--min_score` | Minimum score threshold 0.0-1.0 (default: 0.3) |

**Examples:**

```bash
romcal search "francis"
romcal search "saint john" --type SAINT --limit 10
romcal search --type SAINT --sex MALE --level CANONIZED
romcal search "mary" --title VIRGIN -f json
```

---

## bundle

Generate an optimized JSON bundle for the current preset configuration.

```bash
romcal bundle [OPTIONS]
```

**Examples:**

```bash
romcal bundle > bundle.json
romcal bundle --calendar france --locale fr > france-fr.json
```

---

## validate

Validate JSON files against Romcal schemas.

```bash
romcal validate definitions <FILES>
romcal validate resources <FILES>
```

**Examples:**

```bash
romcal validate definitions path/to/calendar.json
romcal validate resources "data/resources/**/*.json"
```

---

## preset

Display the current configuration (merged from CLI options and config files).

```bash
romcal preset [OPTIONS]
```

---

## completions

Generate shell completion scripts.

```bash
romcal completions <SHELL>
```

**Supported shells:** `bash`, `zsh`, `fish`, `powershell`, `elvish`
