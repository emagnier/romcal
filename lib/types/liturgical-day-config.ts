import { RomcalConfigOutput } from '@romcal/types/config';

export interface BaseLiturgicalDayConfig {
  /**
   * The calendar year to obtain.
   */
  readonly year: number;
}

export type LiturgicalDayConfigOutput = RomcalConfigOutput & BaseLiturgicalDayConfig;
