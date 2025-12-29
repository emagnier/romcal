/// Tests for liturgical calendar generation (equivalent to test_calendar.py)

import XCTest
@testable import Romcal

final class GregorianYearCalendarTests: XCTestCase {
    var romcal: RomcalCalendar!
    var calendar: [String: [[String: Any]]]!

    override func setUpWithError() throws {
        let calendarDefinitionsJson = try loadCalendarDefinitionsJson()
        let resourcesJson = try loadResourcesJson()

        romcal = try RomcalCalendar(
            calendar: "general_roman",
            locale: "en",
            calendarDefinitionsJson: calendarDefinitionsJson,
            resourcesJson: resourcesJson
        )
        calendar = try romcal.liturgicalCalendar(year: 2026)
    }

    func testShouldGenerateFullGregorianYear() throws {
        let dates = Array(calendar.keys)
        XCTAssertGreaterThanOrEqual(dates.count, 365)
    }

    func testShouldStartJan1EndDec31() throws {
        let dates = calendar.keys.sorted()
        XCTAssertEqual(dates.first, "2026-01-01")
        XCTAssertEqual(dates.last, "2026-12-31")
    }

    func testShouldIncludeEaster2026OnApril5() throws {
        let easter = calendar["2026-04-05"]

        XCTAssertNotNil(easter)
        XCTAssertGreaterThan(easter!.count, 0)
        // Easter has the highest precedence (TRIDUUM_1)
        XCTAssertEqual(easter![0]["precedence"] as? String, "TRIDUUM_1")
        XCTAssertEqual(easter![0]["is_holy_day_of_obligation"] as? Bool, true)
    }

    func testShouldHaveCorrectEasterSeason() throws {
        let easter = calendar["2026-04-05"]

        XCTAssertNotNil(easter)
        XCTAssertEqual(easter![0]["fullname"] as? String, "Easter Sunday of the Resurrection of the Lord")
        XCTAssertEqual(easter![0]["season"] as? String, "EASTER_TIME")
    }

    func testShouldIncludeChristmas2026OnDecember25() throws {
        let christmas = calendar["2026-12-25"]

        XCTAssertNotNil(christmas)
        XCTAssertEqual(christmas![0]["rank"] as? String, "SOLEMNITY")
    }

    func testShouldHaveNoMassesForHolySaturday() throws {
        let holySaturday = calendar["2026-04-04"]

        XCTAssertNotNil(holySaturday)
        XCTAssertEqual(holySaturday![0]["fullname"] as? String, "Holy Saturday")
        // Holy Saturday has no masses during the day (only Easter Vigil in the evening)
        let masses = holySaturday![0]["masses"] as? [[String: Any]]
        XCTAssertEqual(masses?.count ?? 0, 0)
    }
}

final class LiturgicalYearCalendarTests: XCTestCase {
    var romcal: RomcalCalendar!
    var calendar: [String: [[String: Any]]]!

    override func setUpWithError() throws {
        let calendarDefinitionsJson = try loadCalendarDefinitionsJson()
        let resourcesJson = try loadResourcesJson()

        romcal = try RomcalCalendar(
            calendar: "general_roman",
            locale: "en",
            context: .liturgical,
            calendarDefinitionsJson: calendarDefinitionsJson,
            resourcesJson: resourcesJson
        )
        calendar = try romcal.liturgicalCalendar(year: 2026)
    }

    func testShouldGenerateFullLiturgicalYear() throws {
        let dates = Array(calendar.keys)
        XCTAssertGreaterThan(dates.count, 350)
    }

    func testShouldStartInLateNovember2025Advent() throws {
        let dates = calendar.keys.sorted()
        // Liturgical year 2026 starts on first Sunday of Advent 2025
        XCTAssertEqual(dates.first, "2025-11-30")
    }

    func testShouldEndInLateNovember2026() throws {
        let dates = calendar.keys.sorted()
        XCTAssertEqual(dates.last, "2026-11-28")
    }

    func testShouldIncludeChristmas2025() throws {
        let christmas = calendar["2025-12-25"]

        XCTAssertNotNil(christmas)
        XCTAssertEqual(christmas![0]["rank"] as? String, "SOLEMNITY")
    }
}

final class MassCalendarGregorianYearTests: XCTestCase {
    var romcal: RomcalCalendar!
    var massCalendar: [String: [[String: Any]]]!

    override func setUpWithError() throws {
        let calendarDefinitionsJson = try loadCalendarDefinitionsJson()
        let resourcesJson = try loadResourcesJson()

        romcal = try RomcalCalendar(
            calendar: "general_roman",
            locale: "en",
            calendarDefinitionsJson: calendarDefinitionsJson,
            resourcesJson: resourcesJson
        )
        massCalendar = try romcal.massCalendar(year: 2026)
    }

    func testShouldGenerateMassCalendar() throws {
        let dates = Array(massCalendar.keys)
        XCTAssertGreaterThanOrEqual(dates.count, 365)
    }

    func testShouldIncludeDecember24Masses() throws {
        let dec24 = massCalendar["2026-12-24"]
        XCTAssertNotNil(dec24)

        let massTimes = dec24!.compactMap { $0["mass_time"] as? String }
        XCTAssertEqual(dec24!.count, 2)
        // Morning mass (Advent weekday) + Previous evening mass (Christmas vigil)
        XCTAssertEqual(massTimes, ["MORNING_MASS", "PREVIOUS_EVENING_MASS"])
    }

    func testShouldIncludeMultipleChristmas2026Masses() throws {
        let christmas = massCalendar["2026-12-25"]
        XCTAssertNotNil(christmas)

        let massTimes = christmas!.compactMap { $0["mass_time"] as? String }
        XCTAssertEqual(christmas!.count, 3)
        XCTAssertEqual(massTimes, ["NIGHT_MASS", "MASS_AT_DAWN", "DAY_MASS"])
    }

    func testShouldHaveCorrectMassTimeNames() throws {
        let christmas = massCalendar["2026-12-25"]
        XCTAssertNotNil(christmas)

        for mass in christmas! {
            XCTAssertNotNil(mass["mass_time"])
            let fullname = mass["fullname"] as? String ?? ""
            XCTAssertTrue(fullname.contains("The Nativity of the Lord"))
        }
    }

    func testShouldPlaceEasterVigilOnSaturdayEvening() throws {
        let easterVigilDay = massCalendar["2026-04-04"]
        XCTAssertNotNil(easterVigilDay)

        let vigil = easterVigilDay!.first { ($0["mass_time"] as? String) == "EASTER_VIGIL" }
        XCTAssertNotNil(vigil)
        XCTAssertEqual(vigil!["liturgical_date"] as? String, "2026-04-05")
    }

    func testShouldHaveMassEntryForHolySaturday() throws {
        // Holy Saturday has no masses in the liturgical day definition,
        // but the mass-centric calendar includes the Easter Vigil on this civil date
        let holySaturday = massCalendar["2026-04-04"]

        XCTAssertNotNil(holySaturday)
        XCTAssertEqual(holySaturday!.count, 1)
        XCTAssertEqual(holySaturday![0]["mass_time"] as? String, "EASTER_VIGIL")
    }
}

final class MassCalendarLiturgicalYearTests: XCTestCase {
    var romcal: RomcalCalendar!
    var massCalendar: [String: [[String: Any]]]!

    override func setUpWithError() throws {
        let calendarDefinitionsJson = try loadCalendarDefinitionsJson()
        let resourcesJson = try loadResourcesJson()

        romcal = try RomcalCalendar(
            calendar: "general_roman",
            locale: "en",
            context: .liturgical,
            calendarDefinitionsJson: calendarDefinitionsJson,
            resourcesJson: resourcesJson
        )
        massCalendar = try romcal.massCalendar(year: 2026)
    }

    func testShouldIncludeChristmas2025Masses() throws {
        let christmas = massCalendar["2025-12-25"]

        XCTAssertNotNil(christmas)
        XCTAssertGreaterThan(christmas!.count, 1)

        for mass in christmas! {
            XCTAssertNotNil(mass["mass_time"])
            let fullname = mass["fullname"] as? String ?? ""
            XCTAssertTrue(fullname.contains("The Nativity of the Lord"))
        }
    }
}

final class FrenchCalendarTests: XCTestCase {
    func testShouldGenerateFrenchLocaleCalendar() throws {
        let calendarDefinitionsJson = try loadCalendarDefinitionsJson()
        let resourcesJson = try loadResourcesJson()

        let romcal = try RomcalCalendar(
            calendar: "france",
            locale: "fr",
            calendarDefinitionsJson: calendarDefinitionsJson,
            resourcesJson: resourcesJson
        )
        let calendar = try romcal.liturgicalCalendar(year: 2026)

        let easter = calendar["2026-04-05"]
        XCTAssertNotNil(easter)
        XCTAssertEqual(easter![0]["fullname"] as? String, "Dimanche de Pâques - La résurrection du Seigneur")
    }
}

final class ErrorHandlingTests: XCTestCase {
    func testShouldThrowErrorForInvalidYear() throws {
        let romcal = try RomcalCalendar()

        XCTAssertThrowsError(try romcal.liturgicalCalendar(year: 1500)) { error in
            XCTAssertTrue(error is RomcalError)
        }
    }

    func testShouldIncludeErrorMessageForInvalidYear() throws {
        let romcal = try RomcalCalendar()

        XCTAssertThrowsError(try romcal.liturgicalCalendar(year: 1500)) { error in
            let description = (error as? RomcalError)?.errorDescription ?? ""
            XCTAssertTrue(description.contains("1500"))
        }
    }

    func testShouldAcceptYear1583() throws {
        let romcal = try RomcalCalendar()
        let calendar = try romcal.liturgicalCalendar(year: 1583)
        XCTAssertGreaterThan(calendar.count, 0)
    }

    func testShouldRejectYear1582() throws {
        let romcal = try RomcalCalendar()

        XCTAssertThrowsError(try romcal.liturgicalCalendar(year: 1582)) { error in
            XCTAssertTrue(error is RomcalError)
        }
    }
}
