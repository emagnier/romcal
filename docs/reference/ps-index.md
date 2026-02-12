---
title: 'PS — Navigation Index'
description: Section-by-section summary of Paschalis Sollemnitatis (Circular Letter on the Easter Feasts), highlighting sections relevant to romcal's implementation and future development.
---

:::note[Purpose]
This index helps quickly locate the relevant rules in the
[PS reference document](/reference/ps) (~425 lines, §1–108).
Each chapter is summarized with its key paragraphs and their relevance to
romcal's implementation and future features. Use the § numbers to jump
to the source text.
:::

:::tip[When to use this index]
Consult it when implementing the Paschal Triduum data model, Easter Vigil
readings structure, Good Friday / Holy Saturday liturgical constraints,
Palm Sunday entrance rite, Chrism Mass placement, Lenten Alleluia
suppression, or Pentecost vigil forms. PS is a **complementary document**
— it clarifies and expands on the primary norms (GNLY, GIRM, GILH, GILM)
for the Lenten and Easter cycles, but does not introduce new calendar
calculation rules. Read the full PS source when you need exact wording or
pastoral detail.
:::

---

## Quick Lookup — romcal concept → PS §§

| romcal concept                               | PS §§    | Notes                                                                        |
| -------------------------------------------- | -------- | ---------------------------------------------------------------------------- |
| Lenten Alleluia suppression                  | §18      | Omitted from beginning of Lent until Paschal Vigil, even on solemnities      |
| Lent Sunday/weekday precedence               | §11      | Sundays > all feasts/solemnities; weekdays > obligatory memorials            |
| Laetare Sunday (4th Sunday of Lent)          | §25      | Flowers, instruments, rose vestments                                         |
| Covering crosses and images                  | §26      | Episcopal conference decision; crosses until Good Friday, images until Vigil |
| Triduum boundaries                           | §27, §38 | Evening Mass of Holy Thursday → Vespers of Easter Sunday                     |
| Easter fast                                  | §39      | Good Friday (obligatory); Holy Saturday (recommended)                        |
| Palm Sunday entrance rite                    | §28–32   | Three forms: solemn procession, solemn entrance, simple entrance             |
| Palm Sunday Passion narrative                | §33      | Three-person format; no candles/incense; read in entirety                    |
| Chrism Mass                                  | §35–36   | Bishop + presbyterium; Holy Thursday (transferable); one per diocese         |
| Holy Thursday Evening Mass                   | §44–48   | Begins Triduum; institution of Eucharist/priesthood; washing of feet         |
| Gloria bells silence (Holy Thursday → Vigil) | §50      | Bells ring at Gloria, then silent until Easter Vigil Gloria                  |
| Transfer of Blessed Sacrament                | §54–55   | Procession to place of repose; no monstrance; not a "tomb"                   |
| Good Friday — no Eucharist                   | §59      | Only Celebration of the Lord's Passion; Communion from reserved Sacrament    |
| Good Friday — no sacraments                  | §61      | Except Penance and Anointing of the Sick                                     |
| Celebration of the Lord's Passion structure  | §64–70   | Liturgy of the Word, adoration of the cross, Holy Communion                  |
| Holy Saturday — no Mass                      | §75      | Strictly no Mass; Communion only as Viaticum                                 |
| Easter Vigil nocturnal character             | §78      | Must begin after nightfall, end before daybreak                              |
| Easter Vigil 4-part structure                | §81      | Light → Word → Baptism → Eucharist                                           |
| Easter Vigil readings                        | §85      | 7 OT + Epistle + Gospel; minimum 3 OT; Exodus 14 never omitted               |
| Easter Vigil Alleluia restoration            | §87      | Priest intones 3×, rising pitch; Psalm 117                                   |
| Easter Vigil baptismal liturgy               | §88–89   | Font blessing, renewal of baptismal promises, sprinkling                     |
| Easter Day Mass                              | §97      | Sprinkling rite with Vigil water; great solemnity                            |
| Paschal candle                               | §99      | Lit until Pentecost; kept in baptistry after Easter season                   |
| Easter Time — 50 days as one feast           | §100     | "Great Sunday"; Easter Sunday to Pentecost                                   |
| Easter Sundays precedence                    | §101     | Precedence over all feasts/solemnities of the Lord                           |
| Pentecost vigil                              | §107     | Prolonged Mass in vigil form; character of urgent prayer, not baptismal      |
| `MassTime::CelebrationOfThePassion`          | §59      | Non-eucharistic; `is_eucharistic: false`                                     |
| `MassTime::EveningMassOfTheLordsSupper`      | §44–48   | Begins the Paschal Triduum                                                   |
| `MassTime::ChrismMass`                       | §35–36   | Assigned in particular (diocesan) calendars                                  |
| `MassTime::EasterVigil`                      | §78, §81 | Civil date = Holy Saturday, liturgical date = Easter Sunday                  |
| `VigilSequence` (readings)                   | §85      | Variable-minimum OT readings; Exodus 14 mandatory                            |
| `ReadingsSet.alleluia` in Lent               | §18      | Replaced by Lenten verse regardless of rank                                  |
| `entrance_gospel` on Palm Sunday             | §28–32   | Entry Gospel (procession) distinct from Passion Gospel (Mass)                |

---

## At a Glance

| Chapter  | §§          | Relevance    | Key topics                                                                   |
| -------- | ----------- | ------------ | ---------------------------------------------------------------------------- |
| Intro    | 1–5         | —            | Historical context and purpose of the circular letter                        |
| **I**    | **6–26**    | **Medium**   | **Lent: initiation, celebrations, Alleluia suppression (§18), Laetare**      |
| **II**   | **27–37**   | **High**     | **Holy Week: Palm Sunday entrance (§28–33), Chrism Mass (§35–36)**           |
| **III**  | **38–43**   | **High**     | **Triduum boundaries (§38), Easter fast, pastoral guidance**                 |
| **IV**   | **44–57**   | **High**     | **Holy Thursday: Evening Mass, Gloria silence (§50), Sacrament transfer**    |
| **V**    | **58–72**   | **Critical** | **Good Friday: no Eucharist (§59), Passion structure (§64–70)**              |
| **VI**   | **73–76**   | **Critical** | **Holy Saturday: no Mass (§75), aliturgical day**                            |
| **VII**  | **77–99**   | **Critical** | **Easter Vigil structure (§81), readings (§85), Alleluia (§87), Easter Day** |
| **VIII** | **100–108** | **Medium**   | **Easter Time: 50 days, Sunday precedence (§101), Pentecost vigil (§107)**   |

---

## Introduction (§1–5)

Historical and pastoral context for the circular letter. Not directly
modeled in romcal.

- **§1:** Recalls the 1951/1955 reforms of Pius XII for the Easter
  Solemnity and Holy Week.
- **§2:** The liturgical year's summit is the Easter Triduum, prepared
  by Lent and prolonged for 50 days.
- **§3–4:** Pastoral concern: waning enthusiasm, incorrect timing,
  competing devotions, holiday-period conflicts.
- **§5:** Purpose of the document: recall existing norms, not replace
  the Missal rubrics.

## I. Lenten Season (§6–26)

### a. Rite of Christian Initiation (§7–10)

- **§7:** Lent has full character as purification/enlightenment time;
  Easter Vigil is the proper time for initiation sacraments.
- **§8–10:** Catechumenate, adult catechesis, penitential rites for
  children. Not directly modeled.

### b. Celebrations During Lent (§11–20)

- **§11 — Lenten precedence rules:**
  Sundays of Lent > all feasts and solemnities (solemnities anticipated
  to Saturday). Lenten weekdays > obligatory memorials. Reinforces
  GNLY §5, §16b.

- **§17 — No flowers on altar in Lent;** instruments only to support
  singing. Preserves penitential character.

- **§18 — Alleluia suppression:**
  "From the beginning of Lent until the Paschal Vigil, 'Alleluia' is to
  be omitted in all celebrations, even on solemnities and feasts."
  Clarifies GNLY §28 — the suppression is absolute regardless of rank.
  Mapped to romcal's Lenten acclamation logic: `ReadingsSet.alleluia`
  replaced by a Lenten verse (cf. GILM §91).

- **§25 — Laetare Sunday (4th Sunday of Lent):**
  Flowers allowed, instruments may be played, rose-colored vestments.
  Mapped to romcal's liturgical color rules.

- **§26 — Covering crosses and images:**
  Optional, per episcopal conference. Crosses until Good Friday
  celebration; images until Easter Vigil.

### c. Particular Days of Lent (§21–26)

- **§21 — Ash Wednesday:** Blessing and imposition of ashes; in Mass
  or Liturgy of the Word.
- **§22:** Day of penance: abstinence and fasting.
- **§23 — First Sunday of Lent:** Rite of election.
- **§24 — Initiation Gospels:** Samaritan woman, man born blind,
  Lazarus (Sundays 3–5, Year A; usable in B/C). Cf. GILM §97.

## II. Holy Week (§27–37)

- **§27 — Triduum boundaries:**
  "The Lenten season lasts until the Thursday of this week. The Easter
  Triduum begins with the evening Mass of the Lord's Supper, is
  continued through Good Friday with the celebration of the passion of
  the Lord and Holy Saturday, reaches its summit in the Easter Vigil, and
  concludes with Vespers of Easter Sunday." Holy Monday–Thursday >
  all other celebrations. No Baptisms/Confirmations on these days.

  This is the normative basis for romcal's Triduum date boundaries.

### a. Passion Sunday / Palm Sunday (§28–34)

- **§28 — Dual character:** Foretelling of Christ's regal triumph +
  proclamation of the Passion.

- **§29–30 — Three entrance forms:**
  - **(1) Solemn procession** (§29): from a secondary church/place,
    with palms, procession to main church. Only once per day.
  - **(2) Solemn entrance** (§30): when procession outside not possible.
  - **(3) Simple entrance** (§30): used at all other Masses.

  Mapped to romcal's `entrance_gospel: Option<ReadingText>` on
  `CelebrationMass`. The three forms are a pastoral choice, not modeled
  as separate celebration types.

- **§33 — Passion narrative:**
  Three-person format (Christ, narrator, people); no candles/incense;
  read in entirety. Two Gospel readings on Palm Sunday: the Entry
  Gospel (procession) and the Passion Gospel (Mass).

### b. Chrism Mass (§35–36)

- **§35:** Bishop concelebrates with presbyterium. Traditionally Holy
  Thursday morning, transferable to another day close to Easter.
- **§36:** One celebration per diocese, in the cathedral.

  Mapped to `MassTime::ChrismMass`, assigned in particular (diocesan)
  calendars.

### c. Penitential Celebrations (§37)

- **§37:** Conclude Lent with penitential celebration, before Triduum.
  Not directly modeled.

## III. The Easter Triduum in General (§38–43)

- **§38 — Triduum definition:**
  "The triduum of the crucified, buried and risen." Evening Mass of
  Holy Thursday → Vespers of Easter Sunday. Reinforces §27.

- **§39 — Easter fast:**
  Sacred on Good Friday and Holy Saturday. Good Friday: fasting and
  abstinence (obligatory). Holy Saturday: recommended.

- **§40:** Communal celebration of Office of Readings and Morning Prayer
  recommended on Good Friday and Holy Saturday.

- **§41–42:** Sufficient ministers; importance of chant.
  §42 lists specific chant elements for the Triduum.

- **§43:** Small communities should join larger churches for the
  Triduum. Pastor with multiple parishes may repeat celebrations
  per norms.

## IV. Holy Thursday Evening Mass of the Lord's Supper (§44–57)

- **§44–45:** Commemorates institution of Eucharist, priesthood, and
  brotherly love (foot-washing command).

- **§46–47:** Concelebration permitted. Additional Masses only by
  Ordinary's permission, for pastoral necessity. All private Masses
  forbidden.

- **§48:** Tabernacle empty before celebration; hosts consecrated for
  Good Friday Communion too.

- **§50 — Gloria bells and organ silence:**
  Bells ring at Gloria, then silent until Easter Vigil Gloria. Organ
  and instruments only to support singing in this interval.

- **§54–55 — Transfer of Blessed Sacrament:**
  Procession to place of repose with "Pange lingua." Not a "tomb."
  No monstrance exposition.

- **§57:** Altar stripped after Mass. Crosses may be covered (red or
  purple veil).

  Mapped to `MassTime::EveningMassOfTheLordsSupper`.

## V. Good Friday (§58–72)

**Critical for romcal's non-eucharistic celebration model.**

- **§59 — No Eucharist:**
  "The Church does not celebrate the Eucharist." Communion only during
  the Celebration of the Lord's Passion (or to the sick at any time).

  Mapped to `MassTime::CelebrationOfThePassion` with
  `is_eucharistic: false`.

- **§60:** Day of penance: abstinence and fasting (obligatory).

- **§61 — No sacraments:** Strictly prohibited except Penance and
  Anointing of the Sick. No funerals with singing/music/bells.

- **§63:** Celebration at about 3 PM, or adjusted for pastoral reasons
  (not later than 9 PM).

- **§64–70 — Structure of the Celebration:**
  - **(1) Liturgy of the Word** (§65–67): silent entrance, prostration;
    readings in entirety; John's Passion; General Intercessions in
    ancient form.
  - **(2) Adoration of the Cross** (§68–69): one cross; individual
    veneration; antiphons and "Reproaches."
  - **(3) Holy Communion** (§70): Lord's Prayer, then Communion; pyx
    removed after.

- **§72:** Devotions (Way of the Cross, etc.) subordinate to the
  liturgical celebration.

## VI. Holy Saturday (§73–76)

**Critical for romcal's aliturgical-day model.**

- **§73:** The Church is at the Lord's tomb. Office of Readings and
  Morning Prayer recommended with people.

- **§75 — No Mass:**
  "The Church abstains strictly from the celebration of the sacrifice
  of the Mass." Communion only as Viaticum.

  No `MassComposition` is generated for Holy Saturday's civil date
  until the Easter Vigil (which belongs liturgically to Easter Sunday
  and is shifted to Saturday evening in Layer 2 Mass).

- **§76:** Festive customs reserved for Easter night/day, not
  anticipated on Holy Saturday.

## VII. Easter Sunday of the Lord's Resurrection (§77–99)

**Critical for romcal's Easter Vigil data model.**

### a. The Easter Vigil (§77–96)

- **§77:** "Mother of all holy vigils." Celebrates the Paschal Mystery
  and Christian initiation.

- **§78 — Nocturnal character:**
  "It should not begin before nightfall; it should end before daybreak
  on Sunday." Strictest sense required.

  Mapped to `MassTime::EasterVigil` with `civil_date: Holy Saturday`,
  `liturgical_date: Easter Sunday`.

- **§81 — Four-part structure:**
  - **(1) Service of Light** (§82–84): new fire, paschal candle,
    procession, Easter Proclamation (_Exsultet_).
  - **(2) Liturgy of the Word** (§85–87): OT readings, Gloria,
    Alleluia restoration, NT readings, Gospel.
  - **(3) Baptismal Liturgy** (§88–89): font blessing, Baptism,
    renewal of promises, sprinkling.
  - **(4) Liturgy of the Eucharist** (§90–92): Easter Sacrament;
    Communion under both species.

  This liturgical order must not be changed.

- **§85 — Easter Vigil readings** _(the key paragraph for romcal)_:
  7 OT readings (Law and Prophets) + Epistle + Gospel. When pastoral
  conditions require reducing: **at least 3 OT readings**, and
  **Exodus 14 with its canticle must never be omitted.**

  Mapped to `VigilSequence` in romcal:
  - `min_ot_readings: 3`
  - `mandatory[Exodus 14]: true`
  - Full set: 7 OT + Romans 6 + Synoptic Gospel

- **§87 — Alleluia restoration:**
  After OT readings and Gloria, priest intones Alleluia 3 times
  (rising pitch). People repeat. Psalm 117. Then Epistle and Gospel.

- **§88–89 — Baptismal liturgy:**
  Font blessing (even without candidates); renewal of baptismal
  promises; sprinkling with "Vidi aquam."

### b. Easter Day (§97–99)

- **§97:** Mass with great solemnity; penitential rite as sprinkling
  with Vigil water.
- **§98:** Baptismal Vespers tradition.
- **§99 — Paschal candle:** Lit at least at all solemn celebrations
  until Pentecost. Kept in baptistry after Easter season; lit at
  funerals.

## VIII. Easter Time (§100–108)

- **§100 — Fifty days as one feast:**
  "The 'great Sunday.'" Easter Sunday to Pentecost. Cf. GNLY §22.

- **§101 — Easter Sunday precedence:**
  "Sundays of this season are regarded as Sundays of Easter… they have
  precedence over all feasts of the Lord and over all solemnities."
  Solemnities falling on these Sundays anticipated to Saturday. BVM/
  saints celebrations not transferred to these Sundays. Cf. GNLY §5, §58.

- **§102–106:** Mystagogical catechesis, neophytes, house blessings,
  popular practices. Not directly modeled.

- **§107 — Pentecost vigil:**
  "Prolonged celebration of Mass in the form of a vigil, whose
  character is not baptismal as in the Easter Vigil, but is one of
  urgent prayer." Confirms Pentecost has a vigil Mass form
  (`PreviousEveningMass` per GNLY §11). Two forms (extended with
  4 OT readings, and simple) modeled as separate `Fixed(ReadingsSet)`
  entries — no variable-minimum constraints like the Easter Vigil.

- **§108:** Concluding exhortation on paschal joy.

---

## Cross-References to Other Liturgical Documents

PS is a complementary document that clarifies and expands on the
primary norms. For the underlying rules, consult:

- **GNLY** ([index](/reference/gnly-index) · [source](/reference/gnly)):
  §5, §16b (Lenten precedence), §18 (Triduum dates), §22 (Easter Time
  50 days), §28 (Alleluia suppression), §58 (Easter Sunday precedence).
  PS reinforces and pastorally expands these norms.
- **GIRM** ([index](/reference/girm-index) · [source](/reference/girm)):
  Mass rubrics for the Triduum celebrations; liturgical color rules
  (§346 for Lent and Easter); Gloria/Creed rules per rank.
- **GILM** ([index](/reference/gilm-index) · [source](/reference/gilm)):
  §91 (Lenten acclamation replacing Alleluia — clarified by PS §18);
  §97 (Lenten Sunday readings, initiation Gospels — cf. PS §24);
  §99 (Easter Triduum readings structure).
- **GILH** ([index](/reference/gilh-index) · [source](/reference/gilh)):
  §210 (Office of Readings on Good Friday / Holy Saturday — cf. PS §40);
  §209–215 (Hour suppression on Triduum days).
- **CP** ([index](/reference/cp-index) · [source](/reference/cp)):
  §2 (Proper of Seasons precedence — Lent/Easter guarded from
  particular celebrations).
