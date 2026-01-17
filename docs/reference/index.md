---
sidebar_position: 1
---

# Liturgical Reference

This section provides reference documentation for the official liturgical documents that define the rules implemented in Romcal.

## Official Documents

The Catholic Church's liturgical calendar is governed by several official documents promulgated by the Congregation for Divine Worship and the Discipline of the Sacraments (now the Dicastery for Divine Worship):

### [General Instruction of the Roman Missal (GIRM)](./girm)

The detailed document that provides the rubrics for the celebration of the Mass, including:

- Liturgical colors
- Order of precedence for celebrations
- Norms for combining celebrations

### [General Norms for the Liturgical Year and the Calendar (GNLYC)](./gnlyc)

The document that defines the structure of the liturgical year:

- Liturgical seasons and their duration
- Ranks of celebrations (solemnities, feasts, memorials)
- Table of precedence for liturgical days
- Rules for concurrence and occurrence

### [General Instruction of the Liturgy of the Hours (GILH)](./gilh)

Instructions for the Divine Office:

- Structure of the Liturgy of the Hours
- Four-week psalter cycle
- Coordination with the liturgical year

## How Romcal Uses These Documents

Romcal implements the rules defined in these documents to:

1. **Calculate dates**: Determine when moveable feasts occur based on Easter
2. **Resolve conflicts**: Apply precedence rules when multiple celebrations fall on the same day
3. **Assign properties**: Set liturgical colors, ranks, and other metadata
4. **Generate cycles**: Calculate Sunday/weekday cycles and psalter weeks

## Sources

- [GIRM (English)](https://www.liturgyoffice.org.uk/Resources/GIRM/Documents/GIRM.pdf)
- [GNLYC (English)](https://www.liturgyoffice.org.uk/Resources/GIRM/Documents/GNLY.pdf)
- [GILH (English)](https://www.liturgyoffice.org.uk/Resources/Rites/GILH.pdf)
