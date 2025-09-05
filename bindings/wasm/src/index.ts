// Import the WASM module
import * as wasm from '../pkg/romcal.js';

// Initialize the WASM module
let wasmInitialized = false;

async function initWasm() {
  if (!wasmInitialized) {
    await (wasm as unknown as { __wasm: Promise<void> }).__wasm;
    wasmInitialized = true;
  }
}

// Configuration interface matching the WASM structure
export interface RomcalConfigInterface {
  calendar: string;
  locale: string;
  epiphanyOnSunday: boolean;
  corpusChristiOnSunday: boolean;
  ascensionOnSunday: boolean;
  easterCalculationType: string;
}

// Partial configuration interface for optional fields
export interface PartialRomcalConfigInterface {
  calendar?: string;
  locale?: string;
  epiphanyOnSunday?: boolean;
  corpusChristiOnSunday?: boolean;
  ascensionOnSunday?: boolean;
  easterCalculationType?: string;
}

// Romcal instance interface
export interface RomcalInstance {
  config: RomcalConfigInterface;
}

// Main romcal function with overloaded signatures
export async function romcal(): Promise<RomcalInstance>;
export async function romcal(calendar: string, locale: string): Promise<RomcalInstance>;
export async function romcal(config: RomcalConfigInterface): Promise<RomcalInstance>;
export async function romcal(config: PartialRomcalConfigInterface): Promise<RomcalInstance>;
export async function romcal(
  calendarOrConfig?: string | RomcalConfigInterface | PartialRomcalConfigInterface,
  locale?: string
): Promise<RomcalInstance> {
  await initWasm();

  let instance: wasm.Romcal;

  // Handle different parameter combinations
  if (typeof calendarOrConfig === 'object' && calendarOrConfig !== null) {
    const config = calendarOrConfig as RomcalConfigInterface | PartialRomcalConfigInterface;

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

    // Use the new Rust function that handles partial configs directly
    instance = wasm.romcal_with_config_object(partialConfig);
  } else if (typeof calendarOrConfig === 'string' && locale) {
    // Calendar and locale strings provided
    instance = wasm.romcal_with_config(calendarOrConfig, locale);
  } else {
    // No parameters - use default configuration
    instance = wasm.romcal();
  }

  return {
    config: {
      calendar: instance.config.calendar,
      locale: instance.config.locale,
      epiphanyOnSunday: instance.config.epiphany_on_sunday,
      corpusChristiOnSunday: instance.config.corpus_christi_on_sunday,
      ascensionOnSunday: instance.config.ascension_on_sunday,
      easterCalculationType: instance.config.easter_calculation_type,
    },
  };
}
