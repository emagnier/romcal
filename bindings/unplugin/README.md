# @romcal/unplugin

Bundler plugin for romcal data optimization. Generates minimal bundles containing only the calendar definitions and resources needed for your configuration. The bundling logic runs in Rust (via WebAssembly) for optimal performance.

## Installation

```bash
npm install @romcal/unplugin
# or
pnpm add @romcal/unplugin
```

## Usage

### Vite

```typescript
// vite.config.ts
import { defineConfig } from 'vite';
import romcal from '@romcal/unplugin/vite';

export default defineConfig({
  plugins: [
    romcal({
      calendar: 'france',
      locale: 'fr',
    }),
  ],
});
```

### Rollup

```typescript
// rollup.config.js
import romcal from '@romcal/unplugin/rollup';

export default {
  plugins: [
    romcal({
      calendar: 'france',
      locale: 'fr',
    }),
  ],
};
```

### Webpack

```typescript
// webpack.config.js
const romcal = require('@romcal/unplugin/webpack').default;

module.exports = {
  plugins: [
    romcal({
      calendar: 'france',
      locale: 'fr',
    }),
  ],
};
```

### esbuild

```typescript
import { build } from 'esbuild';
import romcal from '@romcal/unplugin/esbuild';

build({
  plugins: [
    romcal({
      calendar: 'france',
      locale: 'fr',
    }),
  ],
});
```

### Rspack

```typescript
// rspack.config.js
import romcal from '@romcal/unplugin/rspack';

export default {
  plugins: [
    romcal({
      calendar: 'france',
      locale: 'fr',
    }),
  ],
};
```

### Rolldown

```typescript
// rolldown.config.js
import romcal from '@romcal/unplugin/rolldown';

export default {
  plugins: [
    romcal({
      calendar: 'france',
      locale: 'fr',
    }),
  ],
};
```

### Farm

```typescript
// farm.config.ts
import romcal from '@romcal/unplugin/farm';

export default {
  plugins: [
    romcal({
      calendar: 'france',
      locale: 'fr',
    }),
  ],
};
```

### Bun

```typescript
import romcal from '@romcal/unplugin/bun';

Bun.build({
  plugins: [
    romcal({
      calendar: 'france',
      locale: 'fr',
    }),
  ],
});
```

## In your code

Import the virtual module and use it to create a romcal instance:

```typescript
import bundle from 'virtual:romcal';
import { createRomcal } from 'romcal';

const romcal = await createRomcal(bundle);
const calendar = romcal.generateLiturgicalCalendar(2026);
```

## Options

| Option                  | Type                        | Default            | Description                             |
| ----------------------- | --------------------------- | ------------------ | --------------------------------------- |
| `calendar`              | `string`                    | `'general_roman'`  | Calendar ID (e.g., `'france'`, `'usa'`) |
| `locale`                | `string`                    | `'en'`             | Locale code (e.g., `'fr'`, `'es'`)      |
| `epiphanyOnSunday`      | `boolean`                   | `false`            | Epiphany celebrated on Sunday           |
| `ascensionOnSunday`     | `boolean`                   | `false`            | Ascension celebrated on Sunday          |
| `corpusChristiOnSunday` | `boolean`                   | `true`             | Corpus Christi celebrated on Sunday     |
| `context`               | `'GREGORIAN'\|'LITURGICAL'` | `'GREGORIAN'`      | Year boundary context                   |
| `easterCalculationType` | `'GREGORIAN'\|'JULIAN'`     | `'GREGORIAN'`      | Easter calculation method               |
| `calendarDefinitions`   | `CalendarDefinition[]`      | `[]`               | Additional custom calendar definitions  |
| `resources`             | `Resources[]`               | `[]`               | Additional custom resources (locales)   |
| `moduleId`              | `string`                    | `'virtual:romcal'` | Custom virtual module ID                |

## Custom data

You can add custom calendar definitions and resources that will be merged with the embedded data:

```typescript
// vite.config.ts
import { defineConfig } from 'vite';
import romcal from '@romcal/unplugin/vite';
import myParish from './data/my-parish.json';
import myLocale from './data/my-locale.json';

export default defineConfig({
  plugins: [
    romcal({
      calendar: 'my_parish',
      locale: 'fr',
      calendarDefinitions: [myParish],
      resources: [myLocale],
    }),
  ],
});
```

Custom data is merged with the embedded romcal data, allowing you to:

- Add parish, diocesan or regional calendars that extend existing ones
- Add custom translations or locales
- Override specific celebrations

## How it works

The plugin generates an optimized bundle at build time that includes:

1. **Calendar definitions**: Only the calendars in the hierarchy (e.g., `france` → `europe` → `general_roman`)
2. **Resources**: Only the locales needed (e.g., `fr` → `en`)
3. **Martyrology**: Only the martyrology entries referenced by the selected calendars

This results in significantly smaller bundles compared to importing all data.

## License

Apache-2.0
