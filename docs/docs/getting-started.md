---
sidebar_position: 2
---

# Getting Started

This guide will help you install `romcal` and generate your first liturgical calendar.

## Installation

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

<Tabs groupId="language">
  <TabItem value="ts" label="TypeScript/JavaScript" default>

```bash
npm install romcal
```

Or with yarn:

```bash
yarn add romcal
```

  </TabItem>
  <TabItem value="py" label="Python">

```bash
pip install romcal
```

  </TabItem>
  <TabItem value="rs" label="Rust">

Add to your `Cargo.toml`:

```toml
[dependencies]
romcal = "0.1"
```

Or use cargo:

```bash
cargo add romcal
```

  </TabItem>
</Tabs>

## Generate a Calendar

<Tabs groupId="language">
  <TabItem value="ts" label="TypeScript/JavaScript" default>

```typescript
import { Romcal } from 'romcal';

// Create a Romcal instance with default settings
const romcal = new Romcal();

// Generate the liturgical calendar for 2025
const calendar = romcal.generateCalendar(2025);

// Access individual days
for (const day of calendar) {
  console.log(`${day.date}: ${day.name}`);
}
```

  </TabItem>
  <TabItem value="py" label="Python">

```python
from romcal import Romcal

# Create a Romcal instance with default settings
romcal = Romcal()

# Generate the liturgical calendar for 2025
calendar = romcal.generate_calendar(2025)

# Access individual days
for day in calendar:
    print(f"{day.date}: {day.name}")
```

  </TabItem>
  <TabItem value="rs" label="Rust">

```rust
use romcal::Romcal;

fn main() {
    // Create a Romcal instance with default settings
    let romcal = Romcal::new();

    // Generate the liturgical calendar for 2025
    let calendar = romcal.generate_calendar(2025);

    // Access individual days
    for day in &calendar {
        println!("{}: {}", day.date, day.name);
    }
}
```

  </TabItem>
</Tabs>

## Configuration Options

You can customize Romcal with various options:

<Tabs groupId="language">
  <TabItem value="ts" label="TypeScript/JavaScript" default>

```typescript
import { Romcal } from 'romcal';

const romcal = new Romcal({
  // Use the French calendar
  calendar: 'france',

  // Set the locale for translations
  locale: 'fr',

  // Celebrate Epiphany on Sunday (common in many countries)
  epiphanyOnSunday: true,

  // Celebrate Ascension on Sunday (instead of Thursday)
  ascensionOnSunday: true,

  // Celebrate Corpus Christi on Sunday (instead of Thursday)
  corpusChristiOnSunday: true,
});
```

  </TabItem>
  <TabItem value="py" label="Python">

```python
from romcal import Romcal

romcal = Romcal(
    # Use the French calendar
    calendar="france",

    # Set the locale for translations
    locale="fr",

    # Celebrate Epiphany on Sunday (common in many countries)
    epiphany_on_sunday=True,

    # Celebrate Ascension on Sunday (instead of Thursday)
    ascension_on_sunday=True,

    # Celebrate Corpus Christi on Sunday (instead of Thursday)
    corpus_christi_on_sunday=True,
)
```

  </TabItem>
  <TabItem value="rs" label="Rust">

```rust
use romcal::Romcal;

let romcal = Romcal::new()
    // Use the French calendar
    .calendar("france")

    // Set the locale for translations
    .locale("fr")

    // Celebrate Epiphany on Sunday (common in many countries)
    .epiphany_on_sunday(true)

    // Celebrate Ascension on Sunday (instead of Thursday)
    .ascension_on_sunday(true)

    // Celebrate Corpus Christi on Sunday (instead of Thursday)
    .corpus_christi_on_sunday(true);
```

  </TabItem>
</Tabs>

## Next Steps

- Learn how to [generate calendars](./guides/calendar) with different options
- Explore [available calendars](./guides/locales) for your region
- Use the [CLI](./cli/) for quick lookups and scripting
