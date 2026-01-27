---
title: Entity Resources
---


Entity resources contain the names and metadata for liturgical entities (saints, feasts, etc.) in each locale.

## File Location

Resources are stored in `data/resources/` organized by locale:

```
data/resources/
├── en/                    # English
│   ├── entities.a.json    # Entities starting with 'a'
│   ├── entities.b.json    # Entities starting with 'b'
│   └── ...
├── fr/                    # French
│   ├── entities.a.json
│   └── ...
└── ...
```

## File Format

Each file contains entities for a single letter:

```json
{
  "$schema": "../../../schemas/resources.json",
  "locale": "en",
  "entities": {
    "francis_of_assisi": {
      "fullname": "Saint Francis of Assisi",
      "canonization_level": "SAINT",
      "name": "Francis of Assisi",
      "date_of_death": 1226
    },
    "frances_of_rome_religious": {
      "fullname": "Saint Frances of Rome, Religious",
      "canonization_level": "SAINT",
      "name": "Frances of Rome",
      "titles": ["RELIGIOUS"],
      "date_of_death": 1440
    }
  }
}
```

## Entity Fields

### Required Fields

| Field      | Description                       |
| ---------- | --------------------------------- |
| `fullname` | Complete display name with titles |

### Optional Fields

| Field                | Description                                       |
| -------------------- | ------------------------------------------------- |
| `name`               | Short name (without titles)                       |
| `canonization_level` | SAINT, BLESSED, VENERABLE, etc.                   |
| `titles`             | Array: POPE, BISHOP, PRIEST, MARTYR, VIRGIN, etc. |
| `hide_titles`        | Don't append titles to fullname                   |
| `sex`                | MALE or FEMALE                                    |
| `count`              | Number of individuals (for groups)                |
| `date_of_birth`      | Birth date or year                                |
| `date_of_death`      | Death date or year                                |
| `sources`            | Array of source references                        |

## Examples

### Simple Saint

```json
{
  "francis_xavier_priest": {
    "fullname": "Saint Francis Xavier, Priest",
    "canonization_level": "SAINT",
    "name": "Francis Xavier",
    "titles": ["PRIEST"],
    "date_of_death": 1552
  }
}
```

### Blessed

```json
{
  "frederic_ozanam_founder": {
    "fullname": "Blessed Frédéric Ozanam, Founder",
    "canonization_level": "BLESSED",
    "name": "Frédéric Ozanam",
    "date_of_birth": "1813-4-23",
    "date_of_death": "1853-9-8"
  }
}
```

### Group of Martyrs

```json
{
  "first_martyrs_of_the_holy_roman_church": {
    "fullname": "The First Martyrs of the Holy Roman Church",
    "name": "First Martyrs of the Holy Roman Church",
    "titles": ["martyr"],
    "hide_titles": true,
    "count": "many",
    "date_of_death": 64
  }
}
```

### Non-Person Entity

```json
{
  "finding_of_the_holy_cross": {
    "fullname": "Finding of the Holy Cross"
  }
}
```

## Adding a Translation

To add a French translation for an entity:

1. Find or create `data/resources/fr/entities.f.json`

2. Add the entity:

```json
{
  "francis_of_assisi": {
    "fullname": "Saint François d'Assise",
    "canonization_level": "SAINT",
    "name": "François d'Assise"
  }
}
```

3. Validate: `romcal validate resources "data/resources/fr/**/*.json"`

## ID Naming Conventions

Entity IDs follow these conventions:

- Use `snake_case`
- Include location for disambiguation: `francis_of_assisi`, `francis_of_paola`
- Include role suffix when needed: `fabian_i_pope`, `gregory_vii_pope`
- Use descriptive suffixes: `_priest`, `_bishop`, `_virgin`, `_martyr`

See [Naming Conventions](./naming-conventions) for complete guidelines.
