"""Tests for data loading from /data folder (equivalent to data-loading.test.ts)."""

from typing import Any

import pytest

from romcal import Romcal


class TestDataLoadingFromDataFolder:
    """Tests for data loading from /data folder."""

    def test_should_load_calendar_definitions(
        self, calendar_definitions: list[dict[str, Any]]
    ) -> None:
        """Should load calendar definitions."""
        assert len(calendar_definitions) > 0

    def test_should_load_resource_locales(self, resources: list[dict[str, Any]]) -> None:
        """Should load resource locales."""
        assert len(resources) > 0

    def test_should_have_locale_property_in_resources(
        self, resources: list[dict[str, Any]]
    ) -> None:
        """Should have locale property in resources."""
        for resource in resources:
            assert "locale" in resource
            assert isinstance(resource["locale"], str)


class TestFrenchCalendarWithLoadedData:
    """Tests for French calendar with loaded data."""

    @pytest.fixture(autouse=True)
    def setup(self, calendar_definitions_json: str, resources_json: str) -> None:
        """Set up test fixtures."""
        self.romcal = Romcal(
            calendar="france",
            locale="fr",
            calendar_definitions_json=calendar_definitions_json,
            resources_json=resources_json,
        )

    def test_should_generate_liturgical_calendar_with_french_locale(self) -> None:
        """Should generate liturgical calendar with French locale."""
        calendar = self.romcal.liturgical_calendar(2026)

        easter = calendar.get("2026-04-05")
        assert easter is not None
        assert easter[0]["fullname"] is not None
        assert easter[0]["rank_name"] is not None
        assert easter[0]["season_name"] is not None

    def test_should_include_french_saints(self) -> None:
        """Should include French saints."""
        calendar = self.romcal.liturgical_calendar(2026)

        # Saint Jean-Marie Vianney - August 4
        vianney = calendar.get("2026-08-04")
        assert vianney is not None

        saint_day = next((d for d in vianney if "vianney" in d.get("id", "")), None)
        assert saint_day is not None
        assert saint_day["fullname"] is not None

    def test_should_generate_mass_calendar_with_french_locale(self) -> None:
        """Should generate mass calendar with French locale."""
        mass_calendar = self.romcal.mass_calendar(2026)

        # Default context is GREGORIAN, so Christmas 2026 is in the calendar
        christmas = mass_calendar.get("2026-12-25")
        assert christmas is not None
        assert len(christmas) > 0

        for mass in christmas:
            assert mass["mass_time_name"] is not None
            assert mass["fullname"] is not None


class TestEnglishCalendarWithLoadedData:
    """Tests for English calendar with loaded data."""

    @pytest.fixture(autouse=True)
    def setup(self, calendar_definitions_json: str, resources_json: str) -> None:
        """Set up test fixtures."""
        self.romcal = Romcal(
            calendar="general_roman",
            locale="en",
            calendar_definitions_json=calendar_definitions_json,
            resources_json=resources_json,
        )

    def test_should_generate_liturgical_calendar_with_english_locale(self) -> None:
        """Should generate liturgical calendar with English locale."""
        calendar = self.romcal.liturgical_calendar(2026)

        easter = calendar.get("2026-04-05")
        assert easter is not None
        assert easter[0]["fullname"] is not None
        assert easter[0]["rank_name"] is not None
        assert easter[0]["season_name"] is not None
