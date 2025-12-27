// swift-tools-version: 5.9
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "Romcal",
    platforms: [
        .macOS(.v12),
        .iOS(.v15),
        .tvOS(.v15),
        .watchOS(.v8),
    ],
    products: [
        .library(
            name: "Romcal",
            targets: ["Romcal"]
        ),
    ],
    targets: [
        // System library target for the C FFI (links to libromcal_uniffi)
        .systemLibrary(
            name: "CRomcalFFI",
            path: "Sources/CRomcalFFI",
            pkgConfig: nil,
            providers: []
        ),
        // UniFFI-generated Swift bindings
        .target(
            name: "RomcalFFI",
            dependencies: ["CRomcalFFI"],
            path: "Sources/RomcalFFI",
            exclude: ["romcal_uniffiFFI.h", "romcal_uniffiFFI.modulemap"],
            linkerSettings: [
                .unsafeFlags(["-L../../target/release"]),
                .linkedLibrary("romcal_uniffi"),
            ]
        ),
        // Swift wrapper API
        .target(
            name: "Romcal",
            dependencies: ["RomcalFFI"],
            path: "Sources/Romcal"
        ),
        // Tests
        .testTarget(
            name: "RomcalTests",
            dependencies: ["Romcal"],
            path: "Tests/RomcalTests"
        ),
        // Examples
        .executableTarget(
            name: "BasicUsage",
            dependencies: ["Romcal"],
            path: "Examples",
            sources: ["BasicUsage.swift"]
        ),
    ]
)
