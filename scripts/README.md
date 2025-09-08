# Build Scripts

This directory contains build and automation scripts for the Romcal project.

## 📋 Available Scripts

### `generate-schema.sh`

Generates JSON schemas from Rust definitions.

**Usage:**

```bash
# From the project root
./scripts/generate-schema.sh
```

**Features:**

- Compiles the schema generator
- Generates JSON schemas in `schemas/` (root)
- Displays detailed progress messages

### `build-wasm.sh`

Compiles WASM bindings for JavaScript/TypeScript bindings.

**Usage:**

```bash
# From the project root
./scripts/build-wasm.sh
```

**Features:**

- Compiles Rust code to WASM
- Generates JavaScript/TypeScript bindings
- Optimizes WASM binaries
- Installs necessary dependencies

## 🔧 Adding New Scripts

To add a new script:

1. Create the `.sh` file in this directory
2. Make it executable: `chmod +x script-name.sh`
3. Document its usage in this README
4. Test it from the project root

## 📁 Structure

```
scripts/
├── README.md
├── generate-schema.sh
├── validate-calendars.sh
├── transform-calendars.js
└── build-wasm.sh
```

## 🎯 Conventions

- All scripts must be executable from the project root
- Use relative paths for portability
- Include informative messages with emojis
- Document usage in this README
