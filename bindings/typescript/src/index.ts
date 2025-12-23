// Import the WASM module
import init, * as wasm from '../pkg/romcal_core_wasm.js';

// Re-export types from types.ts
export * from './types.js';

// Import specific types we need
import type {
  AllTypes,
  ColorInfo,
  CommonInfo,
  Entity,
  MassTime,
  PeriodInfo,
  Precedence,
  PsalterWeekCycle,
  Rank,
  Season,
  SundayCycle,
  TitlesDef,
  WeekdayCycle,
} from './types.js';

// Initialize the WASM module
let wasmInitialized = false;
let initPromise: Promise<void> | null = null;

/**
 * Detect if running in Node.js environment
 */
function isNode(): boolean {
  return typeof process !== 'undefined' && process.versions != null && process.versions.node != null;
}

/**
 * Initialize WASM module (cross-platform: Node.js and browsers)
 */
async function initWasm(): Promise<void> {
  if (wasmInitialized) return;

  // Ensure single initialization
  if (initPromise) {
    await initPromise;
    return;
  }

  initPromise = (async () => {
    if (isNode()) {
      // Node.js: read the .wasm file and pass bytes
      const { readFileSync } = await import('node:fs');
      const { fileURLToPath } = await import('node:url');
      const { dirname, join } = await import('node:path');

      const __filename = fileURLToPath(import.meta.url);
      const __dirname = dirname(__filename);
      const wasmPath = join(__dirname, '..', 'pkg', 'romcal_core_wasm_bg.wasm');
      const wasmBytes = readFileSync(wasmPath);
      await init({ module_or_path: wasmBytes });
    } else {
      // Browser: use default fetch-based loading
      await init();
    }
    wasmInitialized = true;
  })();

  await initPromise;
}

// ============================================================================
// Type Definitions
// ============================================================================

/**
 * A single day in the liturgical calendar.
 * Uses AllTypes from the generated types as it covers all liturgical day fields.
 */
export type LiturgicalDay = AllTypes;

/**
 * The liturgical calendar: a map of dates (YYYY-MM-DD) to liturgical days.
 */
export type LiturgicalCalendar = Record<string, LiturgicalDay[]>;

/**
 * Summary of a celebration for optional celebrations list.
 */
export interface CelebrationSummary {
  id: string;
  fullname: string;
  precedence: Precedence;
  rank: Rank;
  rank_name: string;
  colors: ColorInfo[];
  commons: CommonInfo[];
  entities: Entity[];
  titles: TitlesDef;
  is_holy_day_of_obligation: boolean;
  is_optional: boolean;
  from_calendar_id: string;
}

/**
 * A mass context representing a single mass with its full liturgical context.
 */
export interface MassContext {
  // Mass identification
  mass_time: MassTime;
  mass_time_name: string;
  civil_date: string;
  liturgical_date: string;

  // Day-level context
  season?: Season | null;
  season_name?: string | null;
  sunday_cycle: SundayCycle;
  sunday_cycle_name: string;
  weekday_cycle: WeekdayCycle;
  weekday_cycle_name: string;
  psalter_week: PsalterWeekCycle;
  psalter_week_name: string;
  week_of_season?: number | null;
  day_of_season?: number | null;
  day_of_week: number;
  periods: PeriodInfo[];
  start_of_season?: string | null;
  end_of_season?: string | null;
  start_of_liturgical_year: string;
  end_of_liturgical_year: string;

  // Primary celebration
  id: string;
  fullname: string;
  precedence: Precedence;
  rank: Rank;
  rank_name: string;
  colors: ColorInfo[];
  commons: CommonInfo[];
  entities: Entity[];
  titles: TitlesDef;
  is_holy_day_of_obligation: boolean;
  is_optional: boolean;
  from_calendar_id: string;

  // Alternative celebrations
  optional_celebrations: CelebrationSummary[];
}

/**
 * The mass calendar: a map of civil dates (YYYY-MM-DD) to mass contexts.
 */
export type MassCalendar = Record<string, MassContext[]>;

/**
 * Calendar definition type (for custom calendars).
 */
export interface CalendarDefinition {
  id: string;
  metadata?: {
    type?: string;
    jurisdiction?: string;
    [key: string]: unknown;
  } | null;
  parent_calendar_ids?: string[];
  days_definitions?: Record<string, unknown>;
  particular_config?: unknown | null;
}

/**
 * Resources definition type (for custom locales).
 */
export interface ResourcesDefinition {
  locale: string;
  metadata?: unknown | null;
  entities?: unknown | null;
}

// ============================================================================
// Configuration Interfaces
// ============================================================================

/**
 * Full configuration interface matching the WASM structure.
 */
export interface RomcalConfigInterface {
  calendar: string;
  locale: string;
  epiphanyOnSunday: boolean;
  corpusChristiOnSunday: boolean;
  ascensionOnSunday: boolean;
  easterCalculationType: string;
  context: string;
}

/**
 * Partial configuration interface for optional fields.
 */
export interface PartialRomcalConfigInterface {
  calendar?: string;
  locale?: string;
  epiphanyOnSunday?: boolean;
  corpusChristiOnSunday?: boolean;
  ascensionOnSunday?: boolean;
  easterCalculationType?: string;
  context?: string;
  calendarDefinitions?: CalendarDefinition[];
  resources?: ResourcesDefinition[];
}

// ============================================================================
// Error Handling
// ============================================================================

/**
 * Error class for romcal-specific errors.
 */
export class RomcalError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'RomcalError';
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
  config: RomcalConfigInterface;

  /**
   * Generate the complete liturgical calendar for a given liturgical year.
   *
   * @param year - The liturgical year (e.g., 2026 for liturgical year 2025-2026)
   * @returns A map of dates (YYYY-MM-DD) to liturgical days
   */
  generateLiturgicalCalendar(year: number): Promise<LiturgicalCalendar>;

  /**
   * Generate a mass-centric view of the liturgical calendar for a given year.
   *
   * Evening masses (Easter Vigil, Previous Evening Mass) appear on the
   * previous civil day.
   *
   * @param year - The liturgical year (e.g., 2026 for liturgical year 2025-2026)
   * @returns A map of civil dates (YYYY-MM-DD) to mass contexts
   */
  generateMassCalendar(year: number): Promise<MassCalendar>;
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
      easterCalculationType: wasmInstance.config.easter_calculation_type,
      context: wasmInstance.config.context,
    },

    async generateLiturgicalCalendar(year: number): Promise<LiturgicalCalendar> {
      try {
        const json = wasmInstance.generateLiturgicalCalendar(year);
        return JSON.parse(json) as LiturgicalCalendar;
      } catch (error) {
        throw new RomcalError(`Failed to generate liturgical calendar for year ${year}: ${error}`);
      }
    },

    async generateMassCalendar(year: number): Promise<MassCalendar> {
      try {
        const json = wasmInstance.generateMassCalendar(year);
        return JSON.parse(json) as MassCalendar;
      } catch (error) {
        throw new RomcalError(`Failed to generate mass calendar for year ${year}: ${error}`);
      }
    },
  };
}

// ============================================================================
// Main API
// ============================================================================

/**
 * Create a new Romcal instance with default configuration.
 */
export async function createRomcal(): Promise<Romcal>;

/**
 * Create a new Romcal instance with calendar and locale.
 */
export async function createRomcal(calendar: string, locale: string): Promise<Romcal>;

/**
 * Create a new Romcal instance with full configuration.
 */
export async function createRomcal(config: RomcalConfigInterface): Promise<Romcal>;

/**
 * Create a new Romcal instance with partial configuration.
 */
export async function createRomcal(config: PartialRomcalConfigInterface): Promise<Romcal>;

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
  locale?: string
): Promise<Romcal> {
  await initWasm();

  let instance: wasm.Romcal;

  // Handle different parameter combinations
  if (typeof calendarOrConfig === 'object' && calendarOrConfig !== null) {
    const config = calendarOrConfig as PartialRomcalConfigInterface;

    // Create a partial config object and let Rust handle the defaults
    const partialConfig = new wasm.PartialRomcalConfig();

    if (config.calendar !== undefined) {
      partialConfig.set_calendar(config.calendar);
    }
    if (config.locale !== undefined) {
      partialConfig.set_locale(config.locale);
    }
    if (config.epiphanyOnSunday !== undefined) {
      partialConfig.set_epiphany_on_sunday(config.epiphanyOnSunday);
    }
    if (config.corpusChristiOnSunday !== undefined) {
      partialConfig.set_corpus_christi_on_sunday(config.corpusChristiOnSunday);
    }
    if (config.ascensionOnSunday !== undefined) {
      partialConfig.set_ascension_on_sunday(config.ascensionOnSunday);
    }
    if (config.easterCalculationType !== undefined) {
      partialConfig.set_easter_calculation_type(config.easterCalculationType);
    }
    if (config.context !== undefined) {
      partialConfig.set_context(config.context);
    }
    if (config.calendarDefinitions !== undefined) {
      partialConfig.set_calendar_definitions(JSON.stringify(config.calendarDefinitions));
    }
    if (config.resources !== undefined) {
      partialConfig.set_resources(JSON.stringify(config.resources));
    }

    instance = wasm.romcal_with_config_object(partialConfig);
  } else if (typeof calendarOrConfig === 'string' && locale) {
    // Calendar and locale strings provided
    instance = wasm.romcal_with_partial_config(
      calendarOrConfig,
      locale,
      undefined,
      undefined,
      undefined,
      undefined,
      undefined
    );
  } else {
    // No parameters - use default configuration
    instance = wasm.romcal();
  }

  return createInstance(instance);
}
