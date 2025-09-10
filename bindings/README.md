# Romcal Bindings

This directory contains platform-specific bindings for the romcal-core library.

## 📁 Structure

```
bindings/
├── README.md
├── wasm/          # WebAssembly bindings
│   ├── pkg/       # Generated WASM package
│   ├── src/       # TypeScript source files
│   └── ...
├── python/        # Python bindings (planned)
└── dart/          # Dart/Flutter bindings (planned)
```

## 🔧 Building Bindings

### WebAssembly (WASM)

The WASM bindings are generated from the `core/adapters/wasm` module.

**Build:**

```bash
# Build WASM bindings
./scripts/build-wasm.sh

# Or build all adapters
./scripts/build-adapters.sh wasm
```

**Output:**

- Generated files in `bindings/wasm/pkg/`
- TypeScript definitions included
- Ready for npm publishing

**Usage:**

```typescript
import { Romcal, LiturgicalConfig } from './pkg/romcal_core';

const romcal = new Romcal();
const config = new LiturgicalConfig();
```

### Python (Planned)

Python bindings will be generated using PyO3.

**Build:**

```bash
# Build Python bindings (when available)
./scripts/build-adapters.sh python
```

### Dart/Flutter (Planned)

Dart bindings will be generated using dart_bindgen.

**Build:**

```bash
# Build Dart bindings (when available)
./scripts/build-adapters.sh dart
```

## 🚀 Development

### Adding New Bindings

1. Create adapter in `core/adapters/[platform]/`
2. Add build logic to `scripts/build-adapters.sh`
3. Update this README
4. Test the build process

### Testing Bindings

```bash
# Test WASM bindings
cd bindings/wasm
npm test

# Test Python bindings (when available)
cd bindings/python
python -m pytest

# Test Dart bindings (when available)
cd bindings/dart
dart test
```

## 📦 Publishing

Each binding directory contains its own package configuration:

- **WASM**: `bindings/wasm/pkg/package.json`
- **Python**: `bindings/python/pyproject.toml` (planned)
- **Dart**: `bindings/dart/pubspec.yaml` (planned)

## 🔗 Architecture

```
romcal-core (Rust)
       ↓
core/adapters/[platform] (Rust)
       ↓
bindings/[platform] (Generated)
       ↓
Platform-specific package
```

The core library provides the business logic, adapters provide platform-specific bindings, and the bindings directory contains the final packages ready for distribution.
