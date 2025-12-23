# Romcal WASM Bindings

This package provides WebAssembly bindings for the Romcal liturgical calendar library.

## Installation

```bash
npm install
```

## Building

```bash
# Build for Node.js
npm run build

# Build for web browsers
npm run build-web

# Build for bundlers (webpack, rollup, etc.)
npm run build-bundler
```

## Usage

```typescript
import { romcal } from 'romcal'

// Default configuration
const calendar1 = await romcal()

console.log(calendar1.config.calendar) // 'general_roman'
console.log(calendar1.config.locale) // 'en'
console.log(calendar1.config.epiphanyOnSunday) // false
console.log(calendar1.config.corpusChristiOnSunday) // false
console.log(calendar1.config.ascensionOnSunday) // false
console.log(calendar1.config.easterCalculationType) // 'GREGORIAN'

// Custom configuration
const calendar2 = await romcal('france', 'fr')
console.log(calendar2.config.calendar) // 'france'
console.log(calendar2.config.locale) // 'fr'
```

## Testing

```bash
npm test
```

This will run the test file using tsx.
