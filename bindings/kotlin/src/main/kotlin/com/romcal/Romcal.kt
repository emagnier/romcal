/**
 * Romcal - Liturgical calendar for the Roman Rite of the Catholic Church.
 *
 * Romcal is a liturgical calendar library for the Roman Rite of the Catholic Church.
 * It computes liturgical days, seasons, and Mass contexts for any given year.
 *
 * Example usage:
 * ```kotlin
 * import com.romcal.Romcal
 *
 * // Create a Romcal instance with French calendar and locale
 * val romcal = Romcal(calendar = "france", locale = "fr")
 *
 * // Generate the liturgical calendar for 2025
 * val calendar = romcal.liturgicalCalendar(2025)
 *
 * // Access liturgical days
 * for ((date, days) in calendar) {
 *     for (day in days) {
 *         println("$date: ${day["id"]} (${day["rank"]})")
 *     }
 * }
 *
 * // Get a specific celebration date
 * val christmas = romcal.getDate("christmas", 2025)
 * println("Christmas 2025: $christmas")
 * ```
 */
package com.romcal

import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonArray
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.JsonNull
import kotlinx.serialization.json.JsonObject
import kotlinx.serialization.json.JsonPrimitive
import kotlinx.serialization.json.booleanOrNull
import kotlinx.serialization.json.doubleOrNull
import kotlinx.serialization.json.longOrNull
import uniffi.romcal_uniffi.Romcal as RomcalFFI
import uniffi.romcal_uniffi.RomcalConfig as RomcalConfigFFI
import uniffi.romcal_uniffi.RomcalException as RomcalExceptionFFI

/**
 * Easter calculation type.
 */
enum class EasterCalculationType(val value: String) {
  /** Gregorian calculation (default) */
  GREGORIAN("GREGORIAN"),

  /** Julian calculation converted to Gregorian */
  JULIAN("JULIAN"),
}

/**
 * Calendar context type.
 */
enum class CalendarContext(val value: String) {
  /** Gregorian year (January 1 to December 31) */
  GREGORIAN("GREGORIAN"),

  /** Liturgical year (first Sunday of Advent to the day before the first Sunday of Advent of the next year) */
  LITURGICAL("LITURGICAL"),
}

// Re-export RomcalException from the FFI module
typealias RomcalException = RomcalExceptionFFI

/**
 * Liturgical calendar for the Roman Rite of the Catholic Church.
 *
 * Computes liturgical days, seasons, and Mass contexts for any given year.
 * Supports various regional calendars and locales.
 *
 * @property calendar Calendar type (e.g., "general_roman", "france", "usa"). Defaults to "general_roman".
 * @property locale Locale for translations (e.g., "en", "fr", "es"). Defaults to "en".
 * @property epiphanyOnSunday Whether Epiphany is celebrated on Sunday. Defaults to false.
 * @property ascensionOnSunday Whether Ascension is celebrated on Sunday. Defaults to false.
 * @property corpusChristiOnSunday Whether Corpus Christi is celebrated on Sunday. Defaults to true.
 * @property easterCalculationType Easter calculation method. Defaults to GREGORIAN.
 * @property context Calendar context. Defaults to GREGORIAN.
 * @property calendarDefinitionsJson Calendar definitions as JSON string (optional).
 * @property resourcesJson Resources/translations as JSON string (optional).
 */
class Romcal(
  calendar: String = "general_roman",
  locale: String = "en",
  epiphanyOnSunday: Boolean = false,
  ascensionOnSunday: Boolean = false,
  corpusChristiOnSunday: Boolean = true,
  easterCalculationType: EasterCalculationType = EasterCalculationType.GREGORIAN,
  context: CalendarContext = CalendarContext.GREGORIAN,
  calendarDefinitionsJson: String? = null,
  resourcesJson: String? = null,
) : AutoCloseable {
  private val inner: RomcalFFI
  private val json = Json { ignoreUnknownKeys = true }

  init {
    val config =
      RomcalConfigFFI(
        calendar = calendar,
        locale = locale,
        epiphanyOnSunday = epiphanyOnSunday,
        ascensionOnSunday = ascensionOnSunday,
        corpusChristiOnSunday = corpusChristiOnSunday,
        easterCalculationType = easterCalculationType.value,
        context = context.value,
        calendarDefinitionsJson = calendarDefinitionsJson,
        resourcesJson = resourcesJson,
      )
    inner = RomcalFFI(config)
  }

  /** Get the calendar type. */
  val calendar: String
    get() = inner.getCalendar()

  /** Get the locale. */
  val locale: String
    get() = inner.getLocale()

  /** Whether Epiphany is celebrated on Sunday. */
  val epiphanyOnSunday: Boolean
    get() = inner.getEpiphanyOnSunday()

  /** Whether Ascension is celebrated on Sunday. */
  val ascensionOnSunday: Boolean
    get() = inner.getAscensionOnSunday()

  /** Whether Corpus Christi is celebrated on Sunday. */
  val corpusChristiOnSunday: Boolean
    get() = inner.getCorpusChristiOnSunday()

  /** Get the Easter calculation type. */
  val easterCalculationType: EasterCalculationType
    get() =
      when (inner.getEasterCalculationType()) {
        "JULIAN" -> EasterCalculationType.JULIAN
        else -> EasterCalculationType.GREGORIAN
      }

  /** Get the calendar context. */
  val context: CalendarContext
    get() =
      when (inner.getContext()) {
        "LITURGICAL" -> CalendarContext.LITURGICAL
        else -> CalendarContext.GREGORIAN
      }

  /**
   * Generate the complete liturgical calendar for a given liturgical year.
   *
   * @param year The liturgical year to generate (e.g., 2025).
   * @return A map of date strings (YYYY-MM-DD) to lists of liturgical day maps.
   *         Each date may have multiple liturgical days due to optional memorials.
   *         Each day map contains properties like "id", "rank", "season", "fullname", etc.
   * @throws RomcalException If the year is invalid or calendar generation fails.
   */
  fun liturgicalCalendar(year: Int): Map<String, List<Map<String, Any?>>> {
    val jsonString = inner.generateLiturgicalCalendar(year)
    return parseCalendarJson(jsonString)
  }

  /**
   * Generate a mass-centric view of the liturgical calendar for a given year.
   *
   * This provides Mass-specific information including readings, prayers,
   * and other elements needed for celebrating the Eucharist.
   *
   * @param year The year to generate (e.g., 2025).
   * @return A map of date strings (YYYY-MM-DD) to lists of mass context maps.
   * @throws RomcalException If the year is invalid or calendar generation fails.
   */
  fun massCalendar(year: Int): Map<String, List<Map<String, Any?>>> {
    val jsonString = inner.generateMassCalendar(year)
    return parseCalendarJson(jsonString)
  }

  /**
   * Parse calendar JSON into a map structure.
   */
  private fun parseCalendarJson(jsonString: String): Map<String, List<Map<String, Any?>>> {
    val jsonElement = json.parseToJsonElement(jsonString)
    if (jsonElement !is JsonObject) {
      throw IllegalStateException("Expected JSON object at root")
    }
    return jsonElement.mapValues { (_, value) ->
      if (value !is JsonArray) {
        throw IllegalStateException("Expected JSON array for date entries")
      }
      value.map { dayElement ->
        if (dayElement !is JsonObject) {
          throw IllegalStateException("Expected JSON object for day entry")
        }
        dayElement.toMap()
      }
    }
  }

  /**
   * Convert a JsonObject to a Map<String, Any?>.
   */
  private fun JsonObject.toMap(): Map<String, Any?> {
    return mapValues { (_, value) -> value.toAny() }
  }

  /**
   * Convert a JsonElement to its Kotlin equivalent.
   */
  private fun JsonElement.toAny(): Any? {
    return when (this) {
      is JsonNull -> null
      is JsonPrimitive -> {
        when {
          isString -> content
          booleanOrNull != null -> booleanOrNull
          longOrNull != null -> longOrNull
          doubleOrNull != null -> doubleOrNull
          else -> content
        }
      }
      is JsonArray -> map { it.toAny() }
      is JsonObject -> toMap()
    }
  }

  /**
   * Get the date of a specific celebration by its ID.
   *
   * @param celebrationId The unique identifier of the celebration (e.g., "christmas", "easter").
   * @param year The year to look up.
   * @return The date in YYYY-MM-DD format.
   * @throws RomcalException If the celebration is not found or the year is invalid.
   */
  fun getDate(
    celebrationId: String,
    year: Int,
  ): String {
    return inner.getDate(celebrationId, year)
  }

  override fun close() {
    inner.close()
  }

  override fun toString(): String {
    return "Romcal(calendar=$calendar, locale=$locale, context=$context, " +
      "easterCalculationType=$easterCalculationType, epiphanyOnSunday=$epiphanyOnSunday, " +
      "ascensionOnSunday=$ascensionOnSunday, corpusChristiOnSunday=$corpusChristiOnSunday)"
  }

  companion object {
    /**
     * Get the romcal library version.
     */
    val version: String
      get() = uniffi.romcal_uniffi.version()

    /**
     * Merge multiple resource files (meta.json + entities.*.json) into a single Resources JSON string.
     *
     * This helper allows you to load resource files however you want and then
     * merge them into the expected structure.
     *
     * @param locale The locale code (e.g., "fr", "en")
     * @param filesJson A list of JSON strings, each representing a resource file
     * @return A JSON string representing the merged Resources object
     * @throws RomcalException If parsing fails
     */
    fun mergeResourceFilesJson(
      locale: String,
      filesJson: List<String>,
    ): String {
      return uniffi.romcal_uniffi.mergeResourceFiles(locale, filesJson)
    }

    /**
     * Merge/validate multiple calendar definition files as JSON string.
     *
     * This helper allows you to load calendar definition files however you want
     * and then validate them into the expected structure.
     *
     * @param filesJson A list of JSON strings, each representing a calendar definition
     * @return A JSON string representing an array of CalendarDefinition objects
     * @throws RomcalException If parsing fails
     */
    fun mergeCalendarDefinitionsJson(filesJson: List<String>): String {
      return uniffi.romcal_uniffi.mergeCalendarDefinitions(filesJson)
    }
  }
}
