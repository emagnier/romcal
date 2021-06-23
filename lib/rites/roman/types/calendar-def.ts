import { LiturgicalColors } from '@roman-rite/constants/colors';
import { ProperCycles } from '@roman-rite/constants/cycles';
import { LiturgicalDefBuiltData } from '@roman-rite/general-calendar/temporale';
import { RomcalConfig } from '@roman-rite/models/config';
import LiturgicalDay from '@roman-rite/models/liturgical-day';
import { RomcalConfigInput } from '@roman-rite/types/config';
import { PatronTitles, Titles } from '@romcal/constants/martyrology-metadata';
import { SaintCount } from '@romcal/types/martyrology';
import { Dayjs } from 'dayjs';

export type LiturgicalCalendar = Record<string, LiturgicalDay[]>;

export type ParticularConfig = Partial<
  Pick<RomcalConfig, 'ascensionOnSunday' | 'epiphanyOnSunday' | 'corpusChristiOnSunday'>
>;

export type TitlesDef =
  | (Titles | PatronTitles)[]
  | ((titles: (Titles | PatronTitles)[]) => (Titles | PatronTitles)[]);

export type MartyrologyItemPointer = (string | MartyrologyItemRedefined)[];
export type MartyrologyItemRedefined = {
  key: string;
  titles?: TitlesDef;
  hideTitles?: boolean;
  count?: SaintCount;
};

/**
 * Date definition, used in the [CalendarDef] class
 */
export type DateDefinitions = Record<string, DateDefInput>;
export type DateDefInput = Partial<Pick<LiturgicalDay, 'precedence'>> & {
  /**
   * Specify a custom locale key for this date definition, in this calendar.
   */
  customLocaleKey?: string;

  /**
   * Date as a String (in the 'M-D' format), or as a Dayjs object.
   */
  date?: string | ((year: number) => Dayjs);

  /**
   * Holy days of obligation are days on which the faithful are expected to attend Mass,
   * and engage in rest from work and recreation.
   */
  isHolyDayOfObligation?: boolean | ((year: number) => boolean);

  /**
   * Link one or multiple Saints, Blessed, or any other celebrations from the Martyrology catalog.
   */
  martyrology?: MartyrologyItemPointer;

  /**
   * Replace (using an Array) or extend (using a Function) the titles of each Saints linked to this date definition.
   */
  titles?: TitlesDef;

  /**
   * The liturgical colors of the liturgical day.
   */
  liturgicalColors?: LiturgicalColors | LiturgicalColors[];

  /**
   * The proper cycle in which the liturgical day is part.
   */
  properCycle?: ProperCycles;

  /**
   * If this liturgical day must be removed from this calendar and from all those it inherits,
   * on the final calendar generated by romcal.
   */
  drop?: boolean;
};

/**
 * Root interface for Constructor Interfaces. This is a workaround for
 * TypeScript's lack of "static" methods for classes.
 *
 * Based on StackOverflow user chris's solution. See
 * - https://stackoverflow.com/a/43484801/1037165
 * - https://pastebin.com/v8Rf6g6Y
 *
 * @interface IConstructor
 * @template InstanceInterface
 */
interface IConstructor<InstanceInterface> {
  /**
   * Explicitly typed constructor is required, so make an extremely permissive
   * declaration that can be refined in subclasses.
   *
   * @constructor
   */
  new (config: RomcalConfig): InstanceInterface;
}

/**
 * Base [CalendarDef] interface
 */
export interface ICalendarDef {
  inheritFrom?: BaseCalendarDef | null;
  inheritFromInstance?: InstanceType<BaseCalendarDef>;
  particularConfig?: ParticularConfig;
  definitions: DateDefinitions;
  updateConfig: (config?: RomcalConfigInput) => void;
  buildDates: (builtData: LiturgicalDefBuiltData) => LiturgicalDefBuiltData;
}

/**
 * Create a Constructor Interface by extending IConstructor and
 * specifying [ICalendarDef].
 * This allows to define static methods from [ICalendarDef]
 */
interface StaticCalendarComputing<T extends ICalendarDef> extends IConstructor<T> {
  generateCalendar: (builtData: LiturgicalDefBuiltData) => LiturgicalCalendar;
}

export type BaseCalendarDef = StaticCalendarComputing<ICalendarDef>;
