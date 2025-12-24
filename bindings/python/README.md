# Romcal

A Python library for calculating Catholic liturgical dates and generating liturgical calendars. Powered by Rust via UniFFI bindings.

For the Rust library, see [romcal](../../core/). For command-line usage, see the [CLI documentation](../../cli/).

## Installation

```bash
pip install romcal
```

Or with [uv](https://docs.astral.sh/uv/):

```bash
uv add romcal
```

## Quick Start

```python
from romcal import Romcal

# Create a default instance
romcal = Romcal()

# Get a specific liturgical date
easter = romcal.get_date("easter_sunday", 2026)
print(easter)  # "2026-04-05"

# Generate the liturgical calendar for year 2026
calendar = romcal.liturgical_calendar(2026)

# Access a specific date
christmas = calendar.get("2026-12-25")
if christmas:
    print(christmas[0]["fullname"])  # "The Nativity of the Lord"
```

## Configuration

### Using Keyword Arguments

```python
from romcal import Romcal

# With calendar and locale
romcal1 = Romcal(calendar="france", locale="fr")

# With full configuration
romcal2 = Romcal(
    calendar="france",
    locale="fr",
    context="LITURGICAL",
    epiphany_on_sunday=True,
    ascension_on_sunday=True,
    corpus_christi_on_sunday=True,
)
```

### Configuration Options

| Option                      | Type   | Default           | Description                                               |
| --------------------------- | ------ | ----------------- | --------------------------------------------------------- |
| `calendar`                  | `str`  | `"general_roman"` | Calendar ID (e.g., `"france"`, `"united_states"`)         |
| `locale`                    | `str`  | `"en"`            | Locale code (e.g., `"fr"`, `"es"`)                        |
| `context`                   | `str`  | `"GREGORIAN"`     | `"GREGORIAN"` (Jan-Dec) or `"LITURGICAL"` (Advent-Advent) |
| `epiphany_on_sunday`        | `bool` | `False`           | Celebrate Epiphany on Sunday (Jan 2-8) instead of Jan 6   |
| `ascension_on_sunday`       | `bool` | `False`           | Celebrate Ascension on Sunday instead of Thursday         |
| `corpus_christi_on_sunday`  | `bool` | `True`            | Celebrate Corpus Christi on Sunday instead of Thursday    |
| `easter_calculation_type`   | `str`  | `"GREGORIAN"`     | `"GREGORIAN"` or `"JULIAN"` Easter calculation            |
| `calendar_definitions_json` | `str`  | `None`            | JSON string of calendar definitions                       |
| `resources_json`            | `str`  | `None`            | JSON string of locale resources                           |

### Loading Calendar Data

Without loading data, only the Proper of Time is available. To include the General Roman Calendar, particular calendars, and localized names, load calendar definitions and resources:

```python
import json
from pathlib import Path
from romcal import Romcal

DATA_DIR = Path("data")

def load_calendar_definitions():
    """Load all calendar definitions from the data folder."""
    definitions = []
    for json_file in (DATA_DIR / "definitions").rglob("*.json"):
        with open(json_file) as f:
            definitions.append(json.load(f))
    return definitions

def load_resources():
    """Load all resources from the data folder."""
    resources_dir = DATA_DIR / "resources"
    resources = []

    # Group files by locale
    files_by_locale = {}
    for json_file in resources_dir.rglob("*.json"):
        locale = json_file.parent.name
        files_by_locale.setdefault(locale, []).append(json_file)

    # Merge files for each locale
    for locale, locale_files in files_by_locale.items():
        metadata = None
        entities = {}

        for file in locale_files:
            with open(file) as f:
                content = json.load(f)
            if file.name == "meta.json":
                metadata = content.get("metadata")
            elif file.name.startswith("entities.") and "entities" in content:
                entities.update(content["entities"])

        resources.append({
            "locale": locale,
            "metadata": metadata,
            "entities": entities if entities else None,
        })

    return resources

# Create instance with loaded data
romcal = Romcal(
    calendar="france",
    locale="fr",
    calendar_definitions_json=json.dumps(load_calendar_definitions()),
    resources_json=json.dumps(load_resources()),
)
```

## API

### Romcal()

Creates a new Romcal instance.

```python
from romcal import Romcal

# Default configuration
romcal1 = Romcal()

# With calendar and locale
romcal2 = Romcal(calendar="france", locale="fr")

# With partial configuration
romcal3 = Romcal(
    calendar="france",
    locale="fr",
    epiphany_on_sunday=True,
)
```

### Romcal Instance

#### liturgical_calendar(year)

Generate the complete liturgical calendar for a given year.

```python
calendar = romcal.liturgical_calendar(2026)
# calendar is dict[str, list[dict]]
# Keys are dates in "YYYY-MM-DD" format

for date, days in calendar.items():
    for day in days:
        print(f"{date}: {day['fullname']} ({day['rank']})")
```

#### mass_calendar(year)

Generate a mass-centric view of the calendar organized by civil date and mass time.

```python
mass_calendar = romcal.mass_calendar(2026)
# mass_calendar is dict[str, list[dict]]

# Evening masses appear on the previous civil day
easter_vigil_day = mass_calendar.get("2026-04-04")
if easter_vigil_day:
    vigil = next((m for m in easter_vigil_day if m["mass_time"] == "EASTER_VIGIL"), None)
    if vigil:
        print(vigil["liturgical_date"])  # "2026-04-05"
```

#### get_date(id, year)

Get a liturgical date by its ID.

```python
easter = romcal.get_date("easter_sunday", 2026)      # "2026-04-05"
ash_wed = romcal.get_date("ash_wednesday", 2026)     # "2026-02-18"
pentecost = romcal.get_date("pentecost_sunday", 2026) # "2026-05-24"
christmas = romcal.get_date("christmas", 2026)        # "2026-12-25"
```

Any date ID from the liturgical calendar can be used (e.g., `easter_sunday`, `christmas`, `ordinary_time_5_monday`).

#### Properties

Access the resolved configuration:

```python
print(romcal.calendar)                # "france"
print(romcal.locale)                  # "fr"
print(romcal.epiphany_on_sunday)      # True
print(romcal.ascension_on_sunday)     # False
print(romcal.corpus_christi_on_sunday) # True
print(romcal.easter_calculation_type)  # "GREGORIAN"
print(romcal.context)                  # "GREGORIAN"
```

## Key Types

For detailed documentation on liturgical types (seasons, ranks, precedence, colors, cycles, mass times), see the [romcal documentation](../../core/README.md#key-types).

## Error Handling

All operations may raise `RomcalError`:

```python
from romcal import Romcal, RomcalError

try:
    romcal = Romcal()
    # Year must be >= 1583 (Gregorian calendar adoption)
    calendar = romcal.liturgical_calendar(1500)
except RomcalError as e:
    print(f"Romcal error: {e}")
```

## Development

### Requirements

- [Python](https://www.python.org/) 3.10 or later
- [uv](https://docs.astral.sh/uv/) (recommended) or pip
- [Rust](https://rustup.rs/) 1.85 or later

### Setup

```bash
cd bindings/python

# Create virtual environment
uv venv

# Install build tools
uv pip install maturin uniffi-bindgen

# Build and install the native extension
uv run maturin develop

# Install dev dependencies (pytest, ruff, etc.)
uv pip install pytest taskipy ruff mypy
```

### Available Tasks

Using [taskipy](https://github.com/taskipy/taskipy):

```bash
task build          # maturin build --release
task develop        # maturin develop
task generate-types # Generate Pydantic types from JSON schema
task test           # pytest tests/ -v
task test-run       # pytest tests/ (without verbose)
task format         # ruff format .
task format-check   # ruff format --check .
task lint           # ruff check .
task lint-fix       # ruff check --fix .
task typecheck      # mypy src/
```

### Testing

```bash
task test      # Run tests with verbose output
task test-run  # Run tests once
```

### Project Structure

```
bindings/python/
├── src/
│   └── romcal/
│       ├── __init__.py    # Main entry point, API wrapper
│       ├── types.py       # Generated types from JSON schema (Pydantic)
│       └── _uniffi/       # Generated UniFFI bindings
├── tests/
│   ├── conftest.py        # Pytest fixtures (data loading)
│   ├── test_config.py     # Configuration tests
│   ├── test_calendar.py   # Calendar generation tests
│   └── test_data_loading.py # Data loading tests
├── examples/
│   └── basic_usage.py     # Usage example with data loading
└── pyproject.toml         # Project configuration
```

### Running Examples

```bash
# Basic usage example (loads data from /data folder)
python examples/basic_usage.py
```

## Related

- [romcal](https://github.com/romcal/romcal) - Main Romcal project
- [romcal](../../core/) - Core Rust library
- [romcal-cli](../../cli/) - Command-line interface
- [romcal (TypeScript)](../typescript/) - TypeScript/JavaScript binding

## License

Apache License 2.0. See [LICENSE](../../LICENSE) for details.
