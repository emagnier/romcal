import { i18n } from 'i18next';
import { CalendarScope } from '../constants/calendar-scope';
import { RomcalBundleObject } from './bundle';
import { CalendarDefInstance } from './calendar-def';

/**
 * The configuration object that is passed either to the [[Calendar.calendarFor]]
 * methods to retrieve an array of [[DateItems]].
 */
export interface BaseRomcalConfig {
  /**
   * The localized calendar bundle object.
   */
  readonly localizedCalendar?: RomcalBundleObject;

  /**
   * If `false`, fixes Epiphany on January 6th. Usually, Epiphany will be set to a
   * Sunday between the 2nd - 8th Jan based on an internal calculation.
   *
   * Defaults to `true`.
   */
  readonly epiphanyOnSunday?: boolean;

  /**
   * Determines if Corpus Christi should be celebrated on Sunday (63 days after Easter)
   * or on Thursday of the 7th week of Easter (60 days after Easter).
   * (Corpus Christi is celebrated on Sunday by default).
   *
   * Defaults to `true`
   */
  readonly corpusChristiOnSunday?: boolean;

  /**
   * If true, Ascension is moved to the 7th Sunday of Easter)
   *
   * (defaults to false)
   */
  readonly ascensionOnSunday?: boolean;

  /**
   * The calendar scope to query for.
   *
   * The scope can be specified either as:
   * 1. **gregorian**: Which is the civil year for the majority of countries (January 1 to December 31); or
   * 2. **liturgical**: Religious calendar year (1st Sunday of Advent of the current year to the Saturday before the 1st Sunday of Advent in the next year).
   */
  readonly scope?: CalendarScope;

  /**
   * Enable logging output.
   * Logs are newline delimited JSON (NDJSON),
   * a convenient format for production usage and long-term storage.
   */
  readonly verbose?: boolean;

  /**
   * Prettify logs printed in the console, for a better experience in development environnements
   * (instead of output them in NDJSON format).
   */
  readonly prettyPrint?: boolean;

  /**
   * All calendar definitions
   */
  readonly calendarsDef: InstanceType<CalendarDefInstance>[];

  /**
   * The locale key
   */
  readonly localeKey: string;

  /**
   * The calendar name
   */
  readonly calendarName: string;
}

export interface IRoncalConfig extends BaseRomcalConfig {
  readonly i18next: i18n;
}

/**
 * A modified variant of [[RomcalConfig]] specifically for the [[Config]] class constructor
 * where all properties are **required**.
 */
export type RomcalConfigInput = Omit<Partial<BaseRomcalConfig>, 'localeKey' | 'calendarName'>;

/**
 * Output object of the romcal config.
 */
export type RomcalConfigOutput = Required<
  Omit<BaseRomcalConfig, 'localizedCalendar' | 'calendarsDef'>
>;
