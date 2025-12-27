# Romcal

A Swift library for calculating Catholic liturgical dates and generating liturgical calendars. Powered by Rust via UniFFI bindings.

For the Rust library, see [romcal](../../core/). For command-line usage, see the [CLI documentation](../../cli/).

## Requirements

- [Swift](https://swift.org/) 5.9 or later
- macOS 12+, iOS 15+, tvOS 15+, or watchOS 8+
- [Rust](https://rustup.rs/) 1.85 or later (for building from source)

## Installation

### Swift Package Manager

Add the package to your `Package.swift`:

```swift
dependencies: [
    .package(path: "../path/to/romcal/bindings/swift")
]
```

### Building from Source

```bash
cd bindings/swift

# Build everything (Rust + Swift)
make build

# Or step by step:
make build-rust        # Build the Rust UniFFI library
make generate-bindings # Generate Swift bindings
make generate-types    # Generate Swift types from JSON schema
swift build            # Build Swift package
```

## Quick Start

```swift
import Romcal

// Create a default instance
let romcal = try RomcalCalendar()

// Get a specific liturgical date
let easter = try romcal.getDate(celebrationId: "easter_sunday", year: 2026)
print(easter)  // "2026-04-05"

// Generate the liturgical calendar for year 2026
let calendar = try romcal.liturgicalCalendar(year: 2026)

// Access a specific date
if let christmas = calendar["2026-12-25"]?.first {
    print(christmas["fullname"] as? String ?? "")  // "The Nativity of the Lord"
}
```

## Configuration

### Using Configuration Options

```swift
import Romcal

// With calendar and locale
let romcal1 = try RomcalCalendar(calendar: "france", locale: "fr")

// With full configuration
let romcal2 = try RomcalCalendar(
    calendar: "france",
    locale: "fr",
    epiphanyOnSunday: true,
    ascensionOnSunday: true,
    corpusChristiOnSunday: true,
    easterCalculationType: .gregorian,
    context: .liturgical
)
```

### Configuration Options

| Option                    | Type                          | Default           | Description                                             |
| ------------------------- | ----------------------------- | ----------------- | ------------------------------------------------------- |
| `calendar`                | `String`                      | `"general_roman"` | Calendar ID (e.g., `"france"`, `"united_states"`)       |
| `locale`                  | `String`                      | `"en"`            | Locale code (e.g., `"fr"`, `"es"`)                      |
| `context`                 | `RomcalCalendarContext`       | `.gregorian`      | `.gregorian` (Jan-Dec) or `.liturgical` (Advent-Advent) |
| `epiphanyOnSunday`        | `Bool`                        | `false`           | Celebrate Epiphany on Sunday (Jan 2-8) instead of Jan 6 |
| `ascensionOnSunday`       | `Bool`                        | `false`           | Celebrate Ascension on Sunday instead of Thursday       |
| `corpusChristiOnSunday`   | `Bool`                        | `true`            | Celebrate Corpus Christi on Sunday instead of Thursday  |
| `easterCalculationType`   | `RomcalEasterCalculationType` | `.gregorian`      | `.gregorian` or `.julian` Easter calculation            |
| `calendarDefinitionsJson` | `String?`                     | `nil`             | JSON string of calendar definitions                     |
| `resourcesJson`           | `String?`                     | `nil`             | JSON string of locale resources                         |

### Loading Calendar Data

Without loading data, only the Proper of Time is available. To include the General Roman Calendar, particular calendars, and localized names, load calendar definitions and resources:

```swift
import Foundation
import Romcal

let dataDir = URL(fileURLWithPath: "data")

func loadCalendarDefinitions() throws -> String {
    let definitionsDir = dataDir.appendingPathComponent("definitions")
    var definitions: [[String: Any]] = []

    let enumerator = FileManager.default.enumerator(at: definitionsDir, includingPropertiesForKeys: nil)
    while let fileURL = enumerator?.nextObject() as? URL {
        if fileURL.pathExtension == "json" {
            let data = try Data(contentsOf: fileURL)
            if let json = try JSONSerialization.jsonObject(with: data) as? [String: Any] {
                definitions.append(json)
            }
        }
    }

    let data = try JSONSerialization.data(withJSONObject: definitions)
    return String(data: data, encoding: .utf8) ?? "[]"
}

// Create instance with loaded data
let romcal = try RomcalCalendar(
    calendar: "france",
    locale: "fr",
    calendarDefinitionsJson: try loadCalendarDefinitions(),
    resourcesJson: try loadResources()  // Similar implementation
)
```

## API

### RomcalCalendar()

Creates a new RomcalCalendar instance.

```swift
// Default configuration
let romcal1 = try RomcalCalendar()

// With calendar and locale
let romcal2 = try RomcalCalendar(calendar: "france", locale: "fr")

// With partial configuration
let romcal3 = try RomcalCalendar(
    calendar: "france",
    locale: "fr",
    epiphanyOnSunday: true
)
```

### RomcalCalendar Instance

#### liturgicalCalendar(year:)

Generate the complete liturgical calendar for a given year.

```swift
let calendar = try romcal.liturgicalCalendar(year: 2026)
// calendar is [String: [[String: Any]]]
// Keys are dates in "YYYY-MM-DD" format

for (date, days) in calendar {
    for day in days {
        let fullname = day["fullname"] as? String ?? ""
        let rank = day["rank"] as? String ?? ""
        print("\(date): \(fullname) (\(rank))")
    }
}
```

#### massCalendar(year:)

Generate a mass-centric view of the calendar organized by civil date and mass time.

```swift
let massCalendar = try romcal.massCalendar(year: 2026)
// massCalendar is [String: [[String: Any]]]

// Evening masses appear on the previous civil day
if let easterVigilDay = massCalendar["2026-04-04"] {
    let vigil = easterVigilDay.first { ($0["mass_time"] as? String) == "EASTER_VIGIL" }
    print(vigil?["liturgical_date"] as? String ?? "")  // "2026-04-05"
}
```

#### getDate(celebrationId:year:)

Get a liturgical date by its ID.

```swift
let easter = try romcal.getDate(celebrationId: "easter_sunday", year: 2026)     // "2026-04-05"
let ashWed = try romcal.getDate(celebrationId: "ash_wednesday", year: 2026)     // "2026-02-18"
let pentecost = try romcal.getDate(celebrationId: "pentecost_sunday", year: 2026) // "2026-05-24"
let christmas = try romcal.getDate(celebrationId: "christmas", year: 2026)       // "2026-12-25"
```

Any date ID from the liturgical calendar can be used (e.g., `easter_sunday`, `christmas`, `ordinary_time_5_monday`).

#### Properties

Access the resolved configuration:

```swift
print(romcal.calendar)                // "france"
print(romcal.locale)                  // "fr"
print(romcal.epiphanyOnSunday)        // true
print(romcal.ascensionOnSunday)       // false
print(romcal.corpusChristiOnSunday)   // true
print(romcal.easterCalculationType)   // .gregorian
print(romcal.context)                 // .gregorian
```

## Key Types

For detailed documentation on liturgical types (seasons, ranks, precedence, colors, cycles, mass times), see the [romcal documentation](../../core/README.md#key-types).

## Error Handling

All operations may throw `RomcalCalendarError`:

```swift
import Romcal

do {
    let romcal = try RomcalCalendar()
    // Year must be >= 1583 (Gregorian calendar adoption)
    let calendar = try romcal.liturgicalCalendar(year: 1500)
} catch let error as RomcalCalendarError {
    print("Romcal error: \(error.localizedDescription)")
}
```

Error types:

- `.invalidYear` - Year before 1583
- `.invalidConfig` - Invalid configuration
- `.notFound` - Celebration ID not found
- `.parseError` - JSON parsing error
- `.calculationError` - Calendar calculation error

## Development

### Requirements

- [Swift](https://swift.org/) 5.9 or later
- [Rust](https://rustup.rs/) 1.85 or later
- [Node.js](https://nodejs.org/) (for Quicktype type generation)

### Setup

```bash
cd bindings/swift

# Build everything
make build

# Install optional tools
brew install swift-format  # For code formatting
brew install swiftlint     # For linting
```

### Available Make Targets

```bash
make help            # Show all available targets
make build           # Build everything (Rust + bindings + Swift)
make build-rust      # Build Rust UniFFI library
make generate-bindings # Generate Swift bindings from UniFFI
make generate-types  # Generate Swift types with Quicktype
make test            # Run tests
make test-verbose    # Run tests with verbose output
make clean           # Clean build artifacts
make format          # Format code with swift-format
make lint            # Lint code with SwiftLint
make example         # Run the basic usage example
```

### Testing

```bash
make test      # Run tests
make test-verbose  # Run tests with verbose output

# Or directly with Swift
swift test
```

### Project Structure

```
bindings/swift/
├── Sources/
│   ├── CRomcalFFI/
│   │   └── module.modulemap   # C module for UniFFI
│   ├── RomcalFFI/
│   │   ├── romcal_uniffi.swift      # Generated UniFFI bindings
│   │   └── romcal_uniffiFFI.h       # Generated C header
│   └── Romcal/
│       ├── Romcal.swift       # Main API wrapper
│       └── Types.swift        # Generated types from JSON schema
├── Tests/RomcalTests/
│   ├── Fixtures.swift         # Test fixtures (data loading)
│   ├── ConfigTests.swift      # Configuration tests
│   ├── CalendarTests.swift    # Calendar generation tests
│   └── DataLoadingTests.swift # Data loading tests
├── Examples/
│   └── BasicUsage.swift       # Usage example with data loading
├── Package.swift              # Swift Package Manager manifest
├── Makefile                   # Build and development tasks
└── README.md
```

### Running Examples

```bash
# Run the basic usage example
make example
```

## Related

- [romcal](https://github.com/romcal/romcal) - Main Romcal project
- [romcal](../../core/) - Core Rust library
- [romcal-cli](../../cli/) - Command-line interface
- [romcal (TypeScript)](../typescript/) - TypeScript/JavaScript binding
- [romcal (Python)](../python/) - Python binding

## License

Apache License 2.0. See [LICENSE](../../LICENSE) for details.
