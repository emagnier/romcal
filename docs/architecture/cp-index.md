---
title: 'CP — AI Navigation Index'
description: Chapter-by-chapter summary of the Instruction on Particular Calendars (Calendaria Particularia), highlighting sections relevant to romcal's implementation and future development.
---

:::note[Purpose]
This index helps quickly locate the relevant rules in the
[CP reference document](../reference/cp.md) (~350 lines, §1–50).
Each chapter is summarized with its key paragraphs and their relevance to
romcal's implementation and future features. Use the § numbers to jump
to the source text.
:::

:::tip[When to use this index]
Consult it when implementing calendar inheritance, particular calendar
composition, patron/title handling, rank elevation, proper text
requirements, or the cross-domain prayer identity rule (§44). The CP
extends the GNLY's calendar rules with detailed norms for diocesan and
religious calendars. Read the full CP source when you need exact wording
or enumerated sub-items.
:::

---

## Quick Lookup — romcal concept → CP §§

| romcal concept                      | CP §§      | Notes                                                                 |
| ----------------------------------- | ---------- | --------------------------------------------------------------------- |
| Calendar inheritance hierarchy      | §13–16     | General → national → diocesan → church; religious parallel            |
| Particular calendar formation       | §13        | "Formed by insertion of particular celebrations into General"         |
| National / regional calendar        | §14        | Includes proper + indult celebrations above General Calendar          |
| Diocesan calendar composition       | §15        | General + national + diocesan celebrations                            |
| Religious calendar composition      | §16        | General + order/congregation celebrations                             |
| Cross-layering (religious + local)  | §16d       | Religious join local Church for cathedral + principal patrons         |
| Proper celebrations by scope        | §7–12      | Region (§8), diocese (§9), town (§10), church (§11), religious (§12)  |
| Default rank per celebration type   | §8–12, §24 | Patron=feast/solemnity, dedication=solemnity, memorial, etc.          |
| Rank elevation                      | §25        | More specific calendar can raise rank                                 |
| Patron rules                        | §28–33     | Only saints (not blessed); one principal patron (§31)                 |
| Church titles                       | §34        | One title per church; title = solemnity (§11)                         |
| BVM title date resolution           | §35        | Aug 15 or another fitting Marian date                                 |
| Precedence: particular vs. General  | §23        | Solemnity wins, feast kept, proper memorial > optional                |
| Proper of Seasons precedence        | §2         | Temporal cycle always intact; Lent/Octave/Dec 17–31 guarded           |
| One celebration per saint per year  | §3         | Exception: translatio/inventio as optional memorial                   |
| Overburdening guard                 | §17        | Only significant saints for entire diocese/institute                  |
| Proper day assignment               | §21–23     | Birthday rule, indult placement, harmonization with General           |
| Proper Mass texts (structure)       | §39–42     | Entrance/communion antiphons, orations, preface, readings             |
| Collect identifies the saint        | §40b       | Only the opening prayer has "direct bearing on the saint"             |
| Proper Office texts (structure)     | §43–44     | Hagiographical reading, antiphons, hymns, intercessions               |
| Hagiographical reading requirements | §43        | Required for every solemnity/feast/memorial; ≤120 words               |
| Biographical note (not read aloud)  | §43        | Preliminary sketch; separate from the office reading                  |
| **Cross-domain prayer identity**    | **§44**    | **"The prayer is always the same as the opening prayer of the Mass"** |
| Titles of saints                    | §27        | Suppressed titles; retained: Apostle, Martyr, Virgin, Bishop…         |
| Solemnities with suppressed holyday | §36–37     | Kept on General Calendar date; may propose alternate to Rome          |
| Rogation / Ember Days               | §38        | Conference decides time, number, purpose, votive Masses               |
| Privileges and indults              | §48–50     | Conflicting ones revoked; non-conflicting continue                    |

---

## At a Glance

| Chapter | §§        | Relevance    | Key topics                                                                                         |
| ------- | --------- | ------------ | -------------------------------------------------------------------------------------------------- |
| **I**   | **1–6**   | **High**     | **General principles: Proper of Seasons precedence, one celebration per saint, calendar approval** |
| **II**  | **7–27**  | **Critical** | **Proper celebrations, calendar hierarchy, rank, precedence, titles**                              |
| **III** | **28–38** | **High**     | **Patrons, church titles, suppressed holydays, Rogation/Ember**                                    |
| **IV**  | **39–47** | **Critical** | **Proper Mass and Office texts; §44 cross-domain prayer identity**                                 |
| V       | 48–50     | Low          | Privileges and indults (administrative)                                                            |

---

## Chapter I (§1–6) — General Norms

Fundamental principles governing particular calendars.

- **§1:** Individual Churches and religious institutes honor their saints;
  particular calendars coordinate these with the general cycle.

- **§2 — Proper of Seasons precedence** (three guards):
  - **(a)** No permanent particular celebration on Sundays.
  - **(b)** Lent, Easter Octave, and Dec 17–31 kept free of particular
    celebrations — exceptions: optional memorials, Table of Liturgical Days
    no. 8 a–d feasts, non-transferable solemnities.
  - **(c)** Indult celebrations must not duplicate the mystery-of-salvation
    cycle or be too numerous.

  Reinforces GNLY §50 and maps to romcal's precedence guards.

- **§3 — One celebration per saint per year.** Exception: a second
  celebration (optional memorial) for _translatio_/_inventio_ of patron or
  founder body, or a special life event (e.g., conversion). Monthly/weekly
  remembrances are suppressed.

- **§4:** Calendar revision requires theological, historical, and pastoral
  research commission. Not directly modeled, but relevant context for
  understanding why particular calendars vary.

- **§5:** After diocesan revision, Ordinaries must also revise calendars of
  individual churches and religious provinces under their jurisdiction.

- **§6:** Administrative submission requirements (3 typed copies, etc.).
  Not relevant to romcal.

## Chapter II (§7–27) — Proper Celebrations and Calendars

**The most critical chapter for romcal's calendar inheritance model.**

### a. Particular Celebrations (§7–12)

Defines what celebrations belong to each calendar scope and their default
ranks. This is the normative basis for romcal's `CalendarScope` and the
rank defaults in particular calendar definitions.

- **§7:** Proper celebrations = _ipso iure_ or by indult.

- **§8 — Region/nation/wider area:**
  - Feast of the principal patron (may be solemnity for pastoral reasons)
  - Memorial of secondary patron
  - Other saints/blessed with special relationship to the area

- **§9 — Diocese:**
  - Feast of the principal patron (may be solemnity)
  - Feast of cathedral dedication anniversary
  - Memorial of secondary patron
  - Saints/blessed belonging to the diocese (origin, residence, death, _cultus_)

- **§10 — Town/city:**
  - Solemnity of principal patron
  - Memorial of secondary patron

- **§11 — Individual church:**
  - Solemnity of dedication anniversary (if consecrated)
  - Solemnity of title
  - Memorial of saint/blessed whose body is in the church

- **§12 — Religious institute:**
  - **(a) Entire institute:** solemnity/feast of title, canonized founder,
    principal patron; feast of beatified founder; memorial of secondary
    patron; celebrations of member saints/blessed.
  - **(b) Individual provinces:** feast of title/principal patron; memorial
    of secondary patron; celebrations of saints/blessed with provincial ties.
  - **Constraint:** Only one of title/founder/principal patron as solemnity;
    others as feasts. Choice belongs to supreme religious authority.

### b. Particular Calendar and Celebrations to Be Included (§13–20)

Calendar hierarchy and composition rules — the normative basis for romcal's
calendar inheritance chain.

- **§13 — Calendar formation principle:** "A particular calendar is formed
  by the insertion of particular celebrations into the General Calendar."
  This is the architectural foundation of romcal's layered calendar model.

- **§14 — National/regional calendar:** Adds to General Calendar the
  celebrations (proper + indult) not in it, or at higher rank.

- **§15 — Diocesan calendar:**
  - **(a)** Each diocese has its own calendar.
  - **(b)** Built by adding to General Calendar: national + diocesan
    celebrations.
  - **(c)** Individual churches/oratories without their own religious
    calendar follow the diocesan calendar with their proper additions.

- **§16 — Religious calendar:**
  - **(a)** Who has one: orders of men (+ affiliated nuns/sisters/tertiaries),
    pontifical-rank congregations/societies/institutes.
  - **(b)** Built by inserting proper + indult celebrations into General Calendar.
  - **(c)** Province/church calendars build on the religious calendar.
  - **(d) Cross-layering rule:** "Members of religious institutes join with
    the local Church in celebrating the anniversary of the dedication of the
    cathedral and the feast of the principal patrons of both the place and
    the wider area in which they reside." This creates a cross-reference
    between the religious and diocesan calendar chains.

- **§17 — Overburdening guard:** When many saints/blessed, only the most
  significant get individual celebrations in the full calendar. Others
  restricted to local places or collective celebrations. Applies with
  necessary modifications to national calendars.

- **§18:** Historical authenticity required; consult experts.

- **§19:** Expunge saints about whom little is known historically.

- **§20:** After diocese boundary changes, retain only saints of general
  significance for the new diocese.

### c. Proper Day for Celebration (§21–23)

- **§21 — Date assignment:** Death date preferred. If unknown: discovery,
  exhumation, transfer, or canonization date. Otherwise, a free date.
  Traditional dates retained when closely linked to devotion/custom.

- **§22:** Indult celebrations assigned to a pastorally suitable date.

- **§23 — Harmonization with the General Calendar** (precedence rules for
  particular vs. universal celebrations):
  - **(a)** General Calendar **solemnities** on the same date are always
    observed (particular celebration yields).
  - **(b)** General Calendar **feasts** are kept; proper feast on same date
    is **transferred** to nearest free date. Exception: local custom too
    strong to move.
  - **(c)** A **proper memorial** takes precedence over a universal optional
    memorial. In some cases, it may even take precedence over a universal
    **obligatory** memorial — either by demoting the universal to optional,
    or by transferring it to a later date.

  These rules extend GNLY §59–60 for the particular calendar context.

### d. Rank of Celebrations (§24–26)

- **§24:** _Ipso iure_ solemnities and feasts follow the Table of Liturgical
  Days (§8–12). Other proper celebrations → obligatory or optional memorials
  by default. Optional memorials are explicitly endorsed as advantageous for
  calendar flexibility.

- **§25 — Rank elevation:** "The observance of some celebrations in a
  particular place may have greater solemnity than in the entire diocese or
  religious institute." A more specific calendar can override the rank
  inherited from a parent calendar.

- **§26:** Saints listed together: celebrated together if equal rank;
  higher rank celebrated alone, others omitted or transferred as obligatory
  memorial. Parallels GNLY §57.

### e. Titles of the Saints (§27)

- **§27 — Suppressed and retained titles:**
  - Suppressed: "Confessor and Bishop," "Confessor, Nonbishop," "Neither
    Virgin nor Martyr," "Widow."
  - Retained categories: (a) received usage — Apostle, Evangelist, Martyr,
    Virgin; (b) hierarchy — Bishop/Pope, Priest, Deacon; (c) religious —
    Abbot/Monk, Religious.
  - Lay saints: no special title in General Calendar; particular calendars
    may use designations suggesting state in life (King, Father, Mother…).

  This maps to romcal's `Title` enum and `TitleCategory` classification.

## Chapter III (§28–38) — Celebrations in Particular

### a. Patrons and Titles (§28–35)

- **§28:** Only saints (not blessed) as patrons without apostolic indult.
  Divine Persons excluded.
- **§29:** Liturgical celebration as patron only for saints chosen by
  ancient usage or immemorial tradition.
- **§30:** Patron choice by clergy and people, approved by bishop,
  confirmed by Congregation.
- **§31 — One principal patron:** "From now on there is to be only one
  principal patron." A secondary patron may be added. Two+ principal patrons
  allowed only if listed together in the calendar.
  Mapped to romcal's `PatronRole::Principal` / `PatronRole::Secondary`.
- **§32:** Former patrons chosen for obsolete reasons no longer honored.
- **§33:** A new patron may be chosen when _cultus_ has ceased or nothing
  is known about the current patron.
- **§34 — Church titles:** Allowed: Trinity, Lord (under liturgical mystery/
  name), BVM (under liturgical designation), Angels, canonized Saints.
  One title per church (exception: saints listed together).
- **§35 — BVM title dates:** If not in General or particular calendar,
  observed on Aug 15 or a more fitting Marian date. Same for Lord's titles.

### b. Solemnities with the Holyday Precept Suppressed (§36–37)

- **§36:** Kept on their General Calendar date even when holyday precept is
  abolished. Conference may propose an alternate date to the Apostolic See.
- **§37:** When assigned an alternate date, listed in particular calendars on
  that date. Relevant for romcal's holyday-of-obligation configuration.

### c. Rogation and Ember Days (§38)

- **§38:** Conference decides time, number, purpose, and votive Masses.
  Parallels GNLY §45–47.

## Chapter IV (§39–47) — Revision of Propers of Masses and Offices

**Critical for understanding what proper texts exist and how they are
structured.** Defines the text blocks that romcal models in
`FormularySet` (Mass) and `CelebrationHour` (Office).

### a. Propers of Masses (§39–42)

- **§39:** Distinction between Missal texts and Lectionary texts.

- **§40 — Mass proper text structure:**
  - **(a)** Entrance antiphon: directs thoughts to the celebration; must work
    when recited (not sung). Communion antiphon: expresses communion's place
    in the eucharistic mystery.
  - **(b)** Opening prayer (collect): "only [the text with] direct bearing
    on the saint." Prayer over the gifts and prayer after Communion bear on
    the eucharistic mystery — saint mentioned only incidentally. Solemn
    blessing / prayer over the people: optional.
  - **(c)** Preface: proper thanksgiving theme; literary form of praise (not
    petition). Included with the Mass text to which it belongs.

  Maps directly to romcal's `FormularySet` fields.

- **§41 — Readings constraints for proper Masses:**
  - Solemnities: **3 readings**.
  - No OT reading during Easter season.
  - Proper readings must have proper responsorial psalm and proper
    acclamation / Gospel verse.

- **§42:** Commons in the reformed Missal/Lectionary may replace proper texts
  lacking spiritual/pastoral importance or antiquity.

### b. Propers of Offices (§43–44)

- **§43 — Hagiographical reading:**
  - Required for **every** solemnity, feast, and memorial.
  - Sources: Fathers/ecclesiastical writers, saint's own writings,
    or biographical texts about their spiritual life/apostolate.
  - **≤120 words** typical length; avoid generalities, delete false/odd content.
  - **Biographical note:** preliminary sketch; "not to be read as part of
    the office." Modeled as a separate field in romcal
    (`CelebrationOfficeReadings.biographical_note`).
  - Appropriate responsory (proper or from Common) accompanies the reading.

- **§44 — Cross-domain prayer identity rule:**

  > "The prayer is always the same as the opening prayer of the Mass."

  This is the normative basis for romcal's shared `Celebration.prayer` field
  serving both Mass collect and Office concluding prayer. Other proper Office
  elements: invitatory antiphon, antiphons (especially Lauds/Vespers),
  intercessions, and hymns (existing proper hymns may be kept).

  **Exception — Compline:** GILH §198 overrides CP §44 for Night Prayer:
  "the prayer is always the prayer given in the psalter for that hour."
  The engine must never resolve Compline's concluding prayer from
  `Celebration.prayer`.

### c. Format of Offices and Masses (§45–47)

- **§45:** Use the _editio typica_ as format guide.
- **§46:** National celebrations printed in sequence with General Calendar;
  local celebrations in an appendix.
- **§47:** Indicate chant melodies and substitution options.

  Not directly modeled in romcal.

## Chapter V (§48–50) — Liturgical Privileges and Indults

- **§48:** Privileges conflicting with new norms are revoked; others
  continue but must be revised.
- **§49–50:** Administrative listing and submission requirements.

  Not directly modeled in romcal.

---

## Cross-References to Other Liturgical Documents

The CP does not operate in isolation. For a complete picture, these related
documents provide complementary rules:

- **GNLY** ([index](./gnly-index.md) · [source](../reference/gnly.md)):
  §48–55 calendar composition; §59 Table of Precedence; §50 proper
  celebrations rules. The CP extends and details these GNLY norms.
- **GIRM** ([index](./girm-index.md) · [source](../reference/girm.md)):
  §355 choice of Mass (optional memorials); §363 orations. The CP §40
  complements GIRM by specifying what makes a text "proper."
- **GILH** ([index](./gilh-index.md) · [source](../reference/gilh.md)):
  §198 Compline prayer exception to CP §44; §235 memorial overlay rules.
- **GILM** ([source](../reference/gilm.md)):
  Lectionary structure. CP §41 imposes readings constraints (3 for
  solemnities, no OT in Easter) that align with GILM norms.
