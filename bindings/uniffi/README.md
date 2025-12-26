# Romcal UniFFI Adapter

UniFFI adapter for the romcal library. This module provides foreign function interface bindings using Mozilla's [UniFFI](https://mozilla.github.io/uniffi-rs/) framework, enabling the Rust core to be used from Python (and potentially Swift, Kotlin).

> **Note**: This is an internal adapter used by the [Python package](../python/). For end-user documentation, see the [Python README](../python/README.md).

## Building

The Python package uses [maturin](https://www.maturin.rs/) to build and package the UniFFI bindings:

```bash
# From bindings/python directory
cd ../python
maturin develop  # Development build
maturin build    # Release build
```

## Exports

The adapter exposes the following to Python:

- `Romcal` - Main class for calendar generation
- `RomcalConfig` - Configuration record with optional fields
- `RomcalError` - Error type with variants (InvalidYear, InvalidConfig, NotFound, ParseError, CalculationError)

## Related

- [romcal](../../core/) - Rust core library
- [romcal (Python)](../python/) - Python package (uses this adapter)
- [UniFFI documentation](https://mozilla.github.io/uniffi-rs/)

## License

Apache License 2.0. See [LICENSE](../../LICENSE) for details.
