---
sidebar_position: 4
---

# CLI Examples

Common usage patterns and examples for the Romcal CLI.

## Basic Examples

### Get Easter Date

```bash
# Easter 2025
romcal date easter_sunday 2025
# Output: 2025-04-20

# Easter using Julian calculation
romcal date easter_sunday 2025 --easter-calc julian
# Output: 2025-04-20
```

### Generate a Calendar

```bash
# Full calendar for 2025
romcal calendar 2025

# Only specific fields
romcal calendar 2025 --filter id,fullname,date,rank

# French calendar in French
romcal calendar 2025 --calendar france --locale fr
```

### Export to Different Formats

```bash
# JSON for API consumption
romcal calendar 2025 -f json > calendar.json

# CSV for spreadsheets
romcal calendar 2025 --filter id,fullname,date -f csv > calendar.csv

# YAML for human reading (default)
romcal calendar 2025 -f yaml
```

## Advanced Examples

### Filter by Season

Using `jq` to filter the JSON output:

```bash
# Get only Lent celebrations
romcal calendar 2025 -f json | jq '.[] | select(.season == "LENT")'
```

### Generate Calendars for Multiple Locales

```bash
for locale in en fr es it; do
  romcal calendar 2025 --locale $locale -f json > "calendar-$locale.json"
done
```

### Search for Saints

```bash
# Find all saints named Francis
romcal search "francis" --type SAINT

# Find female martyrs
romcal search --type SAINT --sex FEMALE --title MARTYR

# Find with high relevance only
romcal search "john" --min_score 0.8 --limit 5
```

### Mass Planning

```bash
# Get masses for December (Christmas season)
romcal masses 2025 -f json | jq '.[] | select(.civil_date | startswith("2025-12"))'

# Get vigil masses only
romcal masses 2025 -f json | jq '.[] | select(.mass_time == "VIGIL_MASS")'
```

## Configuration Examples

### Using a Config File

Create `.romcal.toml` in your project:

```toml
calendar = "france"
locale = "fr"
format = "yaml"
epiphany_on_sunday = true
ascension_on_sunday = false
corpus_christi_on_sunday = true
```

Then run commands without specifying options:

```bash
romcal calendar 2025  # Uses config file settings
```

### Override Config with CLI

```bash
# Config says locale=fr, but we want English
romcal calendar 2025 --locale en
```

## Shell Scripting

### Iterate Over Calendar Days

```bash
romcal calendar 2025 -f lines | while read line; do
  echo "Processing: $line"
done
```

### Check if Today is a Solemnity

```bash
TODAY=$(date +%Y-%m-%d)
RANK=$(romcal calendar $(date +%Y) -f json | jq -r ".[] | select(.date == \"$TODAY\") | .rank")

if [ "$RANK" = "SOLEMNITY" ]; then
  echo "Today is a Solemnity!"
fi
```

### Generate Weekly Bulletin Data

```bash
# Get next 7 days of celebrations
START=$(date +%Y-%m-%d)
END=$(date -v+7d +%Y-%m-%d)

romcal calendar $(date +%Y) -f json | jq --arg start "$START" --arg end "$END" \
  '[.[] | select(.date >= $start and .date <= $end)]'
```

## Exit Codes

| Code | Meaning                                                        |
| ---- | -------------------------------------------------------------- |
| 0    | Success                                                        |
| 1    | Error (invalid input, file not found, validation failed, etc.) |
