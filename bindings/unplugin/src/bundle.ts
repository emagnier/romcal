import type { RomcalPluginOptions, RomcalBundle } from './types.js';

/**
 * Generate an optimized bundle with only the required data.
 *
 * The hierarchy resolution (parent calendars, locale fallback) is handled
 * automatically by romcal's createBundle() method.
 *
 * @throws Error if calendarDefinitions or resources are not provided
 */
export async function generateBundle(options: RomcalPluginOptions): Promise<RomcalBundle> {
  const { calendarDefinitions, resources } = options;

  // Require explicit data - no magic loading
  if (!calendarDefinitions || calendarDefinitions.length === 0) {
    throw new Error(
      '[@romcal/unplugin] calendarDefinitions is required. ' +
        "Import the calendars you need from 'romcal/definitions'."
    );
  }

  if (!resources || resources.length === 0) {
    throw new Error(
      '[@romcal/unplugin] resources is required. ' + "Import the locales you need from 'romcal/resources'."
    );
  }

  const { createRomcal } = await import('romcal');

  // Create romcal instance with provided data
  const romcal = await createRomcal({
    ...options,
    calendarDefinitions,
    resources,
  });

  // createBundle() automatically handles:
  // - Calendar hierarchy resolution (france → europe → general_roman)
  // - Locale hierarchy resolution (fr-ca → fr → en)
  // - Filtering unused entities
  // - Property deduplication across locales
  return romcal.createBundle();
}
