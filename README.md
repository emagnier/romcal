<h1 align="center">
  <a href="https://github.com/romcal/romcal">
    <img alt="romcal" src="https://user-images.githubusercontent.com/1045997/89793747-854ede00-db26-11ea-8e46-837ab4ca0a96.png">
  </a>
</h1>

<p align="center">
  <strong>Catholic liturgical calendar calculator</strong>
</p>

<p align="center">
  Romcal is a high-performance Rust library that generates liturgical calendars for the Roman Rite of the Catholic Church. It calculates liturgical dates for any year in the standard calendar and provides comprehensive metadata for each liturgical day.
</p>

<p align="center">
  <a href="LICENSE"><img alt="License" src="https://img.shields.io/github/license/romcal/romcal?color=blue&style=flat-square"></a>
  <a href="https://discord.gg/MgWcwE4HZD" target="_blank" rel="noopener noreferrer"><img alt="Discord" src="https://img.shields.io/discord/1353897152119570655?color=blue&label=Discord&logo=discord&style=flat-square"></a>
  <a href="https://www.codetriage.com/romcal/romcal" target="_blank" rel="noopener noreferrer"><img alt="Code Triage Helpers" src="https://www.codetriage.com/romcal/romcal/badges/users.svg" /></a>
</p>

## ✨ Features

- **Perpetual calendar**: Calculate liturgical dates for any year
- **Multiple calendars**: Support for numerous liturgical calendars by country, diocese, city, region...
- **Localization**: Available in multiple languages with easy support for adding new languages
- **Entity catalog**: Enriched metadata for each liturgical day (saints, blessed, places, events)
- **Extensible**: Easy to add new calendars and languages

## 📜 Liturgical accuracy

Romcal generates liturgical calendars of the Roman Rite of the Roman Catholic Church.

Output conforms to the revised liturgical calendar as approved by Paul VI in [Mysterii Paschalis](http://w2.vatican.va/content/paul-vi/en/motu_proprio/documents/hf_p-vi_motu-proprio_19690214_mysterii-paschalis.html) dated 14 February 1969.

The rules are defined in:

- [_General Instruction on the Roman Missal_](https://www.catholicculture.org/culture/library/view.cfm?recnum=337) (GIRM)
- [_General Norms for the Liturgical Year and the Calendar_](https://www.catholicculture.org/culture/library/view.cfm?id=10842) (GNLY)
- [General Instructions of the Liturgy of the Hours](https://divineoffice.org/general-instructions/) (GILH)

This ensures that Romcal provides accurate, canonical liturgical data for any application.

## ⚡ Performance & Architecture

Written in Rust, Romcal delivers exceptional performance with strong typing, JSON schemas, and clean APIs that work with any language or framework. Perfect for building liturgical applications across all platforms.

## 🛠️ Available tools

- **[Complete CLI]()**: Powerful command-line interface for terminal usage, automation, and integration
- **Native bindings**:
  - **[JavaScript/TypeScript]()** (WASM, tree-shaking, modern build tools)
  - **[Python]()** (high-performance bindings, Pydantic models)
  - **[Dart/Flutter]()** (null safety, cross-platform, Flutter integration)
  - Additional language bindings welcome

## 🚀 Quick start

To use Romcal in your project, see our [complete documentation](docs/) for installation and usage instructions.

## 🤝 Contributing

### Getting started

1. Clone this repository
2. Install Rust via [rustup.rs](https://rustup.rs/)
3. To install and use the CLI:
   ```bash
   cd cli
   cargo install --path .
   romcal --help
   ```

### Data structure

- `/data/calendars` - Calendar definitions by country, diocese, region and communities
- `/data/resources` - Localization files and entity catalogs

Contributions are welcome! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md) for community standards.

## 📄 License and Authors

- Code (`/`): Apache License 2.0, authored by Étienne Magnier
- Data (`/data`): MIT License
  - Inherited from Romcal v1–v3 by Julian Pereira
  - Full contributor list: https://github.com/romcal/romcal/graphs/contributors

See [LICENSE](LICENSE), [data/LICENSE-MIT](data/LICENSE-MIT), and [AUTHORS.md](AUTHORS.md) for details.

## 📝 Naming convention

- **Romcal** (capitalized) refers to the project itself, as a proper name
- **`romcal`** (lowercase, monospaced) refers to the package name or when used in code examples: `npm install romcal`

## 📚 Additional resources

- [Changelog](CHANGELOG.md) - Release history and updates
- [Glossary](.archive/docs/glossary.md) - Terms and definitions

<p align="center">
  <a href="https://github.com/romcal/romcal">
    <img alt="romcal-icon" src="https://user-images.githubusercontent.com/1045997/89793396-1c676600-db26-11ea-9426-991ac1e32b82.png">
  </a>
</p>
