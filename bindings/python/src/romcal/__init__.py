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
from typing import TYPE_CHECKING, Any

# Import types from generated Pydantic models
from .types import (
    CalendarContext,
    CalendarDefinition,
    EasterCalculationType,
)

if TYPE_CHECKING:
    from ._uniffi import romcal_uniffi as _core

__version__ = "4.0.0-beta.1"
__all__ = [
    "CalendarContext",
    "CalendarDefinition",
    "EasterCalculationType",
    "Romcal",
    "RomcalError",
]

# Minimum year for Gregorian calendar calculations
MIN_YEAR = 1583


class RomcalError(Exception):
    """Exception raised for Romcal errors."""


def _validate_year(year: int) -> None:
    """Validate that the year is valid for Gregorian calendar calculations.

    Args:
        year: The year to validate.

    Raises:
        RomcalError: If the year is less than 1583.
    """
    if year < MIN_YEAR:
        msg = f"Year must be >= {MIN_YEAR} for the Gregorian calendar, got {year}"
        raise RomcalError(msg)


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

    def liturgical_calendar(self, year: int) -> dict[str, list[dict[str, Any]]]:
        """Generate the complete liturgical calendar for a given liturgical year.

        Args:
            year: The liturgical year to generate (e.g., 2025).

        Returns:
            A dict mapping date strings (YYYY-MM-DD) to lists of liturgical day dicts.
            Each date may have multiple liturgical days due to optional memorials.

        Raises:
            RomcalError: If the year is invalid or calendar generation fails.

        Example:
            >>> r = Romcal()
            >>> calendar = r.liturgical_calendar(2025)
            >>> christmas_days = calendar.get("2025-12-25", [])
            >>> for day in christmas_days:
            ...     print(f"{day['id']}: {day['rank']}")
        """
        _validate_year(year)
        core = _get_core()
        try:
            return json.loads(self._inner.generate_liturgical_calendar(year))
        except core.RomcalError as e:
            raise RomcalError(str(e)) from e
        except json.JSONDecodeError as e:
            msg = f"Failed to parse calendar JSON: {e}"
            raise RomcalError(msg) from e

    def mass_calendar(self, year: int) -> dict[str, list[dict[str, Any]]]:
        """Generate a mass-centric view of the liturgical calendar for a given year.

        This provides Mass-specific information including readings, prayers,
        and other elements needed for celebrating the Eucharist.

        Args:
            year: The year to generate (e.g., 2025).

        Returns:
            A dict mapping date strings (YYYY-MM-DD) to lists of mass context dicts.

        Raises:
            RomcalError: If the year is invalid or calendar generation fails.

        Example:
            >>> r = Romcal()
            >>> masses = r.mass_calendar(2025)
            >>> christmas_masses = masses.get("2025-12-25", [])
        """
        _validate_year(year)
        core = _get_core()
        try:
            return json.loads(self._inner.generate_mass_calendar(year))
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
        _validate_year(year)
        core = _get_core()
        try:
            return self._inner.get_date(celebration_id, year)
        except core.RomcalError as e:
            raise RomcalError(str(e)) from e
