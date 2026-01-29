---
title: Data Structure
---

This page explains the organization of data files in the Romcal repository.

## Overview

```
data/
├── definitions/           # Calendar definitions
│   ├── general_roman/     # Base calendar (General Roman Calendar)
│   ├── regions/           # Regional calendars
│   ├── countries/         # National and diocesan calendars
│   └── communities/       # Religious communities
└── resources/             # Martyrology translations
    ├── en/                # English (default)
    ├── fr/                # French
    ├── es/                # Spanish
    └── ...
```

## Definitions Directory

### general_roman/

Contains the General Roman Calendar, which is the base calendar for the entire Roman Rite.

```
general_roman/
├── general_roman.json         # Main celebrations
├── proper_of_time.json        # Temporal cycle (Advent, Lent, Easter, etc.)
└── proper_of_saints.json      # Sanctoral cycle (fixed feast days)
```

### regions/

Regional calendars that apply to multiple countries:

```
regions/
├── europe.json
├── americas.json
├── asia.json
├── africa.json
└── oceania.json
```

### countries/

National calendars and their subdivisions:

```
countries/
├── france/
│   ├── france.json              # National calendar
│   ├── france__paris.json       # Archdiocese of Paris
│   ├── france__lyon.json        # Archdiocese of Lyon
│   └── france__strasbourg.json  # Diocese of Strasbourg
├── italy/
│   ├── italy.json
│   └── ...
└── united_states/
    ├── united_states.json
    └── ...
```

### communities/

Religious communities and orders:

```
communities/
├── benedictine/
│   └── benedictine.json
├── franciscan/
│   └── franciscan.json
└── dominican/
    └── dominican.json
```

## Resources Directory

Each locale has its own subdirectory with martyrology files organized alphabetically:

```
resources/
├── en/
│   ├── martyrology.a.json    # andrew_apostle, anthony_of_padua, ...
│   ├── martyrology.b.json    # basil_the_great, benedict_of_nursia, ...
│   ├── martyrology.c.json    # catherine_of_siena, charles_borromeo, ...
│   └── ...
├── fr/
│   ├── martyrology.a.json    # French translations
│   └── ...
└── la/
    ├── martyrology.a.json    # Latin names
    └── ...
```

## Inheritance Model

Calendars inherit from their parents in this order:

```
general_roman
    └── region (e.g., europe)
        └── country (e.g., france)
            └── subdivision (e.g., france__paris)
```

When a celebration is defined at multiple levels:

1. The most specific calendar wins
2. Parent definitions provide defaults
3. Precedence rules determine which celebration takes priority on a given day

## JSON Schema Validation

All definition and resource files reference JSON schemas for validation:

```json
{
  "$schema": "../../../schemas/definitions.json",
  "id": "france",
  ...
}
```

Validate files with the CLI:

```bash
# Validate a single file
romcal validate definitions data/definitions/countries/france/france.json

# Validate all resource files
romcal validate resources "data/resources/**/*.json"
```

## Adding New Data

### New Country Calendar

1. Create directory: `data/definitions/countries/new_country/`
2. Create file: `new_country.json`
3. Set `parentCalendarIds` to include the appropriate region
4. Add country-specific celebrations in `inputs`

### New Diocese

1. Create file in country directory: `country__diocese.json`
2. Set `parentCalendarIds` to include the country
3. Add diocese-specific celebrations

### New Locale

1. Create directory: `data/resources/xx/` (ISO language code)
2. Create martyrology files: `martyrology.a.json`, `martyrology.b.json`, etc.
3. Translate martyrology entries from English source

## Best Practices

1. **Validate before committing**: Always run validation
2. **Follow naming conventions**: See [Naming Conventions](./naming-conventions)
3. **Include sources**: Document where celebration data comes from
4. **Start minimal**: Add only celebrations that differ from parent calendars
5. **Test locally**: Generate a calendar with your changes to verify
