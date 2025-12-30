/// Tests for data loading (equivalent to test_data_loading.py)

import XCTest
@testable import Romcal

final class DataLoadingTests: XCTestCase {
    func testShouldLoadCalendarDefinitions() throws {
        let definitions = try loadAllCalendarDefinitions()

        XCTAssertGreaterThan(definitions.count, 0, "Should load at least one calendar definition")

        // Check that definitions have expected structure
        for definition in definitions {
            XCTAssertNotNil(definition["id"], "Each definition should have an id")
        }
    }

    func testShouldLoadResources() throws {
        let resources = try loadAllResources()

        XCTAssertGreaterThan(resources.count, 0, "Should load at least one resource")

        // Check that resources have expected structure
        for resource in resources {
            XCTAssertFalse(resource.locale.isEmpty, "Each resource should have a locale")
        }
    }

    func testShouldHaveLocalePropertyInResources() throws {
        let resources = try loadAllResources()

        for resource in resources {
            XCTAssertFalse(resource.locale.isEmpty, "Each resource should have a locale")
        }
    }

    func testShouldCreateRomcalWithLoadedData() throws {
        let calendarDefinitionsJson = try loadCalendarDefinitionsJson()
        let resourcesJson = try loadResourcesJson()

        let romcal = try RomcalCalendar(
            calendar: "france",
            locale: "fr",
            calendarDefinitionsJson: calendarDefinitionsJson,
            resourcesJson: resourcesJson
        )

        XCTAssertEqual(romcal.calendar, "france")
        XCTAssertEqual(romcal.locale, "fr")

        // Verify we can generate a calendar with the loaded data
        let calendar = try romcal.liturgicalCalendar(year: 2026)
        XCTAssertGreaterThan(calendar.count, 0, "Should generate calendar entries")
    }

    func testShouldGenerateCalendarWithFrenchTranslations() throws {
        let calendarDefinitionsJson = try loadCalendarDefinitionsJson()
        let resourcesJson = try loadResourcesJson()

        let romcal = try RomcalCalendar(
            calendar: "france",
            locale: "fr",
            calendarDefinitionsJson: calendarDefinitionsJson,
            resourcesJson: resourcesJson
        )

        let calendar = try romcal.liturgicalCalendar(year: 2026)

        // Check that Easter has French translation
        let easter = calendar["2026-04-05"]
        XCTAssertNotNil(easter)

        let fullname = easter![0]["fullname"] as? String
        XCTAssertNotNil(fullname)
        XCTAssertTrue(fullname!.contains("Pâques"), "Easter should have French name containing 'Pâques'")
    }

    func testShouldIncludeFrenchSaints() throws {
        let calendarDefinitionsJson = try loadCalendarDefinitionsJson()
        let resourcesJson = try loadResourcesJson()

        let romcal = try RomcalCalendar(
            calendar: "france",
            locale: "fr",
            calendarDefinitionsJson: calendarDefinitionsJson,
            resourcesJson: resourcesJson
        )

        let calendar = try romcal.liturgicalCalendar(year: 2026)

        // Saint Jean-Marie Vianney - August 4
        let vianney = calendar["2026-08-04"]
        XCTAssertNotNil(vianney)

        let saintDay = vianney!.first { day in
            if let id = day["id"] as? String {
                return id.contains("vianney")
            }
            return false
        }
        XCTAssertNotNil(saintDay, "Should include Saint Jean-Marie Vianney")
        XCTAssertNotNil(saintDay!["fullname"])
    }

    func testShouldGenerateMassCalendarWithFrenchLocale() throws {
        let calendarDefinitionsJson = try loadCalendarDefinitionsJson()
        let resourcesJson = try loadResourcesJson()

        let romcal = try RomcalCalendar(
            calendar: "france",
            locale: "fr",
            calendarDefinitionsJson: calendarDefinitionsJson,
            resourcesJson: resourcesJson
        )

        let massCalendar = try romcal.massCalendar(year: 2026)

        // Default context is GREGORIAN, so Christmas 2026 is in the calendar
        let christmas = massCalendar["2026-12-25"]
        XCTAssertNotNil(christmas)
        XCTAssertGreaterThan(christmas!.count, 0)

        for mass in christmas! {
            XCTAssertNotNil(mass["mass_time_name"])
            XCTAssertNotNil(mass["fullname"])
        }
    }

    func testShouldGenerateLiturgicalCalendarWithEnglishLocale() throws {
        let calendarDefinitionsJson = try loadCalendarDefinitionsJson()
        let resourcesJson = try loadResourcesJson()

        let romcal = try RomcalCalendar(
            calendar: "general_roman",
            locale: "en",
            calendarDefinitionsJson: calendarDefinitionsJson,
            resourcesJson: resourcesJson
        )

        let calendar = try romcal.liturgicalCalendar(year: 2026)

        let easter = calendar["2026-04-05"]
        XCTAssertNotNil(easter)
        XCTAssertNotNil(easter![0]["fullname"])
        XCTAssertNotNil(easter![0]["rank_name"])
        XCTAssertNotNil(easter![0]["season_name"])
    }
}
