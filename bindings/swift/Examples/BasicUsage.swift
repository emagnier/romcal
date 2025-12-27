#!/usr/bin/env swift
/// Romcal - Basic Usage Example
///
/// This example demonstrates:
/// - Loading calendar definitions from the data folder
/// - Loading and merging resources (translations) from the data folder
/// - Creating a Romcal instance with loaded data
/// - Generating calendars with translated names
///
/// Build and run with:
///   make example
/// Or manually:
///   swift run BasicUsage

import Foundation
import Romcal

/// Get the data directory path
func getDataDir() -> URL {
    // Navigate from Examples to the data directory
    let currentDir = URL(fileURLWithPath: FileManager.default.currentDirectoryPath)
    return currentDir
        .deletingLastPathComponent()  // swift
        .deletingLastPathComponent()  // bindings
        .appendingPathComponent("data")
}

/// Load all calendar definitions from the data folder
func loadCalendarDefinitions(from dataDir: URL) throws -> [[String: Any]] {
    let definitionsDir = dataDir.appendingPathComponent("definitions")
    var definitions: [[String: Any]] = []

    let fileManager = FileManager.default
    if let enumerator = fileManager.enumerator(
        at: definitionsDir,
        includingPropertiesForKeys: [.isRegularFileKey],
        options: [.skipsHiddenFiles]
    ) {
        for case let fileURL as URL in enumerator {
            if fileURL.pathExtension == "json" {
                let data = try Data(contentsOf: fileURL)
                if let json = try JSONSerialization.jsonObject(with: data) as? [String: Any] {
                    definitions.append(json)
                }
            }
        }
    }

    return definitions
}

/// Load all resources from the data folder
func loadResources(from dataDir: URL) throws -> [[String: Any]] {
    let resourcesDir = dataDir.appendingPathComponent("resources")
    var resources: [[String: Any]] = []

    let fileManager = FileManager.default
    var filesByLocale: [String: [URL]] = [:]

    if let enumerator = fileManager.enumerator(
        at: resourcesDir,
        includingPropertiesForKeys: [.isRegularFileKey],
        options: [.skipsHiddenFiles]
    ) {
        for case let fileURL as URL in enumerator {
            if fileURL.pathExtension == "json" {
                let locale = fileURL.deletingLastPathComponent().lastPathComponent
                if filesByLocale[locale] == nil {
                    filesByLocale[locale] = []
                }
                filesByLocale[locale]?.append(fileURL)
            }
        }
    }

    for (locale, localeFiles) in filesByLocale {
        var metadata: [String: Any]?
        var entities: [String: Any] = [:]

        for file in localeFiles {
            let data = try Data(contentsOf: file)
            if let content = try JSONSerialization.jsonObject(with: data) as? [String: Any] {
                if file.lastPathComponent == "meta.json" {
                    metadata = content["metadata"] as? [String: Any]
                } else if file.lastPathComponent.hasPrefix("entities."),
                          let fileEntities = content["entities"] as? [String: Any] {
                    entities.merge(fileEntities) { _, new in new }
                }
            }
        }

        var resource: [String: Any] = ["locale": locale]
        if let metadata = metadata {
            resource["metadata"] = metadata
        }
        if !entities.isEmpty {
            resource["entities"] = entities
        }
        resources.append(resource)
    }

    return resources
}

func main() throws {
    print("=== Romcal with Data Example ===\n")

    // Determine the data directory path
    let dataDir = getDataDir()

    guard FileManager.default.fileExists(atPath: dataDir.path) else {
        print("Error: Data directory not found at \(dataDir.path)")
        print("Please run this script from the bindings/swift directory.")
        return
    }

    // ========================================================================
    // Load Data
    // ========================================================================

    print("Loading calendar definitions...")
    let calendarDefinitions = try loadCalendarDefinitions(from: dataDir)
    print("  Loaded \(calendarDefinitions.count) calendar definitions")

    print("Loading resources...")
    let resources = try loadResources(from: dataDir)
    print("  Loaded \(resources.count) locale resources")
    print()

    // Convert to JSON strings
    let calendarDefinitionsData = try JSONSerialization.data(withJSONObject: calendarDefinitions)
    let calendarDefinitionsJson = String(data: calendarDefinitionsData, encoding: .utf8) ?? "[]"

    let resourcesData = try JSONSerialization.data(withJSONObject: resources)
    let resourcesJson = String(data: resourcesData, encoding: .utf8) ?? "[]"

    // ========================================================================
    // Create Romcal with French Calendar and Locale
    // ========================================================================

    print("Creating French calendar instance with loaded data...")
    let romcal = try RomcalCalendar(
        calendar: "france",
        locale: "fr",
        calendarDefinitionsJson: calendarDefinitionsJson,
        resourcesJson: resourcesJson
    )
    print("  Calendar: \(romcal.calendar)")
    print("  Locale: \(romcal.locale)")
    print()

    // ========================================================================
    // Generate Calendar
    // ========================================================================

    print("Generating liturgical calendar for 2026...")
    let calendar = try romcal.liturgicalCalendar(year: 2026)
    let dates = calendar.keys.sorted()
    print("  Total dates: \(dates.count)")
    print("  First date: \(dates.first ?? "N/A")")
    print("  Last date: \(dates.last ?? "N/A")")
    print()

    // ========================================================================
    // Notable Celebrations with French Names
    // ========================================================================

    print("Celebrations with French names:")

    // Easter 2026 (April 5)
    if let easter = calendar["2026-04-05"], let day = easter.first {
        print("  Easter (2026-04-05):")
        let fullname = day["fullname"] as? String ?? day["id"] as? String ?? "Unknown"
        let seasonName = day["season_name"] as? String ?? day["season"] as? String ?? "Unknown"
        let rankName = day["rank_name"] as? String ?? day["rank"] as? String ?? "Unknown"
        print("    Full name: \(fullname)")
        print("    Season: \(seasonName)")
        print("    Rank: \(rankName)")
    }

    // Assumption of Mary (August 15)
    if let assumption = calendar["2026-08-15"], let day = assumption.first {
        let fullname = day["fullname"] as? String ?? day["id"] as? String ?? "Unknown"
        print("  Assumption (2026-08-15): \(fullname)")
    }

    // All Saints (November 1)
    if let allSaints = calendar["2026-11-01"], let day = allSaints.first {
        let fullname = day["fullname"] as? String ?? day["id"] as? String ?? "Unknown"
        print("  All Saints (2026-11-01): \(fullname)")
    }

    // Christmas 2026
    if let christmas = calendar["2026-12-25"], let day = christmas.first {
        let fullname = day["fullname"] as? String ?? day["id"] as? String ?? "Unknown"
        print("  Christmas (2026-12-25): \(fullname)")
    }

    // French-specific saint: Saint Jean-Marie Vianney (August 4)
    if let vianneyDay = calendar["2026-08-04"] {
        for day in vianneyDay {
            if let id = day["id"] as? String, id.lowercased().contains("vianney") {
                let fullname = day["fullname"] as? String ?? id
                print("  St. Jean-Marie Vianney (2026-08-04): \(fullname)")
                break
            }
        }
    }

    print()

    // ========================================================================
    // Mass Calendar with French Names
    // ========================================================================

    print("Generating mass calendar for 2026...")
    let massCalendar = try romcal.massCalendar(year: 2026)
    print("  Total dates with masses: \(massCalendar.count)")
    print()

    // Christmas masses with French names
    if let christmasMasses = massCalendar["2026-12-25"] {
        print("  Christmas masses (\(christmasMasses.count) total):")
        for mass in christmasMasses {
            let massTimeName = mass["mass_time_name"] as? String ?? mass["mass_time"] as? String ?? "Unknown"
            let fullname = mass["fullname"] as? String ?? mass["id"] as? String ?? "Unknown"
            print("    - \(massTimeName): \(fullname)")
        }
    }

    print("\n=== Done ===")
}

do {
    try main()
} catch {
    print("Error: \(error)")
    exit(1)
}
