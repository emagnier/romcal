---
title: CLI Overview
---

The Romcal CLI is a command-line tool for calculating Catholic liturgical dates and generating calendars.

## Quick Start

```bash
# Get Easter date for 2025
romcal date easter_sunday 2025

# Generate liturgical calendar for current year
romcal calendar

# Generate calendar with specific locale and calendar
romcal calendar 2025 --calendar france --locale fr

# List available calendars
romcal list calendars
```

## Features

- **Calculate dates**: Get specific liturgical dates (Easter, Pentecost, etc.)
- **Generate calendars**: Output full liturgical calendars in various formats
- **Search martyrology**: Look up saints, blessed, and feasts
- **Multiple formats**: YAML, JSON, CSV, or line-by-line output
- **Shell completion**: Tab completion for bash, zsh, fish, and PowerShell

## Commands

| Command                   | Description                          |
| ------------------------- | ------------------------------------ |
| `date <DATE_NAME> [YEAR]` | Calculate a specific liturgical date |
| `calendar [YEAR]`         | Generate liturgical calendar         |
| `masses [YEAR]`           | Generate mass-centric calendar       |
| `list calendars`          | List available calendars             |
| `list locales`            | List available locales               |
| `martyrology <ID>`        | Lookup a martyrology entry by ID     |
| `search [TEXT]`           | Fuzzy search martyrology entries     |
| `preset`                  | Display current configuration        |
| `bundle`                  | Bundle required data files           |
| `validate`                | Validate definition/resource files   |
| `completions <SHELL>`     | Generate shell completion scripts    |

## Next Steps

- [Installation](./installation) - How to install the CLI
- [Commands Reference](./commands) - Detailed command documentation
- [Examples](./examples) - Common usage patterns
