---
title: Input Data Model
description: Architecture and data-modeling reference for romcal's input data — the three-tier structure of calendar definitions, martyrology catalog, and liturgical texts that the engine transforms into the output model.
tableOfContents:
  minHeadingLevel: 2
  maxHeadingLevel: 4
---

<!-- AI Quick Index — concept/type → section (line numbers are approximate)

INPUT ARCHITECTURE
| Concept                              | Section              | Line   | Key references                    |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| Three-tier rationale                 | Context              | ~76    |                                   |
| Tier overview table                  | Part I §1            | ~120   |                                   |
| Directory layout                     | Part I §2            | ~135   |                                   |
| Graceful degradation                 | Part I §3            | ~207   |                                   |
| Calendar hierarchy (4 levels)        | Part I §4            | ~219   | CP 13-16                          |
| File format and validation           | Part I §5            | ~248   |                                   |

TIER 1 — CALENDAR DEFINITIONS
| Concept                              | Section              | Line   | Key references                    |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| CalendarDef (root)                   | Part II §1           | ~277   |                                   |
| CalendarMetadata                     | Part II §2           | ~311   | CP 13-16                          |
| ParticularConfig                     | Part II §3           | ~354   | GNLY 7                            |
| CelebrationDef (core input type)     | Part II §4           | ~383   |                                   |
| DateDef variants                     | Part II §5           | ~485   |                                   |
| DateFn (movable feasts)              | Part II §6           | ~542   |                                   |
| DateDefExceptions                    | Part II §7           | ~574   |                                   |
| Precedence (GNLY 59)                 | Part II §8           | ~638   | GNLY 59                           |
| CommonDef (simplified)               | Part II §9           | ~711   |                                   |
| TitleCategory and TitlesDef          | Part II §10          | ~783   |                                   |
| PatronageDef                         | Part II §11          | ~859   | CP 31                             |
| MartyrologyRef                       | Part II §12          | ~921   |                                   |
| MassReadingsDef                      | Part II §13          | ~980   | GILM 66, 69, 109                  |
| Definition examples                  | Part II §14          | ~1173  |                                   |

TIER 2 — MARTYROLOGY CATALOG & LOCALIZATION
| Concept                              | Section              | Line   | Key references                    |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| File organization (martyrology/locales) | Part III §1       | ~1268  |                                   |
| ResourcesMetadata                    | Part III §2          | ~1301  |                                   |
| MartyrologyEntryDef                  | Part III §3          | ~1400  |                                   |
| MartyrologyEntryType                 | Part III §4          | ~1518  |                                   |
| SaintDateDef                         | Part III §5          | ~1539  |                                   |
| Title qualifiers and fullname        | Part III §6          | ~1588  |                                   |
| Locale inheritance                   | Part III §7          | ~1708  |                                   |
| Localization of enums                | Part III §8          | ~1752  |                                   |

TIER 3 — LITURGICAL TEXTS
| Concept                              | Section              | Line   | Key references                    |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| Overview and copyright               | Part IV §1           | ~1776  |                                   |
| ProperTexts (Mass)                   | Part IV §2           | ~1799  | GIRM 363                          |
| AntiphonTextDef                      | Part IV §2           | ~1835  | GIRM 48, 87                       |
| MassTimeTexts (enriched)             | Part IV §2           | ~1847  |                                   |
| PrefaceRef (inline/catalog enum)     | Part IV §2           | ~1865  |                                   |
| CommonTexts (Mass)                   | Part IV §3           | ~1907  | GILM 71, 89                       |
| PrefaceCatalog (§3b)                 | Part IV §3b          | ~1950  |                                   |
| PrefaceDef                           | Part IV §3b          | ~1956  | GIRM 364-365                      |
| PrefaceCatalogFile                   | Part IV §3b          | ~1970  |                                   |
| ReadingsTexts (biblical content)     | Part IV §4           | ~1979  | GILM 75, 80                       |
| ReadingsTextsFile (readings + psalms)| Part IV §4           | ~1988  |                                   |
| ReadingTextDef (enriched)            | Part IV §4           | ~2001  | GILM 123, 125                     |
| ShortFormDef                         | Part IV §4           | ~2029  | GIRM 360; GILM 75, 77, 80        |
| SourceRef (shared helper)            | Part IV §4           | ~2047  |                                   |
| ProperTexts (Office)                 | Part IV §5           | ~2063  | GILH 225-236                      |
| PsalmodyEntryDef (enriched)          | Part IV §5           | ~2110  | GILM 19-22; GIRM 61              |
| PsalmAntiphonDef                     | Part IV §5           | ~2138  |                                   |
| CommonTexts (Office)                 | Part IV §6           | ~2189  | GILH 235                          |
| Latin baseline + copyright + examples| Part IV §7           | ~2235  |                                   |
| Citation → Text jointure (enriched)  | Part IV §8           | ~2288  |                                   |

TRANSFORMATION & ERGONOMICS
| Concept                              | Section              | Line   | Key references                    |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| Input → Output mapping               | Part V §1            | ~2336  |                                   |
| CommonDef → Common                   | Part V §2            | ~2386  |                                   |
| Title resolution                     | Part V §3            | ~2422  |                                   |
| Patronage resolution                 | Part V §4            | ~2450  |                                   |
| Canonical prayer                     | Part V §5            | ~2465  | CP 44                             |
| Contributor ergonomics               | Part VI §1           | ~2480  |                                   |
| Adding a new saint                   | Part VI §2           | ~2503  |                                   |
| Adding a new calendar                | Part VI §3           | ~2546  |                                   |
-->

## Context and Motivation

This document is one of four companion architecture documents:

- [**Public API**](./public-api.md) defines **how** to drive the engine — configuration, method signatures, CLI commands, and bindings surface.
- [**Calendar Resolution Algorithm**](./calendar-resolution.md) defines the **transformation pipeline** — how the engine turns input definitions into a resolved liturgical calendar.
- [**Liturgical Composition Model**](./liturgical-composition-model.md) defines romcal's **output** data model — the three-layer structure (`LiturgicalCalendar`, `MassCalendar`, `HoursCalendar`) that the engine produces.

The present document defines romcal's **input** data model — the data that contributors author and maintain, and that the engine transforms into the output model. While the output model is designed for consumers (applications, liturgical software, APIs), the input model is designed for **human editors**: liturgists, developers, and community contributors who add or modify calendar data in JSON files.

### Design Principles

1. **Contributor ergonomics first.** Input types favor simplicity over completeness. Where the engine can deduce information (e.g., liturgical colors from titles, full `Common` variant from `CommonDef` + season + saint count), the input stores the simplified form.

2. **Separation by copyright sensitivity.** Liturgical data has three levels of copyright exposure:
   - **Structural data** (dates, precedence, calendar hierarchy) — factual, no copyright.
   - **Biographical metadata** (saint names, dates, titles) — factual, no copyright.
   - **Liturgical texts** (prayers, readings, antiphons) — potentially copyrighted by the Holy See and/or national Bishops' Conferences.

   This leads naturally to a three-tier architecture.

3. **Graceful degradation.** The engine produces useful output even when not all tiers are available. Without liturgical texts (Tier 3), romcal still generates complete calendars with structural metadata — identities, dates, ranks, commons, colors — and reading citations.

4. **Language independence in structural data.** Tier 1 (calendar definitions) contains no natural-language text. All localized content lives in Tier 2 (names, metadata) and Tier 3 (liturgical texts).

### Relationship to the Output Model

Each input type maps to one or more output types:

| Input (this document)       | Output (companion document)                                    | Transformation                              |
| --------------------------- | -------------------------------------------------------------- | ------------------------------------------- |
| `CalendarDef`               | Calendar hierarchy, `from_calendar_id`                         | Inheritance resolution                      |
| `CelebrationDef`            | `Celebration` (identity fields)                                | Date resolution, precedence rules           |
| `CommonDef`                 | `Common`, `CommonInfo`                                         | Expansion based on season + saint metadata  |
| `TitleCategory` + qualifier | `Title`                                                        | Assembly of category + localized qualifier  |
| `PatronageDef`              | `Patronage`                                                    | Localization of role + subject              |
| `MartyrologyEntryDef`       | `MartyrologyEntry`                                             | Locale merge + type normalization (Tier 2a) |
| `ResourcesMetadata`         | `*Info.name`, `DayContext.*_name`                              | Locale merge (Tier 2b)                      |
| `MassReadingsDef`           | `CelebrationMass.readings` (citations)                         | Cycle resolution                            |
| `ProperMassTexts` (Tier 3)  | `CelebrationMass` (formulary, orations)                        | Text population by GIRM groups              |
| `CommonMassTexts` (Tier 3)  | `ReadingsPool`, `FlexibleOrations`                             | Pool assembly per Common variant            |
| `ProperHoursTexts` (Tier 3) | `CelebrationHour` (per-Hour elements)                          | Text population by GILH rules               |
| `ReadingsTexts` (Tier 3)    | `ReadingText`, `PsalmodyEntry`, `AlleluiaText`, `SequenceText` | Citation → full text jointure               |
| `PrefaceCatalog` (Tier 3)   | `PrefaceText`                                                  | Catalog ID → resolved preface text          |

---

## Part I — Architecture Overview

### 1. Three-Tier Input Model

| Tier | Name                 | Directory      | Content                                                  | Copyright status | In romcal repo? |
| ---- | -------------------- | -------------- | -------------------------------------------------------- | ---------------- | --------------- |
| 1    | Calendar Definitions | `calendars/`   | Dates, precedence, commons, martyrology refs, patronages | Free (factual)   | Yes             |
| 2a   | Martyrology Catalog  | `martyrology/` | Saint metadata, localized names                          | Free (factual)   | Yes             |
| 2b   | Localization         | `locales/`     | UI strings (seasons, ranks, weekdays, ordinals…)         | Free (factual)   | Yes             |
| 3    | Liturgical Texts     | `texts/`       | Prayers, readings, antiphons, orations, Office content   | Restricted       | External        |

**Tier 1** is purely structural: it describes _what_ is celebrated _when_ and at _what rank_, without any natural-language content. A `CelebrationDef` says "St. Scholastica is an Optional Memorial on February 10 with Common of Virgins and Common of Religious" — but not her name, her collect prayer, or the readings for her memorial.

**Tier 2** is split into two directories. **`martyrology/`** provides the biographical metadata: "Saint Scholastica, Virgin" (English), "Sainte Scholastique, vierge" (French), born ~480, died ~547, female, titles: virgin. **`locales/`** provides the UI strings (season names, rank names, weekday names, ordinals). Both share the same BCP-47 locale inheritance model with `en` as base.

**Tier 3** provides the actual liturgical texts: the collect prayer "O God, to make us seek the way of perfect love…", the entrance antiphon, the readings from the Lectionary, the Office hymns and psalmody antiphons. These texts come from published liturgical books that may be under copyright.

### 2. Directory Layout

```
data/
├── calendars/                           TIER 1 — Calendar Definitions
│   ├── general_roman/
│   │   ├── general_roman.json            Sanctoral cycle (General Calendar)
│   │   └── temporal_cycle.json           Proper of Time (with reading citations)
│   ├── regions/
│   │   ├── africa.json
│   │   ├── americas.json
│   │   ├── asia.json
│   │   └── europe.json
│   ├── countries/
│   │   ├── france/
│   │   │   ├── france.json               National calendar
│   │   │   ├── france__lyon.json         Diocesan calendar (Lyon)
│   │   │   └── france__paris.json        Diocesan calendar (Paris)
│   │   ├── italy/
│   │   │   └── italy.json
│   │   └── ...                           (~54 countries)
│   └── communities/
│       ├── benedictines.json             Religious order calendar
│       └── ...
│
├── martyrology/                          TIER 2a — Martyrology Catalog
│   ├── en/                               Base reference locale
│   │   ├── a.json                        Entries A (aaron... augustine)
│   │   ├── b.json                        Entries B
│   │   └── ...                           (one file per initial letter + digits)
│   ├── fr/                               French locale
│   │   ├── a.json                        French overrides for A entries
│   │   └── ...
│   ├── en-gb/                            British English (sparse overrides)
│   │   └── l.json                        "Labour Day" spelling override
│   ├── la/                               Latin
│   └── ...                               (~13 locales)
│
├── locales/                              TIER 2b — UI Strings & Localization
│   ├── en.json                           Base reference locale (seasons, ranks, ordinals...)
│   ├── fr.json                           French UI strings
│   ├── en-gb.json                        British English (sparse overrides)
│   └── ...                               (~13 locales)
│
└── texts/                                TIER 3 — Liturgical Texts (external)
    ├── la/                               Latin (editio typica baseline)
    │   ├── proper_of_time/
    │   │   ├── advent.json
    │   │   ├── christmas.json
    │   │   ├── lent.json
    │   │   ├── easter.json
    │   │   └── ordinary_time.json
    │   ├── proper_of_saints/
    │   │   ├── january.json
    │   │   └── ...
    │   ├── commons/
    │   │   ├── martyrs.json
    │   │   ├── virgins.json
    │   │   ├── pastors.json
    │   │   └── ...
    │   ├── readings/
    │   │   ├── old_testament.json
    │   │   ├── new_testament.json
    │   │   └── psalms.json
    │   └── office/
    │       ├── proper_of_time/
    │       ├── proper_of_saints/
    │       └── commons/
    ├── en/                               English edition (overrides la)
    └── fr/                               French edition (overrides la)
```

### 3. Graceful Degradation

The engine operates in three modes depending on which tiers are available:

| Available tiers | Output capabilities                                                                                                                                                                                |
| --------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **1 + 2**       | Full Layer 1 identity: dates, names, ranks, precedence, colors, commons, titles, patronages, martyrology metadata. Reading citations without full text. No formulary, orations, or Office content. |
| **1 + 2 + 3**   | Complete output: all of the above plus full liturgical texts (Mass formulary, readings with text, flexible orations, Office per-Hour content). Layer 2 Mass and Layer 2 Hours fully populated.     |
| **1 only**      | Structural calendar only: dates, ranks, precedence, commons. No localized names, no texts. Useful for date computation and calendar structure validation.                                          |

**Design consequence:** All output types that carry text content use `Option<String>` (or equivalent), allowing the engine to produce structurally valid output with `None` for text fields when Tier 3 is absent.

### 4. Calendar Hierarchy and Inheritance

The calendar hierarchy follows CP 13-16 (Calendaria Particularia):

```
Level 1: General Roman Calendar
    │     (general_roman.json + temporal_cycle.json)
    │
Level 2: Regional Calendar
    │     (e.g., europe.json)
    │     Inherits from: General Roman
    │
Level 3: National/Diocesan Calendar
    │     (e.g., france.json, france__lyon.json)
    │     Inherits from: Region → General Roman
    │
Level 4: Community Calendar
          (e.g., benedictines.json)
          Inherits from: any combination of the above
```

Each calendar declares its parent(s) via `parent_calendar_ids`. The engine resolves the full inheritance chain and applies overrides in order from most general to most specific. A child calendar can:

- **Add** new celebrations not present in the parent.
- **Override** properties of inherited celebrations (date, precedence, titles, patronages).
- **Drop** inherited celebrations (via `drop: true`).

The `parent_calendar_ids` array supports **multiple inheritance** for communities that follow both a national and a religious order calendar.

### 5. File Format and Validation

All input files use **JSON** with **JSON Schema** validation. Each file references its schema via the `$schema` field:

```json
{
  "$schema": "../../../schemas/calendar_definition.json",
  "id": "france",
  ...
}
```

Four JSON Schema files define the input model:

| Schema file                | Validates                           |
| -------------------------- | ----------------------------------- |
| `calendar_definition.json` | Tier 1 — `CalendarDef` files        |
| `martyrology.json`         | Tier 2a — Martyrology catalog files |
| `locale.json`              | Tier 2b — Locale UI strings files   |
| `liturgical_texts.json`    | Tier 3 — All liturgical text files  |

Editors with JSON Schema support (VS Code, IntelliJ) provide autocompletion and inline validation, significantly reducing errors in manual data entry.

---

## Part II — Tier 1: Calendar Definitions

Tier 1 files define the **structural skeleton** of liturgical calendars: which celebrations exist, when they occur, at what rank, with which Commons, and how they relate to martyrology entries. No natural-language text appears in Tier 1 — only identifiers, enums, dates, and flags.

### 1. `CalendarDef`

**What it is:** The root type for a calendar definition file. One file = one calendar.

```rust
struct CalendarDef {
    /// JSON Schema reference for validation
    schema: Option<String>,
    /// Unique identifier for this calendar (e.g., "france", "france__lyon", "benedictines")
    id: CalendarId,
    /// Calendar metadata (jurisdiction, type)
    metadata: CalendarMetadata,
    /// Configuration for movable feasts (Epiphany, Ascension, Corpus Christi)
    particular_config: Option<ParticularConfig>,
    /// Parent calendar(s) in the inheritance chain
    parent_calendar_ids: Vec<CalendarId>,
    /// Celebrations defined or overridden by this calendar
    celebrations: BTreeMap<CelebrationId, CelebrationDef>,
}

/// Unique calendar identifier (newtype for type safety)
struct CalendarId(String);

/// Unique celebration identifier within a calendar (e.g., "basil_the_great_and_gregory_nazianzen_bishops")
type CelebrationId = String;

/// A BCP-47 locale tag (e.g., "en", "fr", "en-gb", "pt-br")
type LocaleTag = String;
```

**Naming convention for `CalendarId`:** Country calendars use the country name in snake_case (`france`, `united_states`). Diocesan calendars use `country__diocese` with double underscore (`france__lyon`). Regional calendars use the region name (`europe`, `americas`). Community calendars use the order or community name (`benedictines`).

**Naming convention for `CelebrationId`:** The celebration identifier is a `snake_case` string derived from the celebration's name in English, typically following the pattern `{name}_{title}` (e.g., `basil_the_great_bishop`, `joan_of_arc_virgin`). For compound celebrations: `{name1}_and_{name2}_{shared_title}` (e.g., `peter_and_paul_apostles`). Temporal cycle days use `{season}_{week}_{weekday}` (e.g., `advent_1_sunday`, `ordinary_time_5_monday`).

### 2. `CalendarMetadata`

**What it is:** Classification metadata for a calendar.

```rust
struct CalendarMetadata {
    /// Jurisdiction type: ecclesiastical (diocese, religious order) or civil (country)
    jurisdiction: Jurisdiction,
    /// Calendar level in the hierarchy (CP 13-16)
    r#type: CalendarType,
    /// BCP-47 locale tags for which this calendar expects localized content
    /// (e.g., ["en", "fr"] for Canada). Used by audit tools to verify that
    /// all martyrology entries and UI strings are fully translated in every
    /// required locale. When absent (e.g., General Roman Calendar), no
    /// locale audit is enforced.
    locale_tags: Option<Vec<LocaleTag>>,
}

enum Jurisdiction {
    /// Calendar follows ecclesiastical boundaries (diocese, archdiocese, religious order)
    Ecclesiastical,
    /// Calendar follows civil boundaries (country, territory)
    Civil,
}

enum CalendarType {
    /// Universal calendar (General Roman Calendar)
    GeneralRoman,
    /// Multi-country regional calendar (e.g., Europe, Americas)
    Region,
    /// National calendar
    Country,
    /// Diocesan or archdiocesan calendar
    Diocese,
    /// Religious order or community calendar
    Community,
    /// Individual church (parish, shrine, basilica)
    Church,
}
```

**Liturgical basis:** CP 13-16 defines four levels of particular calendars: diocesan, national, regional (for a larger territory), and those of religious families. `CalendarType` extends this to include `GeneralRoman` (the universal base) and `Church` (individual churches per CP 16).

### 3. `ParticularConfig`

**What it is:** Configuration for movable feasts that national Bishops' Conferences may assign to Sunday (GNLY 7).

```rust
struct ParticularConfig {
    /// When true, the Epiphany is celebrated on the Sunday between Jan 2-8
    /// instead of January 6 (GNLY 7)
    epiphany_on_sunday: bool,
    /// When true, the Ascension is celebrated on the 7th Sunday of Easter
    /// instead of the Thursday of the 6th week (GNLY 7)
    ascension_on_sunday: bool,
    /// When true, Corpus Christi is celebrated on the Sunday after Trinity Sunday
    /// instead of the Thursday after Trinity Sunday (GNLY 7)
    corpus_christi_on_sunday: bool,
    /// Easter date calculation method
    easter_calculation_type: Option<EasterCalculationType>,
}

enum EasterCalculationType {
    /// Gregorian calendar (default, used by the Latin Church)
    Gregorian,
    /// Julian calendar (used by some Eastern Catholic churches)
    Julian,
}
```

**Liturgical basis:** GNLY 7 states: "In places, however, where the Solemnity of Epiphany, the Ascension, and Corpus Christi are not observed as Holydays of Obligation, they are assigned to a Sunday." The specific Sunday is determined by GNLY 7 itself. These are **permanent assignments**, not conflict-resolution transfers — they are applied before precedence resolution in the transformation pipeline.

### 4. `CelebrationDef`

**What it is:** The core input type. Defines or overrides a single liturgical day (celebration) within a calendar. All fields are optional to support partial overrides in child calendars.

```rust
struct CelebrationDef {
    // ── Date assignment ──

    /// When this celebration occurs
    date_def: Option<DateDef>,
    /// Conditional date adjustments (e.g., transfer when falling on a Sunday)
    date_exceptions: Option<DateDefExceptions>,

    // ── Rank and precedence ──

    /// Liturgical precedence level (GNLY 59)
    precedence: Option<Precedence>,
    /// Holy day of obligation
    is_holy_day_of_obligation: Option<bool>,
    /// Marks this celebration as optional in this particular calendar,
    /// regardless of its precedence rank. Default: false.
    ///
    /// Two distinct use cases:
    ///
    /// 1. **Dedication of the own church (GNLY §59, rank 4b):**
    ///    When the exact date of a church's dedication is unknown,
    ///    the conference of bishops assigns a common date (e.g., Oct 25).
    ///    At the national calendar level, this solemnity is marked
    ///    `is_optional: true` so that it does not override the weekday
    ///    for parishes that celebrate their dedication on a different date.
    ///
    /// 2. **Obligatory memorials in Lent (GNLY §14):**
    ///    The engine automatically downgrades obligatory memorials to
    ///    optional during Lent; contributors never need to set this flag
    ///    for that case — the engine handles it at runtime.
    ///
    /// Effect: optional celebrations appear in the `optional_celebrations`
    /// list rather than as the primary celebration of the day. When two
    /// celebrations share the same precedence, the non-optional one takes
    /// priority (GNLY §14).
    is_optional: Option<bool>,
    /// When true, other celebrations of the **same rank** that fall on
    /// this day are preserved rather than suppressed by the precedence
    /// engine. Default: false.
    ///
    /// Without this flag, the standard precedence resolution keeps only
    /// the highest-precedence celebration and discards the rest
    /// (GNLY §59). This flag relaxes that rule for same-rank neighbors.
    ///
    /// **Example — Immaculate Heart of Mary:**
    /// This obligatory memorial (rank `general_memorial_10`) always falls
    /// on the Saturday after the Sacred Heart. On that Saturday, other
    /// optional memorials (or a Saturday BVM commemoration per GNLY §15)
    /// may legitimately be celebrated. Setting
    /// `allows_same_rank_coexistence: true` tells the engine to keep them
    /// alongside the Immaculate Heart rather than discarding them.
    ///
    /// This flag is rare — most celebrations follow the standard "highest
    /// precedence wins" rule. Only set it when liturgical norms explicitly
    /// permit multiple celebrations of equal rank on the same day.
    allows_same_rank_coexistence: Option<bool>,

    // ── Identity ──

    /// Simplified Common(s) applicable to this celebration
    commons_def: Option<CommonsDef>,
    /// References to martyrology entries (biographical metadata)
    martyrology: Option<Vec<MartyrologyRef>>,
    /// Title modifications (append or prepend TitleCategory values)
    titles: Option<TitlesDef>,
    /// Patronage designations specific to this calendar level
    patronages: Option<Vec<PatronageDef>>,
    /// Override the locale key used to look up this celebration's name.
    /// When absent, the CelebrationId itself serves as the locale key.
    custom_locale_key: Option<String>,

    // ── Mass reading references ──

    /// Reading citations organized by mass time and liturgical cycle.
    /// Contains only citation strings (e.g., "Isa 2:1-5"), not full text.
    masses: Option<MassesDefinitions>,

    // ── Removal ──

    /// When true, this celebration is removed from the calendar.
    /// Used by child calendars to drop inherited celebrations.
    drop: Option<bool>,
}
```

**Why all fields are `Option`:** A `CelebrationDef` in a child calendar is a **partial override** — it only specifies the fields that differ from the inherited definition. The engine merges child definitions onto parent definitions field by field. A `CelebrationDef` in the General Roman Calendar typically has most fields populated; a `CelebrationDef` in a national calendar may only override `precedence` and add `patronages`.

**Notable absence: `colors`.** Liturgical colors are **not** an input field. They are deduced by the engine from:

- The celebration's titles (presence of `TitleCategory::Martyr` → red per GIRM 346b)
- The liturgical season (Advent/Lent → purple, Easter/Christmas → white, OT → green)
- The celebration type (BVM → white, Apostles/Evangelists → red)

This eliminates a class of data entry errors where colors are inconsistent with titles.

> **Design note:** The current implementation includes a deprecated `colors` field for backward compatibility. The architecture does not include it — colors are always computed. Calendar definitions that need to force a non-standard color (rare edge cases) can achieve this through the titles mechanism or a dedicated override (to be designed if needed).

### 5. `DateDef`

**What it is:** Defines when a celebration occurs. Supports fixed dates, movable feasts, and positional calculations.

```rust
enum DateDef {
    /// Fixed month and day (e.g., January 2)
    MonthDate {
        month: u8,         // 1-12
        date: u8,          // 1-31
        day_offset: Option<i32>,  // shift by N days after resolution
    },
    /// Calculated from a movable feast function (e.g., Easter + 49 = Pentecost)
    DateFunction {
        date_fn: DateFn,
        day_offset: Option<i32>,
    },
    /// Nth weekday of a specific month (e.g., 2nd Sunday of November)
    WeekdayOfMonth {
        month: u8,
        day_of_week: u8,          // 0 = Sunday, 6 = Saturday
        nth_week_in_month: u8,    // 1-based
        day_offset: Option<i32>,
    },
    /// Last weekday of a specific month (e.g., last Sunday of November)
    LastWeekdayOfMonth {
        month: u8,
        last_day_of_week_in_month: u8,  // 0 = Sunday, 6 = Saturday
        day_offset: Option<i32>,
    },
    /// Inherited from the Proper of Time (temporal cycle).
    /// Used by sanctoral celebrations that share a date with a temporal day.
    InheritedFromProperOfTime {},
}
```

**JSON serialization:** `DateDef` is serialized as an **untagged** enum — the variant is inferred from the fields present:

```json
// MonthDate — January 2
{ "month": 1, "date": 2 }

// DateFunction — Easter Sunday + 49 days (Pentecost)
{ "date_fn": "easter_sunday", "day_offset": 49 }

// WeekdayOfMonth — 4th Thursday of November (Thanksgiving in the US)
{ "month": 11, "day_of_week": 4, "nth_week_in_month": 4 }

// LastWeekdayOfMonth — last Sunday of November (Christ the King)
{ "month": 11, "last_day_of_week_in_month": 0 }

// InheritedFromProperOfTime
{}
```

**The `day_offset` field:** An optional signed integer that shifts the resolved date by N days. This handles celebrations defined relative to another date: e.g., the Monday after Pentecost (Mary, Mother of the Church) is `{ "date_fn": "pentecost_sunday", "day_offset": 1 }`.

### 6. `DateFn`

**What it is:** An enum of movable feast functions — celebrations whose civil date changes each year, computed from Easter or from calendar rules.

```rust
enum DateFn {
    EasterSunday,
    PalmSunday,
    PentecostSunday,
    DivineMercySunday,
    EpiphanySunday,
    CorpusChristiSunday,
    MaryMotherOfTheChurch,
    ImmaculateHeartOfMary,
    PresentationOfTheLord,
    Annunciation,
    NativityOfJohnTheBaptist,
    PeterAndPaulApostles,
    Transfiguration,
    Assumption,
    ExaltationOfTheHolyCross,
    AllSaints,
    ImmaculateConceptionOfMary,
}
```

Most of these are **fixed-date feasts** (e.g., Assumption = August 15) that appear in `DateFn` because they may be subject to **transfer rules** (GNLY 5, 60) when they fall on privileged Sundays. The `DateFn` mechanism allows the engine to apply these transfer rules centrally.

The Easter-dependent feasts (`EasterSunday`, `PalmSunday`, `PentecostSunday`, `DivineMercySunday`) are computed from the Easter date using established astronomical algorithms.

`EpiphanySunday` and `CorpusChristiSunday` resolve to their fixed dates or to the assigned Sunday depending on `ParticularConfig`.

### 7. `DateDefExceptions`

**What it is:** Conditional date adjustments that override the base `DateDef` when specific conditions are met.

```rust
/// A single exception: condition + alternative date
struct DateDefException {
    /// The condition that triggers the exception
    when: ExceptionCondition,
    /// The date to use when the condition is met
    then: DateDefExtended,
}

/// Exception conditions
enum ExceptionCondition {
    /// The resolved date falls between two dates (inclusive or exclusive)
    IsBetween {
        from: DateDef,
        to: DateDef,
        inclusive: bool,
    },
    /// The resolved date is the same as another calculated date
    IsSameAsDate {
        date: DateDef,
    },
    /// The resolved date falls on a specific day of the week
    IsDayOfWeek {
        day_of_week: u8,  // 0 = Sunday, 6 = Saturday
    },
}

/// Extended date definition supporting offsets from the base date
enum DateDefExtended {
    /// A fully specified date (replaces the base date)
    DateDef(DateDef),
    /// An offset from the base date (shifts by N days)
    WithOffset { day_offset: i32 },
}

/// Single exception or list of exceptions
enum DateDefExceptions {
    Single(DateDefException),
    Multiple(Vec<DateDefException>),
}
```

**Example — Annunciation transfer:**

When March 25 falls during Holy Week or the Easter Octave, the Annunciation is transferred to the Monday after the 2nd Sunday of Easter:

```json
{
  "date_def": { "date_fn": "annunciation" },
  "date_exceptions": {
    "when": {
      "from": { "date_fn": "palm_sunday" },
      "to": { "date_fn": "divine_mercy_sunday" },
      "inclusive": true
    },
    "then": { "date_fn": "divine_mercy_sunday", "day_offset": 1 }
  }
}
```

### 8. `Precedence`

**What it is:** The liturgical precedence level from the Table of Liturgical Days (GNLY 59). Determines which celebration takes priority when multiple celebrations fall on the same date.

GNLY 59 defines 13 numbered levels. The input model subdivides these into **27 variants** to distinguish sub-levels needed for correct conflict resolution:

```rust
enum Precedence {
    // ── Level 1: Paschal Triduum ──
    Triduum_1,

    // ── Level 2: Major celebrations ──
    ProperOfTimeSolemnity_2,      // Christmas, Epiphany, Ascension, Pentecost...
    PrivilegedSunday_2,           // Sundays of Advent, Lent, Easter
    AshWednesday_2,
    WeekdayOfHolyWeek_2,          // Monday-Wednesday of Holy Week
    WeekdayOfEasterOctave_2,      // Monday-Saturday of Easter Octave

    // ── Level 3: General Calendar solemnities ──
    GeneralSolemnity_3,                          // Immaculate Conception, Assumption...
    CommemorationOfAllTheFaithfulDeparted_3,      // All Souls — sui generis

    // ── Level 4: Proper solemnities (CP 8-12) ──
    ProperSolemnity_PrincipalPatron_4a,
    ProperSolemnity_DedicationOfTheOwnChurch_4b,
    ProperSolemnity_TitleOfTheOwnChurch_4c,
    ProperSolemnity_TitleOrFounderOrPrimaryPatronOfAReligiousOrg_4d,

    // ── Level 5: General Calendar feasts of the Lord ──
    GeneralLordFeast_5,           // Presentation, Transfiguration...

    // ── Level 6: Unprivileged Sundays ──
    UnprivilegedSunday_6,         // Sundays of Christmas Time, Ordinary Time

    // ── Level 7: General Calendar feasts ──
    GeneralFeast_7,               // BVM feasts, Apostle feasts...

    // ── Level 8: Proper feasts (CP 8-12) ──
    ProperFeast_PrincipalPatronOfADiocese_8a,
    ProperFeast_DedicationOfTheCathedralChurch_8b,
    ProperFeast_PrincipalPatronOfARegion_8c,
    ProperFeast_TitleOrFounderOrPrimaryPatronOfAReligiousOrg_8d,
    ProperFeast_ToAnIndividualChurch_8e,
    ProperFeast_8f,

    // ── Level 9: Privileged weekdays ──
    PrivilegedWeekday_9,          // Advent Dec 17-24, Christmas Octave, Lent weekdays

    // ── Level 10: General Calendar obligatory memorials ──
    GeneralMemorial_10,

    // ── Level 11: Proper obligatory memorials ──
    ProperMemorial_SecondPatron_11a,
    ProperMemorial_11b,

    // ── Level 12: Optional memorials ──
    OptionalMemorial_12,

    // ── Level 13: Weekdays ──
    Weekday_13,
}
```

**`Precedence.to_rank()` mapping:** Each precedence variant maps deterministically to a `Rank` (Solemnity, Sunday, Feast, Memorial, OptionalMemorial, Weekday). This mapping is internal to the engine. See the companion document (Part IV §7) for the `Rank` enum and the mapping details.

**JSON serialization:** Precedence values serialize as snake_case strings:

```json
"precedence": "general_memorial_10"
"precedence": "proper_feast__principal_patron_of_a_region_8c"
"precedence": "optional_memorial_12"
```

### 9. `CommonDef`

**What it is:** A simplified enum of liturgical Commons, used in calendar definitions. The engine expands each `CommonDef` into the fully resolved `Common` variant (see Part V §2 for the expansion rules).

**Why simplified:** The full `Common` enum (34 variants) encodes season (BVM in Advent vs. Easter), count (One Martyr vs. Several Martyrs), and specific pastoral categories. These distinctions depend on runtime context (the current season, the number of saints celebrated). Contributors should not need to specify them — they provide the base category, and the engine resolves the rest.

```rust
enum CommonDef {
    /// No Common — the celebration has fully proper texts
    None,

    // ── Dedication of a Church ──
    DedicationAnniversary_Inside,
    DedicationAnniversary_Outside,

    // ── Blessed Virgin Mary ──
    BlessedVirginMary,         // → resolved to season-specific variant at runtime

    // ── Martyrs ──
    Martyrs,                   // → resolved to One/Several + Easter/OutsideEaster
    MissionaryMartyrs,         // → resolved to One/Several
    VirginMartyrs,
    WomanMartyrs,

    // ── Pastors ──
    Pastors,                   // → resolved to One/Several/PopeOrBishop/Bishop
    Popes,
    Bishops,
    Founders,                  // → resolved to One/Several
    Missionaries,

    // ── Doctors of the Church ──
    DoctorsOfTheChurch,

    // ── Virgins ──
    Virgins,                   // → resolved to One/Several

    // ── Holy Men and Women ──
    Saints,                    // → resolved to One/Several + subcategory
    Abbots,
    Monks,
    Nuns,
    Religious,
    MercyWorkers,
    Educators,
    HolyWomen,
}
```

**23 variants** (including `None`), compared to 34 in the output `Common` enum.

**JSON serialization:** Single value or array. The `CommonsDef` type handles both:

```rust
/// Single Common or list of Commons
enum CommonsDef {
    Single(CommonDef),
    Multiple(Vec<CommonDef>),
}
```

```json
// Single Common
"commons_def": "martyrs"

// Multiple Commons (e.g., a bishop who is also a Doctor of the Church)
"commons_def": ["bishops", "doctors_of_the_church"]

// No Common (fully proper texts)
"commons_def": "none"
```

### 10. `TitleCategory` and `TitlesDef`

**What it is:** `TitleCategory` is a closed enum of ecclesiastical title categories. It separates categories from qualifiers (localized free-text in Tier 2) and patronage designations (their own type, `PatronageDef`).

**Design benefits** (see also the companion document, Part III §8):

- **Martyr detection is trivial:** `category == TitleCategory::Martyr` — no fragile match list.
- **Zero core modifications** for new qualifiers — qualifiers are localized free-text in Tier 2.
- **Patronages are separated** into their own type (`PatronageDef`) with only 3 role variants.

```rust
/// Fixed ecclesiastical title categories.
/// Closed enum — changes only if the Church creates a new title category.
/// These categories have liturgical impact (e.g., Martyr → red color).
enum TitleCategory {
    Abbess,
    Abbot,
    Apostle,
    Archangel,
    Bishop,
    Deacon,
    DoctorOfTheChurch,
    Empress,
    Evangelist,
    Hermit,
    King,
    Martyr,
    Missionary,
    Monk,
    Pope,
    Patriarch,
    Pilgrim,
    Priest,
    Prophet,
    Queen,
    Religious,
    Virgin,
    // Unique relational titles (liturgically significant, appear as-is)
    ParentsOfTheBlessedVirginMary,
    SpouseOfTheBlessedVirginMary,
}
```

**24 variants** — each represents a fixed liturgical category recognized by the Church. New qualifiers ("the First Martyr", "Slavic Missionary") do not add variants — they are expressed as localized qualifiers in Tier 2 (see Part III §6).

**`TitlesDef` — title operations in definitions:**

Calendar definitions do not assign titles directly (titles come from the martyrology entry in Tier 2). Instead, definitions **modify** the inherited titles through append/prepend operations:

```rust
enum TitlesDef {
    /// Replace all titles with this list
    Titles(Vec<TitleCategory>),
    /// Modify inherited titles by appending and/or prepending
    CompoundTitle(CompoundTitle),
}

struct CompoundTitle {
    prepend: Option<Vec<TitleCategory>>,
    append: Option<Vec<TitleCategory>>,
}
```

```json
// Append a title to the inherited list
"titles": { "append": ["doctor_of_the_church"] }

// Replace all titles
"titles": ["bishop", "martyr"]

// Prepend (rare — places a title before the inherited ones)
"titles": { "prepend": ["apostle"] }
```

**In `MartyrologyRef` overrides (see §12):** When a `MartyrologyRef` uses the object form, its `titles` field can also use `TitlesDef` to modify the entry's titles in the context of a specific celebration.

### 11. `PatronageDef`

**What it is:** A patronage designation for a celebration within a specific calendar. Patronages are defined at the calendar level (not in the martyrology) because a saint is only a patron in a specific jurisdiction.

```rust
/// Patronage role (CP 31)
enum PatronRole {
    /// Principal patron — at most one per jurisdiction (CP 31)
    PrincipalPatron,
    /// Secondary patron — additional patron after the principal (CP 31)
    SecondaryPatron,
    /// Co-patron (joint patron alongside the principal)
    Copatron,
}

/// Patronage definition in calendar input
struct PatronageDef {
    /// The patronage role
    role: PatronRole,
    /// Locale key referencing the patronage subject (e.g., "france", "the_diocese").
    /// The localized text (e.g., "France", "la France") is in Tier 2 resources metadata.
    of: String,
}
```

**Liturgical basis:** CP 31: "From now on there is to be only one principal patron. Another may be added as a secondary patron." Gender (patron/patroness) is resolved at display time from `MartyrologyEntry.sex` — it is not a distinct role.

```json
// France calendar: Joan of Arc as co-patroness
"joan_of_arc_virgin": {
  "precedence": "proper_memorial__second_patron_11a",
  "patronages": [
    { "role": "copatron", "of": "france" }
  ]
}

// Lyon diocesan calendar: Pothinus as principal patron of the city
"pothinus_of_lyon_bishop": {
  "patronages": [
    { "role": "principal_patron", "of": "the_city_of_lyon" }
  ]
}
```

**Localization:** The `of` field is a locale key. The localized subject name is stored in Tier 2 `ResourcesMetadata.patronage_subjects`:

```json
// locales/en.json
"patronage_subjects": {
  "france": "France",
  "the_city_of_lyon": "the City of Lyon",
  "the_diocese": "the Diocese"
}

// locales/fr.json
"patronage_subjects": {
  "france": "la France",
  "the_city_of_lyon": "la Ville de Lyon",
  "the_diocese": "le Diocèse"
}
```

### 12. `MartyrologyRef`

**What it is:** A reference from a `CelebrationDef` to one or more entries in the martyrology catalog (Tier 2). Most celebrations reference a single entry; compound celebrations (e.g., "Saints Basil and Gregory") reference multiple entries.

```rust
enum MartyrologyRef {
    /// Simple reference by ID
    ResourceId(String),
    /// Reference with overrides specific to this celebration
    Override(MartyrologyEntryOverride),
}

struct MartyrologyEntryOverride {
    /// The martyrology entry ID (must exist in Tier 2)
    id: String,
    /// Title modifications for this celebration context
    titles: Option<TitlesDef>,
    /// Whether to hide titles when displaying this entry
    hide_titles: Option<bool>,
    /// Override the number of persons (for groups)
    count: Option<SaintCount>,
}

/// Number of persons in a group
enum SaintCount {
    /// Known number
    Count(u32),
    /// Unknown number ("and companions")
    Many,
}
```

**JSON serialization:** `MartyrologyRef` is an untagged enum — a plain string is a `ResourceId`, an object is an `Override`:

```json
// Simple reference
"martyrology": ["basil_the_great_bishop", "gregory_nazianzen_bishop"]

// Reference with title override (in a diocesan calendar)
"martyrology": [
  {
    "id": "pothinus_of_lyon_bishop",
    "titles": ["bishop", "martyr"]
  },
  "blandina_of_lyon_virgin",
  "companions_martyrs"
]

// Reference with title append
"martyrology": [
  {
    "id": "irenaeus_of_lyon_bishop",
    "titles": { "append": ["doctor_of_the_church"] }
  }
]
```

**Why overrides exist:** The same martyrology entry may be celebrated differently in different calendars. In the General Calendar, Irenaeus is "Bishop and Martyr"; in Lyon, he is "Bishop, Martyr, and Patron of the Diocese." The override mechanism lets the diocesan calendar add this context without modifying the universal martyrology entry.

### 13. `MassReadingsDef`

**What it is:** Reading citations for the Mass, organized by mass time and liturgical cycle. Contains only **citation strings** (e.g., `"Isa 2:1-5"`), not the full biblical text. The full text is in Tier 3.

```rust
/// All mass reading definitions for a liturgical day
struct MassesDefinitions(BTreeMap<MassTime, MassCycleDefinition>);

/// Reading citations for a specific mass time, organized by liturgical cycle
struct MassCycleDefinition(BTreeMap<LiturgicalCycle, MassReadingsContent>);

/// Reading citations for a specific cycle
struct MassReadingsContent(BTreeMap<ReadingSlot, String>);
```

**`MassTime` — when the Mass is celebrated:**

```rust
enum MassTime {
    EasterVigil,                  // Holy Saturday night
    PreviousEveningMass,          // Vigil Mass (evening before)
    NightMass,                    // Christmas Midnight Mass
    MassAtDawn,                   // Christmas Dawn Mass
    MorningMass,                  // Morning Mass (Dec 24)
    MassOfThePassion,             // Palm Sunday
    CelebrationOfThePassion,      // Good Friday (not a Mass — PS 59)
    DayMass,                      // Regular daytime Mass (default)
    ChrismMass,                   // Holy Thursday morning (diocesan)
    EveningMassOfTheLordsSupper,  // Holy Thursday evening
}
```

**`LiturgicalCycle` — which readings cycle applies:**

```rust
enum LiturgicalCycle {
    /// Elements common to all cycles
    Invariant,
    /// Sunday three-year cycle (GILM 66)
    YearA, YearB, YearC,
    /// Combined cycles (for readings shared across 2 years)
    YearAB, YearAC, YearBC,
    /// Weekday two-year cycle (GILM 69)
    Year1, Year2,
}
```

**`ReadingSlot` — position within the Liturgy of the Word:**

```rust
enum ReadingSlot {
    // ── Standard Mass readings ──
    Reading1,            // First reading (usually OT)
    Psalm,               // Responsorial psalm
    Canticle,            // Biblical canticle (alternative to psalm)
    Reading2,            // Second reading (NT, on Sundays and solemnities)
    Sequence,            // Sequence hymn (GIRM 64 — 5 celebrations only)
    Alleluia,            // Acclamation before the Gospel
    Gospel,              // Gospel reading

    // ── Palm Sunday ──
    MessianicEntry,      // Gospel of the Lord's Entry (procession)

    // ── Easter Vigil (PS 85) ──
    // The Easter Vigil has 7 OT readings, each with its own
    // psalm/canticle, plus Epistle and Gospel.
    EasterVigilPsalm2,
    EasterVigilReading3,
    EasterVigilCanticle3,
    EasterVigilReading4,
    EasterVigilPsalm4,
    EasterVigilReading5,
    EasterVigilCanticle5,
    EasterVigilReading6,
    EasterVigilPsalm6,
    EasterVigilReading7,
    EasterVigilPsalm7,
    EasterVigilEpistle,
}
```

**Layered resolution rule:** `MassCycleDefinition` maps multiple `LiturgicalCycle` keys to partial `MassReadingsContent` entries. The engine merges them in layers:

1. Start with `Invariant` (base readings shared across all cycles).
2. Overlay the applicable **Sunday cycle** key (`YearA`, `YearB`, `YearC`, or a combined key like `YearBC`) — overrides matching slots from step 1.
3. Overlay the applicable **weekday cycle** key (`Year1` or `Year2`) — overrides matching slots from steps 1-2.

Each layer only specifies the reading slots it contributes; the engine accumulates slots from all applicable layers. This avoids duplicating shared readings across cycle keys.

> **Design note — why layered, not compound keys:** A layered merge keeps the cycle dimensions orthogonal. A Sunday may need per-year readings (3 variants of 7 slots each), while a weekday may need per-year first readings _and_ a Sunday-cycle-dependent Gospel override. Layering handles both patterns with the same type, without introducing compound keys like `Year1A` / `Year2B`.

**JSON example — Advent 1st Sunday (3-year Sunday cycle):**

```json
"advent_1_sunday": {
  "masses": {
    "day_mass": {
      "year_a": {
        "reading_1": "Isa 2:1-5",
        "psalm": "Ps 122:1-2,3-4ab,4cd-5,6-7,8-9",
        "reading_2": "Rom 13:11-14a",
        "gospel": "Matt 24:37-44"
      },
      "year_b": {
        "reading_1": "Isa 63:16b-17,19b; 64:2-7",
        "psalm": "Ps 80:2-3,15-16,18-19",
        "reading_2": "1 Thess 3:12—4:2",
        "gospel": "Mark 13:33-37"
      },
      "year_c": {
        "reading_1": "Jer 33:14-16",
        "psalm": "Ps 25:4-5,8-9,10+14",
        "reading_2": "1 Cor 1:3-9",
        "gospel": "Luke 21:25-28,34-36"
      },
      "invariant": {
        "alleluia": "Ps 85:8"
      }
    }
  }
}
```

**JSON example — Solemnity with vigil (Peter and Paul):**

```json
"peter_and_paul_apostles": {
  "masses": {
    "previous_evening_mass": {
      "invariant": {
        "reading_1": "Acts 3:1-10",
        "psalm": "Ps 19:2-3,4-5",
        "gospel": "John 21:15-19"
      }
    },
    "day_mass": {
      "invariant": {
        "reading_1": "Acts 12:1-11",
        "psalm": "Ps 34:2-3,4-5,6-7,8-9",
        "reading_2": "2 Tim 4:6-8,17-18",
        "gospel": "Matt 16:13-19"
      }
    }
  }
}
```

**JSON example — Weekday with Sunday-cycle-dependent Gospel (OT Week 18 Monday):**

The weekday semi-continuous Gospel sequence (GILM §109: Matthew in weeks 10-21) overlaps with the 18th Sunday of Year A (Mt 14:13-21). The Lectionary resolves this by printing a rubrical note: « L'année A, où l'Évangile ci-dessous a été lu le dimanche, on lit aujourd'hui Matthieu 14, 22-36. » Layered resolution models this without any cross-day runtime dependency:

```json
"ordinary_time_18_monday": {
  "masses": {
    "day_mass": {
      "invariant": {
        "alleluia": "Matt 4:4b",
        "gospel": "Matt 14:13-21"
      },
      "year_a": {
        "alleluia": "John 1:49b",
        "gospel": "Matt 14:22-36"
      },
      "year_1": {
        "reading_1": "Num 11:4b-15",
        "psalm": "Ps 81:12-13,14-15,16-17"
      },
      "year_2": {
        "reading_1": "Jer 28:1-17",
        "psalm": "Ps 119:29,43,79-80,95,102"
      }
    }
  }
}
```

Resolution for Year 1 + Year A: `Invariant` provides base readings → `YearA` overrides `alleluia` and `gospel` → `Year1` provides `reading_1` and `psalm`. Result: all 4 slots populated, with the Year-A-specific Gospel (Mt 14:22-36) since Mt 14:13-21 was already read on Sunday.

**Separation of concerns:** Only reading **citations** appear in Tier 1. The corresponding full texts, psalm texts, and short-form variants are in Tier 3. The engine joins them at runtime using the citation string as the key (see Part IV §8).

### 14. Definition Examples

#### General Roman Calendar (sanctoral cycle)

```json
{
  "$schema": "../../../schemas/calendar_definition.json",
  "id": "general_roman",
  "metadata": {
    "jurisdiction": "ecclesiastical",
    "type": "general_roman"
  },
  "particular_config": {
    "epiphany_on_sunday": false,
    "ascension_on_sunday": false,
    "corpus_christi_on_sunday": false
  },
  "parent_calendar_ids": [],
  "celebrations": {
    "basil_the_great_and_gregory_nazianzen_bishops": {
      "precedence": "general_memorial_10",
      "date_def": { "month": 1, "date": 2 },
      "commons_def": "none",
      "martyrology": ["basil_the_great_bishop", "gregory_nazianzen_bishop"]
    },
    "hilary_of_poitiers_bishop": {
      "precedence": "optional_memorial_12",
      "date_def": { "month": 1, "date": 13 },
      "commons_def": ["bishops", "doctors_of_the_church"]
    },
    "agnes_of_rome_virgin": {
      "precedence": "general_memorial_10",
      "date_def": { "month": 1, "date": 21 },
      "commons_def": ["virgin_martyrs", "virgins"]
    }
  }
}
```

#### National calendar (France) overriding the General Calendar

```json
{
  "$schema": "../../../../schemas/calendar_definition.json",
  "id": "france",
  "metadata": {
    "jurisdiction": "civil",
    "type": "country",
    "locale_tags": ["fr"]
  },
  "particular_config": {
    "epiphany_on_sunday": true,
    "ascension_on_sunday": false,
    "corpus_christi_on_sunday": true
  },
  "parent_calendar_ids": ["europe"],
  "celebrations": {
    "joan_of_arc_virgin": {
      "precedence": "proper_memorial__second_patron_11a",
      "date_def": { "month": 5, "date": 30 },
      "commons_def": ["virgins", "saints"],
      "patronages": [{ "role": "copatron", "of": "france" }]
    },
    "our_lady_of_lourdes": {
      "commons_def": "none"
    },
    "denis_of_paris_bishop": {
      "precedence": "general_memorial_10",
      "date_def": { "month": 10, "date": 9 },
      "commons_def": ["martyrs", "bishops"],
      "patronages": [{ "role": "principal_patron", "of": "france" }]
    }
  }
}
```

**Key patterns illustrated:**

- **`joan_of_arc_virgin`**: New celebration with full definition + patronage.
- **`our_lady_of_lourdes`**: Override of an inherited celebration (only `commons_def` changes — all other fields inherited from parent).
- **`denis_of_paris_bishop`**: Elevation from optional to obligatory memorial + patronage.

---

## Part III — Tier 2: Martyrology Catalog & Localization

Tier 2 provides two categories of data, stored in **separate directories**:

- **`martyrology/`** — Biographical metadata about saints, blesseds, and non-person celebrations (the martyrology catalog).
- **`locales/`** — Localized UI strings (season names, rank names, weekday names, ordinals, etc.).

Both categories are organized by locale, with `en` (English) as the base reference locale. Other locales provide **sparse overrides** — only the fields that differ from `en`.

At runtime, the engine merges both categories into a single `Resources` object per locale (the API consumer sees a unified type — see Part V §1).

### 1. File Organization

**Martyrology files** (`martyrology/{locale}/`): For manageability, martyrology entries are split into multiple files per locale, grouped by the first letter of the entry ID: `a.json`, `b.json`, …, `z.json`, `2.json` (for IDs starting with digits).

```rust
/// Root type for a martyrology file
struct MartyrologyFile {
    /// JSON Schema reference
    schema: Option<String>,
    /// BCP-47 locale tag (e.g., "en", "fr", "en-gb")
    locale_tag: LocaleTag,
    /// Martyrology entries (biographical metadata + localized names)
    martyrology: BTreeMap<MartyrologyEntryId, MartyrologyEntryDef>,
}

/// Unique martyrology entry identifier (e.g., "adalbert_of_prague_bishop")
type MartyrologyEntryId = String;
```

**Locale files** (`locales/{locale}.json`): One flat JSON file per locale, containing all UI strings and display configuration.

```rust
/// Root type for a locale file
struct LocaleFile {
    /// JSON Schema reference
    schema: Option<String>,
    /// BCP-47 locale tag (e.g., "en", "fr", "en-gb")
    locale_tag: LocaleTag,
    /// UI strings and localization metadata
    metadata: ResourcesMetadata,
}
```

### 2. `ResourcesMetadata`

**What it is:** Localized UI strings and display configuration. Provides all the natural-language elements needed to compose celebration names, format dates, and display calendar information.

```rust
struct ResourcesMetadata {
    /// Season display names and name templates
    seasons: BTreeMap<Season, SeasonLocalization>,
    /// Period display names
    periods: BTreeMap<Period, String>,
    /// Rank display names
    ranks: BTreeMap<Rank, String>,
    /// Cycle display names
    cycles: CyclesLocalization,
    /// Weekday names (0 = Sunday, 6 = Saturday)
    weekdays: BTreeMap<u8, String>,
    /// Month names (0 = January, 11 = December)
    months: BTreeMap<u8, String>,
    /// Liturgical color names
    colors: BTreeMap<Color, String>,
    /// Ordinal format preference and ordinal strings
    ordinal_format: OrdinalFormat,
    ordinals_letters: Option<BTreeMap<u32, String>>,
    ordinals_numeric: Option<BTreeMap<u32, String>>,

    // ── Display and composition ──

    /// TitleCategory display names
    title_categories: BTreeMap<TitleCategory, String>,
    /// Patronage role display names
    patronage_roles: BTreeMap<PatronRole, String>,
    /// Patronage subject localized names (referenced by PatronageDef.of)
    patronage_subjects: BTreeMap<String, String>,
    /// Common display names (for the full Common enum)
    commons: BTreeMap<Common, String>,
    /// Canonization level display names
    canonization_levels: BTreeMap<CanonizationLevel, String>,
    /// Mass time display names
    mass_times: BTreeMap<MassTime, String>,
    /// Templates for fullname construction (see Part III §6)
    fullname_templates: Option<FullnameTemplates>,
}

enum OrdinalFormat {
    /// Use word-form ordinals ("first", "second"...)
    Letters,
    /// Use numeric ordinals ("1st", "2nd"...)
    Numeric,
}
```

**Season localization — name templates:**

Each season has templates for composing day names from components (weekday, ordinal week number, month, day):

```rust
struct SeasonLocalization {
    /// Season display name (e.g., "Advent", "Ordinary Time")
    season: String,
    /// Template for weekday names (e.g., "{weekday} of the {ordinal} week of Advent")
    weekday: Option<String>,
    /// Template for Sunday names (e.g., "{ordinal} Sunday of Advent")
    sunday: Option<String>,
    /// Template for privileged weekdays (e.g., "{month} {day}")
    privileged_weekday: Option<String>,
    /// Additional season-specific templates (e.g., octave days, special periods)
    /// Keys are season-specific identifiers.
    #[serde(flatten)]
    additional: BTreeMap<String, String>,
}
```

```json
// English
"seasons": {
  "advent": {
    "season": "Advent",
    "weekday": "{weekday} of the {ordinal} week of Advent",
    "sunday": "{ordinal} Sunday of Advent",
    "privileged_weekday": "{month} {day}"
  },
  "ordinary_time": {
    "season": "Ordinary Time",
    "weekday": "{weekday} of the {ordinal} week of Ordinary Time",
    "sunday": "{ordinal} Sunday in Ordinary Time"
  }
}

// French
"seasons": {
  "advent": {
    "season": "Avent",
    "weekday": "{weekday} de la {ordinal_feminine} semaine de l'Avent",
    "sunday": "{ordinal} dimanche de l'Avent",
    "privileged_weekday": "{day} {month}"
  }
}
```

### 3. `MartyrologyEntryDef`

**What it is:** The biographical metadata for a person, group, or event in the martyrology catalog. Stored in `martyrology/{locale}/` files, with `en` as the base locale and other locales providing sparse overrides.

```rust
struct MartyrologyEntryDef {
    // ── Type and identity ──

    /// Entry type (default: Person)
    r#type: Option<MartyrologyEntryType>,
    /// Localized full display name.
    /// When absent, constructed automatically from components
    /// (see Part III §6 for construction rules).
    fullname: Option<String>,
    /// Short name without canonization level or titles (e.g., "Adalbert")
    name: Option<String>,

    // ── Canonization ──

    /// Canonization level
    canonization_level: Option<CanonizationLevel>,
    /// Date of canonization
    date_of_canonization: Option<SaintDateDef>,
    date_of_canonization_is_approximative: Option<bool>,
    /// Date of beatification
    date_of_beatification: Option<SaintDateDef>,
    date_of_beatification_is_approximative: Option<bool>,
    /// Whether to hide the canonization level prefix in display
    /// (e.g., for "Holy Innocents" — no "Saint" prefix)
    hide_canonization_level: Option<bool>,

    // ── Titles and biographical data ──

    /// Ecclesiastical titles (TitleCategory values)
    titles: Option<Vec<TitleCategory>>,
    /// Biological sex (for gendered display: "patron" vs. "patroness")
    sex: Option<Sex>,
    /// Whether to hide titles in display
    hide_titles: Option<bool>,
    /// Title qualifiers — localized free-text modifiers per title category
    /// (see Part III §6)
    title_qualifiers: Option<BTreeMap<TitleCategory, String>>,

    // ── Dates ──

    /// Date of birth
    date_of_birth: Option<SaintDateDef>,
    date_of_birth_is_approximative: Option<bool>,
    /// Date of death
    date_of_death: Option<SaintDateDef>,
    date_of_death_is_approximative: Option<bool>,
    /// Date of dedication (for Place/Event entries)
    date_of_dedication: Option<SaintDateDef>,

    // ── Group data ──

    /// Number of persons (only for type: Group)
    count: Option<SaintCount>,

    // ── Sources ──

    /// Bibliographic references for this entry
    sources: Option<Vec<String>>,
}

enum CanonizationLevel {
    Saint,
    Blessed,
}

enum Sex {
    Male,
    Female,
}
```

**JSON example — base locale (en):**

```json
{
  "adalbert_of_prague_bishop": {
    "canonization_level": "saint",
    "name": "Adalbert",
    "titles": ["bishop", "martyr"],
    "title_qualifiers": {
      "bishop": "of Prague"
    },
    "date_of_death": 997
  },
  "holy_innocents_martyrs": {
    "type": "group",
    "fullname": "The Holy Innocents, Martyrs",
    "hide_canonization_level": true,
    "titles": ["martyr"],
    "count": "many"
  },
  "dedication_of_the_lateran_basilica": {
    "type": "event",
    "fullname": "Dedication of the Lateran Basilica"
  }
}
```

**JSON example — French locale override (sparse):**

```json
{
  "adalbert_of_prague_bishop": {
    "fullname": "Saint Adalbert, évêque de Prague et martyr († 997)"
  },
  "holy_innocents_martyrs": {
    "fullname": "Les Saints Innocents, martyrs"
  }
}
```

The French locale overrides only `fullname` — all other fields (titles, dates, type) are inherited from `en`.

### 4. `MartyrologyEntryType`

**What it is:** Classifies a martyrology entry as a person, a group of persons, or a non-person event.

```rust
enum MartyrologyEntryType {
    /// A single individual (saint, blessed, angel, Mary...)
    Person,
    /// A group of individuals (Holy Innocents, Companions of...)
    /// The number is stored in MartyrologyEntryDef.count.
    Group,
    /// A non-person celebration (Dedication of a Basilica,
    /// Exaltation of the Holy Cross, Finding of the Holy Cross...)
    Event,
}
```

**Default:** When `type` is absent, the entry defaults to `Person`.

**Role in Common resolution:** The engine uses `type` and `count` (for `Group`) to resolve `CommonDef` → `Common`. A `Group` with `count > 1` (or `Many`) resolves to "Several" variants (e.g., `Martyrs_OutsideEaster_Several`). A `Person` resolves to "One" variants.

### 5. `SaintDateDef`

**What it is:** A flexible date specification for biographical dates (birth, death, canonization, etc.). Supports various precision levels, ranges, alternatives, and century-only specifications.

```rust
/// A date with variable precision
enum SaintDate {
    /// Year only (e.g., 997)
    Year(u32),
    /// Year and month (e.g., "1171-03")
    YearMonth(String),
    /// Full date (e.g., "1171-03-29")
    YearMonthDay(String),
}

/// Date specification with uncertainty support
enum SaintDateDef {
    /// Single date at any precision
    Date(SaintDate),
    /// Date range (e.g., born between 1200 and 1210)
    Between { between: [SaintDate; 2] },
    /// Multiple alternative dates (e.g., died in 1100 or 1101)
    Or { or: Vec<SaintDate> },
    /// Century specification (e.g., 5th century)
    Century { century: u32 },
}
```

**JSON serialization:** `SaintDateDef` is an untagged enum:

```json
// Year only
"date_of_death": 997

// Full date
"date_of_death": "1171-03-29"

// Century
"date_of_death": { "century": 5 }

// Range
"date_of_birth": { "between": [1200, 1210] }

// Alternatives
"date_of_death": { "or": [1100, 1101] }
```

The `_is_approximative` companion fields (e.g., `date_of_death_is_approximative: true`) indicate whether the date is approximate (prefixed with "ca." or "~" in display).

### 6. Title Qualifiers and Fullname Construction

#### Title qualifiers

A **qualifier** is a localized free-text string that **replaces** the base title display when present. It is attached to a `TitleCategory` key in `MartyrologyEntryDef.title_qualifiers`. Qualifiers distinguish specific variants of a title without adding enum variants to the code.

The qualifier is the **complete rendered text** of the qualified title — not a fragment to be composed with the base title via a template. This avoids word-order issues across languages (e.g., "the First Martyr" in English vs. "premier martyr" in French vs. "proto-martyr d'Océanie").

```json
// martyrology/en/s.json
"stephen_the_first_martyr": {
  "name": "Stephen",
  "titles": ["martyr"],
  "title_qualifiers": {
    "martyr": "the First Martyr"
  }
}
// Display: "Saint Stephen, the First Martyr"

// martyrology/en/c.json
"cyril_of_thessaloniki_monk": {
  "name": "Cyril",
  "titles": ["monk", "missionary"],
  "title_qualifiers": {
    "missionary": "Slavic Missionary"
  }
}
// Display: "Saint Cyril, Monk and Slavic Missionary"

// martyrology/en/p.json
"peter_chanel_priest": {
  "name": "Peter Chanel",
  "titles": ["priest", "martyr"],
  "title_qualifiers": {
    "martyr": "Proto-martyr of Oceania"
  }
}
// Display: "Saint Peter Chanel, Priest and Proto-martyr of Oceania"
```

Each locale provides its own qualifiers — the string is the final rendered form in that language:

```json
// martyrology/fr/s.json
"stephen_the_first_martyr": {
  "title_qualifiers": {
    "martyr": "premier martyr"
  }
}
// Display: "Saint Étienne, premier martyr"

// martyrology/fr/p.json
"peter_chanel_priest": {
  "title_qualifiers": {
    "martyr": "proto-martyr d'Océanie"
  }
}
// Display: "Saint Pierre Chanel, prêtre et proto-martyr d'Océanie"
```

**Fallback:** When a qualifier is absent for a given locale, the standard BCP-47 locale hierarchy applies (e.g., `fr` → `en`). The engine falls back to the `en` qualifier, not to the base title. This preserves the specificity of the qualified title even if the translation is incomplete. The `locale_tags` field in `CalendarMetadata` enables audit tools to detect missing translations before they reach production (see Part II §2).

**Scope:** Qualifiers are rare — roughly 10–15 entries out of several hundred in the martyrology. Most entries have no qualifier and use the base title from `title_categories` directly.

#### Fullname construction

When `fullname` is absent, the engine constructs it from components using **locale-specific templates** in `ResourcesMetadata.fullname_templates`:

```rust
struct FullnameTemplates {
    /// Template for person entries
    /// Placeholders: {canonization_level}, {name}, {titles}
    person: String,
    /// Template for group entries
    /// Placeholders: {canonization_level}, {name}, {titles}, {count}
    group: Option<String>,
    /// Separator between multiple titles (e.g., ", " or " and ")
    title_separator: String,
    /// Conjunction before the last title (e.g., " and ")
    title_last_conjunction: String,
}
```

```json
// English templates
"fullname_templates": {
  "person": "{canonization_level} {name}, {titles}",
  "title_separator": ", ",
  "title_last_conjunction": " and "
}

// French templates
"fullname_templates": {
  "person": "{canonization_level} {name}, {titles}",
  "title_separator": ", ",
  "title_last_conjunction": " et "
}
```

**Construction example (English):**

Input components:

- `canonization_level`: "saint" → "Saint"
- `name`: "Adalbert"
- `titles`: ["bishop", "martyr"]
- `title_qualifiers`: { "bishop": "Bishop of Prague" }

Steps:

1. For each title, check `title_qualifiers`: if a qualifier exists, use it directly; otherwise look up the base title in `title_categories`.
   - "bishop" → qualifier found: **"Bishop of Prague"**
   - "martyr" → no qualifier, base title: **"Martyr"**
2. Join titles: "Bishop of Prague" + " and " + "Martyr" → "Bishop of Prague and Martyr"
3. Apply person template: "Saint Adalbert, Bishop of Prague and Martyr"

Result: `"Saint Adalbert, Bishop of Prague and Martyr"`

**Override:** When the automatic construction is inadequate (complex word order, exceptional formatting), the contributor provides `fullname` explicitly, which takes precedence over the construction.

### 7. Locale Inheritance

Locale resolution follows **BCP-47 tag hierarchy** with `en` as the universal base:

```
"en-gb" → merge hierarchy: ["en", "en-gb"]
"fr"    → merge hierarchy: ["en", "fr"]
"pt-br" → merge hierarchy: ["en", "pt-br"]
"zh-Hant-TW" → merge hierarchy: ["en", "zh", "zh-Hant", "zh-Hant-TW"]
```

**Merge algorithm:**

1. Load `en` as the complete base (all fields populated).
2. For each subsequent locale in the hierarchy, merge its fields on top:
   - For `ResourcesMetadata`: each field overrides the corresponding `en` field.
   - For `MartyrologyEntryDef`: each entry's fields override the corresponding `en` entry's fields. Entries not present in the override locale inherit entirely from `en`.

**Why `en` as base:** English serves as the development lingua franca and provides the most complete dataset. Using a language-neutral base (like `la`) was considered but rejected because Latin names (e.g., "Sanctus Adalbertus, Episcopus et Martyr") are less familiar to most contributors and would make editing harder.

**Sparse override example (`en-gb`):**

```json
// locales/en-gb.json — empty, inherits everything from en
{
  "locale_tag": "en-gb",
  "metadata": {
    "seasons": {},
    "periods": {},
    "ranks": {}
  }
}

// martyrology/en-gb/l.json — only British spelling differences
{
  "locale_tag": "en-gb",
  "martyrology": {
    "labor_day": {
      "fullname": "Labour Day"
    }
  }
}
```

### 8. Localization of Enums

Several input enums require localized display names. These are stored in `ResourcesMetadata`:

| Enum                | Metadata field        | Example (en)                                 |
| ------------------- | --------------------- | -------------------------------------------- |
| `Season`            | `seasons[].season`    | `"Ordinary Time"`                            |
| `Rank`              | `ranks`               | `{ "memorial": "memorial" }`                 |
| `Color`             | `colors`              | `{ "red": "red", "white": "white" }`         |
| `TitleCategory`     | `title_categories`    | `{ "bishop": "Bishop", "martyr": "Martyr" }` |
| `PatronRole`        | `patronage_roles`     | `{ "principal_patron": "Patron" }`           |
| `Common`            | `commons`             | `{ "martyrs_one": "Common of One Martyr" }`  |
| `CanonizationLevel` | `canonization_levels` | `{ "saint": "Saint", "blessed": "Blessed" }` |
| `MassTime`          | `mass_times`          | `{ "day_mass": "Day Mass" }`                 |
| `Period`            | `periods`             | `{ "easter_octave": "Easter Octave" }`       |

These localized names are used both in output types (e.g., `CommonInfo.name`, `ColorInfo.name`) and for fullname construction.

---

## Part IV — Tier 3: Liturgical Texts

Tier 3 provides the actual text content of liturgical celebrations: prayers, antiphons, readings, psalmody, and Office elements. These texts come from published liturgical books (Roman Missal, Lectionary, Liturgy of the Hours) and may be under copyright.

### 1. Overview and Copyright Considerations

**What Tier 3 contains:**

| Category                  | Source book               | Content                                                    |
| ------------------------- | ------------------------- | ---------------------------------------------------------- |
| Mass formulary texts      | Roman Missal              | Collects, entrance antiphons, communion antiphons          |
| Mass orations             | Roman Missal              | Prayer over the offerings, prayer after communion, preface |
| Mass readings (full text) | Lectionary                | Biblical passages, psalms, alleluia verses                 |
| Common Mass texts         | Roman Missal + Lectionary | Formulary and reading pools per Common category            |
| Office proper texts       | Liturgy of the Hours      | Hymns, antiphons, short readings, intercessions            |
| Office readings           | Liturgy of the Hours      | Patristic readings, hagiographical readings                |
| Common Office texts       | Liturgy of the Hours      | Pool of texts per Common category, per Hour                |

**Copyright status:**

- The **Latin _editio typica_** (Roman Missal 3rd edition, 2002/2008; Liturgia Horarum, 1970-1971/1985-1986) is published by the Libreria Editrice Vaticana. While the Holy See holds copyright, the texts are increasingly available digitally.
- **Vernacular translations** are copyrighted by the respective national Bishops' Conferences (e.g., ICEL for English, AELF for French).
- **Biblical text** copyright depends on the translation (NRSV, NAB, Jerusalem Bible, Vulgate...). The Latin Vulgate (_Nova Vulgata_, 1979) is published by the Vatican.
- **Scripture citations** (reference strings like "Isa 2:1-5") are factual and not copyrighted — they are in Tier 1.

**Consequence:** Tier 3 data lives in a **separate repository** or is provided as an external package. The romcal core repository defines the Tier 3 **schemas and types** but does not ship text data. This allows different text packages (Latin, English, French) to be distributed independently under their respective licenses.

### 2. `ProperMassTexts`

**What it is:** Mass-specific texts for celebrations that have proper (non-Common) content. Organized by celebration ID and mass time.

This type covers the three GIRM substitution groups (see companion document Part I §2):

- **Group 1 — Formulary:** collect, entrance antiphon, communion antiphon
- **Group 3 — Flexible orations:** prayer over the offerings, prayer after communion, preface, solemn blessing, prayer over the people

Group 2 (readings) is handled separately in `ReadingsTexts` (§4), because readings are keyed by citation string, not by celebration ID.

```rust
/// Root type for a proper Mass texts file
struct ProperMassTextsFile {
    schema: Option<String>,
    locale_tag: LocaleTag,
    /// Texts keyed by CelebrationId (same ID as in CalendarDef)
    texts: BTreeMap<CelebrationId, CelebrationMassTexts>,
}

/// Mass texts for one celebration
struct CelebrationMassTexts {
    /// Canonical prayer (CP 44) — the collect that serves as both
    /// Mass collect and Office concluding prayer. Present when the
    /// celebration has a single collect shared across all Mass times.
    /// When absent, each MassTime provides its own collect.
    prayer: Option<String>,
    /// Texts per Mass time
    masses: BTreeMap<MassTime, MassTimeTexts>,
}

/// An antiphon with optional biblical source reference(s).
/// GIRM §48 (entrance antiphon): "its purpose is to [...] accompany the
/// procession of the Priest and ministers"; GIRM §87 (communion antiphon):
/// "its purpose is to express the spiritual union of the communicants."
struct AntiphonTextDef {
    /// Text of the antiphon
    text: String,
    /// Biblical source reference(s). Entrance and communion antiphons in the
    /// Roman Missal often derive from Scripture (e.g., "cf. Ps 24:1-3").
    sources: Option<Vec<SourceRef>>,
}

/// Texts for one Mass time of one celebration
struct MassTimeTexts {
    // ── Group 1: Formulary block ──
    /// Collect override for this specific Mass time.
    /// When absent, the engine uses CelebrationMassTexts.prayer (CP 44).
    collect: Option<String>,
    entrance_antiphon: Option<AntiphonTextDef>,   // GIRM §48
    communion_antiphon: Option<AntiphonTextDef>,  // GIRM §87

    // ── Group 3: Flexible orations ──
    prayer_over_the_offerings: Option<String>,
    prayer_after_communion: Option<String>,
    preface: Option<PrefaceRef>,                  // GIRM §364-365
    solemn_blessing: Option<String>,
    prayer_over_the_people: Option<String>,

    /// Rubrical or pastoral note printed in the liturgical book for this
    /// Mass time (e.g., "Nine readings are proposed...", "One may also
    /// read...", "If there is baptism..."). GILM §15 establishes that
    /// the Lectionary includes pastoral and rubrical notes to guide the
    /// celebrant; GILM §117 provides further guidance on annotations
    /// within the readings for weekdays.
    note: Option<String>,
}

/// Preface — either inline text or a reference to the catalog
enum PrefaceRef {
    /// Inline preface text (for unique proper prefaces)
    Inline(String),
    /// Reference to a preface catalog entry by ID
    CatalogId(String),
}
```

> **Design note — `prayer` as canonical prayer (CP 44):** CP 44 states that the Office concluding prayer is "the same as the collect of the Mass." This identity is modeled by storing the shared prayer once in `CelebrationMassTexts.prayer`. Most celebrations have a single collect; the `MassTimeTexts.collect` field exists only as an override for multi-Mass celebrations (e.g., Christmas has 4 distinct collects for Vigil, Night, Dawn, and Day). The engine resolves the effective collect as: `mass_time_texts.collect` if present, otherwise `celebration_mass_texts.prayer`. See the companion document (Part III §5) for the full analysis.

**JSON example — Proper of Saints (January):**

```json
{
  "$schema": "../../schemas/liturgical_texts.json",
  "locale_tag": "la",
  "texts": {
    "basil_the_great_and_gregory_nazianzen_bishops": {
      "prayer": "Deus, qui Ecclesiæ tuæ beatos Basilium et Gregorium...",
      "masses": {
        "day_mass": {
          "entrance_antiphon": {
            "text": "Os iusti meditabitur sapientiam...",
            "sources": [{ "reference": "Ps 36:30-31", "confer": true }]
          },
          "communion_antiphon": {
            "text": "Fidelis servus et prudens...",
            "sources": [{ "reference": "Mt 24:46-47", "confer": true }]
          },
          "prayer_over_the_offerings": "Sanctorum tuorum nobis, Domine...",
          "prayer_after_communion": "Deus, qui nos cælésti alimónia...",
          "preface": { "catalog_id": "common_preface_1" }
        }
      }
    }
  }
}
```

### 3. `CommonMassTexts`

**What it is:** Pools of Mass texts organized by `Common` variant. When a celebration has no proper text for an element, the celebrant draws from the applicable Common pool.

```rust
/// Root type for a Common Mass texts file
struct CommonMassTextsFile {
    schema: Option<String>,
    locale_tag: LocaleTag,
    /// Pools keyed by the full Common enum variant
    commons: BTreeMap<Common, CommonMassPool>,
}

/// Pool of Mass texts for one Common
struct CommonMassPool {
    /// Multiple formulary sets to choose from
    formularies: Vec<CommonFormularySet>,
    /// Flexible orations pool (prayer over offerings, prayer after communion)
    orations: Vec<CommonOrations>,
    /// Preface pool
    prefaces: Vec<PrefaceRef>,
}

/// One formulary option within a Common
struct CommonFormularySet {
    /// Identifier for this formulary within the Common (e.g., "A", "B", "1")
    id: Option<String>,
    collect: String,
    entrance_antiphon: AntiphonTextDef,
    communion_antiphon: AntiphonTextDef,
    prayer_over_the_offerings: Option<String>,
    prayer_after_communion: Option<String>,
}

/// One set of flexible orations
struct CommonOrations {
    prayer_over_the_offerings: String,
    prayer_after_communion: String,
}
```

**Liturgical basis:** The Roman Missal provides multiple formularies per Common (e.g., the Common of Martyrs has formulary sets A, B, C). GIRM 363: "The prayer over the offerings, however, and the prayer after Communion, unless they are proper, may be taken either from the Common or from the weekdays of the current Season."

### 3b. `PrefaceCatalog`

**What it is:** A catalog of preface texts, keyed by ID. Most prefaces are reused across many celebrations (~62 prefaces in the Roman Missal, reused across hundreds of celebrations). A catalog with IDs avoids duplication. Celebrations with unique proper prefaces (e.g., Easter Day Mass) use the inline `PrefaceRef::Inline` variant instead.

```rust
/// A preface entry in the catalog (GIRM §364-365)
struct PrefaceDef {
    /// Unique identifier (e.g., "advent_1", "common_preface_1")
    id: String,
    /// Theme/title of the preface (e.g., "The two comings of Christ")
    headline: String,
    /// Usage note (e.g., "Used during Advent, from the 1st Sunday to December 16")
    note: Option<String>,
    /// Applicable Commons (when this preface belongs to a Common pool)
    commons: Option<Vec<Common>>,
    /// The full text of the preface
    text: String,
}

/// Root type for a preface catalog file
struct PrefaceCatalogFile {
    schema: Option<String>,
    locale_tag: LocaleTag,
    prefaces: Vec<PrefaceDef>,
}
```

**`PrefaceRef` — inline or catalog reference:** The `preface` field in `MassTimeTexts` and `CommonMassPool` uses the `PrefaceRef` enum (defined above in §2) to support both inline text (for unique proper prefaces) and catalog references (for reused prefaces). The engine resolves `PrefaceRef::CatalogId` by looking up the ID in the preface catalog.

### 4. `ReadingsTexts`

**What it is:** Full biblical text content, keyed by citation string. Provides the text that corresponds to the reading citations stored in Tier 1 `MassReadingsDef`. Readings and psalms are stored in separate maps because they have different structures.

```rust
/// Root type for a readings text file
struct ReadingsTextsFile {
    schema: Option<String>,
    locale_tag: LocaleTag,
    /// Biblical readings keyed by citation string (readings, epistles, gospels).
    /// Alleluia verses and sequences also use this map — see jointure (§8).
    readings: BTreeMap<String, ReadingTextDef>,
    /// Responsorial psalms and canticles keyed by citation string.
    /// Separate from readings because psalms carry an antiphon, not a headline.
    psalms: BTreeMap<String, PsalmodyEntryDef>,
}

/// Full text of a biblical reading
struct ReadingTextDef {
    /// Full citation reference (e.g., "Isa 2:1-5"). Redundant with the map key
    /// in ReadingsTextsFile, but carried explicitly for serialization convenience
    /// and to ensure the reference survives when the entry is extracted from the map.
    reference: Option<String>,
    /// Pericope headline — the summary title printed above the reading
    /// in the Lectionary (e.g., "The Lord gathers the nations in eternal peace").
    /// GILM §123: "In the text of each reading, a heading (titulus) is given
    /// [...] to indicate the main theme of the reading." Standard editorial
    /// element; present for every reading in printed books.
    headline: Option<String>,
    /// The full text of the reading
    text: String,
    /// Abbreviated reference for compact display (e.g., "2, 1-5").
    /// Distinct from the full citation reference.
    ref_abbr: Option<String>,
    /// Optional short form variant (GIRM §360; GILM §75, §80)
    short_form: Option<ShortFormDef>,
    /// When true, the reading concludes without the standard final acclamation
    /// ("Word of the Lord" / "The Gospel of the Lord").
    /// GILM §125: the final acclamation is omitted for Passion narratives;
    /// PS §33 (Palm Sunday) and PS §70 (Easter Vigil OT readings) specify
    /// the same omission in their respective rites.
    /// Option in input (omit = false); the engine resolves to `bool` in output.
    no_final_acclamation: Option<bool>,
}

/// Short form of a reading or psalm (GIRM §360; GILM §75, §77, §80)
struct ShortFormDef {
    /// Citation reference for the short form (e.g., "Gen 1:1.26-31a")
    reference: Option<String>,
    /// Abbreviated reference for display
    ref_abbr: Option<String>,
    /// The text of the short form
    text: String,
}
```

> **Design note — `headline`:** The pericope headline (_titulus_) is a standard Lectionary editorial element defined by GILM §123. It is not to be confused with the reading's biblical reference — rather, it summarizes the theme of the passage, e.g., "Paul's rapture to the third heaven" for 2 Cor 12:1-10.
> **Design note — `short_form` as struct:** `short_form` is a struct rather than a plain string because the Lectionary prints both the short form's own citation reference and its text (e.g., "Short reading: Gen 1:1.26-31a"). The struct carries both.

**Citation string format:** The citation strings used as keys match exactly the strings stored in Tier 1 `MassReadingsDef` (e.g., `"Isa 2:1-5"`, `"Ps 122:1-2,3-4ab,4cd-5,6-7,8-9"`, `"Matt 24:37-44"`). The engine performs a direct key lookup to join citations with texts.

**File organization:** Reading texts can be organized by book of the Bible for manageability:

- `readings/old_testament.json` — Genesis through Malachi
- `readings/new_testament.json` — Matthew through Revelation
- `readings/psalms.json` — All psalms and canticles

#### `SourceRef` — Biblical source reference

A shared helper type used across content types (Mass antiphons, psalm antiphons, alleluia verses) to track the biblical origin of a liturgical text.

```rust
/// A biblical source reference with optional "cf." qualifier
struct SourceRef {
    /// Biblical reference string (e.g., "Ps 24:1-3", "1 Cor 5:7b-8a")
    reference: String,
    /// When true, the source is a comparative reference (printed as "cf." in
    /// liturgical books). Distinguishes direct citation from allusion.
    confer: Option<bool>,
}
```

> **Design note — no `*Def` suffix:** `SourceRef` is a shared helper type used identically in both input and output, like `LocaleTag` or `MassTime`. It carries no transformation semantics, so it does not follow the `*Def` → output naming convention.

### 5. `ProperHoursTexts`

**What it is:** Office-specific texts for celebrations that have proper content for the Liturgy of the Hours. Organized by celebration ID and Hour.

```rust
/// Root type for a proper Office texts file
struct ProperHoursTextsFile {
    schema: Option<String>,
    locale_tag: LocaleTag,
    texts: BTreeMap<CelebrationId, CelebrationHoursTexts>,
}

/// Office texts for one celebration
struct CelebrationHoursTexts {
    /// Texts per Hour
    hours: BTreeMap<HourTime, HourTexts>,
}

/// Texts for one Hour of one celebration
struct HourTexts {
    // ── Elements from GILH 235b ──
    invitatory_antiphon: Option<String>,
    hymn: Option<String>,
    short_reading: Option<String>,
    short_responsory: Option<String>,
    /// Gospel canticle antiphon (Benedictus at Lauds, Magnificat at Vespers,
    /// Nunc Dimittis at Compline)
    canticle_antiphon: Option<String>,
    intercessions: Option<String>,

    /// Concluding prayer override for this specific Hour.
    /// When absent, the engine uses CelebrationMassTexts.prayer (CP 44).
    concluding_prayer: Option<String>,

    /// Psalmody — rare on memorials (GILH 235a: usually from weekday)
    /// Populated on solemnities with proper psalms.
    psalmody: Option<Vec<PsalmodyEntryDef>>,

    /// Office of Readings content (only for HourTime::OfficeOfReadings)
    office_of_readings: Option<OfficeReadingsTextsDef>,
}

/// One psalmody entry (psalm/canticle + antiphon variants).
/// GILM §19-20: "The responsorial psalm [...] is an integral part of the
/// Liturgy of the Word"; GILM §22: "The psalm after the reading is sung
/// or recited by the psalmist or cantor [...]." GIRM §61 further specifies
/// it is "a response to the first reading."
/// Used for both Mass responsorial psalms (in ReadingsTexts) and
/// Office psalmody (in ProperHoursTexts/CommonHoursTexts).
struct PsalmodyEntryDef {
    /// Psalm or canticle reference (e.g., "Ps 63", "Dan 3:57-88")
    reference: String,
    /// Abbreviated reference for display (e.g., "103, 1-2a…")
    ref_abbr: Option<String>,
    /// The psalm/canticle text. None when the entry is citation-only
    /// (the citation is still useful for display or external lookup,
    /// even when the full text is not distributed — see §7 copyright model).
    text: Option<String>,
    /// Responsorial antiphon variants. Multiple entries when different
    /// liturgical cycles use different refrains for the same psalm text
    /// (e.g., Year A vs Year B antiphon for Ps 122); GILM §21: "as a
    /// rule, the response is drawn from the text of the psalm itself."
    /// GILM §89 allows choosing the psalm from either the proper
    /// lectionary or the Common. The engine selects the applicable
    /// variant based on Tier 1 cycle context, producing a single
    /// resolved antiphon in the output.
    antiphons: Vec<PsalmAntiphonDef>,
    /// Optional short form — abbreviated psalm for pastoral use (GILM §77)
    short_form: Option<ShortFormDef>,
}

/// Responsorial antiphon with optional biblical source(s)
struct PsalmAntiphonDef {
    /// Text of the antiphon/refrain
    text: String,
    /// Biblical source reference(s) for the antiphon text
    sources: Option<Vec<SourceRef>>,
}

/// Office of Readings texts
struct OfficeReadingsTextsDef {
    /// 1st reading: Scripture
    scripture_reading: Option<ReadingTextDef>,
    scripture_responsory: Option<String>,
    /// 2nd reading: patristic
    patristic_reading: Option<ReadingTextDef>,
    patristic_responsory: Option<String>,
    /// Hagiographical reading (saint's life)
    hagiographical_reading: Option<ReadingTextDef>,
    hagiographical_responsory: Option<String>,
    /// Biographical note (CP 43, GILH 168) — informational, not read aloud
    biographical_note: Option<String>,
}
```

**`HourTime` — the Hours of the Office:**

```rust
enum HourTime {
    VespersI,          // Evening Prayer I (solemnities — starts the evening before)
    OfficeOfReadings,
    Lauds,             // Morning Prayer
    Terce,             // Mid-morning Prayer
    Sext,              // Midday Prayer
    Nones,             // Mid-afternoon Prayer
    Vespers,           // Evening Prayer (II on solemnities, regular otherwise)
    Compline,          // Night Prayer
}
```

**What a memorial provides vs. what a solemnity provides:**

| Element                | Solemnity/Feast | Memorial                   | Weekday   |
| ---------------------- | --------------- | -------------------------- | --------- |
| Psalmody               | Proper          | Weekday (GILH 235a)        | Weekday   |
| Invitatory antiphon    | Proper          | Optional (GILH 235b)       | Weekday   |
| Hymn                   | Proper          | Optional (GILH 235b)       | Weekday   |
| Short reading          | Proper          | Optional (GILH 235b)       | Weekday   |
| Canticle antiphon      | Proper          | From saint (GILH 235b)     | Weekday   |
| Intercessions          | Proper          | Optional (GILH 235b)       | Weekday   |
| Concluding prayer      | Proper          | From saint (GILH 235c)     | Weekday   |
| Office of Readings 1st | Proper          | Weekday (GILH 235d)        | Weekday   |
| Office of Readings 2nd | Proper          | Hagiographical (GILH 235d) | Patristic |

The engine uses the celebration's `Rank` (derived from `Precedence`) to determine which elements are taken from the saint's proper texts and which from the weekday. In Tier 3, the proper texts file simply provides whatever texts exist — the engine handles the selection logic.

### 6. `CommonHoursTexts`

**What it is:** Pools of Office texts organized by `Common` variant, paralleling `CommonMassTexts` for the Mass.

```rust
/// Root type for a Common Office texts file
struct CommonHoursTextsFile {
    schema: Option<String>,
    locale_tag: LocaleTag,
    commons: BTreeMap<Common, CommonHoursPool>,
}

/// Pool of Office texts for one Common
struct CommonHoursPool {
    /// Texts per Hour
    hours: BTreeMap<HourTime, CommonHourTexts>,
}

/// Pool of texts for one Hour within one Common
struct CommonHourTexts {
    invitatory_antiphons: Vec<String>,
    hymns: Vec<String>,
    short_readings: Vec<String>,
    short_responsories: Vec<String>,
    canticle_antiphons: Vec<String>,
    intercessions: Vec<String>,
    concluding_prayers: Vec<String>,
    /// Psalmody options (for solemnities/feasts that draw from the Common)
    psalmody_options: Vec<Vec<PsalmodyEntryDef>>,
    /// Office of Readings texts
    office_of_readings: Option<CommonOfficeReadingsPool>,
}

/// Pool of Office of Readings texts for one Common
struct CommonOfficeReadingsPool {
    scripture_readings: Vec<ReadingTextDef>,
    scripture_responsories: Vec<String>,
    patristic_readings: Vec<ReadingTextDef>,
    patristic_responsories: Vec<String>,
    hagiographical_readings: Vec<ReadingTextDef>,
    hagiographical_responsories: Vec<String>,
}
```

**Liturgical basis:** GILH 235b: "at Morning Prayer and Evening Prayer [...] from the Proper or from the Common." The Common provides pools of alternative texts for each Office element, from which the celebrant may choose.

### 7. Latin Baseline, Representative Examples, and Copyright

#### Latin as structural reference

The `la` (Latin) locale serves as the **structural reference baseline** for Tier 3, paralleling `en` for Tier 2:

| Tier | Base locale | Rationale                                                                     |
| ---- | ----------- | ----------------------------------------------------------------------------- |
| 2    | `en`        | Most accessible for contributors; complete biographical dataset               |
| 3    | `la`        | _Editio typica_ is the official reference; all translations derive from Latin |

The `la` locale identifies all text slots that exist in the _editio typica_. It serves as a structural skeleton — ensuring that the types, file organization, and keying conventions are complete and correct — rather than as a redistribution of copyrighted texts.

The merge hierarchy for Tier 3 follows the same BCP-47 pattern:

- `en` → merge hierarchy: `["la", "en"]`
- `fr` → merge hierarchy: `["la", "fr"]`

In practice, most fields in Tier 3 are fully overridden by vernacular locales (since the actual text is language-specific). The Latin baseline serves as:

1. A structural reference — ensuring all text slots are identified.
2. A fallback — displaying the Latin original when no vernacular translation exists.
3. A scholarly reference — for liturgists comparing translations with the _editio typica_.

#### Representative examples

romcal ships Tier 3 data for a small set of representative dates that illustrate every structural pattern in the data model. The target dates cover the full range of Tier 3 structures:

| Date                            | Why included                                                                                   |
| ------------------------------- | ---------------------------------------------------------------------------------------------- |
| 1st Sunday of Advent (Year A)   | Start of liturgical year, Sunday cycle, short form readings                                    |
| Nativity of the Lord (Day Mass) | Multi-Mass celebration, proper preface                                                         |
| Easter Sunday                   | Vigil with 7 OT readings (VigilSequence), sequence hymn, canticles                             |
| Pentecost                       | Sequence, proper preface                                                                       |
| 15th Sunday of Ordinary Time    | Standard Sunday cycle, responsorial psalm                                                      |
| 18th Sunday + Mon + Tue of OT   | Layered cycle resolution: weekday Gospel depends on Sunday cycle (§13 example), rubrical notes |
| Ascension                       | Feast of the Lord, proper readings                                                             |
| Immaculate Conception           | Marian solemnity, BVM Commons                                                                  |

These examples are sufficient to validate the entire type system and jointure pipeline. They are not intended as a production-ready data set.

#### Private completion model

Tier 3 is architecturally designed to hold a complete Lectionary and Roman Missal, but romcal itself distributes only the illustrative examples listed above. Users who have access to the full texts (personal copy, publisher agreement) can populate their own private Tier 3 data. The types, file organization, and keying conventions are ready for full coverage.

#### Copyright status by content category

| Category                                                       | Examples                                       | Copyright status                                                                  |
| -------------------------------------------------------------- | ---------------------------------------------- | --------------------------------------------------------------------------------- |
| Structural metadata (references, slot names)                   | `"Isa 2:1-5"`, `reading_1`, `psalm`            | No concern — factual data                                                         |
| Editorial metadata (headlines, abbreviated references)         | `"The Lord gathers the nations..."`, `"2,1-5"` | Low concern — factual, editorial, or _de minimis_                                 |
| Full liturgical texts (readings, prayers, prefaces, antiphons) | Prayer texts, Lectionary passages              | Subject to publisher copyright (Holy See, ICEL, AELF, etc.) in most jurisdictions |

### 8. Citation → Text Jointure

The engine joins reading citations (Tier 1) with reading texts (Tier 3) at runtime, using the appropriate map based on the reading slot:

```
Tier 1 (MassReadingsDef)              Tier 3 (ReadingsTextsFile)
─────────────────────────              ──────────────────────────

"reading_1": "Isa 2:1-5"        ──→   readings["Isa 2:1-5"]         → output ReadingText
"psalm": "Ps 122:1-2,..."       ──→   psalms["Ps 122:1-2,..."]      → output PsalmodyEntry
"alleluia": "Ps 84:8"           ──→   readings["Ps 84:8"]           → output AlleluiaText (+ computed AcclamationType)
"sequence": "Victimae paschali"  ──→   readings["Victimae paschali"] → output SequenceText
"gospel": "Matt 24:37-44"       ──→   readings["Matt 24:37-44"]     → output ReadingText
```

**Which map for which slot:**

| Tier 1 slot | Tier 3 map | Output type     | Notes                                               |
| ----------- | ---------- | --------------- | --------------------------------------------------- |
| `reading_1` | `readings` | `ReadingText`   | Direct mapping                                      |
| `reading_2` | `readings` | `ReadingText`   | Direct mapping                                      |
| `psalm`     | `psalms`   | `PsalmodyEntry` | Different structure: antiphon instead of headline   |
| `canticle`  | `psalms`   | `PsalmodyEntry` | Same structure as psalms                            |
| `alleluia`  | `readings` | `AlleluiaText`  | Engine computes `AcclamationType` from season       |
| `sequence`  | `readings` | `SequenceText`  | Only 4 in the entire year (Easter, Pentecost, etc.) |
| `gospel`    | `readings` | `ReadingText`   | Direct mapping                                      |

**Resolution rules:**

1. The engine identifies the target map from the reading slot type: `psalm`/`canticle` slots look up in `psalms`, all other slots look up in `readings`.
2. If found, the `ReadingTextDef` or `PsalmodyEntryDef` populates the corresponding output type. For alleluia verses, the engine additionally computes the `AcclamationType` from the liturgical season. For sequences, the engine wraps the text into `SequenceText`.
3. If not found (Tier 3 absent or citation not yet populated), the output carries the citation string without full text. The citation itself is still useful to consumers (e.g., for display or for looking up the text in a Bible application).

**Citation format conventions:** Citation strings follow a standardized format based on the OSIS (Open Scripture Information Standard) abbreviations:

- Book abbreviation + space + chapter:verses (e.g., `"Isa 2:1-5"`)
- Verse ranges with `-` (e.g., `"1-5"`)
- Non-contiguous verses with `,` (e.g., `"1-2,4-5"`)
- Sub-verse letters for partial verses (e.g., `"4ab"`, `"14a"`)
- Cross-chapter with `;` (e.g., `"Isa 63:16b-17; 64:2-7"`)
- Em-dash for chapter:verse–chapter:verse spans (e.g., `"1 Thess 3:12—4:2"`)

---

## Part V — Input → Output Transformation

This section describes how input types (Tiers 1-3) map to the output types defined in the companion document. The transformation pipeline is described in the companion document (Part V §2); this section focuses on the **type-level mappings**.

### 1. Overview

```
Tier 1 (CalendarDef)
    │
    ├── CelebrationDef.date_def          → Celebration attachment to LiturgicalDay.date
    ├── CelebrationDef.precedence        → Celebration.precedence + Celebration.rank
    ├── CelebrationDef.commons_def       → Celebration.commons (after expansion)
    ├── CelebrationDef.titles            → Celebration.titles (after merge with Tier 2)
    ├── CelebrationDef.patronages        → Celebration.patronages (after localization)
    ├── CelebrationDef.martyrology       → Celebration.martyrology (after Tier 2 merge)
    ├── CelebrationDef.masses            → CelebrationMass.readings (citations)
    └── CalendarDef.hierarchy    → Celebration.from_calendar_id + LiturgicalDay.parent_overrides

Tier 2a (martyrology/)
    │
    ├── MartyrologyEntryDef             → MartyrologyEntry (merged across locales)
    └── Locale merge                    → All localized martyrology fields

Tier 2b (locales/)
    │
    ├── ResourcesMetadata               → DayContext.*_name, ColorInfo.name, CommonInfo.name, etc.
    └── Locale merge                    → All localized UI strings

Tier 3 (Liturgical Texts)
    │
    ├── ProperMassTexts                 → CelebrationMass.formulary + flexible_orations
    ├── CommonMassTexts                 → ReadingsPool, FlexibleOrations (alternatives)
    ├── PrefaceCatalog                  → PrefaceText (resolved by PrefaceRef)
    ├── ReadingsTexts.readings          → ReadingText, AlleluiaText, SequenceText
    ├── ReadingsTexts.psalms            → PsalmodyEntry (Mass responsorial psalms)
    ├── ProperHoursTexts                → CelebrationHour (per-Hour elements)
    └── CommonHoursTexts                → CelebrationHour (fallback elements)
```

**Tier 3 type-level transformation detail:**

| Input type (Tier 3)         | Output type     | Transformation                                                                                                                                 |
| --------------------------- | --------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `ReadingTextDef`            | `ReadingText`   | Engine populates `reference` from map key (`Option` → required `String`); `no_final_acclamation` defaults to `false` (`Option<bool>` → `bool`) |
| `ShortFormDef`              | `ShortForm`     | Direct mapping                                                                                                                                 |
| `PsalmodyEntryDef`          | `PsalmodyEntry` | Engine selects one antiphon from `antiphons` Vec based on cycle context                                                                        |
| `PsalmAntiphonDef`          | `PsalmAntiphon` | Direct mapping (one variant selected from input Vec)                                                                                           |
| `AntiphonTextDef`           | `AntiphonText`  | Direct mapping                                                                                                                                 |
| `PrefaceRef::Inline`        | `PrefaceText`   | Text carried as-is; `id` and `headline` are `None`                                                                                             |
| `PrefaceRef::CatalogId`     | `PrefaceText`   | Engine resolves ID via `PrefaceCatalog`; carries `id` + `headline`                                                                             |
| `ReadingTextDef` (alleluia) | `AlleluiaText`  | Engine computes `AcclamationType` from season                                                                                                  |
| `ReadingTextDef` (sequence) | `SequenceText`  | Engine wraps text into sequence structure                                                                                                      |
| `SourceRef`                 | `SourceRef`     | Shared — no transformation                                                                                                                     |

### 2. `CommonDef` → `Common` Expansion

The engine expands each `CommonDef` (23 variants) into the fully resolved `Common` (34 variants) based on runtime context:

| `CommonDef`               | Context needed                       | Resolved `Common` variants                                                      |
| ------------------------- | ------------------------------------ | ------------------------------------------------------------------------------- |
| `BlessedVirginMary`       | Current `Season`                     | `BVM_OrdinaryTime`, `BVM_Advent`, `BVM_Christmas`, `BVM_Easter`                 |
| `Martyrs`                 | `Season` + `MartyrologyEntry.count`  | `Martyrs_OutsideEaster_One`, `..._Several`, `Martyrs_Easter_One`, `..._Several` |
| `MissionaryMartyrs`       | `MartyrologyEntry.count`             | `Martyrs_Missionary_One`, `..._Several`                                         |
| `VirginMartyrs`           | —                                    | `Martyrs_Virgin`                                                                |
| `WomanMartyrs`            | —                                    | `Martyrs_Woman`                                                                 |
| `Pastors`                 | `MartyrologyEntry.titles` + `.count` | `Pastors_PopeOrBishop`, `Pastors_Bishop`, `Pastors_One`, `..._Several`          |
| `Popes`                   | —                                    | `Pastors_PopeOrBishop`                                                          |
| `Bishops`                 | —                                    | `Pastors_Bishop`                                                                |
| `Founders`                | `MartyrologyEntry.count`             | `Pastors_Founder_One`, `..._Several`                                            |
| `Missionaries`            | —                                    | `Pastors_Missionary`                                                            |
| `DoctorsOfTheChurch`      | —                                    | `DoctorsOfTheChurch`                                                            |
| `Virgins`                 | `MartyrologyEntry.count`             | `Virgins_One`, `Virgins_Several`                                                |
| `Saints`                  | `MartyrologyEntry.count`             | `Saints_All_One`, `Saints_All_Several`                                          |
| `Abbots`                  | —                                    | `Saints_Abbot`                                                                  |
| `Monks`                   | —                                    | `Saints_Monk`                                                                   |
| `Nuns`                    | —                                    | `Saints_Nun`                                                                    |
| `Religious`               | —                                    | `Saints_Religious`                                                              |
| `MercyWorkers`            | —                                    | `Saints_MercyWorks`                                                             |
| `Educators`               | —                                    | `Saints_Educators`                                                              |
| `HolyWomen`               | —                                    | `Saints_HolyWomen`                                                              |
| `None`                    | —                                    | `None`                                                                          |
| `DedicationAnniversary_*` | —                                    | `DedicationAnniversary_Inside`, `..._Outside`                                   |

**Count resolution:** When the context requires count (One vs. Several), the engine examines the `MartyrologyEntryDef`:

- `type: Person` or `type: Group` with `count: 1` → "One" variant
- `type: Group` with `count > 1` or `count: Many` → "Several" variant

**Season resolution:** For BVM, the engine uses `DayContext.season` at the time of calendar generation.

### 3. Title Resolution

The engine assembles the output `Title` from input components:

```
Input (Tier 1 + Tier 2)                         Output
────────────────────────                         ──────
TitleCategory::Martyr (from Tier 2)         →    Title {
+ title_qualifiers.martyr:                         category: Martyr,
  "the First Martyr" (complete rendered text)      qualifier: Some("the First Martyr")
                                                 }

TitleCategory::Bishop (from Tier 2)         →    Title {
+ no qualifier                                     category: Bishop,
                                                   qualifier: None
                                                 }
```

The qualifier is the **complete rendered text** of the qualified title (see Part III §6). When present, it replaces the base title from `title_categories` in the display. Fallback follows the BCP-47 locale hierarchy (`fr` → `en`).

The `TitlesDef` operations (append/prepend) from Tier 1 definitions are applied to the base titles from Tier 2 martyrology entries. The engine resolves in order:

1. Load base titles from `MartyrologyEntryDef.titles` (Tier 2).
2. Apply `CelebrationDef.titles` operations (Tier 1): append, prepend, or replace.
3. Apply `MartyrologyRef.titles` operations (Tier 1, per-entry override): append, prepend, or replace.
4. For each `TitleCategory`, check `title_qualifiers` (Tier 2, locale-specific): if present, use the qualifier as the rendered title; otherwise use the base title from `title_categories`.
5. Assemble output `Title { category, qualifier }`.

### 4. Patronage Resolution

```
Input (Tier 1 + Tier 2)                    Output
────────────────────────                    ──────
PatronageDef {                         →    Patronage {
  role: Copatron,                             role: Copatron,
  of: "france"   ─── lookup ──→               of: "France" (en) / "la France" (fr)
}                     locales                  }
                      .patronage_subjects
                      ["france"]
```

Gender inflection (patron/patroness/co-patron/co-patroness) is resolved from `MartyrologyEntry.sex` at display time, not stored in the data model.

### 5. Canonical Prayer (CP 44)

The canonical prayer (`Celebration.prayer` in the output model) is resolved from Tier 3:

1. If `CelebrationMassTexts.prayer` is present → that is the canonical prayer.
2. If absent but exactly one `MassTime` has a `collect` → that collect serves as the canonical prayer.
3. If multiple `MassTime` entries have distinct collects (e.g., Christmas) → no single canonical prayer (`prayer: None`); each Mass provides its own.
4. If no proper collect exists → the Common provides the collect.

The Office concluding prayer follows the same resolution: it defaults to `Celebration.prayer` (= Mass collect = CP 44 identity) unless overridden by `HourTexts.concluding_prayer`.

---

## Part VI — Contributor Ergonomics

### 1. Design Rationale: Simplified Input Types

The input model is deliberately simpler than the output model:

| Aspect        | Input                           | Output                                 | Why simpler                                           |
| ------------- | ------------------------------- | -------------------------------------- | ----------------------------------------------------- |
| Commons       | 23 variants (`CommonDef`)       | 34 variants (`Common`)                 | Season/count context resolved by engine               |
| Titles        | 24 categories (`TitleCategory`) | 24 categories + qualifiers (`Title`)   | Qualifiers are free-text in Tier 2, not enum variants |
| Patronages    | 3 roles + locale key            | 3 roles + localized text (`Patronage`) | Gender and display resolved by engine                 |
| Colors        | Not specified                   | `Vec<ColorInfo>`                       | Deduced from titles and season                        |
| Rank          | Not specified                   | `Rank`                                 | Derived from `Precedence`                             |
| Fullname      | Optional override               | Always present (`String`)              | Constructed from components when not overridden       |
| Mass content  | Citation strings only           | Full text + provenance (`SourcedText`) | Text and provenance added by engine from Tier 3       |
| Hours content | Not in Tier 1                   | Full per-Hour structure                | Text from Tier 3, structure from rank rules           |

This means a contributor adding a new saint to a national calendar only needs to:

1. Add a `CelebrationDef` in the calendar's JSON (`calendars/`): date, precedence, commons, martyrology ref.
2. Add a `MartyrologyEntryDef` in the base locale (`martyrology/en/`): name, titles, dates.
3. Add localized overrides in other locales (`martyrology/fr/`, etc.): translated fullname.

No Tier 3 data is required for the calendar to function — the celebration will appear with correct dates, rank, colors, and commons, using Common texts as fallback.

### 2. Adding a New Saint

**Scenario:** Add "Saint Example, Bishop and Doctor" as an optional memorial on March 15 to the French national calendar.

**Step 1 — Tier 1:** Add the celebration definition to `data/calendars/countries/france/france.json`:

```json
"example_bishop": {
  "precedence": "optional_memorial_12",
  "date_def": { "month": 3, "date": 15 },
  "commons_def": ["bishops", "doctors_of_the_church"]
}
```

**Step 2 — Tier 2 (en):** Add the martyrology entry to `data/martyrology/en/e.json`:

```json
"example_bishop": {
  "canonization_level": "saint",
  "name": "Example",
  "titles": ["bishop", "doctor_of_the_church"],
  "sex": "male",
  "date_of_death": 1200
}
```

**Step 3 — Tier 2 (fr):** Add the French override to `data/martyrology/fr/e.json`:

```json
"example_bishop": {
  "fullname": "Saint Exemple, évêque et docteur de l'Église († 1200)"
}
```

**Result:** The engine produces a `Celebration` with:

- `id`: `"example_bishop"`
- `name`: "Saint Exemple, évêque et docteur de l'Église (†1200)" (fr) / "Saint Example, Bishop and Doctor of the Church" (en)
- `rank`: `OptionalMemorial`
- `precedence`: `OptionalMemorial_12`
- `colors`: `[White]` (deduced: bishop, not a martyr)
- `commons`: `[Pastors_Bishop, DoctorsOfTheChurch]` (expanded from `CommonDef`)

### 3. Adding a New Calendar

**Scenario:** Create a diocesan calendar for the Diocese of Rouen (France).

**Step 1:** Create `data/calendars/countries/france/france__rouen.json`:

```json
{
  "$schema": "../../../../schemas/calendar_definition.json",
  "id": "france__rouen",
  "metadata": {
    "jurisdiction": "ecclesiastical",
    "type": "diocese"
  },
  "parent_calendar_ids": ["france"],
  "celebrations": {
    "joan_of_arc_virgin": {
      "precedence": "proper_solemnity__principal_patron_4a",
      "patronages": [{ "role": "principal_patron", "of": "the_diocese" }]
    }
  }
}
```

This calendar:

- Inherits everything from `france` (which inherits from `europe` → `general_roman`).
- Elevates Joan of Arc from obligatory memorial (France level) to proper solemnity (diocesan patron).
- Adds the diocesan patronage designation.

---

## Appendix — Type Summary

### Input-only types (defined in this document)

| Type                       | Tier | Purpose                                    |
| -------------------------- | ---- | ------------------------------------------ |
| `CalendarDef`              | 1    | Root calendar file structure               |
| `CalendarMetadata`         | 1    | Calendar classification                    |
| `ParticularConfig`         | 1    | Movable feast configuration                |
| `CelebrationDef`           | 1    | Core celebration definition                |
| `DateDef`                  | 1    | Date assignment                            |
| `DateFn`                   | 1    | Movable feast functions                    |
| `DateDefExceptions`        | 1    | Conditional date adjustments               |
| `CommonDef`                | 1    | Simplified Common enum (23 variants)       |
| `PatronageDef`             | 1    | Patronage designation                      |
| `MartyrologyRef`           | 1    | Reference to martyrology entry             |
| `MartyrologyEntryOverride` | 1    | Per-celebration martyrology overrides      |
| `MassesDefinitions`        | 1    | Reading citations by mass time and cycle   |
| `MartyrologyFile`          | 2a   | Root type for a martyrology file           |
| `MartyrologyEntryDef`      | 2a   | Biographical metadata + localized names    |
| `SaintDateDef`             | 2a   | Flexible biographical date                 |
| `LocaleFile`               | 2b   | Root type for a locale file                |
| `ResourcesMetadata`        | 2b   | UI strings and localization data           |
| `FullnameTemplates`        | 2b   | Fullname construction templates            |
| `ProperMassTexts`          | 3    | Mass formulary and orations                |
| `AntiphonTextDef`          | 3    | Mass antiphon with biblical sources        |
| `CommonMassTexts`          | 3    | Common Mass text pools                     |
| `PrefaceDef`               | 3    | Preface catalog entry                      |
| `PrefaceCatalogFile`       | 3    | Root file type for preface catalog         |
| `PrefaceRef`               | 3    | Inline text or catalog ID reference        |
| `ReadingsTextsFile`        | 3    | Root file type for readings + psalms       |
| `ReadingTextDef`           | 3    | Biblical reading with headline, short form |
| `ShortFormDef`             | 3    | Short form reference + text                |
| `PsalmodyEntryDef`         | 3    | Psalm/canticle with antiphon               |
| `PsalmAntiphonDef`         | 3    | Responsorial antiphon with source          |
| `ProperHoursTexts`         | 3    | Office proper elements per Hour            |
| `CommonHoursTexts`         | 3    | Common Office text pools                   |

### Shared types (defined in companion document, used in both input and output)

| Type            | Usage in input                                                                    |
| --------------- | --------------------------------------------------------------------------------- |
| `SourceRef`     | `PsalmAntiphonDef.sources`, `AntiphonTextDef.sources`, `AlleluiaText.sources`     |
| `Precedence`    | `CelebrationDef.precedence`                                                       |
| `MassTime`      | `MassesDefinitions` keys, `ProperMassTexts` keys                                  |
| `HourTime`      | `ProperHoursTexts` keys                                                           |
| `TitleCategory` | `MartyrologyEntryDef.titles`, `TitlesDef`                                         |
| `PatronRole`    | `PatronageDef.role`                                                               |
| `Color`         | `ResourcesMetadata.colors` keys                                                   |
| `Season`        | `ResourcesMetadata.seasons` keys                                                  |
| `Rank`          | `ResourcesMetadata.ranks` keys                                                    |
| `Common`        | `CommonMassTexts` keys, `CommonHoursTexts` keys, `ResourcesMetadata.commons` keys |
| `DayOfWeek`     | `DateDef.WeekdayOfMonth.day_of_week`                                              |
| `Period`        | `ResourcesMetadata.periods` keys                                                  |

### Output-only types (defined in companion document, produced by the engine)

| Type                 | Produced from                                             |
| -------------------- | --------------------------------------------------------- |
| `LiturgicalCalendar` | All three tiers                                           |
| `LiturgicalDay`      | `CelebrationDef` + date resolution                        |
| `DayContext`         | Date computation + `ParticularConfig`                     |
| `Celebration`        | `CelebrationDef` + `MartyrologyEntryDef` + texts          |
| `CelebrationMass`    | `MassReadingsDef` + `ProperMassTexts` + `CommonMassTexts` |
| `CelebrationHour`    | `ProperHoursTexts` + `CommonHoursTexts`                   |
| `AlleluiaText`       | `ReadingTextDef` + computed `AcclamationType`             |
| `AcclamationType`    | Computed from liturgical season                           |
| `SequenceText`       | `ReadingTextDef` for sequence references                  |
| `PrefaceText`        | `PrefaceRef` resolution (inline or catalog lookup)        |
| `MassCalendar`       | Layer 1 → Layer 2 Mass transformation                     |
| `MassComposition`    | Layer 1 celebrations + composition rules                  |
| `HoursCalendar`      | Layer 1 → Layer 2 Hours transformation                    |
| `HoursComposition`   | Layer 1 celebrations + GILH rules                         |
| `Title`              | `TitleCategory` + qualifier assembly                      |
| `Patronage`          | `PatronageDef` + locale resolution                        |
| `CommonInfo`         | `Common` + localized name                                 |
| `ColorInfo`          | `Color` + localized name                                  |
| `MartyrologyEntry`   | `MartyrologyEntryDef` + locale merge                      |
| `ParentOverride`     | Calendar inheritance diff                                 |
