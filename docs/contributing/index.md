---
sidebar_position: 1
---

# Contributing

Thank you for your interest in contributing to Romcal! This guide will help you understand how to contribute to the project.

## Ways to Contribute

### Add or Update Calendar Data

The most common contributions are additions or corrections to calendar definitions:

- **[Calendar Definitions](./definitions)** - Add celebrations for a region, country, or diocese
- **[Entity Resources](./resources)** - Add or translate entity names (saints, feasts, etc.)

### Code Contributions

If you want to contribute code:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request

## Project Structure

```
romcal/
├── core/              # Rust core library
├── cli/               # Command-line interface
├── bindings/
│   ├── typescript/    # TypeScript/JavaScript binding
│   ├── python/        # Python binding
│   └── wasm/          # WebAssembly build
├── data/
│   ├── definitions/   # Calendar definitions (JSON)
│   └── resources/     # Entity translations (JSON)
└── docs/              # This documentation
```

## Guidelines

- **[Naming Conventions](./naming-conventions)** - How to name calendars and entities
- **[Data Structure](./data-structure)** - Organization of data files

## Getting Help

- Open an [issue](https://github.com/romcal/romcal/issues) for bugs or feature requests
- Start a [discussion](https://github.com/romcal/romcal/discussions) for questions
