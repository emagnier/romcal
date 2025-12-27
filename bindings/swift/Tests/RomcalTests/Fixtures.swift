/// Test fixtures for Romcal Swift binding (equivalent to conftest.py)

import Foundation

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
func loadAllCalendarDefinitions() throws -> [[String: Any]] {
    let definitionsDir = getDataDir().appendingPathComponent("definitions")
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

/// Load all resources from the data folder.
/// Each locale has meta.json + entities.*.json files that need to be merged.
func loadAllResources() throws -> [[String: Any]] {
    let resourcesDir = getDataDir().appendingPathComponent("resources")
    var resources: [[String: Any]] = []

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

    // Merge files for each locale
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

/// Load calendar definitions as JSON string.
func loadCalendarDefinitionsJson() throws -> String {
    let definitions = try loadAllCalendarDefinitions()
    let data = try JSONSerialization.data(withJSONObject: definitions)
    return String(data: data, encoding: .utf8) ?? "[]"
}

/// Load resources as JSON string.
func loadResourcesJson() throws -> String {
    let resources = try loadAllResources()
    let data = try JSONSerialization.data(withJSONObject: resources)
    return String(data: data, encoding: .utf8) ?? "[]"
}
