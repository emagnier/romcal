---
sidebar_position: 1
---

# Generating Calendars

Learn how to generate liturgical calendars with Romcal.

## Basic Usage

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

<Tabs groupId="language">
  <TabItem value="ts" label="TypeScript" default>

```typescript
import { Romcal } from 'romcal';

const romcal = new Romcal();
const calendar = romcal.generateCalendar(2025);
```

  </TabItem>
  <TabItem value="py" label="Python">

```python
from romcal import Romcal

romcal = Romcal()
calendar = romcal.generate_calendar(2025)
```

  </TabItem>
  <TabItem value="rs" label="Rust">

```rust
use romcal::Romcal;

let romcal = Romcal::new();
let calendar = romcal.generate_calendar(2025);
```

  </TabItem>
</Tabs>

## Using Regional Calendars

Romcal supports many regional and diocesan calendars that include local celebrations.

<Tabs groupId="language">
  <TabItem value="ts" label="TypeScript" default>

```typescript
import { Romcal } from 'romcal';

// French national calendar
const france = new Romcal({ calendar: 'france' });

// Diocese of Paris
const paris = new Romcal({ calendar: 'france__paris' });
```

  </TabItem>
  <TabItem value="py" label="Python">

```python
from romcal import Romcal

# French national calendar
france = Romcal(calendar="france")

# Diocese of Paris
paris = Romcal(calendar="france__paris")
```

  </TabItem>
  <TabItem value="rs" label="Rust">

```rust
use romcal::Romcal;

// French national calendar
let france = Romcal::new().calendar("france");

// Diocese of Paris
let paris = Romcal::new().calendar("france__paris");
```

  </TabItem>
</Tabs>

## Calendar Output

Each liturgical day in the calendar includes:

- **id**: Unique identifier for the celebration
- **date**: The date (in `YYYY-MM-DD` format)
- **name**: Full name of the celebration
- **rank**: Liturgical rank (Solemnity, Feast, Memorial, etc.)
- **season**: Liturgical season (Advent, Christmas, Lent, Easter, Ordinary Time)
- **colors**: Liturgical colors for vestments
- **readings**: References to Scripture readings (when available)

## Next Steps

- Learn about [mass-centric calendars](./masses)
- Explore [localization options](./locales)
