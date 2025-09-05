# Romcal

A TypeScript library that generates liturgical calendars for the Roman rite of the Roman Catholic Church.

## Installation

```bash
npm install romcal
```

## Introduction

Romcal calculates for each day of the Gregorian calendar one or more corresponding liturgical day entities.

In addition, Romcal provides helpers to calculate precise liturgical dates such as Easter (according to the Gregorian or Julian calendar) and other major dates or liturgical seasons.

## Configuration

### Basic configuration

To generate a liturgical calendar, you must specify:

- **Calendar name** (default: `general_roman`)
- **Language** (default: `en`)
- **Easter calculation type** (default: `GREGORIAN`)
- **Specific options** (default: according to the chosen calendar)

### Configuration options

- `epiphanyOnSunday`: `true | false` - Epiphany on Sunday instead of January 6th
- `corpusChristiOnSunday`: `true | false` - Corpus Christi on Sunday
- `ascensionOnSunday`: `true | false` - Ascension on Sunday instead of the 6th Thursday of Easter
- `easterCalculationType`: `'GREGORIAN' | 'JULIAN'` - Type of Easter date calculation

### Usage

```ts
// Default configuration
romcal();

// Specific calendar
romcal('france');

// Calendar + language
romcal('france', 'fr');

// Complete configuration
romcal({
  calendar: 'france',
  locale: 'fr',
  epiphanyOnSunday: false,
  corpusChristiOnSunday: false,
  ascensionOnSunday: false,
  easterCalculationType: 'GREGORIAN',
});
```

### Custom sources

You can provide your own calendar and resource sources:

```ts
romcal({
  calendar: 'france',
  locale: 'fr',
  calendarSources: [], // Array of CalendarDefinition objects
  resources: [], // Array of Resource objects
});
```

## Calendar bundles

Romcal can create optimized bundles containing only the data needed for a specific calendar:

```ts
// Creating a bundle
const bundle = romcal('france', 'fr').createBundle();

// Reusing the bundle
const calendar = romcal(bundle);
```

### Bundle advantages

Bundles offer several significant advantages for application optimization:

#### Front-end optimization

- **Local cache**: Bundles can be stored in localStorage or other caching mechanisms to avoid frequently reloading content that changes little
- **Performance**: Instant loading of pre-calculated data, without needing to recalculate liturgical dates on each use
- **Bandwidth reduction**: Data is already present locally, eliminating network requests

#### Integration with build tools

- **Tree-shaking**: Web bundlers (Webpack, Vite, Rollup, etc.) can include only the necessary data in the final application bundle
- **Size reduction**: Only the data for the calendar and language used are included, avoiding unnecessarily bloating the application

> **Note**: The bundle tool is provided to facilitate this type of optimization, but these optimizations are not implemented directly in Romcal. It's up to the developer to use this functionality to implement optimizations according to their needs and concrete use cases.

## Main API

### Retrieving liturgical days

#### A specific day

```ts
const calendar = romcal('france', 'fr');

// By date
calendar.getDay('2025-02-03'); // LiturgicalDay[]
calendar.getDay(new Date()); // LiturgicalDay[]

// By liturgical identifier
calendar.getDay('easter', 2025); // LiturgicalDay[]
calendar.getDay('christmas', 2025); // LiturgicalDay[]
```

#### Year types

Romcal supports two types of years for calculating liturgical dates:

**Gregorian year** (`YearType.Gregorian`):

- Period: January 1st to December 31st
- Corresponds to the standard civil calendar
- Used by default

**Liturgical year** (`YearType.Liturgical`):

- Period: First Sunday of Advent to the last day of Ordinary Time
- The indicated year corresponds to the main part of the liturgical year
- For liturgical year 2025-2026, indicate only **2026**

**Concrete example**:

- Liturgical year 2026 begins on November 29, 2025 (First Sunday of Advent) and ends on November 27, 2026 (eve of the First Sunday of Advent of the following year)
- The Advent season of liturgical year 2026 (for 2025-2026) extends from November 29, 2025 to December 24, 2025
- The Easter season extends from March 28, 2026 (Easter) to May 15, 2026 (Pentecost)

```ts
// Gregorian year (default)
calendar.getDay('christmas', 2025); // LiturgicalDay[] for 2025-12-25

// Liturgical year
calendar.getDay('christmas', 2026, YearType.Liturgical); // LiturgicalDay[] for 2025-12-25 (Christmas of liturgical year 2025-2026)
```

#### Day collections

```ts
// Date range
calendar.getDayRange('2025-02-03', '2025-02-10'); // Record<DateString, LiturgicalDay[]>

// Complete year
calendar.getYear(2025, YearType.Gregorian); // Record<DateString, LiturgicalDay[]> from 2025-01-01 to 2025-12-31
calendar.getYear(2026, YearType.Liturgical); // Record<DateString, LiturgicalDay[]> from 2025-11-29 (First Sunday of Advent) to 2026-11-27 (end of liturgical year)

// Month
calendar.getMonth(2025, 2); // Record<DateString, LiturgicalDay[]> for February 2025
calendar.getMonth('2025-02'); // Record<DateString, LiturgicalDay[]> for February 2025

// Liturgical season
calendar.getSeason('ADVENT', 2026); // Record<DateString, LiturgicalDay[]> Advent of liturgical year 2025-2026 (Nov 29 - Dec 24, 2025)
calendar.getSeason('ORDINARY_TIME', 2026); // Record<DateString, LiturgicalDay[]> Ordinary Time of liturgical year 2025-2026
```

### Simple date calculation

To get only dates (without LiturgicalDay objects):

```ts
const calendar = romcal('france');

// Specific dates
calendar.dates.getEasterDate(2025); // Date
calendar.dates.getChristmasDate(2025); // Date
calendar.dates.getAshWednesdayDate(2025); // Date
calendar.dates.getPentecostDate(2025); // Date

// Date collections
calendar.dates.getSundaysOfAdventDate(2025); // Date[]
```

## Types

### LiturgicalDay

```ts
interface LiturgicalDay {
  date: Date;
  name: string;
  rank: string;
  season: string;
  color: string;
  // ... other properties
}
```

### YearType

```ts
enum YearType {
  Gregorian = 'gregorian',
  Liturgical = 'liturgical',
}
```

## Usage examples

### Basic example

```ts
import romcal from 'romcal';

const calendar = romcal('france', 'fr');

// Today
const today = calendar.getDay(new Date());
console.log(today[0].name); // Liturgical day name

// Easter 2025
const easter = calendar.getDay('easter', 2025);
console.log(easter[0].date); // Easter date

// February 2025
const february = calendar.getMonth(2025, 2);
Object.entries(february).forEach(([date, days]) => {
  console.log(`${date}: ${days.map((d) => d.name).join(', ')}`);
});
```

### Bundle example

```ts
// Creating and saving a bundle
const bundle = romcal('france', 'fr').createBundle();
localStorage.setItem('france-fr-bundle', JSON.stringify(bundle));

// Loading and using the bundle
const savedBundle = JSON.parse(localStorage.getItem('france-fr-bundle'));
const calendar = romcal(savedBundle);
const today = calendar.getDay(new Date());
```

### Date calculation example

```ts
const calendar = romcal('france');

// Calculate important dates for 2025
const easter = calendar.dates.getEasterDate(2025);
const adventSundays = calendar.dates.getSundaysOfAdventDate(2025);
const ashWednesday = calendar.dates.getAshWednesdayDate(2025);

console.log('Easter 2025:', easter);
console.log('Advent Sundays:', adventSundays);
console.log('Ash Wednesday:', ashWednesday);
```

## Available calendars

- `general_roman` - General Roman calendar (default)
- `france` - France calendar
- `france__paris` - Paris calendar
- `england` - England calendar
- `germany` - Germany calendar
- `spain` - Spain calendar
- ... and many more

## Supported languages

- `en` - English (default)
- `fr` - French
- `es` - Spanish
- `de` - German
- `it` - Italian
- `la` - Latin
- ... and many more

## License

MIT License - see the LICENSE file for more details.
