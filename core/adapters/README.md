# Romcal Core Adapters

This directory contains platform-specific adapters for the romcal library.

## Structure

- `wasm/` - WebAssembly adapter for browser and Node.js environments
- `python/` - Python adapter (planned)

## WASM Adapter

The WASM adapter provides JavaScript-compatible bindings for the core Rust library.

### Usage

```bash
# Build the WASM module
cd adapters/wasm
wasm-pack build --target web --out-dir pkg
```

### Features

- Full TypeScript support
- Zero-copy data structures
- Tree-shaking friendly
- Works in both browser and Node.js

## Architecture

Each adapter is a separate Rust crate that:

- Depends on `romcal` as a library
- Provides platform-specific bindings
- Maintains the same API surface as the core
- Can be built and distributed independently
