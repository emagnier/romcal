---
title: Contributing
---

Thank you for your interest in contributing to Romcal! This guide will help you understand how to contribute to the project.

## Ways to Contribute

### Add or Update Calendar Data

The most common contributions are additions or corrections to calendar definitions:

- **[Calendar Definitions](./definitions)** - Add celebrations for a region, country, or diocese
- **[Martyrology Resources](./resources)** - Add or translate martyrology entries (saints, blessed, etc.)

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
│   ├── typescript/    # TypeScript binding
│   ├── python/        # Python binding
│   └── wasm/          # WebAssembly build
├── data/
│   ├── definitions/   # Calendar definitions (JSON)
│   └── resources/     # Martyrology translations (JSON)
└── docs/              # This documentation
```

## Guidelines

- **[Naming Conventions](./naming-conventions)** - How to name calendars and martyrology entries
- **[Data Structure](./data-structure)** - Organization of data files

## Getting Help

- Open an [issue](https://github.com/romcal/romcal/issues) for bugs or feature requests
- Start a [discussion](https://github.com/romcal/romcal/discussions) for questions
