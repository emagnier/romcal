import type { Color, Common, Period, Precedence, Rank, Season } from './_calendar-def';
import type { DateDef, DateDefException, TitlesDef } from './liturgical-day';
import type { MartyrologyDef } from './martyrology';

// Element à créer dans src/liturgical_day.rs
type LiturgicalDayId = string;

type CalendarId = string; // CalendarId from src/calendar_definition.rs

// Element à créer dans src/types/liturgical/seasons.rs
/**
 * Liturgical season information with localized name.
 */
type SeasonInfo = {
  key: Season; // Use Enum Season
  name: string;
};

// Element à créer dans src/types/liturgical/periods.rs
/**
 * Liturgical period information with localized name.
 */
type PeriodInfo = {
  key: Period; // Use Enum Period
  name: string;
};

// Element à créer dans src/types/liturgical/colors.rs
/**
 * Liturgical color information with localized name.
 */
type ColorInfo = {
  key: Color; // Use Enum Color
  name: string;
};

// Element à créer dans src/types/liturgical/commons.rs
/**
 * Liturgical common information with localized name.
 */
type CommonInfo = {
  key: Common; // Use Enum Common
  name: string;
};

/**
 * A three-year cycle for Sunday Mass readings (and some solemnities), designated by A, B, or C.
 * Each cycle begins on the First Sunday of Advent of the previous civil year and ends on Saturday
 * after the Christ the King Solemnity. The cycles follow each other in alphabetical order.
 * C year is always divisible by 3, A has remainder of 1, and B remainder of 2.
 */
export enum SundayCycle {
  YearA = 'YEAR_A',
  YearB = 'YEAR_B',
  YearC = 'YEAR_C',
}

// Element à créer dans src/types/liturgical/cycles.rs
/**
 * Sunday cycle information with localized name.
 */
type SundayCycleInfo = {
  key: SundayCycle; // Use Enum SundayCycle
  name: string;
};

/**
 * A two-year cycle for the weekday Mass readings (also called Cycle I and Cycle II).
 * Odd-numbered years are the Cycle I (year 1); even-numbered ones are the Cycle II (year 2).
 */
export enum WeekdayCycle {
  Year1 = 'YEAR_1',
  Year2 = 'YEAR_2',
}

// Element à créer dans src/types/liturgical/cycles.rs
/**
 * Weekday cycle information with localized name.
 */
type WeekdayCycleInfo = {
  key: WeekdayCycle; // Use Enum WeekdayCycle
  name: string;
};

/**
 * [GILH §133] The four-week cycle of the psalter is coordinated with the liturgical year in such a way that
 * on the First Sunday of Advent, the First Sunday in Ordinary Time, the First Sunday of Lent,
 * and Easter Sunday the cycle is always begun again with Week 1 (others being omitted when necessary).
 */
export enum PsalterWeekCycle {
  Week1 = 'WEEK_1',
  Week2 = 'WEEK_2',
  Week3 = 'WEEK_3',
  Week4 = 'WEEK_4',
}

// Element à créer dans src/types/liturgical/cycles.rs
/**
 * Psalter week cycle information with localized name.
 */
type PsalterWeekCycleInfo = {
  key: PsalterWeekCycle; // Use Enum PsalterWeekCycle
  name: string;
};

// Element à créer dans src/liturgical_day.rs
/**
 * A complete liturgical day definition with all its properties and metadata.
 * Represents a single day in the liturgical calendar with computed values and inheritance information.
 */
export type LiturgicalDay = {
  id: LiturgicalDayId;
  fullname: string;

  /**
   * The computed date of the liturgical day.
   */
  date: string; // in ISO 8601 format: YYYY-MM-DD

  /**
   * The date definition for this liturgical day.
   */
  dateDef: DateDef; // Use Struct DateDef

  /**
   * The date definition exceptions for this liturgical day.
   */
  dateExceptions: DateDefException[]; // Use Struct DateDefException

  /**
   * The liturgical precedence for this liturgical day.
   */
  precedence: Precedence; // Use Enum Precedence

  /**
   * The liturgical rank for this liturgical day.
   */
  rank: Rank; // Use Enum Rank

  /**
   * The localized liturgical rank for this liturgical day.
   */
  rankName: string;

  /**
   * Allows similar items with the same rank and same or lower precedence
   * to coexist without this liturgical day overwriting them.
   */
  allowSimilarRankItems: boolean;

  /**
   * Holy days of obligation are days on which the faithful are expected to attend Mass,
   * and engage in rest from work and recreation.
   */
  isHolyDayOfObligation: boolean;

  /**
   * Indicates if this liturgical day is optional within a specific liturgical calendar.
   */
  isOptional: boolean;

  /**
   * The liturgical seasons to which this liturgical day belongs.
   */
  seasons: SeasonInfo[]; // Use Enum Season

  /**
   * The liturgical periods to which this liturgical day belongs.
   */
  periods: PeriodInfo[]; // Use Enum Period

  /**
   * The common prayers, readings, and chants used for celebrating saints or
   * feasts that belong to a specific category, such as martyrs, virgins, pastors, or the Blessed
   * Virgin Mary.
   */
  commons: CommonInfo[]; // Use Enum Common

  /**
   * The liturgical colors for this liturgical day.
   */
  colors: ColorInfo[]; // Use Enum Color

  /**
   * The titles for this liturgical day.
   */
  titles: TitlesDef; // Use Enum Title

  /**
   * The entities (Saints, Blessed, or Places) linked to this liturgical day.
   */
  entities: MartyrologyDef[]; // Use Struct Entity

  /**
   * The week number of the current liturgical season.
   * Starts from `1`, except in the seasons of lent,
   * the week of Ash Wednesday to the next Saturday is counted as `0`.
   */
  weekOfSeason: number;

  /**
   * The day number within the current liturgical season.
   */
  dayOfSeason: number;

  /**
   * The day of the week for this liturgical day.
   * Returns a number from 0 (Sunday) to 6 (Saturday).
   */
  dayOfWeek: number; // Use Struct DayOfWeek

  /**
   * The nth occurrence of this day of the week within the current month.
   * For example, the 3rd Sunday of the month would have nthDayOfWeekInMonth = 3.
   */
  nthDayOfWeekInMonth: number;

  /**
   * The first day of the current liturgical season for this liturgical day.
   */
  startOfSeason: string; // in ISO 8601 format: YYYY-MM-DD

  /**
   * The last day of the current liturgical season for this liturgical day.
   */
  endOfSeason: string; // in ISO 8601 format: YYYY-MM-DD

  /**
   * The first day of the current liturgical year for this liturgical day,
   * i.e. the first Sunday of Advent.
   */
  startOfLiturgicalYear: string; // in ISO 8601 format: YYYY-MM-DD

  /**
   * The last day of the current liturgical year for this liturgical day,
   * i.e. the last Saturday of Ordinary Time, in the 34th week.
   */
  endOfLiturgicalYear: string; // in ISO 8601 format: YYYY-MM-DD

  /**
   * The Sunday cycle to which this liturgical day belongs.
   */
  sundayCycle: SundayCycleInfo;

  /**
   * The weekday cycle to which this liturgical day belongs.
   */
  weekdayCycle: WeekdayCycleInfo;

  /**
   * The psalter week cycle to which this liturgical day belongs.
   */
  psalterWeek: PsalterWeekCycleInfo;

  /**
   * The ID of the calendar where this liturgical day is defined.
   * Indicates the source calendar in the inheritance chain.
   */
  fromCalendarId: CalendarId;

  /**
   * Contains the differences between this liturgical day and its parent definitions.
   * Each element in the array represents the diff between two successive overrides in the inheritance chain.
   */
  parentOverrides: Partial<LiturgicalDay>[];
};
