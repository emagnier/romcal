# JSON Schemas for Romcal

This directory contains JSON schemas automatically generated from Rust definitions in the Romcal project.

## 📁 Available Files

- **`calendar_definition.json`** : Main schema for `CalendarDefinition`
- **`day_definition.json`** : Schema for `DayDefinition`
- **`precedence.json`** : Schema for the `Precedence` enumeration
- **`all_types.json`** : Complete schema with all definitions

## 🔄 Schema Regeneration

To regenerate schemas after modifying Rust code:

```bash
# From the core/ directory
cargo run --bin generate_schema
```

Or use the automated script:

```bash
# From the project root
./scripts/generate-schema.sh
```

## 🎯 Usage

These JSON schemas can be used for:

1. **Validation** : Validate calendar data structure
2. **Type Generation** : Generate TypeScript types from schemas
3. **Documentation** : Serve as API documentation
4. **Integration** : Integrate with development tools

## 🔧 Next Steps

- [ ] Automatic TypeScript type generation from schemas
- [ ] Integration with WASM binding
- [ ] Automatic calendar data validation
