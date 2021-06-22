import Temporale from './general-calendar/temporale';
import { GeneralRoman } from './general-calendar/general';
import {
  BaseRomcalConfig,
  RomcalConfig,
  RomcalConfigInput,
} from './models/config';
import { CalendarScope } from '../../constants/calendar-scope';
import {
  BaseCalendarDef,
  CalendarDef,
  LiturgicalCalendar,
} from './models/calendar-def';
import { Dates } from './utils/dates';

export default class Romcal {
  private readonly _config: RomcalConfig;
  private readonly _calendarsDef: InstanceType<BaseCalendarDef>[];
  private _computedCalendar?: LiturgicalCalendar;

  /**
   * Utility helpers to compute the date(s) of specific liturgical days or seasons.
   */
  static dates = Dates;

  constructor(config?: RomcalConfigInput) {
    this._config = new RomcalConfig(config);
    this._calendarsDef = [new GeneralRoman(this._config)];

    if (this._config.particularCalendar) {
      this._calendarsDef.push(
        new this._config.particularCalendar(this._config),
      );
    }

    this._calendarsDef.map((cal) => cal.updateConfig(config));
  }

  /**
   * Get the complete configuration, used to create and generate a calendar.
   */
  public get config(): BaseRomcalConfig {
    return this._config.toObject();
  }

  /**
   * Generate a liturgical calendar, within a Liturgical or Gregorian scope.
   */
  generate(): Promise<LiturgicalCalendar> {
    // Wrap the calendar computing process in a Promise.
    // Even if this method is called with async/await, this makes this method running in a microtask queue:
    // it does not run on the main thread, meaning other things can occur (click events, rendering, etc.).
    return new Promise((resolve, reject) => {
      // Check if calendar data is already computed and saved in a cache variable.
      // If this is the case, no need to compute it again.
      if (this._computedCalendar) {
        resolve(this._computedCalendar);
        return;
      }

      try {
        const data =
          this._config.scope === CalendarScope.Liturgical
            ? new Temporale(this._config).liturgicalYearBuilder()
            : new Temporale(this._config).gregorianYearBuilder();

        this._calendarsDef.forEach((cal) => cal.buildDates(data));
        this._computedCalendar = CalendarDef.generateCalendar(data);

        resolve(this._computedCalendar);
      } catch (e) {
        reject(e);
      }
    });
  }
}

export { CalendarScope, BaseRomcalConfig, LiturgicalCalendar };
