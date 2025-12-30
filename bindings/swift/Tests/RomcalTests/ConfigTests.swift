/// Tests for Romcal configuration (equivalent to test_config.py)

import XCTest
@testable import Romcal

final class RomcalConfigurationTests: XCTestCase {
  func testShouldUseDefaultConfiguration() throws {
    let romcal = try RomcalCalendar()

    XCTAssertEqual(romcal.calendar, "general_roman")
    XCTAssertEqual(romcal.locale, "en")
    XCTAssertEqual(romcal.epiphanyOnSunday, false)
    XCTAssertEqual(romcal.corpusChristiOnSunday, true)
    XCTAssertEqual(romcal.ascensionOnSunday, false)
    XCTAssertEqual(romcal.easterCalculationType, .gregorian)
    XCTAssertEqual(romcal.context, .gregorian)
  }

  func testShouldAcceptCalendarAndLocaleAsArguments() throws {
    let romcal = try RomcalCalendar(calendar: "france", locale: "fr")

    XCTAssertEqual(romcal.calendar, "france")
    XCTAssertEqual(romcal.locale, "fr")
  }

  func testShouldAcceptPartialConfigurationObject() throws {
    let romcal = try RomcalCalendar(
      calendar: "united_states",
      locale: "en",
      epiphanyOnSunday: true,
      ascensionOnSunday: true
    )

    XCTAssertEqual(romcal.calendar, "united_states")
    XCTAssertEqual(romcal.locale, "en")
    XCTAssertEqual(romcal.epiphanyOnSunday, true)
    XCTAssertEqual(romcal.ascensionOnSunday, true)
    // Default values preserved
    XCTAssertEqual(romcal.corpusChristiOnSunday, true)
  }
}
