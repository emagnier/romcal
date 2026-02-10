---
title: Glossary
---

Definitions of terms used in Romcal and Catholic liturgy.

## Abbreviations

### GILH

See [General Instruction of the Liturgy of the Hours](#general-instruction-of-the-liturgy-of-the-hours).

### GILM

See [General Introduction to the Lectionary for Mass](#general-introduction-to-the-lectionary-for-mass).

### GIRM

See [General Instruction of the Roman Missal](#general-instruction-of-the-roman-missal).

### GNLY

See [Universal Norms on the Liturgical Year and the Calendar](#universal-norms-on-the-liturgical-year-and-the-calendar).

### GNLYC

See [Universal Norms on the Liturgical Year and the Calendar](#universal-norms-on-the-liturgical-year-and-the-calendar).

### PS

See [Paschalis Sollemnitatis](#paschalis-sollemnitatis).

### UNLY

See [Universal Norms on the Liturgical Year and the Calendar](#universal-norms-on-the-liturgical-year-and-the-calendar).

---

## C

### Celebration

A liturgical or sacramental event, typically consisting of rituals, readings, prayers, and often the Eucharist (Mass), whether or not it is associated with a liturgical day.

:::note
Currently, Romcal focuses on liturgical days rather than individual celebrations. The ability to list masses and hours for a given day may be added in future versions.
:::

---

## D

### Divine Office

See [Liturgy of the Hours](#liturgy-of-the-hours).

---

## G

### General Instruction of the Liturgy of the Hours

The General Instruction of the Liturgy of the Hours (GILH) is the document that provides the general instructions for the Liturgy of the Hours. It is an official liturgical document promulgated by the Congregation for Divine Worship.

See: [GILH](./reference/gilh.md)

### General Introduction to the Lectionary for Mass

The General Introduction to the Lectionary for Mass (GILM) is the introductory document to the Order of Readings for Mass (_Ordo Lectionum Missae_). It explains the theological foundations of Scripture proclamation in the liturgy, the structure of the Liturgy of the Word, and the principles for arranging readings throughout the liturgical year.

See: [GILM](./reference/gilm.md)

### General Instruction of the Roman Missal

The General Instruction of the Roman Missal (GIRM) is the detailed document that provides the rubrics for the celebration of the Mass. It is an official liturgical document promulgated by the Congregation for Divine Worship.

See: [GIRM](./reference/girm.md)

### General Norms for the Liturgical Year and the Calendar

See [Universal Norms on the Liturgical Year and the Calendar](#universal-norms-on-the-liturgical-year-and-the-calendar).

---

## L

### Liturgical Color

The colors used in liturgical vestments and altar cloths for the celebration of Mass and other liturgical services. Colors signify the nature of the liturgical season or feast.

| Color  | Usage                                                       |
| ------ | ----------------------------------------------------------- |
| White  | Joy, purity (Christmas, Easter, saints who are not martyrs) |
| Red    | Holy Spirit, martyrs, Passion                               |
| Green  | Ordinary Time                                               |
| Violet | Penance (Advent, Lent)                                      |
| Rose   | Gaudete Sunday, Laetare Sunday                              |
| Black  | Funerals (optional)                                         |
| Gold   | Solemn occasions (optional, in place of white/red/green)    |

See: [GIRM §346](./reference/girm.md)

### Liturgical Day

A specific day within the Church's liturgical calendar, characterized by particular celebrations and observances. Each day is assigned a [rank](#rank), specific [liturgical colors](#liturgical-color), and various properties.

A liturgical day:

- Generally aligns with a Gregorian calendar day
- May begin at First Vespers or Vigils (if observed), or at midnight
- Ends at the beginning of the following liturgical day
- Is identified by its Gregorian date (considering the majority of the day)

In Romcal, this is the primary unit of output from `generateCalendar()`.

### Liturgical Period

In addition to liturgical seasons, Romcal provides metadata about specific periods:

- Octaves of Christmas and Easter
- Period before and after Epiphany
- Holy Week
- Period from Baptism of Christ to Presentation of Jesus
- Period from Presentation to Holy Thursday

### Liturgy of the Hours

The Liturgy of the Hours (Divine Office) is a set of prayers prescribed by the Catholic Church to be recited at specific times:

| Hour               | Traditional Name  | Time                               |
| ------------------ | ----------------- | ---------------------------------- |
| Office of Readings | Matins            | Any time                           |
| Morning Prayer     | Lauds             | Dawn                               |
| Daytime Prayer     | Terce, Sext, None | Mid-morning, Midday, Mid-afternoon |
| Evening Prayer     | Vespers           | Evening                            |
| Night Prayer       | Compline          | Before sleep                       |

The structure aims to fulfill St. Paul's exhortation to "pray without ceasing" (1 Thessalonians 5:17).

See: [General Instruction of the Liturgy of the Hours](#general-instruction-of-the-liturgy-of-the-hours)

---

## P

### Paschalis Sollemnitatis

_Paschalis Sollemnitatis_ (Latin: "Of the Paschal Solemnity") is a circular letter from the Congregation for Divine Worship (January 16, 1988) concerning the preparation and celebration of the Easter feasts. It recalls and clarifies norms for Lent, Holy Week, the Easter Triduum, and Easter Time.

See: [PS](./reference/ps.md)

### Precedence

The principle used to determine which liturgical celebration should be observed when two or more celebrations fall on the same day. Based on the liturgical rank of celebrations.

See: [GNLY §59](./reference/gnly.md#table-liturgical-days)

### Proper of Saints

The Proper of Saints (Sanctoral) consists of the fixed feasts celebrated on the same date each year, including Christmas and all saints' days.

### Proper of Time

The Proper of Time (Temporal) consists of the moveable feasts keyed to Easter (which falls on a different Sunday every year), including Ascension, Pentecost, and related celebrations.

### Psalter Week Cycle

The four-week cycle of the psalter, coordinated with the liturgical year:

| Event                         | Psalter Week |
| ----------------------------- | ------------ |
| First Sunday of Advent        | Week 1       |
| First Sunday in Ordinary Time | Week 1       |
| First Sunday of Lent          | Week 1       |
| Easter Sunday                 | Week 1       |

The cycle then proceeds through Weeks 2, 3, 4, and repeats.

See: [GILH §133](./reference/gilh.md)

---

## R

### Rank

The rank of a liturgical celebration indicates its importance in the liturgical calendar and determines how conflicts are resolved.

| Rank                  | Description                                    |
| --------------------- | ---------------------------------------------- |
| Solemnity             | Highest rank (Easter, Christmas, major feasts) |
| Sunday                | Sunday celebrations                            |
| Feast                 | Secondary feasts (apostles, evangelists)       |
| Memorial (Obligatory) | Must be observed                               |
| Memorial (Optional)   | May be observed                                |
| Weekday               | Ordinary weekdays                              |

See: [GNLY §3-16](./reference/gnly.md#title-i-liturgical-days)

---

## S

### Sanctoral

See [Proper of Saints](#proper-of-saints).

### Season

Liturgical seasons are periods marked by particular liturgical colors and focus:

| Season          | Description                               |
| --------------- | ----------------------------------------- |
| Advent          | Four weeks preparing for Christmas        |
| Christmas       | From Christmas Eve to Baptism of the Lord |
| Lent            | Ash Wednesday to Holy Thursday            |
| Paschal Triduum | Holy Thursday to Easter Sunday evening    |
| Easter          | Easter Sunday to Pentecost (50 days)      |
| Ordinary Time   | 33-34 weeks outside the above seasons     |

### Sunday Cycle

A three-year cycle for Sunday Mass readings (and some solemnities), designated A, B, or C.

| Year | Gospel Focus | Determination                  |
| ---- | ------------ | ------------------------------ |
| A    | Matthew      | Gregorian year ÷ 3 remainder 1 |
| B    | Mark         | Gregorian year ÷ 3 remainder 2 |
| C    | Luke         | Gregorian year ÷ 3 remainder 0 |

Each cycle begins on the First Sunday of Advent of the previous civil year.

---

## T

### Temporal

See [Proper of Time](#proper-of-time).

---

## U

### Universal Norms on the Liturgical Year and the Calendar

The Universal Norms on the Liturgical Year and the Calendar (GNLY) is an official liturgical document that defines the structure of liturgical seasons, ranks of celebrations, and precedence rules.

The Latin title is _Normae universales de Anno liturgico et de Calendario_. The standard abbreviation **GNLY** (or GNLYC with "C" for Calendar) comes from the original 1969 English translation which used "**General** Norms" rather than "Universal Norms". The more recent translation (Third Typical Edition, 2002/2010) uses "**Universal** Norms on the Liturgical Year and the Calendar" which is more faithful to the Latin _universales_, but the abbreviation GNLY has remained in common use and is preferred for consistency with established liturgical practice and parallel with GIRM/GILH.

See: [GNLY](./reference/gnly.md)

---

## W

### Weekday Cycle

A two-year cycle for weekday Mass readings:

| Cycle       | Years               |
| ----------- | ------------------- |
| I (Year 1)  | Odd-numbered years  |
| II (Year 2) | Even-numbered years |
