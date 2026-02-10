# Liturgical Composition Model — Mass & Liturgy of the Hours

## Context and Motivation

When a weekday (feria, rank 13 in romcal) also has optional memorials (rank 12) available for the same civil date, the Church's norms (GIRM, GNLY, GILM, GILH) define precise rules for how the celebrant may choose and combine liturgical texts — readings, orations, antiphons, psalmody — between the feria and the optional memorial, both for the Mass and for the Liturgy of the Hours.

This document synthesizes the analysis of these liturgical rules and proposes a data model for romcal that faithfully reflects them, organized around three complementary output approaches:

- **Approach 1 — Liturgical Calendar** (`generate_liturgical_calendar`): centered on the liturgical day, for internal use and as the foundation for Approaches 2 and 3.
- **Approach 2 — Mass Calendar** (`generate_mass_calendar`): centered on the mass as celebrated on a civil date, with pre-resolved options and explicit composition rules.
- **Approach 3 — Hours Calendar** (`generate_hours_calendar`): centered on the Hours of the Office as celebrated on a civil date, with pre-resolved content and composition rules adapted to the Office's overlay mechanism.

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

> **Note on "Easter" in the table:** "Easter" here means **Easter weekdays outside the Octave**. The Easter Octave (GNLY 24) consists of days celebrated as Solemnities of the Lord (precedence level 2 in the Table of Liturgical Days, entry 2). No memorials are celebrated during the Octave. The expression "weekdays of the Easter Season" in GIRM 355.2 automatically excludes the Octave days because they are not weekdays.

> **Note on GIRM 355.1 and Easter:** GIRM 355.1 contains two distinct provisions: (1) the rule for privileged weekdays (Advent 17-24, Octave of Christmas, Lent) where the feria is imposed; and (2) the clarification "On weekdays of the Easter Season, memorials of Saints may rightly be celebrated fully." This second sentence confirms that Easter weekdays follow the 355.2 regime (free choice), not the restricted regime. The word "fully" means all three substitution groups follow the 355.2/355.3 rules — not just collect-borrowing.

**Lenten demotion (GNLY 14, 59 entry 12):** Obligatory Memorials that fall on Lenten weekdays may only be celebrated as Optional Memorials. They follow the same "special manner" as optional memorials on privileged weekdays (collect-borrowing only, except Ash Wednesday and Holy Week).

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
- **GNLY 3** — "The liturgical day runs from midnight to midnight. However, the celebration of Sunday and of Solemnities begins already on the evening of the previous day."
- **GNLY 10** — "Celebrations, according to the importance assigned to them, are hence distinguished one from another and termed: Solemnity, Feast, Memorial."
- **GNLY 11** — Solemnities begin with First Vespers on the preceding day; some have a proper Vigil Mass "to be used on the evening of the preceding day, if an evening Mass is celebrated." Normative basis for `PreviousEveningMass`.
- **GNLY 13** — "Feasts are celebrated within the limits of the natural day; accordingly, they have no First Vespers (Evening Prayer I), unless they are Feasts of the Lord which fall on a Sunday in Ordinary Time or in the Christmas Season and which replace the Sunday Office."
- **GNLY 14** — "Memorials are either obligatory or optional; their observance is integrated into the celebration of the occurring weekday in accordance with the norms set forth in the *General Instruction of the Roman Missal* and of the Liturgy of the Hours." Also: (1) "Obligatory Memorials which fall on weekdays of Lent may only be celebrated as Optional Memorials." (2) "If several Optional Memorials are inscribed in the Calendar on the same day, only one may be celebrated, the others being omitted."
- **GNLY 15** — "On Saturdays in Ordinary Time when no Obligatory Memorial occurs, an Optional Memorial of the Blessed Virgin Mary may be celebrated." A standing structural option generated by a general norm, not from a calendar inscription.
- **GNLY 16** — Weekdays definition and precedence rules. 16a: Ash Wednesday and Holy Week weekdays take precedence over all. 16b: Advent Dec 17-24 and Lent weekdays have precedence over Obligatory Memorials. 16c: "Other weekdays [...] are **combined with** Memorials" — the GNLY's own term for the memorial-weekday integration that this document models.
- **GNLY 24** — "The first eight days of Easter Time constitute the Octave of Easter and are celebrated as Solemnities of the Lord." These are not weekdays — no memorials are celebrated during the Octave.
- **GNLY 59** — Table of Liturgical Days according to Their Order of Precedence. Entry 12: Optional Memorials "may be celebrated, in the special manner described in the *General Instruction*, even on the days listed in no. 9" — the GNLY's authorization for collect-borrowing on privileged weekdays (cf. `ForcedCollectBorrowable`). Entry 12 also: "In the same manner Obligatory Memorials may be celebrated as Optional Memorials if they happen to fall on Lenten weekdays."
- **GNLY 61** — Vespers I/II conflict resolution: "Should Vespers of the current day's Office and First Vespers of the following day be assigned for celebration on the same day, then Vespers of the celebration with the higher rank in the Table of Liturgical Days takes precedence; in cases of equal rank, Vespers of the current day takes precedence."
- **GNLY 60** — Precedence resolution: "If several celebrations fall on the same day, the one that holds the highest rank according to the Table of Liturgical Days is observed." Impeded solemnities are transferred; other celebrations are omitted that year.

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

#### GILH (General Instruction of the Liturgy of the Hours)

> **Note:** The Liturgy of the Hours is not currently implemented in romcal. These references document architectural implications for future extensibility (see Part VIII).

- **GILH 34-36** — The Invitatory: opens the first Hour of the day (normally Office of Readings or Lauds). Consists of an invitatory antiphon + Psalm 95 (or Ps 100, 67, 24). The antiphon varies by celebration and follows the §235b priority on memorials.
- **GILH 37** — Morning Prayer (Lauds) and Evening Prayer (Vespers) are the "double hinge" of the Daily Office: the two principal Hours around which the entire Office is structured
- **GILH 53** — Concluding prayer: from the Psalter in Ordinary Time, from the Proper of the Season or of the Saints otherwise
- **GILH 55-73** — The Office of Readings: hymn, 3 psalms with antiphons, versicle, 1st reading (Scripture) + responsory, 2nd reading (patristic or hagiographical) + responsory, Te Deum (when applicable), concluding prayer. The most complex Hour in terms of content variation.
- **GILH 62** — On memorials, psalms and antiphons are taken from the current week and day of the psalter, unless proper psalms or antiphons are indicated
- **GILH 64, 67** — Office of Readings has two readings: the first from Scripture (continuous reading cycle), the second patristic. On memorials, a hagiographical reading replaces the second reading if one exists
- **GILH 68** — Te Deum: sung on solemnities, feasts, and Sundays (except in Lent); omitted on memorials and weekdays
- **GILH 73** — Vigil extension of the Office of Readings: "after the Office of Readings and before the Te Deum, canticles from the special appendix [...] are inserted, the passage from the Gospels is then read." The vigil is not a separate Hour but an extension celebrated "during the night or early morning."
- **GILH 84-92** — Compline (Night Prayer): the most stable Hour. Examination of conscience, hymn, psalm(s) with antiphon, short reading, versicle, Nunc Dimittis canticle + antiphon, concluding prayer, Marian antiphon. Nearly always from the weekday psalter — only varies on solemnities (§230: Compline after Vespers I uses Sunday scheme; after Vespers II uses Saturday Week I if no proper).
- **GILH 93-98** — Provisions for combining Hours of the Office with Mass: shared opening rite, psalm as entrance chant, single concluding rite
- **GILH 116-119** — Antiphon rules by rank: on memorials without proper antiphons, antiphons for the canticles of Zechariah and Mary may be taken from the Common or from the current psalter
- **GILH 134-135** — Psalm arrangements: on solemnities and feasts, proper psalms; on memorials and weekdays, psalms from the current psalter
- **GILH 175-178** — Daytime Prayer (Terce, Sext, Nones): hymn, 3 psalms with antiphons, short reading, versicle, concluding prayer. Two psalmody schemes: "current" (from the psalter week) and "complementary" (for those who pray all three hours). On memorials, entirely from weekday (§236). On solemnities, may have proper hymn, antiphons, reading, and prayer (§229).
- **GILH 199** — Concluding prayer: on memorials, from the saint's proper or the appropriate Common; on weekdays, from the psalter or the seasonal proper
- **GILH 206** — Vigil form structure for the Triduum: "In the celebration of the vigils, [...] after the two readings and before the canticle Te Deum, the canticles and the Gospel indicated for each case are added."
- **GILH 209** — Vespers suppression on Holy Thursday and Good Friday: those who attend the evening Mass of the Lord's Supper or the Celebration of the Lord's Passion omit Vespers.
- **GILH 211** — Compline suppression on Holy Saturday: those who attend the Easter Vigil omit Compline.
- **GILH 212** — Easter Vigil replaces the Office of Readings on Holy Saturday. For those absent, a reduced Office of Readings is provided (choosing 4 of the Vigil's OT readings). Cross-domain interaction: a Mass celebration affects the Office.
- **GILH 215** — Christmas night: when the vigil form of the Office of Readings is celebrated before Midnight Mass, Compline is omitted. Also specifies the vigil form for Christmas.
- **GILH 225-230** — How the Office is arranged on **solemnities**: everything from Proper or Common (§225); psalmody from Laudate group or Sunday Week I (§225); Te Deum said (§228); Daytime Prayer proper (§229); Compline: after Vespers I → Sunday scheme, after Vespers II → Saturday Week I or proper (§230). Solemnities begin with Vespers I the preceding evening.
- **GILH 231-233** — How the Office is arranged on **feasts**: "celebrated within the limits of the natural day" — no Vespers I (§231), except Lord's Feasts falling on Sunday (GNLY 13); Te Deum said (§231); Office of Readings as solemnities (§232); Daytime Prayer: weekday psalter + feast concluding prayer (§233); Compline from weekday (§233).
- **GILH 234** — No difference in arrangement between obligatory and optional memorials, except during privileged seasons
- **GILH 235-236** — Memorials during Ordinary Time: (a) psalms/antiphons from current weekday; (b) invitatory, hymn, short reading, canticle antiphons, intercessions from saint's Proper, or else from Common or weekday; (c) concluding prayer from the saint (mandatory); (d) Office of Readings: 1st reading from Scripture cycle, 2nd from saint/Common; Te Deum not said. Daytime Prayer and Night Prayer entirely from weekday (§236).
- **GILH 237-239** — Memorials during privileged seasons: no memorials on Sundays/solemnities/feasts/Ash Wed/Holy Week/Easter Octave (§237); obligatory memorials become optional on Advent Dec 17-24, Christmas Octave, Lent weekdays (§238); limited additions: hagiographical reading **added after** patristic (not replacing), saint's antiphon and prayer **appended** to Morning/Evening Prayer (§239)
- **GILH 240** — Optional memorial of BVM on Saturdays in Ordinary Time, celebrated as other memorials
- **GILH 244** — On weekdays with optional memorials, a saint from the Roman Martyrology may be celebrated as other memorials (parallels GIRM 355.3c)
- **GILH 249** — Interrupted continuous reading in the Office: may combine omitted parts or prefer certain readings (parallels GIRM 358)

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
}
```

#### `Celebration`

**What it is:** One liturgical celebration — an entity with a rank, a name, liturgical colors, and mass texts. The feria of Wednesday of the 5th week is a celebration. The optional memorial of St. Scholastica is another celebration.

**Why this name:** GNLY 10 defines it: "Celebrations, according to the importance assigned to them, are hence distinguished one from another and termed: Solemnity, Feast, Memorial." A celebration is the thing you celebrate, with its specific rank and proper texts.

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
    /// Each option = one possible celebration with its collect + antiphons.
    /// The consumer picks ONE option — all three texts come as a block.
    /// GNLY 14: "If several Optional Memorials are inscribed in the Calendar
    /// on the same day, only one may be celebrated, the others being omitted."
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
Type                        App 1   App 2   App 3   Scope
──────────────────────────  ──────  ──────  ──────  ──────────
DayContext                     ✓       ✓       ✓    SHARED
ReadingText                    ✓       ✓       ✓    SHARED
TextSource                     ✗       ✓       ✓    SHARED (App 2+3)
SourcedText                    ✗       ✓       ✓    SHARED (App 2+3)
CelebrationId                  ✓       ✓       ✓    SHARED

FormularySet                   ✓       ✓ ¹     ✗    MASS
ReadingsSet                    ✓       ✓ ¹     ✗    MASS
ReadingsPool                   ✓       ✓ ¹     ✗    MASS
ReadingsContent                ✓       ✓ ¹     ✗    MASS
FlexibleOrations               ✓       ✗ ²     ✗    MASS (App 1)

LiturgicalCalendar             ✓       ✗       ✗    App 1
LiturgicalDay                  ✓       ✗       ✗    App 1
Celebration                    ✓       ✗       ✗    App 1
CelebrationMass                ✓       ✗       ✗    App 1 (Mass)
CelebrationHour                ✓       ✗       ✗    App 1
CelebrationOfficeReadings      ✓       ✗       ✗    App 1 (Office)

MassCalendar                   ✗       ✓       ✗    App 2
MassComposition                ✗       ✓       ✗    App 2
IdentityOption                 ✗       ✓       ✗    App 2
ReadingsOption                 ✗       ✓       ✗    App 2
ReadingsCategory               ✗       ✓       ✗    App 2
CompositionRules               ✗       ✓       ✗    App 2
BlockRule                      ✗       ✓       ✗    App 2
ReadingsRule                   ✗       ✓       ✗    App 2
FlexibleRule                   ✗       ✓       ✗    App 2

HoursCalendar                  ✗       ✗       ✓    App 3
HoursComposition               ✗       ✗       ✓    App 3
HoursCelebrationOption         ✗       ✗       ✓    App 3
ResolvedHourContent            ✗       ✗       ✓    App 3
OfficeReadingsContent          ✗       ✗       ✓    App 3
VigilExtension                 ✗       ✗       ✓    App 3
HoursCompositionRules          ✗       ✗       ✓    App 3
MemorialRule                   ✗       ✗       ✓    App 3
HourSuppression                ✗       ✗       ✓    App 3

HourTime                       ✓       ✗       ✓    SHARED (App 1+3)
HoursPsalmody                  ✓       ✗       ✓    SHARED (App 1+3)
PsalmodyEntry                  ✓       ✗       ✓    SHARED (App 1+3)

Existing types (unchanged)     ✓       ✓       ✓    SHARED
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
│   ├── psalmody.rs                  HoursPsalmody, PsalmodyEntry
│   └── mod.rs
│
├── liturgical_calendar/             APPROACH 1
│   ├── liturgical_day.rs            LiturgicalDay
│   ├── celebration.rs               Celebration, CelebrationId
│   ├── celebration_mass.rs          CelebrationMass
│   ├── celebration_hour.rs          CelebrationHour, CelebrationOfficeReadings, HourTime
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
├── hours_calendar/                  APPROACH 3
│   ├── hours_composition.rs         HoursComposition, HourSuppression
│   ├── hours_celebration_option.rs  HoursCelebrationOption, ResolvedHourContent
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
│  2. Apply precedence rules (GNLY 59, 60) │
│  3. Assemble Celebrations per day         │
│  4. Resolve liturgical cycle              │
│  5. Populate mass content by GIRM groups  │
│  6. Populate hours content by GILH rules  │
└──────────┬────────────────────────────────┘
           │
    ┌──────┼──────────────┐
    ▼      ▼              ▼
 gen_    gen_            gen_
 lit_    mass_           hours_
 cal()   cal()           cal()
    │      │              │
    ▼      │              │
 Liturgical│              │  Hours transformation:
 Calendar  │              │  • Shift Vespers I to previous civil date
    │      │              │  • Resolve §235 overlay per celebration
    │      │              │  • Apply §239 addition logic
    │      │              │  • Compute HoursCompositionRules
    │      │              ▼
    │      │          Hours Calendar (API output)
    │      │
    │      │  Mass transformation:
    │      │  • Shift evening masses to previous civil date
    │      │  • Assemble IdentityOption from each Celebration
    │      │  • Assemble ReadingsOption with source + flags
    │      │  • Explode FlexibleOrations into Vec<SourcedText>
    └──────│  • Compute CompositionRules from season/precedence
           ▼
       Mass Calendar (API output)
```

---

## Part VIII — Liturgy of the Hours: Rules and Data Model

> **Status:** The Liturgy of the Hours is not currently implemented in romcal. This section provides the complete architectural analysis and data model for the Office, to ensure that the current design does not preclude future implementation and that the modelling is fully worked out in advance.

### 1. Office Substitution Groups (vs. Mass)

The Mass has three substitution groups (Part I §2). The Office has fundamentally **different** groups on memorials:

| Group | Mass (GIRM) | Office (GILH) |
|-------|-------------|----------------|
| **Psalmody** | Entrance/communion antiphons follow the formulary choice | Psalms + psalm antiphons ALWAYS from the current weekday psalter (GILH 62, 134) |
| **Identifying texts** | Formulary block: collect + entrance/communion antiphons (inseparable) | Proper elements: invitatory antiphon, hymn, short reading, canticle antiphons (Benedictus/Magnificat), intercessions — from saint's Proper, Common, or weekday (GILH 235b). Concluding prayer mandatory from saint (GILH 235c). |
| **Readings** | Scripture readings: fixed set or pool (GIRM 357, GILM 71) | Office of Readings: 1st reading from Scripture cycle + 2nd reading patristic or hagiographical (GILH 64, 67) |
| **Flexible orations** | Prayer over offerings, prayer after Communion: individually choosable (GIRM 363) | Not applicable — the Office has no equivalent flexible orations |

**Key architectural difference:** In the Mass, the entrance and communion antiphons follow the formulary choice (inseparable from the collect). In the Office, psalm antiphons stay with the psalter on memorials, while only the canticle antiphons (at Benedictus and Magnificat) can come from the saint. This means `FormularySet` cannot be reused for the Office — the Office needs a different structure.

### 2. Structure of Each Hour

Before analyzing how celebrations affect the Office, it is necessary to understand what elements each Hour contains. The GILH defines 7 Hours (8 counting Vespers I), each with a specific structure:

| Hour | Elements | GILH ref |
|------|----------|----------|
| **Invitatory** | Invitatory antiphon + Psalm 95 (or 100, 67, 24) | §34-36 |
| **Office of Readings** | Hymn, 3 psalms with antiphons, ℣, Scripture reading + responsory, patristic/hagiographical reading + responsory, Te Deum (when applicable), concluding prayer | §55-73 |
| **Lauds** (Morning Prayer) | Hymn, morning psalm + OT canticle + praise psalm (with antiphons), short reading + short responsory, Benedictus canticle + antiphon, intercessions, Our Father, concluding prayer | §37-54 |
| **Terce / Sext / Nones** (Daytime Prayer) | Hymn, 3 psalms with antiphons, short reading + ℣, concluding prayer | §74-83 |
| **Vespers** (Evening Prayer) | Hymn, 2 psalms + NT canticle (with antiphons), short reading + short responsory, Magnificat canticle + antiphon, intercessions, Our Father, concluding prayer | §37-54 |
| **Compline** (Night Prayer) | Examination of conscience, hymn, psalm(s) with antiphon, short reading + ℣, Nunc Dimittis + antiphon, concluding prayer, Marian antiphon | §84-92 |

**Architectural implications:**

- **Invitatory**: Precedes the first Hour of the day (normally Office of Readings or Lauds). Its antiphon varies by celebration. On memorials, it follows the §235b priority: saint's Proper → Common → weekday.
- **Lauds and Vespers** are the two "principal Hours" (§37) and have the richest variation per celebration: canticle antiphon (Benedictus/Magnificat), intercessions, hymn, short reading.
- **Office of Readings** has two readings with responsories — the most complex element affected by memorial rules (§235d, §239a).
- **Daytime Prayer** (Terce/Sext/Nones): On memorials, entirely from the weekday (§236). On solemnities, proper texts. The celebrant normally chooses ONE of the three unless bound to all three (clerics with choral obligation). The GILH (§175-178) provides two psalmody schemes: the "current" (from the psalter week) and the "complementary" (for those who pray all three).
- **Compline** is the most stable Hour: almost always from the weekday psalter, unaffected by memorials (§236). It only varies on solemnities (proper antiphon for the Nunc Dimittis) and in the exceptional suppression cases (§211, §215).

### 3. How the Office is Arranged by Rank

The GILH (Chapter IV, §225-244) defines distinct rules for each rank. This section analyzes these rules completely.

#### 3a. On Solemnities (GILH 225-230)

Solemnities receive the most complete proper treatment. **Everything** is from the Proper or Common:

| Element | Source | GILH ref |
|---------|--------|----------|
| **Vespers I** (Evening Prayer I) | Proper/Common — begins the solemnity on the preceding evening | §225 |
| **Invitatory antiphon** | Proper/Common | §225 |
| **Hymn** | Proper/Common | §225 |
| **Psalmody** | Proper, or from the Laudate group (Ps 118, 150), or from Sunday of Week I | §225 |
| **Antiphons** (psalm + canticle) | Proper/Common | §225 |
| **Short reading, short responsory** | Proper/Common | §225 |
| **Canticle antiphon** (Benedictus/Magnificat) | Proper/Common | §225 |
| **Intercessions** | Proper/Common | §225 |
| **Concluding prayer** | Proper/Common | §225 |
| **Office of Readings — 1st reading** | Proper/Common (may differ from weekday cycle) | §228 |
| **Office of Readings — 2nd reading** | Proper/Common (patristic or hagiographical) | §228 |
| **Te Deum** | **Said** | §228 |
| **Daytime Prayer** | Proper hymn, proper antiphons, proper short reading and prayer; psalmody may use the "gradual" psalms (Ps 120-128) unless proper | §229 |
| **Compline** | After Vespers I: Compline of Sunday; after Vespers II: proper if given, otherwise Saturday of Week I | §230 |

**Key points for the data model:**
- Solemnities need **full content for every Hour** — all fields populated, nothing from the weekday.
- Vespers I exists (unlike feasts and memorials) — the `VespersI` entry in `HourTime` is primarily for solemnities.
- The psalmody may be proper (unlike memorials where psalms come from the weekday psalter).
- GNLY 11: "Some Solemnities are also endowed with their own Vigil Mass" — the Mass vigil (`PreviousEveningMass`) is distinct from the Office Vespers I. Both may exist on the same evening.

#### 3b. On Feasts (GILH 231-233)

Feasts are celebrated like solemnities but with two significant restrictions:

| Difference from Solemnity | Rule | GILH ref |
|---------------------------|------|----------|
| **No Vespers I** | Feasts are "celebrated within the limits of the natural day" (GNLY 13). No Evening Prayer I. | §231 |
| **Exception: Lord's Feasts on Sundays** | When a Feast of the Lord falls on a Sunday in OT or Christmas Time, it replaces the Sunday Office — including Vespers I from Saturday evening (GNLY 13). | §231 |
| **Te Deum** | **Said** (same as solemnities) | §231 |
| **Office of Readings** | Proper/Common — same as solemnities | §232 |
| **Daytime Prayer** | From the weekday psalter unless proper hymn/texts indicated; concluding prayer from the feast | §233 |
| **Compline** | From the weekday (since no Vespers I/II distinction) | §233 |

**Key points for the data model:**
- The `VespersI` entry in `HourTime` is NOT generated for feasts (except Lord's Feasts on Sundays that replace the Sunday Office).
- Most feast content is the same as solemnities — the main difference is temporal scope.
- Daytime Prayer is simpler than on solemnities: weekday psalter base with feast concluding prayer.

#### 3c. On Memorials — Summary Table

For reference, the complete comparison by rank:

```
                        Solemnity        Feast           Memorial (OT)      Memorial (priv.)     Weekday
                        ─────────        ─────           ──────────────      ────────────────     ───────
Vespers I               Proper/Common    No ¹            No                  No                   No
Invitatory antiphon     Proper/Common    Proper/Common   §235b ²             Weekday              Weekday
Hymn                    Proper/Common    Proper/Common   §235b               Weekday + saint ⁴    Weekday
Psalmody                Proper ³         Proper ³        Weekday psalter     Weekday psalter      Weekday psalter
Psalm antiphons         Proper/Common    Proper/Common   Weekday psalter     Weekday psalter      Weekday psalter
Short reading           Proper/Common    Proper/Common   §235b               Weekday              Weekday
Short responsory        Proper/Common    Proper/Common   §235b               Weekday              Weekday
Canticle antiphon       Proper/Common    Proper/Common   §235b               Weekday + saint ⁴    Weekday
Intercessions           Proper/Common    Proper/Common   §235b               Weekday              Weekday
Concluding prayer       Proper/Common    Proper/Common   Saint (mandatory)   Saint (appended)     Weekday
OdR 1st reading         Proper/Common    Proper/Common   Scripture cycle     Scripture cycle       Scripture cycle
OdR 2nd reading         Proper/Common    Proper/Common   Saint/Common ⁵      Weekday + saint ⁶   Patristic
Te Deum                 Yes              Yes             No                  No                   No
Daytime Prayer          Proper           Wkday + prayer  Weekday (§236)      Weekday              Weekday
Compline                Special ³        Weekday         Weekday (§236)      Weekday              Weekday

¹ Except Lord's Feasts falling on Sunday (GNLY 13)
² §235b priority: Saint's Proper → Common → weekday
³ Psalms from Laudate group or Sunday Week I (§225); Compline after Vespers I/II has specific rules (§230)
⁴ §239b: Saint's antiphon and prayer appended to Lauds/Vespers
⁵ §235d: Hagiographical reading replaces patristic reading
⁶ §239a: Hagiographical reading added AFTER patristic (not replacing)
```

### 4. Memorial Rules in Ordinary Time (GILH 234-236)

> This section corresponds to the "Memorial (OT)" column in the summary table above (§3c).

GILH 234: "There is no difference in the arrangement of the Office for obligatory and optional memorials except in the case of optional memorials falling during privileged seasons."

The rules of §235-236 apply **identically** to both obligatory and optional memorials in Ordinary Time:

| Element | Source (GILH 235-236) | Flexibility |
|---------|----------------------|-------------|
| **Psalms + psalm antiphons** | Current weekday psalter (§235a) | Fixed (unless proper indicated) |
| **Invitatory antiphon, hymn, short reading** | Saint's Proper → Common or weekday (§235b) | Flexible (priority order) |
| **Canticle antiphons** (Benedictus/Magnificat) | Saint's Proper → Common or weekday (§235b) | Flexible (priority order) |
| **Intercessions** | Saint's Proper → Common or weekday (§235b) | Flexible (priority order) |
| **Concluding prayer** | From the Office of the saint (§235c) | **Mandatory** from saint |
| **Office of Readings — 1st reading** | Current Scripture cycle (§235d) | Fixed |
| **Office of Readings — 2nd reading** | Saint's proper or Common; current patristic if none exists (§235d) | From saint/Common |
| **Te Deum** | Not said (§235d) | Fixed (omitted) |
| **Daytime Prayer, Night Prayer** | Entirely from weekday (§236) | Fixed |

**Comparison with Mass:** In the Mass, the collect is part of the formulary block choice — it comes from whichever celebration is chosen. In the Office, the concluding prayer is **always mandatory from the saint** (§235c) — it is the one element that unambiguously identifies the memorial. The flexible elements (§235b) follow a priority order: saint's Proper if given, otherwise Common or weekday — unlike the Mass's flexible orations (GIRM 363) which are freely choosable between sources without priority.

### 5. Memorials during Privileged Seasons (GILH 237-239)

> This section corresponds to the "Memorial (priv.)" column in the summary table above (§3c).

The Office handles memorials during privileged seasons differently from the Mass, using an **addition** mechanism rather than a **substitution** mechanism:

**§237 — Complete exclusion:** On Sundays, solemnities, feasts, Ash Wednesday, Holy Week, and during the Easter Octave, "no regard is taken of any memorials." This parallels the Mass rule (GIRM 355.1 exception for Ash Wednesday/Holy Week).

**§238 — Demotion:** On weekdays of Advent Dec 17-24, Christmas Octave, and Lent, "obligatory memorials are not celebrated" and "are treated as optional memorials." This parallels GNLY 14 (Lenten demotion) and applies to the Office as well.

**§239 — Limited additions:** During these privileged seasons, if the celebrant wishes to mark the saint's memorial:
- **(a) Office of Readings:** A hagiographical reading may be **added after** the patristic reading (with its responsorial), not replacing it, along with the saint's concluding prayer.
- **(b) Morning/Evening Prayer:** The saint's antiphon (from Proper or Common) and prayer may be **appended** to the Hour.

**Key difference from the Mass:** In the Mass on privileged weekdays (GIRM 355.1), the collect is **borrowed** from the memorial, replacing the weekday collect — a substitution. In the Office (§239), no weekday element is removed or replaced — the saint's elements are **added alongside** the weekday elements. This is architecturally significant: the Mass model uses `ForcedCollectBorrowable` (a substitution rule), but the Office would need an "append" or "supplement" rule with no equivalent in the current `BlockRule` enum.

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
                   OVERLAY (§235)    MANDATORY (§235c)    READINGS (§235d)
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
                    (Benedictus/                         from weekday (§236)
                     Magnificat)
                  • Intercessions

  Psalms + psalm antiphons: ALWAYS from weekday psalter (§235a)
  ─────────────────────────────────────────────────────────────
```

```
PRIVILEGED WEEKDAYS — OFFICE ADDITION MECHANISM (GILH 239)
══════════════════════════════════════════════════════════════

  Base = WEEKDAY (imposed — no substitution)
  │
  ├── Office of Readings (§239a):
  │   ├── 1st reading: Scripture cycle (UNCHANGED)
  │   ├── 2nd reading: Patristic (KEPT — not replaced)
  │   └── + ADDED AFTER: hagiographical reading + responsory
  │       + Saint's concluding prayer
  │
  ├── Lauds / Vespers (§239b):
  │   ├── All elements: weekday (UNCHANGED)
  │   └── + APPENDED: saint's antiphon (Benedictus/Magnificat)
  │       + Saint's prayer
  │
  └── Other Hours: weekday only (no additions)

  ⚠ No additions at all on:
     Ash Wednesday, Holy Week, Easter Octave,
     Sundays, Solemnities, Feasts (§237)
```

```
COMPARISON: MASS vs. OFFICE ON PRIVILEGED WEEKDAYS
═══════════════════════════════════════════════════

  MASS (GIRM 355.1)                      OFFICE (GILH 239)
  ─────────────────                       ─────────────────
  Mechanism: SUBSTITUTION                 Mechanism: ADDITION
  • Collect borrowed FROM memorial        • Nothing removed from weekday
    (replaces weekday collect)            • Saint's elements ADDED ALONGSIDE
  • Rest: weekday imposed                 • Patristic reading KEPT (not replaced)
  Data model: ForcedCollectBorrowable     Data model: AdditionsOnly
```

### 7. Special Topics

#### Te Deum Rules (GILH 68, 228, 231, 235d)

The Te Deum is a hymn of praise sung after the second reading in the Office of Readings. Its presence or absence depends on the rank and season:

| Context | Te Deum | Reference |
|---------|---------|-----------|
| **Solemnities** | Said | §228 |
| **Feasts** | Said | §231 |
| **Sundays** (outside Lent) | Said | §68 |
| **Sundays** (in Lent) | **Not said** | §68 |
| **Memorials** (all seasons) | **Not said** | §235d |
| **Weekdays** (all seasons) | **Not said** | §68 |
| **Vigil form** of Office of Readings | Said (after the vigil canticles and Gospel) | §73 |
| **Easter Vigil reduced form** (absentees) | Said | §212 |

**Consequence for the data model:** The `te_deum: bool` field in `OfficeReadingsContent` is determined by the engine based on rank and season. On memorials and weekdays it is always `false`. On solemnities, feasts, and Sundays (outside Lent) it is `true`. This is a computed field, not a liturgical choice.

#### Vespers I/II Conflict Resolution (GNLY 61, GILH 225, 231)

When the evening of a day is both the conclusion of one celebration and the beginning of another (Vespers I of a solemnity), a conflict arises. GNLY 61 provides the rule:

> "Should Vespers (Evening Prayer) of the current day's Office and First Vespers (Evening Prayer I) of the following day be assigned for celebration on the same day, then Vespers (Evening Prayer) of the celebration with the higher rank in the Table of Liturgical Days takes precedence; in cases of equal rank, Vespers (Evening Prayer) of the current day takes precedence."

In practice:
- **Vespers I of a solemnity** usually wins over regular Vespers, because solemnities have higher precedence.
- **Equal rank** (rare): the current day's Vespers takes precedence.
- **Feasts have no Vespers I** (GNLY 13, §231), so no conflict arises — except Lord's Feasts falling on Sunday.

**Consequence for the data model:** In the `HoursCalendar` (Approach 3), the engine must resolve this conflict during the transformation from Approach 1:
1. Check if the following liturgical day has a Vespers I (only solemnities and Lord's Feasts on Sundays).
2. Compare precedence with the current day's Vespers per GNLY 61.
3. Generate only the winning Vespers entry for that civil date evening.
4. The losing Vespers is omitted entirely — it does not appear as an option.

This is different from the Mass model where `PreviousEveningMass` and `DayMass` of the same civil date coexist as separate entries. For the Office, only one Vespers is celebrated — there is no "pick one" mechanism. The engine makes the determination.

#### Mass-Office Choice Independence (GILH 234, GNLY 14)

A key architectural question: when both Mass and Office are celebrated on a day with optional memorials, must the **same** celebration be chosen for both?

The GILH and GIRM do not explicitly address this question. In liturgical practice:

- **GNLY 14** states: "If several Optional Memorials are inscribed in the Calendar on the same day, only one may be celebrated." This applies to the celebration as a whole — Mass and Office together constitute "celebrating" a memorial.
- **GILH 234** links the Office to the Mass norms: memorials "are integrated into the celebration of the occurring weekday in accordance with the norms set forth in the General Instruction of the Roman Missal and of the Liturgy of the Hours."
- The practical consensus is that the same celebration should be chosen for both Mass and Office on the same day — you don't celebrate St. Scholastica at Mass and the feria at the Office, or vice versa.

**Consequence for the data model:** The `default_celebration_id` in both `MassComposition` and `HoursComposition` should be consistent for the same civil date. The consumer should be informed that choosing a memorial for one mode implies choosing it for the other. This could be modeled through:
- A shared `celebration_choice_id` linking the Mass and Office entries for the same day, or
- Documentation/convention guidance for consumers.

This does not require a new type but should be noted in the consumer-facing API documentation.

#### Saturday BVM Memorial (GNLY 15, GILH 240)

GNLY 15: "On Saturdays in Ordinary Time when no Obligatory Memorial occurs, an Optional Memorial of the Blessed Virgin Mary may be celebrated."

GILH 240 confirms this applies to the Office: the Saturday BVM memorial is celebrated as other optional memorials (§235-236 rules).

**Key point:** This is a **structural option** generated by a general norm, not inscribed in a specific calendar. It exists on every OT Saturday without an obligatory memorial. The engine must generate this option automatically, adding a `HoursCelebrationOption` for the BVM memorial with content from the Common of the Blessed Virgin Mary.

This parallels the Mass model where the BVM Saturday memorial generates an `IdentityOption` and `ReadingsOption`.

#### Commons in the Office vs. Mass

The role of the Common differs between Mass and Office:

| Aspect | Mass | Office |
|--------|------|--------|
| **When used** | When no proper texts exist (GIRM 363) | When no proper texts exist (§235b, §235d) |
| **Readings** | Commons provide pools per reading position (GILM 71) — component-level choice | Commons provide a complete set per Hour — less granular |
| **Choice freedom** | "The celebrant may choose at will" (GILM 71) | Priority order: Proper → Common → weekday (§235b) |
| **Multiple Commons** | The saint's category determines the primary Common, but Common of Men and Women Saints is always available (GILM 83) | Similarly, multiple Commons may be indicated, and broader Commons are available |

**Consequence for the data model:** The `commons: Vec<CommonInfo>` field in `HoursCelebrationOption` lists the applicable Commons for that celebration, allowing the engine to resolve texts from the correct Common when the saint's Proper is absent.

### 8. Type Shareability: Mass → Office

| Type | Reusable? | Reason |
|------|-----------|--------|
| `DayContext` | **YES** | Same temporal frame: season, cycles, psalter week |
| `CelebrationId` | **YES** | Same celebration identity |
| `TextSource` | **YES** | Same provenance concept (Proper of Time, Proper of Saints, Common) |
| `SourcedText` | **YES** | Text + provenance — applies to any liturgical text |
| `ReadingText` | **YES** | Long/short form concept applies to Office readings too |
| `FormularySet` | **NO** | Mass-specific: collect + Mass antiphons. Office has no equivalent inseparable block |
| `ReadingsSet` | **NO** | Mass Liturgy of the Word ≠ Office of Readings (different structure, different sources) |
| `ReadingsPool` | **NO** | Pool-per-component logic is Mass/GILM-specific |
| `ReadingsContent` | **NO** | Enum of `ReadingsSet`/`ReadingsPool` — both Mass-specific |
| `FlexibleOrations` | **NO** | Prayer over offerings, prayer after Communion — Mass-specific |
| `CompositionRules` | **Partially** | The approach (rules governing substitution) transfers, but the specific rule enums (`BlockRule`, `ReadingsRule`, `FlexibleRule`) are Mass-specific |

### 9. Approach 1 Extension — `CelebrationHour`

**What it is:** The raw textual content that a celebration provides for one Hour of the Office. Parallels `CelebrationMass` but with Office-specific structure.

**Why raw (not resolved):** In Approach 1, each `Celebration` carries its own texts — what it *provides*, not what the celebrant *uses*. On a memorial, the saint's `CelebrationHour` contains only the proper elements the saint provides; the weekday base content is in the feria's `CelebrationHour`. The resolution (merging weekday + saint per §235 rules) happens in Approach 3.

**Why per-Hour:** The Proper of Saints distributes elements across Hours: an antiphon at the Benedictus (Lauds), an antiphon at the Magnificat (Vespers), a hagiographical reading (Office of Readings), etc. Storing them per-Hour mirrors how the celebrant actually uses them. On memorials, Daytime Prayer and Night Prayer have no entries from the saint (§236).

```rust
/// Raw content that a celebration provides for one Hour.
/// On memorials: only the saint's proper elements are populated.
/// On solemnities/feasts: fully populated.
/// The feria celebration: fully populated with weekday content.
struct CelebrationHour {
    /// Proper psalmody — rare on memorials (§235a: usually from weekday)
    /// On solemnities: proper psalms from Laudate group or Sunday Week I (§225)
    psalmody: Option<HoursPsalmody>,

    /// Elements from §235b (None = use weekday or Common instead)
    invitatory_antiphon: Option<String>,
    hymn: Option<String>,
    short_reading: Option<String>,
    short_responsory: Option<String>,
    /// Gospel canticle antiphon — at Benedictus (Lauds), Magnificat (Vespers),
    /// or Nunc Dimittis (Compline). Only one of these three canticles appears
    /// per Hour, so a single field suffices. (GILH 116-119)
    canticle_antiphon: Option<String>,
    intercessions: Option<String>,

    /// Concluding prayer — mandatory from saint on memorials (§235c)
    concluding_prayer: Option<String>,

    /// Office of Readings content — only populated for HourTime::OfficeOfReadings.
    /// On memorials: only hagiographical_reading populated (§235d).
    /// On solemnities/feasts: both readings fully populated from Proper/Common (§228).
    /// On the feria: scripture from weekday cycle, patristic from current cycle.
    office_readings_content: Option<CelebrationOfficeReadings>,
}

/// Raw Office of Readings content that a celebration provides.
/// This is the Approach 1 (unresolved) counterpart of OfficeReadingsContent (Approach 3).
struct CelebrationOfficeReadings {
    /// 1st reading: Scripture reading (from Proper on solemnities, from weekday cycle on feria)
    /// None on memorials — the weekday cycle reading applies (§235d)
    scripture_reading: Option<ReadingText>,
    scripture_responsory: Option<String>,
    /// 2nd reading: patristic reading (from Proper on solemnities, from current cycle on feria)
    /// None on memorials when replaced by hagiographical reading (§235d)
    patristic_reading: Option<ReadingText>,
    patristic_responsory: Option<String>,
    /// Hagiographical reading — in honor of the saint
    /// On memorials: replaces patristic (§235d). On §239 additions: added after patristic.
    /// On solemnities: may serve as the second reading (the reading "in honor of the saint").
    hagiographical_reading: Option<ReadingText>,
    hagiographical_responsory: Option<String>,
}
```

> **Design note:** The previous version of `CelebrationHour` had `hagiographical_reading` and `hagiographical_responsory` as top-level fields. These are now nested in `CelebrationOfficeReadings` alongside the scripture and patristic readings, which are necessary for solemnities and feasts where BOTH readings come from the celebration's Proper (§228). On memorials, only `hagiographical_reading` is populated; on the feria, `scripture_reading` + `patristic_reading` are populated. The engine in Approach 3 merges these correctly based on rank and season.

**Integration in `Celebration`:**

```rust
struct Celebration {
    // ... identity fields (name, rank, colors, commons...) ...
    masses: BTreeMap<MassTime, CelebrationMass>,
    hours: BTreeMap<HourTime, CelebrationHour>,
}
```

The same `Celebration` entity — St. Scholastica, Memorial — carries both Mass and Office texts. The identity fields are shared; only the content differs by mode.

#### `HourTime`

**What it is:** An enum identifying a specific Hour of the Daily Office, including the shifted Evening Prayer I for solemnities.

**Why `VespersI`:** GILH 225 states that solemnities begin with Evening Prayer I on the day before. This parallels `PreviousEveningMass` in the Mass model. In Approach 1 (liturgical perspective), Vespers I stays on the solemnity's `Celebration`. In Approach 3 (civil date perspective), it shifts to the previous civil date.

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

> **Note on the Invitatory (§34-36):** The Invitatory is not a separate `HourTime` because it is not an independent Hour — it introduces the first Hour of the day (normally Office of Readings or Lauds). Its antiphon is stored in the `invitatory_antiphon` field of `CelebrationHour` (Approach 1) and `ResolvedHourContent` (Approach 3), and is associated with whichever Hour comes first in the day's sequence. The Invitatory psalm (Ps 95 or its alternatives) is structural, not content that varies per celebration.

#### `HoursPsalmody`

**What it is:** The psalmody for one Hour — a sequence of psalm/canticle entries with their antiphons.

**Liturgical basis:** Each Hour has a specific psalmody structure — typically 3 psalms at Office of Readings, 2 psalms + 1 OT canticle at Lauds, 2 psalms + 1 NT canticle at Vespers, 3 psalms at Daytime Prayer, etc.

```rust
struct HoursPsalmody {
    /// Psalm/canticle entries in order
    entries: Vec<PsalmodyEntry>,
}

struct PsalmodyEntry {
    /// Psalm or canticle reference (e.g., "Ps 63", "Dan 3:57-88")
    reference: String,
    /// The text of the psalm/canticle
    text: Option<String>,
    /// Antiphon text
    antiphon: Option<String>,
}
```

### 10. Approach 3 — Hours Calendar

**Method:** `Calendar::generate_hours_calendar() → HoursCalendar`

**Principle:** Organized by civil date. Each entry is one Hour of the Office, self-contained with all options pre-resolved. Evening Prayer I of solemnities is shifted to the previous civil date. The consumer picks a celebration, and receives fully resolved content for that Hour.

**Why a separate approach (not merged with Approach 2):** The Mass and Office have fundamentally different composition patterns:

- **Mass** = **selection**: the consumer picks from options per substitution group (formulary block, readings, flexible orations). Each group has independent alternatives.
- **Office** = **overlay**: the celebration choice determines a composite content where weekday base elements and saint's proper elements are merged by the engine per §235 rules. During privileged seasons, saint's elements are *added alongside*, not substituted (§239).

These two patterns require different data structures and composition rules. Merging them would force artificial uniformity.

**Generated from Approach 1:** The engine first produces the `LiturgicalCalendar`, then transforms it into the `HoursCalendar` by: shifting Vespers I to the previous civil date, resolving which elements come from the weekday vs. the saint per §235 rules, applying §239 addition logic for privileged seasons, and computing the applicable composition rules.

#### `HoursCalendar`

**What it is:** The top-level output type. A map from civil date to a list of Hours celebrated on that civil day.

**Why this name:** It is a "calendar" organized by "hours" — the practical perspective of which Hours of the Office are celebrated on each civil day.

**Why `Vec<HoursComposition>`:** Each entry is one Hour. A typical day has up to 7 entries (Office of Readings through Compline). When a solemnity begins tomorrow, today also receives a Vespers I entry (shifted), analogous to `PreviousEveningMass` in the Mass Calendar.

```rust
type HoursCalendar = BTreeMap<String, Vec<HoursComposition>>;
```

#### `HoursComposition`

**What it is:** A single Hour of the Office with all its options pre-resolved. The consumer picks a celebration and receives the fully resolved content.

**Why this name:** "Hours" because it represents one Hour of the Office. "Composition" because the Hour is "composed" from weekday and saint elements — the consumer receives the composed result.

**Why per-Hour (not per-day):** Although the celebration choice applies to the whole day (if you celebrate St. Scholastica at Lauds, you celebrate her at Vespers too), the *content* differs per Hour (different psalms, different antiphons, different readings). And Vespers I may belong to a different celebration than the rest of the day. Per-Hour entries keep each unit self-contained, consistent with `MassComposition` in Approach 2.

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
    celebration_options: Vec<HoursCelebrationOption>,

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

#### `HoursCelebrationOption`

**What it is:** One possible celebration that can be chosen for this Hour, with its fully resolved content. The engine has already applied §235 rules — merging weekday base with saint's proper elements.

**Why this name:** It is one "option" among the available "celebrations" for this "Hour." Analogous to `IdentityOption` in the Mass model, but includes the full resolved content rather than just the formulary block.

**Why fully resolved:** Unlike the Mass where the consumer actively composes (picks readings, picks orations), the Office consumer receives a finished composite. Once the celebration is chosen, §235 determines everything. The engine does the work, the consumer picks and uses.

```rust
struct HoursCelebrationOption {
    /// Reference to the celebration
    celebration_id: CelebrationId,
    celebration_name: String,
    rank: Rank,
    precedence: Precedence,
    colors: Vec<ColorInfo>,
    commons: Vec<CommonInfo>,
    from_calendar_id: CalendarId,

    /// Fully resolved content for this Hour when this celebration is chosen.
    /// The engine has already merged weekday base + saint's proper per §235.
    content: ResolvedHourContent,
}
```

#### `ResolvedHourContent`

**What it is:** The fully resolved textual content for one Hour of the Office. Each element is paired with its source (`SourcedText`) so the consumer knows where it comes from.

**Why "Resolved":** The engine has resolved the §235 priority order: if the saint has a proper element, it appears here with `source: ProperOfSaint`; if not, it comes from the Common or weekday, and the source reflects that.

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
    concluding_prayer: SourcedText,

    /// Office of Readings content (only for HourTime::OfficeOfReadings)
    office_readings: Option<OfficeReadingsContent>,
}
```

#### `OfficeReadingsContent`

**What it is:** The readings content specific to the Office of Readings. Two or three readings depending on context.

**Why separate `patristic_reading` and `hagiographical_reading`:** This models the §235d/§239 distinction explicitly:

| Context | `patristic_reading` | `hagiographical_reading` | Behavior |
|---------|---------------------|--------------------------|----------|
| Weekday (no memorial) | Present | None | Patristic only |
| Memorial in OT (§235d) | None | Present | Hagiographical **replaces** patristic |
| Memorial in privileged season (§239a) | Present | Present | Hagiographical **added after** patristic |
| Solemnity/Feast (§225-233) | None | Present | Proper reading (honor of the saint) |

```rust
struct OfficeReadingsContent {
    /// First reading: from the Scripture continuous reading cycle
    scripture_reading: SourcedText,
    /// Patristic reading — from the current cycle or Common
    /// Present on weekdays, absent when replaced by hagiographical on memorials (§235d)
    /// Present alongside hagiographical during privileged seasons (§239a)
    patristic_reading: Option<SourcedText>,
    /// Hagiographical reading — in honor of the saint
    /// Present on memorials (§235d), during §239 additions, and on solemnities/feasts
    hagiographical_reading: Option<SourcedText>,
    /// Te Deum — on solemnities, feasts, Sundays outside Lent (GILH 68)
    /// Not said on memorials or weekdays
    te_deum: bool,
    /// Vigil extension — canticles and Gospel inserted BEFORE Te Deum (GILH 73, 206, 215)
    /// Present only when the vigil form of the Office of Readings is celebrated
    vigil_extension: Option<VigilExtension>,
}
```

#### `VigilExtension`

**What it is:** The additional elements appended to the Office of Readings when it is celebrated in its extended vigil form — canticles from the Old Testament followed by a Gospel reading.

**Why this name:** "Vigil" because this form is used at vigils (GILH §73: "celebrated during the night or early morning"). "Extension" because it extends the Office of Readings — it is not a separate Hour, but additional content inserted after the second reading and **before** the Te Deum.

**Liturgical basis:**
- **GILH §73:** "Whenever it may be desired to celebrate a longer vigil [...] after the Office of Readings and before the Te Deum, canticles from the special appendix [...] are inserted, the passage from the Gospels is then read."
- **GILH §206:** "In the celebration of the vigils, [...] after the two readings and before the canticle *Te Deum*, the canticles and the Gospel indicated for each case are added."
- **GILH §215:** On Christmas night, when the vigil form is used, Compline is omitted by those who attend.

**When present:** On solemnities (especially Easter, Christmas, Pentecost) and Sundays when the community chooses the vigil form. The Te Deum is sung *after* the vigil canticles and Gospel (§73), not before. The `te_deum: bool` field indicates whether Te Deum is said; the `vigil_extension` field adds the vigil elements that precede it.

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

**What it is:** The rules governing how memorials interact with the weekday Office. Simpler than `CompositionRules` for the Mass because the Office has less compositional flexibility — once the celebration is chosen, the content is determined by §235 rules.

**Why simpler than Mass:** The Mass has three independent rule dimensions (formulary block, readings, flexible orations). The Office has one primary rule: how the memorial interacts with the weekday content. The §235 resolution logic is handled by the engine, not exposed to the consumer.

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
    /// Engine applies §235 substitution rules (saint's elements replace weekday)
    /// (Ordinary Time)
    FreeChoice,
    /// Memorial elements may only be added alongside weekday content
    /// Engine applies §239 addition rules (hagiographical reading after patristic,
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

**Why this name:** "Hour" because it concerns a specific Hour of the Office. "Suppression" because the Hour is suppressed (not celebrated) under certain conditions — the GILH's own language (e.g., §209: "il est convenable d'omettre les Vêpres").

**Liturgical basis:**

| Paragraph | Context | Rule |
|-----------|---------|------|
| **GILH §209** | Holy Thursday / Good Friday | Those who attend the evening Mass of the Lord's Supper (Holy Thursday) or the Celebration of the Lord's Passion (Good Friday) omit Vespers. |
| **GILH §211** | Holy Saturday | Those who attend the Easter Vigil omit Compline. |
| **GILH §212** | Holy Saturday — Office of Readings | The Easter Vigil takes the place of the Office of Readings. For those who cannot attend the Vigil, a reduced Office of Readings is provided (choosing 4 of the Vigil's OT readings). |
| **GILH §215** | Christmas night | Those who celebrate the vigil form of the Office of Readings before Midnight Mass omit Compline. |

**Why `Option<HourSuppression>` (not always present):** Most Hours are never suppressed. This field is `None` for all ordinary days. It is `Some(...)` only on the handful of exceptional days listed above.

**Why not a boolean:** The suppression is *conditional* — it depends on what the person attends. The consumer needs to know *which* celebration triggers the suppression in order to inform the user correctly ("If you attend the Easter Vigil, you omit Compline").

```rust
enum HourSuppression {
    /// This Hour is omitted if the person attends the referenced Mass celebration.
    /// The Hour content is still provided for those who do NOT attend.
    /// (GILH 209: Vespers on Holy Thursday/Good Friday; GILH 211: Compline on Holy Saturday;
    ///  GILH 215: Compline on Christmas night)
    SuppressedIfAttends {
        /// The Mass celebration that triggers suppression
        mass_celebration_id: CelebrationId,
    },
    /// This Hour is entirely replaced by a Mass celebration.
    /// The `content` in `celebration_options` carries the reduced form
    /// for those who cannot attend the Mass.
    /// (GILH 212: Easter Vigil replaces Office of Readings on Holy Saturday)
    ReplacedByMass {
        /// The Mass celebration that replaces this Hour
        mass_celebration_id: CelebrationId,
    },
}
```

#### Approach 3 — Example

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
│       celebration_options: [
│           HoursCelebrationOption {                 ← feria
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
│           HoursCelebrationOption {                 ← memorial
│               celebration_id: "st_scholastica",
│               rank: OptionalMemorial,
│               content: ResolvedHourContent {
│                   psalmody: HoursPsalmody { ... },  ← same weekday psalms (§235a)
│                   hymn: SourcedText { source: Common(Virgins), ... },
│                   concluding_prayer: SourcedText {  ← mandatory from saint (§235c)
│                       source: ProperOfSaint("st_scholastica"), ...
│                   },
│                   office_readings: Some(OfficeReadingsContent {
│                       scripture_reading: SourcedText { ... },  ← same cycle (§235d)
│                       patristic_reading: None,                 ← replaced (§235d)
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
│       celebration_options: [
│           HoursCelebrationOption {                 ← feria
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
│           HoursCelebrationOption {                 ← memorial (§239 additions)
│               celebration_id: "st_john_ogilvie",
│               rank: OptionalMemorial,              ← demoted per §238
│               content: ResolvedHourContent {
│                   office_readings: Some(OfficeReadingsContent {
│                       patristic_reading: Some(...),      ← KEPT (§239a)
│                       hagiographical_reading: Some(...), ← ADDED after (§239a)
│                       ...
│                   }),
│                   concluding_prayer: SourcedText {       ← from saint (§239a)
│                       source: ProperOfSaint("st_john_ogilvie"), ...
│                   },
│                   ...
│               }
│           },
│       ],
│
│       composition_rules: HoursCompositionRules {
│           memorial: AdditionsOnly,                 ← §239 mechanism
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
│       celebration_options: [
│           HoursCelebrationOption {
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

### 11. Integration and Calendar API

```rust
impl Calendar {
    fn generate_liturgical_calendar(&self) -> LiturgicalCalendar;  // Approach 1
    fn generate_mass_calendar(&self) -> MassCalendar;              // Approach 2
    fn generate_hours_calendar(&self) -> HoursCalendar;            // Approach 3
}
```

Approach 1 remains the internal foundation. The `Celebration` struct carries both `masses` and `hours` content. Approaches 2 and 3 are generated from Approach 1 by their respective transformation pipelines.

This supports both the Roman Office and monastic propers (e.g., Benedictine, Cistercian) through the existing calendar inheritance mechanism — the same `CalendarId` chain that resolves Mass texts also resolves Office texts.

### 12. Combining Hours with Mass (GILH 93-98)

GILH 93-98 provides for combining Lauds with Morning Mass or Vespers with Evening Mass. When combined:

- The shared opening rite replaces both individual ones
- A psalm from the Hour may serve as the entrance chant
- A single concluding rite concludes both

This interaction means the `MassComposition` (Approach 2) may need a reference to the combined Hour, or a combined output type. This does not affect the current architecture but should be considered when adding Hours support.

### 13. Vigil Extension and Hour Suppression (GILH 73, 206, 209, 211, 212, 215)

Two exceptional mechanisms affect the Office during the Triduum and Christmas. Both involve **cross-domain interactions** — where a Mass celebration affects the Office.

#### A. Vigil Extension (GILH 73, 206, 215)

The Office of Readings can be celebrated in an extended **vigil form** on solemnities and Sundays. This is not a separate Hour — it is an extension appended to the Office of Readings:

1. The normal Office of Readings is celebrated (two readings)
2. **Before** the Te Deum, canticles from the special appendix are inserted (typically 3 OT canticles with antiphons)
3. A Gospel is proclaimed
4. The Te Deum is then sung (or the corresponding seasonal hymn)

The `VigilExtension` struct (added to `OfficeReadingsContent`) models this appendage. When `vigil_extension` is `Some(...)`, the consumer knows that the vigil form is available for this Office of Readings. The canticles and Gospel are provided; the consumer inserts them before the Te Deum.

**Concrete examples:**
- **Holy Saturday → Easter Vigil:** GILH §212 — the Easter Vigil replaces the Office of Readings entirely (see below for `ReplacedByMass`). But the reduced form for absentees also uses the vigil structure.
- **Christmas night:** GILH §215 — the Office of Readings in vigil form, with the Nativity canticles and Gospel, may be celebrated before Midnight Mass.
- **Pentecost, other solemnities:** When the community chooses the vigil form per §73.

**Example — Vigil form for a Solemnity:**

```
HoursComposition {
    hour_time: OfficeOfReadings,
    ...
    celebration_options: [
        HoursCelebrationOption {
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
                            PsalmodyEntry { antiphon: "...", psalm_ref: "..." },
                            PsalmodyEntry { antiphon: "...", psalm_ref: "..." },
                            PsalmodyEntry { antiphon: "...", psalm_ref: "..." },
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

#### B. Hour Suppression (GILH 209, 211, 212, 215)

On certain exceptional days, attending a Mass celebration makes a subsequent Hour of the Office redundant. The `HourSuppression` enum models these cases:

**`SuppressedIfAttends` — conditional omission:**

The Hour is provided with full content (for those who do NOT attend the Mass), but is marked as suppressible for those who DO attend.

| Day | Hour suppressed | Triggered by | Reference |
|-----|----------------|--------------|-----------|
| Holy Thursday | Vespers | Evening Mass of the Lord's Supper | GILH §209 |
| Good Friday | Vespers | Celebration of the Lord's Passion | GILH §209 |
| Holy Saturday | Compline | Easter Vigil | GILH §211 |
| Christmas night | Compline | Vigil form of Office of Readings before Midnight Mass | GILH §215 |

**`ReplacedByMass` — full replacement:**

The Hour is entirely replaced by a Mass celebration. The `content` in `celebration_options` carries a reduced form for those who cannot attend the Mass.

| Day | Hour replaced | Replaced by | Reduced form | Reference |
|-----|--------------|-------------|--------------|-----------|
| Holy Saturday | Office of Readings | Easter Vigil | 4 OT readings from the Vigil + Te Deum | GILH §212 |

**Example — Holy Saturday:**

```
HoursCalendar["2025-04-19"] → [
    HoursComposition {
        hour_time: OfficeOfReadings,
        ...
        celebration_options: [
            HoursCelebrationOption {
                celebration_id: "holy_saturday",
                content: ResolvedHourContent {
                    office_readings: Some(OfficeReadingsContent {
                        scripture_reading: ...,  ← 4 chosen OT readings (reduced form)
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
            mass_celebration_id: "easter_vigil",
        }),
        ...
    },
    HoursComposition {
        hour_time: Compline,
        ...
        suppression: Some(SuppressedIfAttends {
            mass_celebration_id: "easter_vigil",
        }),
        ...
    },
]
```

**Architectural note:** These cross-domain interactions (Mass → Office) are rare — they only occur during the Triduum and at Christmas. The `HourSuppression` field is `None` on all other days. This keeps the model clean for the 99% case while faithfully representing the exceptions.
