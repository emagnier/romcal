---
title: Calendar Resolution Algorithm
description: Normative specification for romcal's calendar resolution pipeline — hierarchy resolution, date computation, precedence application, transfer of impeded celebrations, field inheritance, and Layer 2 transformations.
tableOfContents:
  minHeadingLevel: 2
  maxHeadingLevel: 4
---

<!-- AI Quick Index — concept/rule → section (line numbers are approximate)

CALENDAR HIERARCHY
| Concept                              | Section              | Line   | Key references                    |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| Territorial chain                    | Part I §1            | ~116   | CP 14-15                          |
| Religious chain                      | Part I §1            | ~130   | CP 16a-c                          |
| CalendarType enum table              | Part I §1            | ~143   | CP 13-16                          |
| Cross-layering rule                  | Part I §2            | ~156   | CP 16d, GNLY 52c                  |
| territorial_context_id               | Part I §2            | ~176   |                                   |
| Solesmes example                     | Part I §2            | ~185   |                                   |
| Hierarchy resolution algorithm       | Part I §3            | ~209   |                                   |
| CP §16d algorithm (steps 6-8)        | Part I §3            | ~234   | CP 16d                            |
| particular_config resolution         | Part I §4            | ~258   | GNLY 7                            |

PROPER OF TIME
| Concept                              | Section              | Line   | Key references                    |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| Temporal cycle (season boundaries)   | Part II §5           | ~275   | GNLY 18-44                        |
| Moveable feasts and Sunday assign.   | Part II §6           | ~306   | GNLY 7                            |
| Liturgical cycles (A/B/C, I/II)      | Part II §7           | ~335   |                                   |

DATE RESOLUTION
| Concept                              | Section              | Line   | Key references                    |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| Five DateDef variants                | Part III §8          | ~370   |                                   |
| Date exception conditions            | Part III §9          | ~402   |                                   |
| Date exceptions vs. transfer         | Part III §10         | ~441   | GNLY 60                           |

HIERARCHY MERGING
| Concept                              | Section              | Line   | Key references                    |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| Field-by-field override              | Part IV §11          | ~462   | CP 13                             |
| drop flag                            | Part IV §12          | ~506   |                                   |
| Rank elevation and demotion          | Part IV §13          | ~528   | CP 25                             |

PRECEDENCE RESOLUTION
| Concept                              | Section              | Line   | Key references                    |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| Table of Precedence (13 levels)      | Part V §14           | ~555   | GNLY 59                           |
| Conflict resolution — single date    | Part V §15           | ~589   | GNLY 59                           |
| Lenten demotion rule                 | Part V §16           | ~612   | GNLY 14, 59                       |
| Transfer of impeded solemnities      | Part V §17           | ~626   | GNLY 60, GNLY 5, Notitiae R14    |
| Transfer algorithm                   | Part V §18           | ~642   | GNLY 60                           |
| Vespers conflict resolution          | Part V §19           | ~665   | GNLY 61                           |

PROPERTY INHERITANCE
| Concept                              | Section              | Line   | Key references                    |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| Temporal context propagation         | Part VI §20          | ~680   |                                   |
| Color resolution                     | Part VI §21          | ~699   | GIRM 346                          |
| parent_overrides traceability        | Part VI §22          | ~728   |                                   |

COMPOSITION GUARDS
| Concept                              | Section              | Line   | Key references                    |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| Protected zones                      | Part VII §23         | ~743   | CP 2, GNLY 56f                    |
| One celebration per saint per year   | Part VII §24         | ~766   | CP 3                              |
| Overburdening guard                  | Part VII §25         | ~772   | CP 17                             |
| Harmonization rules                  | Part VII §26         | ~781   | CP 23                             |

LAYER 2 TRANSFORMATIONS
| Concept                              | Section              | Line   | Key references                    |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| Mass calendar (Layer 2 Mass)         | Part VIII §27        | ~799   | GILM 83                           |
| Hours calendar (Layer 2 Hours)       | Part VIII §28        | ~809   | GILH 225-240                      |

EDGE CASES
| Concept                              | Section              | Line   | Key references                    |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| Holy Thursday coexistence            | Part IX §29          | ~829   | PS 35-36, 44-48                   |
| Saturday BVM memorial                | Part IX §30          | ~842   | GNLY 15                           |
| Readings cycle layered resolution    | Part IX §31          | ~852   |                                   |
| Sunday assignment (Epiphany, etc.)   | Part IX §32          | ~862   | GNLY 7, Notitiae R1               |

APPENDICES
| Concept                              | Section              | Line   |
| ------------------------------------ | -------------------- | ------ |
| Complete pipeline summary            | Appendix A           | ~876   |
| Normative references table           | Appendix B           | ~926   |

-->

## Context and Motivation

This document is the fourth of four companion architecture documents. Together, they describe romcal's data pipeline end-to-end:

- [**Input Data Model**](./input-data-model.md) defines what goes **in** — the three-tier structure of calendar definitions, martyrology catalog, and liturgical texts that contributors edit.
- [**Public API**](./public-api.md) defines **how** to drive the engine — configuration, method signatures, CLI commands, and bindings surface.
- [**Liturgical Composition Model**](./liturgical-composition-model.md) defines what comes **out** — the output types and composition rules for the liturgical calendar, Mass calendar, and Hours calendar.
- **This document** (Calendar Resolution Algorithm) defines the **transformation pipeline** — how the engine turns input definitions into a resolved liturgical calendar.

While the input model is designed for human editors, the composition model for type consumers, and the public API for application developers, this document is designed for **engine implementers**: it specifies the algorithm that transforms calendar definitions into the three output layers.

### Normative Status

This document is a **normative specification**, grounded in the liturgical norms (GNLY, CP, Notitiae responses). Any implementation must conform to this specification.

### Religious Calendars

Religious calendars (orders, congregations, institutes) are specified here as first-class features per the liturgical norms (CP §16), even though no religious calendar data has been contributed yet. The `data/definitions/religious/` directory exists and is ready for data.

---

## Part I — Calendar Hierarchy Model

### 1. Two Parallel Chains (CP §13–16)

Two inheritance hierarchies exist in the Roman Rite, both rooted in the General Roman Calendar.

**Territorial chain** (CP §14–15):

```
General Roman Calendar
  → Region (e.g., europe)
    → Country (e.g., france)
      → Diocese (e.g., france__lyon)
        → Parish / Church
```

**Religious chain** (CP §16a–c):

```
General Roman Calendar
  → Order / Congregation (e.g., benedictines)
    → Province (e.g., benedictines__france)
      → Local Community (e.g., benedictines__france__solesmes)
```

Both chains share the General Roman Calendar as root. Each level adds proper celebrations on top of all inherited ones. CP §13 establishes the fundamental principle: "A particular calendar is formed by the insertion of particular celebrations into the General Calendar."

The `CalendarType` enum (defined in the Input Data Model) classifies each calendar's position in the hierarchy:

| `CalendarType`       | Chain       | Level                                                                  | CP ref |
| -------------------- | ----------- | ---------------------------------------------------------------------- | ------ |
| `GeneralRoman`       | Both (root) | Universal calendar                                                     | —      |
| `Region`             | Territorial | Multi-country (e.g., Europe, Americas)                                 | §14    |
| `Country`            | Territorial | National                                                               | §14    |
| `Diocese`            | Territorial | Diocesan or archdiocesan                                               | §15    |
| `Church`             | Territorial | Parish, shrine, basilica                                               | §15c   |
| `ReligiousInstitute` | Religious   | Order, congregation, or institute (e.g., benedictines)                 | §16a–b |
| `ReligiousProvince`  | Religious   | Province (e.g., benedictines\_\_france)                                | §16c   |
| `ReligiousHouse`     | Religious   | Local house, monastery, convent (e.g., benedictines**france**solesmes) | §16c   |

The `parent_calendar_ids` field in each `CalendarDef` declares the **full inheritance** chain. Each calendar only needs to declare its immediate parent(s) — the engine recursively resolves the complete ancestor chain via post-order DFS (§3).

### 2. Cross-Layering Rule (CP §16d, GNLY §52c)

Religious communities additionally observe celebrations from the territorial calendar of their location. CP §16d states: "Members of religious institutes join with the local Church in celebrating the anniversary of the dedication of the cathedral and the feast of the principal patrons of both the place and the wider area in which they reside."

> **Important:** Cross-layering is **not full inheritance**. The religious calendar does not adopt the entire territorial calendar (national proper saints, diocesan memorials, etc.). Only the specific celebrations mandated by CP §16d are extracted from the territorial chain. Full inheritance is handled exclusively by `parent_calendar_ids`.

**What is extracted — three specific precedence variants:**

The engine identifies CP §16d celebrations by filtering on three `Precedence` variants:

| CP §16d celebration                   | `Precedence` variant                            | Description                               |
| ------------------------------------- | ----------------------------------------------- | ----------------------------------------- |
| Principal patron of the diocese/place | `ProperFeast_PrincipalPatronOfADiocese_8a`      | The patron saint of the local diocese     |
| Dedication of the cathedral           | `ProperFeast_DedicationOfTheCathedralChurch_8b` | Anniversary of the cathedral church       |
| Principal patron of the region/nation | `ProperFeast_PrincipalPatronOfARegion_8c`       | The patron saint of the country or region |

Only `8a`, `8b`, and `8c` are extracted. Other proper feasts (`8d` title/founder of a religious org, `8e` individual church, `8f` other proper feasts) are **not** part of the cross-layering — they belong to their respective calendar chains.

These celebrations are inserted at their territorial rank — not elevated, not demoted.

**Configuration — `territorial_context_id`:**

The territorial context is declared via the `territorial_context_id` field in `CalendarMetadata` (see Input Data Model, Part II §2). This field is separate from `parent_calendar_ids` to avoid ambiguity:

| Field                    | Type                 | Semantics                                                    |
| ------------------------ | -------------------- | ------------------------------------------------------------ |
| `parent_calendar_ids`    | `Vec<CalendarId>`    | Full inheritance — all celebrations, field-by-field override |
| `territorial_context_id` | `Option<CalendarId>` | CP §16d extraction only — `8a`, `8b`, `8c` celebrations      |

**Example — Abbey of Solesmes:**

```json
{
  "id": "benedictines__france__solesmes",
  "metadata": {
    "jurisdiction": "ecclesiastical",
    "type": "religious_house",
    "territorial_context_id": "france__le_mans"
  },
  "parent_calendar_ids": ["benedictines__france"],
  "celebrations": {}
}
```

The engine resolves the territorial chain `france__le_mans` → `france` → `europe` → `general_roman` and extracts:

| Source            | Extracted celebration                      | Precedence |
| ----------------- | ------------------------------------------ | ---------- |
| `france__le_mans` | Dedication of the Cathedral of Le Mans     | `8b`       |
| `france__le_mans` | Principal patron of the Diocese of Le Mans | `8a`       |
| `france`          | Principal patron of France                 | `8c`       |
| `europe`          | Principal patrons of Europe                | `8c`       |

### 3. Hierarchy Resolution Algorithm

**Input:** `Config.calendar_id` + all loaded `CalendarDef` files.

**Output:** An ordered list of calendar definitions from most general to most specific, plus the CP §16d celebrations (if applicable).

**Algorithm — full inheritance chain:**

1. Look up the target calendar in the loaded definitions. If not found → `CalendarNotFound` error.
2. Recursively collect all ancestors via `parent_calendar_ids` using **post-order depth-first search** (DFS): ancestors are visited before descendants.
3. **Circular reference detection:** If a calendar is encountered that is already in the current DFS path → `ValidationError` (circular inheritance).
4. **Diamond inheritance:** The same ancestor reachable via multiple paths is processed once; subsequent encounters are skipped (deduplication).
5. The result is an ordered list: `[temporal_cycle, general_roman, ...]` from most general to most specific.

Each calendar only declares its **immediate** parent(s). The engine reconstructs the full chain transitively:

```
benedictines__france__solesmes  →  parent_calendar_ids: ["benedictines__france"]
benedictines__france            →  parent_calendar_ids: ["benedictines"]
benedictines                    →  parent_calendar_ids: ["general_roman"]
general_roman                   →  parent_calendar_ids: []

DFS post-order result:
  temporal_cycle → general_roman → benedictines → benedictines__france → benedictines__france__solesmes
```

**Algorithm — CP §16d cross-layering:**

6. If any calendar in the resolved chain has `territorial_context_id` set, resolve the territorial chain (recursively collect its ancestors the same way).
7. Walk the territorial chain and **extract** all celebrations with precedence `8a`, `8b`, or `8c`.
8. Insert these celebrations into the resolved calendar at the correct position — after `general_roman`, before the first religious calendar level. They participate in precedence resolution (§14–15) like any other celebration.

**Resolved hierarchy for Solesmes (complete):**

```
1. temporal_cycle                            Proper of Time skeleton
2. general_roman                             Universal calendar
3. ── CP §16d (from territorial chain) ──    Extracted celebrations:
   │  Dedication of Cathedral of Le Mans     (8b, from france__le_mans)
   │  Principal patron of Le Mans diocese     (8a, from france__le_mans)
   │  Principal patron of France              (8c, from france)
   │  Principal patrons of Europe             (8c, from europe)
4. benedictines                              Order proper
5. benedictines__france                      Province proper
6. benedictines__france__solesmes            Local community proper
```

The `temporal_cycle` is always the implicit base — it provides the Proper of Time skeleton that all calendars build upon. It is not declared in `parent_calendar_ids` but is always the first element in the resolved hierarchy.

### 4. `particular_config` Resolution

Each calendar level may define a `ParticularConfig` controlling moveable feast assignments:

- `epiphany_on_sunday` — Epiphany on the Sunday between Jan 2–8 (GNLY §7)
- `ascension_on_sunday` — Ascension on the 7th Sunday of Easter (GNLY §7)
- `corpus_christi_on_sunday` — Corpus Christi on the Sunday after Trinity (GNLY §7)
- `easter_calculation_type` — Gregorian or Julian computus

**Resolution rule:** Most specific non-null value wins. The engine traverses the resolved hierarchy from most specific to most general and takes the first explicitly set value for each field. If no calendar in the chain sets a field, the default applies (all `false`, Gregorian computus).

The resolved `particular_config` affects moveable feast date computation (§6) and must be finalized before any date resolution begins.

---

## Part II — Proper of Time Generation

### 5. The Temporal Cycle

The Proper of Time is the foundational liturgical skeleton. It is generated programmatically from the Easter date and the liturgical year boundaries. Every other calendar layer builds on top of this skeleton.

**Season boundaries (GNLY §18–44):**

| Season           | Start                                                    | End                                                    | GNLY ref |
| ---------------- | -------------------------------------------------------- | ------------------------------------------------------ | -------- |
| Advent           | First Vespers of the Sunday on or closest to November 30 | Before First Vespers of Christmas                      | §40      |
| Christmas Time   | First Vespers of Christmas                               | Sunday after Epiphany inclusive (Baptism of the Lord)  | §33      |
| Ordinary Time I  | Monday after the Baptism of the Lord                     | Tuesday before Ash Wednesday                           | §44      |
| Lent             | Ash Wednesday                                            | Before the evening Mass of the Lord's Supper exclusive | §28      |
| Paschal Triduum  | Evening Mass of the Lord's Supper                        | Vespers of Easter Sunday                               | §19      |
| Easter Time      | Easter Sunday                                            | Pentecost Sunday inclusive                             | §22      |
| Ordinary Time II | Monday after Pentecost                                   | Before First Vespers of Advent I                       | §44      |

> The Paschal Triduum is **not a season** — see the Composition Model, Part III §7 for normative analysis. It is tracked via `Period::PaschalTriduum` in `DayContext.periods`.

Each day in the temporal cycle receives the following contextual properties:

| Property         | Type               | Description                                                             |
| ---------------- | ------------------ | ----------------------------------------------------------------------- |
| `season`         | `Option<Season>`   | The liturgical season — `None` during Good Friday and Holy Saturday     |
| `week_of_season` | `u8`               | Week number within the season (1-based)                                 |
| `day_of_season`  | `u16`              | Day number within the season (1-based)                                  |
| `day_of_week`    | `DayOfWeek`        | `Sunday`, `Monday`, … `Saturday`                                        |
| `psalter_week`   | `PsalterWeekCycle` | `Week1`, `Week2`, `Week3`, `Week4`                                      |
| `sunday_cycle`   | `SundayCycle`      | `YearA`, `YearB`, `YearC`                                               |
| `weekday_cycle`  | `WeekdayCycle`     | `Year1`, `Year2`                                                        |
| `periods`        | `Vec<Period>`      | Special periods (e.g., `ChristmasOctave`, `HolyWeek`, `PaschalTriduum`) |

### 6. Moveable Feasts and Sunday Assignments

Moveable feasts are celebrations whose civil date changes each year. Most are simple offsets from Easter and are expressed via `Anchored { anchor: "easter_sunday", day_offset: N }`. Only dates requiring non-trivial computation (computus or `ParticularConfig`-dependent logic) have their own `DateAnchor` variant.

**`DateAnchor` variants:**

| `DateAnchor`       | Computation                                                                                          |
| ------------------ | ---------------------------------------------------------------------------------------------------- |
| `EasterSunday`     | Computus algorithm (Gregorian or Julian per config)                                                  |
| `Epiphany`         | January 6 (fixed) or Sunday between January 2–8 (per `epiphany_on_sunday` config)                    |
| `Ascension`        | Easter + 39 days (Thursday) or 7th Sunday of Easter (per `ascension_on_sunday` config)               |
| `CorpusChristi`    | Thursday after Trinity Sunday or Sunday after Trinity Sunday (per `corpus_christi_on_sunday` config) |
| `BaptismOfTheLord` | Sunday after Epiphany (depends on Epiphany resolution)                                               |

**Common Easter offsets** (expressed as `{ "anchor": "easter_sunday", "day_offset": N }`):

| Celebration                | Offset      |
| -------------------------- | ----------- |
| Ash Wednesday              | Easter − 46 |
| Palm Sunday                | Easter − 7  |
| Divine Mercy Sunday        | Easter + 7  |
| Pentecost Sunday           | Easter + 49 |
| Mary, Mother of the Church | Easter + 50 |
| Immaculate Heart of Mary   | Easter + 69 |

**Sunday assignment rule (GNLY §7):**

When `particular_config` assigns Epiphany, Ascension, or Corpus Christi to Sunday, the feast **completely replaces** the Ordinary Time or Easter Time Sunday (Notitiae R1). No commemoration of the displaced Sunday is retained. All dependent calculations (e.g., `BaptismOfTheLord` anchored to `Epiphany`) use the Sunday date.

### 7. Liturgical Cycles

Three cycles locate each day within the multi-year lectionary and psalter rotation:

**Sunday readings cycle (3-year rotation):**

| Cycle | Active when                | Synoptic Gospel |
| ----- | -------------------------- | --------------- |
| A     | `liturgical_year % 3 == 1` | Matthew         |
| B     | `liturgical_year % 3 == 2` | Mark            |
| C     | `liturgical_year % 3 == 0` | Luke            |

The liturgical year begins on the First Sunday of Advent. The cycle designation applies to the entire liturgical year that starts that day.

**Weekday readings cycle (2-year rotation):**

| Cycle   | Active when                  |
| ------- | ---------------------------- |
| Year I  | `civil_year % 2 == 1` (odd)  |
| Year II | `civil_year % 2 == 0` (even) |

The weekday cycle follows the civil year, not the liturgical year.

**Psalter week (4-week rotation):**

The psalter follows a 4-week cycle (GILH §133) that resets to Week 1 at these points:

- 1st Sunday of Advent
- 1st Sunday of Ordinary Time (after the Baptism of the Lord)
- 1st Sunday of Lent
- Easter Sunday

The cycle rotates: Week1 → Week2 → Week3 → Week4 → Week1 → …

Between reset points the cycle continues uninterrupted. In Ordinary Time, the psalter week is derived from the week number: `((week_of_season - 1) % 4) + 1`. Since Ordinary Time week numbering is continuous across OT I and OT II, the psalter week in OT II follows naturally from the OT week number (GILH §133: "it begins with the week indicated in the Proper of Seasons at the beginning of the appropriate week in Ordinary Time").

---

## Part III — Date Resolution

### 8. The Five `DateDef` Variants

Each `CelebrationDef` specifies its date via a `DateDef` (defined in the Input Data Model, Part II §5). Most variants resolve to a single date per year; the `RecurringWeekday` variant resolves to multiple dates.

The engine resolves each variant as follows:

**1. `MonthDate { month, date, day_offset }`**

Fixed calendar date. Resolution: construct the date from month and day in the target year, then apply `day_offset` (if any).

Example: January 2 → `{ month: 1, date: 2 }`

**2. `Anchored { anchor, day_offset }`**

Anchored to a `DateAnchor`. Resolution: compute the anchor date (§6), then apply `day_offset`.

Example: Pentecost Monday → `{ anchor: "easter_sunday", day_offset: 50 }`

**3. `WeekdayOfMonth { month, day_of_week, nth_week_in_month, day_offset }`**

The Nth occurrence of a weekday in a given month. Resolution: find the Nth `day_of_week` in the specified month, then apply `day_offset`.

Example: 4th Thursday of November (Thanksgiving) → `{ month: 11, day_of_week: Thursday, nth_week_in_month: 4 }`

**4. `LastWeekdayOfMonth { month, last_day_of_week_in_month, day_offset }`**

The last occurrence of a weekday in a given month. Resolution: find the last `last_day_of_week_in_month` in the specified month, then apply `day_offset`.

Example: Last Sunday of November (Christ the King) → `{ month: 11, last_day_of_week_in_month: Sunday }`

**5. `RecurringWeekday { day_of_week, season }`**

Every occurrence of a weekday within a season. Resolution: the engine generates one celebration instance for each matching day in the target season. Unlike other variants, this resolves to **multiple dates per year**. Conflict resolution (§15) applies independently on each date — the celebration is only retained on dates where no higher-precedence celebration exists.

Example: Saturday BVM memorial on every Saturday of Ordinary Time → `{ day_of_week: Saturday, season: OrdinaryTime }`

### 9. Date Exception Conditions

After computing the base date from `DateDef`, the engine evaluates `date_exceptions` — conditional overrides that adjust the date when specific conditions are met.

**Three condition types:**

| Condition                           | Semantics                                                                                                                                      |
| ----------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| `IsBetween { from, to, inclusive }` | Base date falls within the range `[from, to]` (inclusive) or `(from, to)` (exclusive). Bounds `from` and `to` are themselves `DateDef` values. |
| `IsSameAsDate { date }`             | Base date equals the computed reference `date` (a `DateDef`).                                                                                  |
| `IsDayOfWeek { day_of_week }`       | Base date falls on the specified weekday (`DayOfWeek` value).                                                                                  |

**Two resolution outcomes:**

| Outcome                     | Semantics                                                |
| --------------------------- | -------------------------------------------------------- |
| `DateDef(...)`              | Replace the base date with a completely new computation. |
| `WithOffset { day_offset }` | Shift the base date by `day_offset` days.                |

**Evaluation rule:** Exceptions are evaluated in order. The **first matching** exception wins; subsequent exceptions are not evaluated.

**Example — Annunciation transfer:**

When March 25 falls during Holy Week or the Easter Octave, the Annunciation is transferred to the Monday after the 2nd Sunday of Easter:

```json
{
  "date_def": { "month": 3, "date": 25 },
  "date_exceptions": {
    "when": {
      "from": { "anchor": "easter_sunday", "day_offset": -7 },
      "to": { "anchor": "easter_sunday", "day_offset": 7 },
      "inclusive": true
    },
    "then": { "anchor": "easter_sunday", "day_offset": 8 }
  }
}
```

### 10. Date Exceptions vs. Algorithmic Transfer

Two distinct mechanisms can move a celebration from its base date. The difference is **visibility**: `date_exceptions` see only the celebration's own date; the transfer algorithm sees the entire resolved calendar.

**`date_exceptions`** (§9) are conditional overrides written by calendar authors. They express rules with a **known condition and a known target**: "if the date falls in range X, move to date Y." They are appropriate when:

- The conflict is predictable from the celebration's own date (e.g., Annunciation on March 25 falling during Holy Week).
- The transfer target is fixed and prescribed by the norms (e.g., GNLY §60: Annunciation always transfers to the Monday after the 2nd Sunday of Easter).
- A bishops' conference or particular calendar makes a specific decision (e.g., St. Joseph on a Sunday → move to Monday).

**The transfer algorithm** (§17–18) is engine-computed. After all celebrations have been placed on their intended dates, the engine scans for impeded solemnities — solemnities that land on a date where a higher-precedence celebration exists. It then searches for the **closest free day** (GNLY §60). This dynamic search requires full calendar visibility, which `date_exceptions` do not have. The algorithm handles:

- Solemnities impeded by celebrations from a different calendar layer (e.g., a diocesan solemnity impeded by a General Calendar solemnity).
- Cascading transfers (transferred solemnity A lands on a date that impedes solemnity B).
- Any impeded solemnity without an applicable `date_exception`.

**Example — `date_exceptions` (known target):**

The Annunciation (March 25) during Holy Week is transferred to the Monday after the 2nd Sunday of Easter (GNLY §60 absolute rule). The condition is predictable (March 25 falls between Palm Sunday and Divine Mercy Sunday) and the target is fixed (Easter + 8). A `date_exception` handles this:

```json
{
  "date_exceptions": {
    "when": {
      "from": { "anchor": "easter_sunday", "day_offset": -7 },
      "to": { "anchor": "easter_sunday", "day_offset": 7 },
      "inclusive": true
    },
    "then": { "anchor": "easter_sunday", "day_offset": 8 }
  }
}
```

**Example — transfer algorithm (dynamic target):**

A diocese defines a proper solemnity for its patron (precedence `4a`) on March 19 — the same date as St. Joseph (General Calendar, precedence `3`). St. Joseph takes precedence; the diocesan solemnity is impeded. The engine applies GNLY §60: _"If a Solemnity is impeded by a liturgical day that takes precedence over it, it is transferred to the closest day not listed in nos. 1–8 of the Table of Liturgical Days."_ It searches outward from March 19 for the closest day not occupied by a level 1–8 celebration.

The target varies each year depending on what surrounds March 19:

| Year | March 18                     | March 20                   | Transfer target                  |
| ---- | ---------------------------- | -------------------------- | -------------------------------- |
| 2026 | Wednesday of Lent (level 13) | Friday of Lent (level 13)  | **March 18** (closest, free)     |
| 2029 | 4th Sunday of Lent (level 2) | Tuesday of Lent (level 13) | **March 20** (March 18 occupied) |

A `date_exception` cannot express "closest free day" — the target depends on the full calendar. Only the transfer algorithm (§18), which sees all celebrations on all dates, can resolve this dynamically.

**Execution order:**

1. `date_exceptions` are evaluated first (§9) → produce the "intended date."
2. The transfer algorithm (§17–18) operates on the full resolved calendar → may move a celebration again if the intended date is still impeded.

---

## Part IV — Hierarchy Merging

### 11. Definition Merging — Field-by-Field Override

When processing calendar definitions from most general to most specific, each child `CelebrationDef` is a **partial override** (see Input Data Model, Part II §4). Only fields explicitly set in the child override the parent value. `None`/absent fields inherit from the parent.

The inheritance chain: `temporal_cycle → general_roman → region → country → diocese → ...`

**Field-by-field inheritance rules:**

| Field                       | Inheritance behavior                                            |
| --------------------------- | --------------------------------------------------------------- |
| `date_def`                  | Child wins if set; else inherits parent date                    |
| `date_exceptions`           | Child wins if set; else inherits parent exceptions              |
| `precedence`                | Child wins if set; else inherits parent (default: `Weekday_13`) |
| `commons_def`               | Child wins if set                                               |
| `is_holy_day_of_obligation` | Child wins if set (default: `false`)                            |
| `is_optional`               | Child wins if set (default: `false`)                            |
| `titles`                    | Merged via `append` / `prepend` / `replace` operations          |
| `martyrology`               | Child wins if set                                               |
| `masses`                    | Child wins if set                                               |
| `colors`                    | Deprecated; computed from titles + season (see §21)             |

**Semantics of "child wins if set":** When a child calendar defines a `CelebrationDef` with the same `CelebrationId` as an ancestor, only the fields that are explicitly present (`Some(...)`) in the child override the corresponding parent field. All other fields remain as inherited.

**New celebrations:** When a child calendar defines a `CelebrationId` not present in any ancestor, it is a new celebration. All fields must have sensible values — the engine applies defaults (`precedence: Weekday_13`, `is_holy_day_of_obligation: false`, etc.) for any absent field.

**Example — Rank elevation:**

France defines Joan of Arc as `GeneralMemorial_10`. The Diocese of Rouen overrides only `precedence`:

```json
// france__rouen.json
{
  "celebrations": {
    "joan_of_arc_virgin": {
      "precedence": "proper_solemnity__principal_patron_4a",
      "patronages": [{ "role": "principal_patron", "of": "the_diocese" }]
    }
  }
}
```

The engine merges this with the inherited definition: Joan of Arc retains her date (May 30), commons, martyrology reference, and all other fields from the France calendar, but her precedence becomes `ProperSolemnity_PrincipalPatron_4a` and she gains a patronage designation.

### 12. The `drop` Flag

A child calendar may set `drop: true` on any celebration inherited from a parent. This removes the celebration from the resolved calendar entirely — it does not appear on any date.

**Constraints:**

- **Proper of Time days cannot be dropped** → `ValidationError`. The temporal cycle is structurally required.
- **Dropping a celebration that doesn't exist in any parent** → `ValidationError`. A `drop` is only meaningful when applied to an inherited celebration.

**Example:**

```json
// A diocesan calendar that drops an optional memorial
{
  "celebrations": {
    "some_optional_memorial": {
      "drop": true
    }
  }
}
```

### 13. Rank Elevation and Demotion

CP §25: "The observance of some celebrations in a particular place may have greater solemnity than in the entire diocese or religious institute."

A more specific calendar may **raise** the rank of an inherited celebration by overriding its `precedence` field. This is the standard mechanism for patron saints — a saint who is an optional memorial in the General Calendar becomes a proper solemnity in a diocesan calendar where that saint is the principal patron.

**Direction:** Typically upward (elevation). Demotion is possible but uncommon — e.g., a feast that is a solemnity in the General Calendar may remain at feast rank in a particular calendar for pastoral reasons (CP §8: "for pastoral reasons this may be observed as a solemnity" implies the reverse is also possible in exceptional cases).

**Rank assignment defaults by calendar level (CP §8–12):**

| Celebration type                 | Calendar level      | Default rank       | CP ref |
| -------------------------------- | ------------------- | ------------------ | ------ |
| Principal patron of nation       | National            | Feast              | §8     |
| Principal patron of diocese      | Diocesan            | Feast              | §9     |
| Cathedral dedication anniversary | Diocesan            | Feast              | §9     |
| Principal patron of town/city    | Local               | Solemnity          | §10    |
| Church dedication anniversary    | Church              | Solemnity          | §11    |
| Church title                     | Church              | Solemnity          | §11    |
| Religious title/founder/patron   | Religious Institute | Solemnity/Feast    | §12a   |
| Other saints (no special bond)   | Any                 | Obl./Opt. Memorial | §24    |

---

## Part V — Precedence Resolution

This is the core of the resolution algorithm.

### 14. Table of Precedence (GNLY §59)

GNLY §59 defines 13 numbered levels in the Table of Liturgical Days. romcal subdivides these into **27 variants** to distinguish sub-levels needed for correct conflict resolution.

The table below maps each GNLY level to the romcal `Precedence` variants and the resulting `Rank`:

**Tier I — Always prevail. Impeded solemnities must be transferred.**

| Level | GNLY §59 description                                                                                                                                                                 | romcal `Precedence` variant(s)                                                                                                                                                                   | `Rank`                                                                                                                                           |
| ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| 1     | Paschal Triduum of the Passion and Resurrection of the Lord                                                                                                                          | `Triduum_1`                                                                                                                                                                                      | `Weekday`                                                                                                                                        |
| 2     | Christmas, Epiphany, Ascension, Pentecost; Sundays of Advent, Lent, Easter; Ash Wednesday; weekdays of Holy Week (Mon–Wed); days within the Octave of Easter                         | `ProperOfTimeSolemnity_2`, `PrivilegedSunday_2`, `AshWednesday_2`, `WeekdayOfHolyWeek_2`, `WeekdayOfEasterOctave_2`                                                                              | `Solemnity` (for ProperOfTimeSolemnity, WeekdayOfEasterOctave), `Sunday` (for PrivilegedSunday), `Weekday` (for AshWednesday, WeekdayOfHolyWeek) |
| 3     | Solemnities inscribed in the General Calendar                                                                                                                                        | `GeneralSolemnity_3`, `CommemorationOfAllTheFaithfulDeparted_3`                                                                                                                                  | `Solemnity` (GeneralSolemnity), `Feast` (AllSouls — _sui generis_)                                                                               |
| 4     | Proper solemnities: (a) principal patron of the place; (b) dedication of the own church; (c) title of the own church; (d) title, founder, or primary patron of an order/congregation | `ProperSolemnity_PrincipalPatron_4a`, `ProperSolemnity_DedicationOfTheOwnChurch_4b`, `ProperSolemnity_TitleOfTheOwnChurch_4c`, `ProperSolemnity_TitleOrFounderOrPrimaryPatronOfAReligiousOrg_4d` | `Solemnity`                                                                                                                                      |

**Tier II — Feasts and strong weekdays.**

| Level | GNLY §59 description                                                                                                                                                                                                                                                                                    | romcal `Precedence` variant(s)                                                                                                                                                                                                                                 | `Rank`   |
| ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------- |
| 5     | Feasts of the Lord inscribed in the General Calendar                                                                                                                                                                                                                                                    | `GeneralLordFeast_5`                                                                                                                                                                                                                                           | `Feast`  |
| 6     | Sundays of Christmas Time and Sundays in Ordinary Time                                                                                                                                                                                                                                                  | `UnprivilegedSunday_6`                                                                                                                                                                                                                                         | `Sunday` |
| 7     | Feasts of the BVM and of Saints inscribed in the General Calendar                                                                                                                                                                                                                                       | `GeneralFeast_7`                                                                                                                                                                                                                                               | `Feast`  |
| 8     | Proper feasts: (a) principal patron of a diocese; (b) anniversary of the dedication of the cathedral; (c) principal patron of the region/nation; (d) title, founder, or primary patron of a religious org; (e) other proper feasts of an individual church; (f) other feasts in a diocese/religious org | `ProperFeast_PrincipalPatronOfADiocese_8a`, `ProperFeast_DedicationOfTheCathedralChurch_8b`, `ProperFeast_PrincipalPatronOfARegion_8c`, `ProperFeast_TitleOrFounderOrPrimaryPatronOfAReligiousOrg_8d`, `ProperFeast_ToAnIndividualChurch_8e`, `ProperFeast_8f` | `Feast`  |

**Tier III — Memorials and ordinary weekdays.**

| Level | GNLY §59 description                                                                                                                                                                             | romcal `Precedence` variant(s)                          | `Rank`             |
| ----- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------- | ------------------ |
| 9     | Weekdays of Advent Dec 17–24; weekdays of the Christmas Octave; weekdays of Lent                                                                                                                 | `PrivilegedWeekday_9`                                   | `Weekday`          |
| 10    | Obligatory memorials inscribed in the General Calendar                                                                                                                                           | `GeneralMemorial_10`                                    | `Memorial`         |
| 11    | Proper obligatory memorials: (a) memorial of a secondary patron; (b) other obligatory memorials in a diocese/religious org                                                                       | `ProperMemorial_SecondPatron_11a`, `ProperMemorial_11b` | `Memorial`         |
| 12    | Optional memorials (may be celebrated even on Lent weekdays — "Obligatory Memorials may be celebrated as Optional Memorials if they happen to fall on Lenten weekdays")                          | `OptionalMemorial_12`                                   | `OptionalMemorial` |
| 13    | Weekdays of Advent up to and including Dec 16; weekdays of Christmas Time from Jan 2; weekdays of the Easter Season from Mon after the Octave to Sat before Pentecost; weekdays of Ordinary Time | `Weekday_13`                                            | `Weekday`          |

### 15. Conflict Resolution — Single Date

When multiple celebrations are assigned to the same date after date resolution, the engine resolves the conflict:

**Algorithm:**

1. **Sort** all celebrations on the date by precedence (highest first = lowest GNLY level number). Within the same precedence level, the order is: proper solemnities by sub-level (4a > 4b > 4c > 4d), proper feasts by sub-level (8a > 8b > … > 8f), proper memorials by sub-level (11a > 11b).

2. The **highest-precedence** celebration becomes the **primary** celebration for that date.

3. **Coexistence rules:**

   a. **Same-precedence collision within the same calendar:** When two celebrations from the same calendar level have the same precedence on the same date — which can only happen when a moveable date collides with a fixed date — the engine keeps both as alternatives in `celebration_choices` and sets `default_celebration_id` to `None`. Example: the Immaculate Heart of Mary (moveable, level 10, Easter + 69) falls on a Saturday that may already have a fixed obligatory memorial (level 10). Both are retained; neither is the default.

   > **Note:** No norm explicitly prescribes which celebration takes priority when two memorials of the same rank collide within the same calendar. Setting `default_celebration_id` to `None` reflects this: both alternatives are liturgically equal — the celebrant chooses freely. Consumers must present the choices explicitly rather than auto-selecting one.

   b. **Optional memorials with weekdays:** Optional memorials (level 12) may coexist with ordinary weekdays (level 13) — the weekday is the primary celebration and the optional memorials are available as alternatives (GNLY §59 level 12).

   c. **Optional memorials with privileged weekdays:** Optional memorials may also be celebrated on privileged weekdays (level 9) during Lent — as optional alternatives alongside the weekday (GNLY §59 level 12, "Obligatory Memorials may be celebrated as Optional Memorials if they happen to fall on Lenten weekdays").

4. **Impeded celebrations:**
   - Impeded **solemnities** (levels 3–4) → enter the **transfer queue** (see §17–18).
   - Impeded **feasts** → omitted that year (not transferred).
   - Impeded **memorials** (obligatory or optional) → omitted that year.

### 16. Lenten Demotion Rule (GNLY §14, §59 level 12)

During Lent (Ash Wednesday through Holy Thursday morning), obligatory memorials (levels 10–11) that are not the primary celebration are automatically **demoted to optional memorial status**. They remain available as alternatives alongside the Lenten weekday.

GNLY §14: "Obligatory Memorials which fall on weekdays of Lent may only be celebrated as Optional Memorials."

This demotion:

- Changes the effective `Rank` from `Memorial` to `OptionalMemorial`
- Affects both Mass (GIRM 355.1 regime) and Office (GILH §238–239 `AdditionsOnly` mechanism)
- Applies universally to all obligatory memorials during Lent, including proper memorials from particular calendars

Similarly, GILH §238 specifies that obligatory memorials are not celebrated during Advent Dec 17–24 and the Christmas Octave — the same demotion applies in these privileged periods.

### 17. Transfer of Impeded Solemnities (GNLY §60)

When a solemnity cannot be celebrated on its assigned date because a higher-precedence day occupies it, GNLY §60 prescribes: "If a Solemnity is impeded by a liturgical day that takes precedence over it, it is transferred to the closest day not listed in nos. 1–8 of the Table of Liturgical Days."

**General rule:** Transfer to the **closest day** not occupied by a celebration at levels 1–8.

**Special rules:**

1. **Annunciation in Holy Week or Easter Octave** (GNLY §60 absolute rule): Always transfers to the **Monday after the 2nd Sunday of Easter**. This is a fixed transfer target regardless of what occupies that Monday — it is encoded as a `date_exception` in the General Roman Calendar data.

2. **Solemnity impeded by an Advent or Lent Sunday** (GNLY §5, Notitiae R14): Try the **preceding Saturday** first. If Saturday is also occupied at levels 1–8, fall back to the nearest free day per the general rule.

3. **St. Joseph (March 19) on Palm Sunday**: Transfers to the **preceding Saturday** (March 18) when observed as a holy day of obligation. When not a holy day of obligation, the Bishops' Conference may move it outside Lent entirely (GNLY §56f).

**What makes a day "available":** A day at levels 9–13 (privileged weekdays, memorials, weekdays) can receive a transferred solemnity. The solemnity displaces whatever was there.

### 18. Transfer Algorithm

**Step-by-step specification:**

1. After initial precedence resolution (§15), collect all **impeded solemnities** into a **transfer queue**, ordered by their original precedence (highest first — level 3 before level 4).

2. For each impeded solemnity in the queue:
   a. **Determine search direction and starting point:**
   - For Annunciation in Holy Week/Easter Octave → fixed target (Monday after 2nd Sunday of Easter). This case is handled by `date_exceptions` and should already be resolved before the transfer algorithm runs.
   - For solemnities impeded by Advent/Lent Sundays → try the preceding Saturday first (Notitiae R14).
   - For all other cases → search both directions from the original date, alternating forward and backward, to find the closest free day.
     b. **Search for the nearest date** where no celebration at levels 1–8 exists.
     c. **If the candidate date already has a celebration at levels 9–13:** The solemnity takes precedence — it displaces the lower-ranked celebration. The displaced celebration is omitted (it was a weekday or memorial). If the displaced celebration is itself a solemnity that was also transferred, it re-enters the queue.
     d. **Place the solemnity** on the found date.

3. After all transfers, **re-run precedence resolution** on any affected dates to ensure consistency.

**Edge cases:**

- **Multiple solemnities impeded on the same date:** Process in descending precedence order (highest first).
- **A transferred solemnity landing on a date where another solemnity was also transferred:** The higher-precedence one wins; the other re-enters the queue.
- **Cascade limit:** In practice, cascading transfers are extremely rare (at most 2–3 levels). The algorithm should implement a maximum iteration count as a safety guard.

### 19. Vespers Conflict Resolution (GNLY §61)

When a celebration with First Vespers (solemnities and feasts of the Lord on Sundays) follows another celebration:

- **Higher rank wins** → that celebration's Vespers is prayed.
- **Equal rank** → the current day's Vespers (Vespers II) wins.

GNLY §61: "If, on the other hand, another celebration has its own First Vespers on the following evening, then that Evening Prayer is either from the more important celebration or from the celebration which has precedence according to the Table of Liturgical Days."

This rule applies to **Layer 2 Hours output only**. The engine must determine, for each evening, whether to assign Vespers II of the current day or Vespers I of the following day. The result is reflected in the `HoursComposition` for the affected civil date.

---

## Part VI — Property Inheritance from Proper of Time

### 20. Temporal Context Propagation

Every celebration inherits contextual properties from the Proper of Time day on the same date. These properties locate the celebration within the liturgical year:

| Property          | Inheritance rule                                  |
| ----------------- | ------------------------------------------------- |
| `season`          | Always from Proper of Time                        |
| `week_of_season`  | Always from Proper of Time                        |
| `day_of_season`   | Always from Proper of Time                        |
| `day_of_week`     | Always from Proper of Time                        |
| `psalter_week`    | Always from Proper of Time                        |
| `sunday_cycle`    | Always from Proper of Time                        |
| `weekday_cycle`   | Always from Proper of Time                        |
| `periods`         | From Proper of Time if not set on the celebration |
| `start_of_season` | From Proper of Time                               |
| `end_of_season`   | From Proper of Time                               |

These properties are **always** inherited — even a solemnity that completely replaces the weekday still belongs to the season, week, and cycle determined by the Proper of Time. A solemnity on the 5th Sunday of Ordinary Time is still in Ordinary Time, week 5, with the psalter week and readings cycle of that week.

### 21. Color Resolution

Liturgical colors are **computed**, not authored. The `colors` input field is deprecated (see Input Data Model, Part II §4).

**Priority chain:**

1. **Martyrdom-related title** → **Red**: If `titles` contains `TitleCategory::Martyr` (regardless of qualifier — "Martyr", "the First Martyr", "Proto-martyr of Oceania" all trigger red), the primary color is red (GIRM §346b).
2. **Apostles and Evangelists** → **Red**: If `titles` contains `TitleCategory::Apostle` or `TitleCategory::Evangelist`, the primary color is red (GIRM §346b).
3. **BVM celebrations** → **White**: If the celebration is of the Blessed Virgin Mary, the color is white (GIRM §346a).
4. **Legacy explicit color** → that color: If the celebration definition includes an explicit color (deprecated path), it is used as-is.
5. **Season color from Proper of Time** → default: The celebration inherits the season color.

**Season colors (GIRM §346):**

| Season / Day                | Color  |
| --------------------------- | ------ |
| Advent                      | Violet |
| Christmas Time              | White  |
| Ordinary Time               | Green  |
| Lent                        | Violet |
| Gaudete Sunday (Advent III) | Rose   |
| Laetare Sunday (Lent IV)    | Rose   |
| Easter Time                 | White  |
| Pentecost Sunday            | Red    |
| Palm Sunday (Passion)       | Red    |
| Good Friday                 | Red    |

**Multiple colors:** A celebration may have multiple permissible colors (e.g., gold as alternative on solemnities per GIRM §346g; black as alternative to purple for the Dead per GIRM §346d–e). The engine produces a `Vec<ColorInfo>` with the primary color first.

### 22. `parent_overrides` — Traceability

For each celebration modified by a child calendar, the engine records a `ParentOverride` containing only the fields that changed. The `parent_overrides` array is ordered from most general to most specific, providing a full audit trail of the inheritance chain.

**Example:** St. Thomas Aquinas:

- General Roman Calendar: `OptionalMemorial_12`, `commons: [DoctorsOfTheChurch]`
- Dominican calendar: `ProperSolemnity_TitleOrFounderOrPrimaryPatronOfAReligiousOrg_4d`, `commons: [DoctorsOfTheChurch]` (precedence changed)

The output `Celebration` carries `parent_overrides: [{ from_calendar_id: "general_roman", precedence: OptionalMemorial_12 }]`, recording that the General Roman Calendar defined the original precedence before the Dominican calendar elevated it.

---

## Part VII — Calendar Composition Guards

### 23. Protected Zones (CP §2, GNLY §56f)

Three zones where particular celebrations are restricted:

**1. Sundays (CP §2a):**

No permanent particular celebration may be assigned to a Sunday. Exceptions:

- Solemnities that replace the Sunday per GNLY §59 levels 1–4.
- Sunday assignments per GNLY §7 (Epiphany, Ascension, Corpus Christi — these are not "particular" celebrations but universal feasts moved to Sunday by the Bishops' Conference).

**2. Privileged seasons (CP §2b):**

Lent, Easter Octave, and December 17–31 are kept free from particular celebrations. Exceptions:

- Optional memorials (they may be celebrated as alternatives but do not displace the weekday).
- Feasts at GNLY §59 levels 8a–8d (proper feasts of patron, cathedral dedication, etc.).
- Non-transferable solemnities.

**3. Indult celebrations (CP §2c):**

Must not duplicate celebrations already in the cycle of the mystery of salvation, and must not be too numerous.

### 24. One Celebration Per Saint Per Year (CP §3)

A saint may have **only one celebration** per year in any given calendar.

**Exception:** A second celebration as optional memorial for the translation or discovery of the patron's relics or body, or a similar event (e.g., conversion). CP §3: "Each saint is to be celebrated on one day only in a given calendar year. In accordance with tradition, there may be a second celebration, as an optional memorial, for a saint who is the patron of a church — for the discovery or translation of the saint's body."

### 25. Overburdening Guard (CP §17)

Dioceses and religious institutes with many canonized members should limit the number in the calendar:

- Only saints of **special significance** to the diocese or institute get their own celebration at the diocese/institute level.
- Others: restricted to their local places (church, house), or grouped in a **collective celebration** (e.g., "All Saints of the Diocese of …").

CP §17: "The large number of canonized members of some religious families should not be an excuse for overburdening the calendar."

### 26. Harmonization Rules (CP §23)

When proper celebrations conflict with General Calendar celebrations, CP §23 defines the resolution:

| General Calendar    | Particular Calendar         | Resolution                                                                         | CP ref |
| ------------------- | --------------------------- | ---------------------------------------------------------------------------------- | ------ |
| Solemnity           | Any proper                  | General solemnity always wins on its date                                          | §23a   |
| Feast               | Proper feast (same date)    | General feast kept; proper feast transferred to nearest free date                  | §23b   |
| Feast               | Proper feast (deeply local) | Exception: proper feast may stay if transfer would cause "serious inconvenience"   | §23b   |
| Optional memorial   | Proper memorial             | Proper memorial takes precedence                                                   | §23c   |
| Obligatory memorial | Proper memorial             | Proper memorial may take precedence (universal demoted to optional or transferred) | §23c   |

These harmonization rules complement the general precedence table (§14). The engine applies them during the merging phase — when a particular calendar's celebration conflicts with a General Calendar celebration on the same date, CP §23 governs which takes priority and what happens to the impeded one.

---

## Part VIII — Layer 2 Transformations

### 27. Mass Calendar (Layer 2 Mass)

The Mass calendar is derived from the Layer 1 `LiturgicalCalendar` by applying Mass-specific transformations:

1. **Primary celebration** determines the principal Mass formulary for the date.
2. **Alternative celebrations** (optional memorials) provide alternative Mass options — the celebrant may choose among them (GIRM §355).
3. **Civil date assignment:** Evening Masses (Easter Vigil, Mass of the Lord's Supper) are shifted to the preceding civil date in the Mass calendar. The liturgical day begins with First Vespers (evening), but the civil date is the previous day.
4. **Readings resolution:** For each celebration, readings are resolved through the priority chain: proper readings → accommodated readings → common readings (GILM §83). The resolved readings carry a `ReadingsCategory` (Proper, Accommodated, Common) for consumer information.
5. **Composition rules** are computed from the season, precedence, and rank — they indicate which Mass elements are substitutable (see Composition Model, Part IV §4).

### 28. Hours Calendar (Layer 2 Hours)

The Hours calendar is derived from Layer 1 by applying Office-specific transformations:

1. **Rank determines the Office structure** (GILH §225–240):
   - **Solemnities and feasts:** Complete proper Office — all elements from the proper or Common.
   - **Obligatory memorials:** Feria psalmody + proper elements overlay (antiphons, readings, prayer) per GILH §234–236.
   - **Optional memorials:** Entirely from the feria, with the option to use proper elements for those parts that have proper texts (GILH §236).
   - **Weekdays:** Current week's feria Office from the psalter.

2. **Vespers assignment** per §19 — resolve Vespers I / Vespers II conflicts for each evening.

3. **Vigil extension** for solemnities with proper vigil readings (GILH §73) — the Office of Readings may be extended with canticles and a Gospel (see Composition Model, Part V §5).

4. **Hours composition rules** — `MemorialRule` variants (`FullProper`, `FreeChoice`, `AdditionsOnly`, `NoMemorial`) are computed from the rank and season context (see Composition Model, Part IV §5).

---

## Part IX — Edge Cases and Special Rules

### 29. Holy Thursday Coexistence

Holy Thursday has two distinct liturgical moments on the same civil date:

| Moment  | Liturgical context                 | Mass type                               | Season  |
| ------- | ---------------------------------- | --------------------------------------- | ------- |
| Morning | Chrism Mass (PS §35–36)            | `MassTime::ChrismMass`                  | Lent    |
| Evening | Mass of the Lord's Supper (PS §44) | `MassTime::EveningMassOfTheLordsSupper` | Triduum |

Both appear on the same civil date but belong to distinct liturgical contexts. The Chrism Mass belongs to Lent (GNLY §28: Lent runs until the evening Mass of the Lord's Supper). The evening Mass begins the Paschal Triduum (PS §2).

In the Mass calendar (Layer 2), both Mass times appear under the same civil date, each with their own `MassComposition`.

### 30. Saturday BVM Memorial (GNLY §15)

GNLY §15: "On Saturdays in Ordinary Time when there is no Obligatory Memorial, an Optional Memorial of the Blessed Virgin Mary is permitted."

This is defined in the General Roman Calendar data using a `RecurringWeekday` date definition (§8 variant 5):

```json
{
  "saturday_bvm_memorial": {
    "date_def": { "day_of_week": "SATURDAY", "season": "ORDINARY_TIME" },
    "rank": "OPTIONAL_MEMORIAL"
  }
}
```

The engine generates one instance per qualifying Saturday. The precedence system (§15) handles the rest: as an optional memorial, the BVM celebration is retained only on Saturdays where no higher-precedence celebration (obligatory memorial or above) exists. Where retained, it appears as an alternative alongside the weekday.

### 31. Readings Cycle Layered Resolution

For `MassesDefinitions` with multiple cycle keys, the engine merges readings in layers:

1. Start with `invariant` — readings shared across all cycles.
2. Overlay the applicable **Sunday cycle** (`year_a`, `year_b`, `year_c`).
3. Overlay the applicable **weekday cycle** (`year_1`, `year_2`).
4. **Combined keys** (`year_ab`, `year_ac`, `year_bc`) match if either of their constituent years is current — `year_ab` applies in both Year A and Year B.

Later layers override earlier ones for the same reading slot. This allows calendar authors to specify invariant readings once and only override the cycle-specific differences.

### 32. Epiphany / Ascension / Corpus Christi Sunday Assignment

When `particular_config` assigns these feasts to Sunday (§4, §6):

- The feast **completely replaces** the Ordinary Time or Easter Time Sunday (Notitiae R1).
- **No commemoration** of the displaced Sunday is retained.
- **No Sunday elements** are preserved — the feast's proper texts, readings, and Office replace the Sunday entirely.
- All **dependent calculations** (e.g., dates anchored to Epiphany) use the Sunday date.

This is a **permanent date assignment**, not a conflict-resolution transfer. It is applied before precedence resolution (see Composition Model, Part V §2 step 1b).

---

## Appendix A — Complete Pipeline Summary

The full resolution algorithm, from input to output:

1. **Resolve `particular_config`** (§4)
   - Traverse the calendar hierarchy; most specific non-null value wins for each config field.

2. **Generate Proper of Time skeleton** (§5–7)
   - Compute Easter date from the resolved `easter_calculation_type`.
   - Generate the full temporal cycle: all days with season, week, cycle, and psalter data.
   - Apply Sunday assignments (Epiphany, Ascension, Corpus Christi) per `particular_config`.

3. **Resolve calendar hierarchy** (§3)
   - From `Config.calendar_id`, recursively collect all ancestor calendars.
   - Result: ordered list from most general to most specific.
   - For religious calendars: extract CP §16d cross-layering celebrations from the territorial chain.

4. **For each calendar level** (general → specific):
   a. For each `CelebrationDef` in this calendar level:
   - If `drop: true` → remove from calendar (§12).
   - Compute date from `DateDef` (§8).
   - Evaluate `date_exceptions` → apply first matching exception (§9).
   - Merge fields with parent version of same `CelebrationId` (§11).
   - Record `parent_overrides` for changed fields (§22).

5. **Inherit Proper of Time properties** (§20–21)
   - For each celebration on each date: inherit `season`, `week_of_season`, `day_of_week`, cycles, etc.
   - Compute liturgical colors from titles + season (§21).

6. **Apply precedence rules per date** (§14–16)
   - Sort celebrations on each date by precedence.
   - Determine primary and alternative celebrations.
   - Apply Lenten demotion: obligatory memorials → optional during Lent (§16).
   - Collect impeded solemnities into the transfer queue.

7. **Transfer impeded solemnities** (§17–18)
   - Process queue in precedence order.
   - Find nearest available date (levels 9–13) for each impeded solemnity.
   - Re-run precedence resolution on affected dates.

8. **Assemble final calendar**
   - Each date has a primary celebration and zero or more alternative celebrations.
   - `RecurringWeekday` definitions (e.g., Saturday BVM memorial — §30) generate instances on each qualifying date.

9. **Generate Layer 2 output** (if requested):
   - **Layer 2 Mass** (§27): shift evening Masses, resolve readings, compute composition rules.
   - **Layer 2 Hours** (§28): determine Office structure by rank, resolve Vespers conflicts, compute memorial rules.

---

## Appendix B — Normative References

Each section of this document is grounded in specific liturgical norms:

| Section(s) | Primary norm   | Secondary norms              | Topic                                                   |
| ---------- | -------------- | ---------------------------- | ------------------------------------------------------- |
| §1–3       | CP §13–16      | GNLY §49–50                  | Calendar hierarchy and inheritance                      |
| §4         | GNLY §7        | —                            | Particular config, Sunday assignments                   |
| §5         | GNLY §18–44    | PS §2, §27                   | Season boundaries, Proper of Time                       |
| §6         | GNLY §7        | Notitiae R1                  | Moveable feasts, Sunday assignment                      |
| §7         | GILM §66       | —                            | Readings cycles                                         |
| §8–9       | —              | —                            | Date resolution (engine-specific)                       |
| §10        | GNLY §60       | —                            | Date exceptions vs. transfer distinction                |
| §11–12     | CP §13         | —                            | Definition merging, drop flag                           |
| §13        | CP §25         | CP §8–12                     | Rank elevation and demotion                             |
| §14        | GNLY §59       | —                            | Table of Precedence (13 levels, 27 variants)            |
| §15        | GNLY §59       | —                            | Conflict resolution on a single date                    |
| §16        | GNLY §14       | GNLY §59 level 12, GILH §238 | Lenten demotion of obligatory memorials                 |
| §17        | GNLY §60       | GNLY §5, Notitiae R14        | Transfer of impeded solemnities (rules)                 |
| §18        | GNLY §60       | —                            | Transfer algorithm (step-by-step)                       |
| §19        | GNLY §61       | —                            | Vespers conflict resolution                             |
| §20        | —              | —                            | Temporal context propagation                            |
| §21        | GIRM §346      | —                            | Color resolution                                        |
| §22        | —              | —                            | Parent overrides traceability                           |
| §23        | CP §2          | GNLY §56f                    | Protected zones                                         |
| §24        | CP §3          | —                            | One celebration per saint per year                      |
| §25        | CP §17         | —                            | Overburdening guard                                     |
| §26        | CP §23         | —                            | Harmonization rules (general vs. particular)            |
| §27        | GILM §83       | GIRM §355                    | Mass calendar (Layer 2 Mass)                            |
| §28        | GILH §225–240  | GILH §73                     | Hours calendar (Layer 2 Hours)                          |
| §29        | PS §35–36, §44 | GNLY §28                     | Holy Thursday coexistence                               |
| §30        | GNLY §15       | GILH §240                    | Saturday BVM memorial                                   |
| §31        | GILM §66, §69  | —                            | Readings cycle layered resolution                       |
| §32        | GNLY §7        | Notitiae R1                  | Sunday assignment (Epiphany, Ascension, Corpus Christi) |

---

## Conclusion

This document specifies the complete algorithm for transforming calendar definitions into a resolved liturgical calendar:

- **Part I** establishes the calendar hierarchy model — two parallel chains (territorial and religious) rooted in the General Roman Calendar, with cross-layering for religious communities.
- **Parts II–III** specify how dates are computed — from the Proper of Time skeleton through the five `DateDef` variants and conditional exceptions.
- **Part IV** defines the merging semantics — field-by-field override, `drop`, and rank elevation.
- **Part V** is the core precedence resolution — the 13-level table, conflict resolution, Lenten demotion, and the transfer algorithm for impeded solemnities.
- **Parts VI–VII** cover property inheritance and composition guards that ensure the resolved calendar conforms to the liturgical norms.
- **Parts VIII–IX** describe Layer 2 transformations and edge cases.

The algorithm produces the `LiturgicalCalendar` (Layer 1), `MassCalendar` (Layer 2 Mass), and `HoursCalendar` (Layer 2 Hours) defined in the [Liturgical Composition Model](./liturgical-composition-model.md), driven by the [Public API](./public-api.md), from the data specified in the [Input Data Model](./input-data-model.md).
