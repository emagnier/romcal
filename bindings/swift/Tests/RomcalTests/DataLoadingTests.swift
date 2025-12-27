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
            XCTAssertNotNil(resource["locale"], "Each resource should have a locale")
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
}
