---
sidebar_position: 4
---

# General Instruction of the Liturgy of the Hours (GILH)

The General Instruction of the Liturgy of the Hours (GILH) provides instructions for the Divine Office (Liturgy of the Hours). This page summarizes sections relevant to Romcal.

## Structure of the Liturgy of the Hours

The Divine Office consists of:

| Hour                   | Traditional Name  | Time of Day                        |
| ---------------------- | ----------------- | ---------------------------------- |
| **Office of Readings** | Matins            | Any time                           |
| **Morning Prayer**     | Lauds             | Dawn                               |
| **Daytime Prayer**     | Terce, Sext, None | Mid-morning, Midday, Mid-afternoon |
| **Evening Prayer**     | Vespers           | Evening                            |
| **Night Prayer**       | Compline          | Before sleep                       |

## Psalter Week Cycle (§133)

The four-week cycle of the psalter is coordinated with the liturgical year:

| Occasion                      | Psalter Week |
| ----------------------------- | ------------ |
| First Sunday of Advent        | Week 1       |
| First Sunday in Ordinary Time | Week 1       |
| First Sunday of Lent          | Week 1       |
| Easter Sunday                 | Week 1       |

The cycle continues through Weeks 2, 3, 4, and then repeats.

### Romcal Implementation

Romcal calculates the psalter week:

```json
{
  "date": "2025-01-12",
  "name": "First Sunday in Ordinary Time",
  "psalter_week": "WEEK_1"
}
```

## Coordination with the Calendar

The GILH specifies how the Liturgy of the Hours adapts to different ranks:

| Rank      | Office of Readings | Morning/Evening Prayer | Daytime Prayer |
| --------- | ------------------ | ---------------------- | -------------- |
| Solemnity | Proper             | Proper                 | Proper         |
| Feast     | Proper             | Proper                 | From psalter   |
| Memorial  | Proper/Common      | Proper/Common          | From psalter   |
| Weekday   | From psalter       | From psalter           | From psalter   |

## First and Second Vespers

Solemnities (and some feasts) have two celebrations of Evening Prayer:

- **First Vespers**: Evening before the solemnity
- **Second Vespers**: Evening of the solemnity

This affects how Romcal defines the boundaries of liturgical days.

## Sources

- [Full GILH text (English)](https://www.liturgyoffice.org.uk/Resources/Rites/GILH.pdf)
