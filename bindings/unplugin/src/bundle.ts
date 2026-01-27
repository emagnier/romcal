import type { CalendarDefinition, Resources } from 'romcal';
import type { RomcalPluginOptions, RomcalBundle } from './types.js';

/**
 * Generate an optimized bundle with only the required data.
 *
 * The hierarchy resolution (parent calendars, locale fallback) is handled
 * automatically by romcal's createBundle() method. This function loads all
 * available data, merges with custom data if provided, and lets createBundle()
 * filter what's necessary.
 */
export async function generateBundle(options: RomcalPluginOptions): Promise<RomcalBundle> {
  const { createRomcal } = await import('romcal');

  // Load embedded calendar definitions (filter out default export)
  const definitionsModule = await import('romcal/definitions');
  const embeddedDefinitions = Object.entries(definitionsModule)
    .filter(([key]) => key !== 'default')
    .map(([, value]) => value as CalendarDefinition);

  // Load embedded resources (filter out default export)
  const resourcesModule = await import('romcal/resources');
  const embeddedResources = Object.entries(resourcesModule)
    .filter(([key]) => key !== 'default')
    .map(([, value]) => value as Resources);

  // Merge embedded data with custom data (if provided)
  const calendarDefinitions = [...embeddedDefinitions, ...(options.calendarDefinitions ?? [])];
  const resources = [...embeddedResources, ...(options.resources ?? [])];

  // Create romcal instance with merged data
  // createBundle() handles the filtering based on calendar + locale
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
