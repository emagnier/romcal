"""Tests for bundled data functionality.

These tests verify that the Python wheel includes embedded calendar definitions
and resources when compiled with the `bundled-data` feature.
"""

import pytest

from romcal import (
    CalendarDefinition,
    Resources,
    Romcal,
    RomcalError,
    get_bundled_calendar_definitions,
    get_bundled_resources,
    has_bundled_data,
)

# =============================================================================
# Fixtures
# =============================================================================


@pytest.fixture(scope="module")
def bundled_definitions() -> list[CalendarDefinition]:
    """Load bundled calendar definitions once for all tests in this module."""
    if not has_bundled_data():
        pytest.skip("Bundled data not available")
    return get_bundled_calendar_definitions()


@pytest.fixture(scope="module")
def bundled_resources() -> list[Resources]:
    """Load bundled resources once for all tests in this module."""
    if not has_bundled_data():
        pytest.skip("Bundled data not available")
    return get_bundled_resources()


# =============================================================================
# Availability Tests
# =============================================================================


class TestBundledDataAvailability:
    """Tests for bundled data availability."""

    def test_has_bundled_data_returns_bool(self) -> None:
        """has_bundled_data() should return a boolean."""
        result = has_bundled_data()
        assert isinstance(result, bool)

    @pytest.mark.skipif(not has_bundled_data(), reason="Bundled data not available")
    def test_has_bundled_data_is_true_when_compiled_with_feature(self) -> None:
        """has_bundled_data() should return True when compiled with bundled-data feature."""
        assert has_bundled_data() is True


# =============================================================================
# Calendar Definitions Tests
# =============================================================================


class TestBundledCalendarDefinitions:
    """Tests for bundled calendar definitions."""

    def test_get_bundled_calendar_definitions_returns_list(
        self, bundled_definitions: list[CalendarDefinition]
    ) -> None:
        """get_bundled_calendar_definitions() should return a list."""
        assert isinstance(bundled_definitions, list)

    def test_bundled_calendar_definitions_not_empty(
        self, bundled_definitions: list[CalendarDefinition]
    ) -> None:
        """Bundled calendar definitions should not be empty."""
        assert len(bundled_definitions) > 0

    def test_bundled_calendar_definitions_count(
        self, bundled_definitions: list[CalendarDefinition]
    ) -> None:
        """Bundled calendar definitions should have expected count (~69 calendars)."""
        # Allow some flexibility for future additions/removals
        assert len(bundled_definitions) >= 60, (
            f"Expected at least 60 calendars, got {len(bundled_definitions)}"
        )
        assert len(bundled_definitions) <= 100, (
            f"Expected at most 100 calendars, got {len(bundled_definitions)}"
        )

    def test_bundled_calendar_definitions_are_valid(
        self, bundled_definitions: list[CalendarDefinition]
    ) -> None:
        """Each bundled calendar definition should be a valid CalendarDefinition."""
        for definition in bundled_definitions:
            assert isinstance(definition, CalendarDefinition)
            assert definition.id is not None
            assert isinstance(definition.id, str)
            assert len(definition.id) > 0

    def test_bundled_calendar_definitions_have_unique_ids(
        self, bundled_definitions: list[CalendarDefinition]
    ) -> None:
        """Each bundled calendar definition should have a unique ID."""
        ids = [d.id for d in bundled_definitions]
        assert len(ids) == len(set(ids)), "Duplicate calendar IDs found"

    def test_bundled_calendar_definitions_include_general_roman(
        self, bundled_definitions: list[CalendarDefinition]
    ) -> None:
        """Bundled calendar definitions should include general_roman."""
        ids = [d.id for d in bundled_definitions]
        assert "general_roman" in ids

    def test_bundled_calendar_definitions_include_major_calendars(
        self, bundled_definitions: list[CalendarDefinition]
    ) -> None:
        """Bundled calendar definitions should include major national calendars."""
        ids = [d.id for d in bundled_definitions]

        major_calendars = [
            "france",
            "united_states",
            "italy",
            "germany",
            "spain",
            "brazil",
            "mexico",
            "poland",
            "canada",
            "australia",
        ]
        for calendar in major_calendars:
            assert calendar in ids, f"Missing calendar: {calendar}"


# =============================================================================
# Resources Tests
# =============================================================================


class TestBundledResources:
    """Tests for bundled resources."""

    def test_get_bundled_resources_returns_list(self, bundled_resources: list[Resources]) -> None:
        """get_bundled_resources() should return a list."""
        assert isinstance(bundled_resources, list)

    def test_bundled_resources_not_empty(self, bundled_resources: list[Resources]) -> None:
        """Bundled resources should not be empty."""
        assert len(bundled_resources) > 0

    def test_bundled_resources_count(self, bundled_resources: list[Resources]) -> None:
        """Bundled resources should have expected count (~13-14 locales)."""
        # Allow some flexibility for future additions/removals
        assert len(bundled_resources) >= 10, (
            f"Expected at least 10 locales, got {len(bundled_resources)}"
        )
        assert len(bundled_resources) <= 30, (
            f"Expected at most 30 locales, got {len(bundled_resources)}"
        )

    def test_bundled_resources_are_valid(self, bundled_resources: list[Resources]) -> None:
        """Each bundled resource should be a valid Resources object."""
        for resource in bundled_resources:
            assert isinstance(resource, Resources)
            assert resource.locale is not None
            assert isinstance(resource.locale, str)
            assert len(resource.locale) > 0

    def test_bundled_resources_have_unique_locales(
        self, bundled_resources: list[Resources]
    ) -> None:
        """Each bundled resource should have a unique locale."""
        locales = [r.locale for r in bundled_resources]
        assert len(locales) == len(set(locales)), "Duplicate locales found"

    def test_bundled_resources_include_english(self, bundled_resources: list[Resources]) -> None:
        """Bundled resources should include English locale."""
        locales = [r.locale for r in bundled_resources]
        assert "en" in locales

    def test_bundled_resources_include_major_locales(
        self, bundled_resources: list[Resources]
    ) -> None:
        """Bundled resources should include major locales."""
        locales = [r.locale for r in bundled_resources]

        major_locales = ["en", "fr", "es", "it", "de", "la", "pl"]
        for locale in major_locales:
            assert locale in locales, f"Missing locale: {locale}"

    def test_bundled_resources_have_entities(self, bundled_resources: list[Resources]) -> None:
        """Each bundled resource should have entities."""
        for resource in bundled_resources:
            assert resource.entities is not None
            assert len(resource.entities) > 0, f"No entities for locale: {resource.locale}"


# =============================================================================
# Romcal Creation Tests
# =============================================================================


class TestRomcalWithBundledData:
    """Tests for creating Romcal instances with bundled data."""

    def test_create_romcal_with_bundled_data(
        self, bundled_definitions: list[CalendarDefinition], bundled_resources: list[Resources]
    ) -> None:
        """Should create Romcal instance using bundled data."""
        romcal = Romcal(
            calendar="general_roman",
            locale="en",
            calendar_definitions=bundled_definitions,
            resources=bundled_resources,
        )

        assert romcal.calendar == "general_roman"
        assert romcal.locale == "en"

    @pytest.mark.parametrize(
        ("calendar", "locale"),
        [
            ("general_roman", "en"),
            ("general_roman", "la"),
            ("france", "fr"),
            ("germany", "de"),
            ("italy", "it"),
            ("spain", "es"),
            ("brazil", "pt-br"),
            ("united_states", "en"),
        ],
    )
    def test_create_romcal_with_various_calendar_locale_combinations(
        self,
        bundled_definitions: list[CalendarDefinition],
        bundled_resources: list[Resources],
        calendar: str,
        locale: str,
    ) -> None:
        """Should create Romcal with various calendar/locale combinations."""
        romcal = Romcal(
            calendar=calendar,
            locale=locale,
            calendar_definitions=bundled_definitions,
            resources=bundled_resources,
        )

        assert romcal.calendar == calendar
        assert romcal.locale == locale

        # Verify it can generate a calendar
        cal = romcal.liturgical_calendar(2026)
        assert len(cal) > 300  # Should have most days of the year


# =============================================================================
# Calendar Generation Tests
# =============================================================================


class TestCalendarGenerationWithBundledData:
    """Tests for calendar generation using bundled data."""

    def test_generate_liturgical_calendar(
        self, bundled_definitions: list[CalendarDefinition], bundled_resources: list[Resources]
    ) -> None:
        """Should generate liturgical calendar using bundled data."""
        romcal = Romcal(
            calendar="france",
            locale="fr",
            calendar_definitions=bundled_definitions,
            resources=bundled_resources,
        )

        calendar = romcal.liturgical_calendar(2026)

        # Verify calendar is generated
        assert len(calendar) > 0

        # Verify Easter is present
        easter = calendar.get("2026-04-05")
        assert easter is not None
        assert len(easter) > 0
        assert easter[0].fullname is not None

    def test_generate_mass_calendar(
        self, bundled_definitions: list[CalendarDefinition], bundled_resources: list[Resources]
    ) -> None:
        """Should generate mass calendar using bundled data."""
        romcal = Romcal(
            calendar="general_roman",
            locale="en",
            calendar_definitions=bundled_definitions,
            resources=bundled_resources,
        )

        mass_calendar = romcal.mass_calendar(2026)

        # Verify mass calendar is generated
        assert len(mass_calendar) > 0

        # Verify Christmas is present with multiple masses
        christmas = mass_calendar.get("2026-12-25")
        assert christmas is not None
        assert len(christmas) >= 3  # Night, Dawn, Day masses

    def test_get_date_with_bundled_data(
        self, bundled_definitions: list[CalendarDefinition], bundled_resources: list[Resources]
    ) -> None:
        """Should get specific celebration dates using bundled data."""
        romcal = Romcal(
            calendar="general_roman",
            locale="en",
            calendar_definitions=bundled_definitions,
            resources=bundled_resources,
        )

        # Test well-known dates (using actual celebration IDs)
        easter_2026 = romcal.get_date("easter_sunday", 2026)
        assert easter_2026 == "2026-04-05"

        christmas_2026 = romcal.get_date("nativity_of_the_lord", 2026)
        assert christmas_2026 == "2026-12-25"

        # Pentecost is 50 days after Easter
        pentecost_2026 = romcal.get_date("pentecost_sunday", 2026)
        assert pentecost_2026 == "2026-05-24"

    def test_get_date_error_for_invalid_celebration(
        self, bundled_definitions: list[CalendarDefinition], bundled_resources: list[Resources]
    ) -> None:
        """Should raise error for invalid celebration ID."""
        romcal = Romcal(
            calendar="general_roman",
            locale="en",
            calendar_definitions=bundled_definitions,
            resources=bundled_resources,
        )

        with pytest.raises(RomcalError) as exc_info:
            romcal.get_date("nonexistent_celebration", 2026)

        assert "nonexistent_celebration" in str(exc_info.value)

    def test_localized_names_with_bundled_data(
        self, bundled_definitions: list[CalendarDefinition], bundled_resources: list[Resources]
    ) -> None:
        """Should have properly localized names."""
        # French
        romcal_fr = Romcal(
            calendar="france",
            locale="fr",
            calendar_definitions=bundled_definitions,
            resources=bundled_resources,
        )
        cal_fr = romcal_fr.liturgical_calendar(2026)
        easter_fr = cal_fr.get("2026-04-05")
        assert easter_fr is not None
        assert "Pâques" in easter_fr[0].fullname

        # English
        romcal_en = Romcal(
            calendar="general_roman",
            locale="en",
            calendar_definitions=bundled_definitions,
            resources=bundled_resources,
        )
        cal_en = romcal_en.liturgical_calendar(2026)
        easter_en = cal_en.get("2026-04-05")
        assert easter_en is not None
        assert "Easter" in easter_en[0].fullname

        # Latin
        romcal_la = Romcal(
            calendar="general_roman",
            locale="la",
            calendar_definitions=bundled_definitions,
            resources=bundled_resources,
        )
        cal_la = romcal_la.liturgical_calendar(2026)
        easter_la = cal_la.get("2026-04-05")
        assert easter_la is not None
        assert "Pascha" in easter_la[0].fullname or "Dominica" in easter_la[0].fullname


# =============================================================================
# Error Handling Tests
# =============================================================================


class TestBundledDataErrors:
    """Tests for error handling with bundled data."""

    def test_unknown_calendar_raises_error(
        self, bundled_definitions: list[CalendarDefinition], bundled_resources: list[Resources]
    ) -> None:
        """Unknown calendar should raise an explicit error."""
        with pytest.raises(RomcalError) as exc_info:
            Romcal(
                calendar="nonexistent_calendar",
                locale="en",
                calendar_definitions=bundled_definitions,
                resources=bundled_resources,
            )

        # Verify error message mentions the calendar
        assert "nonexistent_calendar" in str(exc_info.value)

    def test_unknown_locale_raises_error(
        self, bundled_definitions: list[CalendarDefinition], bundled_resources: list[Resources]
    ) -> None:
        """Unknown locale should raise an explicit error."""
        with pytest.raises(RomcalError) as exc_info:
            Romcal(
                calendar="general_roman",
                locale="nonexistent_locale",
                calendar_definitions=bundled_definitions,
                resources=bundled_resources,
            )

        # Verify error message mentions the locale
        assert "nonexistent_locale" in str(exc_info.value)

    def test_invalid_year_with_bundled_data(
        self, bundled_definitions: list[CalendarDefinition], bundled_resources: list[Resources]
    ) -> None:
        """Should raise error for invalid year."""
        romcal = Romcal(
            calendar="general_roman",
            locale="en",
            calendar_definitions=bundled_definitions,
            resources=bundled_resources,
        )

        with pytest.raises(RomcalError) as exc_info:
            romcal.liturgical_calendar(1500)

        assert "1500" in str(exc_info.value)
