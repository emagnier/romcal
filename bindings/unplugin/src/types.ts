import type { PartialRomcalConfigInterface, RomcalBundle, CalendarDefinition, Resources } from 'romcal';

/**
 * Options for the @romcal/unplugin plugin.
 *
 * Requires explicit calendarDefinitions and resources - no magic loading.
 * Import the calendars and locales you need from 'romcal/definitions' and 'romcal/resources'.
 *
 * @example
 * ```typescript
 * import { france, europe, generalRoman } from 'romcal/definitions';
 * import { fr, en } from 'romcal/resources';
 *
 * romcalPlugin({
 *   calendar: 'france',
 *   locale: 'fr',
 *   calendarDefinitions: [france, europe, generalRoman],
 *   resources: [fr, en],
 * })
 * ```
 */
export interface RomcalPluginOptions extends Omit<PartialRomcalConfigInterface, 'calendarDefinitions' | 'resources'> {
  /**
   * Calendar definitions to include.
   * Import from 'romcal/definitions'.
   */
  calendarDefinitions: CalendarDefinition[];

  /**
   * Locale resources to include.
   * Import from 'romcal/resources'.
   */
  resources: Resources[];

  /**
   * Virtual module ID for imports
   * @default 'virtual:romcal'
   */
  moduleId?: string;
}

export type { RomcalBundle, CalendarDefinition, Resources };
