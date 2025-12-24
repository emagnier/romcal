"""Test fixtures for Romcal Python binding (equivalent to fixtures.ts)."""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

import pytest

# Data directory path (relative to this file)
DATA_DIR = Path(__file__).parent.parent.parent.parent / "data"


def load_all_calendar_definitions() -> list[dict[str, Any]]:
    """Load all calendar definitions from the data folder."""
    definitions_dir = DATA_DIR / "definitions"
    definitions: list[dict[str, Any]] = []

    for json_file in definitions_dir.rglob("*.json"):
        with open(json_file) as f:
            definitions.append(json.load(f))

    return definitions


def load_all_resources() -> list[dict[str, Any]]:
    """Load all resources from the data folder.

    Each locale has meta.json + entities.*.json files that need to be merged.
    """
    resources_dir = DATA_DIR / "resources"
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
        entities: dict[str, Any] = {}

        for file in locale_files:
            with open(file) as f:
                content = json.load(f)

            if file.name == "meta.json":
                metadata = content.get("metadata")
            elif file.name.startswith("entities.") and "entities" in content:
                entities.update(content["entities"])

        resources.append(
            {
                "locale": locale,
                "metadata": metadata,
                "entities": entities if entities else None,
            }
        )

    return resources


@pytest.fixture(scope="session")
def calendar_definitions() -> list[dict[str, Any]]:
    """Fixture to load all calendar definitions."""
    return load_all_calendar_definitions()


@pytest.fixture(scope="session")
def resources() -> list[dict[str, Any]]:
    """Fixture to load all resources."""
    return load_all_resources()


@pytest.fixture(scope="session")
def calendar_definitions_json(calendar_definitions: list[dict[str, Any]]) -> str:
    """Fixture to get calendar definitions as JSON string."""
    return json.dumps(calendar_definitions)


@pytest.fixture(scope="session")
def resources_json(resources: list[dict[str, Any]]) -> str:
    """Fixture to get resources as JSON string."""
    return json.dumps(resources)
