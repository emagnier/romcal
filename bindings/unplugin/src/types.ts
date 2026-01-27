import type { PartialRomcalConfigInterface, RomcalBundle } from 'romcal';

/**
 * Options for the @romcal/unplugin plugin.
 * Extends PartialRomcalConfigInterface to allow custom calendar definitions and resources.
 */
export interface RomcalPluginOptions extends PartialRomcalConfigInterface {
  /**
   * Virtual module ID for imports
   * @default 'virtual:romcal'
   */
  moduleId?: string;
}

export type { RomcalBundle };
