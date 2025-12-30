"""Test fixtures for Romcal Python binding (equivalent to fixtures.ts)."""

from __future__ import annotations

import json
from pathlib import Path

import pytest

from romcal import (
    CalendarDefinition,
    Resources,
    merge_calendar_definitions,
    merge_resource_files,
)

# Data directory path (relative to this file)
DATA_DIR = Path(__file__).parent.parent.parent.parent / "data"


def load_all_calendar_definitions() -> list[CalendarDefinition]:
    """Load all calendar definitions from the data folder."""
    definitions_dir = DATA_DIR / "definitions"
    files: list[dict] = []

    for json_file in definitions_dir.rglob("*.json"):
        with json_file.open() as f:
            files.append(json.load(f))

    return merge_calendar_definitions(files)


def load_all_resources() -> list[Resources]:
    """Load all resources from the data folder.

    Each locale has meta.json + entities.*.json files that need to be merged.
    """
    resources_dir = DATA_DIR / "resources"
    resources: list[Resources] = []

    # Group files by locale (parent directory name)
    files_by_locale: dict[str, list[Path]] = {}
    for json_file in resources_dir.rglob("*.json"):
        locale = json_file.parent.name
        if locale not in files_by_locale:
            files_by_locale[locale] = []
        files_by_locale[locale].append(json_file)

    # Merge files for each locale using the helper
    for locale, locale_files in files_by_locale.items():
        files_content = []
        for locale_file in locale_files:
            with locale_file.open() as fp:
                files_content.append(json.load(fp))
        resources.append(merge_resource_files(locale, files_content))

    return resources


@pytest.fixture(scope="session")
def calendar_definitions() -> list[CalendarDefinition]:
    """Fixture to load all calendar definitions."""
    return load_all_calendar_definitions()


@pytest.fixture(scope="session")
def resources() -> list[Resources]:
    """Fixture to load all resources."""
    return load_all_resources()


@pytest.fixture(scope="session")
def calendar_definitions_json(calendar_definitions: list[CalendarDefinition]) -> str:
    """Fixture to get calendar definitions as JSON string."""
    return json.dumps(
        [d.model_dump(mode="json", by_alias=True, exclude_none=True) for d in calendar_definitions]
    )


@pytest.fixture(scope="session")
def resources_json(resources: list[Resources]) -> str:
    """Fixture to get resources as JSON string."""
    return json.dumps(
        [r.model_dump(mode="json", by_alias=True, exclude_none=True) for r in resources]
    )
