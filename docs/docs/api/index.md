---
sidebar_position: 1
---

# API Overview

Reference documentation for the Romcal API across TypeScript, Python, and Rust.

## Installation

import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

<Tabs groupId="language">
  <TabItem value="ts" label="TypeScript" default>

```bash
npm install romcal
```

  </TabItem>
  <TabItem value="py" label="Python">

```bash
pip install romcal
```

  </TabItem>
  <TabItem value="rs" label="Rust">

```bash
cargo add romcal
```

  </TabItem>
</Tabs>

## Main Entry Point

<Tabs groupId="language">
  <TabItem value="ts" label="TypeScript" default>

```typescript
import { Romcal } from 'romcal';

const romcal = new Romcal(options);
```

  </TabItem>
  <TabItem value="py" label="Python">

```python
from romcal import Romcal

romcal = Romcal(**options)
```

  </TabItem>
  <TabItem value="rs" label="Rust">

```rust
use romcal::Romcal;

let romcal = Romcal::new().with_options(options);
```

  </TabItem>
</Tabs>

## API Sections

- [Romcal Class](./romcal) - Main class and methods
- [Types](./types) - Type definitions and enums
- [Options](./options) - Configuration options
