// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "Cella",
    platforms: [
        .iOS(.v17),
        .macOS(.v14)
    ],
    products: [
        .library(
            name: "Cella",
            targets: ["Cella"]
        )
    ],
    targets: [
        .target(
            name: "Cella",
            path: "Sources"
        )
    ]
)
