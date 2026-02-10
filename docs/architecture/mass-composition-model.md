# Mass Composition Model — Liturgical Calendar & Mass Calendar

## Context and Motivation

When a weekday (feria, rank 13 in romcal) also has optional memorials (rank 12) available for the same civil date, the Church's norms (GIRM, GNLY, and GILM) define precise rules for how the celebrant may choose and combine liturgical texts — readings, orations, antiphons — between the feria and the optional memorial.

This document synthesizes the analysis of these liturgical rules and proposes a data model for romcal that faithfully reflects them, organized around two complementary output approaches:

- **Approach 1 — Liturgical Calendar** (`generate_liturgical_calendar`): centered on the liturgical day, for internal use and as the foundation for Approach 2.
- **Approach 2 — Mass Calendar** (`generate_mass_calendar`): centered on the mass as celebrated on a civil date, with pre-resolved options and explicit composition rules.

---

## Part I — Liturgical Rules Synthesis

### 1. The Choice of Mass (GIRM 355)

GIRM 355 governs the choice of Mass on days with optional memorials. The range of options varies by season:

- **GIRM 355.3 (Ordinary Time weekdays):** Five options — (a) the weekday Mass, (b) the Mass of an optional memorial occurring that day, (c) the Mass of any Saint listed in the *Martyrology* for that day, (d) a Mass for Various Needs, or (e) a Votive Mass.
- **GIRM 355.2 (Advent before Dec 17, Christmas from Jan 2, Easter):** Four options — (a) the weekday Mass, (b) the Mass of the Saint, (c) the Mass of one of the Saints whose memorial is observed, or (d) the Mass of any Saint listed in the *Martyrology* for that day.
- **GIRM 355.1 (Advent Dec 17-24, Octave of Christmas, Lent):** The Mass of the current liturgical day is obligatory, with limited borrowing from a memorial (see below).

> **Scope note:** This document focuses on the **feria vs. optional memorial** choice — the primary use case for romcal. The Martyrology, Votive Masses, Masses for Various Needs (GIRM 375, 377), and Masses for the Dead (GIRM 381) are valid additional options on Ordinary Time weekdays but are not modeled here.

GIRM 355 also adds two pastoral directives:
- The priest "will take care not to omit the readings assigned for each day in the Lectionary for weekdays too frequently and without sufficient reason, since the Church desires that a richer portion at the table of God's word be provided for the faithful."
- "Where the optional memorials of the Blessed Virgin Mary or of the Saints are dear to the faithful, the priest should satisfy their legitimate devotion."
- When choosing between a memorial in the General Calendar and one in a diocesan or religious calendar, "preference should be given, all things being equal and in keeping with tradition, to the memorial inscribed in the particular calendar."

Once the celebrant has made this global choice, it determines which "formulary" (set of texts) is used as the base. However, certain elements can then be mixed between sources.

### 2. The Three Substitution Groups

The GIRM organizes mass texts into groups with distinct substitution rules:

#### Group 1 — Formulary Block

**Collect + Entrance Antiphon + Communion Antiphon**

These three elements follow the global choice of celebration as a block. If you celebrate the memorial, all three come from the saint (proper or Common). If you celebrate the feria, all three come from the feria.

> **Architectural note:** The GIRM does not explicitly group these three elements together as an "inseparable block." This grouping is an architectural inference derived from what GIRM 363 makes individually flexible (prayer over the offerings, prayer after Communion) and what it does not — leaving the collect, entrance antiphon, and communion antiphon bound to the chosen formulary. Note that GIRM 48 and 87 (cf. GIRM 367) do allow the entrance and communion antiphons to be replaced by other approved chants, which is a separate form of flexibility not modeled here.

The collect is the identifying marker of the celebration. GIRM 363 states: "On memorials of Saints, the collect proper to the day is used, **or, if none is available, one from an appropriate Common.**"

**Exception — Privileged weekdays (GIRM 355.1):** On weekdays of Advent Dec 17-24, Octave of Christmas, and Lent, the Mass of the feria is obligatory. Only the collect may be borrowed from the memorial — this is the only case where the collect is detachable from the antiphons.

**Further exception — Ash Wednesday and Holy Week (GIRM 355.1):** On Ash Wednesday and during Holy Week, even the collect may **not** be borrowed from a memorial. The feria is imposed entirely without exception.

#### Group 2 — Readings Block (GIRM 357-358, GILM 83-84)

**Reading 1 + Psalm + (Reading 2) + Alleluia + Gospel**

The degree of flexibility within this block depends on the **source** of the readings:

- **Weekday and proper readings**: These are **indivisible** — they are taken as a complete, pre-composed set. The psalm responds to the first reading it is paired with.
- **Common readings**: These are **divisible by component** — the celebrant may choose each element independently from pools of texts (GILM 71, 89). See below.

##### The three categories of readings for saints (GILM 83)

The GILM (Introduction to the Lectionary for Mass, *Ordo Lectionum Missae*) distinguishes three categories of readings for celebrations of saints:

| Category | Definition (GILM 83) | Rule |
|---|---|---|
| **Proper readings** (*lectiones propriae*) | Biblical passages about the Saint or the mystery that the Mass is celebrating. The Order of Readings makes explicit note of every case on a memorial. | **Obligatory** — "Even in the case of a memorial these readings must take the place of the weekday readings for the same day." |
| **Accommodated readings** (*lectiones accommodatae*) | Readings that bring out some particular aspect of a Saint's spiritual life or work. | **Facultative** — "Use of such readings does not seem binding, except for compelling pastoral reasons." For the most part, references are given to readings in the Commons to facilitate choice. |
| **Common readings** (*lectiones communes*) | Readings placed in the Commons either for a determined class of Saints (martyrs, virgins, pastors) or for the Saints in general. | **Freely chosen** — "It will be up to the priest to choose the one best suited to those listening." |

##### The Common as a permanent alternative (GILM 83)

GILM 83 explicitly states: "In all celebrations of Saints the readings may be taken not only from the Commons to which the references are given in each case, but also from the **Common of Men and Women Saints**, whenever there is special reason for doing so."

This means that **Common readings are always available as alternatives** for any celebration of a saint — even when no accommodated or proper readings are indicated.

##### How GIRM 357-358 articulates with GILM 83

GIRM 357 states that "unless strictly proper readings are given, the readings assigned for the weekday are customarily used" — implying that strictly proper readings, when they exist, override the weekday readings. The term "strictly proper" (*lectiones stricte propriae*) is used in the GIRM but not fully defined there. GIRM 358 offers a partial definition by referring to "proper New Testament readings, that is to say, readings in which mention is made of the Saint being celebrated." The complete technical definition comes from GILM 78-84.

In practical terms, the readings rule for memorials is:

| Lectionary provides | Rule | Reference |
|---|---|---|
| **Proper readings** (the saint is named or the mystery is directly evoked in the biblical text) | Obligatory — must replace weekday readings | GILM 83, GIRM 357 |
| **Accommodated readings** (the Lectionary gives suggestions for the saint) | Facultative — may use them, or choose from Common, or keep feria | GILM 83 |
| **No specific readings** (the Lectionary refers to a Common) | Feria readings by default, but Common readings are also available | GILM 83, GIRM 357 |

##### When the Lectionary provides only one proper reading

When the Lectionary gives only one proper reading for a saint (e.g., only a gospel), that reading **must** be used (it is proper). The other reading(s) can be taken from:
- The **feria** (weekday lectionary)
- The **Common** corresponding to the saint's category
- An **accommodated reading** if the Lectionary indicates one

It is **not** automatically from the feria — the Common is always an option (GILM 83).

**The celebrant cannot mix readings at will** (e.g., taking the gospel from the saint and the first reading from the feria when both proper readings are available). When proper readings exist, they must all be used.

##### Default preference for weekday readings (GIRM 355, GILM 83)

Both GIRM 355 (in its pastoral remarks following 355.3) and GILM 83 emphasize that the priest should take care not to omit the weekday readings too frequently, as the Church desires "a more lavish table of the word of God" (*GILM 83*) through the *lectio continua*.

##### Interrupted continuous reading (GIRM 358)

GIRM 358 provides that when the continuous weekday reading is interrupted by a solemnity, feast, or particular celebration, "the priest, taking into consideration the entire week's scheme of readings, is allowed either to combine parts omitted with other readings or to decide which readings are to be preferred over others." This flexibility is a pastoral provision, not modeled as a data structure but relevant context for consumers.

##### Component-level choice in Commons readings (GILM 71)

GILM 71 explains the ordering principle of the Commons:

> "As to their sequence, all the texts in this part of the Order of Readings appear in the order in which they are to be read at Mass. Thus the Old Testament texts are first, then the texts from the Apostles, followed by the psalms and verses between the readings, and finally the texts from the Gospels. **The rationale of this arrangement is that, unless otherwise noted, the celebrant may choose at will from such texts.**"

This means Common readings are organized as **pools per reading position**:
- Pool of Old Testament readings → pick one
- Pool of Apostle readings → pick one
- Pool of psalms → pick one
- Pool of gospels → pick one

The celebrant composes the readings set by picking independently from each pool. This is fundamentally different from proper or weekday readings, which are pre-composed and indivisible.

##### Psalm independence in Commons (GILM 89)

GILM 89 reinforces this flexibility specifically for the responsorial psalm:

> "As a rule the psalm to be used is the one assigned to the reading. But **in the case of readings for the Common of Saints, ritual Masses, Masses for various needs and occasions, votive Masses, and Masses for the dead the choice is left up to the priest celebrating.** He will base his choice on the principle of the pastoral benefit of those present."

GILM 89 also notes that alternative psalms are provided for seasons and classes of Saints, which "may replace the text corresponding to the reading" when the psalm is sung.

**Consequence for the data model:** The psalm is bound to reading 1 for proper/weekday readings (indivisible), but independently choosable for Common readings.

##### Acclamation before the Gospel (GILM 90-91)

GILM 90: The acclamation (alleluia verse) between the second reading and the Gospel is "either specified in each Mass and correlated with the Gospel or else it is left as a choice to be made from those in the series given for a liturgical season or one of the Commons."

GILM 91: During Lent, a specific acclamation format is used instead of the Alleluia.

**Consequence for the data model:** When specified, the acclamation is part of the indivisible block. When left as a choice (Commons, seasonal), it follows the same pool logic as other Common components.

##### Long and short forms (GILM 75, 80)

GILM 75 notes that "in the case of certain rather lengthy texts, longer and shorter versions are provided." GILM 80 specifies that "a pastoral criterion must also guide the choice between the longer and shorter forms of the same text."

**Consequence for the data model:** A reading text may have an optional short form variant.

#### Group 3 — Flexible Orations (GIRM 363) and Preface (GIRM 364-365)

**Prayer over the Offerings, Prayer after Communion**

Each of these can be chosen **independently** (à la carte):

> "The prayer over the offerings, however, and the prayer after Communion, unless they are proper, may be taken either from the Common or from the weekdays of the current Season." (GIRM 363)

##### Weekday orations in Ordinary Time (GIRM 363 §3)

On weekdays in Ordinary Time, GIRM 363 provides broader alternatives for orations: "besides the orations from the previous Sunday, orations from another Sunday in Ordinary Time may be used, or one of the prayers for various needs provided in the Missal. It is always permissible, however, to use the collect alone from these Masses."

##### Seasonal restriction (GIRM 363 §5)

"During the more important seasons of the year, however, the proper seasonal orations appointed for each weekday in the Missal already make provision for this." This means the broader alternatives above (other Sundays, prayers for various needs) do **not** apply during Advent, Christmas, Lent, and Easter — the seasonal weekday orations are already provided.

##### Preface (GIRM 364-365)

The preface can come from the Common of the saint or from the season. GIRM 363 does not govern the preface — it only addresses orations (collect, prayer over the offerings, prayer after Communion).

GIRM 365 governs the choice of Eucharistic Prayer, which has a direct interaction with the preface: Eucharistic Prayer IV has an **invariable** preface and "may be used when a Mass has no Preface of its own" (GIRM 365.4). This constraint limits preface flexibility.

> **Architectural note:** The preface is grouped with the flexible orations in this model for structural convenience, but its normative framework is GIRM 364-365, not GIRM 363. The Eucharistic Prayer choice itself is not modeled here.

##### Long and short forms of texts (GIRM 360)

GIRM 360 provides that "at times, a longer and shorter form of the same text is given. In choosing between these two forms, a pastoral criterion must be kept in mind." This parallels GILM 75 and 80 for readings, and applies to any liturgical text with variant forms.

### 3. Summary by Season Context

The rules vary depending on the liturgical season:

| Season | GIRM ref | Formulary block | Readings block | Flexible orations |
|---|---|---|---|---|
| **Ordinary Time weekdays** | 355.3 | Free choice: feria or memorial ¹ | Feria default; saint's proper if they exist (obligatory); or Common (always available) | Each independently: feria or Common |
| **Advent before Dec 17, Christmas from Jan 2, Easter** | 355.2 | Free choice: feria or memorial ² | Free choice: feria, saint's proper, or Common | Each independently |
| **Advent Dec 17-24, Octave of Christmas, Lent** (except Ash Wed & Holy Week) | 355.1 | Feria imposed (only collect borrowable from memorial) | Feria imposed | Feria imposed |
| **Ash Wednesday, Holy Week** | 355.1 | Feria imposed entirely (no collect borrowing) | Feria imposed | Feria imposed |

¹ GIRM 355.3 also allows Martyrology saints, Masses for Various Needs, and Votive Masses (out of scope for this model).
² GIRM 355.2 also allows the Mass of any Saint listed in the *Martyrology* (out of scope for this model).

### 4. Visual Schema

```
WEEKDAY IN ORDINARY TIME + OPTIONAL MEMORIAL
═════════════════════════════════════════════

  Which celebration? ─────────────────────────
  │                                           │
  ▼                                           ▼
FERIA                                     MEMORIAL
(all from feria)                              │
                           ┌──────────────────┼──────────────────┐
                           ▼                  ▼                  ▼
                      AS A BLOCK         À LA CARTE          READINGS
                   (follows choice)      (mixable)
                           │                  │
                    • Collect (saint)   • Pr. over offer.   ┌─ WEEKDAY/PROPER:
                    • Entrance ant.       (feria OR         │  Indivisible set
                      (saint)             Common)           │  (all or nothing)
                    • Communion ant.    • Pr. after comm.   │
                      (saint)             (feria OR         ├─ COMMON (GILM 71):
                                          Common)           │  Pool per component
                                        • Preface           │  (pick each reading,
                                          (season OR        │   psalm, gospel
                                          Common)           │   independently)
                                                            └─ (GILM 83, 89)
```

```
PRIVILEGED WEEKDAYS (Advent 17-24, Octave of Christmas, Lent)
═════════════════════════════════════════════════════════════

  Formulary = FERIA (imposed)
  │
  └── Only exception: the collect ALONE
      may be borrowed from the memorial
      (everything else = feria)

  ⚠ ASH WEDNESDAY & HOLY WEEK:
      No exception at all — feria imposed
      entirely, including the collect.
```

### 5. Source References

#### GIRM (General Instruction of the Roman Missal)
- **GIRM 355** — Choice of Mass on optional memorials (by season): five options in OT (355.3), four in certain seasons (355.2), feria imposed on privileged days (355.1). Pastoral caution on preserving weekday readings. Exception: no collect borrowing on Ash Wednesday and Holy Week.
- **GIRM 357** — Choice of readings for memorials: weekday readings unless strictly proper readings exist
- **GIRM 358** — Weekday Lectionary readings: continuous reading scheme; priest may combine omitted readings when interrupted by celebrations
- **GIRM 360** — Long and short forms of texts: "a pastoral criterion must be kept in mind"
- **GIRM 361** — Pastoral criteria for choosing between alternative texts; prohibition against permanently excluding Scripture passages
- **GIRM 362** — Adaptations to the *Ordo Lectionum Missae* by Conferences of Bishops must be observed
- **GIRM 363** — Choice of orations for memorials: collect from proper or Common; prayer over offerings and prayer after Communion flexible. OT weekdays: orations from other Sundays or prayers for various needs also available (§3). Seasonal restriction: during major seasons, proper seasonal orations are already provided (§5).
- **GIRM 364-365** — Preface (purpose and variety) and Eucharistic Prayer choices. EP IV has an invariable preface and may only be used when a Mass has no preface of its own (365.4).
- **GIRM 367** — Norms for chants at entrance, offertory, and Communion (cf. GIRM 48, 87): entrance and communion antiphons may be replaced by other approved chants
- **GIRM 375, 377** — Votive Masses and Masses for Various Needs on OT weekdays with optional memorials (out of scope for this model)
- **GIRM 381** — Masses for the Dead on OT weekdays with optional memorials (out of scope for this model)

#### GNLY (General Norms for the Liturgical Year and the Calendar)
- **GNLY 3** — "The liturgical day runs from midnight to midnight."
- **GNLY 10** — "Celebrations, according to the importance assigned to them, are distinguished one from another and termed: Solemnity, Feast, Memorial."
- **GNLY 14** — "Memorials are either obligatory or optional; their observance is integrated into the celebration of the occurring weekday."
- **GNLY 16** — Weekdays definition and precedence rules
- **GNLY 59** — Table of Liturgical Days according to Their Order of Precedence

#### GILM (General Introduction to the Lectionary for Mass, *Ordo Lectionum Missae*)
- **GILM 70** — Two series of readings for saints: Proper of Saints and Commons of Saints
- **GILM 71** — Ordering of Commons readings: OT first, then Apostle, then psalms, then Gospels. "The celebrant may choose at will from such texts." This is the basis for component-level choice in Common readings.
- **GILM 75** — Long and short forms of texts: "longer and shorter versions are provided to suit different situations"
- **GILM 80** — Choice between long/short forms: "a pastoral criterion must also guide the choice"
- **GILM 82** — Weekday readings arrangement: used on their assigned days unless a celebration with proper readings occurs
- **GILM 83** — Three categories of readings for saints: proper (obligatory), accommodated (facultative), common (freely chosen). Common readings are always available as alternatives.
- **GILM 84** — Rules by rank: solemnities (proper or Common), feasts and memorials (two readings, first from OT or Apostle, second from Gospels)
- **GILM 89** — Psalm follows the first reading; **but for Commons, the choice is left to the priest.** Alternative psalms per season/class may replace the assigned psalm when sung.
- **GILM 90** — Acclamation before the Gospel: either specified (correlated with Gospel) or left as a choice from the season or Commons series
- **GILM 91** — Lenten acclamation: specific format replaces the Alleluia during Lent

---

## Part II — Vocabulary: Liturgical Day vs. Celebration

The GNLY uses both terms with distinct meanings:

**Liturgical day** (*dies liturgicus*) — GNLY 3:
> "The liturgical day runs from midnight to midnight. However, the celebration of Sunday and of Solemnities begins already on the evening of the previous day."

The liturgical day is the **temporal frame**: a calendar date that can host one or more celebrations.

**Celebration** (*celebratio*) — GNLY 10:
> "Celebrations, according to the importance assigned to them, are hence distinguished one from another and termed: Solemnity, Feast, Memorial."

A celebration is the **liturgical entity** with a rank, a name, and texts. The feria is a celebration. An optional memorial is a different celebration. Both can coexist on the same liturgical day.

### Consequence for the data model

- **`LiturgicalDay`** = the container (one per civil date). Carries the shared temporal context (season, cycle, psalter week...).
- **`Celebration`** = the content (one or more per liturgical day). Carries identity (name, rank, precedence) and mass texts.

A single liturgical day can contain multiple celebrations: the feria as the primary celebration, plus any optional memorials as alternatives.

---

## Part III — Cycle Resolution

Romcal already computes the applicable liturgical cycle for any given date (Year A/B/C for Sundays, Year 1/2 for weekdays). Therefore, the output data model does **not** include a cycle dimension — the engine resolves the correct cycle internally and returns only the applicable content.

The cycle information remains available in `DayContext` (`sunday_cycle`, `weekday_cycle`) for informational purposes, but the mass texts are already those of the resolved cycle.

---

## Part IV — Data Model

### Shared Types

These types are used by both approaches.

#### `DayContext`

**What it is:** The shared temporal context for all celebrations on a given date.

**Why this name:** It provides the calendrical "context" of the "day" — season, cycles, position within the season — without any celebration-specific information.

**Contents:**

```rust
struct DayContext {
    season: Option<Season>,
    season_name: Option<String>,
    sunday_cycle: SundayCycle,
    weekday_cycle: WeekdayCycle,
    psalter_week: PsalterWeekCycle,
    week_of_season: Option<u32>,
    day_of_season: Option<u32>,
    day_of_week: DayOfWeek,
    periods: Vec<PeriodInfo>,
    start_of_season: Option<String>,
    end_of_season: Option<String>,
    start_of_liturgical_year: String,
    end_of_liturgical_year: String,
}
```

#### `FormularySet`

**What it is:** The inseparable block of texts that identify a celebration in the Mass: collect + entrance antiphon + communion antiphon.

**Why this name:** In liturgical terminology, the "formulary" (*formularium*) is the complete set of proper texts for a given Mass. This struct represents the core identifying subset that must be taken as a unit. "Set" emphasizes that these elements are grouped and inseparable.

**Liturgical basis:** Architectural inference — GIRM 363 explicitly makes the prayer over the offerings and prayer after Communion flexible, but leaves these three elements (collect, entrance antiphon, communion antiphon) bound to the chosen formulary. See Group 1 discussion in Part I for caveats.

```rust
struct FormularySet {
    collect: Option<String>,
    entrance_antiphon: Option<String>,
    communion_antiphon: Option<String>,
}
```

#### `ReadingText`

**What it is:** A liturgical reading text that may have an optional short form variant.

**Why this name:** It is a "reading" "text" with potential variant forms.

**Liturgical basis:** GIRM 360, GILM 75, 80 — some readings are provided in both long and short forms; "a pastoral criterion must be kept in mind" when choosing.

```rust
struct ReadingText {
    /// The full text of the reading
    text: String,
    /// Optional short form, when the Lectionary provides one (GIRM 360, GILM 75, 80)
    short_form: Option<String>,
}
```

#### `ReadingsSet`

**What it is:** A complete, **indivisible** set of readings for the Liturgy of the Word. Used for weekday (*lectio continua*) and proper readings, where the Lectionary assigns specific texts as a pre-composed unit.

**Why this name:** "Readings" because it contains all the Scripture readings and their associated chants. "Set" because they form an indivisible block — you take them all together or not at all.

**Liturgical basis:** GIRM 357 — proper and weekday readings are taken as a complete set, not mixed individually. The psalm responds to the first reading it is paired with.

**When NOT to use:** For Common readings, where the celebrant chooses independently per component (GILM 71, 89), use `ReadingsPool` instead.

```rust
struct ReadingsSet {
    reading_1: Option<ReadingText>,
    psalm: Option<String>,
    canticle: Option<String>,
    reading_2: Option<ReadingText>,
    sequence: Option<String>,
    alleluia: Option<String>,
    gospel: Option<ReadingText>,
}
```

#### `ReadingsPool`

**What it is:** A set of **pools** of readings, one per reading position, from which the celebrant chooses independently. Used for Common readings, where GILM 71 and 89 grant component-level freedom.

**Why this name:** "Readings" because it concerns the Liturgy of the Word. "Pool" because each position offers a pool of texts to choose from — unlike `ReadingsSet` which is a fixed, pre-composed unit.

**Liturgical basis:**
- GILM 71 — "the celebrant may choose at will from such texts" (Commons are organized by reading position precisely for this purpose)
- GILM 89 — "in the case of readings for the Common of Saints [...] the choice is left up to the priest" (psalm independence)

```rust
struct ReadingsPool {
    /// Available first readings (OT or Apostle) — pick one
    first_readings: Vec<ReadingText>,
    /// Available psalms — pick one independently (GILM 89)
    psalms: Vec<String>,
    /// Available second readings (when 3 readings are required) — pick one
    second_readings: Vec<ReadingText>,
    /// Available alleluia/acclamation verses — pick one (GILM 90)
    alleluia_verses: Vec<String>,
    /// Available gospels — pick one
    gospels: Vec<ReadingText>,
}
```

#### `ReadingsContent`

**What it is:** An enum distinguishing the two modes of readings provision: a fixed, indivisible set vs. a pool of independently choosable components.

**Why this name:** It represents the "content" of the "readings" block, which can take two structural forms depending on the source.

**Liturgical basis:** The distinction arises from the different treatment of proper/weekday readings (indivisible, GIRM 357) vs. Common readings (component-level choice, GILM 71, 89).

```rust
enum ReadingsContent {
    /// Fixed, indivisible set (weekday lectio continua, proper readings)
    Fixed(ReadingsSet),
    /// Pool of components, each independently choosable (Commons, GILM 71)
    Pool(ReadingsPool),
}
```

#### `FlexibleOrations`

**What it is:** The orations that can each be chosen independently between different sources (feria, Common, season).

**Why this name:** "Flexible" because unlike the formulary block, each oration here can be sourced independently. "Orations" is the liturgical term for these presidential prayers.

**Liturgical basis:**
- GIRM 363 — "The prayer over the offerings [...] and the prayer after Communion [...] may be taken either from the Common or from the weekdays of the current Season."
- GIRM 364-365 — The preface is included here for structural convenience, though its norms come from GIRM 364-365, not 363.

```rust
struct FlexibleOrations {
    prayer_over_the_offerings: Option<String>,
    prayer_after_communion: Option<String>,
    preface: Option<String>,        // Governed by GIRM 364-365, not 363
    solemn_blessing: Option<String>,
    prayer_over_the_people: Option<String>,
}
```

#### `TextSource`

**What it is:** An enum indicating where a liturgical text comes from.

**Why this name:** It identifies the "source" of a "text" — its provenance in the liturgical books.

```rust
enum TextSource {
    /// From the Proper of Time (weekday)
    ProperOfTime { day_id: String },
    /// From the proper texts of a specific saint
    ProperOfSaint { saint_id: String },
    /// From a Common (e.g., Common of Virgins, Common of Pastors)
    Common { common: Common, saint_id: Option<String> },
    /// From a Sunday of Ordinary Time (GIRM 363 alternative)
    SundayOrdinaryTime { week: u32 },
}
```

#### `SourcedText`

**What it is:** A liturgical text paired with its provenance.

**Why this name:** It is a "text" that is "sourced" — you know where it comes from. This is essential for Approach 2 where flexible orations are presented as a list of alternatives, each with its origin.

```rust
struct SourcedText {
    source: TextSource,
    text: String,
}
```

#### `CelebrationId`

**What it is:** A unique identifier for a celebration.

**Why this name:** It identifies a `Celebration` uniquely within the calendar (e.g., `"ord_time_5_mon"`, `"st_scholastica"`).

```rust
type CelebrationId = String;
```

---

### Approach 1 — Liturgical Calendar

**Method:** `Calendar::generate_liturgical_calendar() → LiturgicalCalendar`

**Principle:** Organized by liturgical day. Each civil date maps to one `LiturgicalDay` containing all possible celebrations. Masses are not shifted — evening masses (vigils, PreviousEveningMass) remain attached to their liturgical day. This approach serves as the internal foundation from which Approach 2 is generated.

#### `LiturgicalCalendar`

**What it is:** The top-level output type. A map from civil date to liturgical day.

**Why this name:** It is a "calendar" organized by "liturgical" days — the liturgical perspective on the year.

**Why keyed by civil date:** By convention and for practical convenience, each liturgical day is associated with the civil date where the majority of the day occurs. The `MassTime` enum carries the information about whether a mass is celebrated the evening before (e.g., `PreviousEveningMass`, `EasterVigil`).

```rust
type LiturgicalCalendar = BTreeMap<String, LiturgicalDay>;
```

#### `LiturgicalDay`

**What it is:** One liturgical day — the temporal frame for a given civil date, containing the shared context and all possible celebrations.

**Why this name:** It is the "liturgical day" as defined by GNLY 3: "The liturgical day runs from midnight to midnight." It is the container, not the content. A single `LiturgicalDay` can host multiple `Celebration` objects (a feria + optional memorials).

**Why not `Celebration`:** A liturgical day is not a celebration — it is the temporal frame within which celebrations occur. GNLY 10 distinguishes the two concepts explicitly.

```rust
struct LiturgicalDay {
    /// Civil date of attachment (YYYY-MM-DD)
    date: String,

    /// Shared temporal context for all celebrations on this day
    context: DayContext,

    /// Possible celebrations, ordered by precedence
    /// [0] = primary (feria or highest-ranking celebration)
    /// [1..] = alternatives (optional memorials, etc.)
    celebrations: Vec<Celebration>,
}
```

#### `Celebration`

**What it is:** One liturgical celebration — an entity with a rank, a name, liturgical colors, and mass texts. The feria of Wednesday of the 5th week is a celebration. The optional memorial of St. Scholastica is another celebration.

**Why this name:** GNLY 10 defines it: "Celebrations, according to the importance assigned to them, are distinguished one from another and termed: Solemnity, Feast, Memorial." A celebration is the thing you celebrate, with its specific rank and proper texts.

**Why not `LiturgicalDay`:** In the current romcal model, `LiturgicalDay` mixes the temporal frame and the celebration identity. This separation clarifies that multiple celebrations can coexist within one liturgical day.

```rust
struct Celebration {
    /// Unique identifier (e.g., "ord_time_5_wed", "st_scholastica")
    id: CelebrationId,
    /// Localized full name
    name: String,
    /// Liturgical precedence (GNLY table, levels 1-13)
    precedence: Precedence,
    /// Liturgical rank (Solemnity, Feast, Memorial, OptionalMemorial, Weekday)
    rank: Rank,
    /// Localized rank name
    rank_name: String,
    /// Liturgical colors
    colors: Vec<ColorInfo>,
    /// Common categories (e.g., Common of Virgins, Common of Pastors)
    commons: Vec<CommonInfo>,
    /// Martyrology entries (linked saints, blessed, or places)
    martyrology: Vec<MartyrologyEntry>,
    /// Titles (patron, founder, doctor...)
    titles: TitlesDef,
    /// Holy day of obligation
    is_holy_day_of_obligation: bool,
    /// Optional celebration (can be omitted in favor of the feria)
    is_optional: bool,
    /// Source calendar in the inheritance chain
    from_calendar_id: CalendarId,

    /// Masses attached to this celebration, by mass time
    /// Most celebrations: { DayMass: ... }
    /// Christmas: { PreviousEveningMass, NightMass, MassAtDawn, DayMass }
    /// No civil date shifting here — masses stay on their liturgical day
    masses: BTreeMap<MassTime, CelebrationMass>,
}
```

#### `CelebrationMass`

**What it is:** The textual content of a mass for a specific celebration, structured by the three GIRM substitution groups.

**Why this name:** It is the "mass" content belonging to a "celebration." The cycle is already resolved — this struct contains the applicable texts directly, without a cycle dimension.

**Why not `MassContent` (existing name):** The existing `MassContent` is a flat `BTreeMap<MassPart, String>` with no grouping semantics. `CelebrationMass` structures the content by GIRM substitution groups (formulary, readings, flexible orations), which is the key improvement.

```rust
struct CelebrationMass {
    /// Formulary block — follows the choice of celebration
    formulary: FormularySet,
    /// Readings block — Fixed (proper/weekday) or Pool (Common)
    readings: ReadingsContent,
    /// Flexible orations (GIRM 363) and preface (GIRM 364-365)
    flexible_orations: FlexibleOrations,
}
```

#### Approach 1 — Example

```
LiturgicalCalendar
│
├── "2025-02-10" → LiturgicalDay
│   ├── date: "2025-02-10"
│   ├── context: DayContext { season: OrdinaryTime, week: 5, ... }
│   └── celebrations:
│       ├── [0] Celebration
│       │   ├── id: "ord_time_5_mon"
│       │   ├── name: "Monday, 5th Week of Ordinary Time"
│       │   ├── rank: Weekday (13)
│       │   ├── is_optional: false
│       │   └── masses:
│       │       └── DayMass → CelebrationMass
│       │           ├── formulary: FormularySet { collect, ant_entr, ant_comm }
│       │           ├── readings: ReadingsSet { reading_1, psalm, gospel }
│       │           └── flexible_orations: FlexibleOrations { ... }
│       │
│       ├── [1] Celebration
│       │   ├── id: "st_scholastica"
│       │   ├── name: "Saint Scholastica"
│       │   ├── rank: OptionalMemorial (12)
│       │   ├── is_optional: true
│       │   └── masses:
│       │       └── DayMass → CelebrationMass { ... }
│       │
│       └── [2] Celebration
│           ├── id: "bl_luigi_stepinac"
│           ├── name: "Blessed Luigi Stepinac"
│           ├── rank: OptionalMemorial (12)
│           └── ...
│
├── "2025-12-25" → LiturgicalDay
│   ├── context: DayContext { season: ChristmasTime, ... }
│   └── celebrations:
│       └── [0] Celebration
│           ├── id: "christmas"
│           ├── rank: Solemnity (2)
│           └── masses:                          ← no shift
│               ├── PreviousEveningMass → ...    ← stays on Dec 25
│               ├── NightMass → ...
│               ├── MassAtDawn → ...
│               └── DayMass → ...
```

---

### Approach 2 — Mass Calendar

**Method:** `Calendar::generate_mass_calendar() → MassCalendar`

**Principle:** Organized by civil date and mass time. Each mass is a self-contained unit with all options pre-resolved by the engine. Evening masses are shifted to the previous civil day. The consumer picks from the options according to the explicit composition rules.

**Generated from Approach 1:** The engine first produces the `LiturgicalCalendar`, then transforms it into the `MassCalendar` by: shifting evening masses to the previous civil date, assembling identity and readings options from available celebrations, resolving flexible orations with their sources, and computing the applicable composition rules based on season and precedence.

#### `MassCalendar`

**What it is:** The top-level output type. A map from civil date to a list of masses celebrated that day.

**Why this name:** It is a "calendar" organized by "masses" — the practical perspective of what is actually celebrated on each civil day.

```rust
type MassCalendar = BTreeMap<String, Vec<MassComposition>>;
```

#### `MassComposition`

**What it is:** A single mass with all its options pre-resolved, structured by GIRM substitution groups. The consumer receives everything needed to compose the mass without knowing the GIRM rules themselves.

**Why this name:** "Mass" because it represents one mass celebration. "Composition" because the mass is "composed" from options across different blocks — the consumer composes the final mass by picking from the provided options according to the rules.

**Why not `MassContext` (existing name):** The existing `MassContext` is a flat structure that merely references optional celebrations by summary. `MassComposition` goes further: it provides the actual texts organized by substitution groups, with explicit composition rules.

```rust
struct MassComposition {
    // === Identification ===
    /// Type of mass (DayMass, NightMass, EasterVigil...)
    mass_time: MassTime,
    /// Civil date — after shifting for evening masses
    civil_date: String,
    /// Liturgical date — before shifting (the "theological" date)
    liturgical_date: String,

    // === Context ===
    /// Shared day context
    context: DayContext,

    // === Default celebration ===
    /// The celebration to use by default (typically the feria or highest-ranking)
    default_celebration_id: CelebrationId,

    // === FORMULARY BLOCK ===
    /// Each option = one possible celebration with its collect + antiphons
    /// The consumer picks ONE option — all three texts come as a block
    identity_options: Vec<IdentityOption>,

    // === READINGS BLOCK (GIRM 357, GILM 71/83/89) ===
    /// Each option = either a fixed set (weekday/proper) or a pool (Common)
    /// The consumer picks ONE option, then composes from it
    readings_options: Vec<ReadingsOption>,

    // === FLEXIBLE ORATIONS (GIRM 363) AND PREFACE (GIRM 364-365) ===
    /// Each oration has its own list of alternatives, chosen independently
    prayer_over_offerings_options: Vec<SourcedText>,
    prayer_after_communion_options: Vec<SourcedText>,
    preface_options: Vec<SourcedText>,   // Governed by GIRM 364-365
    solemn_blessing_options: Vec<SourcedText>,
    prayer_over_people_options: Vec<SourcedText>,

    // === COMPOSITION RULES ===
    /// Constraints determined by the engine based on season/precedence
    composition_rules: CompositionRules,
}
```

#### `IdentityOption`

**What it is:** One possible celebration that can be chosen for the formulary block. Contains the celebration's metadata and its inseparable text trio (collect + antiphons).

**Why this name:** "Identity" because the formulary block is what *identifies* which celebration is being performed — the collect is the defining prayer. "Option" because it is one choice among several.

**Why it wraps `FormularySet`:** The `FormularySet` (shared type) provides the three inseparable texts. `IdentityOption` adds the celebration metadata (name, rank, colors...) that the consumer needs for display and logic.

```rust
struct IdentityOption {
    /// Reference to the celebration
    celebration_id: CelebrationId,
    celebration_name: String,
    rank: Rank,
    precedence: Precedence,
    colors: Vec<ColorInfo>,
    commons: Vec<CommonInfo>,
    martyrology: Vec<MartyrologyEntry>,
    titles: TitlesDef,
    is_holy_day_of_obligation: bool,
    from_calendar_id: CalendarId,

    /// The three inseparable texts (shared type)
    formulary: FormularySet,
}
```

#### `ReadingsOption`

**What it is:** One possible readings provision for the Liturgy of the Word. May be a fixed set (weekday, proper) or a pool of independently choosable components (Common).

**Why this name:** "Readings" because it concerns the Scripture readings. "Option" because it is one choice among several (weekday readings, saint's proper readings, or Common readings).

**Why it wraps `ReadingsContent`:** The `ReadingsContent` enum (shared type) distinguishes the two structural modes. `ReadingsOption` adds source provenance, the GILM 83 category, and flags.

```rust
struct ReadingsOption {
    /// Where these readings come from
    source: TextSource,
    /// Category per GILM 83 (determines binding force)
    category: ReadingsCategory,
    /// Is this the default option for this day?
    is_default: bool,

    /// Fixed set or pool of components (shared type)
    readings: ReadingsContent,
}
```

#### `ReadingsCategory`

**What it is:** The three categories of readings for saints defined by GILM 83, each with a different binding force.

**Why this name:** It categorizes readings per their normative weight, following the GILM's own distinction.

**Liturgical basis:** GILM 83 — "proper readings... must take the place of the weekday readings"; "accommodated readings... does not seem binding"; "common readings... it will be up to the priest to choose."

```rust
enum ReadingsCategory {
    /// Weekday readings from the lectio continua (Proper of Time)
    Weekday,
    /// Proper readings for the saint — obligatory, must replace weekday readings (GILM 83)
    Proper,
    /// Accommodated readings — facultative, not binding except for pastoral reasons (GILM 83)
    Accommodated,
    /// Common readings — freely chosen from the Commons of Saints (GILM 83)
    Common,
}
```

#### `CompositionRules`

**What it is:** The set of constraints that govern how the consumer may combine the options. Determined by the engine based on the liturgical season and precedence context.

**Why this name:** "Composition" because these are the rules for *composing* the mass from the available options. "Rules" because they are normative constraints from the GIRM, not suggestions.

```rust
struct CompositionRules {
    /// Rule for the formulary block (collect + antiphons)
    identity: BlockRule,
    /// Rule for the readings block
    readings: ReadingsRule,
    /// Rule for the flexible orations
    flexible_orations: FlexibleRule,
}
```

#### `BlockRule`

**What it is:** A rule governing a block where you must pick one option entirely.

**Why this name:** It is a "rule" for a "block" of inseparable texts.

```rust
enum BlockRule {
    /// Free choice among the proposed options
    PickOne,
    /// First option is imposed, but the collect alone may be borrowed from a memorial
    /// (privileged weekdays: Advent Dec 17-24, Octave of Christmas, Lent — GIRM 355.1)
    ForcedCollectBorrowable,
    /// First option is imposed entirely — no borrowing permitted
    /// (Ash Wednesday, Holy Week — GIRM 355.1 exception)
    ForcedNoException,
}
```

#### `ReadingsRule`

**What it is:** A rule specific to the readings block, which has more nuanced behavior than a simple pick-one.

**Why this name:** It is a "rule" specific to "readings," reflecting the three distinct behaviors defined by GIRM 357-358.

```rust
enum ReadingsRule {
    /// Weekday readings are obligatory, no exception
    /// (privileged weekdays: Advent Dec 17-24, Lent)
    WeekdayOnly,
    /// Weekday readings by default; overridden if the saint has proper readings
    /// (GILM 83, GIRM 357). Common readings also available as alternatives.
    WeekdayDefaultWithProperOverride,
    /// Free choice among the proposed reading sets
    /// (weekday, saint's proper/accommodated, or Common)
    PickOne,
}
```

#### `FlexibleRule`

**What it is:** A rule for the flexible orations, where each oration may be chosen independently.

**Why this name:** It is a "rule" for the "flexible" orations (those governed by GIRM 363).

```rust
enum FlexibleRule {
    /// Each oration can be chosen independently from its own list
    PickEachIndependently,
    /// All orations must come from the weekday
    /// (privileged weekdays — except the collect, handled in the identity block)
    WeekdayOnly,
}
```

#### Approach 2 — Example

```
MassCalendar
│
├── "2025-02-10" → [
│   MassComposition {
│       mass_time: DayMass,
│       civil_date: "2025-02-10",
│       liturgical_date: "2025-02-10",
│       context: DayContext { season: OrdinaryTime, week: 5, ... },
│       default_celebration_id: "ord_time_5_mon",
│
│       identity_options: [
│           IdentityOption {                          ← feria
│               celebration_id: "ord_time_5_mon",
│               rank: Weekday,
│               formulary: FormularySet { collect: "...", ... }
│           },
│           IdentityOption {                          ← optional memorial
│               celebration_id: "st_scholastica",
│               rank: OptionalMemorial,
│               formulary: FormularySet { collect: "...", ... }
│           },
│       ],
│
│       readings_options: [
│           ReadingsOption {                          ← weekday (default)
│               source: ProperOfTime("ord_time_5_mon"),
│               category: Weekday,
│               is_default: true,
│               readings: Fixed(ReadingsSet {         ← indivisible
│                   reading_1: "1 Kgs 8:1-7...",
│                   psalm: "Ps 132:6-10",
│                   gospel: "Mk 6:53-56", ...
│               })
│           },
│           ReadingsOption {                          ← Common of Virgins
│               source: Common(Virgins, "st_scholastica"),
│               category: Common,
│               is_default: false,
│               readings: Pool(ReadingsPool {         ← per-component choice
│                   first_readings: ["Song 8:6-7", "1 Cor 7:25-35", ...],
│                   psalms: ["Ps 148:1-2...", "Ps 45:11-12...", ...],
│                   gospels: ["Mt 25:1-13", "Mk 3:31-35", ...],
│                   ...
│               })
│           },
│       ],
│
│       prayer_over_offerings_options: [
│           SourcedText { source: ProperOfTime("..."), text: "..." },
│           SourcedText { source: Common(Virgins, ...), text: "..." },
│       ],
│       prayer_after_communion_options: [ ... ],
│       preface_options: [ ... ],
│       solemn_blessing_options: [],
│       prayer_over_people_options: [],
│
│       composition_rules: CompositionRules {
│           identity: PickOne,
│           readings: WeekdayDefaultWithProperOverride,
│           flexible_orations: PickEachIndependently,
│       }
│   }
│ ]
│
├── "2025-12-24" → [                                  ← SHIFTED
│   MassComposition {
│       mass_time: DayMass,                            ← feria of Dec 24
│       civil_date: "2025-12-24",
│       liturgical_date: "2025-12-24",
│       ...
│   },
│   MassComposition {
│       mass_time: PreviousEveningMass,                ← Christmas eve mass
│       civil_date: "2025-12-24",                      ← shifted here
│       liturgical_date: "2025-12-25",                 ← belongs to Dec 25
│       ...
│   },
│ ]
│
├── "2025-12-25" → [
│   MassComposition { mass_time: NightMass, ... },
│   MassComposition { mass_time: MassAtDawn, ... },
│   MassComposition { mass_time: DayMass, ... },
│ ]
```

---

## Part V — Type Sharing Summary

```
Type                        Approach 1   Approach 2   Shared?
──────────────────────────  ──────────   ──────────   ───────
DayContext                     ✓            ✓          YES
FormularySet                   ✓            ✓ ¹        YES
ReadingText                    ✓            ✓          YES
ReadingsSet                    ✓            ✓ ¹        YES
ReadingsPool                   ✓            ✓ ¹        YES
ReadingsContent                ✓            ✓ ¹        YES
FlexibleOrations               ✓            ✗ ²        APP 1
TextSource                     ✗            ✓          APP 2
SourcedText                    ✗            ✓          APP 2
CelebrationId                  ✓            ✓          YES

LiturgicalCalendar             ✓            ✗          APP 1
LiturgicalDay                  ✓            ✗          APP 1
Celebration                    ✓            ✗          APP 1
CelebrationMass                ✓            ✗          APP 1

MassCalendar                   ✗            ✓          APP 2
MassComposition                ✗            ✓          APP 2
IdentityOption                 ✗            ✓          APP 2
ReadingsOption                 ✗            ✓          APP 2
ReadingsCategory               ✗            ✓          APP 2
CompositionRules               ✗            ✓          APP 2
BlockRule                      ✗            ✓          APP 2
ReadingsRule                   ✗            ✓          APP 2
FlexibleRule                   ✗            ✓          APP 2

Existing types (unchanged)     ✓            ✓          YES
  Season, Rank, Precedence, MassTime, Common, CommonInfo,
  Color, ColorInfo, DayOfWeek, SundayCycle, WeekdayCycle,
  PsalterWeekCycle, PeriodInfo, TitlesDef, MartyrologyEntry,
  CalendarId

¹ Reused inside IdentityOption / ReadingsOption / ReadingsContent
² Exploded into Vec<SourcedText> per oration in Approach 2
```

---

## Part VI — Module Organization

```
core/src/types/
├── shared/                          SHARED TYPES
│   ├── day_context.rs               DayContext
│   ├── text_blocks.rs               FormularySet, ReadingText, ReadingsSet,
│   │                                ReadingsPool, ReadingsContent, FlexibleOrations
│   ├── sourced_text.rs              TextSource, SourcedText
│   └── mod.rs
│
├── liturgical_calendar/             APPROACH 1
│   ├── liturgical_day.rs            LiturgicalDay
│   ├── celebration.rs               Celebration, CelebrationId, CelebrationMass
│   └── mod.rs
│
├── mass_calendar/                   APPROACH 2
│   ├── mass_composition.rs          MassComposition
│   ├── identity_option.rs           IdentityOption
│   ├── readings_option.rs           ReadingsOption, ReadingsCategory
│   ├── composition_rules.rs         CompositionRules, BlockRule,
│   │                                ReadingsRule, FlexibleRule
│   └── mod.rs
│
├── liturgical/                      EXISTING (unchanged)
│   ├── rank.rs                      Rank
│   ├── precedence.rs                Precedence
│   ├── season.rs                    Season
│   ├── cycles.rs                    SundayCycle, WeekdayCycle
│   └── ...
│
├── mass/                            EXISTING (unchanged)
│   ├── mass_time.rs                 MassTime
│   ├── common.rs                    Common
│   └── ...
```

---

## Part VII — Transformation Pipeline

```
Calendar source files (YAML/JSON input)
        │
        ▼
   DayDefinition (existing input type)
        │
        ▼
┌───────────────────────────────────────────┐
│  Calendar engine                          │
│                                           │
│  1. Resolve dates                         │
│  2. Apply precedence rules (GNLY 59)     │
│  3. Assemble Celebrations per day         │
│  4. Resolve liturgical cycle              │
│  5. Populate mass content by GIRM groups  │
└──────────┬────────────────────────────────┘
           │
    ┌──────┴──────┐
    ▼             ▼
 generate_      generate_
 liturgical_    mass_
 calendar()     calendar()
    │             │
    ▼             │
 Liturgical       │  Transformation:
 Calendar         │  • Shift evening masses to previous civil date
    │             │  • Assemble IdentityOption from each Celebration
    │             │  • Assemble ReadingsOption with source + flags
    │             │  • Explode FlexibleOrations into Vec<SourcedText>
    └─────────────│  • Compute CompositionRules from season/precedence
                  ▼
              Mass Calendar (API output)
```

---

## Part VIII — Future Extensibility

This architecture is designed to extend naturally to the **Liturgy of the Hours** in the future. The shared types (`DayContext`, `TextSource`, `ReadingsSet`...) are agnostic to the Mass and can be reused for the Office.

```rust
// Future
impl Calendar {
    fn generate_liturgical_calendar(&self) -> LiturgicalCalendar;  // exists
    fn generate_mass_calendar(&self) -> MassCalendar;              // exists
    fn generate_hours_calendar(&self) -> HoursCalendar;            // future
}
```

The `Celebration` struct in Approach 1 could naturally carry both mass and hours content:

```rust
struct Celebration {
    // ... identity fields ...
    masses: BTreeMap<MassTime, CelebrationMass>,
    hours: BTreeMap<HourTime, CelebrationHour>,  // future
}
```

This would support both the Roman Office and monastic propers (e.g., Benedictine, Cistercian) through the existing calendar inheritance mechanism.
