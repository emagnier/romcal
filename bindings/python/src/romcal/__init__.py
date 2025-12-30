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
from typing import TYPE_CHECKING

# Import types from generated Pydantic models
from .types import (
    CalendarContext,
    CalendarDefinition,
    EasterCalculationType,
    LiturgicalDay,
    MassContext,
    Resources,
)

if TYPE_CHECKING:
    from ._uniffi import romcal_uniffi as _core

__all__ = [
    "CalendarContext",
    "CalendarDefinition",
    "EasterCalculationType",
    "LiturgicalDay",
    "MassContext",
    "Resources",
    "Romcal",
    "RomcalError",
    "get_version",
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
    """Merge multiple resource files (meta.json + entities.*.json) into a single Resources object.

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
        >>> with open("data/resources/fr/entities.a.json") as f:
        ...     entities = json.load(f)
        >>> resources = merge_resource_files("fr", [meta, entities])
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


def __getattr__(name: str) -> str:
    """Lazy load __version__ from the FFI module."""
    if name == "__version__":
        return get_version()
    msg = f"module {__name__!r} has no attribute {name!r}"
    raise AttributeError(msg)


class RomcalError(Exception):
    """Exception raised for Romcal errors."""


def _get_core() -> _core:
    """Lazy import of the UniFFI core module."""
    from . import _uniffi

    return _uniffi.romcal_uniffi


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
            Defaults to EasterCalculationType.GREGORIAN.
        context: Calendar context.
            Defaults to CalendarContext.GREGORIAN.

    Example:
        >>> r = Romcal(calendar="france", locale="fr")
        >>> calendar = r.liturgical_calendar(2025)
        >>> print(len(calendar))  # Number of days in the liturgical year
    """

    def __init__(
        self,
        calendar: str = "general_roman",
        locale: str = "en",
        *,
        epiphany_on_sunday: bool = False,
        ascension_on_sunday: bool = False,
        corpus_christi_on_sunday: bool = True,
        easter_calculation_type: EasterCalculationType = EasterCalculationType.GREGORIAN,
        context: CalendarContext = CalendarContext.GREGORIAN,
        calendar_definitions_json: str | None = None,
        resources_json: str | None = None,
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
            calendar_definitions_json=calendar_definitions_json,
            resources_json=resources_json,
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
