---
title: Liturgical Reference
---

The Catholic Church's liturgical calendar follows official norms from the Dicastery for Divine Worship. Romcal implements these rules to calculate dates, resolve conflicts, and assign liturgical properties.

:::note[Selection Criteria]
This page lists documents that contain **calendar calculation rules** or **reference data** that Romcal implements. Many important liturgical documents (ceremonial instructions, translation norms, sacramental rites) fall outside this scope and are intentionally not included.
:::

## Core Documents

These documents are essential for Romcal's calendar engine. Without them, the core functionality cannot work correctly.

### [Universal Norms on the Liturgical Year and the Calendar (GNLY)](./gnly)

The foundational document for Romcal. Defines the **liturgical seasons** (Advent, Christmas, Lent, Easter, Ordinary Time), **celebration ranks** (solemnity, feast, memorial), and the critical **Table of Precedence** (§59) used to resolve conflicts when multiple celebrations fall on the same day. Also specifies rules for **transferring solemnities** and the proper day for each celebration.

### [Instruction on Particular Calendars (CP)](./cp)

Instruction _Calendaria Particularia_ from the Congregation for Divine Worship (1970). Extends the GNLY with norms for **particular calendars** (diocesan, national, religious institutes). Defines rules for **patron saints**, **proper celebrations**, **rank assignments**, and **liturgical privileges**. Essential for Romcal's handling of calendars beyond the General Roman Calendar.

### Dicastery Decrees

Individual decrees from the Dicastery for Divine Worship are the **living source** for calendar updates: inscriptions of new saints, rank elevations, date changes, and removals. Unlike the documents above, these are not a single text to read but an ongoing stream of authoritative changes that Romcal's data must track.

**Official source**: [Dicastery for Divine Worship](https://www.vatican.va/roman_curia/congregations/ccdds/index.htm) on vatican.va — individual decrees in multiple languages.

**Chronological index**: [GCatholic](https://gcatholic.org/documents/data/curia-d05.htm) maintains a historical list of all documents from the Dicastery and its predecessors (Congregation of Rites, Congregation for Divine Worship).

## Extended Scope Documents

These documents govern aspects of the liturgy that Romcal partially implements or may extend in the future.

### [General Instruction of the Roman Missal (GIRM)](./girm)

The rubrics for celebrating Mass. Romcal uses the GIRM for **liturgical colors** assignment (§346), **Mass formulary** selection rules, and **regional adaptations** (e.g., Epiphany/Ascension on Sunday). Covers the structure of the Mass, ministries, sacred vessels, vestments, and adaptations by Bishops' Conferences.

### [General Introduction to the Lectionary for Mass (GILM)](./gilm)

Introduction to the Lectionary for Mass. Provides the **Order of Readings** arrangement throughout the liturgical year, the **Sunday cycle** (A, B, C), and the **weekday cycle** (I, II). Romcal outputs cycle information for each liturgical day.

### [General Instruction of the Liturgy of the Hours (GILH)](./gilh)

Instructions for the Divine Office (Liturgy of the Hours). Romcal uses the GILH for **psalter week** calculations (§133). The document also defines how celebrations affect the structure of the Office (proper antiphons, readings, hymns).

## Complementary Documents

These documents clarify and expand on the primary norms. They provide valuable context for understanding the liturgical year but do not introduce new calendar calculation rules beyond what the primary documents establish.

### [Preparation and Celebration of the Easter Feasts (PS)](./ps)

Circular letter _Paschalis Sollemnitatis_ from the Congregation for Divine Worship (1988). Clarifies and complements the norms for **Lent**, **Holy Week**, the **Easter Triduum**, and **Easter Time**. While the calendar computation rules for these periods are already in the GNLY, this document provides pastoral guidance on timing, roles of ministers, and liturgical elements.

## Data Sources

These are not normative documents but reference data that Romcal uses.

### Martyrologium Romanum

The official list of saints and blessed recognized by the Church, with their liturgical dates. Romcal uses this as the authoritative source for sanctoral entries. The current edition (2004, with supplements) contains over 7,000 entries.

### Notitiae Responses

The Dicastery's journal _Notitiae_ publishes responses to _dubia_ (formal questions) that clarify edge cases in the GNLY: ambiguous transfers, impeded solemnities, vigil rules, etc. Useful when debugging unusual calendar scenarios.
