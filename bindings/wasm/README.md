# Romcal WASM Adapter

WebAssembly adapter for the romcal library. This module compiles the Rust core to WASM and provides JavaScript bindings via `wasm-bindgen`.

> **Note**: This is an internal adapter used by the [TypeScript package](../typescript/). For end-user documentation, see the [TypeScript README](../typescript/README.md).

## Building

```bash
# From bindings/wasm directory
wasm-pack build --target web --out-dir ../typescript/pkg
```

The compiled WASM module is output to `bindings/typescript/pkg/`.

## Exports

The adapter exposes the following to JavaScript:

- `Romcal` - Main class for calendar generation
- `RomcalConfig` - Configuration with getters
- `PartialRomcalConfig` - Builder for configuration
- `romcal()` - Factory function (default config)
- `romcal_with_partial_config()` - Factory with options
- `romcal_with_config_object()` - Factory with config object

## Related

- [romcal](../../core/) - Rust core library
- [romcal (TypeScript)](../typescript/) - TypeScript package (uses this adapter)

## License

Apache License 2.0. See [LICENSE](../../LICENSE) for details.
