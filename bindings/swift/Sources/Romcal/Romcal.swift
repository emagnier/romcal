/// Romcal - Liturgical calendar for the Roman Rite of the Catholic Church.
///
/// Romcal is a liturgical calendar library for the Roman Rite of the Catholic Church.
/// It computes liturgical days, seasons, and Mass contexts for any given year.
///
/// Example usage:
///
/// ```swift
/// import Romcal
///
/// // Create a Romcal instance with French calendar and locale
/// let romcal = try RomcalCalendar(calendar: "france", locale: "fr")
///
/// // Generate the liturgical calendar for 2025
/// let calendar = try romcal.liturgicalCalendar(year: 2025)
///
/// // Access liturgical days
/// for (date, days) in calendar {
///     for day in days {
///         print("\(date): \(day.id) (\(day.rank))")
///     }
/// }
///
/// // Get a specific celebration date
/// let christmas = try romcal.getDate(celebrationId: "christmas", year: 2025)
/// print("Christmas 2025: \(christmas)")
/// ```

import Foundation
@preconcurrency import RomcalFFI

/// Error type for Romcal operations
public enum RomcalError: Error, LocalizedError {
    case invalidYear(String)
    case invalidConfig(String)
    case notFound(String)
    case parseError(String)
    case calculationError(String)

    public var errorDescription: String? {
        switch self {
        case .invalidYear(let message): return "Invalid year: \(message)"
        case .invalidConfig(let message): return "Invalid configuration: \(message)"
        case .notFound(let message): return "Not found: \(message)"
        case .parseError(let message): return "Parse error: \(message)"
        case .calculationError(let message): return "Calculation error: \(message)"
        }
    }

    init(from error: RomcalFFI.RomcalError) {
        switch error {
        case .InvalidYear(let msg): self = .invalidYear(msg)
        case .InvalidConfig(let msg): self = .invalidConfig(msg)
        case .NotFound(let msg): self = .notFound(msg)
        case .ParseError(let msg): self = .parseError(msg)
        case .CalculationError(let msg): self = .calculationError(msg)
        }
    }
}

/// Easter calculation type
public enum RomcalEasterCalculationType: String, Sendable {
    case gregorian = "GREGORIAN"
    case julian = "JULIAN"
}

/// Calendar context type
public enum RomcalCalendarContext: String, Sendable {
    case gregorian = "GREGORIAN"
    case liturgical = "LITURGICAL"
}

/// Liturgical calendar for the Roman Rite of the Catholic Church.
///
/// Computes liturgical days, seasons, and Mass contexts for any given year.
/// Supports various regional calendars and locales.
public final class RomcalCalendar {
    private let inner: Romcal

    /// The version of the romcal library.
    public static var version: String {
        RomcalFFI.version()
    }

    /// Create a new RomcalCalendar instance with the specified configuration.
    ///
    /// - Parameters:
    ///   - calendar: Calendar type (e.g., "general_roman", "france", "usa"). Defaults to "general_roman".
    ///   - locale: Locale for translations (e.g., "en", "fr", "es"). Defaults to "en".
    ///   - epiphanyOnSunday: Whether Epiphany is celebrated on Sunday. Defaults to false.
    ///   - ascensionOnSunday: Whether Ascension is celebrated on Sunday. Defaults to false.
    ///   - corpusChristiOnSunday: Whether Corpus Christi is celebrated on Sunday. Defaults to true.
    ///   - easterCalculationType: Easter calculation method. Defaults to .gregorian.
    ///   - context: Calendar context. Defaults to .gregorian.
    ///   - calendarDefinitionsJson: Calendar definitions as JSON string (optional).
    ///   - resourcesJson: Resources/translations as JSON string (optional).
    /// - Throws: `RomcalError` if configuration is invalid.
    public init(
        calendar: String = "general_roman",
        locale: String = "en",
        epiphanyOnSunday: Bool = false,
        ascensionOnSunday: Bool = false,
        corpusChristiOnSunday: Bool = true,
        easterCalculationType: RomcalEasterCalculationType = .gregorian,
        context: RomcalCalendarContext = .gregorian,
        calendarDefinitionsJson: String? = nil,
        resourcesJson: String? = nil
    ) throws {
        let config = RomcalFFI.RomcalConfig(
            calendar: calendar,
            locale: locale,
            epiphanyOnSunday: epiphanyOnSunday,
            ascensionOnSunday: ascensionOnSunday,
            corpusChristiOnSunday: corpusChristiOnSunday,
            easterCalculationType: easterCalculationType.rawValue,
            context: context.rawValue,
            calendarDefinitionsJson: calendarDefinitionsJson,
            resourcesJson: resourcesJson
        )
        do {
            self.inner = try RomcalFFI.Romcal(config: config)
        } catch let error as RomcalFFI.RomcalError {
            throw RomcalError(from: error)
        }
    }

    /// Get the calendar type.
    public var calendar: String {
        inner.getCalendar()
    }

    /// Get the locale.
    public var locale: String {
        inner.getLocale()
    }

    /// Whether Epiphany is celebrated on Sunday.
    public var epiphanyOnSunday: Bool {
        inner.getEpiphanyOnSunday()
    }

    /// Whether Ascension is celebrated on Sunday.
    public var ascensionOnSunday: Bool {
        inner.getAscensionOnSunday()
    }

    /// Whether Corpus Christi is celebrated on Sunday.
    public var corpusChristiOnSunday: Bool {
        inner.getCorpusChristiOnSunday()
    }

    /// Get the Easter calculation type.
    public var easterCalculationType: RomcalEasterCalculationType {
        RomcalEasterCalculationType(rawValue: inner.getEasterCalculationType()) ?? .gregorian
    }

    /// Get the calendar context.
    public var context: RomcalCalendarContext {
        RomcalCalendarContext(rawValue: inner.getContext()) ?? .gregorian
    }

    /// Generate the complete liturgical calendar for a given liturgical year.
    ///
    /// - Parameter year: The liturgical year to generate (e.g., 2025).
    /// - Returns: A dictionary mapping date strings (YYYY-MM-DD) to arrays of liturgical day dictionaries.
    /// - Throws: `RomcalError` if the year is invalid or calendar generation fails.
    public func liturgicalCalendar(year: Int32) throws -> [String: [[String: Any]]] {
        do {
            let json = try inner.generateLiturgicalCalendar(year: year)
            return try parseCalendarJson(json)
        } catch let error as RomcalFFI.RomcalError {
            throw RomcalError(from: error)
        }
    }

    /// Generate a mass-centric view of the liturgical calendar for a given year.
    ///
    /// This provides Mass-specific information including readings, prayers,
    /// and other elements needed for celebrating the Eucharist.
    ///
    /// - Parameter year: The year to generate (e.g., 2025).
    /// - Returns: A dictionary mapping date strings (YYYY-MM-DD) to arrays of mass context dictionaries.
    /// - Throws: `RomcalError` if the year is invalid or calendar generation fails.
    public func massCalendar(year: Int32) throws -> [String: [[String: Any]]] {
        do {
            let json = try inner.generateMassCalendar(year: year)
            return try parseCalendarJson(json)
        } catch let error as RomcalFFI.RomcalError {
            throw RomcalError(from: error)
        }
    }

    /// Get the date of a specific celebration by its ID.
    ///
    /// - Parameters:
    ///   - celebrationId: The unique identifier of the celebration (e.g., "christmas", "easter").
    ///   - year: The year to look up.
    /// - Returns: The date in YYYY-MM-DD format.
    /// - Throws: `RomcalError` if the celebration is not found or the year is invalid.
    public func getDate(celebrationId: String, year: Int32) throws -> String {
        do {
            return try inner.getDate(id: celebrationId, year: year)
        } catch let error as RomcalFFI.RomcalError {
            throw RomcalError(from: error)
        }
    }

    // MARK: - Private helpers

    private func parseCalendarJson(_ json: String) throws -> [String: [[String: Any]]] {
        guard let data = json.data(using: .utf8) else {
            throw RomcalError.parseError("Failed to convert JSON string to data")
        }
        guard let result = try JSONSerialization.jsonObject(with: data) as? [String: [[String: Any]]] else {
            throw RomcalError.parseError("Failed to parse calendar JSON")
        }
        return result
    }
}

// MARK: - Data Merge Helpers

/// Merge multiple resource files (meta.json + entities.*.json) into a single Resources object.
///
/// This helper allows you to load resource files however you want and then
/// merge them into the expected structure.
///
/// - Parameters:
///   - locale: The locale code (e.g., "fr", "en")
///   - filesJson: An array of JSON strings, each representing a resource file
/// - Returns: A merged Resources object
/// - Throws: `RomcalError` if parsing fails
public func mergeResourceFiles(locale: String, filesJson: [String]) throws -> Resources {
    do {
        let json = try RomcalFFI.mergeResourceFiles(locale: locale, filesJson: filesJson)
        guard let data = json.data(using: .utf8) else {
            throw RomcalError.parseError("Failed to convert JSON to data")
        }
        return try JSONDecoder().decode(Resources.self, from: data)
    } catch let error as RomcalFFI.RomcalError {
        throw RomcalError(from: error)
    }
}

/// Merge multiple resource files (meta.json + entities.*.json) into a single JSON string.
///
/// - Parameters:
///   - locale: The locale code (e.g., "fr", "en")
///   - filesJson: An array of JSON strings, each representing a resource file
/// - Returns: A JSON string representing the merged Resources object
/// - Throws: `RomcalError` if parsing fails
public func mergeResourceFilesJson(locale: String, filesJson: [String]) throws -> String {
    do {
        return try RomcalFFI.mergeResourceFiles(locale: locale, filesJson: filesJson)
    } catch let error as RomcalFFI.RomcalError {
        throw RomcalError(from: error)
    }
}

/// Merge/validate multiple calendar definition files.
///
/// This helper allows you to load calendar definition files however you want
/// and then validate them into the expected structure.
///
/// - Parameter filesJson: An array of JSON strings, each representing a calendar definition
/// - Returns: An array of validated CalendarDefinition objects
/// - Throws: `RomcalError` if parsing fails
public func mergeCalendarDefinitions(filesJson: [String]) throws -> [CalendarDefinition] {
    do {
        let json = try RomcalFFI.mergeCalendarDefinitions(filesJson: filesJson)
        guard let data = json.data(using: .utf8) else {
            throw RomcalError.parseError("Failed to convert JSON to data")
        }
        return try JSONDecoder().decode([CalendarDefinition].self, from: data)
    } catch let error as RomcalFFI.RomcalError {
        throw RomcalError(from: error)
    }
}

/// Merge/validate multiple calendar definition files as JSON string.
///
/// - Parameter filesJson: An array of JSON strings, each representing a calendar definition
/// - Returns: A JSON string representing an array of CalendarDefinition objects
/// - Throws: `RomcalError` if parsing fails
public func mergeCalendarDefinitionsJson(filesJson: [String]) throws -> String {
    do {
        return try RomcalFFI.mergeCalendarDefinitions(filesJson: filesJson)
    } catch let error as RomcalFFI.RomcalError {
        throw RomcalError(from: error)
    }
}
