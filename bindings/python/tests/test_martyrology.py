"""Tests for martyrology search methods."""

import pytest

from romcal import (
    MartyrologyEntry,
    MartyrologyQuery,
    MartyrologySearchResult,
    MatchType,
    Romcal,
    get_bundled_calendar_definitions,
    get_bundled_resources,
)
from romcal.types import CanonizationLevel, Title


@pytest.fixture(scope="module")
def romcal() -> Romcal:
    """Create a Romcal instance with bundled data."""
    return Romcal(
        calendar="general_roman",
        locale="en",
        calendar_definitions=get_bundled_calendar_definitions(),
        resources=get_bundled_resources(),
    )


class TestGetMartyrologyEntry:
    """Tests for the get_martyrology_entry method."""

    def test_should_return_entry_by_exact_id(self, romcal: Romcal) -> None:
        entry = romcal.get_martyrology_entry("agnes_of_rome_virgin")

        assert entry is not None
        assert entry.id == "agnes_of_rome_virgin"

    def test_should_return_none_for_nonexistent_entry(self, romcal: Romcal) -> None:
        entry = romcal.get_martyrology_entry("non_existent_entry_id")

        assert entry is None

    def test_should_return_entry_with_expected_properties(self, romcal: Romcal) -> None:
        entry = romcal.get_martyrology_entry("francis_of_assisi")

        assert entry is not None
        assert entry.id == "francis_of_assisi"
        assert entry.name == "Francis of Assisi"
        assert entry.canonization_level == CanonizationLevel.saint


class TestSearchMartyrologyEntries:
    """Tests for the search_martyrology method."""

    def test_should_search_entries_by_text(self, romcal: Romcal) -> None:
        query = MartyrologyQuery(text="francis")
        results = romcal.search_martyrology(query)

        assert len(results) > 0
        # Results should be sorted by score (highest first)
        for i in range(1, len(results)):
            assert results[i - 1].score >= results[i].score

    def test_should_return_search_result_with_correct_structure(self, romcal: Romcal) -> None:
        query = MartyrologyQuery(text="agnes", limit=1)
        results = romcal.search_martyrology(query)

        assert len(results) == 1
        result = results[0]

        # Check MartyrologySearchResult structure
        assert isinstance(result, MartyrologySearchResult)
        assert isinstance(result.entry, MartyrologyEntry)
        assert isinstance(result.score, float)
        assert isinstance(result.match_type, MatchType)
        assert isinstance(result.matched_fields, list)

        # Verify score bounds
        assert result.score > 0
        assert result.score <= 1

    def test_should_filter_by_canonization_level(self, romcal: Romcal) -> None:
        query = MartyrologyQuery(canonization_level=CanonizationLevel.blessed, limit=10)
        results = romcal.search_martyrology(query)

        assert len(results) > 0
        for result in results:
            assert result.entry.canonization_level == CanonizationLevel.blessed

    def test_should_respect_limit_parameter(self, romcal: Romcal) -> None:
        query = MartyrologyQuery(text="saint", limit=5)
        results = romcal.search_martyrology(query)

        assert len(results) <= 5

    def test_should_return_fewer_results_with_high_min_score(self, romcal: Romcal) -> None:
        query_low = MartyrologyQuery(text="john", min_score=0.3, limit=50)
        query_high = MartyrologyQuery(text="john", min_score=0.9, limit=50)

        results_low = romcal.search_martyrology(query_low)
        results_high = romcal.search_martyrology(query_high)

        # Higher min_score should return fewer or equal results
        assert len(results_high) <= len(results_low)
        # All results should be above the min_score threshold
        for result in results_high:
            assert result.score >= 0.9

    def test_should_return_exact_id_match_with_score_1(self, romcal: Romcal) -> None:
        query = MartyrologyQuery(text="agnes_of_rome_virgin")
        results = romcal.search_martyrology(query)

        assert len(results) > 0
        exact_match = next((r for r in results if r.entry.id == "agnes_of_rome_virgin"), None)
        assert exact_match is not None
        assert exact_match.score == 1.0
        assert exact_match.match_type == MatchType.exact_id

    def test_should_filter_by_titles(self, romcal: Romcal) -> None:
        query = MartyrologyQuery(titles=[Title.martyr], limit=10)
        results = romcal.search_martyrology(query)

        assert len(results) > 0
        for result in results:
            assert result.entry.titles is not None
            assert Title.martyr in result.entry.titles

    def test_should_combine_text_search_with_canonization_filter(self, romcal: Romcal) -> None:
        query = MartyrologyQuery(text="john", canonization_level=CanonizationLevel.saint, limit=10)
        results = romcal.search_martyrology(query)

        assert len(results) > 0
        for result in results:
            # Verify filter is applied
            assert result.entry.canonization_level == CanonizationLevel.saint
            # Verify text search worked (fuzzy match on 'john')
            has_john_in_fields = (
                "john" in result.entry.id.lower()
                or (result.entry.name and "john" in result.entry.name.lower())
                or (result.entry.fullname and "john" in result.entry.fullname.lower())
            )
            assert has_john_in_fields


class TestMatchType:
    """Tests for the MatchType enum."""

    def test_should_have_expected_values(self) -> None:
        assert MatchType.exact_id.value == "exact_id"
        assert MatchType.fuzzy.value == "fuzzy"
        assert MatchType.filter_only.value == "filter_only"

    def test_should_be_comparable(self) -> None:
        assert MatchType.exact_id == MatchType.exact_id
        assert MatchType.exact_id != MatchType.fuzzy
