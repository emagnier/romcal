# Generated Constants

This document describes the auto-generated constants available in romcal-core.

## Overview

The romcal-core library automatically generates constants containing all available calendars and locales at build time. This provides a fast and efficient way to access this information without parsing JSON files at runtime.

## Available Constants

### `CALENDAR_IDS`

- **Type**: `&[&str]`
- **Description**: Array of all calendar IDs, sorted alphabetically

### `LOCALE_CODES`

- **Type**: `&[&str]`
- **Description**: Array of all locale codes, sorted alphabetically

## Usage Example

```rust
use romcal_core::{CALENDAR_IDS, LOCALE_CODES};

fn main() {
    println!("Calendar IDs: {}", CALENDAR_IDS.len());
    for calendar in CALENDAR_IDS {
        println!("  - {}", calendar);
    }

    println!("Locale codes: {}", LOCALE_CODES.len());
    for locale in LOCALE_CODES {
        println!("  - {}", locale);
    }
}
```

## How It Works

1. **Build-time Generation**: Constants are generated during the build process by the `build.rs` script
2. **Source**: Data is extracted from JSON files in the `data/` directory
3. **Automatic Updates**: Constants are regenerated whenever calendar or locale data changes
4. **Performance**: No runtime JSON parsing required - constants are compiled into the binary

## File Structure

- `build.rs` - Build script that runs the generation
- `scripts/generate_constants.sh` - Shell script that extracts data from JSON files
- `src/generated/` - Auto-generated constants (do not edit)
  - `calendar_ids.rs` - Calendar IDs and tree structure
  - `locale_ids.rs` - Locale codes and tree structure
  - `mod.rs` - Module exports
- `src/lib.rs` - Exports the constants for public use

## Regenerating Constants

Constants are automatically regenerated during build, but you can manually regenerate them:

```bash
# From the core directory
./scripts/generate_constants.sh

# From the project root
./scripts/generate-constants.sh
```
