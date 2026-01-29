#!/usr/bin/env python3
"""
Romcal - Basic Usage Example

This example demonstrates:
- Loading calendar definitions from the data folder
- Loading and merging resources (translations) from the data folder
- Creating a Romcal instance with loaded data
- Generating calendars with translated names

Run with: python examples/basic_usage.py
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

from romcal import Romcal


def load_calendar_definitions(data_dir: Path) -> list[dict[str, Any]]:
    """Load all calendar definitions from the data folder."""
    definitions_dir = data_dir / "definitions"
    definitions: list[dict[str, Any]] = []

    for json_file in definitions_dir.rglob("*.json"):
        with open(json_file) as f:
            definitions.append(json.load(f))

    return definitions


def load_resources(data_dir: Path) -> list[dict[str, Any]]:
    """Load all resources from the data folder.

    Each locale has meta.json + martyrology.*.json files that need to be merged.
    """
    resources_dir = data_dir / "resources"
    resources: list[dict[str, Any]] = []

    # Group files by locale (parent directory name)
    files_by_locale: dict[str, list[Path]] = {}
    for json_file in resources_dir.rglob("*.json"):
        locale = json_file.parent.name
        if locale not in files_by_locale:
            files_by_locale[locale] = []
        files_by_locale[locale].append(json_file)

    # Merge files for each locale
    for locale, locale_files in files_by_locale.items():
        metadata: dict[str, Any] | None = None
        martyrology: dict[str, Any] = {}

        for file in locale_files:
            with open(file) as f:
                content = json.load(f)

            if file.name == "meta.json":
                metadata = content.get("metadata")
            elif file.name.startswith("martyrology.") and "martyrology" in content:
                martyrology.update(content["martyrology"])

        resources.append(
            {
                "locale": locale,
                "metadata": metadata,
                "martyrology": martyrology if martyrology else None,
            }
        )

    return resources


def main() -> None:
    print("=== Romcal with Data Example ===\n")

    # Determine the data directory path
    # This assumes running from bindings/python/
    script_dir = Path(__file__).parent
    data_dir = script_dir.parent.parent.parent / "data"

    if not data_dir.exists():
        print(f"Error: Data directory not found at {data_dir}")
        print("Please run this script from the bindings/python directory.")
        return

    # ========================================================================
    # Load Data
    # ========================================================================

    print("Loading calendar definitions...")
    calendar_definitions = load_calendar_definitions(data_dir)
    print(f"  Loaded {len(calendar_definitions)} calendar definitions")

    print("Loading resources...")
    resources = load_resources(data_dir)
    print(f"  Loaded {len(resources)} locale resources")
    print()

    # ========================================================================
    # Create Romcal with French Calendar and Locale
    # ========================================================================

    print("Creating French calendar instance with loaded data...")
    romcal = Romcal(
        calendar="france",
        locale="fr",
        calendar_definitions=calendar_definitions,
        resources=resources,
    )
    print(f"  Calendar: {romcal.calendar}")
    print(f"  Locale: {romcal.locale}")
    print()

    # ========================================================================
    # Generate Calendar
    # ========================================================================

    print("Generating liturgical calendar for 2026...")
    calendar = romcal.liturgical_calendar(2026)
    dates = sorted(calendar.keys())
    print(f"  Total dates: {len(dates)}")
    print(f"  First date: {dates[0]}")
    print(f"  Last date: {dates[-1]}")
    print()

    # ========================================================================
    # Notable Celebrations with French Names
    # ========================================================================

    print("Celebrations with French names:")

    # Easter 2026 (April 5)
    easter = calendar.get("2026-04-05")
    if easter:
        day = easter[0]
        print("  Easter (2026-04-05):")
        print(f"    Full name: {day.get('fullname', day['id'])}")
        print(f"    Season: {day.get('season_name', day['season'])}")
        print(f"    Rank: {day.get('rank_name', day['rank'])}")

    # Assumption of Mary (August 15)
    assumption = calendar.get("2026-08-15")
    if assumption:
        day = assumption[0]
        print(f"  Assumption (2026-08-15): {day.get('fullname', day['id'])}")

    # All Saints (November 1)
    all_saints = calendar.get("2026-11-01")
    if all_saints:
        day = all_saints[0]
        print(f"  All Saints (2026-11-01): {day.get('fullname', day['id'])}")

    # Christmas 2026
    christmas = calendar.get("2026-12-25")
    if christmas:
        day = christmas[0]
        print(f"  Christmas (2026-12-25): {day.get('fullname', day['id'])}")

    # French-specific saint: Saint Jean-Marie Vianney (August 4)
    vianney_day = calendar.get("2026-08-04")
    if vianney_day:
        for day in vianney_day:
            if "vianney" in day.get("id", "").lower():
                print(f"  St. Jean-Marie Vianney (2026-08-04): {day.get('fullname', day['id'])}")
                break

    print()

    # ========================================================================
    # Mass Calendar with French Names
    # ========================================================================

    print("Generating mass calendar for 2026...")
    mass_calendar = romcal.mass_calendar(2026)
    print(f"  Total dates with masses: {len(mass_calendar)}")
    print()

    # Christmas masses with French names
    christmas_masses = mass_calendar.get("2026-12-25")
    if christmas_masses:
        print(f"  Christmas masses ({len(christmas_masses)} total):")
        for mass in christmas_masses:
            mass_time_name = mass.get("mass_time_name", mass["mass_time"])
            fullname = mass.get("fullname", mass["id"])
            print(f"    - {mass_time_name}: {fullname}")

    print("\n=== Done ===")


if __name__ == "__main__":
    main()
