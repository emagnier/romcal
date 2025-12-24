# Romcal Python Examples

## Prerequisites

- Rust toolchain
- Python 3.10+
- [uv](https://docs.astral.sh/uv/) package manager

## Setup

From the `bindings/python/` directory:

```bash
uv run maturin develop --release
```

This command:

1. Compiles the Rust library (`romcal-uniffi`) in release mode
2. Generates Python bindings from the Rust code using UniFFI
3. Installs the `romcal` package in development mode in the virtual environment

## Running Examples

```bash
uv run python examples/basic_usage.py
```

## Examples

### basic_usage.py

Demonstrates:

- Loading calendar definitions from the `data/` folder
- Loading and merging resources (translations) from the `data/` folder
- Creating a Romcal instance with French calendar and locale
- Generating liturgical and mass calendars
- Displaying celebrations with translated names
