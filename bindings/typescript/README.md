# Romcal (TypeScript)

A TypeScript/JavaScript library for calculating Catholic liturgical dates and generating liturgical calendars. Works in Node.js and browsers via WebAssembly.

For the Rust library, see [romcal-core](../../core/). For command-line usage, see the [CLI documentation](../../cli/).

## Installation

```bash
npm install romcal
```

## Quick Start

```typescript
import { createRomcal } from 'romcal'

// Create a default instance
const romcal = await createRomcal()

// Get a specific liturgical date
const easter = await romcal.getDate('easter_sunday', 2026)
console.log(easter) // "2026-04-05"

// Generate the liturgical calendar for year 2026
const calendar = await romcal.generateLiturgicalCalendar(2026)

// Access a specific date
const christmas = calendar['2026-12-25']
if (christmas) {
  console.log(christmas[0].fullname) // "The Nativity of the Lord"
}
```

## Configuration

### Using Partial Configuration

```typescript
import { createRomcal } from 'romcal'

// With calendar and locale
const romcal1 = await createRomcal('france', 'fr')

// With full configuration
const romcal2 = await createRomcal({
  calendar: 'france',
  locale: 'fr',
  context: 'LITURGICAL',
  epiphanyOnSunday: true,
  ascensionOnSunday: true,
  corpusChristiOnSunday: true,
})
```

### Configuration Options

| Option                  | Type                    | Default           | Description                                               |
| ----------------------- | ----------------------- | ----------------- | --------------------------------------------------------- |
| `calendar`              | `string`                | `"general_roman"` | Calendar ID (e.g., `"france"`, `"united_states"`)         |
| `locale`                | `string`                | `"en"`            | Locale code (e.g., `"fr"`, `"es"`)                        |
| `context`               | `CalendarContext`       | `"GREGORIAN"`     | `"GREGORIAN"` (Jan-Dec) or `"LITURGICAL"` (Advent-Advent) |
| `epiphanyOnSunday`      | `boolean`               | `false`           | Celebrate Epiphany on Sunday (Jan 2-8) instead of Jan 6   |
| `ascensionOnSunday`     | `boolean`               | `false`           | Celebrate Ascension on Sunday instead of Thursday         |
| `corpusChristiOnSunday` | `boolean`               | `true`            | Celebrate Corpus Christi on Sunday instead of Thursday    |
| `easterCalculationType` | `EasterCalculationType` | `"GREGORIAN"`     | `"GREGORIAN"` or `"JULIAN"` Easter calculation            |
| `calendarDefinitions`   | `CalendarDefinition[]`  | `[]`              | Custom calendar definitions                               |
| `resources`             | `ResourcesDefinition[]` | `[]`              | Custom locale resources                                   |

### Loading Calendar Data

Without loading data, only the Proper of Time is available. To include the General Roman Calendar, particular calendars, and localized names, load calendar definitions and resources:

```typescript
import { createRomcal, CalendarDefinition, ResourcesDefinition } from 'romcal'
import { glob, readFile } from 'node:fs/promises'

// Load calendar definitions from JSON files
async function loadDefinitions(): Promise<CalendarDefinition[]> {
  const definitions: CalendarDefinition[] = []
  for await (const file of glob('data/definitions/**/*.json')) {
    definitions.push(JSON.parse(await readFile(file, 'utf-8')))
  }
  return definitions
}

// Load resources from JSON files
async function loadResources(): Promise<ResourcesDefinition[]> {
  // Your loading logic here
}

// Create instance with loaded data
const romcal = await createRomcal({
  calendar: 'france',
  locale: 'fr',
  calendarDefinitions: await loadDefinitions(),
  resources: await loadResources(),
})
```

## API

### createRomcal()

Creates a new Romcal instance.

```typescript
// Default configuration
const romcal1 = await createRomcal()

// With calendar and locale
const romcal2 = await createRomcal('france', 'fr')

// With partial configuration
const romcal3 = await createRomcal({
  calendar: 'france',
  locale: 'fr',
  epiphanyOnSunday: true,
})
```

### Romcal Instance

#### generateLiturgicalCalendar(year)

Generate the complete liturgical calendar for a given year.

```typescript
const calendar = await romcal.generateLiturgicalCalendar(2026)
// calendar is Record<string, LiturgicalDay[]>
// Keys are dates in "YYYY-MM-DD" format

for (const [date, days] of Object.entries(calendar)) {
  for (const day of days) {
    console.log(`${date}: ${day.fullname} (${day.rank})`)
  }
}
```

#### generateMassCalendar(year)

Generate a mass-centric view of the calendar organized by civil date and mass time.

```typescript
const massCalendar = await romcal.generateMassCalendar(2026)
// massCalendar is Record<string, MassContext[]>

// Evening masses appear on the previous civil day
const easterVigil = massCalendar['2026-04-04']?.find((m) => m.mass_time === 'EASTER_VIGIL')
console.log(easterVigil?.liturgical_date) // "2026-04-05"
```

#### getDate(id, year)

Get a liturgical date by its ID.

```typescript
const easter = await romcal.getDate('easter_sunday', 2026) // "2026-04-05"
const ashWed = await romcal.getDate('ash_wednesday', 2026) // "2026-02-18"
const pentecost = await romcal.getDate('pentecost_sunday', 2026) // "2026-05-24"
const christmas = await romcal.getDate('christmas', 2026) // "2026-12-25"
```

Any date ID from the liturgical calendar can be used (e.g., `easter_sunday`, `christmas`, `ordinary_time_5_monday`).

#### config

Access the resolved configuration.

```typescript
console.log(romcal.config.calendar) // "france"
console.log(romcal.config.locale) // "fr"
console.log(romcal.config.epiphanyOnSunday) // true
console.log(romcal.config.easterCalculationType) // "GREGORIAN"
console.log(romcal.config.context) // "GREGORIAN"
```

## Key Types

For detailed documentation on liturgical types (seasons, ranks, precedence, colors, cycles, mass times), see the [romcal-core documentation](../../core/README.md#key-types).

## Error Handling

All async operations may throw `RomcalError`:

```typescript
import { createRomcal, RomcalError } from 'romcal'

try {
  const romcal = await createRomcal()
  // Year must be >= 1583 (Gregorian calendar adoption)
  const calendar = await romcal.generateLiturgicalCalendar(1500)
} catch (error) {
  if (error instanceof RomcalError) {
    console.error('Romcal error:', error.message)
  }
}
```

## Development

### Requirements

- [Node.js](https://nodejs.org/) 24.0 or later
- [Rust](https://rustup.rs/) 1.85 or later (for WASM compilation)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/) (for WASM compilation)

### Building

```bash
# Install dependencies
npm install

# Full build (WASM + TypeScript)
npm run build

# Individual build steps
npm run build:wasm      # Compile Rust → WASM (via wasm-pack)
npm run build:vite      # Bundle TypeScript (via Vite)
npm run generate-types  # Generate types from JSON schema (via quicktype)
```

### Testing

```bash
npm test          # Run tests in watch mode
npm run test:run  # Run tests once
```

### Project Structure

```
bindings/typescript/
├── src/
│   ├── index.ts      # Main entry point, API wrapper
│   └── types.ts      # Generated types from JSON schema
├── test/             # Vitest tests
├── examples/
│   ├── node.ts       # Node.js usage example
│   └── browser.html  # Browser usage example
├── pkg/              # WASM output (generated by wasm-pack)
├── dist/             # Build output (generated by Vite)
├── vite.config.ts    # Vite configuration
├── vitest.config.ts  # Vitest configuration
└── package.json
```

### Running Examples

```bash
# Node.js example
npx tsx examples/node.ts

# Browser example (open in browser after build)
open examples/browser.html
```

## Related

- [romcal](https://github.com/romcal/romcal) - Main Romcal project
- [romcal-core](../../core/) - Core Rust library
- [romcal-cli](../../cli/) - Command-line interface
- [romcal (Python)](../python/) - Python binding

## License

Apache License 2.0. See [LICENSE](../../LICENSE) for details.
