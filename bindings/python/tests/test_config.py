"""Tests for Romcal configuration (equivalent to config.test.ts)."""

import pytest

from romcal import (
    CalendarContext,
    EasterCalculationType,
    Romcal,
    get_bundled_calendar_definitions,
    get_bundled_resources,
    has_bundled_data,
)


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


@pytest.mark.skipif(not has_bundled_data(), reason="Bundled data not available")
class TestDataParameterTypes:
    """Tests for calendar_definitions and resources parameter types."""

    def test_should_accept_raw_dicts_for_data_parameters(self) -> None:
        """Should accept raw dicts instead of Pydantic models."""
        # Get bundled data and convert to dicts
        definitions = get_bundled_calendar_definitions()
        resources = get_bundled_resources()

        definitions_as_dicts = [
            d.model_dump(mode="json", by_alias=True, exclude_none=True) for d in definitions
        ]
        resources_as_dicts = [
            r.model_dump(mode="json", by_alias=True, exclude_none=True) for r in resources
        ]

        romcal = Romcal(
            calendar="general_roman",
            locale="en",
            calendar_definitions=definitions_as_dicts,
            resources=resources_as_dicts,
        )

        calendar = romcal.liturgical_calendar(2026)
        assert len(calendar) > 300

    def test_should_accept_mixed_pydantic_models_and_dicts(self) -> None:
        """Should accept a mix of Pydantic models and dicts in the same list."""
        definitions = get_bundled_calendar_definitions()
        resources = get_bundled_resources()

        # Mix: first item as dict, rest as Pydantic models
        first_def_as_dict = definitions[0].model_dump(mode="json", by_alias=True, exclude_none=True)
        mixed_definitions = [first_def_as_dict, *definitions[1:]]

        first_res_as_dict = resources[0].model_dump(mode="json", by_alias=True, exclude_none=True)
        mixed_resources = [first_res_as_dict, *resources[1:]]

        romcal = Romcal(
            calendar="general_roman",
            locale="en",
            calendar_definitions=mixed_definitions,
            resources=mixed_resources,
        )

        calendar = romcal.liturgical_calendar(2026)
        assert len(calendar) > 300
