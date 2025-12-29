// Import the WASM module
import init, * as wasm from '../pkg/romcal_core_wasm.js'

// Re-export types from types/
export * from './types/index.js'

// Import specific types we need
import type {
  CalendarContext,
  CalendarDefinition,
  EasterCalculationType,
  LiturgicalDay,
  MassContext,
  Resources,
} from './types/index.js'

// Initialize the WASM module
let wasmInitialized = false
let initPromise: Promise<void> | null = null

/**
 * Detect if running in Node.js environment
 */
function isNode(): boolean {
  return typeof process !== 'undefined' && process.versions != null && process.versions.node != null
}

/**
 * Initialize WASM module (cross-platform: Node.js and browsers)
 */
async function initWasm(): Promise<void> {
  if (wasmInitialized) return

  // Ensure single initialization
  if (initPromise) {
    await initPromise
    return
  }

  initPromise = (async () => {
    if (isNode()) {
      // Node.js: read the .wasm file and pass bytes
      const { readFileSync } = await import('node:fs')
      const { fileURLToPath } = await import('node:url')
      const { dirname, join } = await import('node:path')

      const __filename = fileURLToPath(import.meta.url)
      const __dirname = dirname(__filename)
      const wasmPath = join(__dirname, '..', 'pkg', 'romcal_core_wasm_bg.wasm')
      const wasmBytes = readFileSync(wasmPath)
      await init({ module_or_path: wasmBytes })
    } else {
      // Browser: use default fetch-based loading
      await init()
    }
    wasmInitialized = true
  })()

  await initPromise
}

// ============================================================================
// Type Aliases (for convenience)
// ============================================================================

/**
 * The liturgical calendar: a map of dates (YYYY-MM-DD) to liturgical days.
 */
export type LiturgicalCalendar = Record<string, LiturgicalDay[]>

/**
 * The mass calendar: a map of civil dates (YYYY-MM-DD) to mass contexts.
 */
export type MassCalendar = Record<string, MassContext[]>

// ============================================================================
// Configuration Interfaces
// ============================================================================

/**
 * Full configuration interface matching the WASM structure.
 */
export interface RomcalConfigInterface {
  calendar: string
  locale: string
  epiphanyOnSunday: boolean
  corpusChristiOnSunday: boolean
  ascensionOnSunday: boolean
  easterCalculationType: EasterCalculationType
  context: CalendarContext
}

/**
 * Partial configuration interface for optional fields.
 */
export interface PartialRomcalConfigInterface {
  calendar?: string
  locale?: string
  epiphanyOnSunday?: boolean
  corpusChristiOnSunday?: boolean
  ascensionOnSunday?: boolean
  easterCalculationType?: EasterCalculationType
  context?: CalendarContext
  calendarDefinitions?: CalendarDefinition[]
  resources?: Resources[]
}

// ============================================================================
// Error Handling
// ============================================================================

/**
 * Error class for romcal-specific errors.
 */
export class RomcalError extends Error {
  constructor(message: string, options?: { cause?: unknown }) {
    super(message, options)
    this.name = 'RomcalError'
  }
}

// ============================================================================
// Romcal Instance
// ============================================================================

/**
 * A Romcal instance with calendar generation methods.
 */
export interface Romcal {
  /** The configuration of this Romcal instance */
  config: RomcalConfigInterface

  /**
   * Generate the complete liturgical calendar for a given liturgical year.
   *
   * @param year - The liturgical year (e.g., 2026 for liturgical year 2025-2026)
   * @returns A map of dates (YYYY-MM-DD) to liturgical days
   */
  generateLiturgicalCalendar(year: number): Promise<LiturgicalCalendar>

  /**
   * Generate a mass-centric view of the liturgical calendar for a given year.
   *
   * Evening masses (Easter Vigil, Previous Evening Mass) appear on the
   * previous civil day.
   *
   * @param year - The liturgical year (e.g., 2026 for liturgical year 2025-2026)
   * @returns A map of civil dates (YYYY-MM-DD) to mass contexts
   */
  generateMassCalendar(year: number): Promise<MassCalendar>

  /**
   * Get a liturgical date by its ID for a given year.
   *
   * @param id - Date ID (e.g., "easter_sunday", "christmas")
   * @param year - The year
   * @returns Date in YYYY-MM-DD format
   */
  getDate(id: string, year: number): Promise<string>
}

// ============================================================================
// Internal Helpers
// ============================================================================

/**
 * Creates a Romcal instance from a WASM Romcal instance.
 */
function createInstance(wasmInstance: wasm.Romcal): Romcal {
  return {
    config: {
      calendar: wasmInstance.config.calendar,
      locale: wasmInstance.config.locale,
      epiphanyOnSunday: wasmInstance.config.epiphany_on_sunday,
      corpusChristiOnSunday: wasmInstance.config.corpus_christi_on_sunday,
      ascensionOnSunday: wasmInstance.config.ascension_on_sunday,
      easterCalculationType: wasmInstance.config.easter_calculation_type as EasterCalculationType,
      context: wasmInstance.config.context as CalendarContext,
    },

    async generateLiturgicalCalendar(year: number): Promise<LiturgicalCalendar> {
      try {
        const json = wasmInstance.generateLiturgicalCalendar(year)
        return JSON.parse(json) as LiturgicalCalendar
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error)
        throw new RomcalError(message, { cause: error })
      }
    },

    async generateMassCalendar(year: number): Promise<MassCalendar> {
      try {
        const json = wasmInstance.generateMassCalendar(year)
        return JSON.parse(json) as MassCalendar
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error)
        throw new RomcalError(message, { cause: error })
      }
    },

    async getDate(id: string, year: number): Promise<string> {
      try {
        return wasmInstance.getDate(id, year)
      } catch (error) {
        const message = error instanceof Error ? error.message : String(error)
        throw new RomcalError(message, { cause: error })
      }
    },
  }
}

// ============================================================================
// Main API
// ============================================================================

/**
 * Create a new Romcal instance with default configuration.
 */
export async function createRomcal(): Promise<Romcal>

/**
 * Create a new Romcal instance with calendar and locale.
 */
export async function createRomcal(calendar: string, locale: string): Promise<Romcal>

/**
 * Create a new Romcal instance with full configuration.
 */
export async function createRomcal(config: RomcalConfigInterface): Promise<Romcal>

/**
 * Create a new Romcal instance with partial configuration.
 */
export async function createRomcal(config: PartialRomcalConfigInterface): Promise<Romcal>

/**
 * Create a new Romcal instance.
 *
 * @example
 * ```typescript
 * // Default configuration
 * const romcal = await createRomcal();
 *
 * // With calendar and locale
 * const romcal = await createRomcal('france', 'fr');
 *
 * // With partial configuration
 * const romcal = await createRomcal({
 *   calendar: 'france',
 *   locale: 'fr',
 *   epiphanyOnSunday: true,
 * });
 *
 * // Generate calendar
 * const calendar = await romcal.generateLiturgicalCalendar(2026);
 * ```
 */
export async function createRomcal(
  calendarOrConfig?: string | RomcalConfigInterface | PartialRomcalConfigInterface,
  locale?: string,
): Promise<Romcal> {
  await initWasm()

  let instance: wasm.Romcal

  // Handle different parameter combinations
  if (typeof calendarOrConfig === 'object' && calendarOrConfig !== null) {
    const config = calendarOrConfig as PartialRomcalConfigInterface

    // Create a partial config object and let Rust handle the defaults
    const partialConfig = new wasm.PartialRomcalConfig()

    if (config.calendar !== undefined) {
      partialConfig.set_calendar(config.calendar)
    }
    if (config.locale !== undefined) {
      partialConfig.set_locale(config.locale)
    }
    if (config.epiphanyOnSunday !== undefined) {
      partialConfig.set_epiphany_on_sunday(config.epiphanyOnSunday)
    }
    if (config.corpusChristiOnSunday !== undefined) {
      partialConfig.set_corpus_christi_on_sunday(config.corpusChristiOnSunday)
    }
    if (config.ascensionOnSunday !== undefined) {
      partialConfig.set_ascension_on_sunday(config.ascensionOnSunday)
    }
    if (config.easterCalculationType !== undefined) {
      partialConfig.set_easter_calculation_type(config.easterCalculationType)
    }
    if (config.context !== undefined) {
      partialConfig.set_context(config.context)
    }
    if (config.calendarDefinitions !== undefined) {
      partialConfig.set_calendar_definitions(JSON.stringify(config.calendarDefinitions))
    }
    if (config.resources !== undefined) {
      partialConfig.set_resources(JSON.stringify(config.resources))
    }

    instance = wasm.romcal_with_config_object(partialConfig)
  } else if (typeof calendarOrConfig === 'string' && locale) {
    // Calendar and locale strings provided
    instance = wasm.romcal_with_partial_config(
      calendarOrConfig,
      locale,
      undefined,
      undefined,
      undefined,
      undefined,
      undefined,
    )
  } else {
    // No parameters - use default configuration
    instance = wasm.romcal()
  }

  return createInstance(instance)
}
