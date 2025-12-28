# Romcal Kotlin Types

Kotlin type definitions for the romcal liturgical calendar library.

## Requirements

- Kotlin 2.1.0+
- JDK 21+
- Gradle 8.x (or use the wrapper after first setup)

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
gradle wrapper
```

### Building

```bash
# With Makefile
make build

# Or directly with Gradle
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
    implementation("dev.romcal:romcal-types:4.0.0-beta.3")
}
```

### Example

```kotlin
import com.romcal.types.*
import kotlinx.serialization.json.Json

fun main() {
    val json = Json { ignoreUnknownKeys = true }

    // Parse a liturgical day from JSON
    val jsonString = """
        {
            "id": "easter_sunday",
            "fullname": "Easter Sunday",
            "date": "2025-04-20",
            "rank": "SOLEMNITY",
            "season": "EASTER_TIME",
            ...
        }
    """

    // Enums are strongly typed
    val rank = Rank.Solemnity
    println(rank.string)  // "SOLEMNITY"

    val color = Color.White
    println(color.string)  // "WHITE"

    val season = Season.EasterTime
    println(season.string)  // "EASTER_TIME"
}
```

## Type Structure

The types are organized as follows:

- **Types.kt** - Generated types from Rust via Typeshare
- **TypesCompat.kt** - Compatibility types for:
  - `BTreeMap<K, V>` → `Map<K, V>`
  - Untagged enum types represented as `JsonElement`

### Untagged Enum Types

Some Rust types use `#[serde(untagged)]` which cannot be directly represented in Kotlin's type system. These are typed as `JsonElement`:

- `DateDef` - Date definition (can be string, object, or enum)
- `SaintDateDef` - Saint date (can be number, string, or object)
- `SaintCount` - Saint count (can be number or "many")
- `ColorsDef`, `CommonsDef`, `TitlesDef` - Compound definitions
- `EntityRef` - Entity reference (can be string or object)

Use Kotlin's serialization polymorphism or runtime type checking to handle these types.

## Regenerating Types

If you modify Rust types in `core/src/`, you need to regenerate `Types.kt`:

```bash
# Install Typeshare CLI (one-time setup)
cargo install typeshare-cli

# Regenerate types
make generate-types
```

## License

Apache License 2.0. See [LICENSE](../../LICENSE) for details.
