/**
 * Compatibility types for Kotlin bindings.
 *
 * This file provides type aliases and placeholder types for:
 * - BTreeMap (mapped to Map)
 * - Untagged enum types that Typeshare cannot generate
 *
 * Untagged enums (SaintDateDef, DateDef, etc.) are complex union types
 * that cannot be directly represented in Kotlin's type system.
 * They are represented as Any and require runtime type checking.
 */

package com.romcal.types

import kotlinx.serialization.Serializable
import kotlinx.serialization.json.JsonElement

/**
 * Alias for BTreeMap to Kotlin's Map type.
 * BTreeMap in Rust is a sorted map, which we map to standard Kotlin Map.
 */
typealias BTreeMap<K, V> = Map<K, V>

// =============================================================================
// Untagged enum type aliases
// =============================================================================
// These types are serde untagged enums in Rust, which cannot be directly
// represented in Kotlin. We use JsonElement as a flexible container that
// preserves the original JSON structure for runtime inspection.

/**
 * Date definition for a liturgical day.
 *
 * Can be one of:
 * - DateDefExtended (object with month, day, and optional offset)
 * - DateDefWithOffset (object with date function and offset)
 * - DateFn (string like "EASTER_SUNDAY")
 * - String in "MM-DD" format
 */
typealias DateDef = JsonElement

/**
 * Date exception for specific conditions.
 *
 * Represents conditional overrides to a date definition based on
 * specific circumstances (e.g., when a date falls on a particular day of week).
 */
typealias DateDefException = JsonElement

/**
 * Collection of date exceptions.
 *
 * Can be either a single exception or an array of exceptions.
 */
typealias DateDefExceptions = JsonElement

/**
 * Color definition for a liturgical day.
 *
 * Can be one of:
 * - Color enum value (e.g., "RED", "WHITE")
 * - Array of Color values
 * - CompoundColor object with append/prepend
 */
typealias ColorsDef = JsonElement

/**
 * Common definition for a liturgical day.
 *
 * Can be one of:
 * - CommonDefinition enum value
 * - Array of CommonDefinition values
 */
typealias CommonsDef = JsonElement

/**
 * Reference to an entity in the catalog.
 *
 * Can be one of:
 * - String (entity ID)
 * - EntityOverride object
 */
typealias EntityRef = JsonElement

/**
 * Saint date representation.
 *
 * Can be one of:
 * - Number (year only)
 * - String in various formats
 * - SaintDate object describing a date range or century
 */
typealias SaintDateDef = JsonElement

/**
 * Saint count representation.
 *
 * Can be one of:
 * - Number (exact count)
 * - "many" string for undefined count
 */
typealias SaintCount = JsonElement

/**
 * Titles definition for a liturgical day.
 *
 * Can be one of:
 * - Array of Title enum values
 * - CompoundTitle object with append/prepend
 */
typealias TitlesDef = JsonElement
