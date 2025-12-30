/// Test fixtures for Romcal Swift binding (equivalent to conftest.py)

import Foundation
import Romcal

/// Data directory path (relative to the test bundle)
func getDataDir() -> URL {
    // Navigate from Tests/RomcalTests to the data directory
    let testDir = URL(fileURLWithPath: #file).deletingLastPathComponent()
    return testDir
        .deletingLastPathComponent()  // Tests
        .deletingLastPathComponent()  // swift
        .deletingLastPathComponent()  // bindings
        .deletingLastPathComponent()  // romcal
        .appendingPathComponent("data")
}

/// Load all calendar definitions from the data folder.
///
/// Note: Returns untyped data because `CalendarDefinition` contains nested
/// dictionary structures (MassesDefinitions) that Swift's JSONDecoder doesn't handle well.
func loadAllCalendarDefinitions() throws -> [[String: Any]] {
    let definitionsDir = getDataDir().appendingPathComponent("definitions")
    var filesJson: [String] = []

    let fileManager = FileManager.default
    if let enumerator = fileManager.enumerator(
        at: definitionsDir,
        includingPropertiesForKeys: [.isRegularFileKey],
        options: [.skipsHiddenFiles]
    ) {
        for case let fileURL as URL in enumerator {
            if fileURL.pathExtension == "json" {
                let data = try Data(contentsOf: fileURL)
                if let jsonString = String(data: data, encoding: .utf8) {
                    filesJson.append(jsonString)
                }
            }
        }
    }

    // Use the JSON helper and parse to untyped data (avoids decoding issues with nested enum-keyed dicts)
    let mergedJson = try mergeCalendarDefinitionsJson(filesJson: filesJson)
    let mergedData = mergedJson.data(using: .utf8)!
    return try JSONSerialization.jsonObject(with: mergedData) as! [[String: Any]]
}

/// Load all resources from the data folder.
/// Each locale has meta.json + entities.*.json files that need to be merged.
func loadAllResources() throws -> [Resources] {
    let resourcesDir = getDataDir().appendingPathComponent("resources")
    var resources: [Resources] = []

    let fileManager = FileManager.default

    // Group files by locale (parent directory name)
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

    // Merge files for each locale using the helper
    for (locale, localeFiles) in filesByLocale {
        var filesJson: [String] = []
        for file in localeFiles {
            let data = try Data(contentsOf: file)
            if let jsonString = String(data: data, encoding: .utf8) {
                filesJson.append(jsonString)
            }
        }

        let resource = try mergeResourceFiles(locale: locale, filesJson: filesJson)
        resources.append(resource)
    }

    return resources
}

/// Load calendar definitions as JSON string.
func loadCalendarDefinitionsJson() throws -> String {
    let definitionsDir = getDataDir().appendingPathComponent("definitions")
    var filesJson: [String] = []

    let fileManager = FileManager.default
    if let enumerator = fileManager.enumerator(
        at: definitionsDir,
        includingPropertiesForKeys: [.isRegularFileKey],
        options: [.skipsHiddenFiles]
    ) {
        for case let fileURL as URL in enumerator {
            if fileURL.pathExtension == "json" {
                let data = try Data(contentsOf: fileURL)
                if let jsonString = String(data: data, encoding: .utf8) {
                    filesJson.append(jsonString)
                }
            }
        }
    }

    // Use the JSON helper to get the raw merged JSON (avoids re-encoding issues)
    return try mergeCalendarDefinitionsJson(filesJson: filesJson)
}

/// Load resources as JSON string.
func loadResourcesJson() throws -> String {
    let resourcesDir = getDataDir().appendingPathComponent("resources")
    var allResources: [String] = []

    let fileManager = FileManager.default

    // Group files by locale (parent directory name)
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

    // Merge files for each locale using the JSON helper
    for (locale, localeFiles) in filesByLocale {
        var filesJson: [String] = []
        for file in localeFiles {
            let data = try Data(contentsOf: file)
            if let jsonString = String(data: data, encoding: .utf8) {
                filesJson.append(jsonString)
            }
        }

        let resourceJson = try mergeResourceFilesJson(locale: locale, filesJson: filesJson)
        allResources.append(resourceJson)
    }

    return "[\(allResources.joined(separator: ","))]"
}
