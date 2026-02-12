---
title: 'GNLY — Navigation Index'
description: Chapter-by-chapter summary of the Universal Norms on the Liturgical Year and the Calendar, highlighting sections relevant to romcal's implementation and future development.
---

:::note[Purpose]
This index helps quickly locate the relevant rules in the
[GNLY reference document](/reference/gnly) (~400 lines, §1–61).
Each section is summarized with its key paragraphs and their relevance to
romcal's implementation and future features. Use the § numbers to jump
to the source text.
:::

:::tip[When to use this index]
Consult it when implementing season boundaries, rank definitions, precedence
logic, calendar composition (general vs. particular), transfer rules, or
movable feast assignment. The GNLY is romcal's **primary normative source**
for calendar structure — nearly every paragraph is relevant. Read the full
GNLY source when you need exact wording or enumerated sub-items.
:::

---

## Quick Lookup — romcal concept → GNLY §§

| romcal concept                         | GNLY §§   | Notes                                                                                              |
| -------------------------------------- | --------- | -------------------------------------------------------------------------------------------------- |
| Season boundaries (all)                | §18–44    | Triduum (§18–21), Easter (§22–26), Lent (§27–31), Christmas (§32–38), Advent (§39–42), OT (§43–44) |
| Advent start / end                     | §40       | 1st Vespers of nearest Sunday to Nov 30 → 1st Vespers of Christmas                                 |
| Advent Sundays                         | §41       | 1st, 2nd, 3rd, 4th Sundays of Advent                                                               |
| Dec 17–24 privileged weekdays          | §42       | Elevated precedence (level 9 in §59)                                                               |
| Christmas Time boundaries              | §33       | 1st Vespers of Nativity → Sunday after Epiphany inclusive                                          |
| Christmas Octave / feasts              | §12, §35  | Octave days; Holy Family, Stephen, John, Innocents, Mary Mother                                    |
| Lent boundaries                        | §28       | Ash Wednesday → Mass of the Lord's Supper exclusive                                                |
| Lent Sundays                           | §30       | 1st–5th Sundays of Lent; 6th = Palm Sunday                                                         |
| Holy Week                              | §16a, §31 | Ash Wednesday + Mon–Thu of Holy Week: highest weekday rank                                         |
| Triduum boundaries                     | §19       | Evening Mass of the Lord's Supper → Vespers of Easter Sunday                                       |
| Easter Time boundaries                 | §22       | Easter Sunday → Pentecost (50 days)                                                                |
| Easter Octave                          | §12, §24  | 8 days celebrated as Solemnities of the Lord                                                       |
| Ascension (movable)                    | §7b, §25  | 40th day or 7th Sunday of Easter                                                                   |
| Ordinary Time                          | §43–44    | Two runs: post-Baptism of the Lord → Lent; post-Pentecost → Advent                                 |
| Sunday precedence                      | §4–6      | Primordial feast; yields only to solemnities/Lord's feasts                                         |
| Movable solemnities to Sunday          | §7        | Epiphany (§7a), Ascension (§7b), Corpus Christi (§7c)                                              |
| Fixed Sunday celebrations              | §6        | Holy Family, Baptism, Trinity, Christ the King                                                     |
| Rank: Solemnity                        | §10–11    | Begins at 1st Vespers; some with Vigil Mass                                                        |
| Rank: Feast                            | §10, §13  | Within the natural day; no 1st Vespers (exception: Lord's on Sunday)                               |
| Rank: Memorial (obligatory / optional) | §10, §14  | Integrated into weekday; obligatory in Lent → optional                                             |
| Rank: Weekday                          | §16       | 3 tiers (a–c) by season                                                                            |
| Saturday BVM memorial                  | §15       | OT Saturdays when no obligatory memorial occurs                                                    |
| Octave rules                           | §12       | Only Easter and Christmas have octaves                                                             |
| Rogation / Ember Days                  | §45–47    | Time/manner left to Conferences of Bishops                                                         |
| General vs. particular calendar        | §48–49    | General = Roman Rite; particular = diocese / religious                                             |
| Particular calendar composition        | §50–55    | Patrons, dedications, founders; avoid overburdening (§53)                                          |
| Proper day for celebrations            | §56–58    | Birthday rule, impediment resolution, Lent-free dates                                              |
| Table of Precedence                    | §59       | 13 levels — romcal's core precedence engine                                                        |
| Transfer of impeded solemnities        | §60       | Closest free day (levels 1–8); Annunciation special rule                                           |
| Vespers conflict resolution            | §61       | Higher rank wins; equal rank → current day                                                         |
| Alleluia suppression (Lent)            | §28       | From Lent start until Paschal Vigil                                                                |

---

## At a Glance

| Section             | §§        | Relevance    | Key topics                                                                     |
| ------------------- | --------- | ------------ | ------------------------------------------------------------------------------ |
| Preamble            | 1–2       | —            | Theological frame; applicability to the Roman Rite                             |
| **Title I.I**       | **3**     | **Low**      | **Liturgical day = midnight to midnight; Sunday/solemnity from evening prior** |
| **Title I.II**      | **4–7**   | **Critical** | **Sunday precedence, movable solemnities**                                     |
| **Title I.III**     | **8–15**  | **Critical** | **Rank definitions: solemnity, feast, memorial**                               |
| **Title I.IV**      | **16**    | **High**     | **Weekday tiers by season**                                                    |
| **Title II.I**      | **18–21** | **Critical** | **Triduum boundaries and Easter Vigil**                                        |
| **Title II.II**     | **22–26** | **Critical** | **Easter Time, Octave, Ascension**                                             |
| **Title II.III**    | **27–31** | **Critical** | **Lent boundaries, Ash Wednesday, Palm Sunday**                                |
| **Title II.IV**     | **32–38** | **Critical** | **Christmas Time, Octave, Epiphany, Baptism of the Lord**                      |
| **Title II.V**      | **39–42** | **Critical** | **Advent boundaries, Dec 17–24**                                               |
| **Title II.VI**     | **43–44** | **High**     | **Ordinary Time: two runs, 33–34 weeks**                                       |
| Title II.VII        | 45–47     | Low          | Rogation/Ember Days (national calendar concern)                                |
| **Ch. II Title I**  | **48–55** | **High**     | **General vs. particular calendar; proper celebrations**                       |
| **Ch. II Title II** | **56–58** | **High**     | **Proper day for celebrations; impediment rules**                              |
| **Ch. II §59**      | **59**    | **Critical** | **Table of Precedence (13 levels)**                                            |
| **Ch. II §60–61**   | **60–61** | **Critical** | **Transfer rules; Vespers conflict**                                           |

---

## Chapter I — The Liturgical Year (§1–47)

### Preamble (§1–2)

Theological introduction. §2 limits practical norms to the Roman Rite.
Not directly relevant to romcal.

### Title I — The Liturgical Days

#### I. The Liturgical Day in General (§3)

- **§3:** The liturgical day runs **midnight to midnight**. However, the
  celebration of **Sunday and Solemnities begins on the evening of the
  previous day** (First Vespers / Vigil Mass). This is the normative basis
  for romcal's `has_first_vespers` and vigil Mass logic.

#### II. Sunday (§4–7)

- **§4:** Sunday is the "primordial feast day" — the weekly Easter.
- **§5 — Sunday precedence:** Sunday yields only to solemnities and
  Lord's feasts. **Advent, Lent, and Easter Sundays** have precedence
  over all Lord's feasts and all solemnities (solemnities on these Sundays
  are transferred to Monday, except Palm Sunday and Easter Sunday).
- **§6 — Fixed Sunday celebrations:** Four celebrations permanently replace
  a Sunday:
  - (a) Holy Family (Sunday within Christmas Octave)
  - (b) Baptism of the Lord (Sunday after Jan 6)
  - (c) Most Holy Trinity (Sunday after Pentecost)
  - (d) Christ the King (Last Sunday in OT)
- **§7 — Movable solemnities to Sunday** (when not a holyday of obligation):
  - (a) Epiphany → Sunday between Jan 2–8
  - (b) Ascension → 7th Sunday of Easter
  - (c) Corpus Christi → Sunday after Trinity

  Implemented in romcal via the `epiphany_on_sunday` / `ascension_on_sunday`
  / `corpus_christi_on_sunday` configuration flags.

#### III. Solemnities, Feasts, and Memorials (§8–15)

- **§10 — Rank taxonomy:** Celebrations are distinguished as **Solemnity**,
  **Feast**, or **Memorial**. This is the normative basis for romcal's
  `Rank` enum.
- **§11 — Solemnities:** Begin with First Vespers (Evening Prayer I) on the
  preceding day. Some have a proper Vigil Mass for the evening before.
  Basis for `has_first_vespers: true` and `vigil_mass: Option<...>`.
- **§12 — Octaves:** Only Easter and Christmas have octaves. Easter Octave =
  solemnities of the Lord (§24). Christmas Octave = specific arrangement (§35).
- **§13 — Feasts:** Celebrated within the **natural day** (no First Vespers).
  Exception: Lord's feasts on Sundays in OT/Christmas Time replace the Sunday
  Office (and thus have First Vespers). Basis for the conditional
  `has_first_vespers` logic on feasts.
- **§14 — Memorials:**
  - Obligatory or optional.
  - Integrated into the weekday celebration (GIRM and GILH memorial overlay rules).
  - **Obligatory memorials on Lenten weekdays → optional.** Implemented in
    romcal's precedence engine.
  - Multiple optional memorials on the same day → only one celebrated.
- **§15 — Saturday BVM:** On OT Saturdays when no obligatory memorial occurs,
  an optional memorial of the Blessed Virgin Mary may be celebrated.

#### IV. Weekdays (§16)

- **§16 — Three weekday tiers:**
  - **(a)** Ash Wednesday + Holy Week Mon–Thu: precedence over all other celebrations.
  - **(b)** Advent Dec 17–24 + all Lent weekdays: precedence over obligatory memorials.
  - **(c)** Other weekdays: yield to all solemnities and feasts; combined with memorials.

  This maps directly to precedence levels 2, 9, and 13 in the Table of Precedence (§59).

### Title II — The Cycle of the Year (§17–47)

#### I. The Paschal Triduum (§18–21)

- **§18:** The Triduum is "the high point of the entire liturgical year."
  Romcal models it as a distinct `Period` (not a `Season`).
- **§19 — Triduum boundaries:** Begins with the **evening Mass of the Lord's
  Supper**, has its center in the **Easter Vigil**, and closes with **Vespers
  of Easter Sunday**.
- **§20:** Paschal fast on Good Friday (and optionally Holy Saturday until the
  Easter Vigil).
- **§21 — Easter Vigil:** "Mother of all holy Vigils." Must take place
  **at night** (after nightfall, before dawn on Sunday).

#### II. Easter Time (§22–26)

- **§22 — Easter Time boundaries:** 50 days, Easter Sunday → Pentecost.
  Celebrated as one "great Sunday." Alleluia sung throughout.
- **§23:** Sundays named 2nd–7th Sundays of Easter. Pentecost concludes
  the period.
- **§24 — Easter Octave:** First 8 days = Solemnities of the Lord.
- **§25 — Ascension:** 40th day after Easter, or 7th Sunday of Easter
  where not a holyday of obligation (cf. §7).
- **§26:** Weekdays from Ascension to Saturday before Pentecost prepare
  for the Holy Spirit.

#### III. Lent (§27–31)

- **§28 — Lent boundaries:** Ash Wednesday → **Mass of the Lord's Supper
  exclusive**. Alleluia not said from beginning of Lent until the Paschal
  Vigil.
- **§29:** Ash Wednesday is a fast day; ashes are distributed.
- **§30 — Lent Sundays:** 1st–5th Sundays of Lent. 6th = "Palm Sunday of
  the Passion of the Lord."
- **§31:** Holy Week commemorates Christ's Passion. Thursday morning:
  Chrism Mass.

#### IV. Christmas Time (§32–38)

- **§33 — Christmas Time boundaries:** First Vespers of the Nativity →
  Sunday after Epiphany (or after Jan 6) inclusive.
- **§34:** Vigil Mass of the Nativity on Dec 24 evening. Three Masses on
  Christmas Day (night, dawn, day).
- **§35 — Christmas Octave arrangement:**
  - (a) Holy Family: Sunday within octave (or Dec 30 if no Sunday)
  - (b–d) Stephen (Dec 26), John (Dec 27), Holy Innocents (Dec 28)
  - (e) Dec 29–31: days within the octave
  - (f) Jan 1: Mary, Mother of God (octave day)
- **§36:** Sunday between Jan 2–5 = 2nd Sunday after the Nativity.
- **§37 — Epiphany:** Jan 6, or Sunday between Jan 2–8 (cf. §7).
- **§38:** Sunday after Jan 6 = Feast of the Baptism of the Lord.

#### V. Advent (§39–42)

- **§40 — Advent boundaries:** First Vespers of the Sunday on or closest to
  Nov 30 → before First Vespers of Christmas.
- **§41:** Four Sundays of Advent.
- **§42 — Dec 17–24:** "Ordered in a more direct way" to preparing for
  Christmas. Elevated precedence (level 9 in §59). Proper O Antiphons.

#### VI. Ordinary Time (§43–44)

- **§43:** 33 or 34 weeks with no particular aspect of the mystery of Christ.
- **§44 — Two runs:**
  - Monday after the Sunday following Jan 6 → Tuesday before Lent
  - Monday after Pentecost → before First Vespers of Advent I

  Romcal models these as `EarlyOrdinaryTime` and `LateOrdinaryTime` periods.

#### VII. Rogation Days and Ember Days (§45–47)

- **§45–46:** Time, duration, and manner left to Conferences of Bishops.
- **§47:** Mass chosen from Masses for Various Needs.

  When a national calendar defines them, romcal models them as particular
  celebrations within the calendar inheritance hierarchy.

---

## Chapter II — The Calendar (§48–61)

### Title I — The Calendar and Celebrations to Be Inscribed in It (§48–55)

- **§48:** Calendar is **general** (Roman Rite) or **particular** (diocese,
  religious family). Romcal's `CalendarScope` reflects this distinction.
- **§49 — General Calendar:** Contains the Proper of Time (mystery of
  salvation cycle) and universally significant Saints. **Particular calendars**
  add proper celebrations, combined organically with the general cycle.
- **§50 — Particular calendar rules** (three guards):
  - (a) Proper of Time must remain intact with rightful preeminence.
  - (b) Proper celebrations combined organically; one celebration per saint
    per year (exception: _translatio_/_inventio_ as optional memorial).
  - (c) Indult celebrations must not duplicate or overburden.
- **§51:** Provinces, regions, or nations may share a common calendar.
  Religious provinces under same jurisdiction likewise.
- **§52 — What to inscribe:**
  - (a) Diocesan: patrons, cathedral dedication, saints connected to the diocese.
  - (b) Religious: title, founder, patron, members and associates.
  - (c) Individual churches: proper celebrations + cathedral anniversary + principal patrons of the place.
- **§53 — Overburdening guard:** When many saints, use common celebration;
  only most significant get individual entries in the full calendar.
- **§54:** Proper celebrations = obligatory or optional memorials, unless the
  Table of Precedence or pastoral reasons dictate otherwise.
- **§55:** Celebrations in a particular calendar bind all who follow it;
  changes require Apostolic See approval.

### Title II — The Proper Day for Celebrations (§56–58)

- **§56 — Birthday rule and impediment resolution:**
  - (a) General Calendar celebrations keep their date in particular calendars.
  - (b) Saints not in General Calendar → assigned to birthday; if unknown,
    another proper date.
  - (c) If birthday is impeded → closest free date.
  - (d) If pastoral reasons prevent transfer → the impeding celebration moves.
  - (e) Indult celebrations → pastorally appropriate date.
  - (f) **Lent-free dates:** Avoid particular celebrations during Lent, Easter
    Octave, and Dec 17–31, unless obligatory memorial or higher.
    St. Joseph special rule (Palm Sunday → preceding Saturday).
- **§57:** Saints inscribed together: celebrated together if equal rank;
  higher rank celebrated alone, others omitted or transferred as optional memorial.
- **§58:** For pastoral good, celebrations ranking above a Sunday in OT may be
  observed on that Sunday (all Masses).

### Table of Precedence (§59)

**The core of romcal's precedence engine.** 13 levels, grouped in 3 tiers:

**Tier I** (always prevail):

1. Paschal Triduum
2. Nativity, Epiphany, Ascension, Pentecost; Advent/Lent/Easter Sundays;
   Ash Wednesday; Holy Week Mon–Thu; Easter Octave
3. General Calendar solemnities (Lord, BVM, Saints); All Souls
4. Proper solemnities (principal patron, dedication, title, founder)

**Tier II** (feasts and strong weekdays): 5. Lord's feasts in the General Calendar 6. Sundays of Christmas Time and Ordinary Time 7. BVM and Saints feasts in the General Calendar 8. Proper feasts (diocese patron, cathedral dedication, regional patron, etc.) 9. Advent Dec 17–24; Christmas Octave days; Lent weekdays

**Tier III** (memorials and ordinary weekdays): 10. General Calendar obligatory memorials 11. Proper obligatory memorials (secondary patron, etc.) 12. Optional memorials (may be celebrated even on level-9 days; obligatory
memorials on Lent weekdays also treated as optional) 13. Ordinary weekdays: Advent up to Dec 16; Christmas Jan 2–Epiphany Saturday;
Easter Mon after Octave–Sat before Pentecost; OT weekdays

### Transfer and Conflict Rules (§60–61)

- **§60 — Transfer of impeded solemnities:** When multiple celebrations
  coincide, the highest-ranked prevails. An impeded solemnity moves to the
  **closest day not in levels 1–8**. Special rule: Annunciation on any day
  of Holy Week → always transferred to Monday after the 2nd Sunday of Easter.
  Other impeded celebrations are simply **omitted** that year.
- **§61 — Vespers conflict:** When current-day Vespers and next-day First
  Vespers collide, the **higher-ranked** celebration wins; on equal rank,
  current day prevails.

---

## Cross-References to Other Liturgical Documents

The GNLY does not operate in isolation. For a complete picture, these related
documents provide complementary rules:

- **GIRM** ([index](/architecture/girm-index) · [source](/reference/girm)):
  §355 choice of Mass (optional memorials decision tree); §346 liturgical
  colors; §372–385 Ritual/Votive/Dead Mass restrictions — all rely on the
  GNLY rank and precedence system.
- **CP** ([index](/architecture/cp-index) · [source](/reference/cp)):
  Detailed rules for particular calendars (expands GNLY §48–55); patron
  categories, calendar inheritance, proper celebration inscription.
- **GILM** ([index](/architecture/gilm-index) · [source](/reference/gilm)):
  Lectionary structure and readings cycles. The GILM relies on GNLY ranks
  to determine number of readings (3 for solemnities, 2 for feasts, etc.).
- **GILH** ([index](/architecture/gilh-index) · [source](/reference/gilh)):
  Liturgy of the Hours rules. The GILH Office overlay logic (§225–240)
  mirrors GNLY rank definitions; §133 psalter cycle resets align with
  season boundaries defined here.
- **PS** ([index](/architecture/ps-index) · [source](/reference/ps)):
  Easter feasts norms. PS reinforces and pastorally expands GNLY §5 (Lent),
  §16b (Lenten precedence), §18 (Triduum), §22 (Easter Time 50 days),
  §28 (Alleluia suppression), §58 (Easter Sunday precedence).
