# Romcal Bindings

This directory contains platform-specific bindings for the romcal library.

## 📁 Structure

```
bindings/
├── README.md
├── typescript/          # Typescript bindings
│   ├── pkg/       # Generated WASM package
│   ├── src/       # TypeScript source files
│   └── ...
├── python/        # Python bindings (planned)
```

## 🔧 Building Bindings

### WebAssembly (WASM)

The WASM bindings are generated from the `bindings/wasm` module.

**Build:**

```bash
# Build WASM bindings
./scripts/build-wasm.sh

# Or build all adapters
./scripts/build-adapters.sh wasm
```

**Output:**

- Generated files in `bindings/typescript/pkg/`
- TypeScript definitions included
- Ready for npm publishing

**Usage:**

```typescript
import { Romcal, Preset } from './pkg/romcal_core';

const romcal = new Romcal();
const preset = new Preset();
```

### Python (Planned)

Python bindings will be generated using PyO3.

**Build:**

```bash
# Build Python bindings (when available)
./scripts/build-adapters.sh python
```

## 🚀 Development

### Adding New Bindings

1. Create adapter in `bindings/[platform]/`
2. Add build logic to `scripts/build-adapters.sh`
3. Update this README
4. Test the build process

### Testing Bindings

```bash
# Test Typescript bindings
cd bindings/typescript
npm test

# Test Python bindings (when available)
cd bindings/python
python -m pytest


## 📦 Publishing

Each binding directory contains its own package configuration:

- **Typescript**: `bindings/typescript/pkg/package.json`
- **Python**: `bindings/python/pyproject.toml` (planned)

## 🔗 Architecture

```

romcal (Rust core library)
↓
bindings/[platform] (Platform adapters + generated code)
↓
Platform-specific package

```

The core library provides the business logic, and the bindings directory contains platform-specific adapters and the final packages ready for distribution.
```
