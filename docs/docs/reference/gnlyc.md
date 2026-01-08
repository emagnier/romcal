---
sidebar_position: 3
---

# General Norms for the Liturgical Year and the Calendar (GNLYC)

The General Norms for the Liturgical Year and the Calendar (GNLYC) defines the structure of the liturgical year and the rules for the calendar. This is the primary document that Romcal implements.

:::note Historical Names
This document was previously known as the "Universal Norms on the Liturgical Year and General Roman Calendar" (UNLY or UNLYGRC).
:::

## The Liturgical Year (§1-16)

### Seasons

The liturgical year consists of the following seasons:

| Season              | Duration                                       |
| ------------------- | ---------------------------------------------- |
| **Advent**          | Four weeks before Christmas                    |
| **Christmas**       | From Christmas Eve to the Baptism of the Lord  |
| **Lent**            | Ash Wednesday to Holy Thursday (exclusive)     |
| **Paschal Triduum** | Holy Thursday evening to Easter Sunday evening |
| **Easter**          | Easter Sunday to Pentecost (50 days)           |
| **Ordinary Time**   | 33-34 weeks outside the above seasons          |

### Ranks of Celebrations (§3-16)

| Rank                      | Description                                  | Examples                        |
| ------------------------- | -------------------------------------------- | ------------------------------- |
| **Solemnity**             | Greatest importance, begins at First Vespers | Easter, Christmas, Epiphany     |
| **Feast**                 | Celebrated within one day                    | Apostles, Evangelists           |
| **Memorial (Obligatory)** | Must be observed                             | Most saints                     |
| **Memorial (Optional)**   | May be observed                              | Some saints, local celebrations |

## Table of Precedence (§59)

The GNLYC provides a table of 13 levels of precedence that determines which celebration takes priority when multiple fall on the same day:

1. Easter Triduum
2. Christmas, Epiphany, Ascension, Pentecost; Sundays of Advent, Lent, Easter; Ash Wednesday; weekdays of Holy Week; days within Easter Octave
3. Solemnities of the Lord, Mary, and saints in General Calendar; All Souls
4. Proper solemnities
5. Feasts of the Lord in General Calendar
6. Sundays of Christmas and Ordinary Time
7. Feasts of Mary and saints in General Calendar
8. Proper feasts
9. Weekdays of Advent (Dec 17-24); days within Christmas Octave; weekdays of Lent
10. Obligatory memorials in General Calendar
11. Proper obligatory memorials
12. Optional memorials
13. Weekdays of Advent (before Dec 17), Christmas, Easter, and Ordinary Time

### Romcal Implementation

Romcal assigns a `precedence` value to each celebration:

```json
{
  "date": "2025-04-20",
  "name": "Easter Sunday",
  "precedence": 1,
  "rank": "SOLEMNITY"
}
```

## Occurrence and Concurrence

### Occurrence (§60)

When two celebrations fall on the same day:

- The celebration with higher precedence is observed
- The impeded celebration is either transferred or omitted

### Concurrence

When First Vespers of one celebration coincides with Second Vespers of another:

- Vespers of the higher-ranking celebration is observed

## Transfer of Celebrations

Certain solemnities that are impeded are transferred:

- To the nearest day not listed in §1-8 of the precedence table
- Following specific rules in §5 and §59

## Sources

- [Full GNLYC text (English)](https://www.liturgyoffice.org.uk/Resources/GIRM/Documents/GNLY.pdf)
- [Older version: UNLY](https://www.liturgyoffice.org.uk/Calendar/Info/GNLY.pdf)
