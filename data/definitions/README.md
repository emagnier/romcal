# Calendar Definitions

This document provides information about calendar definitions in the romcal project, including naming conventions and other relevant details.

## Directory Structure

The `definitions` directory is organized into four main subdirectories:

- **`general_roman/`** - General Roman Calendar
- **`regions/`** - Regional calendars (continents, cultural regions)
- **`countries/`** - National and diocesan calendars
- **`communities/`** - Religious communities and orders

## Naming Conventions

### Hierarchy Structure

The calendar system follows a hierarchical structure:

```
Region → Country → Diocese → City/Parish
```

**Important**: For convenience and to avoid overly long names, the region is typically omitted from country names unless different calendars exist in multiple regions (e.g., Russia has both `asia__russia` and `europe__russia`).

Examples:

- `france` (Country - no region prefix needed)
- `france__strasbourg` (Diocese within country)
- `france__strasbourg__city` (City within diocese)
- `russia` (Country with shared definitions between regions)
- `asia__russia` (Country with region prefix due to multiple regions)
- `europe__russia` (Country with region prefix due to multiple regions)

### Naming Rules

### 1. Directory Names

**Format**: `snake_case` for single-level directories, subdirectories for hierarchy

**Examples**:

```
countries/
├── france/
│   ├── france.json
│   ├── france__strasbourg.json
│   ├── france__strasbourg__city.json
│   ├── france__paris.json
│   └── france__saint_denis.json
├── russia/
│   ├── russia.json
│   ├── asia__russia.json
│   └── europe__russia.json
└── united_states/
    ├── united_states.json
    └── united_states__new_york.json

regions/
├── europe.json
└── americas.json

communities/
├── benedictine/
│   ├── benedictine.json
│   └── benedictine__monte_cassino.json
└── franciscan/
    ├── franciscan.json
    └── franciscan__assisi.json
```

**Rules**:

- Use `snake_case` for all parts
- Use subdirectories to represent hierarchy (not double underscores)
- Keep names concise but descriptive
- Avoid special characters except underscores
- **Entity naming**: Use the actual name of the entity (country, region) without specifying its type
- **Consistency**: Directory names should match the base calendar ID they contain

### 2. Calendar IDs

**Format**: `snake_case` with double underscores (`__`) to separate hierarchy levels

**Examples**:

```
france
france__strasbourg
france__strasbourg__city
france__paris
france__saint_denis
russia
asia__russia
europe__russia
```

**Rules**:

- Use `snake_case` for all parts
- Use double underscores (`__`) to separate hierarchy levels
- Keep names concise but descriptive
- Avoid special characters except underscores
- **Entity naming**: Use the actual name of the entity (country, diocese) without specifying its type (e.g., use `strasbourg` not `archdiocese-of-strasbourg`)
- **Region prefix**: Only include region prefix when multiple calendars exist in different regions (e.g., `asia__russia` vs `europe__russia`)
- **Shared definitions**: When a country spans multiple regions, create a base calendar (e.g., `russia`) containing shared definitions, and specific regional calendars (e.g., `asia__russia`, `europe__russia`) that inherit from it

### 2.1. Calendar Metadata

**Format**: Include a `metadata` object to specify entity type and additional information

**Examples**:

```json
{
  "id": "france__lyon",
  "metadata": {
    "type": "ARCHDIOCESE",
    "jurisdiction": "ECCLESIASTICAL"
  },
  "parentCalendarIds": ["Europe", "France"],
  "inputs": { ... }
}
```

```json
{
  "id": "france__strasbourg",
  "metadata": {
    "type": "DIOCESE",
    "jurisdiction": "ECCLESIASTICAL"
  },
  "parentCalendarIds": ["Europe", "France"],
  "inputs": { ... }
}
```

**Metadata Fields**:

- **`type`** (required): Entity type (`ARCHDIOCESE`, `DIOCESE`, `COUNTRY`, `REGION`, `COMMUNITY`, etc.) - Use uppercase constants
- **`jurisdiction`** (optional): Type of jurisdiction (`ECCLESIASTICAL`, `CIVIL`, etc.) - Use uppercase constants

### 3. JSON Files

**Format**: Same as calendar ID with `.json` extension

**Examples**:

```
france.json
france__strasbourg.json
france__strasbourg__city.json
france__paris.json
france__saint_denis.json
russia.json
asia__russia.json
europe__russia.json
```

**Base Calendars**: Each directory contains a main file with the entity name (country, region, community) that contains the base definitions:

- **Countries**: `france.json`, `russia.json`, `united_states.json`
- **Regions**: `europe.json`, `americas.json`
- **Communities**: `benedictine.json`, `franciscan.json`

**Multi-regional Calendars**: For countries that span multiple regions, the base file contains shared definitions, and region-specific files are created:

- `russia.json` → shared definitions between regions
- `asia__russia.json` → specific to Asian region
- `europe__russia.json` → specific to European region

### 4. TypeScript Variables

**Format**: `PascalCase` without underscores

**Examples**:

```typescript
const France = { ... };
const FranceStrasbourg = { ... };
const FranceStrasbourgCity = { ... };
const FranceParis = { ... };
const FranceSaintDenis = { ... };
const Russia = { ... };
const AsiaRussia = { ... };
const EuropeRussia = { ... };
```

**Rules**:

- Convert snake_case ID to PascalCase
- Remove all underscores
- Follow standard TypeScript naming conventions

### 5. NPM Packages

**Format**: `@romcal/calendar.{hierarchy}` with dots (`.`) as separators

**Examples**:

```
@romcal/calendar.france
@romcal/calendar.france.strasbourg
@romcal/calendar.france.strasbourg.city
@romcal/calendar.france.paris
@romcal/calendar.france.saint-denis
@romcal/calendar.russia
@romcal/calendar.asia-russia
@romcal/calendar.europe-russia
```

**Rules**:

- Use dots (`.`) instead of double underscores (`__`) to separate hierarchy levels
- Preserve hyphens (`-`) in entity names (e.g., `saint-denis` remains `saint-denis`)

### Complete Examples

#### Country Calendar

```
ID: france
File: france.json
Variable: France
Package: @romcal/calendar.france
```

#### Diocese Calendar

```
ID: france__strasbourg
File: france__strasbourg.json
Variable: FranceStrasbourg
Package: @romcal/calendar.france.strasbourg
Metadata: { "type": "DIOCESE" }
```

```
ID: france__saint_denis
File: france__saint_denis.json
Variable: FranceSaintDenis
Package: @romcal/calendar.france.saint-denis
Metadata: { "type": "DIOCESE" }
```

#### Archdiocese Calendar

```
ID: france__lyon
File: france__lyon.json
Variable: FranceLyon
Package: @romcal/calendar.france.lyon
Metadata: { "type": "ARCHDIOCESE" }
```

```
ID: france__paris
File: france__paris.json
Variable: FranceParis
Package: @romcal/calendar.france.paris
Metadata: { "type": "ARCHDIOCESE" }
```

#### Multi-regional Country Calendar

Countries particularly concerned are Russia (Asia/Europe), Turkey (Asia/Europe), and Egypt (Africa/Asia)

**Base Calendar (shared definitions)**:

```
ID: russia
File: russia.json
Variable: Russia
Package: @romcal/calendar.russia
```

**Regional Calendar**:

```
ID: asia__russia
File: asia__russia.json
Variable: AsiaRussia
Package: @romcal/calendar.asia-russia
```

#### City Calendar (within diocese)

```
ID: france__strasbourg__city
File: france__strasbourg__city.json
Variable: FranceStrasbourgCity
Package: @romcal/calendar.france.strasbourg.city
```

```
ID: france__saint_denis__city
File: france__saint_denis__city.json
Variable: FranceSaintDenisCity
Package: @romcal/calendar.france.saint-denis.city
```

**Note**: The `_city` suffix refers to the main city of the diocese where the bishop's seat is located (i.e., the city that bears the same name as the diocese).

### Consistency Guidelines

1. **Directory names**: Always use `snake_case` with subdirectories for hierarchy
2. **IDs and files**: Always use `snake_case` with double underscores for hierarchy
3. **TypeScript variables**: Always use `PascalCase` without underscores
4. **NPM packages**: Always use dots for hierarchy separation
5. **File naming**: Match the calendar ID exactly

### Best Practices

- Keep hierarchy levels to a maximum of 3-4 levels for readability
- Use descriptive but concise names
- Ensure uniqueness across the entire calendar system
