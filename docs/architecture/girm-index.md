---
title: 'GIRM — Navigation Index'
description: Chapter-by-chapter summary of the General Instruction of the Roman Missal, highlighting sections relevant to romcal's implementation and future development.
---

:::note[Purpose]
This index helps quickly locate the relevant rules in the
[GIRM reference document](/reference/girm) (~2 000 lines, §1–399).
Each chapter is summarized with its key paragraphs and their relevance to
romcal's implementation and future features. Use the § numbers to jump
to the source text.
:::

:::tip[When to use this index]
Consult it when implementing calendar logic, color assignment, Mass text
selection, or rank-based element rules (Gloria, Creed, Sequence). Read the
full GIRM source only when you need exact wording or context beyond what
this index provides.
:::

---

## Quick Lookup — romcal concept → GIRM §§

| romcal concept                       | GIRM §§        | Notes                                             |
| ------------------------------------ | -------------- | ------------------------------------------------- |
| Liturgical colors                    | §346, §347     | Color rules by season, rank, saint category       |
| Gloria (yes/no)                      | §53            | Sundays (not Advent/Lent), solemnities, feasts    |
| Creed (yes/no)                       | §67–68         | Sundays and solemnities only                      |
| Sequence (yes/no)                    | §64            | Obligatory: Easter, Pentecost; optional: 3 others |
| Entrance / Communion antiphons       | §48, §87, §367 | Part of formulary; replaceable by approved chants |
| Choice of Mass (optional memorials)  | §355           | Decision tree by season (355.1 / 355.2 / 355.3)   |
| Readings by rank                     | §357           | 3 readings (sol.), 2 (feast), weekday (mem.)      |
| Continuous reading / interruptions   | §358           | Pastoral provision when _lectio continua_ broken  |
| Responsorial Psalm alternatives      | §359           | Common psalm of the season allowed                |
| Long / short text forms              | §360           | Pastoral criterion                                |
| Orations (collect, flexible prayers) | §363           | Collect identifies celebration; others flexible   |
| Preface choice                       | §364–365       | EP IV has invariable preface (§365.4)             |
| EP inserts (_Communicantes_, etc.)   | §366           | Proper inserts on certain solemnities/feasts      |
| Solemn blessing / prayer over people | §167           | Optional on solemnities and certain feasts        |
| Ritual Masses (when allowed)         | §372–373       | Prohibited on certain days                        |
| Votive Masses (when allowed)         | §375–377       | OT weekdays; forbidden on obligatory mem.         |
| Masses for the Dead (when allowed)   | §380–381       | Funeral: any day except...; daily: OT only        |
| Saturday BVM commemoration           | §378           | Recommended                                       |
| Bishop's pastoral faculty            | §332           | Transfer of celebrations for serious need         |

---

## At a Glance

| Chapter  | §§          | Relevance    | Key topics                                                      |
| -------- | ----------- | ------------ | --------------------------------------------------------------- |
| Preamble | 1–15        | —            | Historical context of the Missal reform                         |
| I        | 16–26       | —            | Theological dignity of the Eucharist                            |
| II       | 27–90       | Medium       | Structure of the Mass: parts, rites, elements                   |
| III      | 91–111      | —            | Ministries (priest, deacon, acolyte, lector)                    |
| IV       | 112–287     | Low          | Ceremonial rubrics; §167 solemn blessing / prayer over people   |
| V        | 288–318     | —            | Church architecture, altar, ambo, tabernacle                    |
| **VI**   | **319–351** | **High**     | **Liturgical colors (§346a–g, §347)**                           |
| **VII**  | **352–367** | **Critical** | **Choice of Mass (§355), texts (§357–363), preface (§364–365)** |
| **VIII** | **368–385** | **High**     | **When Ritual / Votive / Dead Masses are allowed**              |
| IX       | 386–399     | Low          | Adaptations by bishops; §332 episcopal pastoral faculty         |

---

## Preamble (§1–15)

Historical context of the Roman Missal reform after Vatican II.
Not relevant to romcal.

## Chapter I (§16–26) — Importance and Dignity of the Eucharistic Celebration

Theological principles. Not relevant to romcal.

## Chapter II (§27–90) — Structure of the Mass, Its Elements and Parts

Describes the parts of the Mass in sequence. Useful as reference when determining
which liturgical elements (Gloria, Creed, Sequence, antiphons, etc.) apply for
a given rank.

### Introductory Rites (§46–54)

- **§48 — Entrance chant:** The antiphon with its Psalm may be taken from the
  _Graduale Romanum_ or the _Graduale Simplex_, or replaced by another approved
  chant. If no singing, the antiphon in the Missal is recited.
- **§53 — Gloria:** Sung on Sundays outside Advent and Lent, on solemnities,
  feasts, and at more solemn celebrations. **Not** on Advent/Lent weekdays,
  even if a memorial occurs.

### Liturgy of the Word (§55–71)

- §57–60: Biblical readings structure (OT → Apostle → Gospel).
- §61: Responsorial Psalm is an integral part of the Liturgy of the Word.
- **§62–64 — Alleluia / Gospel acclamation:** Rules by season (Alleluia replaced
  by another acclamation during Lent).
- **§64 — Sequence:** Optional except on **Easter Sunday** and **Pentecost**.
  Five sequences exist: _Victimae Paschali_ (Easter, obligatory; Easter Octave,
  optional), _Veni Sancte Spiritus_ (Pentecost, obligatory), _Lauda Sion_
  (Corpus Christi, optional), _Stabat Mater_ (Our Lady of Sorrows, optional).
- **§67–68 — Creed:** Required on **Sundays and solemnities** only.
  Not on feasts, memorials, or weekdays.

### Liturgy of the Eucharist (§72–89)

- §78–79: Eucharistic Prayer elements (Preface, Sanctus, Epiclesis,
  Institution narrative, Anamnesis, Offering, Intercessions, Doxology).
- **§87 — Communion chant:** The antiphon from the _Graduale Romanum_ may be
  used, or another suitable chant approved by the Conference of Bishops.
  Like the entrance antiphon (§48), it may be replaced by approved alternatives.

### Concluding Rites (§90)

Brief announcements, blessing, dismissal.

## Chapter III (§91–111) — Duties and Ministries

Roles of priest, deacon, acolyte, lector, etc. Not modeled in romcal.

## Chapter IV (§112–287) — Different Forms of Celebrating Mass

Detailed ceremonial rubrics for Mass with congregation (§115–198),
concelebrated Mass (§199–251), Mass with one minister (§252–272),
and general norms (§273–287). Mostly not modeled, except:

- **§167 — Solemn blessing and prayer over the people:** On solemnities, certain
  feasts, and other occasions, the priest may use an optional solemn blessing
  (tripartite formula) or a prayer over the people before the final blessing.
  Modeled as optional fields in the Mass output structure.

## Chapter V (§288–318) — Arrangement and Furnishing of Churches

Physical layout of worship space. Not relevant to romcal.

## Chapter VI (§319–351) — Requisites for Celebration

Mostly about bread, wine, vessels, and vestments. The critical section is
**liturgical colors**.

### Liturgical Colors (§346–347)

- **§346 — Color assignment rules** (the primary reference for romcal):
  - **(a) White:** Easter and Christmas seasons; celebrations of the Lord
    (other than his Passion); BVM; Holy Angels; non-Martyr Saints;
    All Saints (Nov 1); Nativity of St John the Baptist (Jun 24);
    St John the Evangelist (Dec 27); Chair of St Peter (Feb 22);
    Conversion of St Paul (Jan 25).
  - **(b) Red:** Palm Sunday; Good Friday; Pentecost Sunday; celebrations of
    the Lord's Passion; feasts of Apostles and Evangelists; Martyr Saints.
  - **(c) Green:** Ordinary Time.
  - **(d) Violet/Purple:** Advent; Lent. Also the **standard** color for
    Masses and Offices for the Dead in contemporary usage.
  - **(e) Black:** Masses for the Dead — "where it is the practice."
    Alternative to purple; local-practice dependent.
  - **(f) Rose:** Gaudete Sunday (Advent III); Laetare Sunday (Lent IV).
  - **(g) Festive vestments:** On more solemn days, "sacred vestments may be
    used that are festive, that is, more precious, even if not of the color
    of the day." Gold is not named in the universal GIRM but comes from
    national adaptations (e.g. US GIRM mentions gold/silver).

- **§347 — Colors for special Mass categories:**
  - Ritual Masses: proper color, or white, or festive.
  - Masses for Various Needs: color of the day/season, or violet if penitential.
  - Votive Masses: color suited to the Mass, or color of the day/season.

## Chapter VII (§352–367) — Choice of Mass and Its Parts

**The most critical chapter for romcal's Mass-selection logic.**

### Choice of Mass (§353–355)

- **§353 — Solemnities:** Priest follows the calendar of the church where he
  celebrates.

- **§354 — Sundays, seasonal weekdays, feasts, obligatory memorials:**
  Follows the local church calendar; if celebrating alone, may follow his own
  proper calendar.

- **§355 — Optional memorials** (the key decision tree):
  1. **Advent Dec 17–24, Octave of Christmas, Lent weekdays** (except Ash Wed
     and Holy Week): Mass of the current liturgical day is **obligatory**;
     the **collect alone** may be taken from the memorial listed that day.
     **Exception within 355.1:** On Ash Wednesday and during Holy Week, even the
     collect may **not** be borrowed — the feria is imposed entirely.
  2. **Advent before Dec 17, Christmas from Jan 2, Easter weekdays:**
     Choose between (a) weekday Mass, (b) Saint's Mass, (c) a memorial Saint,
     (d) any Martyrology Saint.
     **Note:** On Easter weekdays (after the Octave), memorials of Saints may be
     **celebrated fully** — unlike Advent/Christmas/Lent regimes.
  3. **Ordinary Time weekdays:**
     Choose between (a) weekday Mass, (b) optional memorial, (c) Martyrology
     Saint, (d) Mass for Various Needs, (e) Votive Mass.
  - **Particular calendar preference:** When choosing between a General Calendar
    memorial and a particular calendar memorial, **preference goes to the
    particular calendar**, all things being equal.
  - **Pastoral directive:** The priest should take care not to omit the weekday
    readings too frequently; the Church desires "a richer portion at the table
    of God's word" (also emphasized in GILM §83).

### Choice of Mass Texts (§356–367)

- **§357 — Readings by rank:**
  - Sundays and solemnities: **3 readings** (OT/Prophet, Apostle, Gospel).
    During Easter: Acts of the Apostles replaces OT.
  - Feasts: **2 readings** (3 if raised to solemnity rank).
  - Memorials: **weekday readings by default**, unless _strictly proper readings_
    exist (i.e. readings where the saint is named or the mystery is directly
    evoked — see GILM §83 for the precise definition of "proper" vs
    "accommodated" vs "common" readings).

- **§358 — Weekday lectionary / continuous reading:**
  Continuous reading (_lectio continua_) is the norm. When interrupted by a
  solemnity, feast, or particular celebration, the priest may either **combine
  omitted passages** with other readings or **choose which to prefer**,
  considering the entire week's scheme. This is a pastoral provision.

- **§359 — Responsorial Psalm choices:**
  The psalm indicated in the Lectionary for each reading may be replaced by a
  _common psalm_ for the season or by a psalm from the _Graduale Simplex_.
  Relevant for lectionary implementation.

- **§360 — Long and short forms of texts:**
  When a longer and shorter form of the same text exists, "a pastoral criterion
  must be kept in mind." Applies to readings (parallels GILM §75, §80) and to
  any liturgical text with variant forms.

- **§361 — Choosing between alternative texts:**
  Pastoral criteria for choosing between fixed or optional alternatives. Care
  must be taken that "parts of Sacred Scripture are not permanently excluded."

- **§362 — Adaptations to the _Ordo Lectionum Missae_:**
  Adaptations approved by the Conference of Bishops must be observed.

- **§363 — Orations** (three sub-provisions):
  - **General rule (memorials):** Proper collect (or from appropriate Common);
    prayer over offerings and prayer after Communion from Common **or** from the
    weekdays of the current season — these two are **flexible**, unlike the
    collect which identifies the celebration.
  - **§363 ¶3 — OT weekdays:** Besides the previous Sunday's orations, the
    collect from another Sunday in Ordinary Time may be used, or one of the
    prayers for Various Needs. "It is always permissible to use the collect
    alone from these Masses."
  - **§363 ¶5 — Strong seasons:** During Advent, Lent, and Easter, proper
    seasonal orations are already provided for each weekday — no need for
    alternative sources.

- **§364 — Preface** (distinct from §365):
  The many prefaces "bring out more fully the motives for thanksgiving" and
  "set out more clearly the different facets of the mystery of salvation."
  The preface is governed by §364, **not** by §363. Its normative framework
  is separate from the orations.

- **§365 — Eucharistic Prayer preferences:**
  - EP I (Roman Canon): solemnities, Apostle/Saint feasts, Sundays.
  - EP II: weekdays, special circumstances.
  - EP III: Sundays, feast days (preferred over EP I for pastoral reasons).
  - **EP IV (§365.4):** Has an **invariable preface**; may only be used "when a
    Mass has no Preface of its own"; Sundays in OT. No special formula for the
    dead. This constraint limits preface flexibility.

- **§366 — Special inserts in Eucharistic Prayers:**
  Certain solemnities and feasts have proper inserts (_Communicantes_,
  _Hanc igitur_ in EP I; similar adaptations in other EPs). Relevant for
  future Mass text generation.

- **§367 — Chants (entrance, offertory, communion):**
  Cross-references §48 (entrance) and §87 (communion) for the norms allowing
  the Missal antiphons to be replaced by other approved chants. The entrance
  and communion antiphons are part of the formulary block but may be substituted.

## Chapter VIII (§368–385) — Masses for Various Circumstances and for the Dead

Rules on **when** Ritual, Votive, and Masses for the Dead are permitted or
forbidden, by calendar day.

### Masses for Various Circumstances (§368–378)

- **§371:** Four categories: Ritual Masses, Masses for Various Needs,
  Masses for Various Circumstances, Votive Masses.

- **§372 — Ritual Masses prohibited on:**
  Advent/Lent/Easter Sundays, solemnities, Easter Octave, All Souls,
  Ash Wednesday, Holy Week.

- **§373 — Ritual Masses linked to sacraments:**
  Ritual Masses accompany a sacramental rite (baptism, confirmation, marriage,
  ordination, etc.) and follow the rubrics of that rite. Gives context on
  _why_ Ritual Masses exist and how they differ from Votive Masses.

- **§374 — Masses for serious need:** Permitted any day **except**
  solemnities, Advent/Lent/Easter Sundays, Easter Octave, All Souls,
  Ash Wednesday, Holy Week.

- **§375 — Votive Masses:**
  Allowed on OT weekdays even with optional memorial.
  **Not** for mysteries whose celebration is already part of the liturgical year
  (exception: Mass of the Immaculate Conception).

- **§376 — Obligatory memorials + Advent/Christmas/Easter weekdays:**
  Masses for Various Needs and Votive Masses are **forbidden** unless real
  pastoral need justifies it.

- **§377 — OT weekdays (optional memorial or ferial):**
  Any Mass or oration for various circumstances is permitted,
  **except** Ritual Masses (which have their own restrictions in §372).

- **§378:** Saturday BVM commemoration is recommended.

### Masses for the Dead (§379–385)

- **§380 — Funeral Mass:** Any day **except** solemnities that are holy days
  of obligation, Holy Thursday, Easter Triduum, Advent/Lent/Easter Sundays.

- **§381 ¶1 — Mass on news of death / burial / first anniversary:**
  Permitted on Christmas Octave days, obligatory memorials, and weekdays
  (except Ash Wednesday and Holy Week weekdays).

- **§381 ¶2 — Daily Masses for the Dead:**
  Only on OT weekdays with optional memorial or ferial office.

## Chapter IX (§386–399) — Adaptations by Bishops and Conferences

Faculties for local adaptation of rites, gestures, chants, and materials.
Not directly modeled in romcal, but relevant context for understanding
particular calendar variations.

- **§332** (referenced from Chapter IV): The diocesan bishop may invoke this
  provision for serious pastoral need — e.g. transferring the cathedral
  dedication anniversary to a Sunday (see Notitiae R12).

---

## Cross-References to Other Liturgical Documents

The GIRM does not operate in isolation. For a complete picture, these related
documents provide complementary rules:

- **GNLY** ([index](/architecture/gnly-index) · [source](/reference/gnly)):
  §59 Table of Precedence (which celebration wins on a given day);
  §4–16 seasons and their boundaries; §48–61 rank definitions.
- **CP** ([index](/architecture/cp-index) · [source](/reference/cp)):
  Rules for particular calendars (national, diocesan, religious);
  proper celebrations, patron saints, calendar inheritance.
- **GILM** ([index](/architecture/gilm-index) · [source](/reference/gilm)):
  Detailed rules for readings composition (lectionary cycles, proper vs
  accommodated vs common readings for saints, continuous reading rules).
  Complements GIRM §356–362. Key cross-references: GILM §83–84 define
  "strictly proper readings" (_lectiones propriae_) vs "accommodated readings"
  (_lectiones accommodatae_) vs "common readings."
- **GILH** ([index](/architecture/gilh-index) · [source](/reference/gilh)):
  Liturgy of the Hours rules (Office structure, psalmody, antiphons).
  Separate from Mass but shares the same calendar and rank system.
- **PS** ([index](/architecture/ps-index) · [source](/reference/ps)):
  Easter feasts norms (Lent, Holy Week, Triduum, Easter Time).
  PS §18 Lenten acclamation, §85 Easter Vigil readings;
  complements GIRM liturgical color and Mass-selection rules for these seasons.
