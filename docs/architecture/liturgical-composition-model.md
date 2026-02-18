---
title: Liturgical Composition Model
description: Architecture and data-modeling reference for Mass & Liturgy of the Hours composition rules in romcal.
tableOfContents:
  minHeadingLevel: 2
  maxHeadingLevel: 4
---

<!-- AI Quick Index — concept/type → section (line numbers are approximate)

LITURGICAL RULES
| Concept                              | Section              | Line   | Key references                    |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| Terminology glossary                 | Terminology          | ~119   |                                   |
| Choice of Mass (season options)      | Part I §1            | ~138   | GIRM 355                         |
| Formulary block (collect+antiphons)  | Part I §2 Group 1    | ~163   | GIRM 363                         |
| Readings block (3 categories)        | Part I §2 Group 2    | ~177   | GIRM 357-358, GILM 83-84         |
| Proper / accommodated / common       | Part I §2 Group 2    | ~186   | GILM 83                          |
| Readings by rank (tables)            | Part I §2 Group 2    | ~202   | GILM 83-84                       |
| Sequence rules                       | Part I §2 Group 2    | ~267   | GIRM 64                          |
| Flexible orations + preface          | Part I §2 Group 3    | ~289   | GIRM 363-365                     |
| Season context summary table         | Part I §3            | ~317   |                                   |
| Visual schema (Mass)                 | Part I §4            | ~340   |                                   |
| Office substitution groups           | Part II §1           | ~389   |                                   |
| Structure of each Hour               | Part II §2           | ~402   |                                   |
| Office by rank (solemnity/feast)     | Part II §3           | ~423   | GILH 225-233                     |
| Memorial overlay rules               | Part II §3c-§4       | ~485   | GILH 234-236                     |
| Memorials on privileged weekdays     | Part II §5           | ~541   | GILH 237-239                     |
| Visual schemas (Office)              | Part II §6           | ~564   |                                   |
| Te Deum rules                        | Part II §7           | ~639   | GILH 68, 228, 231, 235d          |
| Saturday BVM memorial                | Part II §8           | ~657   | GNLY 15, GILH 240                |
| Commons: Office vs. Mass             | Part II §9           | ~667   |                                   |

CROSS-CUTTING CONCERNS
| Concept                              | Section              | Line   | Key references                    |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| Liturgical day vs. celebration       | Part III §1          | ~684   |                                   |
| Cycle resolution                     | Part III §2          | ~707   |                                   |
| Vespers I/II conflict                | Part III §3          | ~713   | GNLY 61, GILH 225, 231           |
| Mass-Office choice independence      | Part III §4          | ~734   | GILH 234, GNLY 14                |
| Office prayer = Mass collect         | Part III §5          | ~746   | CP 44                            |
| Particular calendars & inheritance   | Part III §6          | ~785   | CP 8-16, 23-26, 40-44            |
| Calendar hierarchy (4 levels)        | Part III §6.1        | ~789   | CP 13-16                         |
| Rank by calendar level               | Part III §6.2        | ~816   | CP 8-12, 24-26                   |
| Precedence conflicts                 | Part III §6.3        | ~855   | CP 23                            |
| Proper of Seasons primacy            | Part III §6.4        | ~869   | CP 2                             |
| Proper texts alignment               | Part III §6.5        | ~879   | CP 40, 43-44                     |
| Reading constraints                  | Part III §6.6        | ~912   | CP 41                            |
| Paschal Triduum is not a season      | Part III §7          | ~922   | PS 38-72                         |
| Transfer of impeded solemnities      | Part III §7b         | ~945   | GNLY 60                          |
| Title model                          | Part III §8          | ~958   |                                   |

DATA MODEL (types)
| Type / Section                       | Section              | Line   | Notes                             |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| Type shareability overview           | Part IV §1           | ~1058  | Mass → Office reuse table         |
| JSON serialization convention        | Part IV §2           | ~1078  | Adjacently tagged enums           |
| DayContext                           | Part IV §2           | ~1080  | Shared — temporal context         |
| FormularySet                         | Part IV §2           | ~1148  | Mass — collect + antiphons        |
| AntiphonText                         | Part IV §2           | ~1168  | Shared — antiphon + sources       |
| ReadingText (enriched)               | Part IV §2           | ~1179  | Shared — headline, short form     |
| ShortForm                            | Part IV §2           | ~1203  | Shared — short form ref + text    |
| ReadingsSet (enriched)               | Part IV §2           | ~1209  | Mass — structured reading types   |
| AlleluiaText                         | Part IV §2           | ~1239  | Mass — acclamation + verse        |
| AcclamationType                      | Part IV §2           | ~1251  | Mass — computed by engine         |
| SequenceText                         | Part IV §2           | ~1261  | Mass — sequence hymn              |
| ReadingsPool (enriched)              | Part IV §2           | ~1265  | Mass — divisible per component    |
| ReadingsContent                      | Part IV §2           | ~1297  | Mass — Set or Pool enum           |
| VigilReadingsSequence                | Part IV §2           | ~1310  | Easter/Pentecost vigil            |
| FlexibleOrations (enriched)          | Part IV §2           | ~1352  | Mass — PrefaceText for preface    |
| PrefaceText                          | Part IV §2           | ~1379  | Mass — resolved preface metadata  |
| TextSource                           | Part IV §2           | ~1391  | Shared — provenance               |
| SourcedText                          | Part IV §2           | ~1410  | Shared — text + provenance        |
| SourcedPreface                       | Part IV §2           | ~1423  | L2M — preface + provenance        |
| SourceRef                            | Part IV §2           | ~1434  | Shared — biblical ref + confer    |
| CelebrationId                        | Part IV §2           | ~1451  | Shared — identity (newtype)       |
| LiturgicalCalendar (Layer 1)         | Part IV §3           | ~1463  | Calendar structure (newtype)      |
| LiturgicalDay                        | Part IV §3           | ~1483  | Day wrapper                       |
| Celebration                          | Part IV §3           | ~1512  | Core celebration type             |
| CelebrationMass                      | Part IV §3           | ~1585  | Mass texts per celebration        |
| MassTime                             | Part IV §3           | ~1618  | Existing type reference           |
| Layer 1 example                      | Part IV §3           | ~1671  |                                   |
| CelebrationHour                      | Part IV §3           | ~1796  | Office content per celebration    |
| HourTime                             | Part IV §3           | ~1880  | Hour slots                        |
| HoursPsalmody                        | Part IV §3           | ~1901  | Psalmody structure                |
| PsalmodyEntry (enriched)             | Part IV §3           | ~1916  | Shared — Mass + Office psalmody   |
| PsalmAntiphon                        | Part IV §3           | ~1930  | Shared — antiphon + source        |
| MassCalendar (Layer 2 Mass)          | Part IV §4           | ~1938  | Output structure (newtype)        |
| MassComposition                      | Part IV §4           | ~1958  | Resolved Mass for a date          |
| IdentityChoice                       | Part IV §4           | ~2036  | Celebration choice                |
| ReadingsChoice                       | Part IV §4           | ~2064  | Readings resolution               |
| ReadingsCategory                     | Part IV §4           | ~2086  | Proper/accommodated/common        |
| CompositionRules                     | Part IV §4           | ~2109  | Substitution rules                |
| BlockRule / ReadingsRule / FlexRule   | Part IV §4           | ~2126  | Rule enums                        |
| Layer 2 Mass example                 | Part IV §4           | ~2181  |                                   |
| HoursCalendar (Layer 2 Hours)        | Part IV §5           | ~2270  | Output structure (newtype)        |
| HoursComposition                     | Part IV §5           | ~2299  | Resolved Office for a date        |
| HoursCelebrationChoice               | Part IV §5           | ~2350  | Per-celebration choice            |
| ResolvedHourContent                  | Part IV §5           | ~2379  | Content of one Hour               |
| OfficeReadingsContent                | Part IV §5           | ~2415  | Office of Readings                |
| VigilExtension                       | Part IV §5           | ~2455  | Extended vigil structure           |
| HoursCompositionRules                | Part IV §5           | ~2479  | Office substitution rules         |
| MemorialRule / HourSuppression       | Part IV §5           | ~2492  | Memorial + suppression enums      |
| Layer 2 Hours example                | Part IV §5           | ~2556  |                                   |
| Type sharing summary                 | Part IV §6           | ~2684  |                                   |
| Existing types reference             | Part IV §7           | ~2754  | Common, Precedence, Rank, etc.    |

ARCHITECTURE
| Concept                              | Section              | Line   | Notes                             |
| ------------------------------------ | -------------------- | ------ | --------------------------------- |
| Module organization                  | Part V §1            | ~3163  | File tree                         |
| Transformation pipeline              | Part V §2            | ~3210  | Layer 1 → Layer 2 steps           |
| Calendar API                         | Part V §3            | ~3288  | Public interface                  |
| Combining Hours with Mass            | Part V §4            | ~3302  | GILH 93-98                        |
| Vigil extension & hour suppression   | Part V §5            | ~3312  | GILH 73, 206, 209-215; PS 59, 75 |
| Source references appendix           | Appendix             | ~3438  | GIRM, GNLY, GILM, GILH, CP, PS   |
| Conclusion                           | Conclusion           | ~3582  |                                   |
-->

## Context and Motivation

The Roman Rite's liturgical norms (GIRM, GNLY, GILM, GILH, CP) define precise rules for how liturgical texts — readings, orations, antiphons, psalmody — are selected, combined, and composed for every rank of celebration (solemnity, feast, memorial, weekday), across both the Mass and the Liturgy of the Hours, and at every level of the calendar hierarchy (general, national, diocesan, religious, local).

This document is the companion to [Input Data Model](/architecture/input-data-model), which defines the three-tier input architecture — the data that contributors edit and that the engine transforms into the output types defined here.

This document is a comprehensive architecture and data-modeling reference for romcal. It synthesizes these liturgical rules and proposes a data model that reflects them, organized around three complementary output layers:

- **Layer 1 — Liturgical Calendar** (`generate_liturgical_calendar`): centered on the liturgical day, for internal use and as the foundation for Layers 2 Mass and 2 Hours.
- **Layer 2 Mass — Mass Calendar** (`generate_mass_calendar`): centered on the mass as celebrated on a civil date, with pre-resolved options and explicit composition rules.
- **Layer 2 Hours — Hours Calendar** (`generate_hours_calendar`): centered on the Hours of the Office as celebrated on a civil date, with pre-resolved content and composition rules adapted to the Office's overlay mechanism.

### Terminology

The following terms appear throughout this document with specific meanings:

| Term                        | Definition                                                                                                                                                                                                      |
| --------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Weekday / Feria**         | Synonyms. A day without a feast or memorial as its primary celebration. The code and this document prefer "weekday"; liturgical sources often use "feria."                                                      |
| **Formulary**               | The complete set of texts (collect, antiphons, orations) associated with a specific celebration in the Missal. See Part I §2 Group 1.                                                                           |
| **Proper**                  | (1) Texts specific to a saint or celebration, as opposed to Common texts. (2) _Lectiones propriae_ — readings about the saint or the mystery being celebrated (GILM 83). Context determines the intended sense. |
| **Accommodated**            | Readings that highlight a particular aspect of a saint's life, without being strictly about the saint (GILM 83). Facultative — see Part I §2 Group 2.                                                           |
| **Common**                  | A pool of liturgical texts (prayers, readings, antiphons) organized by category of saint (e.g., Common of Virgins). Used as fallback when no proper text exists.                                                |
| **Indivisible / Divisible** | Indivisible: readings taken as a pre-composed set (weekday, proper). Divisible: readings chosen independently per component from pools (Common). See Part I §2 Group 2.                                         |
| **Privileged weekdays**     | Weekdays of Advent Dec 17-24, Christmas Octave, and Lent (Precedence level 9 — `PrivilegedWeekday_9`). The feria is imposed, with limited memorial interaction.                                                 |

---

## Part I — Liturgical Rules: Mass

### 1. The Choice of Mass (GIRM 355)

GIRM 355 governs the choice of Mass on days with optional memorials. The range of options varies by season:

- **GIRM 355.3 (Ordinary Time weekdays):** Five options — (a) the weekday Mass, (b) the Mass of an optional memorial occurring that day, (c) the Mass of any Saint listed in the _Martyrology_ for that day, (d) a Mass for Various Needs, or (e) a Votive Mass.
- **GIRM 355.2 (Advent before Dec 17, Christmas from Jan 2, Easter):** Four options — (a) the weekday Mass, (b) the Mass of the Saint, (c) the Mass of one of the Saints whose memorial is observed, or (d) the Mass of any Saint listed in the _Martyrology_ for that day.
- **GIRM 355.1 (Advent Dec 17-24, Octave of Christmas, Lent):** The Mass of the current liturgical day is obligatory, with limited borrowing from a memorial (see below).

> **Scope note:** This document models the composition rules for all ranks of celebrations in the General and Particular Calendars (solemnities, feasts, memorials, weekdays), for both Mass and Liturgy of the Hours. The following are not modeled here:
>
> - The Martyrology, Votive Masses, Masses for Various Needs (GIRM 375, 377), and Masses for the Dead (GIRM 381) — valid additional options on Ordinary Time weekdays.
> - **Rogation Days and Ember Days** (GNLY §45-47) — GNLY leaves their time, duration, and manner to the Conferences of Bishops; their Mass is chosen from the Masses for Various Needs (GNLY §47). When a national calendar defines them, they are modeled as particular celebrations within the existing calendar inheritance hierarchy.

GIRM 355 also adds two pastoral directives:

- The priest "will take care not to omit the readings assigned for each day in the Lectionary for weekdays too frequently and without sufficient reason, since the Church desires that a richer portion at the table of God's word be provided for the faithful."
- "Where the optional memorials of the Blessed Virgin Mary or of the Saints are dear to the faithful, the priest should satisfy their legitimate devotion."
- When choosing between a memorial in the General Calendar and one in a diocesan or religious calendar, "preference should be given, all things being equal and in keeping with tradition, to the memorial inscribed in the particular calendar."

Once the celebrant has made this global choice, it determines which formulary block (see §2 Group 1) is used as the base. However, certain elements can then be mixed between sources.

### 2. The Three Substitution Groups

The GIRM organizes mass texts into groups with distinct substitution rules:

#### Group 1 — Formulary Block

**Collect + Entrance Antiphon + Communion Antiphon**

These three elements follow the global choice of celebration as a block. When the memorial is celebrated, all three come from the saint (proper or Common). When the feria is celebrated, all three come from the feria.

> **Architectural note:** The GIRM does not explicitly group these three elements together as an "inseparable block." This grouping is an architectural inference derived from what GIRM 363 makes individually flexible (prayer over the offerings, prayer after Communion) and what it does not — leaving the collect, entrance antiphon, and communion antiphon bound to the chosen formulary. Note that GIRM 48 and 87 (see GIRM 367 in the Appendix) do allow the entrance and communion antiphons to be replaced by other approved chants, which is a separate form of flexibility not modeled here.

The collect is the identifying marker of the celebration. GIRM 363 states: "On memorials of Saints, the collect proper to the day is used, **or, if none is available, one from an appropriate Common.**"

**Exception — Privileged weekdays (GIRM 355.1):** On weekdays of Advent Dec 17-24, Octave of Christmas, and Lent, the Mass of the feria is obligatory. Only the collect may be borrowed from the memorial — this is the only case where the collect is detachable from the antiphons.

**Further exception — Ash Wednesday and Holy Week (GIRM 355.1):** On Ash Wednesday and during Holy Week, even the collect may **not** be borrowed from a memorial. The feria is imposed entirely without exception. See Part III §7 for the Triduum norms (PS 38-72).

#### Group 2 — Readings Block (GIRM 357-358, GILM 83-84)

**Reading 1 + Psalm + (Reading 2) + Alleluia + Gospel**

The degree of flexibility within this block depends on the **category** of the readings:

- **Weekday and proper readings**: These are **indivisible** — they are taken as a complete, pre-composed set. The psalm responds to the first reading it is paired with.
- **Common readings**: These are **divisible by component** — the celebrant may choose each element independently from pools of texts (GILM 71, 89). See below.

##### The three categories of readings for saints (GILM 83)

The GILM (Introduction to the Lectionary for Mass, _Ordo Lectionum Missae_) distinguishes three categories of readings for celebrations of saints:

| Category                                             | Definition (GILM 83)                                                                                                                                  | Rule                                                                                                                                                                                             |
| ---------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Proper readings** (_lectiones propriae_)           | Biblical passages about the Saint or the mystery that the Mass is celebrating. The Order of Readings makes explicit note of every case on a memorial. | **Obligatory** — "Even in the case of a memorial these readings must take the place of the weekday readings for the same day."                                                                   |
| **Accommodated readings** (_lectiones accommodatae_) | Readings that bring out some particular aspect of a Saint's spiritual life or work.                                                                   | **Facultative** — "Use of such readings does not seem binding, except for compelling pastoral reasons." For the most part, references are given to readings in the Commons to facilitate choice. |
| **Common readings** (_lectiones communes_)           | Readings placed in the Commons either for a determined class of Saints (martyrs, virgins, pastors) or for the Saints in general.                      | **Freely chosen** — "It will be up to the priest to choose the one best suited to those listening."                                                                                              |

##### The Common as a permanent alternative (GILM 83)

GILM 83 explicitly states: "In all celebrations of Saints the readings may be taken not only from the Commons to which the references are given in each case, but also from the **Common of Men and Women Saints**, whenever there is special reason for doing so."

This means that **Common readings are always available as alternatives** for any celebration of a saint — even when no accommodated or proper readings are indicated.

##### How GIRM 357-358 articulates with GILM 83

GIRM 357 states that "unless strictly proper readings are given, the readings assigned for the weekday are customarily used" — implying that strictly proper readings, when they exist, override the weekday readings. The term "strictly proper" (_lectiones stricte propriae_) is used in the GIRM but not fully defined there. GIRM 358 offers a partial definition by referring to "proper New Testament readings, that is to say, readings in which mention is made of the Saint being celebrated." The complete technical definition comes from GILM 78-84.

In practical terms, the readings rule for memorials is:

| Lectionary provides                                                                             | Rule                                                              | Reference         |
| ----------------------------------------------------------------------------------------------- | ----------------------------------------------------------------- | ----------------- |
| **Proper readings** (the saint is named or the mystery is directly evoked in the biblical text) | Obligatory — must replace weekday readings                        | GILM 83, GIRM 357 |
| **Accommodated readings** (the Lectionary gives suggestions for the saint)                      | Facultative — may use them, or choose from Common, or keep feria  | GILM 83           |
| **No specific readings** (the Lectionary refers to a Common)                                    | Feria readings by default, but Common readings are also available | GILM 83, GIRM 357 |

##### When the Lectionary provides only one proper reading

When the Lectionary gives only one proper reading for a saint (e.g., only a gospel), that reading **must** be used (it is proper). The other reading(s) can be taken from:

- The **feria** (weekday lectionary)
- The **Common** corresponding to the saint's category
- An **accommodated reading** if the Lectionary indicates one

It is **not** automatically from the feria — the Common is always an option (GILM 83).

**The celebrant cannot mix readings at will** (e.g., taking the gospel from the saint and the first reading from the feria when both proper readings are available). When proper readings exist, they must all be used.

##### Default preference for weekday readings (GIRM 355, GILM 83)

Both GIRM 355 (in its pastoral remarks following 355.3) and GILM 83 emphasize that the priest should take care not to omit the weekday readings too frequently, as the Church desires "a more lavish table of the word of God" (_GILM 83_) through the _lectio continua_.

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

##### Sequence (GIRM 64)

GIRM 64: "The Sequence, which is optional except on Easter Sunday and on Pentecost Day, is sung before the _Alleluia_."

The Sequence is a hymn that precedes the Alleluia (and thus the Gospel). It exists for five celebrations only:

| Celebration                  | Sequence                   | Status                    |
| ---------------------------- | -------------------------- | ------------------------- |
| Easter Sunday                | _Victimae Paschali Laudes_ | **Obligatory** (GIRM §64) |
| Easter Octave (Mon–Sat)      | _Victimae Paschali Laudes_ | Optional                  |
| Pentecost                    | _Veni Sancte Spiritus_     | **Obligatory** (GIRM §64) |
| Corpus Christi               | _Lauda Sion Salvatorem_    | Optional                  |
| Our Lady of Sorrows (Sep 15) | _Stabat Mater_             | Optional                  |

**Consequence for the data model:** The `sequence: Option<String>` field in `ReadingsSet` is `None` on most days and `Some(reference)` on these five celebrations. The engine provides the sequence text; whether it is obligatory or optional is determined by the celebration itself (see table above). The engine does **not** encode `sequence.obligatory: bool` — the consumer must know that Easter Sunday and Pentecost sequences are obligatory (GIRM §64) while the others are optional.

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

The rules vary depending on the liturgical season.

> **Note on "Easter\*" in the table:** "Easter" here means **Easter weekdays outside the Octave**. The Easter Octave (GNLY 24) consists of days celebrated as Solemnities of the Lord (precedence level 2 in the Table of Liturgical Days, entry 2). No memorials are celebrated during the Octave. The expression "weekdays of the Easter Season" in GIRM 355.2 automatically excludes the Octave days because they are not weekdays.
>
> **Note on GIRM 355.1 and Easter:** GIRM 355.1 contains two distinct provisions: (1) the rule for privileged weekdays (Advent 17-24, Octave of Christmas, Lent) where the feria is imposed; and (2) the clarification "On weekdays of the Easter Season, memorials of Saints may rightly be celebrated fully." This second sentence confirms that Easter weekdays follow the 355.2 regime (free choice), not the restricted regime. The word "fully" means all three substitution groups follow the 355.2/355.3 rules — not just collect-borrowing.

| Season                                                                       | GIRM ref | Formulary block                                       | Readings block                                                                         | Flexible orations                   |
| ---------------------------------------------------------------------------- | -------- | ----------------------------------------------------- | -------------------------------------------------------------------------------------- | ----------------------------------- |
| **Ordinary Time weekdays**                                                   | 355.3    | Free choice: feria or memorial ¹                      | Feria default; saint's proper if they exist (obligatory); or Common (always available) | Each independently: feria or Common |
| **Advent before Dec 17, Christmas from Jan 2, Easter\***                     | 355.2    | Free choice: feria or memorial ²                      | Free choice: feria, saint's proper, or Common                                          | Each independently                  |
| **Advent Dec 17-24, Octave of Christmas, Lent** (except Ash Wed & Holy Week) | 355.1    | Feria imposed (only collect borrowable from memorial) | Feria imposed ³                                                                        | Feria imposed                       |
| **Ash Wednesday, Holy Week**                                                 | 355.1    | Feria imposed entirely (no collect borrowing)         | Feria imposed                                                                          | Feria imposed                       |

¹ GIRM 355.3 also allows Martyrology saints, Masses for Various Needs, and Votive Masses (out of scope for this model).
² GIRM 355.2 also allows the Mass of any Saint listed in the _Martyrology_ (out of scope for this model).
³ Exception: strictly proper readings (GILM 83 — "proper readings [...] must take the place of the weekday readings") still override feria readings even on privileged weekdays.

**Lenten demotion (GNLY 14, 59 entry 12):** Obligatory Memorials that fall on Lenten weekdays may only be celebrated as Optional Memorials. They follow the same "special manner" as optional memorials on privileged weekdays (collect-borrowing only, except Ash Wednesday and Holy Week).

**No commemorations (Notitiae R8, see `notitiae-responses.mdx` R8):** The reformed liturgy has eliminated the practice of commemorations. When a solemnity occurs on an Advent or Lent weekday, only the solemnity is celebrated — no elements of the weekday are added as a "commemoration" in either Mass or Office. The composition model therefore has no commemoration mechanism.

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

---

## Part II — Liturgical Rules: Liturgy of the Hours

> **Scope:** This section provides the complete rules analysis for the Liturgy of the Hours, establishing the normative foundation for the Office data model (Part IV §3, §5) and the Hours transformation pipeline (Part V §2).

### 1. Office Substitution Groups (vs. Mass)

The Mass has three substitution groups (Part I §2). The Office has fundamentally **different** groups on memorials:

| Group                 | Mass (GIRM)                                                                      | Office (GILH)                                                                                                                                                                                                                   |
| --------------------- | -------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Psalmody**          | Entrance/communion antiphons follow the formulary choice                         | Psalms + psalm antiphons ALWAYS from the current weekday psalter (GILH 62, 134)                                                                                                                                                 |
| **Identifying texts** | Formulary block: collect + entrance/communion antiphons (inseparable)            | Proper elements: invitatory antiphon, hymn, short reading, canticle antiphons (Benedictus/Magnificat), intercessions — from saint's Proper, Common, or weekday (GILH 235b). Concluding prayer mandatory from saint (GILH 235c). |
| **Readings**          | Scripture readings: fixed set or pool (GIRM 357, GILM 71)                        | Office of Readings: 1st reading from Scripture cycle + 2nd reading patristic or hagiographical (GILH 64, 67)                                                                                                                    |
| **Flexible orations** | Prayer over offerings, prayer after Communion: individually choosable (GIRM 363) | Not applicable — the Office has no equivalent flexible orations                                                                                                                                                                 |

**Key architectural difference:** In the Mass, the entrance and communion antiphons follow the formulary choice (inseparable from the collect). In the Office, psalm antiphons stay with the psalter on memorials, while only the canticle antiphons (at Benedictus and Magnificat) can come from the saint. This means `FormularySet` cannot be reused for the Office — the Office needs a different structure.

### 2. Structure of Each Hour

Before analyzing how celebrations affect the Office, it is necessary to understand what elements each Hour contains. The GILH defines 7 Hours (8 counting Vespers I), each with a specific structure:

| Hour                                      | Elements                                                                                                                                                                          | GILH ref    |
| ----------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------- |
| **Invitatory**                            | Invitatory antiphon + Psalm 95 (or 100, 67, 24)                                                                                                                                   | GILH §34-36 |
| **Office of Readings**                    | Hymn, 3 psalms with antiphons, ℣, Scripture reading + responsory, patristic/hagiographical reading + responsory, Te Deum (when applicable), concluding prayer                     | GILH §55-73 |
| **Lauds** (Morning Prayer)                | Hymn, morning psalm + OT canticle + praise psalm (with antiphons), short reading + short responsory, Benedictus canticle + antiphon, intercessions, Our Father, concluding prayer | GILH §37-54 |
| **Terce / Sext / Nones** (Daytime Prayer) | Hymn, 3 psalms with antiphons, short reading + ℣, concluding prayer                                                                                                               | GILH §74-83 |
| **Vespers** (Evening Prayer)              | Hymn, 2 psalms + NT canticle (with antiphons), short reading + short responsory, Magnificat canticle + antiphon, intercessions, Our Father, concluding prayer                     | GILH §37-54 |
| **Compline** (Night Prayer)               | Examination of conscience, hymn, psalm(s) with antiphon, short reading + ℣, Nunc Dimittis + antiphon, concluding prayer, Marian antiphon                                          | GILH §84-92 |

**Architectural implications:**

- **Invitatory**: Precedes the first Hour of the day (normally Office of Readings or Lauds). Its antiphon varies by celebration. On memorials, it follows the GILH §235b priority: saint's Proper → Common → weekday.
- **Lauds and Vespers** are the two "principal Hours" (GILH §37) and have the richest variation per celebration: canticle antiphon (Benedictus/Magnificat), intercessions, hymn, short reading.
- **Office of Readings** has two readings with responsories — the most complex element affected by memorial rules (GILH §235d, GILH §239a).
- **Daytime Prayer** (Terce/Sext/Nones): On memorials, entirely from the weekday (GILH §236). On solemnities, proper texts. The celebrant normally chooses ONE of the three unless bound to all three (clerics with choral obligation). The GILH (GILH §175-178) provides two psalmody schemes: the "current" (from the psalter week) and the "complementary" (for those who pray all three). **Consequence for the data model:** Each `HoursComposition` for Terce/Sext/Nones carries a single `HoursPsalmody` — the "current" scheme (psalter week). The "complementary" scheme is a fixed redistribution of psalms 120-128 across the three Hours, deterministic from the psalter structure. The engine does not need to provide both: a consumer who prays all three Hours can derive the complementary scheme from the psalter, or the engine can offer a configuration option.
- **Compline** is the most stable Hour: almost always from the weekday psalter, unaffected by memorials (GILH §236). It only varies on solemnities (proper antiphon for the Nunc Dimittis) and in the exceptional suppression cases (GILH §211, GILH §215).

### 3. How the Office is Arranged by Rank

The GILH (Chapter IV, GILH §225-244) defines distinct rules for each rank. This section analyzes the rules for each rank.

#### 3a. On Solemnities (GILH 225-230)

Solemnities receive the most complete proper treatment. **Everything** is from the Proper or Common:

| Element                                       | Source                                                                                                                                                                                                          | GILH ref                 |
| --------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------ |
| **Vespers I** (Evening Prayer I)              | Proper/Common — begins the solemnity on the preceding evening                                                                                                                                                   | GILH §225                |
| **Invitatory antiphon**                       | Proper/Common                                                                                                                                                                                                   | GILH §225                |
| **Hymn**                                      | Proper/Common                                                                                                                                                                                                   | GILH §225                |
| **Psalmody**                                  | Per Hour — see detail below                                                                                                                                                                                     | GILH §134, GILH §225-229 |
| **Antiphons** (psalm + canticle)              | Proper/Common                                                                                                                                                                                                   | GILH §225                |
| **Short reading, short responsory**           | Proper/Common                                                                                                                                                                                                   | GILH §225                |
| **Canticle antiphon** (Benedictus/Magnificat) | Proper/Common                                                                                                                                                                                                   | GILH §225                |
| **Intercessions**                             | Proper/Common                                                                                                                                                                                                   | GILH §225                |
| **Concluding prayer**                         | Proper/Common                                                                                                                                                                                                   | GILH §225                |
| **Office of Readings — 1st reading**          | Proper/Common (may differ from weekday cycle)                                                                                                                                                                   | GILH §228                |
| **Office of Readings — 2nd reading**          | Proper/Common (patristic or hagiographical)                                                                                                                                                                     | GILH §228                |
| **Te Deum**                                   | **Said**                                                                                                                                                                                                        | GILH §228                |
| **Daytime Prayer**                            | Proper hymn, proper antiphons, proper short reading and prayer; psalmody may use the "gradual" psalms (Ps 120-128) unless proper                                                                                | GILH §229                |
| **Compline**                                  | "Everything is said as on Sundays, after evening prayer I and II respectively" — i.e., Compline after Vespers I uses the Sunday-after-EP-I scheme; Compline after Vespers II uses the Sunday-after-EP-II scheme | GILH §230                |

**Psalmody detail by Hour on Solemnities (GILH §134, GILH §226-229):**

| Hour                   | Psalms source                                                                                                                   | Reference            |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------- | -------------------- |
| **Vespers I**          | Laudate Psalms (Ps 113, 117, 135, 146, 147A, 147B), following ancient custom                                                    | GILH §134, GILH §226 |
| **Office of Readings** | Proper psalms from tradition                                                                                                    | GILH §134, GILH §228 |
| **Lauds**              | Psalms from Sunday of Week I                                                                                                    | GILH §134, GILH §227 |
| **Daytime Prayer**     | Gradual Psalms (Ps 120-128) with proper antiphon; on Sundays: Sunday of Week I; certain solemnities of the Lord: special psalms | GILH §134, GILH §229 |
| **Vespers II**         | Proper psalms and canticle                                                                                                      | GILH §134, GILH §226 |
| **Compline**           | After Vespers I → Sunday scheme; after Vespers II → Sunday scheme                                                               | GILH §230            |

**Key points for the data model:**

- Solemnities need **full content for every Hour** — all fields populated, nothing from the weekday.
- Vespers I exists (unlike feasts and memorials) — the `VespersI` entry in `HourTime` is primarily for solemnities.
- The psalmody may be proper (unlike memorials where psalms come from the weekday psalter).
- GNLY 11: "Some Solemnities are also endowed with their own Vigil Mass" — the Mass vigil (`PreviousEveningMass`) is distinct from the Office Vespers I. Both may exist on the same evening.

#### 3b. On Feasts (GILH 231-233)

Feasts are celebrated like solemnities but with two significant restrictions:

| Difference from Solemnity               | Rule                                                                                                                                                                                                                                                    | GILH ref               |
| --------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------- |
| **No Vespers I**                        | Feasts are "celebrated within the limits of the natural day" (GNLY 13). No Evening Prayer I.                                                                                                                                                            | GILH §231              |
| **Exception: Lord's Feasts on Sundays** | When a Feast of the Lord falls on a Sunday in OT or Christmas Time, it replaces the Sunday Office — including Vespers I from Saturday evening (GNLY 13). Notitiae R2 confirms this applies specifically to the Holy Family and the Baptism of the Lord. | GILH §231, Notitiae R2 |
| **Te Deum**                             | **Said** (same as solemnities)                                                                                                                                                                                                                          | GILH §231              |
| **Office of Readings, Lauds, Vespers**  | "All is done as on solemnities" — Proper/Common                                                                                                                                                                                                         | GILH §231              |
| **Daytime Prayer**                      | Hymn: weekday (always). Psalms + antiphons: weekday (unless special tradition requires a proper antiphon). Short reading: **Proper/Common**. Concluding prayer: **Proper/Common**.                                                                      | GILH §232              |
| **Compline**                            | "As on ordinary days" — from the weekday                                                                                                                                                                                                                | GILH §233              |

**Key points for the data model:**

- The `VespersI` entry in `HourTime` is NOT generated for feasts (except Lord's Feasts on Sundays that replace the Sunday Office).
- Most feast content is the same as solemnities — the main difference is temporal scope.
- Daytime Prayer is simpler than on solemnities: weekday hymn, weekday psalms and antiphons (rarely a proper antiphon from tradition), but both the short reading and concluding prayer are proper (GILH §232).

#### 3c. On Memorials — Summary Table

For reference, the complete comparison by rank:

Legend: OT = Ordinary Time, priv. = privileged weekdays (Advent 17-24, Christmas Octave, Lent).

```
                        Solemnity        Feast           Memorial (OT)      Memorial (priv.)     Weekday
                        ─────────        ─────           ──────────────      ────────────────     ───────
Vespers I               Proper/Common    No ¹            No                  No                   No
Invitatory antiphon     Proper/Common    Proper/Common   GILH §235b ²             Weekday              Weekday
Hymn                    Proper/Common    Proper/Common   GILH §235b               Weekday              Weekday
Psalmody                Proper ³         Proper ³        Weekday psalter     Weekday psalter      Weekday psalter
Psalm antiphons         Proper/Common    Proper/Common   Weekday psalter     Weekday psalter      Weekday psalter
Short reading           Proper/Common    Proper/Common   GILH §235b               Weekday              Weekday
Short responsory        Proper/Common    Proper/Common   GILH §235b               Weekday              Weekday
Canticle antiphon       Proper/Common    Proper/Common   GILH §235b               Weekday + saint ⁴ᵃ   Weekday
Intercessions           Proper/Common    Proper/Common   GILH §235b               Weekday              Weekday
Concluding prayer       Proper/Common    Proper/Common   Saint (mandatory)   Saint ⁴ᵇ             Weekday
OdR 1st reading         Proper/Common    Proper/Common   Scripture cycle     Scripture cycle       Scripture cycle
OdR 2nd reading         Proper/Common    Proper/Common   Saint/Common ⁵      Weekday + saint ⁶   Patristic
Te Deum                 Yes              Yes             No                  No                   No
Daytime Prayer          Proper           Wkday + rdr/pr  Weekday (GILH §236)      Weekday              Weekday
Compline                Special ³        Weekday         Weekday (GILH §236)      Weekday              Weekday

¹ Except Lord's Feasts falling on Sunday (GNLY 13)
² GILH §235b priority: Saint's Proper → Common → weekday
³ Per-Hour psalm detail: see §3a Psalmody table above; Compline uses Sunday scheme (GILH §230)
⁴ᵃ GILH §239b: Saint's antiphon (Benedictus/Magnificat) and prayer appended to Lauds/Vespers
⁴ᵇ GILH §239a (OdR): Saint's concluding prayer replaces weekday; GILH §239b (Lauds/Vespers): Saint's prayer appended
⁵ GILH §235d: Hagiographical reading replaces patristic reading
⁶ GILH §239a: Hagiographical reading added AFTER patristic (not replacing)
```

### 4. Memorial Rules in Ordinary Time (GILH 234-236)

> This section corresponds to the "Memorial (OT)" column in the summary table above (§3c).

GILH 234: "There is no difference in the arrangement of the Office for obligatory and optional memorials except in the case of optional memorials falling during privileged seasons." (Note: "privileged seasons" in GILH §234 refers to privileged weekdays — see §5 below.)

The rules of GILH §235-236 apply **identically** to both obligatory and optional memorials in Ordinary Time:

| Element                                        | Source (GILH 235-236)                                                   | Flexibility                     |
| ---------------------------------------------- | ----------------------------------------------------------------------- | ------------------------------- |
| **Psalms + psalm antiphons**                   | Current weekday psalter (GILH §235a)                                    | Fixed (unless proper indicated) |
| **Invitatory antiphon, hymn, short reading**   | Saint's Proper → Common or weekday (GILH §235b)                         | Flexible (priority order)       |
| **Canticle antiphons** (Benedictus/Magnificat) | Saint's Proper → Common or weekday (GILH §235b)                         | Flexible (priority order)       |
| **Intercessions**                              | Saint's Proper → Common or weekday (GILH §235b)                         | Flexible (priority order)       |
| **Concluding prayer**                          | From the Office of the saint (GILH §235c)                               | **Mandatory** from saint        |
| **Office of Readings — 1st reading**           | Current Scripture cycle (GILH §235d)                                    | Fixed                           |
| **Office of Readings — 2nd reading**           | Saint's proper or Common; current patristic if none exists (GILH §235d) | From saint/Common               |
| **Te Deum**                                    | Not said (GILH §235d)                                                   | Fixed (omitted)                 |
| **Daytime Prayer, Night Prayer**               | Entirely from weekday (GILH §236)                                       | Fixed                           |

**Comparison with Mass:** In the Mass, the collect is part of the formulary block choice — it comes from whichever celebration is chosen. In the Office, the concluding prayer is **always mandatory from the saint** (GILH §235c) — it is the one element that unambiguously identifies the memorial. The flexible elements (GILH §235b) follow a priority order: saint's Proper if given, otherwise Common or weekday — unlike the Mass's flexible orations (GIRM 363) which are freely choosable between sources without priority.

### 5. Memorials on Privileged Weekdays (GILH 237-239)

> This section corresponds to the "Memorial (priv.)" column in the summary table above (§3c).

Privileged weekdays are the weekdays of Advent Dec 17-24, the Christmas Octave, and all Lent weekdays (Precedence level 9 — `PrivilegedWeekday_9`). Note: Advent Dec 1-16 weekdays are NOT privileged (level 13, regular weekdays).

The Office handles memorials on privileged weekdays differently from the Mass, using an **addition** mechanism rather than a **substitution** mechanism:

**GILH §237 — Complete exclusion:** On Sundays, solemnities, feasts, Ash Wednesday, Holy Week, and during the Easter Octave, "no regard is taken of any memorials." This parallels the Mass rule (GIRM 355.1 exception for Ash Wednesday/Holy Week).

**GILH §238 — Demotion:** On weekdays of Advent Dec 17-24, Christmas Octave, and Lent, "no obligatory memorials are celebrated, even in particular calendars." For Lent specifically: "When any happen to fall during Lent in a given year, they are treated as optional memorials." This parallels GNLY 14 (Lenten demotion). Note the distinction: during Lent, obligatory memorials are explicitly demoted to optional; during Dec 17-24 and Christmas Octave, they are simply not celebrated — though GILH §239 additions remain available for any memorial in all three periods.

**GILH §239 — Limited additions:** On these privileged weekdays, if the celebrant wishes to mark the saint's memorial:

- **(a) Office of Readings:** A hagiographical reading may be **added after** the patristic reading (with its responsory), not replacing it. The concluding prayer of the saint is used (replacing the weekday prayer).
- **(b) Morning/Evening Prayer:** The ending of the weekday concluding prayer may be omitted, and the saint's antiphon (from Proper or Common, for the Benedictus or Magnificat) and prayer may be **appended** to the Hour.

> **Note on concluding prayer:** GILH §239a and GILH §239b have different mechanisms. In the Office of Readings (GILH §239a), the saint's concluding prayer _replaces_ the weekday prayer. In Lauds/Vespers (GILH §239b), the saint's prayer is _appended_ alongside the weekday prayer (whose ending is omitted). This distinction matters for the data model.

**Key difference from the Mass:** In the Mass on privileged weekdays (GIRM 355.1), the collect is **borrowed** from the memorial, replacing the weekday collect — a substitution. In the Office (GILH §239), no weekday element is removed or replaced — the saint's elements are **added alongside** the weekday elements. This is architecturally significant: the Mass model uses `BlockRule::ForcedCollectBorrowable` (a substitution rule), but the Office uses `MemorialRule::AdditionsOnly` (Part IV §5), which models the "append" mechanism with no equivalent in the Mass's `BlockRule` enum.

**How the additions materialize in the data model:** The GILH §239 additions (hagiographical reading, saint's antiphon + prayer) are resolved by the engine in Layer 2 Hours and appear in `ResolvedHourContent` and `OfficeReadingsContent`. Specifically: the hagiographical reading appears in `OfficeReadingsContent.hagiographical_reading` alongside the preserved `patristic_reading` (both `Some`); the saint's canticle antiphon and concluding prayer appear in `ResolvedHourContent.canticle_antiphon` and `ResolvedHourContent.concluding_prayer` with `source: ProperOfSaint`. The `MemorialRule::AdditionsOnly` variant (Part IV §5) signals to consumers that these elements are additions, not substitutions.

### 6. Visual Schemas for Office Composition

```
MEMORIAL IN ORDINARY TIME — OFFICE OVERLAY MECHANISM (GILH 235)
═══════════════════════════════════════════════════════════════

  Which celebration? ──────────────────────
  │                                        │
  ▼                                        ▼
WEEKDAY (feria)                        MEMORIAL
(all from weekday)                         │
                         ┌─────────────────┼──────────────────┐
                         ▼                 ▼                  ▼
                   OVERLAY (GILH §235)    MANDATORY (GILH §235c)    READINGS (GILH §235d)
                   Engine merges     Concluding prayer    Office of Readings
                   weekday base +    ALWAYS from saint
                   saint's proper                        ┌─ 1st: Scripture
                         │                               │  (weekday cycle)
                  From saint if      ← if none exist,   │
                  proper exists,       Common or         ├─ 2nd: Saint/Common
                  otherwise from       weekday ─→        │  REPLACES patristic
                  Common/weekday:
                  • Invitatory ant.                      └─ Te Deum: NOT said
                  • Hymn
                  • Short reading                        Daytime Prayer,
                  • Canticle ant.                        Compline: entirely
                    (Benedictus/                         from weekday (GILH §236)
                     Magnificat)
                  • Intercessions

  Psalms + psalm antiphons: ALWAYS from weekday psalter (GILH §235a)
  ─────────────────────────────────────────────────────────────
```

```
PRIVILEGED WEEKDAYS — OFFICE ADDITION MECHANISM (GILH 239)
══════════════════════════════════════════════════════════════

  Base = WEEKDAY (imposed — no substitution)
  │
  ├── Office of Readings (GILH §239a):
  │   ├── 1st reading: Scripture cycle (UNCHANGED)
  │   ├── 2nd reading: Patristic (KEPT — not replaced)
  │   ├── + ADDED AFTER: hagiographical reading + responsory
  │   └── Concluding prayer: SAINT'S (replaces weekday)
  │
  ├── Lauds / Vespers (GILH §239b):
  │   ├── All elements: weekday (UNCHANGED)
  │   ├── Concluding prayer ending: OMITTED
  │   └── + APPENDED: saint's antiphon (Benedictus/Magnificat)
  │       + Saint's prayer (alongside weekday prayer)
  │
  └── Other Hours: weekday only (no additions)

  ⚠ No additions at all on:
     Ash Wednesday, Holy Week, Easter Octave,
     Sundays, Solemnities, Feasts (GILH §237)
```

```
COMPARISON: MASS vs. OFFICE ON PRIVILEGED WEEKDAYS
═══════════════════════════════════════════════════

  MASS (GIRM 355.1)                      OFFICE (GILH 239)
  ─────────────────                       ─────────────────
  Mechanism: SUBSTITUTION                 Mechanism: ADDITION
  • Collect borrowed FROM memorial        • OdR (GILH §239a): hagiographical reading
    (replaces weekday collect)              ADDED after patristic; saint's
  • Rest: weekday imposed                   concluding prayer REPLACES weekday
                                          • Lauds/Vespers (GILH §239b): saint's
                                            antiphon + prayer APPENDED
                                          • Patristic reading KEPT (not replaced)
  Data model: BlockRule::ForcedCollectBorrowable     Data model: MemorialRule::AdditionsOnly
```

### 7. Te Deum Rules (GILH 68, 228, 231, 235d)

The Te Deum is a hymn of praise sung after the second reading in the Office of Readings. Its presence or absence depends on the rank and season:

| Context                                     | Te Deum                                     | Reference  |
| ------------------------------------------- | ------------------------------------------- | ---------- |
| **Solemnities**                             | Said                                        | GILH §228  |
| **Feasts**                                  | Said                                        | GILH §231  |
| **Days within Octaves** (Easter, Christmas) | Said                                        | GILH §68   |
| **Sundays** (outside Lent)                  | Said                                        | GILH §68   |
| **Sundays** (in Lent)                       | **Not said**                                | GILH §68   |
| **Memorials** (all seasons)                 | **Not said**                                | GILH §235d |
| **Weekdays** (all seasons)                  | **Not said**                                | GILH §68   |
| **Vigil form** of Office of Readings        | Said (after the vigil canticles and Gospel) | GILH §73   |
| **Easter Vigil reduced form** (absentees)   | Said                                        | GILH §212  |

**Consequence for the data model:** The `te_deum: bool` field in `OfficeReadingsContent` is determined by the engine based on rank and season. On memorials and weekdays it is always `false`. On solemnities, feasts, and Sundays (outside Lent) it is `true`. This is a computed field, not a liturgical choice.

### 8. Saturday BVM Memorial (GNLY 15, GILH 240)

GNLY 15: "On Saturdays in Ordinary Time when no Obligatory Memorial occurs, an Optional Memorial of the Blessed Virgin Mary may be celebrated."

GILH 240 confirms this applies to the Office: the Saturday BVM memorial is celebrated as other optional memorials (GILH §235-236 rules).

**Key point:** This is a **structural option** generated by a general norm, not inscribed in a specific calendar. It exists on every OT Saturday without an obligatory memorial. The engine must generate this option automatically, adding a `HoursCelebrationChoice` for the BVM memorial with content from the Common of the Blessed Virgin Mary.

This parallels the Mass model where the BVM Saturday memorial generates an `IdentityChoice` (Part IV §4) and `ReadingsChoice` (Part IV §4).

### 9. Commons in the Office vs. Mass

The role of the Common differs between Mass and Office:

| Aspect               | Mass                                                                                                                 | Office                                                                          |
| -------------------- | -------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------- |
| **When used**        | When no proper texts exist (GIRM 363)                                                                                | When no proper texts exist (GILH §235b, GILH §235d)                             |
| **Readings**         | Commons provide pools per reading position (GILM 71) — component-level choice                                        | Commons provide a complete set per Hour — less granular                         |
| **Choice freedom**   | "The celebrant may choose at will" (GILM 71)                                                                         | Priority order: Proper → Common → weekday (GILH §235b)                          |
| **Multiple Commons** | The saint's category determines the primary Common, but Common of Men and Women Saints is always available (GILM 83) | Similarly, multiple Commons may be indicated, and broader Commons are available |

**Consequence for the data model:** The `commons: Vec<CommonInfo>` field in `HoursCelebrationChoice` lists the applicable Commons for that celebration, allowing the engine to resolve texts from the correct Common when the saint's Proper is absent.

---

## Part III — Cross-Cutting Concerns

### 1. Vocabulary: Liturgical Day vs. Celebration

The GNLY uses both terms with distinct meanings:

**Liturgical day** (_dies liturgicus_) — GNLY 3:

> "The liturgical day runs from midnight to midnight. However, the celebration of Sunday and of Solemnities begins already on the evening of the previous day."

The liturgical day is the **temporal frame**: a calendar date that can host one or more celebrations.

**Celebration** (_celebratio_) — GNLY 10:

> "Celebrations, according to the importance assigned to them, are hence distinguished one from another and termed: Solemnity, Feast, Memorial."

A celebration is the **liturgical entity** with a rank, a name, and texts. The feria is a celebration. An optional memorial is a different celebration. Both can coexist on the same liturgical day.

#### Consequence for the data model

- **`LiturgicalDay`** = the container (one per civil date). Carries the shared temporal context (season, cycle, psalter week...). See Part IV §3 for the type definition.
- **`Celebration`** = the content (one or more per liturgical day). Carries identity (name, rank, precedence) and mass texts. See Part IV §3 for the type definition.

A single liturgical day can contain multiple celebrations: the feria as the primary celebration, plus any optional memorials as alternatives.

### 2. Cycle Resolution

The engine computes the applicable liturgical cycle for any given date (Year A/B/C for Sundays, Year 1/2 for weekdays). Therefore, the output data model does **not** include a cycle dimension — the engine resolves the correct cycle internally and returns only the applicable content. In `CelebrationMass.readings`, only the readings of the applicable cycle are present — not all three years.

The cycle information remains available in `DayContext` (`sunday_cycle`, `weekday_cycle`) for informational purposes, but the mass texts are already those of the resolved cycle.

### 3. Vespers I/II Conflict Resolution (GNLY 61, GILH 225, 231)

When the evening of a day is both the conclusion of one celebration and the beginning of another (Vespers I of a solemnity), a conflict arises. GNLY 61 provides the rule:

> "Should Vespers (Evening Prayer) of the current day's Office and First Vespers (Evening Prayer I) of the following day be assigned for celebration on the same day, then Vespers (Evening Prayer) of the celebration with the higher rank in the Table of Liturgical Days takes precedence; in cases of equal rank, Vespers (Evening Prayer) of the current day takes precedence."

In practice:

- **Vespers I of a solemnity** usually wins over regular Vespers, because solemnities have higher precedence.
- **Equal rank** (rare): the current day's Vespers takes precedence.
- **Feasts have no Vespers I** (GNLY 13, GILH §231), so no conflict arises — except Lord's Feasts falling on Sunday.

**Consequence for the data model:** In the `HoursCalendar` (Layer 2 Hours), the engine must resolve this conflict during the transformation from Layer 1:

1. Check if the following liturgical day has a Vespers I (only solemnities and Lord's Feasts on Sundays).
2. Compare precedence with the current day's Vespers per GNLY 61.
3. Generate only the winning Vespers entry for that civil date evening. The `HoursComposition.replaces_vespers_of` field identifies the losing celebration, enabling a pastoral note for consumers (e.g., "First Vespers of Sunday. Vespers of the Saturday are omitted.").
4. The losing Vespers is omitted entirely — it does not appear as an option.

This is different from the Mass model where `PreviousEveningMass` and `DayMass` of the same civil date coexist as separate entries. For the Office, only one Vespers is celebrated — there is no "pick one" mechanism. The engine makes the determination.

### 4. Mass-Office Choice Independence (GILH 234, GNLY 14)

A key architectural question: when both Mass and Office are celebrated on a day with optional memorials, must the **same** celebration be chosen for both?

The GILH and GIRM do not explicitly address this question. In liturgical practice:

- **GNLY 14** states: "If several Optional Memorials are inscribed in the Calendar on the same day, only one may be celebrated." This applies to the celebration as a whole — Mass and Office together constitute "celebrating" a memorial.
- **GILH 234** links the Office to the Mass norms: memorials "are integrated into the celebration of the occurring weekday in accordance with the norms set forth in the General Instruction of the Roman Missal and of the Liturgy of the Hours."
- In practice, the same celebration is chosen for both Mass and Office on the same day — celebrating St. Scholastica at Mass and the feria at the Office, or vice versa, is not consistent with how GNLY 14 frames celebration as a single act.

**Consequence for the data model:** Both `MassComposition` and `HoursComposition` carry a `default_celebration_id` field. For a given civil date, these fields must be consistent — the consumer must choose the same celebration for both Mass and Office. This is enforced by convention, not by a shared structural link: each Layer 2 output (Mass Calendar, Hours Calendar) is self-contained, but the consumer is responsible for keeping the choice in sync across both. This design preserves the independence of each calendar while reflecting the liturgical norm that "celebrating" a memorial is a single act spanning both Mass and Office.

### 5. Office Prayer = Mass Collect (CP 44) — Shared `Celebration.prayer`

CP §44 states a cross-domain identity rule: "The prayer is always the same as the opening prayer of the Mass." This means the Office concluding prayer and the Mass collect are **the same text** for a given celebration.

**Data model:** The `Celebration` struct carries a `prayer: Option<String>` field — the canonical prayer of the celebration, stored once. Both domains resolve to it:

```
Celebration.prayer            ← single source of truth (CP §44)
    │
    ├──► FormularySet.collect            (Mass)
    │    None → resolves to Celebration.prayer
    │    Some(...) → override for this specific Mass time
    │
    └──► CelebrationHour.concluding_prayer  (Office)
         None → resolves to Celebration.prayer
         Some(...) → override for this specific Hour
```

**Resolution chain** (applied by the engine in Layers 2 Mass and 2 Hours):

| Step                | Mass collect                     | Office concluding prayer                      |
| ------------------- | -------------------------------- | --------------------------------------------- |
| 1. Field override   | `FormularySet.collect` if `Some` | `CelebrationHour.concluding_prayer` if `Some` |
| 2. Canonical prayer | `Celebration.prayer` if `Some`   | `Celebration.prayer` if `Some`                |
| 3. Fallback         | Common of the saint (GIRM 363)   | Common of the saint or weekday (GILH §235c)   |

**Why `Celebration.prayer` and not duplication:**

- On memorials, GILH §235c makes the concluding prayer mandatory from the saint — and it is the same text that serves as the Mass collect. Storing it once ensures consistency.
- The identity reinforces the shared `Celebration` entity design (Layer 1): the `Celebration` is the unifying concept across Mass and Office.
- In Layers 2 Mass and 2 Hours, the resolved text appears in both `IdentityChoice.formulary_set.collect` and `ResolvedHourContent.concluding_prayer` — identical content, traceable to the same source.

**When `FormularySet.collect` overrides `Celebration.prayer`:**
Multi-Mass celebrations (e.g., Christmas: Vigil, Night, Dawn, Day) have distinct collects per Mass time. Each `FormularySet` provides its own `collect`, and `Celebration.prayer` typically holds the DayMass collect (or is `None` if all four are distinct). This override is rare — most celebrations have a single Mass with a single collect.

**Exception — Night Prayer (Compline):** GILH §198 notes that at Night Prayer, "the prayer is always the prayer given in the psalter for that hour." The CP §44 identity does **not** apply to Compline. The engine must never resolve Compline's concluding prayer from `Celebration.prayer`. This exception applies universally — even on solemnities.

**Hours where CP §44 applies:** Lauds, Vespers, Office of Readings, and Daytime Prayer (on feasts/solemnities where the concluding prayer is "from the proper"). On memorials, GILH §235c governs: the concluding prayer is mandatory from the saint at any Hour where it is said (Lauds, Vespers, Office of Readings) — and that text is `Celebration.prayer`.

### 6. Particular Calendars and Calendar Inheritance (CP)

> **Source:** _Calendaria Particularia_ (CP), Instruction from the Congregation for Divine Worship, 24 June 1970 (Notitiae 58, 1970). This document defines how particular calendars (diocesan, national, religious) are constructed by layering proper celebrations onto the General Calendar.

#### 1. Calendar Inheritance Hierarchy (CP 13-16)

CP §13 defines the fundamental principle: "A particular calendar is formed by the insertion of particular celebrations into the General Calendar."

The layering works as follows:

```
                    General Calendar (base)
                            │
              ┌─────────────┼──────────────┐
              ▼                             ▼
    National / Regional              Religious Order
    Calendar (CP 14)                 Calendar (CP 16)
              │                             │
              ▼                             ▼
    Diocesan Calendar                Religious Province
    (CP 15a-b)                       Calendar (CP 16c)
              │                             │
              ▼                             ▼
    Local / Church                   House / Church
    Calendar (CP 15c)                Calendar (CP 16c)
```

**Cross-layering rule (CP §16d):** "Members of religious institutes join with the local Church in celebrating the anniversary of the dedication of the cathedral and the feast of the principal patrons of both the place and the wider area in which they reside."

**Consequence for the data model:** The engine models this via `CalendarId` chains (see Part IV §7) and the `from_calendar_id` field. Each celebration carries the identity of the calendar that introduced it. The engine resolves the complete calendar by traversing the inheritance chain from the most specific calendar up to the General Calendar.

#### 2. Rank Assignment by Calendar Level (CP 8-12, 24-26)

The same saint can have **different ranks** depending on the calendar level. CP §8-12 defines the default rank for each type of proper celebration:

```
Type of celebration                  Calendar level        Default rank       CP ref
───────────────────                  ──────────────        ────────────       ──────
Principal patron of nation/region    National/Regional     Feast ¹            §8
Secondary patron of nation/region    National/Regional     Memorial           §8
Principal patron of diocese          Diocesan              Feast ¹            §9
Cathedral dedication anniversary     Diocesan              Feast              §9
Secondary patron of diocese          Diocesan              Memorial           §9
Principal patron of town/city        Local                 Solemnity          §10
Secondary patron of town/city        Local                 Memorial           §10
Church dedication anniversary ³       Church                Solemnity          §11
Church title                         Church                Solemnity          §11
Saint buried in church               Church                Memorial           §11
Religious title/founder/patron ²     Religious Institute   Solemnity/Feast    §12a
Beatified founder                    Religious Institute   Feast              §12a
Secondary patron of religious        Religious Institute   Memorial           §12a
Province title or principal patron   Religious Province    Feast              §12b
Province secondary patron            Religious Province    Memorial           §12b
Other saints (no special bond)       Any                   Obl./Opt. Memorial §24

¹ "For pastoral reasons this may be observed as a solemnity" (CP §8, §9)
² Only ONE of title/founder/patron may be a solemnity; others are feasts (§12)
³ "If it is consecrated" (CP §11) — unconsecrated churches have no dedication anniversary
```

**Rank elevation rule (CP §25):** "The observance of some celebrations in a particular place may have greater solemnity than in the entire diocese or religious institute." This means a more specific calendar in the inheritance chain can **override** the rank from a parent calendar.

**Co-cathedral distinction (Notitiae R10):** A co-cathedral's dedication anniversary is celebrated **in that church only**, not throughout the diocese. The cathedral church is unique as the sign of unity of the local church — only its anniversary is celebrated diocese-wide. Exception: when a diocese was formed from merged dioceses that retain a degree of autonomy (own curia and chapter), each may celebrate their own cathedral's anniversary.

**Titular feast permanence (Notitiae R11):** Every church retains its original title and celebrates its titular feast (Solemnity, CP §11), even if the saint has been removed from the General Calendar. Church-level calendars may therefore include titular feasts for saints absent from the General Calendar.

**Example:** St. Thomas Aquinas is an optional memorial in the General Calendar. In a Dominican calendar (§12), he is a solemnity (as founder). In the diocese of Aquino, he could be a feast (§9 principal patron).

**Consequence for the data model:** The `Rank` field in `Celebration` (Layer 1) and in `IdentityChoice`/`HoursCelebrationChoice` (Layers 2 Mass / 2 Hours) reflects the rank as resolved for the specific calendar in use. The engine inherits rank from the most specific calendar that defines it. The `from_calendar_id` field traces which calendar contributed the celebration and its rank.

#### 3. Precedence Conflicts: General vs. Particular (CP 23)

When a particular celebration falls on the same date as a General Calendar celebration, CP §23 defines the resolution:

| General Calendar    | Proper Calendar             | Resolution                                                                                                | CP ref |
| ------------------- | --------------------------- | --------------------------------------------------------------------------------------------------------- | ------ |
| Solemnity           | Any proper                  | General solemnity observed on its date                                                                    | §23a   |
| Feast               | Proper feast (same date)    | General feast kept; proper feast transferred to nearest free date                                         | §23b   |
| Feast               | Proper feast (deeply local) | Exception: proper feast may stay if transfer would cause "serious inconvenience"                          | §23b   |
| Optional memorial   | Proper memorial             | Proper memorial takes precedence                                                                          | §23c   |
| Obligatory memorial | Proper memorial             | Proper memorial **may** take precedence (by changing universal to optional, or by transferring universal) | §23c   |

**Consequence for the data model:** These precedence rules are applied during step 2 of the transformation pipeline (Part V §2): "Apply precedence rules (GNLY 59, 60)." CP §23 extends these rules for the particular calendar context. The engine must handle the case where a universal obligatory memorial is demoted to optional when a proper memorial claims the date (§23c).

#### 4. Proper of Seasons Primacy (CP 2)

CP §2 reinforces the GNLY principle that the temporal cycle always takes precedence:

- **§2a:** On Sundays, no particular celebration is permitted (per se).
- **§2b:** Lent, Easter Octave, and Dec 17-31 are to be kept free of particular celebrations — except optional memorials, certain feasts listed in Table of Liturgical Days §8 a-d, and non-transferable solemnities.
- **§2c:** Indult celebrations must not "duplicate celebrations already in the cycle of the mystery of salvation" and "must not be too numerous."

This reinforces the `MemorialRule::NoMemorial` and `MemorialRule::AdditionsOnly` mechanisms described in Part II and formalized in Part IV §5, and the GNLY 59-60 precedence rules in the pipeline.

#### 5. Proper Texts: Mass and Office Alignment (CP 40, 43-44)

CP specifies the proper texts expected for each celebration in both Mass and Office:

**Mass proper texts (CP §40):**

| Text                                     | Scope                                                            | CP ref  |
| ---------------------------------------- | ---------------------------------------------------------------- | ------- |
| Entrance antiphon                        | Directs thoughts to the celebration                              | CP §40a |
| Opening prayer (collect)                 | "Only [text with] direct bearing on the saint"                   | CP §40b |
| Prayer over the gifts                    | Bears on eucharistic mystery (saint mentioned incidentally)      | CP §40b |
| Preface                                  | Proper thanksgiving theme; literary form of praise, not petition | CP §40c |
| Communion antiphon                       | Expresses communion within the eucharistic mystery               | CP §40a |
| Prayer after Communion                   | Bears on eucharistic mystery                                     | CP §40b |
| Solemn blessing / prayer over the people | Optional                                                         | CP §40b |

**Office proper texts (CP §43-44):**

| Text                           | Scope                                                                                                | CP ref |
| ------------------------------ | ---------------------------------------------------------------------------------------------------- | ------ |
| Hagiographical reading         | Required for every solemnity, feast, and memorial — "usually not more than one hundred twenty words" | CP §43 |
| Responsory for the reading     | Proper or from a Common                                                                              | CP §43 |
| Biographical note              | Preliminary note; "not to be read as part of the office"                                             | CP §43 |
| Invitatory antiphon            | On solemnities and feasts                                                                            | CP §44 |
| Antiphons (esp. Lauds/Vespers) | Canticle antiphons                                                                                   | CP §44 |
| Intercessions                  | On solemnities and feasts                                                                            | CP §44 |
| Hymns                          | Existing proper hymns may be kept                                                                    | CP §44 |
| **Concluding prayer**          | **"Always the same as the opening prayer of the Mass"**                                              | CP §44 |

The last row is the cross-domain identity rule modeled by `Celebration.prayer` (see Part III §5 "Office Prayer = Mass Collect"): the text is stored once and resolved by both domains.

**Consequence for the data model:** The `FormularySet` structure (Mass) aligns with CP §40's enumeration. The `CelebrationHour` structure (Office) aligns with CP §44's enumeration. The `hagiographical_reading` field in `CelebrationOfficeReadings` should carry content for every celebration above weekday rank, per CP §43.

#### 6. Reading Constraints for Proper Masses (CP 41)

CP §41 imposes structural constraints on proper Mass readings:

- **Solemnities:** 3 readings required (OT + Epistle + Gospel)
- **Easter season:** No Old Testament reading (replaced by Acts or Revelation)
- **Proper readings:** Must always include a proper responsorial psalm and a proper acclamation or verse before the Gospel

These constraints complement GILM §83-84 and should be validated by the engine when assembling `ReadingsContent` for particular calendar celebrations.

### 7. The Paschal Triduum Is Not a Season

The `Season` enum has exactly five variants — there is no `PaschalTriduum` season. This architectural decision is based on the following normative evidence:

1. **GNLY structure:** Title II (§17-47) gives the Triduum its own section (I, §18-21), separate from the five seasons (sections II-VI). GNLY lists five seasons only: Advent, Christmas Time, Lent, Easter Time, Ordinary Time.
2. **GNLY §28:** "The forty days of Lent run from Ash Wednesday up to but excluding the Mass of the Lord's Supper." Lent ends before the Triduum begins.
3. **GNLY §22:** "The fifty days from the Sunday of the Resurrection to Pentecost Sunday..." Easter Time begins on Easter Sunday.
4. **GNLY §18:** The Triduum is described as "the high point of the entire liturgical year" — not as a season.
5. **PS §27:** "The Lenten season lasts until the Thursday of this week. The Easter Triduum begins with the evening Mass of the Lord's Supper..." — explicit separation.

The Triduum is a distinct liturgical unit that falls **between** Lent and Easter Time. It is tracked via `Period::PaschalTriduum` in `DayContext.periods`. During the Triduum, `DayContext.season` is:

| Civil date    | `season`           | Rationale                                                                                                       |
| ------------- | ------------------ | --------------------------------------------------------------------------------------------------------------- |
| Holy Thursday | `Some(Lent)`       | GNLY §28: Lent until the evening Mass. The civil date begins in Lent (the Chrism Mass is a Lenten celebration). |
| Good Friday   | `None`             | Between Lent and Easter Time. Not in any season.                                                                |
| Holy Saturday | `None`             | Between Lent and Easter Time. Not in any season.                                                                |
| Easter Sunday | `Some(EasterTime)` | GNLY §22: Easter Time begins. The Triduum also ends this day (at Vespers).                                      |

This design ensures `Season` has exactly 5 variants matching GNLY, while `DayContext.season: Option<Season>` naturally handles the 2-3 days per year where no season applies. The consumer can detect the Triduum via `periods.contains(PaschalTriduum)`.

The TLHM (Thesaurus Liturgiae Horarum Monasticae) confirms this classification: its Proprium de Tempore has 7 sections, with "Sacrum Triduum Paschale" (§4) as its own section, separate from the five seasonal sections.

### 7b. Transfer of Impeded Solemnities (GNLY 60)

GNLY §60: "If a Solemnity is impeded by a liturgical day that takes precedence over it, it is transferred to the closest day not listed in nos. 1–8 of the Table of Liturgical Days." This creates a special case in calendar generation: an impeded solemnity must be moved to a nearby date, respecting precedence rules.

**Key rules:**

- A solemnity is "impeded" when it falls on a day with higher or equal precedence (GNLY §59, Table of Liturgical Days nos. 1–8).
- The transfer target is the closest available day — typically the following Monday, but the engine must check that the target day itself can receive the solemnity.
- Transferred solemnities retain their full rank and all proper texts.
- Some solemnities are never transferred — they are simply omitted that year (e.g., an impeded optional solemnity in a particular calendar).

**Consequence for the data model:** The transfer is handled at calendar generation time (Part V, pipeline step 3). The resulting `LiturgicalDay` carries the solemnity on its new date with the original `CelebrationId`. No special field marks a celebration as "transferred" — the consumer sees the solemnity on its actual date. The implementation details (conflict resolution, target-finding algorithm) are documented in Part V ¹.

### 8. Title Model: TitleCategory + Qualifier + Patronage

A flat `Title` enum with many variants mixes three distinct concerns:

- **Ecclesiastical categories** (fixed, liturgically significant): `Martyr`, `Bishop`, `Virgin`...
- **Category + qualifiers** (specific): `TheFirstMartyr`, `ProtoMartyrOfOceania`, `SlavicMissionary`...
- **Patronages** (country-specific): `PatronOfFrance`, `CopatronessOfEurope`... (37 variants)

This forces modifications to the core enum every time a data file needs a new qualifier or patronage. The `is_martyr_title()` method must manually list all martyr-like variants (currently 3), making it fragile.

The new model separates these concerns into three layers:

```rust
// ── Layer 1: Fixed ecclesiastical categories ──
// Closed enum — only changes if the Church creates a new title category.
// These categories have liturgical impact (e.g., Martyr → red color).
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
    // Unique relational titles (liturgically significant, appear in the calendar as-is)
    ParentsOfTheBlessedVirginMary,
    SpouseOfTheBlessedVirginMary,
}

// ── Layer 2: Title = category + optional free-text qualifier ──
// The qualifier comes from the localized data files, not from the code.
struct Title {
    category: TitleCategory,
    /// Complete rendered text of the qualified title, replacing the base
    /// title from `title_categories` in display. Localized per locale.
    /// e.g. "the First Martyr", "Proto-martyr of Oceania", "Slavic Missionary"
    qualifier: Option<String>,
}

// ── Layer 3: Patronages (fully data-driven) ──
// CP §31: "from now on there is to be only one principal patron.
// Another may be added as a secondary patron."
// Gender (patron/patroness) is resolved at display time from
// MartyrologyEntry.sex — not a distinct role.
enum PatronRole {
    PrincipalPatron,
    SecondaryPatron,
    Copatron,
}

struct Patronage {
    role: PatronRole,
    of: String,  // localized, e.g. "France", "Europe", "the Diocese"
}
```

**Examples:**

| Title                           | Representation                                                            |
| ------------------------------- | ------------------------------------------------------------------------- |
| Martyr (no qualifier)           | `Title { category: Martyr, qualifier: None }`                             |
| the First Martyr                | `Title { category: Martyr, qualifier: Some("the First Martyr") }`         |
| Proto-martyr of Oceania         | `Title { category: Martyr, qualifier: Some("Proto-martyr of Oceania") }`  |
| Slavic Missionary               | `Title { category: Missionary, qualifier: Some("Slavic Missionary") }`    |
| Queen of Poland                 | `Title { category: Queen, qualifier: Some("Queen of Poland") }`           |
| Mother and Queen of Chile       | `Title { category: Queen, qualifier: Some("Mother and Queen of Chile") }` |
| Patron of France                | `Patronage { role: PrincipalPatron, of: "France" }`                       |
| Co-patroness of Europe          | `Patronage { role: Copatron, of: "Europe" }`                              |
| Principal Patron of the Diocese | `Patronage { role: PrincipalPatron, of: "the Diocese" }`                  |

When a qualifier is present, it is the **complete rendered text** of the qualified title (see the Input Data Model, Part III §6). It replaces the base title from `title_categories` in display.

**Key benefits:**

- **Martyr detection is trivial:** `title.category == TitleCategory::Martyr` — regardless of qualifier.
- **Zero core modifications** for new qualifiers or patronages — everything is in the data files.
- **`PatronRole`** has only 3 variants (CP §31: principal, secondary, co-patron; gender resolved from `MartyrologyEntry.sex`).
- **`Celebration` and `HoursCelebrationChoice`** carry both `titles: TitlesDef` and `patronages: Vec<Patronage>`. Patronages are defined at calendar level (country/diocese), not in martyrology resources.

---

## Part IV — Data Model

### 1. Type Shareability Overview: Mass → Office

| Type               | Reusable?     | Reason                                                                                                                                             |
| ------------------ | ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| `DayContext`       | **YES**       | Same temporal frame: season, cycles, psalter week                                                                                                  |
| `CelebrationId`    | **YES**       | Same celebration identity                                                                                                                          |
| `TextSource`       | **YES**       | Same provenance concept (Proper of Time, Proper of Saints, Common)                                                                                 |
| `SourcedText`      | **YES**       | Text + provenance — applies to any liturgical text                                                                                                 |
| `SourceRef`        | **YES**       | Biblical reference with confer flag — applies to antiphons, psalms, acclamations                                                                   |
| `ReadingText`      | **YES**       | Reading with headline, short form, `no_final_acclamation` — applies to Office readings too                                                         |
| `ShortForm`        | **YES**       | Short form reference + text — applies to both readings and psalms                                                                                  |
| `PsalmodyEntry`    | **YES**       | Psalm/canticle with antiphon — used for both Mass responsorial psalms and Office psalmody                                                          |
| `PsalmAntiphon`    | **YES**       | Responsorial antiphon with biblical source(s) — used in both Mass and Office psalmody                                                              |
| `FormularySet`     | **NO**        | Mass-specific: collect + Mass antiphons. Office has no equivalent inseparable block                                                                |
| `ReadingsSet`      | **NO**        | Mass Liturgy of the Word ≠ Office of Readings (different structure, different sources)                                                             |
| `ReadingsPool`     | **NO**        | Pool-per-component logic is Mass/GILM-specific                                                                                                     |
| `ReadingsContent`  | **NO**        | Enum of `ReadingsSet`/`ReadingsPool` — both Mass-specific                                                                                          |
| `FlexibleOrations` | **NO**        | Prayer over offerings, prayer after Communion — Mass-specific                                                                                      |
| `CompositionRules` | **Partially** | The approach (rules governing substitution) transfers, but the specific rule enums (`BlockRule`, `ReadingsRule`, `FlexibleRule`) are Mass-specific |

### 2. Shared Types

This section defines core types used in the data model. Some types (`FormularySet`, `ReadingsSet`, `ReadingsPool`, `ReadingsContent`, `FlexibleOrations`) are Mass-specific but are placed here alongside shared types for logical grouping — they are building blocks referenced by both Layer 1 and Layer 2 Mass types. Shared helper types (`SourceRef`, `ShortForm`, `PsalmodyEntry`, `PsalmAntiphon`, `AntiphonText`, `PrefaceText`) are used identically in both Mass and Office contexts (or in both input and output models).

> **JSON serialization convention:** All enums with data variants (e.g., `ReadingsContent`, `BlockRule`, `ReadingsRule`, `FlexibleRule`, `MemorialRule`, `HourSuppression`, `TextSource`) are serialized using **adjacently tagged** representation: `{"type": "Fixed", "data": {...}}`. This format is explicit, avoids field-name conflicts, and is straightforward to consume in any language (TypeScript, Python, etc.). Simple enums without data (e.g., `Season`, `Rank`, `Color`) serialize as plain strings. Newtypes (`CelebrationId`, `CalendarId`) serialize as plain strings.

#### `DayContext`

**What it is:** The shared temporal context for all celebrations on a given date.

**Why this name:** It provides the calendrical "context" of the "day" — season, cycles, position within the season — without any celebration-specific information.

**Contents:**

```rust
struct DayContext {
    /// Liturgical season (GNLY 17-44). None only on Good Friday and Holy
    /// Saturday, which fall between Lent and Easter Time — the Paschal Triduum
    /// is not a season but a distinct liturgical unit tracked via `periods`
    /// (see Part III §7). Holy Thursday remains Some(Lent) because GNLY §28
    /// places it within Lent until the evening Mass; Easter Sunday is
    /// Some(EasterTime) because GNLY §22 begins Easter Time on that day.
    season: Option<Season>,
    /// Localized season name for display
    season_name: Option<String>,
    /// Sunday readings cycle: Year A, B, or C (GILM 66)
    sunday_cycle: SundayCycle,
    /// Weekday readings cycle: Year 1 or 2 (GILM 69)
    weekday_cycle: WeekdayCycle,
    /// Psalter week: I-IV (GILH §133). Restarted at Week I on 1st Sunday
    /// of Advent, 1st Sunday of OT, 1st Sunday of Lent, Easter Sunday.
    psalter_week: PsalterWeekCycle,
    /// Week number within the season. Weeks are Sunday-based (GILH §133):
    /// the week number corresponds to the Sunday that opens it.
    ///
    /// Season-specific behavior:
    /// - **Advent:** 1-4 (GNLY §41). Week 1 begins on the 1st Sunday of Advent.
    /// - **Christmas Time:** None. The Lectionary organizes this season by
    ///   calendar dates (octave, feasts), not by week numbers (GNLY §32-38,
    ///   GILM §95-96).
    /// - **Lent:** 1-6 (GNLY §30). Week 1 = 1st Sunday of Lent, Week 6 = Palm
    ///   Sunday. **Convention:** the days from Ash Wednesday to the Saturday
    ///   before the 1st Sunday of Lent use `week_of_season: Some(0)`. This is
    ///   a practical convention (not normative) — the liturgical norms call
    ///   these "weekdays of Lent" without assigning a week number.
    /// - **Easter Time:** 1-7 (GNLY §23). Easter Sunday = Week 1.
    /// - **Ordinary Time:** 1-34 (GNLY §43-44, GILM §104). Week 1 begins on
    ///   the Monday after the Baptism of the Lord. Interrupted by Lent/Easter,
    ///   resumes after Pentecost.
    /// - **Triduum (season: None):** None.
    week_of_season: Option<u32>,
    /// Day number within the season (1-based)
    day_of_season: Option<u32>,
    /// Day of the week
    day_of_week: DayOfWeek,
    /// Sub-periods this day belongs to (e.g., HolyWeek + PaschalTriduum).
    /// A day can belong to multiple overlapping periods.
    /// See Part IV §7 for `Period` and `PeriodInfo` definitions.
    periods: Vec<PeriodInfo>,
    /// First date of the current season (ISO format)
    start_of_season: Option<String>,
    /// Last date of the current season (ISO format)
    end_of_season: Option<String>,
    /// First date of the liturgical year (1st Sunday of Advent, ISO format)
    start_of_liturgical_year: String,
    /// Last date of the liturgical year (Saturday before next Advent)
    end_of_liturgical_year: String,
}
```

#### `FormularySet`

**What it is:** The inseparable block of texts that identify a celebration in the Mass: collect + entrance antiphon + communion antiphon.

**Why this name:** In liturgical terminology, the "formulary" (_formularium_) is the complete set of proper texts for a given Mass. This struct represents the core identifying subset that must be taken as a unit. "Set" emphasizes that these elements are grouped and inseparable.

**Liturgical basis:** Architectural inference — GIRM 363 explicitly makes the prayer over the offerings and prayer after Communion flexible, but leaves these three elements (collect, entrance antiphon, communion antiphon) bound to the chosen formulary. See Group 1 discussion in Part I for caveats.

```rust
struct FormularySet {
    /// Collect override for this specific Mass time.
    /// When None, **will be resolved in Layer 2** to `Celebration.prayer` (CP §44).
    /// When Some(...), this Mass has a specific collect that differs from
    /// the canonical prayer (e.g., Christmas NightMass vs DayMass).
    collect: Option<String>,
    entrance_antiphon: Option<AntiphonText>,
    communion_antiphon: Option<AntiphonText>,
}

/// An antiphon with optional biblical source reference(s)
struct AntiphonText {
    /// Text of the antiphon
    text: String,
    /// Biblical source reference(s). Entrance and communion antiphons in the
    /// Roman Missal often derive from Scripture (e.g., "cf. Ps 24:1-3").
    sources: Option<Vec<SourceRef>>,
}
```

> **Resolution rule:** The effective collect for a Mass is: `formulary_set.collect` if present, otherwise `celebration.prayer`, otherwise the Common provides it. Most celebrations have a single collect stored in `Celebration.prayer`; the `FormularySet.collect` override is only needed when multiple Masses of the same celebration have distinct collects (e.g., Christmas: 4 different collects for Vigil, Night, Dawn, Day).

#### `ReadingText`

**What it is:** A liturgical reading text that may have an optional short form variant.

**Why this name:** It is a "reading" "text" with potential variant forms.

**Liturgical basis:** GIRM 360, GILM 75, 80 — some readings are provided in both long and short forms; "a pastoral criterion must be kept in mind" when choosing.

```rust
struct ReadingText {
    /// Full citation reference (e.g., "Isa 2:1-5") — the Tier 1 citation
    /// string, carried through the jointure for consumer display.
    reference: String,
    /// Pericope headline (from Tier 3 input)
    headline: Option<String>,
    /// The full text of the reading
    text: String,
    /// Abbreviated reference for display
    ref_abbr: Option<String>,
    /// Optional short form variant (GIRM 360, GILM 75, 80)
    short_form: Option<ShortForm>,
    /// No final acclamation flag (default false).
    /// True for Passion narratives and certain Easter Vigil readings.
    no_final_acclamation: bool,
}

/// Short form of a reading or psalm
struct ShortForm {
    /// Citation reference for the short form (e.g., "Gen 1:1.26-31a")
    reference: Option<String>,
    /// Abbreviated reference for display
    ref_abbr: Option<String>,
    /// The text of the short form
    text: String,
}
```

#### `ReadingsSet`

**What it is:** A complete, **indivisible** set of readings for the Liturgy of the Word. Used for weekday (_lectio continua_) and proper readings, where the Lectionary assigns specific texts as a pre-composed unit.

**Why this name:** "Readings" because it contains all the Scripture readings and their associated chants. "Set" because they form an indivisible block — they are taken together or not at all.

**Liturgical basis:** GIRM 357 — proper and weekday readings are taken as a complete set, not mixed individually. The psalm responds to the first reading it is paired with.

**When NOT to use:** For Common readings, where the celebrant chooses independently per component (GILM 71, 89), use `ReadingsPool` instead.

```rust
struct ReadingsSet {
    reading_1: Option<ReadingText>,
    psalm: Option<PsalmodyEntry>,
    canticle: Option<PsalmodyEntry>,
    reading_2: Option<ReadingText>,
    /// Sequence hymn (GIRM §64): sung before the Alleluia on specific
    /// celebrations only (Easter, Easter Octave, Pentecost, Corpus Christi,
    /// Our Lady of Sorrows). None on all other days.
    sequence: Option<SequenceText>,
    alleluia: Option<AlleluiaText>,
    gospel: Option<ReadingText>,
}

/// Gospel acclamation (Alleluia or Lenten acclamation).
/// AcclamationType is computed by the engine from the liturgical season.
struct AlleluiaText {
    /// Type of acclamation (computed by engine from season)
    acclamation_type: AcclamationType,
    /// The acclamation word/phrase itself (e.g., "Alleluia" or "Praise to you...")
    acclamation: Option<String>,
    /// The verse text
    verse: String,
    /// Biblical source reference(s) for the verse
    sources: Option<Vec<SourceRef>>,
}

/// Type of Gospel acclamation — computed by the engine from the season
enum AcclamationType {
    /// "Alleluia" — used throughout the year except Lent
    Alleluia,
    /// Lenten acclamation (e.g., "Praise to you, Lord Jesus Christ...")
    Lent,
    /// No acclamation (rare — certain rites like the Passion)
    None,
}

/// Sequence hymn (GIRM §64)
struct SequenceText {
    /// Reference (e.g., "Victimae paschali laudes")
    reference: Option<String>,
    /// The sequence text
    text: String,
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
    psalms: Vec<PsalmodyEntry>,
    /// Available second readings (when 3 readings are required) — pick one
    second_readings: Vec<ReadingText>,
    /// Available alleluia/acclamation verses — pick one (GILM 90)
    alleluia_verses: Vec<AlleluiaText>,
    /// Available gospels — pick one
    gospels: Vec<ReadingText>,
}
```

#### `ReadingsContent`

**What it is:** An enum distinguishing the three modes of readings provision: a fixed indivisible set, a pool of independently choosable components, or a variable-length vigil sequence.

**Why this name:** It represents the "content" of the "readings" block, which can take three structural forms depending on the source.

**Liturgical basis:** The distinction arises from the different treatment of proper/weekday readings (indivisible, GIRM 357), Common readings (component-level choice, GILM 71, 89), and Easter Vigil readings (ordered sequence with variable selection, PS 85).

```rust
enum ReadingsContent {
    /// Fixed, indivisible set (weekday lectio continua, proper readings)
    Fixed(ReadingsSet),
    /// Pool of components, each independently choosable (Commons, GILM 71)
    Pool(ReadingsPool),
    /// Ordered sequence with variable selection (Easter Vigil — PS 85)
    VigilSequence(VigilReadingsSequence),
}
```

#### `VigilReadingsSequence`

**What it is:** An ordered sequence of Old Testament readings with variable selection, followed by fixed New Testament readings. Used for the Easter Vigil, which has a unique structure: 7 OT readings (reducible to a minimum of 3), each with its own responsorial psalm or canticle, followed by Gloria, Epistle, Alleluia, and Gospel.

**Why this name:** "Vigil" because this structure is specific to the Easter Vigil (PS 85). "Readings" because it represents the Liturgy of the Word. "Sequence" because the OT readings are ordered and the celebrant selects a contiguous or designated subset, not individual picks from a pool.

**Liturgical basis:** PS 85 — "After the first reading (the account of creation) at least two others should be read from the Old Testament, and in any case the reading from Exodus 14 must never be omitted." When pastoral conditions require reducing, at least 3 OT readings must be proclaimed, and Exodus 14 (the crossing of the Red Sea) is always mandatory. The full set is 7 OT + Epistle (Romans 6) + Gospel.

**Why neither `Fixed` nor `Pool`:** `ReadingsSet` supports 2-3 readings (Reading 1 + optional Reading 2 + Gospel) — structurally insufficient for 9 readings. `ReadingsPool` models independent per-position choice from pools — semantically wrong, since the celebrant selects a _subset_ from an ordered sequence, not one from each pool.

```rust
struct VigilReadingsSequence {
    /// Ordered OT readings, each with its responsorial psalm/canticle.
    /// The celebrant selects at least `min_ot_readings` from this sequence.
    ot_readings: Vec<VigilReading>,
    /// Minimum number of OT readings to proclaim (PS 85: 3)
    min_ot_readings: u8,
    /// Epistle — always proclaimed after the OT sequence and Gloria
    epistle: ReadingText,
    /// Alleluia — solemnly restored after the Lenten suppression
    alleluia: AlleluiaText,
    /// Gospel — always proclaimed
    gospel: ReadingText,
}

struct VigilReading {
    /// The OT reading text
    reading: ReadingText,
    /// Responsorial psalm or canticle following this reading
    response: PsalmodyEntry,
    /// Whether this reading must always be included even when reducing
    /// (PS 85: true for Exodus 14 — "must never be omitted")
    mandatory: bool,
}
```

> **Scope:** Only the Easter Vigil uses `VigilSequence`. The Pentecost Vigil (PS §107) has an extended form with 4 OT readings, but without variable-minimum constraints — its two forms (extended and simple) are modeled as two separate `Fixed(ReadingsSet)` entries.

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
    preface: Option<PrefaceText>,         // Governed by GIRM 364-365, not 363
    /// Optional solemn blessing (GIRM 167). Present on solemnities, certain feasts,
    /// and special occasions. Alternative to the simple blessing.
    solemn_blessing: Option<String>,
    /// Optional prayer over the people (GIRM 167). May be used in place of
    /// the solemn blessing, especially during Lent.
    prayer_over_the_people: Option<String>,
}

/// Resolved preface text with optional provenance metadata.
/// In the output, the preface is always resolved to full text —
/// the engine resolves PrefaceRef::CatalogId from the input.
struct PrefaceText {
    /// Preface catalog ID (provenance — from which catalog entry this was resolved)
    id: Option<String>,
    /// Theme/title
    headline: Option<String>,
    /// The preface text
    text: String,
}
```

> **Note on solemn blessing and prayer over the people:** These two fields are included in `FlexibleOrations` for structural convenience. Unlike the other orations, they are not governed by GIRM 363 substitution rules — they are always optional and the consumer chooses freely among the provided alternatives (or omits them entirely in favor of the simple blessing).

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

**Why this name:** It is a "text" that is "sourced" — its provenance is tracked. This is essential for Layer 2 where flexible orations are presented as a list of alternatives, each with its origin.

```rust
struct SourcedText {
    source: TextSource,
    text: String,
}
```

#### `SourcedPreface`

**What it is:** A preface text paired with its provenance. Like `SourcedText`, but carries the enriched `PrefaceText` metadata (catalog ID, headline) instead of a plain string.

```rust
struct SourcedPreface {
    source: TextSource,
    preface: PrefaceText,
}
```

#### `SourceRef`

**What it is:** A biblical source reference with optional "cf." qualifier. Shared between input and output models.

**Why this name:** It is a "reference" to a "source" (biblical passage). No `*Def` suffix — identical in both input and output, like `LocaleTag` or `MassTime`.

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

#### `CelebrationId`

**What it is:** A unique identifier for a celebration.

**Why this name:** It identifies a `Celebration` uniquely within the calendar (e.g., `"ord_time_5_mon"`, `"st_scholastica"`).

```rust
struct CelebrationId(String);
```

> **Newtype pattern:** `CelebrationId` is a newtype wrapping `String` rather than a type alias. This ensures type safety — prevents accidentally passing a `CalendarId` or a plain `String` where a `CelebrationId` is expected. In JSON, it serializes as a plain string.

### 3. Layer 1 — Liturgical Calendar

**Method:** `Calendar::generate_liturgical_calendar() → LiturgicalCalendar`

**Principle:** Organized by liturgical day. Each civil date maps to one `LiturgicalDay` containing all possible celebrations. Masses are not shifted — evening masses (vigils, PreviousEveningMass) remain attached to their liturgical day. This layer serves as the internal foundation from which Layer 2 is generated.

#### `LiturgicalCalendar`

**What it is:** The top-level output type. A map from civil date to liturgical day.

**Why this name:** It is a "calendar" organized by "liturgical" days — the liturgical perspective on the year.

**Why keyed by civil date:** By convention and for practical convenience, each liturgical day is associated with the civil date where the majority of the day occurs. The `MassTime` enum carries the information about whether a mass is celebrated the evening before (e.g., `PreviousEveningMass`, `EasterVigil`).

```rust
struct LiturgicalCalendar(BTreeMap<String, LiturgicalDay>);
```

> **Newtype pattern:** `LiturgicalCalendar` wraps the `BTreeMap` to provide a named type with domain-specific methods, rather than exposing the raw map. In JSON, it serializes as a plain object keyed by date strings.

#### `LiturgicalDay`

**What it is:** One liturgical day — the temporal frame for a given civil date, containing the shared context and all possible celebrations.

**Why this name:** It is the "liturgical day" as defined by GNLY 3: "The liturgical day runs from midnight to midnight." It is the container, not the content. A single `LiturgicalDay` can host multiple `Celebration` objects (a feria + optional memorials).

**Why not `Celebration`:** A liturgical day is not a celebration — it is the temporal frame within which celebrations occur. The GNLY distinguishes the two concepts (cf. GNLY 3, 10).

```rust
struct LiturgicalDay {
    /// Civil date of attachment (YYYY-MM-DD)
    date: String,

    /// Shared temporal context for all celebrations on this day
    context: DayContext,

    /// Possible celebrations, ordered by precedence
    /// [0] = primary (feria or highest-ranking celebration)
    /// [1..] = alternatives (optional memorials, etc.)
    /// Note: GNLY 14 — only one optional memorial may be celebrated per day.
    celebrations: Vec<Celebration>,

    /// Diff-based override history from calendar inheritance.
    /// Ordered from most general (e.g., `general_roman`) to most specific
    /// (e.g., `diocese_paris`). Only overrides with actual changes are included.
    parent_overrides: Vec<ParentOverride>,
}
```

#### `Celebration`

**What it is:** One liturgical celebration — an entity with a rank, a name, liturgical colors, and mass texts. The feria of Wednesday of the 5th week is a celebration. The optional memorial of St. Scholastica is another celebration.

**Why this name:** GNLY 10 defines it: "Celebrations, according to the importance assigned to them, are hence distinguished one from another and termed: Solemnity, Feast, Memorial." A celebration is the liturgical entity that is celebrated, with its specific rank and proper texts.

**Why not `LiturgicalDay`:** A single `LiturgicalDay` type mixing the temporal frame and the celebration identity would conflate two distinct GNLY concepts. This separation clarifies that multiple celebrations can coexist within one liturgical day.

```rust
struct Celebration {
    /// Unique identifier (e.g., "ord_time_5_wed", "st_scholastica")
    id: CelebrationId,
    /// Localized full name
    name: String,
    /// Liturgical precedence (GNLY table, levels 1-13)
    precedence: Precedence,
    /// Liturgical rank (Solemnity, Sunday, Feast, Memorial, OptionalMemorial, Weekday)
    rank: Rank,
    /// Localized rank name
    rank_name: String,
    /// Permissible liturgical colors (GIRM 346). Multiple when alternatives
    /// exist (e.g., gold/white on solemnities, black/purple for funerals).
    /// Red is automatically assigned when MartyrologyEntry.titles contains Martyr.
    /// See Part IV §7 for `Color` and `ColorInfo` definitions.
    colors: Vec<ColorInfo>,
    /// Applicable Commons from the Roman Missal (see Part IV §7 for `Common`
    /// and `CommonInfo` definitions). Empty when the celebration has all proper
    /// texts. Multiple when the saint belongs to several categories (e.g.,
    /// Virgin + Martyr). Each Common provides a pool of fallback texts (prayers,
    /// readings, antiphons) for elements that have no proper text.
    commons: Vec<CommonInfo>,
    /// Persons, entities, or groups celebrated on this day.
    /// Carries biographical metadata (titles, sex, count) used by the engine
    /// to resolve Commons and assign colors. See Part IV §7 for key fields.
    martyrology: Vec<MartyrologyEntry>,
    /// Titles as published in the liturgical books (Martyr, Virgin, Bishop, etc.).
    /// TitleCategory + optional qualifier. See Part III §8 for the design decision
    /// and Part IV §7 for `TitlesDef` and `TitleCategory` definitions.
    titles: TitlesDef,
    /// Patronages (Patron/Copatron of a country, diocese, etc.).
    /// Fully data-driven, defined at calendar level. See Part IV §7.
    patronages: Vec<Patronage>,
    /// Holy day of obligation
    is_holy_day_of_obligation: bool,
    /// Optional celebration (can be omitted in favor of the feria)
    is_optional: bool,
    /// Calendar that last defined or modified this celebration.
    /// If a General Calendar celebration is overridden by a local calendar,
    /// this is the local calendar's ID. Full modification history available
    /// in LiturgicalDay.parent_overrides.
    from_calendar_id: CalendarId,

    /// Canonical prayer (CP §44) — serves as Mass collect and Office concluding
    /// prayer unless overridden by `FormularySet.collect` or
    /// `CelebrationHour.concluding_prayer` respectively.
    /// This is the single text that serves as:
    /// - the Mass collect (FormularySet.collect)
    /// - the Office concluding prayer (CelebrationHour.concluding_prayer)
    /// When present, both domains resolve to this text unless they provide
    /// their own override. See "Office Prayer = Mass Collect" (Part III §5).
    ///
    /// None = no proper prayer (e.g., the celebration relies on Common texts
    /// or on the seasonal weekday prayer).
    prayer: Option<String>,

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

**Why not `MassContent` (existing name):** The existing `MassContent` is a flat `BTreeMap<MassPart, String>` with no grouping semantics. `CelebrationMass` structures the content by GIRM substitution groups (formulary, readings, flexible orations), .

```rust
struct CelebrationMass {
    /// Formulary block — follows the choice of celebration
    formulary: FormularySet,
    /// Readings block — Fixed (proper/weekday), Pool (Common), or VigilSequence (Easter Vigil)
    readings: ReadingsContent,
    /// Flexible orations (GIRM 363) and preface (GIRM 364-365)
    flexible_orations: FlexibleOrations,

    /// Whether this celebration is a eucharistic celebration (true for all Masses,
    /// false for the Celebration of the Lord's Passion on Good Friday).
    /// PS §59: "the Church does not celebrate the Eucharist" on Good Friday.
    is_eucharistic: bool,

    /// Gospel reading proclaimed during a pre-Mass entrance rite.
    /// Present only on Palm Sunday (MassTime::MassOfThePassion): the Gospel of the
    /// Lord's Entry into Jerusalem (Mt 21 / Mc 11 / Lc 19 by liturgical year).
    /// The procession chants (Ps 23, 46, antiphons) replace the normal entrance rite
    /// and are carried by FormularySet.entrance_antiphon.
    /// Three entrance forms exist (solemn procession, solemn entrance, simple entrance
    /// — PS §29-30); the choice between them is pastoral, not modeled.
    entrance_gospel: Option<ReadingText>,
}
```

#### `MassTime` (existing type)

**What it is:** An enum identifying the type of mass or liturgical action within a given celebration. Most celebrations have a single `DayMass`; multi-Mass days (Christmas, Triduum) have multiple variants.

**Why this name:** It identifies the "time" or "occasion" of the "mass" within the liturgical day.

**All variants:**

```rust
enum MassTime {
    /// The most important Mass of the liturgical year, celebrated on Holy Saturday night.
    /// Liturgically belongs to Easter Sunday — shifted to Saturday evening civil date
    /// in Layer 2 Mass. Unique readings structure: 7 OT + 2 NT (PS 85).
    EasterVigil,
    /// Mass celebrated the evening before a major feast (GNLY 11).
    /// Shifted to previous civil date in Layer 2 Mass.
    PreviousEveningMass,
    /// Mass celebrated during the night (Christmas Midnight Mass).
    NightMass,
    /// Mass celebrated at dawn (Christmas Dawn Mass, Easter morning).
    MassAtDawn,
    /// Used when the same civil date also hosts a `PreviousEveningMass`
    /// belonging to a different liturgical day. Signals to the consumer that
    /// this is the morning celebration. In practice: December 24 morning
    /// (Advent feria) alongside Christmas `PreviousEveningMass` that same evening.
    MorningMass,
    /// Regular daytime Mass — the default when only one Mass time exists
    /// for the celebration, or when no disambiguation with an evening Mass is needed.
    DayMass,
    /// Palm Sunday Mass beginning with the commemoration of the Lord's Entry
    /// into Jerusalem (PS §28-32). The procession/entrance rite has its own
    /// Gospel reading (entrance_gospel field). The Mass Gospel is the Passion
    /// narrative, proclaimed in the traditional three-person format (PS §33).
    MassOfThePassion,
    /// Celebration of the Lord's Passion on Good Friday (PS §59).
    /// NOT a eucharistic celebration (is_eucharistic: false) — no consecration.
    /// Includes readings (Isaiah, Hebrews, John's Passion), Great Intercessions,
    /// Adoration of the Cross, and Communion from the reserved Sacrament.
    CelebrationOfThePassion,
    /// Chrism Mass — celebrated by the bishop with his presbyterium (PS §35-36).
    /// Holy oils are consecrated and blessed. Traditionally on Holy Thursday morning,
    /// but may be transferred to another day close to Easter. Primarily a diocesan
    /// celebration — assigned in particular calendars, not in the General Calendar.
    ChrismMass,
    /// Evening Mass of the Lord's Supper on Holy Thursday (PS §44-48).
    /// The evening Mass that begins the Paschal Triduum. Followed by the
    /// transfer of the Blessed Sacrament and the stripping of the altar.
    EveningMassOfTheLordsSupper,
}
```

> **Architectural note:** `MassTime` includes `CelebrationOfThePassion`, which is not a eucharistic celebration (PS §59). This is a pragmatic design choice: the Celebration of the Lord's Passion has readings, prayers, and a structured liturgy of the Word — the same data structure as a Mass. Placing it within `MassTime` ensures consumers find "what happens in church" for every date in a single calendar, rather than needing a separate output type for one day per year. The `is_eucharistic: bool` field on `CelebrationMass` / `MassComposition` distinguishes it explicitly.

#### Layer 1 — Example

> **Notation convention:** Examples use simplified notation: `Some()` wrappers and `None` values are omitted for readability. Fields shown with a value are `Some(value)`; absent optional fields are `None`.

```
LiturgicalCalendar
│
├── "2025-02-10" → LiturgicalDay
│   ├── date: "2025-02-10"
│   ├── context: DayContext { season: OrdinaryTime, week_of_season: 5, ... }
│   └── celebrations:
│       ├── [0] Celebration
│       │   ├── id: "ord_time_5_mon"
│       │   ├── name: "Monday, 5th Week of Ordinary Time"
│       │   ├── rank: Weekday
│       │   ├── precedence: Weekday_13
│       │   ├── is_optional: false
│       │   ├── prayer: "Deus, qui..."            ← weekday collect (CP §44)
│       │   └── masses:
│       │       └── DayMass → CelebrationMass
│       │           ├── is_eucharistic: true
│       │           ├── entrance_gospel: None
│       │           ├── formulary: FormularySet { collect: None, ant_entr, ant_comm }
│       │           │                              ↑ resolved in Layer 2 to Celebration.prayer
│       │           ├── readings: ReadingsSet { reading_1, psalm, gospel, ... }
│       │           └── flexible_orations: FlexibleOrations { ... }
│       │
│       ├── [1] Celebration
│       │   ├── id: "st_scholastica"
│       │   ├── name: "Saint Scholastica"
│       │   ├── rank: OptionalMemorial
│       │   ├── precedence: OptionalMemorial_12
│       │   ├── is_optional: true
│       │   ├── prayer: "Deus, qui animam..."     ← saint's collect = Office prayer (CP §44)
│       │   └── masses:
│       │       └── DayMass → CelebrationMass { ... }
│       │
│       └── [2] Celebration
│           ├── id: "bl_luigi_stepinac"
│           ├── name: "Blessed Luigi Stepinac"
│           ├── rank: OptionalMemorial
│           ├── precedence: OptionalMemorial_12
│           └── ...
│
├── "2025-12-25" → LiturgicalDay
│   ├── context: DayContext { season: ChristmasTime, ... }
│   └── celebrations:
│       └── [0] Celebration
│           ├── id: "christmas"
│           ├── rank: Solemnity
│           ├── precedence: Solemnity_2
│           ├── prayer: None                     ← multi-Mass: each has its own collect
│           └── masses:                          ← no shift
│               ├── PreviousEveningMass → CelebrationMass { formulary: { collect: Some("..."), ... } }
│               ├── NightMass → CelebrationMass { formulary: { collect: Some("..."), ... } }
│               ├── MassAtDawn → CelebrationMass { formulary: { collect: Some("..."), ... } }
│               └── DayMass → CelebrationMass { formulary: { collect: Some("..."), ... } }
│
├── "2025-04-13" → LiturgicalDay                           ← Palm Sunday
│   ├── context: DayContext { season: Lent, ... }
│   └── celebrations:
│       └── [0] Celebration
│           ├── id: "palm_sunday"
│           ├── rank: Sunday
│           ├── precedence: Sunday_2
│           └── masses:
│               └── MassOfThePassion → CelebrationMass {
│                       is_eucharistic: true,
│                       entrance_gospel: Some(ReadingText { ... }),  ← Gospel of the Entry
│                       readings: Fixed(ReadingsSet {               ← Passion = Mass Gospel
│                           reading_1: "Is 50:4-7", psalm: "Ps 22",
│                           reading_2: "Phil 2:6-11", gospel: "Mt 26-27 (Year A)", ...
│                       }),
│                       ...
│                   }
│
├── "2025-04-17" → LiturgicalDay                           ← Holy Thursday
│   ├── context: DayContext { season: Some(Lent),          ← still Lent (GNLY §28)
│   │       periods: [HolyWeek, PaschalTriduum], ... }
│   └── celebrations:
│       └── [0] Celebration
│           ├── id: "holy_thursday"
│           └── masses:
│               └── EveningMassOfTheLordsSupper → CelebrationMass {
│                       is_eucharistic: true, entrance_gospel: None, ... }
│
├── "2025-04-18" → LiturgicalDay                           ← Good Friday
│   ├── context: DayContext { season: None,                ← between seasons
│   │       periods: [HolyWeek, PaschalTriduum], ... }
│   └── celebrations:
│       └── [0] Celebration
│           ├── id: "good_friday"
│           └── masses:
│               └── CelebrationOfThePassion → CelebrationMass {
│                       is_eucharistic: false,              ← NOT a Mass (PS §59)
│                       entrance_gospel: None,
│                       readings: Fixed(ReadingsSet { ... }),  ← Is 52-53, Heb 4-5, Jn 18-19
│                       ... }
│
├── "2025-04-19" → LiturgicalDay                           ← Holy Saturday
│   ├── context: DayContext { season: None,                ← between seasons
│   │       periods: [HolyWeek, PaschalTriduum], ... }
│   └── celebrations:
│       └── [0] Celebration
│           ├── id: "holy_saturday"
│           └── masses: {}                                  ← empty — aliturgical (PS §75)
│
├── "2025-04-20" → LiturgicalDay                           ← Easter Sunday
│   ├── context: DayContext { season: Some(EasterTime),    ← Easter Time begins
│   │       periods: [PaschalTriduum, EasterOctave], ... }
│   └── celebrations:
│       └── [0] Celebration
│           ├── id: "easter_sunday"
│           └── masses:
│               ├── EasterVigil → CelebrationMass {         ← shifted to Apr 19 in L2M
│                       is_eucharistic: true,
│                       readings: VigilSequence(VigilReadingsSequence {
│                           ot_readings: [7 readings...],   ← min 3, Ex 14 mandatory
│                           min_ot_readings: 3,
│                           epistle: "Rom 6:3-11", gospel: "Mt 28 (Year A)", ...
│                       }),
│                       ... }
│               └── DayMass → CelebrationMass { is_eucharistic: true, ... }
```

#### Layer 1 Extension — `CelebrationHour`

**What it is:** The raw textual content that a celebration provides for one Hour of the Office. Parallels `CelebrationMass` but with Office-specific structure.

**Why raw (not resolved):** In Layer 1, each `Celebration` carries its own texts — what it _provides_, not what the celebrant _uses_. On a memorial, the saint's `CelebrationHour` contains only the proper elements the saint provides; the weekday base content is in the feria's `CelebrationHour`. The resolution (merging weekday + saint per GILH §235 rules) happens in Layer 2 Hours.

**Why per-Hour:** The Proper of Saints distributes elements across Hours: an antiphon at the Benedictus (Lauds), an antiphon at the Magnificat (Vespers), a hagiographical reading (Office of Readings), etc. Storing them per-Hour mirrors how the celebrant actually uses them. On memorials, Daytime Prayer and Night Prayer have no entries from the saint (GILH §236).

```rust
/// Raw content that a celebration provides for one Hour.
/// On memorials: only the saint's proper elements are populated.
/// On solemnities/feasts: fully populated.
/// The feria celebration: fully populated with weekday content.
struct CelebrationHour {
    /// Proper psalmody — rare on memorials (GILH §235a: usually from weekday)
    /// On solemnities: proper psalms from Laudate group or Sunday Week I (GILH §225)
    psalmody: Option<HoursPsalmody>,

    /// Elements from GILH §235b (None = use weekday or Common instead)
    invitatory_antiphon: Option<String>,
    hymn: Option<String>,
    short_reading: Option<String>,
    short_responsory: Option<String>,
    /// Gospel canticle antiphon — at Benedictus (Lauds), Magnificat (Vespers),
    /// or Nunc Dimittis (Compline). Only one of these three canticles appears
    /// per Hour, so a single field suffices. (GILH 116-119)
    canticle_antiphon: Option<String>,
    intercessions: Option<String>,

    /// Concluding prayer override for this specific Hour.
    /// When None, resolves to Celebration.prayer (CP §44 identity).
    /// When Some(...), this Hour has a specific prayer that differs from
    /// the canonical prayer (rare — e.g., a solemnity with distinct per-Hour prayers).
    /// Exception: Compline always uses the psalter prayer (GILH §198),
    /// never Celebration.prayer.
    /// On memorials: mandatory from saint (GILH §235c) — the engine ensures
    /// Celebration.prayer is populated for any celebrated memorial.
    concluding_prayer: Option<String>,

    /// Office of Readings content — only populated for HourTime::OfficeOfReadings.
    /// On memorials: only hagiographical_reading populated (GILH §235d).
    /// On solemnities/feasts: both readings fully populated from Proper/Common (GILH §228).
    /// On the feria: scripture from weekday cycle, patristic from current cycle.
    office_of_readings_content: Option<CelebrationOfficeReadings>,
}

/// Raw Office of Readings content that a celebration provides.
/// This is the Layer 1 (unresolved) counterpart of OfficeReadingsContent (Layer 2 Hours).
struct CelebrationOfficeReadings {
    /// 1st reading: Scripture reading (from Proper on solemnities, from weekday cycle on feria)
    /// None on memorials — the weekday cycle reading applies (GILH §235d)
    scripture_reading: Option<ReadingText>,
    scripture_responsory: Option<String>,
    /// 2nd reading: patristic reading (from Proper on solemnities, from current cycle on feria)
    /// None on memorials when replaced by hagiographical reading (GILH §235d)
    patristic_reading: Option<ReadingText>,
    patristic_responsory: Option<String>,
    /// Hagiographical reading — in honor of the saint
    /// On memorials: replaces patristic (GILH §235d). On GILH §239 additions: added after patristic.
    /// On solemnities: may serve as the second reading (the reading "in honor of the saint").
    hagiographical_reading: Option<ReadingText>,
    hagiographical_responsory: Option<String>,
    /// Biographical note (CP §43, GILH §168): preliminary sketch preceding the
    /// hagiographical reading. "Not to be read as part of the office" — informational
    /// metadata for presentation layers.
    biographical_note: Option<String>,
}
```

> **Design note:** The previous version of `CelebrationHour` had `hagiographical_reading` and `hagiographical_responsory` as top-level fields. These are now nested in `CelebrationOfficeReadings` alongside the scripture and patristic readings, which are necessary for solemnities and feasts where BOTH readings come from the celebration's Proper (GILH §228). On memorials, only `hagiographical_reading` is populated; on the feria, `scripture_reading` + `patristic_reading` are populated. The engine in Layer 2 Hours merges these correctly based on rank and season.

**Integration in `Celebration`:**

```rust
struct Celebration {
    // ... identity fields (name, rank, colors, commons...) ...
    prayer: Option<String>,                          // ← canonical prayer (CP §44)
    masses: BTreeMap<MassTime, CelebrationMass>,
    hours: BTreeMap<HourTime, CelebrationHour>,
}
```

The same `Celebration` entity — St. Scholastica, Memorial — carries both Mass and Office texts. The identity fields are shared; `prayer` is the cross-domain canonical prayer (CP §44) that serves as both Mass collect and Office concluding prayer; only the domain-specific content differs by mode.

#### `HourTime`

**What it is:** An enum identifying a specific Hour of the Daily Office, including the shifted Evening Prayer I for solemnities.

**Why `VespersI`:** GILH 225 states that solemnities begin with Evening Prayer I on the day before. This parallels `PreviousEveningMass` in the Mass model. In Layer 1 (liturgical perspective), Vespers I stays on the solemnity's `Celebration`. In Layer 2 Hours (civil date perspective), it shifts to the previous civil date.

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

> **Note on the Invitatory (GILH §34-36):** The Invitatory is not a separate `HourTime` because it is not an independent Hour — it introduces the first Hour of the day (normally Office of Readings or Lauds). Its antiphon is stored in the `invitatory_antiphon` field of `CelebrationHour` (Layer 1) and `ResolvedHourContent` (Layer 2 Hours), and is associated with whichever Hour comes first in the day's sequence. The Invitatory psalm (Ps 95 or its alternatives) is structural, not content that varies per celebration.

#### `HoursPsalmody`

**What it is:** The psalmody for one Hour — a sequence of psalm/canticle entries with their antiphons.

**Liturgical basis:** Each Hour has a specific psalmody structure — typically 3 psalms at Office of Readings, 2 psalms + 1 OT canticle at Lauds, 2 psalms + 1 NT canticle at Vespers, 3 psalms at Daytime Prayer, etc.

```rust
struct HoursPsalmody {
    /// Psalm/canticle entries in order
    entries: Vec<PsalmodyEntry>,
}

/// One psalmody entry (psalm/canticle + resolved antiphon).
/// Used for both Mass responsorial psalms (in ReadingsSet/ReadingsPool) and
/// Office psalmody (in HoursPsalmody).
struct PsalmodyEntry {
    /// Psalm or canticle reference (e.g., "Ps 63", "Dan 3:57-88")
    reference: String,
    /// Abbreviated reference for display (e.g., "103, 1-2a…")
    ref_abbr: Option<String>,
    /// The text of the psalm/canticle. None when Tier 3 text data is absent
    /// (the citation is still carried for display or external lookup).
    text: Option<String>,
    /// Resolved responsorial antiphon — single refrain selected by the engine
    /// from the input's `antiphons` Vec based on Tier 1 cycle context.
    antiphon: Option<PsalmAntiphon>,
    /// Optional short form (abbreviated psalm for pastoral use)
    short_form: Option<ShortForm>,
}

/// Resolved responsorial antiphon with optional biblical source(s)
struct PsalmAntiphon {
    /// Text of the antiphon/refrain
    text: String,
    /// Biblical source reference(s) for the antiphon text
    sources: Option<Vec<SourceRef>>,
}
```

### 4. Layer 2 Mass — Mass Calendar

**Method:** `Calendar::generate_mass_calendar() → MassCalendar`

**Principle:** Organized by civil date and mass time. Each mass is a self-contained unit with all options pre-resolved by the engine. Evening masses are shifted to the previous civil day. The consumer picks from the options according to the explicit composition rules.

**Generated from Layer 1:** The engine first produces the `LiturgicalCalendar`, then transforms it into the `MassCalendar` by: shifting evening masses to the previous civil date, assembling identity and readings options from available celebrations, resolving flexible orations with their sources, and computing the applicable composition rules based on season and precedence.

#### `MassCalendar`

**What it is:** The top-level output type. A map from civil date to a list of masses celebrated that day.

**Why this name:** It is a "calendar" organized by "masses" — the practical perspective of what is actually celebrated on each civil day.

```rust
struct MassCalendar(BTreeMap<String, Vec<MassComposition>>);
```

> **Newtype pattern:** `MassCalendar` wraps the `BTreeMap` to provide a named type with domain-specific methods (e.g., `masses_for_date`), consistent with `LiturgicalCalendar`. In JSON, it serializes as a plain object keyed by date strings.

#### `MassComposition`

**What it is:** A single mass with all its options pre-resolved, structured by GIRM substitution groups. The consumer receives everything needed to compose the mass without knowing the GIRM rules themselves.

**Why this name:** "Mass" because it represents one mass celebration. "Composition" because the mass is "composed" from options across different blocks — the consumer composes the final mass by picking from the provided options according to the rules.

**Why not `MassContext` (existing name):** The existing `MassContext` is a flat structure that merely references optional celebrations by summary. `MassComposition` provides the actual texts organized by substitution groups, with explicit composition rules.

```rust
struct MassComposition {
    // === Identification ===
    /// Type of mass (DayMass, NightMass, EasterVigil, CelebrationOfThePassion...)
    mass_time: MassTime,
    /// Civil date — after shifting for evening masses
    civil_date: String,
    /// Liturgical date — before shifting (the "theological" date)
    liturgical_date: String,
    /// Whether this is a eucharistic celebration (false only for
    /// CelebrationOfThePassion on Good Friday — PS §59)
    is_eucharistic: bool,

    // === CONTEXT ===
    /// Shared day context
    context: DayContext,

    // === ORDINARIUM MISSAE FLAGS ===
    /// Gloria (GIRM §53): said on Sundays outside Advent/Lent,
    /// on solemnities and feasts.
    /// Computed by the engine from rank + season.
    /// Exception: false on All Souls despite Feast rank (Mass for the Dead).
    gloria: bool,
    /// Creed (GIRM §68): said on Sundays and solemnities.
    /// Computed by the engine from rank + season.
    /// Exception: false on All Souls despite level 3 precedence.
    creed: bool,

    // === DEFAULT CELEBRATION ===
    /// The celebration to use by default (typically the feria or highest-ranking)
    default_celebration_id: CelebrationId,

    // Note: GIRM 355 regime (which substitution rules apply) is not a separate
    // field — it is fully deducible from `context.season` + `context.periods` +
    // the `rank` of the default celebration, and is encoded in `composition_rules`.

    // === PRE-MASS ENTRANCE RITE ===
    /// Gospel reading proclaimed during a pre-Mass entrance rite.
    /// Present only on Palm Sunday (MassOfThePassion): the Gospel of the
    /// Lord's Entry into Jerusalem (PS §29-32). See CelebrationMass for details.
    entrance_gospel: Option<ReadingText>,

    // === FORMULARY BLOCK ===
    /// Each option = one possible celebration with its collect + antiphons.
    /// The consumer picks ONE option — all three texts come as a block.
    /// GNLY 14: "If several Optional Memorials are inscribed in the Calendar
    /// on the same day, only one may be celebrated, the others being omitted."
    /// Parallel: HoursComposition.celebration_choices serves the same role
    /// in the Office — one choice per celebration identity.
    identity_choices: Vec<IdentityChoice>,

    // === READINGS BLOCK (GIRM 357, GILM 71/83/89) ===
    /// Each option = either a fixed set (weekday/proper) or a pool (Common)
    /// The consumer picks ONE option, then composes from it
    readings_choices: Vec<ReadingsChoice>,

    // === FLEXIBLE ORATIONS (GIRM 363) AND PREFACE (GIRM 364-365) ===
    /// Each oration has its own list of alternatives, chosen independently
    prayer_over_offerings_options: Vec<SourcedText>,
    prayer_after_communion_options: Vec<SourcedText>,
    preface_options: Vec<SourcedPreface>, // Governed by GIRM 364-365
    solemn_blessing_options: Vec<SourcedText>,
    prayer_over_people_options: Vec<SourcedText>,

    // === COMPOSITION RULES ===
    /// Constraints determined by the engine based on season/precedence
    composition_rules: CompositionRules,
}
```

#### `IdentityChoice`

**What it is:** One possible celebration that can be chosen for the formulary block. Contains the celebration's metadata and its inseparable text trio (collect + antiphons).

**Why this name:** "Identity" because the formulary block is what _identifies_ which celebration is being performed — the collect is the defining prayer. "Choice" because it is one choice among several.

**Why it wraps `FormularySet`:** The `FormularySet` (shared type) provides the three inseparable texts. `IdentityChoice` adds the celebration metadata (name, rank, colors...) that the consumer needs for display and logic.

```rust
struct IdentityChoice {
    /// Reference to the celebration
    celebration_id: CelebrationId,
    celebration_name: String,
    rank: Rank,
    precedence: Precedence,
    colors: Vec<ColorInfo>,
    commons: Vec<CommonInfo>,
    martyrology: Vec<MartyrologyEntry>,
    titles: TitlesDef,
    patronages: Vec<Patronage>,
    is_holy_day_of_obligation: bool,
    from_calendar_id: CalendarId,

    /// The three inseparable texts (shared type)
    formulary: FormularySet,
}
```

#### `ReadingsChoice`

**What it is:** One possible readings provision for the Liturgy of the Word. May be a fixed set (weekday, proper) or a pool of independently choosable components (Common).

**Why this name:** "Readings" because it concerns the Scripture readings. "Choice" because it is one choice among several (weekday readings, saint's proper readings, or Common readings).

**Why it wraps `ReadingsContent`:** The `ReadingsContent` enum (shared type) distinguishes the two structural modes. `ReadingsChoice` adds source provenance, the GILM 83 category, and flags.

```rust
struct ReadingsChoice {
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
    /// Weekday readings from the lectio continua (Proper of Time).
    /// Note: Weekday is not a GILM 83 category — it is an architectural
    /// addition representing the default feria readings.
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

**Why this name:** "Composition" because these are the rules for _composing_ the mass from the available options. "Rules" because they are normative constraints from the GIRM, not suggestions.

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

**What it is:** A rule governing a block where one option must be picked entirely.

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

#### Layer 2 Mass — Example

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
│       identity_choices: [
│           IdentityChoice {                          ← feria
│               celebration_id: "ord_time_5_mon",
│               rank: Weekday,
│               formulary: FormularySet { collect: "...", ... }
│           },
│           IdentityChoice {                          ← optional memorial
│               celebration_id: "st_scholastica",
│               rank: OptionalMemorial,
│               formulary: FormularySet { collect: "...", ... }
│           },
│       ],
│
│       readings_choices: [
│           ReadingsChoice {                          ← weekday (default)
│               source: ProperOfTime { day_id: "ord_time_5_mon" },
│               category: Weekday,
│               is_default: true,
│               readings: Fixed(ReadingsSet {         ← indivisible
│                   reading_1: "1 Kgs 8:1-7...",
│                   psalm: "Ps 132:6-10",
│                   gospel: "Mk 6:53-56", ...
│               })
│           },
│           ReadingsChoice {                          ← Common of Virgins
│               source: Common { common: Virgins, saint_id: "st_scholastica" },
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
│           SourcedText { source: ProperOfTime { day_id: "..." }, text: "..." },
│           SourcedText { source: Common { common: Virgins, ... }, text: "..." },
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
│       mass_time: MorningMass,                        ← feria of Dec 24
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

### 5. Layer 2 Hours — Hours Calendar

**Method:** `Calendar::generate_hours_calendar() → HoursCalendar`

**Principle:** Organized by civil date. Each entry is one Hour of the Office, self-contained with all options pre-resolved. Evening Prayer I of solemnities is shifted to the previous civil date. The consumer picks a celebration, and receives fully resolved content for that Hour.

**Why a separate layer (not merged with Layer 2 Mass):** The Mass and Office have fundamentally different composition patterns:

- **Mass** = **selection**: the consumer picks from options per substitution group (formulary block, readings, flexible orations). Each group has independent alternatives.
- **Office** = **overlay**: the celebration choice determines a composite content where weekday base elements and saint's proper elements are merged by the engine per GILH §235 rules. On privileged weekdays, saint's elements are _added alongside_, not substituted (GILH §239).

These two patterns require different data structures and composition rules. Merging them would require a single data structure to represent two fundamentally different composition patterns.

**Generated from Layer 1:** The engine first produces the `LiturgicalCalendar`, then transforms it into the `HoursCalendar` by: shifting Vespers I to the previous civil date, resolving which elements come from the weekday vs. the saint per GILH §235 rules, applying GILH §239 addition logic for privileged weekdays, and computing the applicable composition rules.

#### `HoursCalendar`

**What it is:** The top-level output type. A map from civil date to a list of Hours celebrated on that civil day.

**Why this name:** It is a "calendar" organized by "hours" — the practical perspective of which Hours of the Office are celebrated on each civil day.

**Why `Vec<HoursComposition>`:** Each entry is one Hour. A typical day has up to 7 entries (Office of Readings through Compline). When a solemnity begins tomorrow, today also receives a Vespers I entry (shifted), analogous to `PreviousEveningMass` in the Mass Calendar.

```rust
struct HoursCalendar(BTreeMap<String, Vec<HoursComposition>>);
```

> **Newtype pattern:** `HoursCalendar` wraps the `BTreeMap` to provide a named type with domain-specific methods (e.g., `hours_for_date`), consistent with `LiturgicalCalendar` and `MassCalendar`. In JSON, it serializes as a plain object keyed by date strings.

#### `HoursComposition`

**What it is:** A single Hour of the Office with all its options pre-resolved. The consumer picks a celebration and receives the fully resolved content.

**Why this name:** "Hours" because it represents one Hour of the Office. "Composition" because the Hour is "composed" from weekday and saint elements — the consumer receives the composed result.

**Why per-Hour (not per-day):** Although the celebration choice applies to the whole day (celebrating St. Scholastica at Lauds implies celebrating her at Vespers too), the _content_ differs per Hour (different psalms, different antiphons, different readings). And Vespers I may belong to a different celebration than the rest of the day. Per-Hour entries keep each unit self-contained, consistent with `MassComposition` in Layer 2 Mass.

```rust
struct HoursComposition {
    // === Identification ===
    /// Which Hour of the Office
    hour_time: HourTime,
    /// Civil date — after shifting for Vespers I
    civil_date: String,
    /// Liturgical date — before shifting (the "theological" date)
    liturgical_date: String,

    // === Context ===
    /// Shared day context
    context: DayContext,

    // === Default celebration ===
    default_celebration_id: CelebrationId,

    // === CELEBRATION OPTIONS ===
    /// Each option = one possible celebration with its pre-resolved Hour content.
    /// The consumer picks ONE option.
    /// GNLY 14: only one optional memorial may be celebrated per day.
    celebration_choices: Vec<HoursCelebrationChoice>,

    // === VESPERS CONFLICT ===
    /// When this Hour is Vespers and it won a GNLY §61 conflict against the
    /// Vespers of another celebration, this field identifies the losing
    /// celebration. None if no conflict. Allows the consumer to display a
    /// pastoral note (e.g., "First Vespers of Sunday. Vespers of the
    /// Saturday are omitted.").
    replaces_vespers_of: Option<CelebrationId>,

    // === COMPOSITION RULES ===
    /// How memorials interact with the weekday Office
    composition_rules: HoursCompositionRules,

    // === SUPPRESSION ===
    /// Indicates if this Hour is suppressed or replaced by a Mass celebration
    /// (GILH 209, 211, 212, 215). When present, the consumer should inform
    /// the user that this Hour may be omitted under certain conditions.
    suppression: Option<HourSuppression>,
}
```

#### `HoursCelebrationChoice`

**What it is:** One possible celebration that can be chosen for this Hour, with its fully resolved content. The engine has already applied GILH §235 rules — merging weekday base with saint's proper elements.

**Why this name:** It is one "choice" among the available "celebrations" for this "Hour." Analogous to `IdentityChoice` in the Mass model, but includes the full resolved content rather than just the formulary block.

**Why fully resolved:** Unlike the Mass where the consumer actively composes (picks readings, picks orations), the Office consumer receives a finished composite. Once the celebration is chosen, GILH §235 determines everything. The engine does the work, the consumer picks and uses.

```rust
struct HoursCelebrationChoice {
    /// Reference to the celebration
    celebration_id: CelebrationId,
    celebration_name: String,
    rank: Rank,
    precedence: Precedence,
    colors: Vec<ColorInfo>,
    commons: Vec<CommonInfo>,
    martyrology: Vec<MartyrologyEntry>,
    titles: TitlesDef,
    patronages: Vec<Patronage>,
    is_holy_day_of_obligation: bool,
    from_calendar_id: CalendarId,

    /// Fully resolved content for this Hour when this celebration is chosen.
    /// The engine has already merged weekday base + saint's proper per GILH §235.
    content: ResolvedHourContent,
}
```

#### `ResolvedHourContent`

**What it is:** The fully resolved textual content for one Hour of the Office. Each element is paired with its source (`SourcedText`) so the consumer knows where it comes from.

**Why "Resolved":** The engine has resolved the GILH §235 priority order: if the saint has a proper element, it appears here with `source: ProperOfSaint`; if not, it comes from the Common or weekday, and the source reflects that.

**Why `Option` on some fields:** Not all Hours have all elements. The invitatory is only at the first Hour of the day. The canticle antiphon (Benedictus/Magnificat) is only at Lauds/Vespers. Intercessions are only at Lauds/Vespers. Office of Readings content is only for that specific Hour.

```rust
struct ResolvedHourContent {
    /// Psalmody — resolved (from weekday on memorials, from Proper on solemnities)
    psalmody: HoursPsalmody,

    /// Proper elements — each resolved with its source
    invitatory_antiphon: Option<SourcedText>,
    hymn: SourcedText,
    short_reading: SourcedText,
    short_responsory: SourcedText,
    /// At Benedictus (Lauds) or Magnificat (Vespers) — GILH 116-119
    canticle_antiphon: Option<SourcedText>,
    intercessions: Option<SourcedText>,
    /// Exception: at Compline, the concluding prayer always comes from the
    /// psalter, never from Celebration.prayer (GILH §198). The engine must not
    /// resolve this field from the celebration's canonical prayer.
    concluding_prayer: SourcedText,

    /// Marian antiphon at the end of Night Prayer (Compline).
    /// Present only for HourTime::Compline. The antiphon varies by liturgical
    /// period (see Part IV §7, Period variants). None for other Hours.
    marian_antiphon: Option<SourcedText>,

    /// Office of Readings content (only for HourTime::OfficeOfReadings)
    office_readings: Option<OfficeReadingsContent>,
}
```

#### `OfficeReadingsContent`

**What it is:** The readings content specific to the Office of Readings. Two or three readings depending on context.

**Why separate `patristic_reading` and `hagiographical_reading`:** This models the GILH §235d/GILH §239 distinction explicitly:

| Context                                     | `patristic_reading` | `hagiographical_reading` | Behavior                                             |
| ------------------------------------------- | ------------------- | ------------------------ | ---------------------------------------------------- |
| Weekday (no memorial)                       | Present             | None                     | Patristic only                                       |
| Memorial in OT (GILH §235d)                 | None                | Present                  | Hagiographical **replaces** patristic                |
| Memorial on privileged weekday (GILH §239a) | Present             | Present                  | Hagiographical **added after** patristic             |
| Solemnity/Feast of a **Saint** (GILH §228)  | None                | Present                  | Proper reading about the saint serves as 2nd reading |
| Solemnity/Feast of the **Lord** (GILH §228) | Present             | None                     | Patristic reading from the Proper                    |

```rust
struct OfficeReadingsContent {
    /// First reading: from the Scripture continuous reading cycle
    scripture_reading: SourcedText,
    /// Patristic reading — from the current cycle or Common
    /// Present on weekdays, absent when replaced by hagiographical on memorials (GILH §235d)
    /// Present alongside hagiographical on privileged weekdays (GILH §239a)
    patristic_reading: Option<SourcedText>,
    /// Hagiographical reading — in honor of the saint
    /// Present on memorials (GILH §235d), during GILH §239 additions, and on solemnities/feasts
    hagiographical_reading: Option<SourcedText>,
    /// Te Deum — on solemnities, feasts, days within the Easter and Christmas
    /// Octaves, and Sundays outside Lent (GILH 68). Not said on memorials or
    /// weekdays. When `vigil_extension` is `Some(...)`, the Te Deum is sung
    /// AFTER the vigil canticles and Gospel (GILH §73), not before.
    te_deum: bool,
    /// Vigil extension — canticles and Gospel inserted BEFORE Te Deum (GILH 73;
    /// GILH §206 cross-refs GILH §73; GILH §215).
    /// `Some(...)` when the engine provides the vigil form of the Office of
    /// Readings for this day (solemnities, feasts, Sundays — GILH §73).
    /// `None` when the vigil form is not available (weekdays, memorials).
    /// The consumer decides whether to celebrate the vigil form.
    vigil_extension: Option<VigilExtension>,
}
```

#### `VigilExtension`

**What it is:** The additional elements appended to the Office of Readings when it is celebrated in its extended vigil form — canticles from the Old Testament followed by a Gospel reading.

**Why this name:** "Vigil" because this form is used at vigils of Sundays, solemnities, and feasts (GILH §73). "Extension" because it extends the Office of Readings — it is not a separate Hour, but additional content inserted after the two readings and **before** the Te Deum.

**Liturgical basis:**

- **GILH §73:** "After the two readings and before the Te Deum canticles should be added from the special appendix [...]. Then the gospel should be read; a homily on the gospel may be added. After this the Te Deum is sung and the prayer said." On solemnities/feasts: Gospel from the Lectionary for Mass; on Sundays: from the paschal mystery series in the appendix.
- **GILH §206:** References GILH §73 for Sunday vigils: "The way to celebrate Sunday vigils, as circumstances suggest, has been discussed in no. 73."
- **GILH §215:** On Christmas night, when the vigil form is used, Compline is omitted by those who attend.

**When present:** On solemnities (especially Easter, Christmas, Pentecost) and Sundays when the community chooses the vigil form. The Te Deum is sung _after_ the vigil canticles and Gospel (GILH §73), not before. The `te_deum: bool` field indicates whether Te Deum is said; the `vigil_extension` field adds the vigil elements that precede it.

```rust
struct VigilExtension {
    /// OT canticles from the special appendix (GILH 73)
    /// Typically 3 canticles with their antiphons
    canticles: Vec<PsalmodyEntry>,
    /// Gospel reading proclaimed after the canticles (GILH 73, 206)
    gospel: SourcedText,
}
```

#### `HoursCompositionRules`

**What it is:** The rules governing how memorials interact with the weekday Office. Simpler than `CompositionRules` for the Mass because the Office has less compositional flexibility — once the celebration is chosen, the content is determined by GILH §235 rules.

**Why simpler than Mass:** The Mass has three independent rule dimensions (formulary block, readings, flexible orations). The Office has one primary rule: how the memorial interacts with the weekday content. The GILH §235 resolution logic is handled by the engine, not exposed to the consumer.

```rust
struct HoursCompositionRules {
    /// How memorials interact with the weekday Office
    memorial: MemorialRule,
}
```

#### `MemorialRule`

**What it is:** An enum governing the celebration of memorials in the Office on a given day.

**Why this name:** It is a "rule" about "memorials" — whether they can be celebrated, and if so, how they interact with the weekday content.

```rust
enum MemorialRule {
    /// Free choice: celebrate the memorial or the weekday
    /// Engine applies GILH §235 substitution rules (saint's elements replace weekday)
    /// (Ordinary Time)
    FreeChoice,
    /// Memorial elements may only be added alongside weekday content
    /// Engine applies GILH §239 addition rules (hagiographical reading after patristic,
    /// saint's antiphon and prayer appended)
    /// (Advent Dec 17-24, Christmas Octave, Lent — except Ash Wed and Holy Week)
    AdditionsOnly,
    /// No memorial permitted at all
    /// (Ash Wed, Holy Week, Easter Octave, Sundays, Solemnities, Feasts)
    NoMemorial,
}
```

#### `HourSuppression`

**What it is:** An enum indicating that a specific Hour of the Office is conditionally suppressed or replaced by a Mass celebration. This models the Triduum and Christmas exceptions where attending a liturgical celebration makes a subsequent Hour redundant.

**Why this name:** "Hour" because it concerns a specific Hour of the Office. "Suppression" because the Hour is suppressed (not celebrated) under certain conditions — the GILH's own language (e.g., GILH §209: "il est convenable d'omettre les Vêpres").

**Liturgical basis:**

| Paragraph     | Context                            | Rule                                                                                                                                                                                 |
| ------------- | ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **GILH §209** | Holy Thursday / Good Friday        | Those who attend the evening Mass of the Lord's Supper (Holy Thursday) or the Celebration of the Lord's Passion (Good Friday) omit Vespers.                                          |
| **GILH §211** | Holy Saturday                      | Those who attend the Easter Vigil omit Compline.                                                                                                                                     |
| **GILH §212** | Holy Saturday — Office of Readings | The Easter Vigil takes the place of the Office of Readings. For those who cannot attend the Vigil, a reduced Office of Readings is provided (choosing 4 of the Vigil's OT readings). |
| **GILH §215** | Christmas night                    | Those who celebrate the vigil form of the Office of Readings before Midnight Mass omit Compline.                                                                                     |

**Why `Option<HourSuppression>` (not always present):** Most Hours are never suppressed. This field is `None` for all ordinary days. It is `Some(...)` only on the handful of exceptional days listed above.

**Why not a boolean:** The suppression is _conditional_ — it depends on what the person attends. The consumer needs to know _which_ celebration triggers the suppression in order to inform the user correctly ("Those who attend the Easter Vigil omit Compline").

```rust
enum HourSuppression {
    /// This Hour is omitted if the person attends the referenced celebration.
    /// The Hour content is still provided for those who do NOT attend.
    /// The triggering celebration may be a Mass (Holy Thursday, Easter Vigil),
    /// a non-eucharistic action (Good Friday — CelebrationOfThePassion),
    /// or an Office celebration (Christmas — vigil form of OdR, GILH §215).
    SuppressedIfAttends {
        /// The celebration whose attendance triggers suppression
        celebration_id: CelebrationId,
    },
    /// This Hour is entirely replaced by a Mass celebration.
    /// The `content` in `celebration_choices` carries the reduced form
    /// for those who cannot attend the Mass.
    /// (GILH 212: Easter Vigil replaces Office of Readings on Holy Saturday)
    ReplacedByMass {
        /// The Mass celebration that replaces this Hour
        celebration_id: CelebrationId,
    },
}
```

#### Layer 2 Hours — Example

```
HoursCalendar
│
├── "2025-02-10" → [
│   HoursComposition {
│       hour_time: OfficeOfReadings,
│       civil_date: "2025-02-10",
│       liturgical_date: "2025-02-10",
│       context: DayContext { season: OrdinaryTime, week: 5, ... },
│       default_celebration_id: "ord_time_5_mon",
│
│       celebration_choices: [
│           HoursCelebrationChoice {                 ← feria
│               celebration_id: "ord_time_5_mon",
│               rank: Weekday,
│               content: ResolvedHourContent {
│                   psalmody: HoursPsalmody { ... },  ← weekday psalms
│                   hymn: SourcedText { source: ProperOfTime, ... },
│                   concluding_prayer: SourcedText { source: ProperOfTime, ... },
│                   office_readings: Some(OfficeReadingsContent {
│                       scripture_reading: SourcedText { ... },
│                       patristic_reading: Some(SourcedText { ... }),
│                       hagiographical_reading: None,
│                       te_deum: false,
│                   }),
│                   ...
│               }
│           },
│           HoursCelebrationChoice {                 ← memorial
│               celebration_id: "st_scholastica",
│               rank: OptionalMemorial,
│               content: ResolvedHourContent {
│                   psalmody: HoursPsalmody { ... },  ← same weekday psalms (GILH §235a)
│                   hymn: SourcedText { source: Common { common: Virgins, ... }, ... },
│                   concluding_prayer: SourcedText {  ← mandatory from saint (GILH §235c)
│                       source: ProperOfSaint("st_scholastica"), ...
│                   },                                ← resolved from Celebration.prayer (CP §44)
│                   office_readings: Some(OfficeReadingsContent {
│                       scripture_reading: SourcedText { ... },  ← same cycle (GILH §235d)
│                       patristic_reading: None,                 ← replaced (GILH §235d)
│                       hagiographical_reading: Some(SourcedText {
│                           source: ProperOfSaint("st_scholastica"), ...
│                       }),
│                       te_deum: false,
│                   }),
│                   ...
│               }
│           },
│       ],
│
│       composition_rules: HoursCompositionRules {
│           memorial: FreeChoice,
│       }
│   },
│
│   HoursComposition { hour_time: Lauds, ... },
│   HoursComposition { hour_time: Terce, ... },
│   HoursComposition { hour_time: Sext, ... },
│   HoursComposition { hour_time: Nones, ... },
│   HoursComposition { hour_time: Vespers, ... },
│   HoursComposition { hour_time: Compline, ... },
│ ]
│
├── "2025-03-10" → [                                ← Lenten weekday with memorial
│   HoursComposition {
│       hour_time: OfficeOfReadings,
│       ...
│       celebration_choices: [
│           HoursCelebrationChoice {                 ← feria
│               celebration_id: "lent_2_mon",
│               rank: Weekday,
│               content: ResolvedHourContent {
│                   office_readings: Some(OfficeReadingsContent {
│                       patristic_reading: Some(...),
│                       hagiographical_reading: None,
│                       ...
│                   }),
│                   ...
│               }
│           },
│           HoursCelebrationChoice {                 ← memorial (GILH §239 additions)
│               celebration_id: "st_john_ogilvie",
│               rank: OptionalMemorial,              ← demoted per GILH §238
│               content: ResolvedHourContent {
│                   office_readings: Some(OfficeReadingsContent {
│                       patristic_reading: Some(...),      ← KEPT (GILH §239a)
│                       hagiographical_reading: Some(...), ← ADDED after (GILH §239a)
│                       ...
│                   }),
│                   concluding_prayer: SourcedText {       ← from saint (GILH §239a)
│                       source: ProperOfSaint("st_john_ogilvie"), ...
│                   },
│                   ...
│               }
│           },
│       ],
│
│       composition_rules: HoursCompositionRules {
│           memorial: AdditionsOnly,                 ← GILH §239 mechanism
│       }
│   },
│   ...
│ ]
│
├── "2025-12-24" → [                                ← Vespers I shifted
│   HoursComposition { hour_time: OfficeOfReadings, ... },  ← Dec 24 feria
│   HoursComposition { hour_time: Lauds, ... },
│   ...
│   HoursComposition {
│       hour_time: VespersI,                         ← Christmas Vespers I
│       civil_date: "2025-12-24",                    ← shifted here
│       liturgical_date: "2025-12-25",               ← belongs to Dec 25
│       celebration_choices: [
│           HoursCelebrationChoice {
│               celebration_id: "christmas",
│               rank: Solemnity,
│               content: ResolvedHourContent { ... }, ← proper solemnity content
│           },
│       ],
│       composition_rules: HoursCompositionRules {
│           memorial: NoMemorial,
│       }
│   },
│ ]
```

### 6. Type Sharing Summary

```
Type                         L1    L2M   L2H   Scope
──────────────────────────  ────  ────  ────  ──────────
DayContext                     ✓       ✓       ✓    SHARED
ReadingText                    ✓       ✓       ✓    SHARED
ShortForm                      ✓       ✓       ✓    SHARED
SourceRef                      ✓       ✓       ✓    SHARED
AntiphonText                   ✓       ✓       ✗    SHARED (L1+L2M)
PrefaceText                    ✓       ✓       ✗    SHARED (L1+L2M)
TextSource                     ✗       ✓       ✓    SHARED (L2M+2H)
SourcedText                    ✗       ✓       ✓    SHARED (L2M+2H)
SourcedPreface                 ✗       ✓       ✗    L2M
CelebrationId                  ✓       ✓       ✓    SHARED

FormularySet                   ✓       ✓ ¹     ✗    MASS
ReadingsSet                    ✓       ✓ ¹     ✗    MASS
AlleluiaText                   ✓       ✓ ¹     ✗    MASS
AcclamationType                ✓       ✓ ¹     ✗    MASS
SequenceText                   ✓       ✓ ¹     ✗    MASS
ReadingsPool                   ✓       ✓ ¹     ✗    MASS
ReadingsContent                ✓       ✓ ¹     ✗    MASS
VigilReadingsSequence          ✓       ✓ ¹     ✗    MASS (Easter Vigil)
VigilReading                   ✓       ✓ ¹     ✗    MASS (Easter Vigil)
FlexibleOrations               ✓       ✗ ²     ✗    MASS (L1)

LiturgicalCalendar             ✓       ✗       ✗    L1
LiturgicalDay                  ✓       ✗       ✗    L1
Celebration                    ✓       ✗       ✗    L1
CelebrationMass                ✓       ✗       ✗    L1 (Mass)
CelebrationHour                ✓       ✗       ✗    L1
CelebrationOfficeReadings      ✓       ✗       ✗    L1 (Office)

MassCalendar                   ✗       ✓       ✗    L2M
MassComposition                ✗       ✓       ✗    L2M
IdentityChoice                 ✗       ✓       ✗    L2M
ReadingsChoice                 ✗       ✓       ✗    L2M
ReadingsCategory               ✗       ✓       ✗    L2M
CompositionRules               ✗       ✓       ✗    L2M
BlockRule                      ✗       ✓       ✗    L2M
ReadingsRule                   ✗       ✓       ✗    L2M
FlexibleRule                   ✗       ✓       ✗    L2M

HoursCalendar                  ✗       ✗       ✓    L2H
HoursComposition               ✗       ✗       ✓    L2H
HoursCelebrationChoice         ✗       ✗       ✓    L2H
ResolvedHourContent            ✗       ✗       ✓    L2H
OfficeReadingsContent          ✗       ✗       ✓    L2H
VigilExtension                 ✗       ✗       ✓    L2H
HoursCompositionRules          ✗       ✗       ✓    L2H
MemorialRule                   ✗       ✗       ✓    L2H
HourSuppression                ✗       ✗       ✓    L2H

HourTime                       ✓       ✗       ✓    SHARED (L1+2H)
HoursPsalmody                  ✓       ✗       ✓    SHARED (L1+2H)
PsalmodyEntry                  ✓       ✓       ✓    SHARED (Mass + Office)
PsalmAntiphon                  ✓       ✓       ✓    SHARED (Mass + Office)

Existing types (documented)    ✓       ✓       ✓    SHARED
  Season, Rank, Precedence, MassTime ³, Common, CommonInfo,
  Color, ColorInfo, DayOfWeek, SundayCycle, WeekdayCycle,
  PsalterWeekCycle, PeriodInfo, TitlesDef, TitleCategory, Title,
  PatronRole, Patronage, MartyrologyEntry, CalendarId

¹ Reused inside IdentityChoice / ReadingsChoice / ReadingsContent
² Exploded into Vec<SourcedText> per oration (or Vec<SourcedPreface> for preface) in Layer 2 Mass
³ MassTime: existing enum, documented in Part IV §4 with all 10 variants
```

### 7. Existing Types Reference

The types below exist in the romcal codebase and are referenced by the new types defined in this document. This section documents their structure and role in the final data model (output types), not the internal resolution or definition pipeline.

#### `Common`

**What it is:** An enum identifying a specific Common of the Roman Missal / Lectionary — a pool of liturgical texts (prayers, readings, antiphons) categorized by type of saint.

**Liturgical basis:** The Roman Missal organizes Proper texts for saints and provides Common texts as fallbacks. Each Common corresponds to a category of saints (Martyrs, Virgins, Pastors, etc.) and provides a complete set of Mass formularies and readings. When a memorial has no proper text for a given element, the celebrant draws from the applicable Common.

**How it works in the composition model:**

- A `Celebration.commons` vector lists the applicable Commons for that celebration (e.g., `[Martyrs_OutsideEaster_One, Virgins_One]` for a virgin martyr in Ordinary Time).
- When the celebration has no proper readings, the engine generates a `ReadingsChoice` with `source: TextSource::Common` and `readings: Pool(ReadingsPool)` for each applicable Common.
- The celebrant may freely choose from any of the applicable Commons' pools (GILM 71: "the celebrant may choose at will from such texts").
- A celebration with all proper texts has `commons: []` (empty) — no Common is needed.
- The `Common` enum is season-aware for the Blessed Virgin Mary (BVM has distinct Commons for Ordinary Time, Advent, Christmas, and Easter).

**Variants (34):**

```rust
enum Common {
    None,
    // Dedication of a Church
    DedicationAnniversary_Inside, DedicationAnniversary_Outside,
    // Blessed Virgin Mary (season-specific)
    BlessedVirginMary_OrdinaryTime, BlessedVirginMary_Advent,
    BlessedVirginMary_Christmas, BlessedVirginMary_Easter,
    // Martyrs
    Martyrs_OutsideEaster_Several, Martyrs_OutsideEaster_One,
    Martyrs_Easter_Several, Martyrs_Easter_One,
    Martyrs_Missionary_Several, Martyrs_Missionary_One,
    Martyrs_Virgin, Martyrs_Woman,
    // Pastors
    Pastors_PopeOrBishop, Pastors_Bishop,
    Pastors_Several, Pastors_One,
    Pastors_Founder_One, Pastors_Founder_Several,
    Pastors_Missionary,
    // Doctors of the Church
    DoctorsOfTheChurch,
    // Virgins
    Virgins_Several, Virgins_One,
    // Holy Men and Women
    Saints_All_Several, Saints_All_One,
    Saints_Abbot, Saints_Monk, Saints_Nun,
    Saints_Religious, Saints_MercyWorks,
    Saints_Educators, Saints_HolyWomen,
}
```

> **Note on `CommonDef`:** The definition pipeline uses a simplified enum (`CommonDef`, e.g., `Martyrs`, `Virgins`) from which the engine deduces the fully resolved `Common` variant based on the `MartyrologyEntry` properties (season, `SaintCount`, sex). This resolution is internal to the engine and not exposed in the output types.

#### `CommonInfo`

**What it is:** A `Common` enum value paired with its localized display name.

```rust
struct CommonInfo {
    /// The resolved Common variant
    key: Common,
    /// Localized name (e.g., "Common of One Martyr" / "Commun d'un Martyr")
    name: String,
}
```

#### `Precedence` (GNLY 59)

**What it is:** An enum representing the liturgical precedence level from the Table of Liturgical Days (GNLY 59). Determines which celebration takes priority when multiple celebrations fall on the same date.

**Structure:** GNLY 59 defines 13 numbered levels. romcal subdivides these into **27 variants** to distinguish sub-levels (e.g., level 2 has 5 sub-variants, level 4 has 4 sub-variants for different types of proper solemnities, level 8 has 6 sub-variants for different types of proper feasts):

```rust
enum Precedence {
    // 1. Paschal Triduum
    Triduum_1,
    // 2. Proper of Time solemnities, privileged Sundays, Ash Wednesday,
    //    Holy Week weekdays, Easter Octave
    ProperOfTimeSolemnity_2, PrivilegedSunday_2, AshWednesday_2,
    WeekdayOfHolyWeek_2, WeekdayOfEasterOctave_2,
    // 3. General Calendar solemnities + All Souls
    GeneralSolemnity_3, CommemorationOfAllTheFaithfulDeparted_3,
    // 4. Proper solemnities (patron, dedication, title, religious org)
    ProperSolemnity_PrincipalPatron_4a, ProperSolemnity_DedicationOfTheOwnChurch_4b,
    ProperSolemnity_TitleOfTheOwnChurch_4c,
    ProperSolemnity_TitleOrFounderOrPrimaryPatronOfAReligiousOrg_4d,
    // 5. General Calendar feasts of the Lord
    GeneralLordFeast_5,
    // 6. Unprivileged Sundays (Christmas Time, Ordinary Time)
    UnprivilegedSunday_6,
    // 7. General Calendar feasts (BVM, saints)
    GeneralFeast_7,
    // 8. Proper feasts (diocese, cathedral, region, religious org, individual church)
    ProperFeast_PrincipalPatronOfADiocese_8a,
    ProperFeast_DedicationOfTheCathedralChurch_8b,
    ProperFeast_PrincipalPatronOfARegion_8c,
    ProperFeast_TitleOrFounderOrPrimaryPatronOfAReligiousOrg_8d,
    ProperFeast_ToAnIndividualChurch_8e, ProperFeast_8f,
    // 9. Privileged weekdays (Advent Dec 17-24, Christmas Octave, Lent)
    PrivilegedWeekday_9,
    // 10. General Calendar obligatory memorials
    GeneralMemorial_10,
    // 11. Proper obligatory memorials
    ProperMemorial_SecondPatron_11a, ProperMemorial_11b,
    // 12. Optional memorials
    OptionalMemorial_12,
    // 13. Weekdays
    Weekday_13,
}
```

> **`Precedence.to_rank()`:** Each `Precedence` variant maps deterministically to a `Rank`. The Triduum (level 1), Ash Wednesday, Holy Week weekdays, and privileged weekdays (level 9) have `Rank::Weekday` despite their high precedence — their importance is conveyed by precedence, not rank. Easter Octave days have `Rank::Solemnity`. All Souls has `Rank::Feast` (see note below).
>
> **All Souls — _sui generis_ celebration.** GNLY §59 places the Commemoration of All the Faithful Departed at precedence level 3, alongside General Calendar Solemnities. However, All Souls is liturgically a **Mass for the Dead**, not a standard solemnity or feast:
>
> | Element            | Solemnity | Feast   | All Souls (actual) |
> | ------------------ | --------- | ------- | ------------------ |
> | Gloria (GIRM §53)  | Yes       | Yes     | **No**             |
> | Creed (GIRM §68)   | Yes       | No      | **No**             |
> | Te Deum (GILH §68) | Yes       | Yes     | **No**             |
> | First Vespers      | Yes       | No      | **No**             |
> | Colors             | Festive   | Festive | Purple / Black     |
>
> `Rank::Feast` is a **pragmatic approximation**: it correctly reflects no First Vespers, no Creed, and the "within the natural day" scope (GNLY §13). The Gloria and Te Deum exceptions (which a standard Feast would have) are handled by the engine through the `gloria: bool` and `te_deum: bool` computed fields — both set to `false` for All Souls because it is a Mass for the Dead. This avoids the need for a dedicated Rank variant for a single celebration.

#### `Rank`

**What it is:** The liturgical rank of a celebration (GNLY 10-13). Determines which composition rules apply.

```rust
enum Rank {
    Solemnity,       // Highest: full proper, First Vespers, Gloria, Creed
    Sunday,          // "Primordial feast day" — yields only to higher solemnities
    Feast,           // Proper within the natural day, no First Vespers (exceptions exist)
    Memorial,        // Obligatory; demoted to OptionalMemorial in Lent (GNLY 14)
    OptionalMemorial,// Non-obligatory; only one may be celebrated per day (GNLY 14)
    Weekday,         // Feria — base celebration when no saint is celebrated
}
```

#### `Season`

**What it is:** The liturgical season of the Church year.

**Liturgical basis:** GNLY 17-44 defines five liturgical seasons (tempora). Each has its own liturgical characteristics (colors, readings cycle, presence/absence of Gloria and Alleluia).

```rust
enum Season {
    Advent,          // First Vespers of the Sunday nearest Nov 30
                     // → before First Vespers of Christmas (GNLY §40)
    ChristmasTime,   // First Vespers of Christmas (Dec 24 evening)
                     // → Baptism of the Lord inclusive (GNLY §33)
    Lent,            // Ash Wednesday → Mass of the Lord's Supper exclusive (GNLY §28)
    EasterTime,      // Easter Sunday → Pentecost (GNLY §22)
    OrdinaryTime,    // Two periods (GNLY §44):
                     // Day after the Baptism of the Lord → day before Ash Wednesday
                     // Monday after Pentecost → day before First Vespers of Advent I
                     // Note: the Baptism of the Lord may be celebrated on Sunday
                     // OR on Monday (when Epiphany falls Jan 7/8), so OT may begin
                     // on Monday or Tuesday.
}
```

> The Paschal Triduum is NOT a Season — see Part III §7 for normative analysis and `DayContext.season` values during the Triduum.

#### `Color` and `ColorInfo`

**What it is:** Liturgical colors prescribed for vestments and decorations (GIRM 346).

```rust
enum Color {
    White,   // Christmas and Easter seasons; celebrations of the Lord (non-Passion);
             // BVM; Holy Angels; saints who were not martyrs (GIRM 346a)
    Red,     // Palm Sunday, Good Friday, Pentecost; celebrations of the Lord's Passion
             // (e.g. Exaltation of the Holy Cross); Apostles and Evangelists;
             // Martyr Saints (GIRM 346b)
    Green,   // Ordinary Time (GIRM 346c)
    Purple,  // Advent, Lent; Masses and Offices for the Dead (GIRM 346d)
    Rose,    // Gaudete Sunday (3rd Advent), Laetare Sunday (4th Lent)
             // — "where it is the practice" (GIRM 346f)
    Black,   // Masses for the Dead — "where it is the practice" (GIRM 346e)
    Gold,    // See note below on festive vestments
    Blue,    // Marian feasts — by national indult only (not in universal GIRM)
}

struct ColorInfo {
    key: Color,
    name: String, // localized
}
```

**Notes on specific colors:**

- **Purple vs. Black for the Dead:** GIRM 346 positions purple as the standard color for Masses and Offices for the Dead (§346d), while black is the local-practice alternative (§346e: "where it is the practice"). Purple is the primary choice in most contemporary usage.
- **Gold / festive vestments:** GIRM §346g says: "On more solemn days, sacred vestments may be used that are festive, that is, more precious, even if not of the color of the day." Gold is not named explicitly in the universal GIRM, but comes from national adaptations (e.g., US GIRM §346 mentions gold/silver). In romcal, `Gold` serves as a concrete variant for this rule.
- **Blue:** Not present in the universal GIRM §346. Some countries (notably Spain, parts of Latin America, the Philippines) have a Marian indult authorizing blue for feasts of the Blessed Virgin Mary. The `Blue` variant is included in the `Color` enum to support these calendars.

**Why `Vec<ColorInfo>`:** A celebration may have multiple permissible colors. Examples: gold as alternative on solemnities (§346g); black as alternative to purple for the Dead (§346d-e); blue as alternative to white for Marian feasts (by indult). The engine automatically assigns red for martyrs based on the `Title::Martyr` in `MartyrologyEntry.titles`.

#### `CalendarId`

**What it is:** A unique identifier for a calendar in the inheritance chain.

```rust
struct CalendarId(String);
```

> **Newtype pattern:** Same rationale as `CelebrationId` — prevents mixing `CalendarId` with `CelebrationId` or plain `String`. In JSON, it serializes as a plain string.

**Role in the model:** `Celebration.from_calendar_id` identifies which calendar last defined or modified this celebration. When a celebration is defined in the General Roman Calendar and a local calendar overrides some properties, `from_calendar_id` is the local calendar's ID. The complete modification history is available in `LiturgicalDay.parent_overrides`.

#### `ParentOverride`

**What it is:** A diff-based record of how a parent calendar modified a celebration. Only fields that actually changed are `Some(...)` — unchanged fields are `None`. This allows the consumer to trace the full inheritance chain and understand what each calendar contributed.

**Why diff-based:** A celebration may be defined in the General Roman Calendar, then modified by a national calendar, then further modified by a diocesan calendar. Each `ParentOverride` captures only the delta introduced by one calendar, not the full state. This is more compact and makes it clear what each calendar changed.

```rust
struct ParentOverride {
    /// Which calendar introduced this override
    from_calendar_id: CalendarId,
    // Only changed fields are Some(...):
    date_def: Option<DateDef>,
    date_exceptions: Option<Vec<DateDefException>>,
    precedence: Option<Precedence>,
    rank: Option<Rank>,
    colors: Option<Vec<ColorInfo>>,
    titles: Option<TitlesDef>,
    commons: Option<Vec<CommonInfo>>,
    is_holy_day_of_obligation: Option<bool>,
    is_optional: Option<bool>,
}
```

> **Ordering:** The `parent_overrides` vec in `LiturgicalDay` is ordered from most general (e.g., `general_roman`) to most specific (e.g., `diocese_paris`). Only overrides with actual changes are included — a calendar that inherits a celebration without modifying it does not produce an entry.

#### `DayOfWeek`

**What it is:** The day of the week. Used in `DayContext.day_of_week`.

```rust
enum DayOfWeek { Sunday, Monday, Tuesday, Wednesday, Thursday, Friday, Saturday }
```

#### Cycle types

**What they are:** Enums identifying the liturgical reading cycle and psalter week.

```rust
/// Three-year Sunday readings cycle (GILM 66, GNLY 3-4)
enum SundayCycle { YearA, YearB, YearC }

/// Two-year weekday readings cycle (GILM 69)
enum WeekdayCycle { Year_1, Year_2 }

/// Four-week psalter cycle (GILH §133)
/// Restarted at Week 1 on: 1st Sunday of Advent, 1st Sunday of OT,
/// 1st Sunday of Lent, Easter Sunday.
enum PsalterWeekCycle { Week_1, Week_2, Week_3, Week_4 }
```

#### `PeriodInfo`

**What it is:** Specific sub-periods within or across liturgical seasons, plus pastoral observances. Most sub-periods determine applicable rules for texts, readings, and Office structure, and are essential for religious and monastic liturgies (particularly the Benedictine monastic office per the TLHM). Some periods represent pastoral observances that do not modify liturgical texts but are broadly observed in parishes and religious communities.

```rust
enum Period {
    // ── Christmas Time sub-periods ──
    ChristmasOctave,       // Dec 25 → Jan 1 (Octave Day)
    DaysBeforeEpiphany,    // Jan 2 → day before Epiphany
    DaysFromEpiphany,      // Epiphany → Sunday of the Baptism of the Lord

    // ── Advent sub-period ──
    DaysBeforeChristmas,      // Dec 17 → Dec 24

    // ── Broader cross-season periods ──
    ChristmasToPresentationOfTheLord,       // Dec 25 → Feb 2
    PresentationOfTheLordToHolyThursday,    // Feb 2 → Holy Thursday

    // ── Holy Week and Triduum ──
    HolyWeek,              // Palm Sunday → Holy Saturday
    PaschalTriduum,        // Holy Thursday evening → Easter Sunday Vespers

    // ── Easter Time sub-period ──
    EasterOctave,          // Easter Sunday → 2nd Sunday of Easter

    // ── Ordinary Time sub-periods ──
    EarlyOrdinaryTime,     // Monday after Baptism of the Lord → day before Ash Wednesday
    LateOrdinaryTime,      // Monday after Pentecost → Saturday before 1st Sunday of Advent

    // ── Pastoral/ecumenical observance ──
    WeekOfPrayerForChristianUnity, // Jan 18 → Jan 25
}

struct PeriodInfo {
    key: Period,
    name: String, // localized
}
```

**Why `Vec<PeriodInfo>`:** A day can belong to multiple overlapping periods (e.g., Good Friday belongs to both `HolyWeek` and `PaschalTriduum`; Dec 28 belongs to both `ChristmasOctave` and `ChristmasToPresentationOfTheLord`).

**Normative basis per variant:**

| Period                                | Status         | Source                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                   |
| ------------------------------------- | -------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `DaysBeforeChristmas`                 | Normative      | The final week of Advent (Dec 17-24), named by GNLY §42: "The weekdays from December 17 up to and including December 24 are ordered in a more direct way to preparing for the Nativity of the Lord." Precedence level 9 (GNLY §59). Each day has proper readings (GILM) and a proper O Antiphon at the Magnificat (GILH). Named `DaysBeforeChristmas` to mirror `DaysBeforeEpiphany`; the scope is narrower than "all of Advent" — only the 8 days with proper texts and elevated precedence.                                                                                                            |
| `ChristmasOctave`                     | Normative      | GNLY §12, §35; GILH §238 (no obligatory memorials during octave)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `DaysBeforeEpiphany`                  | Normative      | GNLY §35-36, §353 (weekdays from Jan 2); distinct memorial rules                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                         |
| `DaysFromEpiphany`                    | Normative      | TLHM « Ordinarium Tempore Epiphaniae » (explicit named sub-period with own proper texts); GILH §149 (readings from Isaiah 60-66 from Jan 7)                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `ChristmasToPresentationOfTheLord`    | Rubrical       | The Presentation of the Lord (Feb 2) is a period boundary in the rubrics of the Liturgia Horarum for the final Marian antiphon of Compline (GILH §92): _Alma Redemptoris Mater_ is sung from First Vespers of Advent through Feb 2. This period captures the Christmas portion of that span. Also relevant to monastic propers (TLHM §2 Tempus Nativitatis — sub-period details to be confirmed).                                                                                                                                                                                                        |
| `PresentationOfTheLordToHolyThursday` | Rubrical       | Corresponds to the _Ave Regina Caelorum_ period: sung from Feb 2 through Holy Wednesday (Liturgia Horarum rubrics, cf. GILH §92). This cross-season period spans Early Ordinary Time, Lent, and the beginning of Holy Week. Also used in monastic/religious propers for text selection.                                                                                                                                                                                                                                                                                                                  |
| `HolyWeek`                            | Normative      | GNLY §30-31; PS §27, §134; TLHM Tempus Quadragesimae distinguishes Office texts of weeks I-V from Holy Week                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| `PaschalTriduum`                      | Normative      | GNLY §18-21; PS §2, §38-99; TLHM has its own section (§4 Sacrum Triduum Paschale, separate from the five seasons)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `EasterOctave`                        | Normative      | GNLY §12, §24 (celebrated as Solemnities of the Lord)                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| `EarlyOrdinaryTime`                   | Semi-normative | GILH §152: "From the Monday after the feast of the Baptism of the Lord until Lent" (explicit boundary). GNLY §43 acknowledges two runs of OT without naming them.                                                                                                                                                                                                                                                                                                                                                                                                                                        |
| `LateOrdinaryTime`                    | Semi-normative | GILH §152: "from the Monday after Pentecost until Advent" (explicit boundary). Same status as above.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| `WeekOfPrayerForChristianUnity`       | Pastoral       | Fixed dates: Jan 18 (former feast of the Chair of St. Peter) → Jan 25 (Conversion of St. Paul). Not a liturgical period that modifies Mass or Office texts, but a pastoral and ecumenical observance encouraged in parishes and religious communities. Established in 1908 (P. Paul Wattson), renewed in 1935 (Abbé Paul Couturier, Lyon). Official basis: _Unitatis Redintegratio_ §8 (Vatican II); Directory for the Application of Principles and Norms on Ecumenism §110 (PCPCU, 1993). Materials prepared jointly by the Dicastery for Promoting Christian Unity and the World Council of Churches. |

> **The Presentation of the Lord as period boundary — Marian antiphons of Compline.**
>
> The Liturgia Horarum prescribes one of four Marian antiphons at the end of Night Prayer (Compline), each assigned to a specific period of the year (GILH §92). These assignments, inherited from the Breviarium Romanum, make the Presentation of the Lord (Feb 2) a rubrical period boundary:
>
> | Marian antiphon          | Period                             | Corresponding `Period` variant(s)                  |
> | ------------------------ | ---------------------------------- | -------------------------------------------------- |
> | _Alma Redemptoris Mater_ | 1st Vespers of Advent → Feb 2      | Advent season + `ChristmasToPresentationOfTheLord` |
> | _Ave Regina Caelorum_    | Feb 2 → Holy Wednesday             | `PresentationOfTheLordToHolyThursday`              |
> | _Regina Caeli_           | Easter → Pentecost                 | Easter Time season                                 |
> | _Salve Regina_           | Trinity → Saturday before Advent I | ≈ `LateOrdinaryTime`                               |
>
> These four periods cross seasonal boundaries (e.g., _Alma Redemptoris_ spans Advent, Christmas Time, and early Ordinary Time), which is precisely why they are modeled as `Period` (cross-season sub-periods) rather than as properties of `Season`.

> **Note on the TLHM (Thesaurus Liturgiae Horarum Monasticae):** The TLHM organizes its Proprium de Tempore into 7 sections: (1) Tempus Adventus, (2) Tempus Nativitatis, (3) Tempus Quadragesimae, (4) Sacrum Triduum Paschale, (5) Tempus Paschale, (6) Sollemnitates Domini per Annum occurrentes, (7) Tempus per Annum. This structure confirms the Triduum as a distinct liturgical unit and the Solemnities of the Lord in OT as a separate organizational grouping. The sub-period details within each section (particularly §2 Tempus Nativitatis for the Presentation boundary) remain to be documented from the TLHM Proprium. <!-- TODO: TLHM sub-period details -->

#### `MartyrologyEntry`

**What it is:** Metadata about a person, entity, or group whose memorial, feast, or solemnity is celebrated. Based on the Roman Martyrology — the official catalog of saints recognized by the Catholic Church.

**Key fields (subset relevant to the composition model):**

```rust
struct MartyrologyEntryId(String);

enum MartyrologyEntryType {
    /// A single person (saint, blessed, angel, Mary...)
    Person,
    /// A group of persons (Holy Innocents, Companions of...).
    /// `count` is the known number, or `None` if unknown ("and companions").
    Group { count: Option<u32> },
    /// A non-person celebration (Dedication of a Basilica, Exaltation of the Holy Cross...)
    Event,
}

enum CanonizationLevel {
    Saint,
    Blessed,
}

enum Sex {
    Male,
    Female,
}

struct MartyrologyEntry {
    id: MartyrologyEntryId,
    r#type: MartyrologyEntryType,
    fullname: Option<String>,
    name: Option<String>,             // Short name (without canonization level/titles)
    canonization_level: Option<CanonizationLevel>,  // None for Event
    titles: Option<Vec<Title>>,       // Martyr, Virgin, Bishop, Doctor, etc.
    sex: Option<Sex>,                 // None for Group/Event
    // ... dates (birth, death, canonization, beatification, dedication)
    // ... display flags (hide_canonization_level, hide_titles)
}
```

**Role in the composition model:**

- `titles` determines automatic color assignment (presence of `Title::Martyr` → red)
- `r#type` and `sex` are used by the engine to resolve `CommonDef` → `Common` (e.g., `Martyrs` + `Group { count: Some(3) }` → `Martyrs_OutsideEaster_Several`)
- `canonization_level` affects the display name but not the composition rules

> **Scope note:** The `MartyrologyEntry` struct carries rich biographical metadata that is out of scope for this document. Only the fields that affect composition (type, titles, sex) or consumer display (name, canonization_level) are listed here.

#### `TitlesDef`, `Title`, `TitleCategory`, `Patronage`

**What it is:** The titles associated with a celebration as published in the Missal, Lectionary, and Liturgy of the Hours. See Part III §8 for the full design decision and type definitions (`TitleCategory`, `Title`, `PatronRole`, `Patronage`).

```rust
enum TitlesDef {
    /// Direct list: replaces all titles
    Titles(Vec<Title>),
    /// Compound: appends/prepends to inherited titles
    CompoundTitle(CompoundTitle),
}

struct CompoundTitle {
    /// Titles added before the inherited titles
    prepend: Vec<Title>,
    /// Titles added after the inherited titles
    append: Vec<Title>,
}
```

`Celebration` and `HoursCelebrationChoice` carry both `titles: TitlesDef` and `patronages: Vec<Patronage>`.

---

## Part V — Architecture and Pipeline

### 1. Module Organization

```
core/src/types/
├── shared/                          SHARED TYPES
│   ├── day_context.rs               DayContext
│   ├── text_blocks.rs               FormularySet, ReadingText, ReadingsSet,
│   │                                ReadingsPool, ReadingsContent, FlexibleOrations
│   ├── sourced_text.rs              TextSource, SourcedText
│   ├── psalmody.rs                  HoursPsalmody, PsalmodyEntry
│   └── mod.rs
│
├── liturgical_calendar/             LAYER 1
│   ├── liturgical_day.rs            LiturgicalDay
│   ├── celebration.rs               Celebration, CelebrationId
│   ├── celebration_mass.rs          CelebrationMass
│   ├── celebration_hour.rs          CelebrationHour, CelebrationOfficeReadings, HourTime
│   └── mod.rs
│
├── mass_calendar/                   LAYER 2 MASS
│   ├── mass_composition.rs          MassComposition
│   ├── identity_choice.rs           IdentityChoice
│   ├── readings_choice.rs           ReadingsChoice, ReadingsCategory
│   ├── composition_rules.rs         CompositionRules, BlockRule,
│   │                                ReadingsRule, FlexibleRule
│   └── mod.rs
│
├── hours_calendar/                  LAYER 2 HOURS
│   ├── hours_composition.rs         HoursComposition, HourSuppression
│   ├── hours_celebration_choice.rs  HoursCelebrationChoice, ResolvedHourContent
│   ├── office_readings.rs           OfficeReadingsContent, VigilExtension
│   ├── hours_composition_rules.rs   HoursCompositionRules, MemorialRule
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

### 2. Transformation Pipeline

```
Calendar source files (YAML/JSON input)
        │
        ▼
   CelebrationDef (input type — see Input Data Model)
        │
        ▼
┌───────────────────────────────────────────┐
│  Calendar engine                          │
│                                           │
│  1. Resolve dates                         │
│  1b. Apply permanent Sunday assignments   │
│      (GNLY 7: Epiphany, Ascension,       │
│       Corpus Christi) per calendar config │
│  2. Apply precedence rules (GNLY 59, 60) │
│  2b. Transfer impeded solemnities         │
│      (GNLY 60, Notitiae R14) ¹           │
│  2c. Demote Lenten obligatory memorials   │
│      to optional (GNLY 14, GILH §238) ²  │
│  3. Assemble Celebrations per day         │
│  3b. Generate structural options          │
│      (GNLY 15 Saturday BVM, Martyrology  │
│       saints — GILH 244, GIRM 355.3c)   │
│  4. Resolve liturgical cycle              │
│  5. Populate mass content by GIRM groups  │
│     a. Resolve formulary (collect,        │
│        antiphons) per celebration          │
│     b. Resolve readings per GILM 83       │
│        categories (proper/accommodated/   │
│        common)                             │
│     c. Resolve flexible orations           │
│        (GIRM 363, 364-365)                │
│  6. Populate hours content by GILH rules  │
│     a. Resolve psalmody per rank           │
│        (GILH §134-135)                     │
│     b. Resolve proper elements per         │
│        GILH §235b priority                 │
│     c. Resolve Office of Readings          │
│        content (GILH §235d)                │
│     d. Resolve vigil extension             │
│        (GILH §73) where applicable         │
└──────────┬────────────────────────────────┘
           │
    ┌──────┼──────────────┐
    ▼      ▼              ▼
generate_ generate_     generate_
liturgical mass_         hours_
calendar() calendar()   calendar()
    │      │              │
    ▼      │              │
 Liturgical│              │  Hours transformation:
 Calendar  │              │  • Shift Vespers I to previous civil date
    │      │              │  • Resolve GILH §235 overlay per celebration
    │      │              │  • Apply GILH §239 addition logic
    │      │              │  • Compute HoursCompositionRules
    │      │              ▼
    │      │          Hours Calendar (API output)
    │      │
    │      │  Mass transformation:
    │      │  • Shift evening masses to previous civil date
    │      │  • Assemble IdentityChoice from each Celebration
    │      │  • Assemble ReadingsChoice with source + flags
    │      │  • Explode FlexibleOrations into Vec<SourcedText> ³
    └──────│  • Compute CompositionRules from season/precedence
           ▼
       Mass Calendar (API output)
```

**Step 1b — Permanent Sunday assignments (GNLY 7):** When the Epiphany, Ascension, or Corpus Christi are not observed as Holydays of Obligation, GNLY 7 permanently assigns them to a specific Sunday (Epiphany → Sunday between Jan 2-8; Ascension → 7th Sunday of Easter; Corpus Christi → Sunday after Trinity). This is a per-calendar-configuration rule (controlled by `is_holy_day_of_obligation` at the national level), applied before precedence resolution. It is not a conflict-resolution transfer but a permanent date assignment.

¹ **Transfer of impeded solemnities (step 2b):** When a solemnity is impeded by a higher-ranking celebration on the same date, GNLY 60 requires it to be transferred to the nearest free day. GNLY 5 provides the base rule for privileged Sundays: transfer to the following Monday. Notitiae R14 refines the method: the preceding Saturday should be tried first, before falling back to the Monday or general nearest-free-day rule. Implementation: the engine recalculates the target date within Layer 1 — the transferred `Celebration` retains its original `CelebrationId` and texts but is moved to the new `LiturgicalDay`. The `from_calendar_id` is unchanged. The consumer sees the solemnity on its transferred date in all three output layers (Layer 1, Layer 2 Mass, Layer 2 Hours).

³ **FlexibleOrations explosion:** In Layer 1, `CelebrationMass.flexible_orations` is a single `FlexibleOrations` struct per celebration. In Layer 2 Mass, each oration field (prayer over the offerings, prayer after Communion, preface, etc.) is expanded into a list of alternatives — one entry per available source (feria, Common, other Sundays per GIRM 363 §3). Most orations use `Vec<SourcedText>` (text + provenance); prefaces use `Vec<SourcedPreface>` (enriched `PrefaceText` + provenance, carrying the catalog ID and headline metadata).

² **Lenten demotion (step 2c):** GNLY 14 states: "Obligatory Memorials which fall on weekdays of Lent may only be celebrated as Optional Memorials." This is a rank change (Memorial → OptionalMemorial) that affects both Mass (GIRM 355.1 regime) and Office (GILH §238-239 AdditionsOnly mechanism). Similarly, GILH §238 specifies that obligatory memorials are not celebrated during Advent Dec 17-24 and Christmas Octave.

### 3. Calendar API

```rust
impl Calendar {
    fn generate_liturgical_calendar(&self) -> LiturgicalCalendar;  // Layer 1
    fn generate_mass_calendar(&self) -> MassCalendar;              // Layer 2 Mass
    fn generate_hours_calendar(&self) -> HoursCalendar;            // Layer 2 Hours
}
```

Layer 1 remains the internal foundation. The `Celebration` struct carries both `masses` and `hours` content. Layers 2 Mass and 2 Hours are generated from Layer 1 by their respective transformation pipelines.

This supports both the Roman Office and monastic propers (e.g., Benedictine, Cistercian) through the existing calendar inheritance mechanism — the same `CalendarId` chain that resolves Mass texts also resolves Office texts.

### 4. Combining Hours with Mass (GILH 93-98)

GILH 93-98 provides for combining Lauds with Morning Mass or Vespers with Evening Mass. When combined:

- The shared opening rite replaces both individual ones
- A psalm from the Hour may serve as the entrance chant
- A single concluding rite concludes both

This interaction means the `MassComposition` (Layer 2 Mass) may need a reference to the combined Hour, or a combined output type. This does not affect the current architecture but should be considered when adding Hours support.

### 5. Vigil Extension and Hour Suppression (GILH 73, 206, 209, 211, 212, 215)

Two exceptional mechanisms affect the Office during the Triduum and Christmas. Both involve **cross-domain interactions** — where a Mass celebration affects the Office.

#### A. Vigil Extension (GILH 73, 206, 215)

The Office of Readings can be celebrated in an extended **vigil form** on solemnities, feasts, and Sundays (GILH §73). This is not a separate Hour — it is an extension appended to the Office of Readings:

1. The normal Office of Readings is celebrated (two readings)
2. **Before** the Te Deum, canticles from the special appendix are inserted (typically 3 OT canticles with antiphons)
3. A Gospel is proclaimed (a homily may be added — GILH §73)
4. The Te Deum is then sung (or the corresponding seasonal hymn)

The `VigilExtension` struct (added to `OfficeReadingsContent`) models this appendage. When `vigil_extension` is `Some(...)`, the consumer knows that the vigil form is available for this Office of Readings. The canticles and Gospel are provided; the consumer inserts them before the Te Deum.

**Concrete examples:**

- **Holy Saturday → Easter Vigil:** GILH §212 — the Easter Vigil replaces the Office of Readings entirely (see below for `ReplacedByMass`). But the reduced form for absentees also uses the vigil structure.
- **Christmas night:** GILH §215 — the Office of Readings in vigil form, with the Nativity canticles and Gospel, may be celebrated before Midnight Mass.
- **Pentecost, other solemnities:** When the community chooses the vigil form per GILH §73.

**Example — Vigil form for a Solemnity:**

```
HoursComposition {
    hour_time: OfficeOfReadings,
    ...
    celebration_choices: [
        HoursCelebrationChoice {
            celebration_id: "christmas",
            content: ResolvedHourContent {
                ...
                office_readings: Some(OfficeReadingsContent {
                    scripture_reading: SourcedText { ... },
                    patristic_reading: Some(SourcedText { ... }),
                    hagiographical_reading: None,
                    te_deum: true,           ← said AFTER the vigil extension
                    vigil_extension: Some(VigilExtension {
                        canticles: [         ← 3 OT canticles with antiphons
                            PsalmodyEntry { antiphon: "...", reference: "..." },
                            PsalmodyEntry { antiphon: "...", reference: "..." },
                            PsalmodyEntry { antiphon: "...", reference: "..." },
                        ],
                        gospel: SourcedText { source: ProperOfTime, text: "..." },
                    }),
                }),
                ...
            }
        },
    ],
    ...
}
```

The sequence in the vigil form: Office of Readings (2 readings) → vigil canticles → Gospel → Te Deum → concluding prayer.

#### B. Hour Suppression (GILH 209, 211, 212, 215) and No-Mass Days (PS 59, 75)

On certain exceptional days, attending a Mass celebration makes a subsequent Hour of the Office redundant. The `HourSuppression` enum models these cases.

Additionally, PS §59 and §75 confirm specific constraints for Good Friday and Holy Saturday:

- **Good Friday (PS §59):** "The Church does not celebrate the Eucharist." The Celebration of the Lord's Passion (readings, Great Intercessions, Adoration of the Cross, Communion from the reserved Sacrament) takes place instead. In the data model, this generates a `MassComposition` with `mass_time: CelebrationOfThePassion` and `is_eucharistic: false` — the same structure as a Mass (readings, prayers) but explicitly marked as non-eucharistic. This avoids creating a separate output type for one day per year while ensuring the consumer can distinguish it.
- **Holy Saturday (PS §75):** "The Church abstains strictly from the celebration of the sacrifice of the Mass." No `MassComposition` entries are generated for this civil date until the Easter Vigil, which belongs liturgically to Easter Sunday and is shifted to Saturday evening in Layer 2 Mass (`MassTime::EasterVigil`, `civil_date: Holy Saturday`, `liturgical_date: Easter Sunday`).

**`SuppressedIfAttends` — conditional omission:**

The Hour is provided with full content (for those who do NOT attend the Mass), but is marked as suppressible for those who DO attend.

| Day             | Hour suppressed | Triggered by                                          | Reference |
| --------------- | --------------- | ----------------------------------------------------- | --------- |
| Holy Thursday   | Vespers         | Evening Mass of the Lord's Supper                     | GILH §209 |
| Good Friday     | Vespers         | Celebration of the Lord's Passion                     | GILH §209 |
| Holy Saturday   | Compline        | Easter Vigil                                          | GILH §211 |
| Christmas night | Compline        | Vigil form of Office of Readings before Midnight Mass | GILH §215 |

**`ReplacedByMass` — full replacement:**

The Hour is entirely replaced by a Mass celebration. The `content` in `celebration_choices` carries a reduced form for those who cannot attend the Mass.

| Day           | Hour replaced      | Replaced by  | Reduced form                                                                                                      | Reference |
| ------------- | ------------------ | ------------ | ----------------------------------------------------------------------------------------------------------------- | --------- |
| Holy Saturday | Office of Readings | Easter Vigil | At least 4 readings from the Vigil (recommended: Exodus, Ezekiel, St. Paul, Gospel) + Te Deum + prayer of the day | GILH §212 |

**Example — Holy Saturday:**

```
HoursCalendar["2025-04-19"] → [
    HoursComposition {
        hour_time: OfficeOfReadings,
        ...
        celebration_choices: [
            HoursCelebrationChoice {
                celebration_id: "holy_saturday",
                content: ResolvedHourContent {
                    office_readings: Some(OfficeReadingsContent {
                        scripture_reading: ...,  ← at least 4 Vigil readings (reduced form)
                        patristic_reading: None,
                        hagiographical_reading: None,
                        te_deum: true,
                        vigil_extension: None,
                    }),
                    ...
                }
            },
        ],
        suppression: Some(ReplacedByMass {
            celebration_id: "easter_vigil",
        }),
        ...
    },
    HoursComposition {
        hour_time: Compline,
        ...
        suppression: Some(SuppressedIfAttends {
            celebration_id: "easter_vigil",
        }),
        ...
    },
]
```

**Architectural note:** These cross-domain interactions (Mass → Office) only occur during the Triduum and at Christmas. The `HourSuppression` field is `None` on all other days (approximately 360 out of 365), avoiding unnecessary complexity on ordinary days while representing the exceptional cases.

---

## Appendix — Source References

### GIRM (General Instruction of the Roman Missal)

- **GIRM 53** — Gloria: "sung or said on Sundays outside Advent and Lent, and also on Solemnities and Feasts, and at particular celebrations of a more solemn nature." Basis for the `gloria: bool` field in `MassComposition`.
- **GIRM 68** — Creed (Profession of Faith): "sung or said by the Priest together with the people on Sundays and Solemnities." Basis for the `creed: bool` field in `MassComposition`.
- **GIRM 167** — Solemn blessing and prayer over the people: "The Priest may use one of the formularies of solemn blessing [...] or the prayer over the people." Basis for the `solemn_blessing` and `prayer_over_the_people` fields in `FlexibleOrations`.
- **GIRM 346** — Liturgical colors: white for Easter/Christmas, Lord's feasts, BVM, Angels, non-Martyrs; red for Palm Sunday, Good Friday, Pentecost, Martyrs, Apostles/Evangelists; green for Ordinary Time; violet for Advent/Lent; rose optionally for Gaudete/Laetare Sundays; black optionally for Masses for the Dead. Basis for `ColorInfo`.
- **GIRM 355** — Choice of Mass on optional memorials (by season): five options in OT (355.3), four in certain seasons (355.2), feria imposed on privileged days (355.1). Pastoral caution on preserving weekday readings. Exception: no collect borrowing on Ash Wednesday and Holy Week.
- **GIRM 357** — Choice of readings for memorials: weekday readings unless strictly proper readings exist
- **GIRM 358** — Weekday Lectionary readings: continuous reading scheme; priest may combine omitted readings when interrupted by celebrations
- **GIRM 64** — Sequence: "The Sequence, which is optional except on Easter Sunday and on Pentecost Day, is sung before the _Alleluia_." Five occurrences: Easter (obligatory), Easter Octave (optional), Pentecost (obligatory), Corpus Christi (optional), Our Lady of Sorrows (optional).
- **GIRM 360** — Long and short forms of texts: "a pastoral criterion must be kept in mind"
- **GIRM 361** — Pastoral criteria for choosing between alternative texts; prohibition against permanently excluding Scripture passages
- **GIRM 362** — Adaptations to the _Ordo Lectionum Missae_ by Conferences of Bishops must be observed
- **GIRM 363** — Choice of orations for memorials: collect from proper or Common; prayer over offerings and prayer after Communion flexible. OT weekdays: orations from other Sundays or prayers for various needs also available (§3). Seasonal restriction: during major seasons, proper seasonal orations are already provided (§5).
- **GIRM 364-365** — Preface (purpose and variety) and Eucharistic Prayer choices. EP IV has an invariable preface and may only be used when a Mass has no preface of its own (365.4).
- **GIRM 367** — Norms for chants at entrance, offertory, and Communion (cf. GIRM 48, 87): entrance and communion antiphons may be replaced by other approved chants
- **GIRM 375, 377** — Votive Masses and Masses for Various Needs on OT weekdays with optional memorials (out of scope for this model)
- **GIRM 381** — Masses for the Dead on OT weekdays with optional memorials (out of scope for this model)

### GNLY (General Norms for the Liturgical Year and the Calendar)

- **GNLY 3** — "The liturgical day runs from midnight to midnight. However, the celebration of Sunday and of Solemnities begins already on the evening of the previous day."
- **GNLY 5** — Sundays of Advent, Lent, and Easter take precedence over all Feasts of the Lord and all Solemnities. "Solemnities occurring on these Sundays are transferred to the following Monday unless they occur on Palm Sunday or on Sunday of the Lord's Resurrection." Refined by Notitiae R14 (try preceding Saturday first).
- **GNLY 7** — Permanent Sunday assignments for transferable solemnities: when the Epiphany, Ascension, or Corpus Christi are not Holydays of Obligation, they are assigned to a specific Sunday (Epiphany → Sunday between Jan 2-8; Ascension → 7th Sunday of Easter; Corpus Christi → Sunday after Trinity). Per-calendar-configuration rule (pipeline step 1b).
- **GNLY 10** — "Celebrations, according to the importance assigned to them, are hence distinguished one from another and termed: Solemnity, Feast, Memorial."
- **GNLY 11** — Solemnities begin with First Vespers on the preceding day; some have a proper Vigil Mass "to be used on the evening of the preceding day, if an evening Mass is celebrated." Normative basis for `PreviousEveningMass`.
- **GNLY 13** — "Feasts are celebrated within the limits of the natural day; accordingly, they have no First Vespers (Evening Prayer I), unless they are Feasts of the Lord which fall on a Sunday in Ordinary Time or in the Christmas Season and which replace the Sunday Office."
- **GNLY 14** — "Memorials are either obligatory or optional; their observance is integrated into the celebration of the occurring weekday in accordance with the norms set forth in the _General Instruction of the Roman Missal_ and of the Liturgy of the Hours." Also: (1) "Obligatory Memorials which fall on weekdays of Lent may only be celebrated as Optional Memorials." (2) "If several Optional Memorials are inscribed in the Calendar on the same day, only one may be celebrated, the others being omitted."
- **GNLY 15** — "On Saturdays in Ordinary Time when no Obligatory Memorial occurs, an Optional Memorial of the Blessed Virgin Mary may be celebrated." A standing structural option generated by a general norm, not from a calendar inscription.
- **GNLY 16** — Weekdays definition and precedence rules. 16a: Ash Wednesday and Holy Week weekdays take precedence over all. 16b: Advent Dec 17-24 and Lent weekdays have precedence over Obligatory Memorials. 16c: "Other weekdays [...] are **combined with** Memorials" — the GNLY's own term for the memorial-weekday integration that this document models.
- **GNLY 12** — Octaves: "The Octave of Easter and the Octave of Christmas are governed by their own rules." Only these two octaves remain in the reformed calendar.
- **GNLY 17-21** — Paschal Triduum: from the evening Mass of the Lord's Supper to Vespers of Easter Sunday. The culmination of the liturgical year.
- **GNLY 22-24** — Easter Time: 50 days from Easter Sunday to Pentecost Sunday, celebrated "as one feast day, or better as one 'great Sunday'" (Athanasius).
- **GNLY 24** — "The first eight days of Easter Time constitute the Octave of Easter and are celebrated as Solemnities of the Lord." These are not weekdays — no memorials are celebrated during the Octave.
- **GNLY 28-31** — Lent: from Ash Wednesday to before the Mass of the Lord's Supper on Holy Thursday. Penitential character; violet vestments.
- **GNLY 33-38** — Christmas Time: from First Vespers of Christmas to the Sunday after Epiphany (Baptism of the Lord) inclusive.
- **GNLY 40-42** — Advent: from First Vespers of the First Sunday of Advent to before First Vespers of Christmas. Two phases: eschatological (weeks 1-3) and preparatory for Christmas (Dec 17-24).
- **GNLY 43-44** — Ordinary Time: 33 or 34 weeks. Two runs: (1) Monday after Baptism of the Lord to Ash Wednesday eve; (2) Monday after Pentecost to First Vespers of Advent I.
- **GNLY 45-47** — Rogation Days and Ember Days. GNLY 45-46: the Conferences of Bishops arrange the time, duration, and manner; GNLY 47: the Mass is chosen from the Masses for Various Needs. Not modeled in this document (see Part I §1 Scope note).
- **GNLY 58** — Pastoral transfer to Sunday: "For the pastoral good of the faithful, it is permitted to observe on Sundays in Ordinary Time those celebrations that fall during the week and that are agreeable to the devotion of the faithful, provided the celebrations rank above that Sunday in the Table of Liturgical Days." A consumer-side pastoral option — the engine does not enforce it, but consumers may present this as a choice for Sundays in Ordinary Time.
- **GNLY 59** — Table of Liturgical Days according to Their Order of Precedence. Entry 12: Optional Memorials "may be celebrated, in the special manner described in the _General Instruction_, even on the days listed in no. 9" — the GNLY's authorization for collect-borrowing on privileged weekdays (cf. `BlockRule::ForcedCollectBorrowable`). Entry 12 also: "In the same manner Obligatory Memorials may be celebrated as Optional Memorials if they happen to fall on Lenten weekdays."
- **GNLY 61** — Vespers I/II conflict resolution: "Should Vespers of the current day's Office and First Vespers of the following day be assigned for celebration on the same day, then Vespers of the celebration with the higher rank in the Table of Liturgical Days takes precedence; in cases of equal rank, Vespers of the current day takes precedence."
- **GNLY 60** — Precedence resolution: "If several celebrations fall on the same day, the one that holds the highest rank according to the Table of Liturgical Days is observed." Impeded solemnities are transferred; other celebrations are omitted that year. GNLY 5 provides the base rule: solemnities impeded by a privileged Sunday (Advent, Lent, Easter) are "transferred to the following Monday." **Transfer refinement (Notitiae R14):** the preceding Saturday should be tried first, before falling back to the Monday or general nearest-free-day rule.

### GILM (General Introduction to the Lectionary for Mass, _Ordo Lectionum Missae_)

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

### GILH (General Instruction of the Liturgy of the Hours)

> **Note:** These references document the normative basis for the Office data model (Part II, Part IV §3 and §5) and the Hours transformation pipeline (Part V §2).

- **GILH 133** — Psalter cycle: "The psalter is distributed over a period of four weeks. It is arranged so that very few psalms are omitted." Basis for `PsalterWeekCycle` (4 weeks).
- **GILH 149** — Post-Epiphany readings: special readings between Epiphany and the Baptism of the Lord. Basis for the `PostEpiphany` period variant.
- **GILH 34-36** — The Invitatory: opens the first Hour of the day (normally Office of Readings or Lauds). Consists of an invitatory antiphon + Psalm 95 (or Ps 100, 67, 24). The antiphon varies by celebration and follows the GILH §235b priority on memorials.
- **GILH 37** — Morning Prayer (Lauds) and Evening Prayer (Vespers) are the "double hinge" of the Daily Office: the two principal Hours around which the entire Office is structured
- **GILH 53** — Concluding prayer at Lauds/Vespers: "for weekdays in Ordinary Time is found in the psalter and for other days in the proper." This is the structural rule; GILH §197-200 provide the detailed per-Hour rules.
- **GILH 55-73** — The Office of Readings: hymn, 3 psalms with antiphons, versicle, 1st reading (Scripture) + responsory, 2nd reading (patristic or hagiographical) + responsory, Te Deum (when applicable), concluding prayer. The most complex Hour in terms of content variation.
- **GILH 62** — On memorials, psalms and antiphons are taken from the current week and day of the psalter, unless proper psalms or antiphons are indicated
- **GILH 64, 67** — Office of Readings has two readings: the first from Scripture (continuous reading cycle), the second patristic. On memorials, a hagiographical reading replaces the second reading if one exists
- **GILH 68** — Te Deum: said "on Sundays outside Lent, on days within the octaves of Easter and Christmas, and on solemnities and feasts" after the second reading + responsory; "omitted on memorials and weekdays." The last part (from "Save your people, Lord") may be omitted.
- **GILH 73** — Vigil extension of the Office of Readings: "After the two readings and before the Te Deum canticles should be added from the special appendix [...]. Then the gospel should be read; a homily on the gospel may be added. After this the Te Deum is sung and the prayer said." On solemnities/feasts: Gospel from the Lectionary for Mass; on Sundays: from the series on the paschal mystery in the appendix.
- **GILH 84-92** — Compline (Night Prayer): the most stable Hour. Examination of conscience, hymn, psalm(s) with antiphon, short reading, versicle, Nunc Dimittis canticle + antiphon, concluding prayer, Marian antiphon. Nearly always from the weekday psalter — only varies on solemnities (GILH §230: "everything is said as on Sundays, after evening prayer I and II respectively").
- **GILH 93-98** — Provisions for combining Hours of the Office with Mass: shared opening rite, psalm as entrance chant, single concluding rite
- **GILH 116-119** — Antiphon rules by rank and season. GILH §116: proper antiphons for psalms during Easter Triduum, octaves, and seasons. GILH §117: solemnities and feasts have proper antiphons (if not, from the Common). GILH §118: saints' memorials retain any proper antiphons. GILH §119: antiphons for the Canticles of Zechariah and Mary come from Proper of Seasons (OT) or Proper/Common (solemnities, feasts); on memorials without proper antiphons, "the antiphon may be taken at will either from the common or from the current week."
- **GILH 134-135** — Psalm arrangements by rank. GILH §134: detailed per-Hour assignments on solemnities/feasts — Office of Readings: proper psalms from tradition; Lauds: Sunday of Week I; EP I on solemnities: Laudate Psalms (Ps 113, 117, 135, 146, 147A, 147B); EP II on solemnities and EP on feasts: proper; Daytime Prayer on solemnities: Gradual Psalms (Ps 120-128) or Sunday of Week I; Daytime Prayer on feasts: current psalter. GILH §135: "In all other cases the psalms are taken from the current week and day of the psalter, unless there are proper antiphons or proper psalms."
- **GILH 144** — No Gospel readings in the Liturgy of the Hours, except in the vigil form of the Office of Readings (GILH §73): "Without prejudice to the exception noted in no. 73, there are no readings from the Gospel in the liturgy of the hours, since in the Mass each year the Gospel is read in its entirety." Architectural principle: `VigilExtension.gospel` is the sole Gospel element in the Office data model.
- **GILH 156-158** — Short readings (capitula): chosen to give "clear and concise expression to a theme or an exhortation." Four weekly series for OT, weekly series per season, proper for solemnities/feasts/some memorials, one-week series for Night Prayer. New Testament only at Evening Prayer (GILH §158c).
- **GILH 166-168** — Hagiographical readings definition. GILH §166: "either texts from a Father of the Church or another ecclesiastical writer, referring specifically or rightly applicable to the saint being commemorated, or the readings are texts from the saint's own writings, or are biographical." GILH §167: historical accuracy required, spiritual benefit emphasized. GILH §168: biographical note at the head of the reading is for information only, not read aloud.
- **GILH 175-178** — Daytime Prayer (Terce, Sext, Nones): hymn, 3 psalms with antiphons, short reading, versicle, concluding prayer. Two psalmody schemes: "current" (from the psalter week) and "complementary" (for those who pray all three hours). On memorials, entirely from weekday (GILH §236). On solemnities, may have proper hymn, antiphons, reading, and prayer (GILH §229).
- **GILH 197-200** — Concluding prayer rules per Hour. GILH §197: marks the completion of each Hour; said by priest/deacon in public celebration. GILH §198: Office of Readings = "the prayer proper to the day"; Night Prayer = "always the prayer given in the psalter for that hour." GILH §199: Lauds/Vespers = from the proper on Sundays, seasonal weekdays, solemnities, feasts, and memorials; on OT weekdays = from the psalter. GILH §200: Daytime Prayer = from the proper on Sundays, seasonal weekdays, solemnities, and feasts; on other days = from the psalter, "expressing the character of the particular hour."
- **GILH 206** — Sunday vigils: "The way to celebrate Sunday vigils, as circumstances suggest, has been discussed in no. 73." Cross-reference to GILH §73 for the vigil form of the Office of Readings.
- **GILH 209** — Vespers suppression on Holy Thursday and Good Friday: "Those who take part in the evening Mass of the Lord's Supper or the celebration of the Lord's passion on Good Friday do not say evening prayer on either day."
- **GILH 210** — On Good Friday and Holy Saturday, the Office of Readings "should be celebrated publicly with the people before morning prayer, as far as this is possible." Pastoral directive on timing.
- **GILH 211** — Compline suppression on Holy Saturday: those who attend the Easter Vigil omit Compline.
- **GILH 212** — Easter Vigil replaces the Office of Readings on Holy Saturday. For those absent, a reduced Office of Readings is provided: "at least four of its readings" — recommended Exodus, Ezekiel, St. Paul (NT), and the Gospel (NT) — followed by Te Deum and the prayer of the day. Cross-domain interaction: a Mass celebration affects the Office.
- **GILH 215** — Christmas night: when the vigil form of the Office of Readings is celebrated before Midnight Mass, Compline is omitted. Also specifies the vigil form for Christmas.
- **GILH 225-230** — How the Office is arranged on **solemnities**: everything from Proper or Common (GILH §226-227); psalmody per Hour (EP I: Laudate Psalms, Lauds: Sunday Week I, OdR: proper, Daytime: Gradual Psalms — see GILH §134 and §3a psalmody table); Te Deum said (GILH §228); GILH §228 also: "In the case of a saint with a purely local cult and without special texts even in the local proper, everything is taken from the common"; Daytime Prayer proper (GILH §229); Compline: "as on Sundays, after evening prayer I and II respectively" (GILH §230). Solemnities begin with Vespers I the preceding evening.
- **GILH 231-233** — How the Office is arranged on **feasts**: "celebrated within the limits of the natural day" — no Vespers I (GILH §231), except Lord's Feasts falling on Sunday (GNLY 13); Te Deum said (GILH §231); "At the office of readings, at morning prayer, and at evening prayer, all is done as on solemnities" (GILH §231). Daytime Prayer (GILH §232): weekday hymn (always), weekday psalms/antiphons (rarely proper antiphon from tradition), proper short reading and concluding prayer. Night Prayer: "as on ordinary days" (GILH §233).
- **GILH 234** — No difference in arrangement between obligatory and optional memorials, except on privileged weekdays
- **GILH 235-236** — Memorials during Ordinary Time: (a) psalms/antiphons from current weekday; (b) invitatory, hymn, short reading, canticle antiphons, intercessions from saint's Proper, or else from Common or weekday; (c) concluding prayer from the saint (mandatory); (d) Office of Readings: 1st reading from Scripture cycle, 2nd from saint/Common; Te Deum not said. Daytime Prayer and Night Prayer entirely from weekday (GILH §236).
- **GILH 237-239** — Memorials on privileged weekdays: no memorials on Sundays/solemnities/feasts/Ash Wed/Holy Week/Easter Octave (GILH §237); obligatory memorials become optional on Advent Dec 17-24, Christmas Octave, Lent weekdays (GILH §238); limited additions: hagiographical reading **added after** patristic (not replacing), saint's antiphon and prayer **appended** to Morning/Evening Prayer (GILH §239)
- **GILH 240** — Optional memorial of BVM on Saturdays in Ordinary Time, celebrated as other memorials
- **GILH 244** — On weekdays with optional memorials, a saint from the Roman Martyrology may be celebrated as other memorials (parallels GIRM 355.3c)
- **GILH 247** — Immutability of formularies on privileged days: "In the office for Sundays, solemnities, feasts of the Lord listed in the General Calendar, the weekdays of Lent and Holy Week, the days within the octaves of Easter and Christmas, and the weekdays from 17 to 24 December inclusive, it is never permissible to change the formularies that are proper or adapted to the celebration, such as antiphons, hymns, readings, responsories, prayers, and very often also the psalms." Exception: Sunday psalms may be substituted with psalms from another week.
- **GILH 249** — Interrupted continuous reading in the Office: may combine omitted parts or prefer certain readings (parallels GIRM 358)

### CP (Calendaria Particularia — Instruction on Particular Calendars, 1970)

> **Note:** CP provides norms for revising particular calendars (diocesan, national, religious) and their propers. These norms define the calendar inheritance hierarchy that romcal implements via `CalendarId` chains.

- **CP 2** — Proper of Seasons always takes precedence over particular celebrations. §2a: Sundays — no particular celebration (per se). §2b: Lent, Easter Octave, Dec 17-31 kept free of particular celebrations (exceptions: optional memorials, certain feasts, non-transferable solemnities).
- **CP 3** — One celebration per year per saint. Exception: a second optional memorial for translation of body, conversion, or similar event.
- **CP 8-12** — Rank assignment by calendar level. These norms define the default rank for proper celebrations at each level of the calendar hierarchy — see Part III §6 for the complete rank table.
- **CP 13-16** — Calendar inheritance hierarchy: General Calendar → National/Regional → Diocesan → Local/Church; separately, General Calendar → Religious → Province → House. A particular calendar is "formed by the insertion of particular celebrations into the General Calendar" (§13). Religious members also celebrate the diocese's patron and cathedral dedication (§16d).
- **CP 23** — Precedence conflicts between General and Particular calendars. §23a: General Calendar solemnities always observed on their date. §23b: General Calendar feasts kept; proper feast of same date transferred to nearest free date (unless deeply rooted in local custom). §23c: "A proper memorial is to take precedence over a universal, optional memorial" — may sometimes take precedence over a universal obligatory memorial (by changing the universal to optional or by transferring it).
- **CP 24-26** — Rank flexibility. §24: proper celebrations generally enter as obligatory or optional memorials unless the Table of Liturgical Days specifies otherwise. §25: "The observance of some celebrations in a particular place may have greater solemnity than in the entire diocese or religious institute." §26: Saints listed together must be celebrated together at the same rank.
- **CP 27** — Titles of the saints. Suppressed: "Confessor and Bishop," "Confessor, Nonbishop," "Neither Virgin nor Martyr," "Widow." Retained titles by category: (a) received usage (Apostle/Evangelist, Martyr, Virgin); (b) hierarchical rank (Bishop/Pope, Priest, Deacon); (c) religious institute (Abbot/Monk, Religious). For lay saints in particular calendars: "certain designations that suggest in some way the saints' state in life (e.g., 'King,' 'Father,' 'Mother,' etc.)." Normative basis for `TitleCategory` (Part III §8).
- **CP 28-31** — Patron appointment rules. §28: only saints may be patrons (blessed require apostolic indult); Divine Persons excluded. §29: liturgical celebration only for duly chosen/immemorial patrons. §30: choice requires clergy/people, bishop approval, and Congregation confirmation. §31: "from now on there is to be only one principal patron" — a secondary patron may be added; two saints as principal patron only if listed together in the calendar. Normative basis for `PatronRole` and `Patronage` (Part III §8).
- **CP 40** — Mass proper texts enumeration: entrance antiphon, opening prayer (collect), prayer over the gifts, preface, communion antiphon, prayer after communion, optional solemn blessing. "Only the opening prayer has direct bearing on the saint being celebrated" (§40b).
- **CP 41** — Reading constraints for proper Masses: solemnities require 3 readings; no OT during Easter season; proper responsorial psalm and Gospel acclamation required.
- **CP 43-44** — Office proper texts. §43: hagiographical reading required for every solemnity, feast, and memorial — "usually not more than one hundred twenty words"; biographical note "is not to be read as part of the office." §44: proper elements for solemnities/feasts include invitatory, antiphons (especially Lauds/Vespers), intercessions, hymns. **Critical cross-domain rule:** "The prayer is always the same as the opening prayer of the Mass" — the Office concluding prayer = the Mass collect.

**Notitiae Responses** (Responsa ad dubia clarifying the GNLY):

- **Notitiae R1** (GNLY 7) — When Epiphany, Ascension, or Corpus Christi are transferred to Sunday, the solemnity replaces the Sunday entirely. No Sunday elements are retained, no commemoration.
- **Notitiae R2** (GNLY 13) — The Holy Family and Baptism of the Lord DO have Vespers I when they fall on Sunday. These are Feasts of the Lord on Sundays and thus have Evening Prayer I. (Correction of an initial "No" to "Yes".)
- **Notitiae R3** (GNLY 14) — When several optional memorials fall on the same day, only one may be celebrated. The others are simply omitted — they are not "transferred" or "commemorated." Confirms the `is_optional: true` mechanism and single-choice constraint.
- **Notitiae R5** (GNLY 16c) — Clarifies the integration of memorials with weekdays: the memorial's proper elements overlay the weekday base according to the GIRM/GILH rules for each substitution group. This is the normative basis for the "overlay" mechanism modeled in Part II §4 and the `MemorialRule::FreeChoice` variant.
- **Notitiae R6** (GNLY 34) — The Christmas Vigil Mass is celebrated on the evening of December 24 only (not in the morning). The Midnight Mass must occur around midnight. Normative source for the semantic distinction between `PreviousEveningMass` (vigil, evening) and `NightMass` (midnight).
- **Notitiae R8** (GNLY 40) — No commemorations in Advent or Lent — neither in Mass nor in Office. The reformed liturgy has eliminated the practice of commemorations. Confirms the model needs no commemoration mechanism.
- **Notitiae R10** (GNLY 52) — A co-cathedral's dedication anniversary is celebrated locally only, not throughout the diocese. Only the main cathedral has a diocese-wide celebration.
- **Notitiae R12** (GNLY 58) — Dedication anniversary and patron solemnity may be transferred to an OT or Christmas Time Sunday (for local churches). The cathedral anniversary may NOT be transferred to Sunday (diocese-wide). Exception: the bishop may invoke GIRM 332 for serious pastoral need.
- **Notitiae R14** (GNLY 60) — Method for transferring impeded solemnities: generally to the nearest free day. **But** when a solemnity is impeded by an Advent or Lent Sunday, the preceding Saturday should be tried first (per GNLY 5), before falling back to the general rule.

### PS (Paschalis Sollemnitatis — Circular Letter on the Easter Feasts, 1988)

> **Note:** PS is a **complementary document**. It clarifies and expands on the primary norms (GNLY, GIRM, GILH) for the Lenten and Easter cycles, but does not introduce new calendar calculation rules. Only the paragraphs with direct architectural or data-model relevance are listed here.

- **PS 2** — "The Triduum begins with the evening Mass of the Lord's Supper, reaches its high point in the Easter Vigil, and closes with evening prayer on Easter Sunday." Normative basis for Triduum boundaries.
- **PS 38** — Norms for the Triduum: "During the Sacred Triduum, the practice of decorating the altar with flowers should be observed moderately" and liturgical prescriptions for the Triduum days. General rubrical norms.
- **PS 18** (GNLY 28) — Alleluia is omitted "from the beginning of Lent until the Paschal Vigil" in **all** celebrations, "even on solemnities and feasts." Clarifies that the Lenten acclamation format (GILM 91) applies universally — `ReadingsSet.alleluia` is replaced by a Lenten verse throughout Lent regardless of rank.
- **PS 28-32** — Palm Sunday entrance rite. Three forms: solemn procession (§29), solemn entrance (§30), simple entrance (§30). The procession includes the blessing of palms, its own Gospel reading (the Lord's Entry into Jerusalem), and processional chants (Psalms 23, 46). This pre-Mass rite is modeled via `entrance_gospel: Option<ReadingText>` on `CelebrationMass`. The three forms are a pastoral choice, not modeled.
- **PS 33** — Palm Sunday Passion narrative. The Mass Gospel is the Passion (synoptic of the year), proclaimed in the traditional three-person format (narrator, Christ, people), without candles or incense, and "in its entirety." Two Gospel readings on the same day: the Entry Gospel (procession) and the Passion Gospel (Mass).
- **PS 35-36** — Chrism Mass. Celebrated by the bishop with his presbyterium, traditionally on Holy Thursday morning (transferable to another day close to Easter — §35). Only one celebration per diocese, in the cathedral (§36). Modeled as `MassTime::ChrismMass` — assigned in particular (diocesan) calendars.
- **PS 44-48** — Evening Mass of the Lord's Supper on Holy Thursday. Begins the Paschal Triduum. Modeled as `MassTime::EveningMassOfTheLordsSupper`.
- **PS 59** — Good Friday: "the Church does not celebrate the Eucharist." The Celebration of the Lord's Passion takes place instead. Modeled as `MassTime::CelebrationOfThePassion` with `is_eucharistic: false` — same data structure as a Mass, explicitly marked as non-eucharistic.
- **PS 75** — Holy Saturday: "the Church abstains strictly from the celebration of the sacrifice of the Mass." Holy Communion may only be given as Viaticum. No `MassComposition` is generated for this civil date; the Easter Vigil (belonging liturgically to Easter Sunday) is shifted here in Layer 2 Mass.
- **PS 85** — Easter Vigil readings: 7 Old Testament readings + 2 New Testament (Epistle + Gospel). When pastoral conditions require reducing, at least 3 OT readings must be read, and **Exodus 14 (the crossing of the Red Sea) must never be omitted**. This unique variable-minimum structure is specific to the Easter Vigil's `ReadingsContent`.
- **PS 107** — Pentecost vigil: "prolonged celebration of Mass in the form of a vigil, whose character is not baptismal as in the Easter Vigil, but is one of urgent prayer." Confirms that Pentecost has a vigil Mass form (`PreviousEveningMass`, per GNLY 11).

---

## Conclusion

This document defines a three-layer architecture for liturgical composition:

- **Layer 1 (Liturgical Calendar)** organizes celebrations by liturgical day, carrying both Mass and Office texts in their raw form — what each celebration _provides_.
- **Layer 2 Mass (Mass Calendar)** transforms Layer 1 into a consumer-ready structure organized by civil date and mass time, with pre-resolved options per GIRM substitution group and explicit `CompositionRules`.
- **Layer 2 Hours (Hours Calendar)** transforms Layer 1 into a consumer-ready structure organized by civil date and Hour, with fully resolved content per GILH §235/§239 rules and `HoursCompositionRules`.

The transformation pipeline (Part V §2) handles date-shifting, precedence resolution, cycle resolution, and the assembly of options — so that consumers receive self-contained, pre-resolved options (Mass) and pre-composed content (Hours). The calendar inheritance hierarchy (CP 13-16) and the shared `Celebration` entity ensure that particular calendars, monastic propers, and future extensions plug into the same pipeline without structural changes.
