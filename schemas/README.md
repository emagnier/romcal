# JSON Schemas

This directory contains JSON schemas automatically generated from Rust type definitions in the Romcal project.

## Available Schemas

| Schema                     | Description                                     |
| -------------------------- | ----------------------------------------------- |
| `all_types.json`           | Complete schema with all type definitions       |
| `calendar_definition.json` | Schema for `CalendarDefinition` (main calendar) |
| `resources.json`           | Schema for `Resources` (localization data)      |

## Regeneration

To regenerate schemas after modifying Rust code:

```bash
# From the core/ directory
cargo run --bin generate-schema

# Or from the project root
./scripts/generate-schema.sh
```

## Usage

These schemas enable:

- **Data validation** - Validate calendar and resource files
- **Type generation** - Generate types for any language
- **API documentation** - Reference for data structure
- **Tool integration** - IDE support and development tools

## Community Bindings

These JSON Schemas are available for creating community bindings in other languages (Swift, Kotlin, Dart, Go, Java, C#, etc.) using standard JSON Schema tooling. Official bindings are available for TypeScript and Python.

## Schema Structure

The main `all_types.json` schema contains all type definitions with `$defs` for reusable types:

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "AllTypes",
  "$defs": {
    "Rank": { ... },
    "Color": { ... },
    "LiturgicalDay": { ... },
    ...
  }
}
```

## License

Apache License 2.0. See [LICENSE](../LICENSE) for details.
