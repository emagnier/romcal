---
title: Public API
description: Interface reference for romcal's public API — configuration, calendar generation, lookup methods, CLI commands, and bindings surface.
tableOfContents:
  minHeadingLevel: 2
  maxHeadingLevel: 4
---

<!-- AI Quick Index — concept/method → section (line numbers are approximate)

CONFIGURATION
| Concept                              | Section              | Line   | Notes                             |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| Config (all fields)                  | Part I §1            | ~107   | All-optional config struct        |
| YearFrame NOT in Config              | Part I §1            | ~132   | Belongs at method level           |
| Romcal::new()                        | Part I §2            | ~136   | Validates + resolves defaults     |
| Romcal::empty()                      | Part I §2            | ~152   | Bare instance, no data loaded     |
| Romcal::new() errors                 | Part I §2            | ~160   | CalendarNotFound, LocaleNotFound  |

CALENDAR GENERATION
| Concept                              | Section              | Line   | Notes                             |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| generate_liturgical_calendar()       | Part II §3           | ~174   | Layer 1 output                    |
| generate_mass_calendar()             | Part II §4           | ~197   | Layer 2 Mass output               |
| generate_hours_calendar()            | Part II §5           | ~215   | Layer 2 Hours output              |
| YearFrame semantics                  | Part II §3           | ~190   | Gregorian vs Liturgical range     |
| Year parameter semantics             | Part II §6           | ~233   | Civil year interpretation         |

LOOKUPS
| Concept                              | Section              | Line   | Notes                             |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| date_of()                            | Part III §7          | ~259   | Date by celebration ID + year     |
| liturgical_day_of()                  | Part III §7          | ~267   | Layer 1 by celebration ID + year  |
| masses_of()                          | Part III §7          | ~275   | Layer 2 Mass by ID + year         |
| hours_of()                           | Part III §7          | ~283   | Layer 2 Hours by ID + year        |
| get_martyrology_entry()              | Part III §8          | ~298   | Exact ID lookup                   |
| search_martyrology()                 | Part III §8          | ~312   | Fuzzy search with filters         |

DATA MANAGEMENT
| Concept                              | Section              | Line   | Notes                             |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| add_calendar_definition()            | Part IV §9           | ~347   | Progressive loading               |
| add_resources()                      | Part IV §9           | ~355   | Merge semantics                   |
| create_bundle()                      | Part IV §10          | ~375   | Optimized export                  |

CLI
| Concept                              | Section              | Line   | Notes                             |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| Command table                        | Part V §11           | ~407   | 15 commands mapped to API         |
| Global options table                 | Part V §12           | ~447   | Flags mapped to Config fields     |

BINDINGS
| Concept                              | Section              | Line   | Notes                             |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| Architecture diagram                 | Part VI §13          | ~470   | Single C ABI FFI surface          |
| TypeScript/WASM binding              | Part VI §14          | ~495   | Primary non-Rust binding          |
| Community bindings                   | Part VI §15          | ~505   | Native dynamic library            |
| JSON Schemas                         | Part VI §16          | ~516   | Type generation for any language   |

-->

## Context and Motivation

This document is one of four companion architecture documents. Together, they describe romcal's data pipeline end-to-end:

- [**Input Data Model**](./input-data-model.md) defines what goes **in** — the three-tier structure of calendar definitions, martyrology catalog, and liturgical texts that contributors edit.
- **This document** (Public API) defines **how** to drive the engine — the interface through which consumers configure romcal, generate calendars, and retrieve results.
- [**Calendar Resolution Algorithm**](./calendar-resolution.md) defines the **transformation pipeline** — how the engine turns input definitions into a resolved liturgical calendar.
- [**Liturgical Composition Model**](./liturgical-composition-model.md) defines what comes **out** — the output types and composition rules for the liturgical calendar, Mass calendar, and Hours calendar.

While the input model is designed for human editors and the composition model for type consumers, this document is designed for **application developers**: it specifies every configuration option, method signature, return type, CLI command, and bindings surface needed to integrate romcal.

### Scope

This document covers the public interface only — it does not describe how the engine resolves calendars internally (see the Calendar Resolution Algorithm for the full pipeline, and the Composition Model's Part V for the transformation architecture), nor the input format specifications (see the Input Data Model).

The four documents are complementary and share cross-references. Output types mentioned here (`LiturgicalCalendar`, `MassCalendar`, `HoursCalendar`, `MassComposition`, `HoursComposition`, etc.) are defined in the Composition Model. Input types mentioned here (`CalendarDefinition`, `Resources`, `MartyrologyEntryDef`, etc.) are defined in the Input Data Model.

### Workspace Structure

The romcal codebase is organized as a Cargo workspace:

| Crate          | Path             | Role                                                       |
| -------------- | ---------------- | ---------------------------------------------------------- |
| `romcal`       | `core/`          | Core library — configuration, engine, types, C ABI exports |
| `romcal-cli`   | `cli/`           | Command-line interface                                     |
| `romcal-wasm`  | `bindings/wasm/` | WASM compilation + TypeScript wrapper                      |
| `romcal-tools` | `tools/`         | Build-time tooling (JSON Schema generation, etc.)          |

The C ABI surface defined in `core/` can also be compiled as a native dynamic library for community bindings (see §15) — this does not require a separate crate.

### Method Naming Conventions

Public methods follow three naming patterns, each signaling a different kind of operation:

| Pattern | Methods                                                             | Signature shape | Semantics                                                                                                                                                                                                            |
| ------- | ------------------------------------------------------------------- | --------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `*_of`  | `date_of`, `liturgical_day_of`, `masses_of`, `hours_of`             | `(id, year)`    | **Resolution** — takes a celebration ID and a year, computes and returns the result for that celebration. The `_of` suffix signals that the engine resolves the celebration to its concrete data for the given year. |
| `get_*` | `get_martyrology_entry`, `get_calendar_definition`, `get_resources` | `(key)`         | **Direct lookup** — takes an exact key (ID or locale) and returns the matching entry from loaded data. The `get_` prefix follows the Rust convention for straightforward accessors.                                  |
| verb    | `generate_*`, `search_martyrology`, `add_*`, `create_bundle`        | varies          | **Action** — performs a named operation (generate a calendar, search entries, add data, create a bundle). The verb makes the side effect or computation explicit.                                                    |

The pattern is self-documenting: when a developer sees `_of`, they know the method resolves a celebration for a year; when they see `get_`, they know it is a direct lookup by key; when they see a verb, they know it triggers an action.

---

## Part I — Configuration

### 1. `Config` — Configuration Object

`Config` is the all-optional configuration struct passed to `Romcal::new()`. Every field has a default value; omitting all fields produces a valid configuration for the General Roman Calendar in English.

> **Why `Config`.** The name follows the standard Rust convention for configuration objects (`ClientConfig`, `ServerConfig`, etc.). The same `Config` struct serves both interactive use (the developer fills a few fields, defaults handle the rest) and bundle loading (a bundle deserializes into a `Config` with all fields populated). A bundle is a serialized, optimized `Config` — same type, different packaging. See §10 for the `create_bundle()` method.

| Field                      | Type                              | Default           | Purpose                                                                                                                   |
| -------------------------- | --------------------------------- | ----------------- | ------------------------------------------------------------------------------------------------------------------------- |
| `calendar_id`              | `Option<CalendarId>`              | `"general_roman"` | Calendar to compute (e.g., `"france"`, `"united_states"`). See note on calendar hierarchy below.                          |
| `locale`                   | `Option<LocaleTag>`               | `"en"`            | BCP 47 locale tag for localized output (case-insensitive)                                                                 |
| `easter_calculation`       | `Option<EasterCalculation>`       | `Gregorian`       | Easter algorithm (`Gregorian` or `Julian`)                                                                                |
| `epiphany_on_sunday`       | `Option<bool>`                    | `false`           | Move Epiphany to the Sunday between January 2–8                                                                           |
| `ascension_on_sunday`      | `Option<bool>`                    | `false`           | Move Ascension to the 7th Sunday of Easter                                                                                |
| `corpus_christi_on_sunday` | `Option<bool>`                    | `false`           | Move Corpus Christi to Sunday                                                                                             |
| `ordinal_format`           | `Option<OrdinalFormat>`           | locale-dependent  | Ordinal number formatting (`Numeric` or `Letters`). When not set, resolved from locale metadata; falls back to `Numeric`. |
| `calendar_definitions`     | `Option<Vec<CalendarDefinition>>` | bundled           | Calendar definition data. `None` = use bundled data; `Some(vec)` = use exactly the provided definitions.                  |
| `resources`                | `Option<Vec<Resources>>`          | bundled           | Resource data (martyrology, locale strings). `None` = use bundled data; `Some(vec)` = use exactly the provided resources. |

**Calendar hierarchy:** All calendars ultimately inherit from `"temporal_cycle"` — the root calendar that contains only the Proper of Time (Advent, Christmas, Lent, Easter, Ordinary Time) with no sanctoral celebrations. `"general_roman"` extends `"temporal_cycle"` by adding the General Roman sanctoral (universal saints, feasts, solemnities). Particular calendars (e.g., `"france"`, `"united_states"`) extend `"general_roman"` by adding or overriding celebrations. See [Input Data Model — Part II §1](./input-data-model.md) for how calendar inheritance works.

**Field naming conventions:**

- `calendar_id` uses the `_id` suffix to distinguish it from output types (`LiturgicalCalendar`, `MassCalendar`) and from `CalendarDefinition`. The type `CalendarId` (a newtype wrapping `String`) provides additional type safety — see [Input Data Model — Part II §1](./input-data-model.md) for its definition.
- `locale` keeps its standard name — a locale IS an identifier by nature, and this name is the universal convention across libraries (ICU, BCP 47, Java `Locale`, JavaScript `navigator.language`). The type `LocaleTag` (a newtype wrapping `String`) provides the type safety.

> **`YearFrame` is not in `Config`.** The `YearFrame` parameter (Gregorian vs. Liturgical year framing) defines the output date range, not the engine configuration. It belongs at the method level — see §3–5.

### 2. `Romcal` — Engine Instance

**`Romcal::new(config: Config) -> Result<Romcal, RomcalError>`**

Creates a new engine instance. Validates the configuration, resolves defaults for all `None` fields, loads calendar definitions and resources. The returned instance is ready for calendar generation.

Validation rules:

- **Calendar validation**: if `calendar_definitions` is `Some(...)` (the developer explicitly provided definitions, even an empty `Vec`) and the requested `calendar_id` is not `"temporal_cycle"`, the calendar ID must exist in the provided definitions. Failure returns `RomcalError::CalendarNotFound(id, available)`. Passing `Some(vec![])` with `calendar_id = "general_roman"` is therefore an error — the developer said "use exactly these definitions" but they contain nothing.
- **Locale validation**: if `resources` is `Some(...)` (same logic), the requested locale (or its base locale, e.g., `"fr"` for `"fr-ca"`) must exist in the provided resources. Failure returns `RomcalError::LocaleNotFound(locale, available)`. There is no implicit fallback to `"en"` — an unknown locale is an error.
- **Locale normalization**: BCP 47 tags are normalized to lowercase (e.g., `"FR-CA"` → `"fr-ca"`). A locale value that is not a valid BCP 47 tag returns `InvalidConfig("invalid BCP 47 locale tag: ...")`.

**Ordinal format resolution** follows a priority chain:

1. Value from `Config` (highest priority)
2. Value from the target locale's `ResourcesMetadata`
3. `OrdinalFormat::Numeric` (default)

**`Romcal::empty() -> Romcal`**

Creates a bare instance with no calendar data loaded. Uses `"temporal_cycle"` as the calendar and all other fields at their defaults. Intended for progressive loading via `add_calendar_definition()` and `add_resources()` (see §9).

The calendar is set to `"temporal_cycle"` (not `"general_roman"`) because `empty()` starts with no calendar definitions loaded — and `"general_roman"` requires its sanctoral definitions to be present. `"temporal_cycle"`, as the root of the hierarchy, requires no external definitions: the engine can compute the Proper of Time from its internal rules alone. The developer then loads definitions incrementally to build up to the desired calendar.

**Instance lifecycle:** After construction (via `new` or `empty`), the instance can be mutated through the data management methods (§9) to load additional definitions and resources. All generation methods (§3–5), lookup methods (§7–8), accessors (§9), and `create_bundle` (§10) take `&self` — only the progressive loading methods `add_calendar_definition` and `add_resources` take `&mut self`.

**Errors returned by `Romcal::new()`:**

| Error                               | When it occurs                                                                                                                                                                                                                                                                                             |
| ----------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CalendarNotFound(id, available)`   | `calendar_definitions` is provided but the requested `calendar_id` does not match any definition. `available` lists the IDs that were found.                                                                                                                                                               |
| `LocaleNotFound(locale, available)` | `resources` is provided but the requested locale (or its base locale) does not match any resource. `available` lists the locales that were found.                                                                                                                                                          |
| `InvalidConfig(reason)`             | The configuration is internally contradictory. Examples: `epiphany_on_sunday` is `true` but the loaded calendar already fixes Epiphany to January 6; a calendar definition references a parent that is not present in the provided `calendar_definitions`. `reason` contains a human-readable explanation. |

---

## Part II — Calendar Generation

All generation methods take `&self`, a `year` parameter, and an optional `year_frame` parameter (`YearFrame`, default `Gregorian`) that determines the output date range. They return a `Result` wrapping the output type.

### 3. `generate_liturgical_calendar`

```rust
fn generate_liturgical_calendar(
    &self,
    year: i32,
    year_frame: YearFrame,  // default: Gregorian
) -> Result<LiturgicalCalendar, RomcalError>
```

**Layer 1 output.** Computes the liturgical days for a year, organized by liturgical date.

Returns `LiturgicalCalendar` — a newtype wrapping a map from date strings (`YYYY-MM-DD`) to vectors of `LiturgicalDay` objects. See [Composition Model — Part IV §3](./liturgical-composition-model.md#3-layer-1--liturgical-calendar) for the type definition.

Returns `RomcalError::YearOutOfRange(year, min, max)` if the year falls outside the valid range (see §6).

The `year_frame` parameter determines the date range:

| YearFrame             | Date range                                                                      |
| --------------------- | ------------------------------------------------------------------------------- |
| `Gregorian` (default) | January 1 to December 31 of the civil year                                      |
| `Liturgical`          | First Sunday of Advent (year − 1) to the Saturday before the next Advent (year) |

### 4. `generate_mass_calendar`

```rust
fn generate_mass_calendar(
    &self,
    year: i32,
    year_frame: YearFrame,  // default: Gregorian
) -> Result<MassCalendar, RomcalError>
```

**Layer 2 Mass output.** Computes the Mass calendar for a year, organized by civil date and mass time. Evening masses (Easter Vigil, previous evening masses) appear on the previous civil day.

Returns `MassCalendar` — a newtype wrapping a map from civil date strings (`YYYY-MM-DD`) to vectors of `MassComposition` objects. See [Composition Model — Part IV §4](./liturgical-composition-model.md#4-layer-2-mass--mass-calendar) for the type definition.

Includes Layer 1 as its foundation — the engine first produces the `LiturgicalCalendar`, then transforms it into the `MassCalendar`.

Same `year_frame` semantics and errors as §3.

### 5. `generate_hours_calendar`

```rust
fn generate_hours_calendar(
    &self,
    year: i32,
    year_frame: YearFrame,  // default: Gregorian
) -> Result<HoursCalendar, RomcalError>
```

**Layer 2 Hours output.** Computes the Liturgy of the Hours for a year, organized by civil date.

Returns `HoursCalendar` — a newtype wrapping a map from civil date strings to vectors of `HoursComposition` objects. See [Composition Model — Part IV §5](./liturgical-composition-model.md#5-layer-2-hours--hours-calendar) for the type definition.

Includes Layer 1 as its foundation — same transformation pattern as the Mass calendar.

Same `year_frame` semantics and errors as §3.

### 6. Year Parameter Semantics

The `year` parameter is a civil year (e.g., `2025`).

- When `year_frame` is `Gregorian`: the year maps directly to January 1 – December 31 of that civil year.
- When `year_frame` is `Liturgical`: the year identifies the liturgical year whose **greater part** falls in that civil year. For example, `2025` covers the liturgical year from the first Sunday of Advent 2024 (late November/early December 2024) through the Saturday before the first Sunday of Advent 2025.

Liturgical year boundaries are computed internally by the engine.

**Valid year range** depends on the Easter calculation configured in `Config`:

| `EasterCalculation` | Min year | Max year | Reason                                    |
| ------------------- | -------- | -------- | ----------------------------------------- |
| `Gregorian`         | 1583     | 9999     | First full year of the Gregorian calendar |
| `Julian`            | 326      | 9999     | First Council of Nicaea (325 AD)          |

Passing a year outside the valid range returns `RomcalError::YearOutOfRange(year, min, max)`.

---

## Part III — Lookup Methods

### 7. Celebration Lookups

Four methods resolve a celebration by its ID and year, each returning a different level of detail. The `id` parameter is a `CelebrationId` — a `snake_case` string such as `"easter_sunday"`, `"peter_and_paul_apostles"`, or `"ordinary_time_5_monday"`. See [Input Data Model — Part II §1](./input-data-model.md) for the naming convention.

**`date_of`**

```rust
fn date_of(&self, id: &str, year: i32) -> Result<String, RomcalError>
```

Returns the civil date (`YYYY-MM-DD`) on which the celebration falls. Always succeeds for a valid ID and year — every celebration has a computable date regardless of whether it is actually celebrated (impeded celebrations still have a date).

**`liturgical_day_of`**

```rust
fn liturgical_day_of(&self, id: &str, year: i32) -> Result<LiturgicalDay, RomcalError>
```

Returns the full `LiturgicalDay` for the celebration as it resolves in the actual calendar. If the celebration is impeded by a higher-precedence celebration, the `LiturgicalDay` reflects its effective status (e.g., reduced to a commemoration). See [Composition Model — Part IV §3](./liturgical-composition-model.md#3-layer-1--liturgical-calendar) for the type definition.

**`masses_of`**

```rust
fn masses_of(&self, id: &str, year: i32) -> Result<Vec<MassComposition>, RomcalError>
```

Returns the Mass composition(s) for the celebration. A celebration may have multiple Mass formularies (e.g., Christmas: Vigil, Night, Dawn, Day). Returns an empty `Vec` if the celebration is fully impeded in the given year and has no Mass available (e.g., an optional memorial falling on a solemnity). See [Composition Model — Part IV §4](./liturgical-composition-model.md#4-layer-2-mass--mass-calendar) for the type definition.

**`hours_of`**

```rust
fn hours_of(&self, id: &str, year: i32) -> Result<Vec<HoursComposition>, RomcalError>
```

Returns the Hours composition(s) for the celebration. Same impeded-celebration semantics as `masses_of` — returns an empty `Vec` when no Hours are available. See [Composition Model — Part IV §5](./liturgical-composition-model.md#5-layer-2-hours--hours-calendar) for the type definition.

**Errors:**

| Error                            | When it occurs                                                                                                                                                                                                                                                 |
| -------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `CelebrationNotFound(id)`        | The `id` does not match any celebration known to the loaded calendar. Possible causes: typo in the ID, or the celebration belongs to a particular calendar that is not loaded (e.g., looking up `"our_lady_of_guadalupe"` with only `"general_roman"` loaded). |
| `YearOutOfRange(year, min, max)` | The `year` falls outside the valid range for the configured `EasterCalculation` — see §6 for the exact bounds.                                                                                                                                                 |

### 8. Martyrology Access

**`get_martyrology_entry`**

```rust
fn get_martyrology_entry(&self, id: &str) -> Option<MartyrologyEntry>
```

Retrieves a single martyrology entry by its exact ID (e.g., `"francis_of_assisi"`). Locale resolution merges entries from `en` (base), through parent locale, to the specific locale — the most specific locale's values take precedence.

Returns `None` if no entry matches the ID after all locale fallbacks. A missing entry is not an error — it is an expected outcome for `get_*` lookups (see [Method Naming Conventions](#method-naming-conventions)).

See [Input Data Model — Part III §3](./input-data-model.md) for the `MartyrologyEntryDef` input type.

**`search_martyrology`**

```rust
fn search_martyrology(&self, query: MartyrologyQuery) -> Vec<MartyrologySearchResult>
```

Searches martyrology entries with fuzzy matching and structured filters. Searches entries merged from all locales in the fallback chain. Returns an empty `Vec` if no entries match — a search with no results is not an error.

**`MartyrologyQuery` fields** (all optional):

| Field                | Type                           | Purpose                                         |
| -------------------- | ------------------------------ | ----------------------------------------------- |
| `text`               | `Option<String>`               | Fuzzy text search on `id`, `name`, `fullname`   |
| `entry_type`         | `Option<MartyrologyEntryType>` | Filter by type (`Person`, `Place`, `Event`)     |
| `canonization_level` | `Option<CanonizationLevel>`    | Filter by level (`Saint`, `Blessed`, etc.)      |
| `sex`                | `Option<Sex>`                  | Filter by sex (`Male`, `Female`)                |
| `titles`             | `Option<Vec<Title>>`           | Filter by titles (entry must have at least one) |

Returns a `Vec<MartyrologySearchResult>` sorted by score (highest first). Each result contains:

| Field            | Type               | Purpose                                   |
| ---------------- | ------------------ | ----------------------------------------- |
| `entry`          | `MartyrologyEntry` | The matched martyrology entry             |
| `score`          | `f64`              | Match score from 0.0 to 1.0               |
| `match_type`     | `MatchType`        | Type of match (exact, fuzzy, filter-only) |
| `matched_fields` | `Vec<String>`      | Names of fields that matched              |

---

## Part IV — Data Management

### 9. Mutators (Progressive Loading)

These methods mutate the `Romcal` instance and are intended for progressive loading when starting from `Romcal::empty()`. They can be called at any point during the instance's lifetime — the engine does not cache generation results, so subsequent calls to generation or lookup methods will use the updated data.

**`add_calendar_definition`**

```rust
fn add_calendar_definition(&mut self, calendar_def: CalendarDefinition)
```

Appends a calendar definition to the instance. Used with `Romcal::empty()` to load definitions one at a time.

**`add_resources`**

```rust
fn add_resources(&mut self, resources: Resources)
```

Adds resource data to the instance. If resources for the same locale already exist, the new entries are **merged** into the existing resource:

- Martyrology entries are added or replaced (by ID).
- Metadata is overwritten when provided.

This prevents duplicate locale entries and allows incremental loading of resource files.

**Accessors:**

| Method                        | Returns                       | Purpose                                    |
| ----------------------------- | ----------------------------- | ------------------------------------------ |
| `get_calendar_definition(id)` | `Option<&CalendarDefinition>` | Look up a loaded calendar definition by ID |
| `get_resources(locale)`       | `Option<&Resources>`          | Look up loaded resources by locale         |

### 10. Bundle Export

```rust
fn create_bundle(&self) -> Result<String, RomcalError>
```

Serializes the current configuration into an optimized JSON bundle suitable for distribution or caching. The bundle contains:

- Only calendar definitions in the hierarchy (`general_roman` → parents → target calendar).
- Only resources for locales in the fallback chain (`en` → parent → specific).
- Property-level deduplication across the locale hierarchy.
- No null values or empty objects.

Returns a pretty-printed JSON string. The output deserializes back into a `Config` — a bundle is a complete, pre-resolved `Config` ready to be passed to `Romcal::new()`.

**Errors:**

| Error                               | When it occurs                                                                                                                                  |
| ----------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| `CalendarNotFound(id, available)`   | The configured calendar hierarchy cannot be resolved from the loaded definitions (e.g., a parent is missing).                                   |
| `LocaleNotFound(locale, available)` | The locale fallback chain cannot be resolved from the loaded resources (e.g., `"fr-ca"` requested but neither `"fr-ca"` nor `"fr"` is present). |

---

## Part V — CLI

The `romcal` CLI provides access to all core API methods via shell commands. It reads configuration from three sources with descending priority:

1. **CLI flags** (highest priority)
2. **Config file** (`.romcal.toml` or `~/.config/romcal/config.toml`)
3. **Defaults** (lowest priority)

### 11. Commands

| Command                        | API equivalent                                   | Description                        |
| ------------------------------ | ------------------------------------------------ | ---------------------------------- |
| `calendar [year]`              | `generate_liturgical_calendar(year, year_frame)` | Generate liturgical calendar       |
| `masses [year]`                | `generate_mass_calendar(year, year_frame)`       | Generate Mass calendar             |
| `date <id> [year]`             | `date_of(id, year)`                              | Calculate a liturgical date        |
| `day <id> [year]`              | `liturgical_day_of(id, year)`                    | Get liturgical day for celebration |
| `mass <id> [year]`             | `masses_of(id, year)`                            | Get Mass composition(s)            |
| `hours <id> [year]`            | `hours_of(id, year)`                             | Get Hours composition(s)           |
| `martyrology <id>`             | `get_martyrology_entry(id)`                      | Get a martyrology entry by ID      |
| `search [text]`                | `search_martyrology(query)`                      | Search martyrology entries         |
| `list calendars`               | —                                                | List available calendars           |
| `list locales`                 | —                                                | List available locales             |
| `config`                       | —                                                | Display resolved configuration     |
| `bundle`                       | `create_bundle()`                                | Export optimized bundle            |
| `validate definitions <paths>` | —                                                | Validate calendar definition files |
| `validate resources <paths>`   | —                                                | Validate resource files            |
| `completions <shell>`          | —                                                | Generate shell completion scripts  |

**Command-specific options.** Options that apply to only a subset of commands are scoped to those commands rather than declared globally. This avoids silently ignored flags (e.g., `romcal list calendars --year-frame liturgical` would accept a flag that has no effect) and makes the `--help` output of each command self-documenting.

`calendar` and `masses` (generation commands):

| Flag               | Maps to                   | Notes                                                                   |
| ------------------ | ------------------------- | ----------------------------------------------------------------------- |
| `-t, --year-frame` | method `year_frame` param | `gregorian` (default) or `liturgical`                                   |
| `--filter`         | _(output)_                | Show only specific properties (supports dot notation for nested fields) |

`search`:

| Flag          | Maps to                               | Notes                                   |
| ------------- | ------------------------------------- | --------------------------------------- |
| `--type`      | `MartyrologyQuery.entry_type`         | Filter by type (`person`, `place`, …)   |
| `--sex`       | `MartyrologyQuery.sex`                | Filter by sex (`male`, `female`)        |
| `--level`     | `MartyrologyQuery.canonization_level` | Filter by level (`saint`, `blessed`, …) |
| `--title`     | `MartyrologyQuery.titles`             | Filter by title (repeatable)            |
| `--limit`     | _(output)_                            | Maximum number of results               |
| `--min-score` | _(output)_                            | Minimum match score (0.0–1.0)           |

### 12. Global Options

Global options apply to all commands that instantiate the engine (all commands except `list`, `completions`). They are global because they map to `Config` fields shared across the entire API — scoping them per-command would force users to remember which flags go where for no benefit, since any command that uses the engine accepts the same configuration.

| Flag                         | Maps to                           | Notes                                              |
| ---------------------------- | --------------------------------- | -------------------------------------------------- |
| `-c, --calendar`             | `Config.calendar_id`              | Calendar name (e.g., `france`, `united_states`)    |
| `-l, --locale`               | `Config.locale`                   | Locale code (e.g., `en`, `fr`, `es`)               |
| `-C, --config`               | _(file)_                          | Path to TOML config file                           |
| `-f, --format`               | _(output)_                        | Output format: `json`, `yaml`, `csv`, `lines`      |
| `-d, --definitions`          | `Config.calendar_definitions`     | Paths to external definition files (glob patterns) |
| `-r, --resources`            | `Config.resources`                | Paths to external resource files (glob patterns)   |
| `--replace`                  | _(mode)_                          | Use only custom data, ignore bundled data          |
| `--easter-calc`              | `Config.easter_calculation`       | `gregorian` or `julian`                            |
| `--epiphany-on-sunday`       | `Config.epiphany_on_sunday`       | Boolean flag                                       |
| `--ascension-on-sunday`      | `Config.ascension_on_sunday`      | Boolean flag                                       |
| `--corpus-christi-on-sunday` | `Config.corpus_christi_on_sunday` | Boolean flag                                       |
| `-D, --debug`                | _(logging)_                       | Enable debug output                                |

---

## Part VI — Bindings Surface

### 13. Architecture Overview

The Rust core exposes its API through multiple surfaces:

```
┌─────────────┐     ┌──────────────┐
│  Rust Core  │────▶│   CLI (Rust) │
│  (library)  │     └──────────────┘
│             │
│             │────▶┌──────────────┐
│             │     │  WASM / FFI  │
└─────────────┘     └──────┬───────┘
                           │
         ┌─────────────┬───┴───┬──────────┐
         │             │       │          │
  ┌──────▼──────┐  ┌──▼────┐  ┌▼─────┐  ┌▼────┐
  │ TypeScript   │  │Python │  │ Dart │  │ PHP │
  └─────────────┘  └───────┘  └──────┘  └─────┘
```

- The **CLI** links directly against the Rust core library.
- **Non-Rust bindings** go through a C-compatible FFI / WASM surface.
- Data exchange uses **JSON serialization** — language-agnostic and simple. Calendar-sized payloads make serialization overhead negligible.
- Bindings receive all inputs as JSON strings and return all outputs as JSON strings. When the input JSON cannot be parsed or does not conform to the expected schema, the FFI function returns `RomcalError::DeserializationError(reason)` with a human-readable explanation of what failed (missing field, wrong type, etc.). This error is specific to the bindings surface — it does not apply to the Rust library API, where inputs are typed at compile time.

### 14. TypeScript / WASM Binding

The primary non-Rust binding, maintained by the project.

- The core library is compiled to WASM via the `wasm32-unknown-unknown` target.
- A thin TypeScript wrapper provides a typed API around the WASM exports.
- The WASM binding mirrors the Rust API: configuration via `Config`, instance creation, and the same generation/lookup methods with camelCase naming.
- Types generated from JSON Schemas (see §16).
- Distributed as an npm package.

### 15. Community Bindings

The same C ABI surface can be compiled as a native dynamic library (`.so`/`.dylib`/`.dll`). Community maintainers can build bindings for:

- **Python** — via `ctypes` or `cffi`
- **Dart/Flutter** — via `dart:ffi`
- **PHP** — via `FFI`
- Other languages with C FFI support

All bindings use the same JSON serialization protocol as the WASM binding.

### 16. JSON Schemas

JSON Schemas generated from Rust types serve as the single source of truth for type definitions across all bindings:

- Generated from Rust types using `schemars`.
- Cover all output models (`LiturgicalCalendar`, `MassCalendar`, `HoursCalendar`, and their constituent types).
- Enable type generation in any language via tools like `quicktype`, `json-schema-to-typescript`, etc.
- Published alongside the Rust crate and npm package.

---

## Appendix — Method Reference

All `Result` types in this table use `RomcalError` as the error type (i.e., `Result<T>` is shorthand for `Result<T, RomcalError>`).

| Method                         | Signature                                              | Return type                    | Composition Model                                                               |
| ------------------------------ | ------------------------------------------------------ | ------------------------------ | ------------------------------------------------------------------------------- |
| `Romcal::new`                  | `(Config) → Result<Romcal>`                            | `Romcal`                       | —                                                                               |
| `Romcal::empty`                | `() → Romcal`                                          | `Romcal`                       | —                                                                               |
| `generate_liturgical_calendar` | `(&self, i32, YearFrame) → Result<LiturgicalCalendar>` | `LiturgicalCalendar`           | [Part IV §3](./liturgical-composition-model.md#3-layer-1--liturgical-calendar)  |
| `generate_mass_calendar`       | `(&self, i32, YearFrame) → Result<MassCalendar>`       | `MassCalendar`                 | [Part IV §4](./liturgical-composition-model.md#4-layer-2-mass--mass-calendar)   |
| `generate_hours_calendar`      | `(&self, i32, YearFrame) → Result<HoursCalendar>`      | `HoursCalendar`                | [Part IV §5](./liturgical-composition-model.md#5-layer-2-hours--hours-calendar) |
| `date_of`                      | `(&self, &str, i32) → Result<String>`                  | `String` (YYYY-MM-DD)          | —                                                                               |
| `liturgical_day_of`            | `(&self, &str, i32) → Result<LiturgicalDay>`           | `LiturgicalDay`                | [Part IV §3](./liturgical-composition-model.md#3-layer-1--liturgical-calendar)  |
| `masses_of`                    | `(&self, &str, i32) → Result<Vec<MassComposition>>`    | `Vec<MassComposition>`         | [Part IV §4](./liturgical-composition-model.md#4-layer-2-mass--mass-calendar)   |
| `hours_of`                     | `(&self, &str, i32) → Result<Vec<HoursComposition>>`   | `Vec<HoursComposition>`        | [Part IV §5](./liturgical-composition-model.md#5-layer-2-hours--hours-calendar) |
| `get_martyrology_entry`        | `(&self, &str) → Option<MartyrologyEntry>`             | `MartyrologyEntry`             | —                                                                               |
| `search_martyrology`           | `(&self, MartyrologyQuery) → Vec<…>`                   | `Vec<MartyrologySearchResult>` | —                                                                               |
| `add_calendar_definition`      | `(&mut self, CalendarDefinition)`                      | `()`                           | —                                                                               |
| `add_resources`                | `(&mut self, Resources)`                               | `()`                           | —                                                                               |
| `get_calendar_definition`      | `(&self, &str) → Option<&CalendarDefinition>`          | `&CalendarDefinition`          | —                                                                               |
| `get_resources`                | `(&self, &str) → Option<&Resources>`                   | `&Resources`                   | —                                                                               |
| `create_bundle`                | `(&self) → Result<String>`                             | `String` (JSON)                | —                                                                               |
