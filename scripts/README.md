# Build Scripts

This directory contains build and automation scripts for the Romcal project.

## 📊 Script Responsibilities

| Script                  | Build | Lint | Format | Test | Generate Schema | Validate Data | Scope             |
| ----------------------- | ----- | ---- | ------ | ---- | --------------- | ------------- | ----------------- |
| `build-core.sh`         | ✅    | ❌   | ❌     | ❌   | ❌              | ❌            | Core only         |
| `build-adapters.sh`     | ✅    | ❌   | ❌     | ❌   | ❌              | ❌            | Adapters only     |
| `build-all.sh`          | ✅    | ❌   | ❌     | ❌   | ❌              | ❌            | Complete project  |
| `check-core.sh`         | ❌    | ✅   | ✅     | ✅   | ❌              | ❌            | Core only         |
| `check-adapters.sh`     | ❌    | ✅   | ✅     | ✅   | ❌              | ❌            | Adapters only     |
| `check-all.sh`          | ❌    | ✅   | ✅     | ✅   | ❌              | ❌            | Complete project  |
| `generate-schema.sh`    | ❌    | ❌   | ❌     | ❌   | ✅              | ❌            | Schema generation |
| `validate-calendars.sh` | ❌    | ❌   | ❌     | ❌   | ❌              | ✅            | Calendar data     |
| `validate-resources.sh` | ❌    | ❌   | ❌     | ❌   | ❌              | ✅            | Resource data     |
| `ci.sh`                 | ✅    | ✅   | ✅     | ✅   | ❌              | ❌            | Complete project  |

## 🚀 Quick Start

**Development:**

```bash
./scripts/build-core.sh      # Quick build
./scripts/build-adapters.sh wasm  # Build WASM bindings
./scripts/check-core.sh      # Quality checks
```

**Complete workflow:**

```bash
./scripts/ci.sh              # Full CI pipeline
./scripts/check-all.sh       # Global quality checks
```

## 📋 Scripts Overview

### Build Scripts

- **`build-all.sh`** - Builds entire project (core + adapters + tools)
- **`build-core.sh`** - Builds core module only
- **`build-adapters.sh`** - Builds all adapters or specific one (`wasm`, `python`)

### Quality Scripts

- **`check-core.sh`** - Runs clippy, rustfmt, tests on core
- **`check-adapters.sh`** - Runs quality checks on adapters
- **`check-all.sh`** - Global quality checks (core + adapters + tools + schemas)

### Utility Scripts

- **`ci.sh`** - Complete CI/CD pipeline
- **`generate-schema.sh`** - Generates JSON schemas from Rust definitions

## 📁 Structure

```
scripts/
├── README.md
├── build-all.sh          # Build entire project
├── build-core.sh         # Build core module only
├── build-adapters.sh     # Build all/specific adapters
├── check-core.sh         # Quality checks for core
├── check-adapters.sh     # Quality checks for adapters
├── ci.sh                 # Complete CI/CD pipeline
├── check-all.sh          # Global quality checks
├── generate-schema.sh    # Generate JSON schemas
├── validate-calendars.sh # Validate calendar data
├── transform-calendars.js # Transform calendar data
└── validate-resources.sh # Validate resource data
```

## 🔧 Adding New Scripts

1. Create the `.sh` file in this directory
2. Make it executable: `chmod +x script-name.sh`
3. Document its usage in this README
4. Test it from the project root

## 🎯 Conventions

- All scripts must be executable from the project root
- Use relative paths for portability
- Document usage in this README
