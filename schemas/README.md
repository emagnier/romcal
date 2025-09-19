# JSON Schemas

This directory contains JSON schemas automatically generated from Rust type definitions in the Romcal project.

## 📁 Available Schemas

- **`calendar_definition.json`** - Schema for `CalendarDefinition` (main calendar structure)
- **`resources.json`** - Schema for `Resources` (localization data)
- **`all_types.json`** - Complete schema with all type definitions

## 🔄 Regeneration

To regenerate schemas after modifying Rust code:

```bash
# From the core/ directory
cargo run --bin generate-schema
```

Or use the automated script:

```bash
# From the project root
./scripts/generate-schema.sh
```

## 🎯 Usage

These schemas enable:

- **Data validation** - Validate calendar and resource files
- **Type generation** - Generate TypeScript/Python types
- **API documentation** - Reference for data structure
- **Tool integration** - IDE support and development tools
