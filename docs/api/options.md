---
sidebar_position: 4
---

# Options

Configuration options for Romcal.

## RomcalOptions

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

<Tabs groupId="language">
  <TabItem value="ts" label="TypeScript" default>

```typescript
interface RomcalOptions {
  calendar?: string;
  locale?: string;
  easterCalculation?: 'gregorian' | 'julian';
  epiphanyOnSunday?: boolean;
  ascensionOnSunday?: boolean;
  corpusChristiOnSunday?: boolean;
}
```

  </TabItem>
  <TabItem value="py" label="Python">

```python
class RomcalOptions:
    calendar: str = "general_roman"
    locale: str = "en"
    easter_calculation: str = "gregorian"
    epiphany_on_sunday: bool = False
    ascension_on_sunday: bool = False
    corpus_christi_on_sunday: bool = False
```

  </TabItem>
  <TabItem value="rs" label="Rust">

```rust
pub struct RomcalOptions {
    pub calendar: String,
    pub locale: String,
    pub easter_calculation: EasterCalculation,
    pub epiphany_on_sunday: bool,
    pub ascension_on_sunday: bool,
    pub corpus_christi_on_sunday: bool,
}
```

  </TabItem>
</Tabs>

## Option Details

### calendar

The liturgical calendar to use.

| Type     | Default           |
| -------- | ----------------- |
| `string` | `"general_roman"` |

**Examples:**

- `"general_roman"` - General Roman Calendar
- `"france"` - French national calendar
- `"france__paris"` - Diocese of Paris
- `"united_states"` - United States calendar

Use `romcal list calendars` to see all available calendars.

### locale

The locale for translations.

| Type     | Default |
| -------- | ------- |
| `string` | `"en"`  |

**Examples:**

- `"en"` - English
- `"fr"` - French
- `"es"` - Spanish
- `"it"` - Italian
- `"la"` - Latin

Use `romcal list locales` to see all available locales.

### easterCalculation

The method used to calculate Easter.

| Type                        | Default       |
| --------------------------- | ------------- |
| `"gregorian"` \| `"julian"` | `"gregorian"` |

- **gregorian**: Western (Catholic, Protestant) Easter calculation
- **julian**: Eastern (Orthodox) Easter calculation

### epiphanyOnSunday

Whether to celebrate Epiphany on Sunday (between January 2-8) instead of January 6.

| Type      | Default |
| --------- | ------- |
| `boolean` | `false` |

Common in countries like the United States, where Epiphany is transferred to Sunday.

### ascensionOnSunday

Whether to celebrate the Ascension of the Lord on Sunday instead of Thursday (40 days after Easter).

| Type      | Default |
| --------- | ------- |
| `boolean` | `false` |

When `true`, the Ascension is celebrated on the 7th Sunday of Easter instead of the Thursday before.

### corpusChristiOnSunday

Whether to celebrate Corpus Christi (The Body and Blood of Christ) on Sunday instead of Thursday.

| Type      | Default |
| --------- | ------- |
| `boolean` | `false` |

When `true`, Corpus Christi is celebrated on the Sunday after Trinity Sunday instead of the Thursday after.

## Search Options

Options for the `searchEntities` method.

<Tabs groupId="language">
  <TabItem value="ts" label="TypeScript" default>

```typescript
interface SearchOptions {
  type?: EntityType;
  sex?: Sex;
  canonizationLevel?: CanonizationLevel;
  titles?: Title[];
  limit?: number;
  minScore?: number;
}
```

  </TabItem>
  <TabItem value="py" label="Python">

```python
class SearchOptions:
    type: EntityType | None = None
    sex: Sex | None = None
    canonization_level: CanonizationLevel | None = None
    titles: list[Title] | None = None
    limit: int = 20
    min_score: float = 0.3
```

  </TabItem>
  <TabItem value="rs" label="Rust">

```rust
pub struct SearchOptions {
    pub entity_type: Option<EntityType>,
    pub sex: Option<Sex>,
    pub canonization_level: Option<CanonizationLevel>,
    pub titles: Option<Vec<Title>>,
    pub limit: usize,
    pub min_score: f64,
}
```

  </TabItem>
</Tabs>

### Search Option Details

| Option              | Type                | Default | Description                                  |
| ------------------- | ------------------- | ------- | -------------------------------------------- |
| `type`              | `EntityType`        | -       | Filter by entity type (SAINT, BLESSED, etc.) |
| `sex`               | `Sex`               | -       | Filter by sex (MALE, FEMALE)                 |
| `canonizationLevel` | `CanonizationLevel` | -       | Filter by canonization level                 |
| `titles`            | `Title[]`           | -       | Filter by titles (MARTYR, VIRGIN, etc.)      |
| `limit`             | `number`            | `20`    | Maximum number of results                    |
| `minScore`          | `number`            | `0.3`   | Minimum relevance score (0.0-1.0)            |
