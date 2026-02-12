---
title: 'GILM — Navigation Index'
description: Chapter-by-chapter summary of the General Introduction to the Lectionary for Mass, highlighting sections relevant to romcal's implementation and future development.
---

:::note[Purpose]
This index helps quickly locate the relevant rules in the
[GILM reference document](/reference/gilm) (~700 lines, §1–125).
Each chapter is summarized with its key paragraphs and their relevance to
romcal's implementation and future features. Use the § numbers to jump
to the source text.
:::

:::tip[When to use this index]
Consult it when implementing readings composition, the three categories of
readings for saints (proper / accommodated / common), long/short forms,
Common readings pools, responsorial psalm logic, acclamation rules, or
season-by-season readings structure. The GILM is the normative basis for
romcal's `ReadingsContent`, `ReadingsPool`, `ReadingsCategory`, and
`ReadingsSet` types. Read the full GILM source when you need exact wording
or enumerated sub-items.
:::

---

## Quick Lookup — romcal concept → GILM §§

| romcal concept                          | GILM §§       | Notes                                                                     |
| --------------------------------------- | ------------- | ------------------------------------------------------------------------- |
| Sunday 3-year readings cycle            | §66           | Year A / B / C; 3 readings per Mass                                       |
| Weekday 2-year readings cycle           | §69           | Year 1 (odd) / Year 2 (even); 2 readings per Mass                         |
| Readings for saints — two series        | §70           | Proper of Saints + Commons of Saints                                      |
| Commons ordering + component choice     | §71           | OT → Apostle → psalms → Gospels; "celebrant may choose at will"           |
| Three categories of readings for saints | §83           | Proper (obligatory), accommodated (facultative), common (freely chosen)   |
| Rules by rank (readings count)          | §84           | Solemnities: 3 readings; feasts/memorials: 2                              |
| Reservation of books to seasons         | §74           | Acts in Easter, Isaiah in Advent, John in Lent/Easter                     |
| Long and short forms of texts           | §75, §80      | "Longer and shorter versions"; pastoral criterion for choice              |
| Difficult texts (omission rationale)    | §76–77        | Pastoral omissions; verse omission tradition                              |
| Freedom of choice — general principle   | §78           | Rare on Sundays/solemnities; frequent for saints/ritual/votive            |
| Two readings before the Gospel          | §79           | When conference permits 2 instead of 3; harmony criterion                 |
| When two texts are provided             | §81           | Best interest of congregation; avoid near-repetition                      |
| Weekday readings on assigned days       | §82           | Default: use assigned day; skip for proper-readings celebrations          |
| Default preference for weekday readings | §83 (end)     | "Not to omit too often… the weekday readings"                             |
| Common of Men and Women Saints fallback | §83 (end)     | Always available as alternative, regardless of saint's class              |
| Psalm follows first reading             | §89           | For Commons: choice left to priest; seasonal alternatives when sung       |
| Acclamation before the Gospel           | §90           | Specified (correlated with Gospel) or chosen from season/Commons          |
| Lenten acclamation                      | §91           | Replaces Alleluia during Lent                                             |
| Advent readings structure               | §93–94        | Sundays: Messianic prophecies; weekdays: two series (before/after Dec 16) |
| Christmas Time readings                 | §95–96        | Solemnities/feasts/Sundays + weekday 1 John continuous reading            |
| Lent readings structure                 | §97–98        | Baptismal/penitential catechesis; John semicontinuous from Week 4         |
| Easter Triduum readings                 | §99           | Exodus, Servant Songs, Vigil 7 OT readings                                |
| Easter season readings                  | §100–102      | Acts replaces OT; 1 Pet / 1 John / Rev by year                            |
| OT week numbering (33 vs 34 weeks)      | §104          | Resumption rules after Pentecost; eschatological weeks preserved          |
| OT Sunday readings — Gospel harmony     | §105–106      | Semicontinuous Synoptics; OT harmonized with Gospel                       |
| OT Apostle readings                     | §107          | Semicontinuous Paul + James; Hebrews split across B/C                     |
| Solemnities of the Lord in OT           | §108          | Trinity, Corpus Christi, Sacred Heart, Christ the King                    |
| OT weekday Gospel arrangement           | §109          | Mark (Wk 1–9), Matthew (Wk 10–21), Luke (Wk 22–34)                        |
| OT weekday first reading                | §110          | Alternating OT/NT blocks; nearly all OT books included                    |
| Ritual / votive / dead Masses readings  | §72, §85–88   | Same component-choice logic as Commons                                    |
| `ReadingsContent` enum                  | §83, §71      | Fixed set (proper/weekday) vs pool (Commons)                              |
| `ReadingsPool` type                     | §71, §89      | Pool-per-component: OT, Apostle, psalm, acclamation, Gospel               |
| `ReadingsCategory` enum                 | §83           | Proper / Accommodated / Common — binding force per category               |
| `ReadingsSet` type                      | §66, §69, §84 | Number of readings by rank and cycle                                      |
| `TextVariant` (long/short)              | §75, §80      | Pastoral criterion; printed separately                                    |

---

## At a Glance

| Part / Chapter | §§         | Relevance    | Key topics                                                                     |
| -------------- | ---------- | ------------ | ------------------------------------------------------------------------------ |
| Preamble / I   | 1–10       | —            | Theology of the Word in liturgy                                                |
| II             | 11–37      | Low          | Elements and rites of the Liturgy of the Word; ambo, books                     |
| III            | 38–57      | —            | Offices and ministries (president, reader, psalmist, commentator)              |
| **IV.1**       | **58–63**  | **Low**      | **Pastoral purpose of the Order of Readings**                                  |
| **IV.2**       | **64–77**  | **Critical** | **Composition principles: cycles, saints' readings, long/short, omissions**    |
| **IV.3**       | **78–91**  | **Critical** | **Use principles: choice freedom, §83 three categories, psalms, acclamations** |
| **V**          | **92–110** | **High**     | **Season-by-season readings description; OT week numbering (§104)**            |
| VI             | 111–125    | —            | Adaptations, translations, format of individual readings                       |

---

## Preamble — Chapter I (§1–10): General Principles

Theological foundation for the Liturgy of the Word. Not directly modeled
in romcal, but provides context for why readings composition matters.

- **§1:** Importance of the Word of God in liturgical celebration.
- **§2:** Terminology for "Word of God" in different contexts.
- **§3–6:** Proper character of the Word in liturgy; economy of salvation;
  faithful's participation.
- **§7–10:** Word of God in the Church's life; explanation; Holy Spirit;
  bond with the Eucharist.

## Chapter II (§11–37): Celebration of the Liturgy of the Word at Mass

Elements, rites, and aids for celebrating the Liturgy of the Word.

- **§12–18 — Biblical Readings:** Number and importance of readings;
  Gospel as summit; reading from the ambo.
- **§19–22 — Responsorial Psalm:** Integral part of the Liturgy of the
  Word; importance of the cantor/psalmist.
- **§23 — Acclamation before the Gospel:** Assembly greets the Lord
  speaking in the Gospel.
- **§24–27 — Homily:** Required on Sundays and holydays; recommended
  on weekdays (especially Lent).
- **§28 — Silence:** Meditation after readings.
- **§29 — Profession of Faith:** Creed recited when prescribed.
- **§30–31 — Universal Prayer:** Concludes the Liturgy of the Word.
- **§32–37 — Aids:** Ambo placement, lectionary books, dignity of the
  book of Gospels.

## Chapter III (§38–57): Offices and Ministries

Not relevant to romcal. Covers the president's role, faithful's
participation, and specific ministries (reader, psalmist, commentator).

## Chapter IV (§58–91): General Arrangement of Readings

**The most critical chapter for romcal's readings engine.**

### 1. Pastoral Purpose (§58–63)

- **§58–59:** Design goals of the Order of Readings: pastoral purpose,
  accord with liturgical tradition.
- **§60:** Knowledge of the whole of God's Word across the liturgical year.
- **§61:** Pedagogical and catechetical dimension.
- **§62:** Unity across assemblies — same readings everywhere on the same day.
- **§63:** Pastoral freedom within the Order of Readings; faculties for
  particular groups.

### 2. Composition Principles (§64–77)

This section defines the structural rules that romcal implements in
`ReadingsSet`, `ReadingsContent`, and `ReadingsPool`.

- **§64:** Hermeneutical principles underlying the composition.

- **§65 — Two independent series:** Sundays/festive days and weekdays
  run independently. Saints/ritual/votive/dead governed by their own rules.

- **§66 — Sunday/festive readings (3-year cycle):**
  - **(1)** Three readings: OT, Apostle, Gospel.
  - **(2)** Three-year cycle (same texts every 4th year).
  - **(3)** Principles of "harmony" and "semicontinuous reading."

  Mapped to romcal's `SundayCycle` (A/B/C) and the 3-reading structure.

- **§67 — Harmony principle:** OT chosen to correlate with Gospel (explicit
  in Advent/Lent/Easter; on OT Sundays, OT harmonized, Apostle semicontinuous).

- **§68:** No thematic harmony imposed on OT Sundays — liturgy is
  celebration of mystery, not catechetical outline.

- **§69 — Weekday readings (2-year cycle):**
  - **(1)** Two readings: OT or Apostle + Gospel.
  - **(2)** Lent cycle is yearly (baptismal/penitential).
  - **(3)** Advent, Christmas, Easter cycles are yearly.
  - **(4)** OT: single Gospel cycle; first reading in 2-year cycle
    (Year 1 = odd, Year 2 = even).

  Mapped to romcal's `WeekdayCycle` (1/2).

- **§70 — Two series for saints:**
  - **(1)** Proper of Saints: for solemnities, feasts, memorials with
    proper or semi-proper texts.
  - **(2)** Commons of Saints: extensive group for classes of saints
    (martyrs, pastors, virgins…) or saints in general.

- **§71 — Commons ordering and component-level choice:**
  OT → Apostle → psalms → Gospels. "The celebrant may choose at will
  from such texts." This is the normative basis for romcal's
  `ReadingsPool` (pool-per-component architecture).

- **§72:** Ritual, votive, and Masses for the dead follow the same
  component-choice logic as Commons.

- **§73:** General criteria introduction.

- **§74 — Reservation of books to seasons:**
  Acts in Easter, Isaiah in Advent, John in late Lent/Easter. Relevant
  for season-aware readings validation.

- **§75 — Long and short forms:**
  "Longer and shorter versions are provided to suit different situations."
  Mapped to romcal's `TextVariant::Long` / `TextVariant::Short`.

- **§76 — Difficult texts:** Avoided on Sundays/solemnities for pastoral
  reasons; clarified by correlation with other readings.

- **§77 — Verse omissions:** Traditional practice continued; essential
  meaning preserved.

### 3. Use Principles (§78–91)

This section defines the **freedom-of-choice** rules that romcal must
model in `ReadingsCategory` and the readings resolution engine.

- **§78 — Freedom of choice:**
  Rare on Sundays/solemnities/feasts (to preserve seasonal character).
  Frequent for saints, ritual, votive, and Masses for the dead.

- **§79 — Two readings before the Gospel:**
  When a conference permits 2 instead of 3, choose the one more in
  harmony with the Gospel.

- **§80 — Long/short form choice:**
  "A pastoral criterion must also guide the choice." Capacity of hearers
  is the main consideration. Complements §75.

- **§81 — When two texts are provided:**
  Best interest of congregation; avoid near-repetition within days.

- **§82 — Weekday readings default:**
  "Used on their assigned days, unless a solemnity, a feast, or else a
  memorial with proper readings occurs." The celebrant may rearrange
  the week to avoid omitting significant passages.

- **§83 — Three categories of readings for saints** _(the key paragraph)_:

  | Category         | Definition                                          | Binding force                                   |
  | ---------------- | --------------------------------------------------- | ----------------------------------------------- |
  | **Proper**       | Biblical text names the saint or evokes the mystery | **Obligatory** — must replace weekday readings  |
  | **Accommodated** | Lectionary suggests texts for the saint             | **Facultative** — not binding except pastorally |
  | **Common**       | Lectionary refers to a Common                       | **Freely chosen** from any applicable Common    |

  Additional rules in §83:
  - "Not to omit too often… the weekday readings" — preference for feria
    _lectio continua_.
  - "Readings may be taken… also from the Common of Men and Women Saints"
    — universal fallback regardless of saint's class.

  Mapped to romcal's `ReadingsCategory` enum (Proper / Accommodated / Common).

- **§84 — Rules by rank:**
  - **(a)** Solemnities and feasts: readings from Proper or Commons.
  - **(b)** Particular-calendar solemnities: 3 readings (unless conference
    permits 2). OT or Acts/Revelation; Apostle; Gospel.
  - **(c)** Feasts and memorials: 2 readings. First from OT or Apostle
    (Easter: from Apostle); second from Gospels.

  Mapped to romcal's `ReadingsSet` structure (3 vs 2 readings).

- **§85–88 — Other parts:**
  Ritual Masses (§85), Masses for various needs (§86), choice criteria
  (§87), and when ritual Mass not permitted (§88). Same component-choice
  logic as Commons.

- **§89 — Psalm independence in Commons:**
  "In the case of readings for the Common of Saints… the choice is left
  up to the priest." Alternative psalms per season/class may replace the
  assigned psalm when sung. Mapped to `ReadingsPool.psalms`.

- **§90 — Acclamation before the Gospel:**
  Either specified (correlated with Gospel) or left as a choice from
  the season or Commons. Mapped to `ReadingsPool.alleluia`.

- **§91 — Lenten acclamation:**
  Specific format replaces the Alleluia during Lent. Applies universally
  regardless of rank (cf. PS §18).

## Chapter V (§92–110): Description of the Order of Readings

**Season-by-season detail** for the readings arrangement. High relevance
for romcal's season-specific readings logic.

- **§92:** Introduction — description aids pastors in understanding the
  Order of Readings structure.

### 1. Advent (§93–94)

- **§93 — Sundays:** Distinctive Gospel themes per Sunday (eschatology,
  John the Baptist, Nativity preparation). OT: Messianic prophecies
  (especially Isaiah). Apostle: exhortations fitting Advent.
- **§94 — Weekdays:** Two series split at Dec 16/17.
  First part: Isaiah + Gospel correlated. Last week: Matthew 1 / Luke 1
  (Nativity preparation) + OT Messianic prophecies.

### 2. Christmas Season (§95–96)

- **§95 — Solemnities/feasts/Sundays:** Vigil + 3 Christmas Masses
  (Roman tradition). Holy Family, Mary Mother of God, 2nd Sunday
  (Incarnation), Epiphany, Baptism of the Lord.
- **§96 — Weekdays:** From Dec 29: continuous 1 John (begins Dec 27–28
  with St. John Evangelist / Holy Innocents). Gospels: childhood events
  (Luke), John 1, other manifestations.

  Referenced in romcal's `cycles.rs` for Christmas Time cycle rules.

### 3. Lent (§97–98)

- **§97 — Sundays:** Temptation + Transfiguration (Sundays 1–2).
  Sundays 3–5 Year A: Samaritan woman, man born blind, Lazarus
  (initiation Gospels, usable in B/C). Palm Sunday: procession + Passion.
  OT: salvation history. Apostle: connects OT and Gospel.
- **§98 — Weekdays:** Gospels and OT correlated. John semicontinuous
  from Monday of Week 4. Optional Masses for initiation Gospels
  (Weeks 3–5). Holy Week: Passion mystery + Chrism Mass.

### 4. Sacred Triduum and Easter Season (§99–102)

- **§99 — Triduum:** Holy Thursday (Exodus meal, foot-washing, Paul on
  Eucharist). Good Friday (John's Passion, Isaiah Servant Songs).
  Easter Vigil (7 OT readings, Synoptic Resurrection, Paul on baptism).
  Easter Day (John: empty tomb; option: Vigil Gospels or Luke: Emmaus).
- **§100 — Easter Sundays:** Appearance accounts (1–3), Good Shepherd (4),
  Last Supper discourse (5–7). Acts in 3-year parallel cycle. Apostle:
  1 Peter (A), 1 John (B), Revelation (C).
- **§101 — Easter weekdays:** Acts semicontinuous + John paschal reading.
- **§102 — Ascension and Pentecost:** Ascension: Acts account +
  exaltation + Synoptic variants. Pentecost Vigil: 4 OT options.
  Pentecost Day: Acts + Paul on Spirit + John on Spirit.

### 5. Ordinary Time (§103–110)

- **§103 — Boundaries:** Monday after Sunday following Jan 6 → Tuesday
  before Lent; Monday after Pentecost → before Advent I Vespers.
  34 Sundays provided, sometimes only 33 used.

- **§104 — Week numbering and resumption rules** _(referenced in romcal's
  `cycles.rs`)_:
  - **(1)** Baptism of the Lord replaces OT Sunday 1; Week 1 readings
    begin on Monday after the Sunday following Jan 6.
  - **(2)** Consecutive numbering up to Lent; Ash Wednesday week interrupted.
  - **(3)** Resumption after Pentecost: 34 weeks → resume from last
    pre-Lent week; 33 weeks → skip one week to preserve eschatological
    ending.

  Critical for romcal's OT week calculation engine.

- **§105 — Sunday Gospels:** Semicontinuous Synoptics from Sunday 3.
  John 6 inserted in Year B after Sunday 16. Luke introduction prefixed
  in Year C.
- **§106 — Sunday OT readings:** Harmonized with Gospel; major OT pages
  distributed across Sundays.
- **§107 — Sunday Apostle readings:** Semicontinuous Paul + James.
  1 Corinthians spread across 3 years. Hebrews split: B + C.
- **§108 — Solemnities of the Lord in OT:** Trinity, Corpus Christi,
  Sacred Heart, Christ the King.
- **§109 — Weekday Gospels:** Mark (Wk 1–9), Matthew (Wk 10–21),
  Luke (Wk 22–34). Mark 1–12 complete; Matthew/Luke complement Mark.
- **§110 — Weekday first reading:** Alternating OT/NT blocks. Nearly all
  OT books included (omissions: Obadiah, Zephaniah, Song of Songs).
  End of year: Daniel + Revelation (eschatological).

## Chapter VI (§111–125): Adaptations, Translations, Format

Not relevant to romcal. Covers translation norms, volume division,
typography, biblical references, headings, incipits, and final
acclamation.

---

## Cross-References to Other Liturgical Documents

The GILM does not operate in isolation. For a complete picture, these
related documents provide complementary rules:

- **GNLY** ([index](/architecture/gnly-index) · [source](/reference/gnly)):
  Calendar structure, seasons, ranks, and cycles. The GILM relies on
  GNLY ranks and seasons to determine which readings apply.
- **GIRM** ([index](/architecture/girm-index) · [source](/reference/girm)):
  §355 choice of Mass (optional memorials), §357–358 readings by rank
  and continuous reading, §359 common psalm alternatives, §360 long/short
  forms. GIRM §357 articulates with GILM §83 for readings on memorials.
- **CP** ([index](/architecture/cp-index) · [source](/reference/cp)):
  §41 readings constraints for particular calendar solemnities (3 readings,
  no OT in Easter, proper responsorial psalm). Aligns with GILM §84.
- **GILH** ([index](/architecture/gilh-index) · [source](/reference/gilh)):
  Office of Readings has its own readings structure (GILH §64, §67)
  distinct from Mass readings. The GILM governs Mass only.
- **PS** ([index](/architecture/ps-index) · [source](/reference/ps)):
  §18 Lenten acclamation applies universally (clarifies GILM §91);
  §85 Easter Vigil readings with variable selection.
