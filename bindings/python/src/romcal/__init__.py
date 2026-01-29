"""Romcal - Calendrier liturgique catholique romain.

Romcal is a liturgical calendar library for the Roman Rite of the Catholic Church.
It computes liturgical days, seasons, and Mass contexts for any given year.

Example usage:

    from romcal import Romcal

    # Create a Romcal instance with French calendar and locale
    r = Romcal(calendar="france", locale="fr")

    # Generate the liturgical calendar for 2025
    calendar = r.liturgical_calendar(2025)

    # Access liturgical days
    for date, days in calendar.items():
        for day in days:
            print(f"{date}: {day.name} ({day.rank})")

    # Get a specific celebration date
    christmas = r.get_date("christmas", 2025)
    print(f"Christmas 2025: {christmas}")
"""

from __future__ import annotations

import json
from dataclasses import dataclass, field
from enum import Enum
from typing import TYPE_CHECKING

from pydantic import BaseModel

# Import types from generated Pydantic models
from .types import (
    CalendarContext,
    CalendarDefinition,
    CanonizationLevel,
    EasterCalculationType,
    LiturgicalDay,
    MartyrologyEntry,
    MartyrologyEntryType,
    MassContext,
    Resources,
    Sex,
    Title,
)

if TYPE_CHECKING:
    from ._uniffi import romcal_uniffi as _core

__all__ = [
    "CalendarContext",
    "CalendarDefinition",
    "CanonizationLevel",
    "EasterCalculationType",
    "LiturgicalDay",
    "MartyrologyEntry",
    "MartyrologyEntryType",
    "MartyrologyQuery",
    "MartyrologySearchResult",
    "MassContext",
    "MatchType",
    "Resources",
    "Romcal",
    "RomcalError",
    "Sex",
    "Title",
    "get_bundled_calendar_definitions",
    "get_bundled_resources",
    "get_version",
    "has_bundled_data",
    "merge_calendar_definitions",
    "merge_resource_files",
]


def get_version() -> str:
    """Get the romcal library version.

    Returns:
        The version string (e.g., "4.0.0-beta.3").
    """
    return _get_core().version()


def merge_resource_files(locale: str, files: list[dict]) -> Resources:
    """Merge multiple resource files (meta.json + martyrology.*.json) into a single Resources object.

    This helper allows you to load resource files however you want and then
    merge them into the expected structure.

    Args:
        locale: The locale code (e.g., "fr", "en")
        files: A list of parsed JSON dicts from resource files

    Returns:
        A merged Resources object

    Example:
        >>> import json
        >>> with open("data/resources/fr/meta.json") as f:
        ...     meta = json.load(f)
        >>> with open("data/resources/fr/martyrology.a.json") as f:
        ...     martyrology = json.load(f)
        >>> resources = merge_resource_files("fr", [meta, martyrology])
    """
    core = _get_core()
    files_json = [json.dumps(f) for f in files]
    result_json = core.merge_resource_files(locale, files_json)
    return Resources.model_validate(json.loads(result_json))


def merge_calendar_definitions(files: list[dict]) -> list[CalendarDefinition]:
    """Merge/validate multiple calendar definition files.

    This helper allows you to load calendar definition files however you want
    and then validate them into the expected structure.

    Args:
        files: A list of parsed JSON dicts from calendar definition files

    Returns:
        A list of validated CalendarDefinition objects

    Example:
        >>> import json
        >>> with open("data/definitions/france.json") as f:
        ...     france = json.load(f)
        >>> definitions = merge_calendar_definitions([france])
    """
    core = _get_core()
    files_json = [json.dumps(f) for f in files]
    result_json = core.merge_calendar_definitions(files_json)
    raw_list = json.loads(result_json)
    return [CalendarDefinition.model_validate(d) for d in raw_list]


def has_bundled_data() -> bool:
    """Check if bundled data is available.

    Returns:
        True if the library was compiled with embedded calendar definitions
        and resources data. This is typically the case when installing from PyPI.
    """
    return _get_core().has_bundled_data()


def get_bundled_calendar_definitions() -> list[CalendarDefinition]:
    """Get all bundled calendar definitions.

    Returns all calendar definitions (general_roman, countries, regions, dioceses)
    embedded in the binary. This includes ~69 calendars.

    Returns:
        A list of CalendarDefinition objects.

    Raises:
        RomcalError: If bundled data is not available (library compiled without it).

    Example:
        >>> if has_bundled_data():
        ...     definitions = get_bundled_calendar_definitions()
        ...     print(f"Loaded {len(definitions)} calendars")
    """
    core = _get_core()
    try:
        result_json = core.get_bundled_calendar_definitions()
        raw_list = json.loads(result_json)
        return [CalendarDefinition.model_validate(d) for d in raw_list]
    except core.RomcalError as e:
        raise RomcalError(str(e)) from e


def get_bundled_resources() -> list[Resources]:
    """Get all bundled locale resources.

    Returns all locale resources (en, fr, es, it, de, la, etc.)
    embedded in the binary. This includes ~13 locales.

    Returns:
        A list of Resources objects.

    Raises:
        RomcalError: If bundled data is not available (library compiled without it).

    Example:
        >>> if has_bundled_data():
        ...     resources = get_bundled_resources()
        ...     locales = [r.locale for r in resources]
        ...     print(f"Available locales: {locales}")
    """
    core = _get_core()
    try:
        result_json = core.get_bundled_resources()
        raw_list = json.loads(result_json)
        return [Resources.model_validate(r) for r in raw_list]
    except core.RomcalError as e:
        raise RomcalError(str(e)) from e


def __getattr__(name: str) -> str:
    """Lazy load __version__ from the FFI module."""
    if name == "__version__":
        return get_version()
    msg = f"module {__name__!r} has no attribute {name!r}"
    raise AttributeError(msg)


class RomcalError(Exception):
    """Exception raised for Romcal errors."""


class MatchType(Enum):
    """Type of match that was found for a search result."""

    exact_id = "exact_id"
    """Exact ID match (score = 1.0)."""

    fuzzy = "fuzzy"
    """Fuzzy match on text fields (score < 1.0)."""

    filter_only = "filter_only"
    """Match by filters only (no text query provided)."""


@dataclass
class MartyrologyQuery:
    """Query parameters for searching martyrology entries.

    All fields are optional. When a field is None, it is not used for filtering.
    When `text` is provided, fuzzy matching is performed on entry ID, fullname, and name.

    Example:
        >>> from romcal import MartyrologyQuery, CanonizationLevel, Title
        >>> query = MartyrologyQuery(text="francis", canonization_level=CanonizationLevel.saint)
        >>> results = romcal.search_martyrology(query)
    """

    text: str | None = None
    """Fuzzy text search on id, fullname, and name fields."""

    entry_type: MartyrologyEntryType | None = None
    """Filter by entry type."""

    canonization_level: CanonizationLevel | None = None
    """Filter by canonization level."""

    sex: Sex | None = None
    """Filter by sex."""

    titles: list[Title] | None = None
    """Filter by titles. Entry must have at least one of the specified titles."""

    limit: int | None = None
    """Maximum number of results to return. Default: 20."""

    min_score: float | None = None
    """Minimum score threshold (0.0 to 1.0). Default: 0.3."""

    def _to_json_dict(self) -> dict:
        """Convert to JSON-compatible dict with snake_case keys."""
        d: dict = {}
        if self.text is not None:
            d["text"] = self.text
        if self.entry_type is not None:
            d["entry_type"] = self.entry_type.value
        if self.canonization_level is not None:
            d["canonization_level"] = self.canonization_level.value
        if self.sex is not None:
            d["sex"] = self.sex.value
        if self.titles is not None:
            d["titles"] = [t.value for t in self.titles]
        if self.limit is not None:
            d["limit"] = self.limit
        if self.min_score is not None:
            d["min_score"] = self.min_score
        return d


@dataclass
class MartyrologySearchResult:
    """Result of a martyrology search.

    Attributes:
        entry: The matched martyrology entry.
        score: Match score from 0.0 to 1.0, where 1.0 is a perfect match.
        match_type: Type of match that was found.
        matched_fields: Names of fields that matched the query.
    """

    entry: MartyrologyEntry
    score: float
    match_type: MatchType
    matched_fields: list[str] = field(default_factory=list)

    @classmethod
    def _from_json_dict(cls, d: dict) -> MartyrologySearchResult:
        """Create from JSON dict with snake_case keys."""
        return cls(
            entry=MartyrologyEntry.model_validate(d["entry"]),
            score=d["score"],
            match_type=MatchType(d["match_type"]),
            matched_fields=d.get("matched_fields", []),
        )


def _get_core() -> _core:
    """Lazy import of the UniFFI core module."""
    from . import _uniffi

    return _uniffi.romcal_uniffi


def _serialize_to_json(items: list[BaseModel | dict] | None) -> str | None:
    """Serialize a list of Pydantic models or dicts to JSON string."""
    if items is None:
        return None
    serialized = [
        item.model_dump(mode="json", by_alias=True, exclude_none=True)
        if isinstance(item, BaseModel)
        else item
        for item in items
    ]
    return json.dumps(serialized)


class Romcal:
    """Liturgical calendar for the Roman Rite of the Catholic Church.

    Computes liturgical days, seasons, and Mass contexts for any given year.
    Supports various regional calendars and locales.

    Args:
        calendar: Calendar type (e.g., 'general_roman', 'france', 'usa').
            Defaults to 'general_roman'.
        locale: Locale for translations (e.g., 'en', 'fr', 'es').
            Defaults to 'en'.
        epiphany_on_sunday: Whether Epiphany is celebrated on Sunday.
            Defaults to False.
        ascension_on_sunday: Whether Ascension is celebrated on Sunday.
            Defaults to False.
        corpus_christi_on_sunday: Whether Corpus Christi is celebrated on Sunday.
            Defaults to True.
        easter_calculation_type: Easter calculation method.
            Defaults to EasterCalculationType.gregorian.
        context: Calendar context.
            Defaults to CalendarContext.gregorian.
        calendar_definitions: List of calendar definitions (Pydantic models or dicts).
            Use get_bundled_calendar_definitions() to get built-in definitions.
        resources: List of locale resources (Pydantic models or dicts).
            Use get_bundled_resources() to get built-in resources.

    Example:
        >>> r = Romcal(calendar="france", locale="fr")
        >>> calendar = r.liturgical_calendar(2025)
        >>> print(len(calendar))  # Number of days in the liturgical year

    Example with custom calendar:
        >>> from romcal import Romcal, get_bundled_calendar_definitions, get_bundled_resources
        >>> my_parish = {"id": "my_parish", "parent_calendar_ids": ["france"], ...}
        >>> r = Romcal(
        ...     calendar="my_parish",
        ...     locale="fr",
        ...     calendar_definitions=[*get_bundled_calendar_definitions(), my_parish],
        ...     resources=get_bundled_resources(),
        ... )
    """

    def __init__(
        self,
        calendar: str = "general_roman",
        locale: str = "en",
        *,
        epiphany_on_sunday: bool = False,
        ascension_on_sunday: bool = False,
        corpus_christi_on_sunday: bool = True,
        easter_calculation_type: EasterCalculationType = EasterCalculationType.gregorian,
        context: CalendarContext = CalendarContext.gregorian,
        calendar_definitions: list[CalendarDefinition | dict] | None = None,
        resources: list[Resources | dict] | None = None,
    ) -> None:
        core = _get_core()
        config = core.RomcalConfig(
            calendar=calendar,
            locale=locale,
            epiphany_on_sunday=epiphany_on_sunday,
            ascension_on_sunday=ascension_on_sunday,
            corpus_christi_on_sunday=corpus_christi_on_sunday,
            easter_calculation_type=easter_calculation_type.value,
            context=context.value,
            calendar_definitions_json=_serialize_to_json(calendar_definitions),
            resources_json=_serialize_to_json(resources),
        )
        try:
            self._inner = core.Romcal(config)
        except core.RomcalError as e:
            raise RomcalError(str(e)) from e

    def __repr__(self) -> str:
        """Return a string representation for debugging."""
        return (
            f"Romcal("
            f"calendar={self.calendar!r}, "
            f"locale={self.locale!r}, "
            f"context={self.context.name}, "
            f"easter_calculation_type={self.easter_calculation_type.name}, "
            f"epiphany_on_sunday={self.epiphany_on_sunday}, "
            f"ascension_on_sunday={self.ascension_on_sunday}, "
            f"corpus_christi_on_sunday={self.corpus_christi_on_sunday})"
        )

    @property
    def calendar(self) -> str:
        """Get the calendar type."""
        return self._inner.get_calendar()

    @property
    def locale(self) -> str:
        """Get the locale."""
        return self._inner.get_locale()

    @property
    def epiphany_on_sunday(self) -> bool:
        """Whether Epiphany is celebrated on Sunday."""
        return self._inner.get_epiphany_on_sunday()

    @property
    def ascension_on_sunday(self) -> bool:
        """Whether Ascension is celebrated on Sunday."""
        return self._inner.get_ascension_on_sunday()

    @property
    def corpus_christi_on_sunday(self) -> bool:
        """Whether Corpus Christi is celebrated on Sunday."""
        return self._inner.get_corpus_christi_on_sunday()

    @property
    def easter_calculation_type(self) -> EasterCalculationType:
        """Get the Easter calculation type."""
        return EasterCalculationType(self._inner.get_easter_calculation_type())

    @property
    def context(self) -> CalendarContext:
        """Get the calendar context."""
        return CalendarContext(self._inner.get_context())

    def liturgical_calendar(self, year: int) -> dict[str, list[LiturgicalDay]]:
        """Generate the complete liturgical calendar for a given liturgical year.

        Args:
            year: The liturgical year to generate (e.g., 2025).

        Returns:
            A dict mapping date strings (YYYY-MM-DD) to lists of LiturgicalDay objects.
            Each date may have multiple liturgical days due to optional memorials.

        Raises:
            RomcalError: If the year is invalid or calendar generation fails.

        Example:
            >>> r = Romcal()
            >>> calendar = r.liturgical_calendar(2025)
            >>> christmas_days = calendar.get("2025-12-25", [])
            >>> for day in christmas_days:
            ...     print(f"{day.id}: {day.rank}")
        """
        core = _get_core()
        try:
            raw = json.loads(self._inner.generate_liturgical_calendar(year))
            return {
                date: [LiturgicalDay.model_validate(d) for d in days] for date, days in raw.items()
            }
        except core.RomcalError as e:
            raise RomcalError(str(e)) from e
        except json.JSONDecodeError as e:
            msg = f"Failed to parse calendar JSON: {e}"
            raise RomcalError(msg) from e

    def mass_calendar(self, year: int) -> dict[str, list[MassContext]]:
        """Generate a mass-centric view of the liturgical calendar for a given year.

        This provides Mass-specific information including readings, prayers,
        and other elements needed for celebrating the Eucharist.

        Args:
            year: The year to generate (e.g., 2025).

        Returns:
            A dict mapping date strings (YYYY-MM-DD) to lists of MassContext objects.

        Raises:
            RomcalError: If the year is invalid or calendar generation fails.

        Example:
            >>> r = Romcal()
            >>> masses = r.mass_calendar(2025)
            >>> christmas_masses = masses.get("2025-12-25", [])
        """
        core = _get_core()
        try:
            raw = json.loads(self._inner.generate_mass_calendar(year))
            return {
                date: [MassContext.model_validate(m) for m in masses]
                for date, masses in raw.items()
            }
        except core.RomcalError as e:
            raise RomcalError(str(e)) from e
        except json.JSONDecodeError as e:
            msg = f"Failed to parse calendar JSON: {e}"
            raise RomcalError(msg) from e

    def get_date(self, celebration_id: str, year: int) -> str:
        """Get the date of a specific celebration by its ID.

        Args:
            celebration_id: The unique identifier of the celebration (e.g., 'christmas', 'easter').
            year: The year to look up.

        Returns:
            The date in YYYY-MM-DD format.

        Raises:
            RomcalError: If the celebration is not found or the year is invalid.

        Example:
            >>> r = Romcal()
            >>> easter = r.get_date("easter", 2025)
            >>> print(easter)  # '2025-04-20'
        """
        core = _get_core()
        try:
            return self._inner.get_date(celebration_id, year)
        except core.RomcalError as e:
            raise RomcalError(str(e)) from e

    def get_martyrology_entry(self, entry_id: str) -> MartyrologyEntry | None:
        """Get a martyrology entry by its exact ID.

        Args:
            entry_id: The unique identifier of the entry (e.g., 'agnes_of_rome_virgin').

        Returns:
            The MartyrologyEntry object, or None if not found.

        Example:
            >>> r = Romcal()
            >>> entry = r.get_martyrology_entry("agnes_of_rome_virgin")
            >>> if entry:
            ...     print(f"{entry.name} ({entry.canonization_level})")
        """
        entry_json = self._inner.get_martyrology_entry(entry_id)
        if entry_json is None:
            return None
        return MartyrologyEntry.model_validate(json.loads(entry_json))

    def search_martyrology(self, query: MartyrologyQuery) -> list[MartyrologySearchResult]:
        """Search martyrology entries with fuzzy matching and filters.

        Args:
            query: Search parameters including text, filters, and options.

        Returns:
            A list of MartyrologySearchResult sorted by score (highest first).

        Raises:
            RomcalError: If the search fails.

        Example:
            >>> r = Romcal()
            >>> query = MartyrologyQuery(text="francis", canonization_level="saint")
            >>> results = r.search_martyrology(query)
            >>> for result in results:
            ...     print(f"{result.entry.name}: {result.score:.2f}")
        """
        core = _get_core()
        try:
            query_json = json.dumps(query._to_json_dict())
            results_json = self._inner.search_martyrology(query_json)
            raw_results = json.loads(results_json)
            return [MartyrologySearchResult._from_json_dict(r) for r in raw_results]
        except core.RomcalError as e:
            raise RomcalError(str(e)) from e
        except json.JSONDecodeError as e:
            msg = f"Failed to parse search results JSON: {e}"
            raise RomcalError(msg) from e
