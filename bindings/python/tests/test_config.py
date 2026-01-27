"""Tests for Romcal configuration (equivalent to config.test.ts)."""

import pytest

from romcal import CalendarContext, EasterCalculationType, Romcal


class TestRomcalConfiguration:
    """Tests for Romcal configuration."""

    def test_should_use_default_configuration(self) -> None:
        """Should use default configuration."""
        romcal = Romcal()

        assert romcal.calendar == "general_roman"
        assert romcal.locale == "en"
        assert romcal.epiphany_on_sunday is False
        assert romcal.corpus_christi_on_sunday is True
        assert romcal.ascension_on_sunday is False
        assert romcal.easter_calculation_type == EasterCalculationType.gregorian
        assert romcal.context == CalendarContext.gregorian

    def test_should_accept_calendar_and_locale_as_arguments(self) -> None:
        """Should accept calendar and locale as arguments."""
        romcal = Romcal(calendar="france", locale="fr")

        assert romcal.calendar == "france"
        assert romcal.locale == "fr"

    def test_should_accept_partial_configuration_object(self) -> None:
        """Should accept partial configuration object."""
        romcal = Romcal(
            calendar="united_states",
            locale="en",
            epiphany_on_sunday=True,
            ascension_on_sunday=True,
        )

        assert romcal.calendar == "united_states"
        assert romcal.locale == "en"
        assert romcal.epiphany_on_sunday is True
        assert romcal.ascension_on_sunday is True
        # Default values preserved
        assert romcal.corpus_christi_on_sunday is True

    def test_should_raise_error_for_invalid_easter_calculation_type(self) -> None:
        """Should raise RomcalError for invalid easter calculation type (FFI level)."""
        # Test at FFI level since Python wrapper uses typed enums
        from romcal._uniffi import romcal_uniffi as core

        config = core.RomcalConfig(
            calendar="general_roman",
            locale="en",
            epiphany_on_sunday=False,
            ascension_on_sunday=False,
            corpus_christi_on_sunday=True,
            easter_calculation_type="INVALID",
            context="gregorian",
            calendar_definitions_json=None,
            resources_json=None,
        )
        with pytest.raises(core.RomcalError, match="Invalid easter_calculation_type"):
            core.Romcal(config)

    def test_should_raise_error_for_invalid_context(self) -> None:
        """Should raise RomcalError for invalid context (FFI level)."""
        # Test at FFI level since Python wrapper uses typed enums
        from romcal._uniffi import romcal_uniffi as core

        config = core.RomcalConfig(
            calendar="general_roman",
            locale="en",
            epiphany_on_sunday=False,
            ascension_on_sunday=False,
            corpus_christi_on_sunday=True,
            easter_calculation_type="gregorian",
            context="INVALID",
            calendar_definitions_json=None,
            resources_json=None,
        )
        with pytest.raises(core.RomcalError, match="Invalid context"):
            core.Romcal(config)
