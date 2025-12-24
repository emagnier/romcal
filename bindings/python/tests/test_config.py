"""Tests for Romcal configuration (equivalent to config.test.ts)."""

from romcal import Romcal


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
        assert romcal.easter_calculation_type == "GREGORIAN"
        assert romcal.context == "GREGORIAN"

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
