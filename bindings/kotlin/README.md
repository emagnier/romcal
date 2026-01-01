# Romcal Kotlin Binding

Kotlin bindings for the romcal liturgical calendar library, powered by UniFFI.

## Requirements

- Kotlin 2.1.0+
- JDK 21+
- Gradle 8.x (or use the wrapper after first setup)
- Rust (for building from source)

## Setup

### First-time setup

Install Gradle if not already installed:

```bash
# macOS
brew install gradle

# Linux (SDKMAN)
sdk install gradle

# Or download from https://gradle.org/install/
```

Then initialize the wrapper:

```bash
cd bindings/kotlin
gradle wrapper
```

### Building

```bash
# Build everything (Rust + UniFFI bindings + Kotlin)
make build

# Or just build Kotlin (after UniFFI bindings are generated)
./gradlew build
```

### Running tests

```bash
make test
# or
./gradlew test
```

## Usage

Add the dependency to your `build.gradle.kts`:

```kotlin
dependencies {
    implementation("dev.romcal:romcal:4.0.0-beta.3")
}
```

### Basic Example

```kotlin
import com.romcal.Romcal
import com.romcal.CalendarContext
import com.romcal.EasterCalculationType

fun main() {
    // Create a Romcal instance with French calendar and locale
    Romcal(calendar = "france", locale = "fr").use { romcal ->
        // Generate the liturgical calendar for 2025
        val calendar = romcal.liturgicalCalendar(2025)

        // Access liturgical days
        for ((date, days) in calendar) {
            for (day in days) {
                println("$date: ${day["id"]} (${day["rank"]})")
            }
        }

        // Get a specific celebration date
        val christmas = romcal.getDate("christmas", 2025)
        println("Christmas 2025: $christmas")
    }
}
```

### With Loaded Data

For full translations and calendar definitions, load them from the data folder:

```kotlin
import com.romcal.Romcal
import java.io.File

fun main() {
    // Load calendar definitions
    val definitionsDir = File("data/definitions")
    val definitionsJson = definitionsDir.walkTopDown()
        .filter { it.extension == "json" }
        .map { it.readText() }
        .toList()
    val mergedDefinitions = Romcal.mergeCalendarDefinitionsJson(definitionsJson)

    // Load resources for French locale
    val resourcesDir = File("data/resources/fr")
    val resourcesJson = resourcesDir.walkTopDown()
        .filter { it.extension == "json" }
        .map { it.readText() }
        .toList()
    val mergedResources = Romcal.mergeResourceFilesJson("fr", resourcesJson)

    // Create Romcal with loaded data
    Romcal(
        calendar = "france",
        locale = "fr",
        calendarDefinitionsJson = mergedDefinitions,
        resourcesJson = "[$mergedResources]"
    ).use { romcal ->
        val calendar = romcal.liturgicalCalendar(2025)
        // Now calendar entries have French translations
    }
}
```

### Configuration Options

```kotlin
Romcal(
    calendar = "general_roman",      // Calendar type: "general_roman", "france", "usa", etc.
    locale = "en",                   // Locale for translations
    epiphanyOnSunday = false,        // Celebrate Epiphany on Sunday
    ascensionOnSunday = false,       // Celebrate Ascension on Sunday
    corpusChristiOnSunday = true,    // Celebrate Corpus Christi on Sunday
    easterCalculationType = EasterCalculationType.GREGORIAN,  // or JULIAN
    context = CalendarContext.GREGORIAN,  // or LITURGICAL
    calendarDefinitionsJson = null,  // Custom calendar definitions
    resourcesJson = null             // Custom translations
)
```

### Mass Calendar

```kotlin
Romcal().use { romcal ->
    val massCalendar = romcal.massCalendar(2025)

    for ((date, masses) in massCalendar) {
        for (mass in masses) {
            println("$date: ${mass["mass_time"]}")
        }
    }
}
```

## API Reference

### Romcal

| Method                                      | Description                                                   |
| ------------------------------------------- | ------------------------------------------------------------- |
| `liturgicalCalendar(year: Int)`             | Generate the liturgical calendar for a year                   |
| `massCalendar(year: Int)`                   | Generate a mass-centric view of the calendar                  |
| `getDate(celebrationId: String, year: Int)` | Get the date of a specific celebration                        |
| `close()`                                   | Release resources (use `.use {}` block for automatic cleanup) |

### Companion Object

| Property/Method                             | Description                       |
| ------------------------------------------- | --------------------------------- |
| `version`                                   | Get the romcal library version    |
| `mergeResourceFilesJson(locale, filesJson)` | Merge resource files for a locale |
| `mergeCalendarDefinitionsJson(filesJson)`   | Merge calendar definition files   |

## Project Structure

```
bindings/kotlin/
├── src/main/kotlin/com/romcal/
│   ├── Romcal.kt           # Main API wrapper
│   ├── ffi/                # UniFFI-generated bindings (auto-generated)
│   └── types/
│       ├── Types.kt        # Typeshare-generated types
│       └── TypesCompat.kt  # Compatibility types
├── src/main/resources/     # Native libraries (per platform)
├── src/test/kotlin/        # Tests
├── build.gradle.kts        # Gradle configuration
└── Makefile                # Build automation
```

## Build Targets

| Target                   | Description                                 |
| ------------------------ | ------------------------------------------- |
| `make build`             | Build everything (Rust + bindings + Kotlin) |
| `make build-rust`        | Build Rust UniFFI library                   |
| `make generate-bindings` | Generate Kotlin bindings from UniFFI        |
| `make generate-types`    | Regenerate types with Typeshare             |
| `make test`              | Run tests                                   |
| `make clean`             | Clean build artifacts                       |

## Notes on Types

The calendar methods return `Map<String, List<Map<String, Any?>>>` rather than strongly-typed objects. This is because:

1. The Rust FFI returns JSON strings
2. The types generated by Typeshare use different serialization formats than the Rust FFI output

For strongly-typed access, you can use the types in `com.romcal.types` to parse specific fields, or access map entries by key:

```kotlin
val calendar = romcal.liturgicalCalendar(2025)
val christmas = calendar["2025-12-25"]?.firstOrNull()
val rank = christmas?.get("rank") as? String  // "SOLEMNITY"
val season = christmas?.get("season") as? String  // "CHRISTMAS_TIME"
```

## License

Apache License 2.0. See [LICENSE](../../LICENSE) for details.
