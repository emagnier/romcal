"""Tests for liturgical calendar generation (equivalent to calendar.test.ts)."""

import pytest

from romcal import CalendarContext, CalendarDefinition, Resources, Romcal, RomcalError
from romcal.types import MassTime, Precedence, Rank, Season


class TestGregorianYearCalendar:
    """Tests for Gregorian year calendar (default context)."""

    @pytest.fixture(autouse=True)
    def setup(
        self, calendar_definitions: list[CalendarDefinition], resources: list[Resources]
    ) -> None:
        """Set up test fixtures."""
        self.romcal = Romcal(
            calendar="general_roman",
            locale="en",
            calendar_definitions=calendar_definitions,
            resources=resources,
        )
        self.calendar = self.romcal.liturgical_calendar(2026)

    def test_should_generate_full_gregorian_year(self) -> None:
        """Should generate a full Gregorian year."""
        dates = list(self.calendar.keys())
        assert len(dates) >= 365

    def test_should_start_jan_1_end_dec_31(self) -> None:
        """Should start on January 1 and end on December 31."""
        dates = sorted(self.calendar.keys())
        assert dates[0] == "2026-01-01"
        assert dates[-1] == "2026-12-31"

    def test_should_include_easter_2026_on_april_5(self) -> None:
        """Should include Easter 2026 on April 5."""
        easter = self.calendar.get("2026-04-05")

        assert easter is not None
        assert len(easter) > 0
        # Easter has the highest precedence (TRIDUUM_1)
        assert easter[0].precedence == Precedence.triduum_1
        assert easter[0].is_holy_day_of_obligation is True

    def test_should_have_correct_easter_season(self) -> None:
        """Should have correct Easter season."""
        easter = self.calendar.get("2026-04-05")

        assert easter is not None
        assert easter[0].fullname == "Easter Sunday of the Resurrection of the Lord"
        assert easter[0].season == Season.easter_time

    def test_should_include_christmas_2026_on_december_25(self) -> None:
        """Should include Christmas 2026 on December 25."""
        christmas = self.calendar.get("2026-12-25")

        assert christmas is not None
        assert christmas[0].rank == Rank.solemnity

    def test_should_have_no_masses_for_holy_saturday(self) -> None:
        """Should have no masses defined for Holy Saturday (April 4)."""
        holy_saturday = self.calendar.get("2026-04-04")

        assert holy_saturday is not None
        assert holy_saturday[0].fullname == "Holy Saturday"
        # Holy Saturday has no masses during the day (only Easter Vigil in the evening)
        assert holy_saturday[0].masses == []


class TestLiturgicalYearCalendar:
    """Tests for liturgical year calendar (LITURGICAL context)."""

    @pytest.fixture(autouse=True)
    def setup(
        self, calendar_definitions: list[CalendarDefinition], resources: list[Resources]
    ) -> None:
        """Set up test fixtures."""
        self.romcal = Romcal(
            calendar="general_roman",
            locale="en",
            context=CalendarContext.liturgical,
            calendar_definitions=calendar_definitions,
            resources=resources,
        )
        self.calendar = self.romcal.liturgical_calendar(2026)

    def test_should_generate_full_liturgical_year(self) -> None:
        """Should generate a full liturgical year."""
        dates = list(self.calendar.keys())
        assert len(dates) > 350

    def test_should_start_in_late_november_2025_advent(self) -> None:
        """Should start in late November 2025 (Advent)."""
        dates = sorted(self.calendar.keys())
        # Liturgical year 2026 starts on first Sunday of Advent 2025
        assert dates[0] == "2025-11-30"

    def test_should_end_in_late_november_2026(self) -> None:
        """Should end in late November 2026 (Saturday after Christ the King)."""
        dates = sorted(self.calendar.keys())
        assert dates[-1] == "2026-11-28"

    def test_should_include_christmas_2025(self) -> None:
        """Should include Christmas 2025."""
        christmas = self.calendar.get("2025-12-25")

        assert christmas is not None
        assert christmas[0].rank == Rank.solemnity


class TestMassCalendarGregorianYear:
    """Tests for mass calendar (Gregorian year context)."""

    @pytest.fixture(autouse=True)
    def setup(
        self, calendar_definitions: list[CalendarDefinition], resources: list[Resources]
    ) -> None:
        """Set up test fixtures."""
        self.romcal = Romcal(
            calendar="general_roman",
            locale="en",
            calendar_definitions=calendar_definitions,
            resources=resources,
        )
        self.mass_calendar = self.romcal.mass_calendar(2026)

    def test_should_generate_mass_calendar(self) -> None:
        """Should generate mass calendar."""
        dates = list(self.mass_calendar.keys())
        assert len(dates) >= 365

    def test_should_include_december_24_masses(self) -> None:
        """Should include December 24 masses (morning + Christmas vigil)."""
        dec24 = self.mass_calendar.get("2026-12-24")
        assert dec24 is not None

        mass_times = [m.mass_time for m in dec24]
        assert len(dec24) == 2
        # Morning mass (Advent weekday) + Previous evening mass (Christmas vigil)
        assert mass_times == [MassTime.morning_mass, MassTime.previous_evening_mass]

    def test_should_include_multiple_christmas_2026_masses(self) -> None:
        """Should include multiple Christmas 2026 masses."""
        christmas = self.mass_calendar.get("2026-12-25")
        assert christmas is not None

        mass_times = [m.mass_time for m in christmas]
        assert len(christmas) == 3
        assert mass_times == [MassTime.night_mass, MassTime.mass_at_dawn, MassTime.day_mass]

    def test_should_have_correct_mass_time_names(self) -> None:
        """Should have correct mass time names."""
        christmas = self.mass_calendar.get("2026-12-25")
        assert christmas is not None

        for mass in christmas:
            assert mass.mass_time is not None
            assert "The Nativity of the Lord" in mass.fullname

    def test_should_place_easter_vigil_on_saturday_evening(self) -> None:
        """Should place Easter Vigil on Saturday evening (April 4)."""
        easter_vigil_day = self.mass_calendar.get("2026-04-04")
        assert easter_vigil_day is not None

        vigil = next((m for m in easter_vigil_day if m.mass_time == MassTime.easter_vigil), None)
        assert vigil is not None
        assert vigil.liturgical_date == "2026-04-05"

    def test_should_have_mass_entry_for_holy_saturday(self) -> None:
        """Should have a mass entry for Holy Saturday in mass-centric view."""
        # Holy Saturday has no masses in the liturgical day definition,
        # but the mass-centric calendar includes the Easter Vigil on this civil date
        holy_saturday = self.mass_calendar.get("2026-04-04")

        assert holy_saturday is not None
        assert len(holy_saturday) == 1
        assert holy_saturday[0].mass_time == MassTime.easter_vigil


class TestMassCalendarLiturgicalYear:
    """Tests for mass calendar (Liturgical year context)."""

    @pytest.fixture(autouse=True)
    def setup(
        self, calendar_definitions: list[CalendarDefinition], resources: list[Resources]
    ) -> None:
        """Set up test fixtures."""
        self.romcal = Romcal(
            calendar="general_roman",
            locale="en",
            context=CalendarContext.liturgical,
            calendar_definitions=calendar_definitions,
            resources=resources,
        )
        self.mass_calendar = self.romcal.mass_calendar(2026)

    def test_should_include_christmas_2025_masses(self) -> None:
        """Should include Christmas 2025 masses."""
        christmas = self.mass_calendar.get("2025-12-25")

        assert christmas is not None
        assert len(christmas) > 1

        for mass in christmas:
            assert mass.mass_time is not None
            assert "The Nativity of the Lord" in mass.fullname


class TestFrenchCalendar:
    """Tests for French calendar."""

    def test_should_generate_french_locale_calendar(
        self, calendar_definitions: list[CalendarDefinition], resources: list[Resources]
    ) -> None:
        """Should generate French locale calendar."""
        romcal = Romcal(
            calendar="france",
            locale="fr",
            calendar_definitions=calendar_definitions,
            resources=resources,
        )
        calendar = romcal.liturgical_calendar(2026)

        easter = calendar.get("2026-04-05")
        assert easter is not None
        assert easter[0].fullname == "Dimanche de Pâques - La résurrection du Seigneur"


class TestErrorHandling:
    """Tests for error handling."""

    def test_should_raise_error_for_invalid_year(self) -> None:
        """Should raise RomcalError for invalid year."""
        romcal = Romcal()

        with pytest.raises(RomcalError):
            romcal.liturgical_calendar(1500)

    def test_should_include_error_message_for_invalid_year(self) -> None:
        """Should include error message for invalid year."""
        romcal = Romcal()

        with pytest.raises(RomcalError) as exc_info:
            romcal.liturgical_calendar(1500)

        assert "1500" in str(exc_info.value)

    def test_should_accept_year_1583(self) -> None:
        """Should accept year 1583 (first valid Gregorian year)."""
        romcal = Romcal()
        calendar = romcal.liturgical_calendar(1583)
        assert len(calendar) > 0

    def test_should_reject_year_1582(self) -> None:
        """Should reject year 1582 (before Gregorian calendar)."""
        romcal = Romcal()

        with pytest.raises(RomcalError):
            romcal.liturgical_calendar(1582)

    def test_should_preserve_error_cause_chain(self) -> None:
        """Should preserve error cause chain."""
        from romcal._uniffi import romcal_uniffi as core

        romcal = Romcal()

        with pytest.raises(RomcalError) as exc_info:
            romcal.liturgical_calendar(1500)

        # The cause should be the original FFI error
        assert exc_info.value.__cause__ is not None
        assert isinstance(exc_info.value.__cause__, core.RomcalError)
