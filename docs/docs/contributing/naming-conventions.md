---
sidebar_position: 4
---

# Naming Conventions

Consistent naming conventions ensure the project remains organized and searchable.

## Calendar IDs

Calendar IDs use `snake_case` with double underscores (`__`) to separate hierarchy levels.

### Format

```
[region__]country[__subdivision][__city]
```

### Examples

| ID                         | Description                           |
| -------------------------- | ------------------------------------- |
| `france`                   | French national calendar              |
| `france__strasbourg`       | Diocese of Strasbourg                 |
| `france__strasbourg__city` | City of Strasbourg                    |
| `france__paris`            | Archdiocese of Paris                  |
| `russia`                   | Russian calendar (shared definitions) |
| `asia__russia`             | Russia in Asia region                 |
| `europe__russia`           | Russia in Europe region               |

### Rules

1. Use `snake_case` for all parts
2. Use `__` (double underscore) to separate hierarchy levels
3. Use the entity name, not its type (e.g., `strasbourg` not `archdiocese_of_strasbourg`)
4. Only include region prefix when different calendars exist in multiple regions

## Entity IDs

Entity IDs use `snake_case` with descriptive suffixes.

### Format

```
name[_of_location][_role]
```

### Examples

| ID                                       | Description                      |
| ---------------------------------------- | -------------------------------- |
| `francis_of_assisi`                      | Saint Francis of Assisi          |
| `francis_of_paola_hermit`                | Saint Francis of Paola, Hermit   |
| `fabian_i_pope`                          | Saint Fabian, Pope               |
| `frances_of_rome_religious`              | Saint Frances of Rome, Religious |
| `first_martyrs_of_the_holy_roman_church` | Group celebration                |
| `finding_of_the_holy_cross`              | Non-person feast                 |

### Rules

1. Use `snake_case`
2. Use English names (translations go in resources)
3. Include location for disambiguation: `john_of_the_cross`, `john_of_god`
4. Include role suffix when commonly used: `_pope`, `_bishop`, `_priest`, `_virgin`, `_martyr`
5. Use Roman numerals for popes: `gregory_vii_pope`, `john_xxiii_pope`

## File Names

### Definitions

File names match the calendar ID:

```
france.json
france__strasbourg.json
france__strasbourg__city.json
```

### Resources

Resources are organized alphabetically by first letter:

```
entities.a.json
entities.b.json
entities.f.json  # Contains francis_of_assisi, frances_of_rome, etc.
```

## TypeScript/JavaScript Variables

Convert IDs to PascalCase:

| ID                         | Variable               |
| -------------------------- | ---------------------- |
| `france`                   | `France`               |
| `france__strasbourg`       | `FranceStrasbourg`     |
| `france__strasbourg__city` | `FranceStrasbourgCity` |
| `asia__russia`             | `AsiaRussia`           |

## NPM Package Names

Use dots instead of double underscores:

| ID                    | Package                               |
| --------------------- | ------------------------------------- |
| `france`              | `@romcal/calendar.france`             |
| `france__strasbourg`  | `@romcal/calendar.france.strasbourg`  |
| `france__saint_denis` | `@romcal/calendar.france.saint-denis` |

## Summary Table

| Context     | Format              | Example                         |
| ----------- | ------------------- | ------------------------------- |
| Calendar ID | `snake_case` + `__` | `france__paris`                 |
| Entity ID   | `snake_case`        | `francis_of_assisi`             |
| File name   | Match ID + `.json`  | `france__paris.json`            |
| TypeScript  | PascalCase          | `FranceParis`                   |
| npm package | Dots                | `@romcal/calendar.france.paris` |
