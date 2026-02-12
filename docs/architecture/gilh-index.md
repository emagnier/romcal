---
title: 'GILH — Navigation Index'
description: Chapter-by-chapter summary of the General Instruction of the Liturgy of the Hours, highlighting sections relevant to romcal's implementation and future development.
---

:::note[Purpose]
This index helps quickly locate the relevant rules in the
[GILH reference document](/reference/gilh) (~1 300 lines, §1–284).
Each chapter is summarized with its key paragraphs and their relevance to
romcal's implementation and future features. Use the § numbers to jump
to the source text.
:::

:::tip[When to use this index]
Consult it when implementing Liturgy of the Hours features: psalm cycle
coordination, memorial overlay rules, Te Deum conditions, Hour suppression,
vigil extensions, or antiphon/reading selection by rank. Read the full GILH
source only when you need exact wording or context beyond what this index
provides.
:::

---

## Quick Lookup — romcal concept → GILH §§

| romcal concept                         | GILH §§        | Notes                                                          |
| -------------------------------------- | -------------- | -------------------------------------------------------------- |
| Psalter 4-week cycle                   | §126–135       | Resets at §133; psalm distribution by Hour                     |
| Psalmody on solemnities                | §134, §225–229 | Laudate Psalms (EVP I), Sunday Wk I (Lauds), Gradual (Daytime) |
| Invitatory                             | §34–36         | Psalm 95 (or 100, 67, 24) + variable antiphon                  |
| Lauds / Vespers structure              | §37–54         | Principal Hours; Benedictus / Magnificat canticles             |
| Office of Readings structure           | §55–69         | 3 psalms + 2 readings + Te Deum (conditional)                  |
| Te Deum (yes/no)                       | §68            | Solemnities, feasts, Sundays outside Lent; not memorials       |
| Vigil extension (Office of Readings)   | §73, §206      | Canticles + Gospel before Te Deum                              |
| Daytime Prayer structure               | §74–83         | Current vs. complementary psalmody (§81–83)                    |
| Compline structure                     | §84–92         | Most stable Hour; Marian antiphon (§92)                        |
| Marian antiphon at Compline            | §92            | 4 antiphons by liturgical period                               |
| Antiphon selection rules               | §116–120       | Proper (seasons/solemnities), Common, weekday                  |
| Gospel canticle antiphons              | §119           | Benedictus / Magnificat antiphon sources by rank               |
| Alleluia in antiphons (Easter)         | §120           | Added to all antiphons during Easter season                    |
| Scripture reading cycles               | §143–155       | 1-year and 2-year; seasonal distribution                       |
| Short readings (capitula)              | §156–158       | 4-week series (OT); seasonal series; NT only at Vespers        |
| Hagiographical readings                | §166–168       | Biographical note not read aloud (§168)                        |
| Concluding prayer by rank              | §197–200       | Proper vs. psalter, per Hour and day type                      |
| Compline concluding prayer             | §198           | Always from psalter — CP §44 identity does NOT apply           |
| Solemnity Office rules                 | §225–230       | EVP I, proper everything, Te Deum, Compline as Sunday          |
| Feast Office rules                     | §231–233       | No EVP I (except Lord's on Sunday); proper OdR/Lauds/Vespers   |
| Memorial overlay (OT)                  | §234–236       | Weekday psalms + saint elements; §235a-d priority rules        |
| Memorial suppression (privileged days) | §237–239       | §237 exclusion; §238 demotion; §239 limited additions          |
| Saturday BVM memorial                  | §240           | Celebrated as other optional memorials                         |
| Vespers suppression (Triduum)          | §209           | Omitted on Holy Thursday / Good Friday if at Mass              |
| Compline suppression                   | §211, §215     | Holy Saturday (Easter Vigil), Christmas night (vigil OdR)      |
| Easter Vigil replaces OdR              | §212           | Reduced form for absentees: 4 readings + Te Deum               |
| Combining Hours with Mass              | §93–98         | Shared opening rite, psalm as entrance chant                   |

---

## At a Glance

| Chapter | §§          | Relevance    | Key topics                                                                                            |
| ------- | ----------- | ------------ | ----------------------------------------------------------------------------------------------------- |
| I       | 1–33        | —            | Theology, participants, obligation to pray the Office                                                 |
| **II**  | **34–99**   | **High**     | **Structure of each Hour (Invitatory → Lauds → OdR → Daytime → Vespers → Compline)**                  |
| **III** | **100–203** | **High**     | **Elements: psalms, antiphons, readings, hymns, responsories, prayers**                               |
| **IV**  | **204–252** | **Critical** | **How celebrations at each rank affect the Office (solemnities, feasts, memorials, privileged days)** |
| V       | 253–284     | —            | Roles and singing in communal celebration                                                             |

---

## Chapter I (§1–33) — Importance of the Liturgy of the Hours

Theology, prayer of Christ and the Church, participants, obligation.
Not relevant to romcal.

- **§33** (minor): Basic structural principle — "hymn, then always psalmody,
  then a long or short reading, and finally prayer of petition."

## Chapter II (§34–99) — The Different Liturgical Hours

Structure of each Hour in sequence. Essential reference when determining
which elements apply for a given Hour and rank.

### Invitatory (§34–36)

- **§34–35:** Opens the first Hour of the day (Office of Readings or Lauds).
  Verse "Lord, open my lips" + Psalm 95 (alternatives: Ps 100, 67, 24).
- **§36:** Invitatory antiphon varies by liturgical day.

### Morning Prayer and Evening Prayer (§37–54)

- **§37:** Lauds and Vespers are the "two hinges" of the daily Office —
  the two principal Hours.
- **§43:** Psalmody structure:
  - Lauds: 1 morning psalm + 1 OT canticle + 1 praise psalm.
  - Vespers: 2 psalms + 1 NT canticle.
- **§50:** Gospel canticles: Benedictus (Lauds), Magnificat (Vespers).
- **§51:** Morning = invocations to consecrate the day;
  evening = intercessions (including intention for the dead, §186).
- **§53:** Concluding prayer: from the psalter on OT weekdays;
  from the proper on other days. Detailed rules in §197–200.

### Office of Readings (§55–69)

- **§59:** May be recited at any hour of the day (not bound to night).
- **§62:** 3 psalms — proper on solemnities/feasts/Easter Triduum/octaves;
  from the psalter on Sundays/weekdays/memorials.
- **§64:** Two readings: 1st = Scripture; 2nd = Fathers/Church writers
  or hagiographical (on saints).
- **§66:** Scripture reading from Proper of Seasons; on solemnities/feasts
  from proper or common.
- **§67:** On memorials, hagiographical reading replaces current 2nd reading
  (if one exists in proper or common).
- **§68 — Te Deum:** Said on solemnities, feasts, Sundays outside Lent,
  days within Easter/Christmas octaves. **Not** on memorials or weekdays.
- **§69:** Concluding prayer + acclamation "Let us praise the Lord."

### Vigils (§70–73)

- **§70:** Easter Vigil is the supreme vigil (replaces Office of Readings).
- **§73 — Vigil extension:** On Sundays, solemnities, and feasts, the Office
  of Readings may be extended with canticles from the appendix + a Gospel
  reading (+ optional homily), inserted **before** the Te Deum. Gospel from
  the Lectionary (solemnities/feasts) or paschal mystery series (Sundays).

### Daytime Prayer (§74–83)

- **§77:** Outside choir, one may choose the hour best suited to time of day.
- **§79:** Structure: verse → hymn → 3 psalms → short reading → ℣ → prayer.
- **§81–83:** Two psalmody schemes:
  - **Current** (§81–82): from the psalter week; used when praying one hour.
    On solemnities/Easter Triduum/Easter Octave: complementary psalms with
    proper antiphons.
  - **Complementary** (§83): Gradual Psalms (Ps 120–128) in 3 sets of 3;
    used when praying all three hours.

### Night Prayer / Compline (§84–92)

- **§84:** Final prayer before retiring, even after midnight.
- **§88:** Psalmody by day:
  - After EVP I (Sunday): Ps 4 + 134.
  - After EVP II (Sunday): Ps 91.
  - Other days: confidence psalms (or Sunday psalms by permission).
- **§89:** Reading → responsory "Into your hands" → Nunc Dimittis + antiphon.
- **§90:** Concluding prayer always from the psalter.
- **§92 — Marian antiphon:** One of four antiphons assigned by liturgical
  period (_Regina Caeli_ during Easter season). The Presentation of the
  Lord (Feb 2) is a rubrical period boundary.

### Combining Hours with Mass or Each Other (§93–99)

- **§93–98:** Provisions for combining Lauds with Morning Mass or Vespers
  with Evening Mass: shared opening rite, psalm as entrance chant, single
  concluding rite. Not modeled as data structures, but relevant context for
  consumers.

## Chapter III (§100–203) — The Different Elements

Detailed rules for each element of the Office. Essential for implementing
element selection logic.

### Psalms (§100–109)

Theological and pastoral guidance on psalm prayer. Not relevant to romcal.

### Antiphons (§110–120)

- **§116 — Proper antiphons by season:** Provided for Easter Triduum,
  Easter/Christmas octaves, Advent/Christmas/Lent/Easter Sundays and
  weekdays, Dec 17–24.
- **§117:** On solemnities, proper antiphons for OdR, Lauds, Daytime, Vespers;
  if none, from common.
- **§118:** Memorials with proper antiphons retain them.
- **§119 — Gospel canticle antiphons (Benedictus/Magnificat):** From Proper
  of Seasons if given; else from current psalter week; on solemnities/feasts
  from proper or common; on memorials without proper, from common or weekday.
- **§120:** During Easter season, _Alleluia_ added to all antiphons.

### Psalm Arrangement (§126–135)

- **§126:** 4-week cycle; few psalms omitted; important psalms more frequent.
- **§129–130:** Sunday psalms express paschal mystery; Friday psalms
  penitential. Psalms 78, 105, 106 reserved for Advent/Christmas/Lent/Easter.
- **§131:** Psalms 58, 83, 109 omitted (imprecatory).
- **§133 — Cycle coordination:** Resets Week I at 1st Sunday of Advent,
  1st Sunday of Lent, 1st Sunday of OT, Easter Sunday. After Pentecost,
  resumes per Proper of Seasons indication. **Currently implemented in
  romcal** (`core/src/types/liturgical/cycles.rs`).
- **§134 — Psalmody on solemnities/feasts:**
  - EVP I: Laudate Psalms (Ps 113, 117, 135, 146, 147A, 147B).
  - Office of Readings: proper psalms from tradition.
  - Lauds: psalms from Sunday of Week I.
  - Daytime Prayer: Gradual Psalms (Ps 120–128) with proper antiphon;
    on Sundays: Sunday of Week I.
  - EVP II: proper psalms and canticle.
- **§135:** All other cases: psalms from current week/day of psalter.

### Canticles (§136–139)

- **§136:** Morning: OT canticle between 1st and 2nd psalm; each weekday
  has its own; Sundays alternate sections of Canticle of Three Children.
- **§137:** Evening: NT canticle after 2 psalms; 7 canticles per week.
  Lenten Sundays: 1 Peter canticle replaces Alleluia canticle.
- **§138:** Gospel canticles (Zechariah, Mary, Simeon) treated with
  gospel-level solemnity.
- **§144:** No Gospel readings in the Office, except in the vigil form
  of OdR (§73). Architectural principle for the data model.

### Scripture Readings (§140–155)

- **§143–146:** One-year and two-year cycles complement the Mass lectionary.
- **§147:** Advent: Isaiah (semicontinuous); Dec 17–24 special readings.
- **§150:** Lent: Deuteronomy + Hebrews (Year I); Exodus/Leviticus/Numbers
  - Hebrews (Year II). Holy Week: Servant Songs + Lamentations or Jeremiah.
- **§151:** Easter: 1 Peter, Revelation, 1–3 John (Year I); Acts (Year II).
- **§152:** Ordinary Time: 34-week continuous scheme; interrupted by Lent;
  resumed after Pentecost. 33-week years drop week after Pentecost.
- **§154:** Proper readings for solemnities/feasts; otherwise from Common.

### Short Readings (§156–158)

- **§156–157:** Four weekly series for OT (rotate with psalter); weekly
  series per season; proper for solemnities/feasts/some memorials;
  one-week series for Compline.
- **§158:** (a) Exclude Gospels; (b) respect Sunday/Friday/Hour character;
  (c) NT only at Evening Prayer.

### Hagiographical Readings (§166–168)

- **§166:** Patristic texts applicable to the saint, saint's own writings,
  or biographical texts.
- **§168:** Biographical note is for information only — not read aloud.
  Modeled as a separate field in the data structure.

### Responsories (§169–172)

- **§169:** Follow Scripture reading in OdR (traditional or newly composed).
- **§172:** Short responsory (Lauds/Vespers/Compline) and verse (Daytime
  Prayer) linked to the short reading.

### Hymns (§173–178)

- **§175–176:** Twofold cycle for Ordinary Time (alternating weeks);
  twofold cycle for OdR (night and day selections).

### Intercessions, Lord's Prayer, Concluding Prayer (§179–200)

- **§183:** Different formularies for each day of the 4-week psalter +
  special seasons/feasts.
- **§186:** Evening intercessions end with intention for the dead.
- **§197–200 — Concluding prayer selection:**
  - **§198:** At Compline, **always** from the psalter (CP §44 identity
    does NOT apply).
  - **§199:** Lauds/Vespers: from proper (Sundays, Advent/Christmas/Lent/
    Easter weekdays, solemnities/feasts/memorials); from psalter (OT weekdays).
  - **§200:** Daytime Prayer: from proper (Sundays, seasonal weekdays,
    solemnities/feasts); from psalter (other days).

## Chapter IV (§204–252) — Various Celebrations Throughout the Year

**The most critical chapter for romcal's Office implementation.** Defines
how celebrations at each rank affect the Office content.

### Mysteries of the Lord (§204–217)

#### Sunday (§204–207)

- **§204:** Sunday Office begins with EVP I from the 4-week psalter
  (except proper parts).
- **§205:** When a Lord's feast falls on Sunday, it has proper EVP I.

#### Easter Triduum (§208–213)

- **§209 — Vespers suppression:** Those attending the evening Mass (Holy
  Thursday) or the Celebration of the Lord's Passion (Good Friday) omit
  Vespers on those days.
- **§210:** OdR on Good Friday / Holy Saturday should be publicly celebrated
  before Lauds if possible.
- **§211 — Compline suppression (Holy Saturday):** Those attending the
  Easter Vigil omit Compline.
- **§212 — Easter Vigil replaces OdR:** For absentees, a reduced OdR with
  at least 4 readings (recommended: Exodus, Ezekiel, St. Paul, Gospel)
  - Te Deum + prayer of the day.
- **§213:** Lauds on Easter Sunday said by all.

#### Christmas Season (§215–216)

- **§215 — Christmas night:** Vigil form of OdR before Midnight Mass;
  Compline omitted by those who attend.
- **§216:** Lauds before Dawn Mass on Christmas Day.

### The Saints (§218–240)

#### General Principles (§218–224)

- **§219:** Celebrations = solemnities, feasts, or memorials.
- **§221:** Multiple optional memorials on same day → celebrate only one.
- **§222:** Only solemnities are transferred (not feasts/memorials).
- **§224:** Missing proper parts supplied from Common.

#### Solemnities (§225–230)

- **§225:** Solemnities have EVP I on the preceding evening.
- **§226:** EVP I/II: proper hymn, antiphons, short reading, responsory,
  concluding prayer; EVP I psalms from Laudate Psalms.
- **§227:** Lauds: proper everything; psalms from Sunday of Week I.
- **§228:** OdR: wholly proper (hymn, antiphons, psalms, readings,
  responsories); ends with Te Deum + proper prayer.
- **§229:** Daytime Prayer: weekday hymn (unless directed); Gradual Psalms
  with proper antiphon (weekdays) or Sunday Wk I (Sundays).
- **§230:** Compline as Sundays (after EVP I or II respectively).

#### Feasts (§231–233)

- **§231:** No EVP I (except Lord's feasts on Sundays). OdR, Lauds, Vespers
  as solemnities. Te Deum said.
- **§232:** Daytime Prayer: weekday hymn, weekday psalms + antiphons
  (unless special tradition), **proper** short reading + concluding prayer.
- **§233:** Compline as ordinary days (from weekday).

#### Memorials (§234–240)

- **§234:** No difference between obligatory and optional memorials,
  except on privileged weekdays.

- **§235 — Memorial overlay in Ordinary Time** (the key rule set):
  - **(a)** Psalms + psalm antiphons: from current weekday psalter
    (unless proper indicated).
  - **(b)** Invitatory antiphon, hymn, short reading, canticle antiphons
    (Benedictus/Magnificat), intercessions: priority order =
    saint's Proper → Common → weekday.
  - **(c)** Concluding prayer: **mandatory** from the saint.
  - **(d)** Office of Readings: 1st reading from Scripture cycle;
    2nd reading = hagiographical (replaces patristic); no Te Deum.

- **§236:** Daytime Prayer and Compline entirely from weekday —
  no saint elements.

- **§237 — Complete exclusion:** On Sundays, solemnities, feasts,
  Ash Wednesday, Holy Week, and Easter Octave — no memorials at all.

- **§238 — Demotion:** Obligatory memorials become optional on
  Advent Dec 17–24, Christmas Octave, and Lent weekdays.

- **§239 — Limited additions on privileged weekdays:**
  - **(a)** OdR: hagiographical reading **added after** patristic
    (not replacing); saint's concluding prayer **replaces** weekday.
  - **(b)** Lauds/Vespers: saint's antiphon (Benedictus/Magnificat)
    and prayer **appended** (weekday prayer ending omitted).

- **§240:** Saturday BVM memorial in OT: celebrated as other optional
  memorials (§235–236 rules).

### Calendar and Choice of Office (§241–252)

- **§244:** On optional memorial weekdays, a Martyrology saint may be
  celebrated as a memorial (parallels GIRM §355.3c).
- **§247:** Never change formularies for Sundays, solemnities, Lord's feasts,
  Lent/Holy Week, Easter/Christmas octaves, Dec 17–24.
- **§249:** Interrupted continuous reading: may combine omitted parts or
  choose preferred texts (parallels GIRM §358).

## Chapter V (§253–284) — Rites for Celebration in Common

Roles of ministers and singing norms. Not relevant to romcal.

---

## Cross-References to Other Liturgical Documents

The GILH does not operate in isolation. For a complete picture, these
related documents provide complementary rules:

- **GNLY** ([index](/architecture/gnly-index) · [source](/reference/gnly)):
  §59 Table of Precedence; §4–16 seasons; §13 natural day / EVP I rules;
  §14 memorial demotion in Lent.
- **GIRM** ([index](/architecture/girm-index) · [source](/reference/girm)):
  Mass-selection logic (§355); the Mass counterpart to the Office's
  memorial overlay (§363 orations vs. GILH §235).
- **CP** ([index](/architecture/cp-index) · [source](/reference/cp)):
  §44 cross-domain identity rule: "The prayer is always the same as the
  opening prayer of the Mass." Exception: Compline (GILH §198).
- **GILM** ([index](/architecture/gilm-index) · [source](/reference/gilm)):
  Lectionary structure and readings cycles. Complements GILH §143–155
  for the Office of Readings Scripture cycle.
- **PS** ([index](/architecture/ps-index) · [source](/reference/ps)):
  §40 Good Friday Office of Readings (cf. GILH §210);
  §209–215 Hour suppression on Triduum days.
