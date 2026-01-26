---
title: Calendar Definitions
---

# Calendar Definitions

Calendar definitions specify liturgical celebrations for a particular region, country, diocese, or community.

## File Location

Definitions are stored in `data/definitions/` with the following structure:

```
data/definitions/
├── general_roman/     # General Roman Calendar (base)
├── regions/           # Regional calendars (Europe, Americas, etc.)
├── countries/         # National and diocesan calendars
└── communities/       # Religious communities and orders
```

## File Format

Each calendar is defined in a JSON file:

```json
{
  "$schema": "../../../schemas/definitions.json",
  "id": "france__paris",
  "metadata": {
    "type": "ARCHDIOCESE",
    "jurisdiction": "ECCLESIASTICAL"
  },
  "parentCalendarIds": ["europe", "france"],
  "inputs": {
    "our_lady_of_paris": {
      "date_def": { "month": 2, "day": 9 },
      "rank": "FEAST",
      "liturgical_colors": ["WHITE"]
    },
    "denis_of_paris_bishop": {
      "date_def": { "month": 10, "day": 9 },
      "rank": "SOLEMNITY"
    }
  }
}
```

## Key Fields

### id

Unique identifier for the calendar. Uses `snake_case` with double underscores (`__`) for hierarchy:

- `france` - National calendar
- `france__paris` - Diocese within France
- `france__paris__city` - City within diocese

### metadata

Describes the type of calendar:

```json
{
  "type": "DIOCESE",
  "jurisdiction": "ECCLESIASTICAL"
}
```

**Types**: `COUNTRY`, `REGION`, `ARCHDIOCESE`, `DIOCESE`, `COMMUNITY`, etc.

### parentCalendarIds

Calendars this calendar inherits from:

```json
["europe", "france"]
```

Celebrations are inherited from parents and can be overridden.

### inputs

The celebrations specific to this calendar:

```json
{
  "celebration_id": {
    "date_def": { "month": 6, "day": 29 },
    "rank": "SOLEMNITY",
    "liturgical_colors": ["RED"],
    "precedence": 35
  }
}
```

## Celebration Fields

| Field                       | Description                                        |
| --------------------------- | -------------------------------------------------- |
| `date_def`                  | Fixed date `{month, day}` or computed reference    |
| `rank`                      | Liturgical rank (SOLEMNITY, FEAST, MEMORIAL, etc.) |
| `liturgical_colors`         | Array of colors (WHITE, RED, GREEN, VIOLET, etc.)  |
| `precedence`                | Override precedence level (1-100)                  |
| `is_holy_day_of_obligation` | Whether attendance is obligatory                   |

## Date Definitions

### Fixed Date

```json
{
  "date_def": { "month": 12, "day": 25 }
}
```

### Relative to Another Date

```json
{
  "date_def": {
    "ref": "easter_sunday",
    "offset": 49
  }
}
```

### Day of Week in Month

```json
{
  "date_def": {
    "month": 11,
    "day_of_week": "SUNDAY",
    "nth_day_of_week_in_month": -1
  }
}
```

## Validation

Validate your definition files:

```bash
romcal validate definitions path/to/your/calendar.json
```

## Example: Adding a Diocese

1. Create `data/definitions/countries/france/france__lyon.json`:

```json
{
  "$schema": "../../../../schemas/definitions.json",
  "id": "france__lyon",
  "metadata": {
    "type": "ARCHDIOCESE"
  },
  "parentCalendarIds": ["europe", "france"],
  "inputs": {
    "pothinus_of_lyon_bishop": {
      "date_def": { "month": 6, "day": 2 },
      "rank": "MEMORIAL"
    },
    "irenaeus_of_lyon_bishop": {
      "date_def": { "month": 6, "day": 28 },
      "rank": "SOLEMNITY"
    }
  }
}
```

2. Add the corresponding entity in the resources (if not already present)

3. Validate: `romcal validate definitions data/definitions/countries/france/france__lyon.json`
