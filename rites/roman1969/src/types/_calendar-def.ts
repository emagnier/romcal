/**
 * Unique identifier for calendar definitions and liturgical day definitions.
 *
 * Calendar IDs follow the naming convention: `snake_case` with double underscores (`__`)
 * to separate hierarchy levels. Examples:
 * - `france` (country)
 * - `france__strasbourg` (diocese)
 * - `france__strasbourg__city` (city within diocese)
 * - `asia__russia` (country with region prefix)
 *
 * @example "france__strasbourg"
 * @example "europe__russia"
 */
export type CalendarId = string;

/**
 * Enumeration of calendar types that define the scope and hierarchy of a calendar.
 *
 * The calendar system follows a hierarchical structure:
 * Region → Country → Diocese → City/Parish
 *
 * These types are used to categorize calendars from the most general (GENERAL_ROMAN)
 * to the most specific (LOCAL_COMMUNITY).
 */
export enum CalendarType {
  /** The general Roman calendar, the base calendar for all other calendars */
  GENERAL_ROMAN = 'GENERAL_ROMAN',
  /** National calendar for a specific country (e.g., France, Germany) */
  COUNTRY = 'COUNTRY',
  /** Regional calendar covering multiple countries or a large geographical area (e.g., Europe, Americas) */
  REGION = 'REGION',
  /** Diocesan calendar for a specific diocese (e.g., Strasbourg, Paris) */
  DIOCESE = 'DIOCESE',
  /** City-level calendar for a specific city within a diocese */
  CITY = 'CITY',
  /** Parish-level calendar for a specific parish */
  PARISH = 'PARISH',
  /** Calendar for a general religious community (e.g., Benedictines worldwide) */
  GENERAL_COMMUNITY = 'GENERAL_COMMUNITY',
  /** Calendar for a regional religious community */
  REGIONAL_COMMUNITY = 'REGIONAL_COMMUNITY',
  /** Calendar for a local religious community */
  LOCAL_COMMUNITY = 'LOCAL_COMMUNITY',
  /** Other types of calendars not covered by the above categories */
  OTHER = 'OTHER',
}

/**
 * Enumeration of calendar jurisdictions that define the authority governing the calendar.
 *
 * This determines whether the calendar is governed by Church authority or civil authority.
 */
export enum CalendarJurisdiction {
  /** Calendar governed by ecclesiastical (Church) authority */
  ECCLESIASTICAL = 'ECCLESIASTICAL',
  /** Calendar governed by civil (governmental) authority */
  CIVIL = 'CIVIL',
}

/**
 * Metadata describing the basic properties of a calendar definition.
 *
 * This information helps categorize and understand the scope of a calendar,
 * and is used in the JSON calendar definitions to specify the entity type.
 *
 * @example
 * ```json
 * {
 *   "type": "DIOCESE",
 *   "jurisdiction": "ECCLESIASTICAL"
 * }
 * ```
 */
export interface CalendarMetadata {
  /** The type of calendar, indicating its scope and hierarchy level */
  type: CalendarType;
  /** The jurisdiction that governs this calendar */
  jurisdiction: CalendarJurisdiction;
}

/**
 * Interface defining the parent calendar relationships for a calendar definition.
 *
 * This allows calendars to inherit from other calendars in a hierarchical structure.
 * The inheritance system enables calendars to build upon more general calendars,
 * adding specific liturgical days while inheriting common definitions.
 *
 * @example
 * ```json
 * {
 *   "parentCalendarIds": ["Europe", "France"]
 * }
 * ```
 */
export interface CalendarDefParentCalendarIds {
  /** Array of parent calendar IDs that this calendar inherits from.
   * The order is important as it determines inheritance priority.
   * Calendars are processed from left to right, with later calendars
   * potentially overriding definitions from earlier ones. */
  parentCalendarIds: CalendarId[];
}

/**
 * Core interface defining a calendar definition.
 *
 * A calendar definition contains all the necessary information to create a liturgical calendar,
 * including its metadata, parent relationships, and liturgical day definitions.
 *
 * This interface is used to define the structure of JSON calendar files in the project.
 *
 * @example
 * ```json
 * {
 *   "id": "france__strasbourg",
 *   "metadata": {
 *     "type": "DIOCESE",
 *     "jurisdiction": "ECCLESIASTICAL"
 *   },
 *   "parentCalendarIds": ["Europe", "France"],
 *   "dayDefinitions": {
 *     "arbogast_of_strasbourg_bishop": {
 *       "precedence": "PROPER_FEAST__PRINCIPAL_PATRON_OF_A_DIOCESE_8A",
 *       "dateDef": { "month": 7, "date": 21 }
 *     }
 *   }
 * }
 * ```
 */
export interface CalendarDef {
  /** The JSON schema for the calendar definition */
  $schema: string;

  /** Unique identifier for this calendar definition.
   * Follows the naming convention: `snake_case` with double underscores for hierarchy. */
  id: CalendarId;
  /** Metadata describing the type and jurisdiction of this calendar */
  metadata: CalendarMetadata;
  /** Optional array of parent calendar IDs that this calendar inherits from.
   * If not provided, this calendar is considered a root calendar (e.g., GENERAL_ROMAN). */
  parentCalendarIds?: CalendarId[];
  /** Collection of liturgical day definitions specific to this calendar.
   * These definitions will be merged with those from parent calendars.
   * Each key is a liturgical day ID, and each value contains the day's configuration. */
  // dayDefinitions: {};
  dayDefinitions: DayDefinitions;
}

/**
 * Unique identifier for a liturgical day definition.
 */
type DayId = string;

/**
 * List of the precedence of the Liturgical Days (UNLY #59)
 * Order is important: higher precedence type first, lower precedence type at the end.
 * @readonly
 */
export enum Precedence {
  // Note: there is a limit of 63 chars per name because of string literal typing & Pascal to Snake case

  /**
   * 1 - The Paschal Triduum of the Passion and Resurrection of the Lord.
   */
  Triduum_1 = 'TRIDUUM_1',

  /**
   * 2 - The Nativity of the Lord, the Epiphany, the Ascension, or Pentecost.
   */
  ProperOfTimeSolemnity_2 = 'PROPER_OF_TIME_SOLEMNITY_2',
  /**
   * 2 - A Sunday of Advent, Lent, or Easter.
   */
  PrivilegedSunday_2 = 'PRIVILEGED_SUNDAY_2',
  /**
   * 2 - Ash Wednesday.
   */
  AshWednesday_2 = 'ASH_WEDNESDAY_2',
  /**
   * 2 - A weekday of Holy Week from Monday up to and including Thursday.
   */
  WeekdayOfHolyWeek_2 = 'WEEKDAY_OF_HOLY_WEEK_2',
  /**
   * 2 - A day within the Octave of Easter.
   */
  WeekdayOfEasterOctave_2 = 'WEEKDAY_OF_EASTER_OCTAVE_2',

  /**
   * 3 - A Solemnity inscribed in the General Calendar, whether of the Lord, of the Blessed Virgin Mary, or of a Saint.
   */
  GeneralSolemnity_3 = 'GENERAL_SOLEMNITY_3',

  /**
   * 3 - The Commemoration of All the Faithful Departed.
   */
  CommemorationOfAllTheFaithfulDeparted_3 = 'COMMEMORATION_OF_ALL_THE_FAITHFUL_DEPARTED_3',

  /**
   * 4 - Proper Solemnity.
   * */

  /**
   * 4a - A proper Solemnity of the principal Patron of the place, city, or state.
   */
  ProperSolemnity_PrincipalPatron_4a = 'PROPER_SOLEMNITY__PRINCIPAL_PATRON_4A',
  /**
   * 4b - The Solemnity of the dedication and of the anniversary of the dedication of the own church.
   */
  ProperSolemnity_DedicationOfTheOwnChurch_4b = 'PROPER_SOLEMNITY__DEDICATION_OF_THE_OWN_CHURCH_4B',
  /**
   * 4c - The solemnity of the title of the own church.
   */
  ProperSolemnity_TitleOfTheOwnChurch_4c = 'PROPER_SOLEMNITY__TITLE_OF_THE_OWN_CHURCH_4C',
  /**
   *  4d - A Solemnity either of the Title
   *  or of the Founder
   *  or of the principal Patron of an Order or Congregation.
   */
  ProperSolemnity_TitleOrFounderOrPrimaryPatronOfAReligiousOrg_4d = 'PROPER_SOLEMNITY__TITLE_OR_FOUNDER_OR_PRIMARY_PATRON_OF_A_RELIGIOUS_ORG_4D',

  /**
   * 5 - A Feast of the Lord inscribed in the General Calendar.
   */
  GeneralLordFeast_5 = 'GENERAL_LORD_FEAST_5',

  /**
   * 6 - A Sunday of Christmas Time or a Sunday in Ordinary Time.
   */
  UnprivilegedSunday_6 = 'UNPRIVILEGED_SUNDAY_6',

  /**
   * 7 - A Feast of the Blessed Virgin Mary or of a Saint in the General Calendar.
   */
  GeneralFeast_7 = 'GENERAL_FEAST_7',

  /**
   * 8 - Proper Feast
   */

  /**
   * 8a - The Proper Feast of the principal Patron of the diocese.
   */
  ProperFeast_PrincipalPatronOfADiocese_8a = 'PROPER_FEAST__PRINCIPAL_PATRON_OF_A_DIOCESE_8A',

  /**
   * 8b - The Proper Feast of the anniversary of the dedication of the cathedral church
   */
  ProperFeast_DedicationOfTheCathedralChurch_8b = 'PROPER_FEAST__DEDICATION_OF_THE_CATHEDRAL_CHURCH_8B',

  /**
   * 8c - The Proper Feast of the principal Patron of a region or province, or a country, or of a wider territory.
   */
  ProperFeast_PrincipalPatronOfARegion_8c = 'PROPER_FEAST__PRINCIPAL_PATRON_OF_A_REGION_8C',

  /**
   * 8d - The Proper Feast of the Title, Founder, or principal Patron of an Order or Congregation
   * and of a religious province, without prejudice to the prescriptions given under no. 4.
   */
  ProperFeast_TitleOrFounderOrPrimaryPatronOfAReligiousOrg_8d = 'PROPER_SOLEMNITY__TITLE_OR_FOUNDER_OR_PRIMARY_PATRON_OF_A_RELIGIOUS_ORG_8D',

  /**
   * 8e - Other Feast, proper to an individual church.
   */
  ProperFeast_ToAnIndividualChurch_8e = 'PROPER_FEAST__TO_AN_INDIVIDUAL_CHURCH_8E',

  /**
   * 8f - Other Proper Feast
   * inscribed in the Calendar of each diocese or Order or Congregation.
   */
  ProperFeast_8f = 'PROPER_FEAST_8F',

  /**
   * 9 - Privileged Weekday
   *
   * - A Weekday of Advent from December 17 up to and including December 24.
   * - A Day within the Octave of Christmas.
   * - A Weekday of Lent.
   */
  PrivilegedWeekday_9 = 'PRIVILEGED_WEEKDAY_9',

  /**
   * 10 - Obligatory Memorials in the General Calendar.
   */
  GeneralMemorial_10 = 'GENERAL_MEMORIAL_10',

  /**
   * 11 - Proper Obligatory Memorial.
   */

  /**
   * 11a - Proper Obligatory Memorial of a secondary Patron
   * of the place, diocese, region, or religious province.
   */
  ProperMemorial_SecondPatron_11a = 'PROPER_MEMORIAL__SECOND_PATRON_11A',

  /**
   * 11b - Other Proper Obligatory Memorial
   * inscribed in the Calendar of each diocese, or Order or congregation.
   */
  ProperMemorial_11b = 'PROPER_MEMORIAL_11B',

  /**
   * 13 - Weekday
   *
   * A Weekday of Advent up to and including December 16.
   * A Weekday of Christmas Time from January 2 until the Saturday after the Epiphany.
   * A Weekday of the Easter Time from Monday after the Octave of Easter up to and including the The Saturday before Pentecost.
   * A Weekday in Ordinary Time.
   */
  Weekday_13 = 'WEEKDAY_13',

  /**
   * 12 - Optional Memorial
   *
   * Optional Memorial, which, however, may be celebrated, in the special manner described in the
   * General Instruction of the Roman Missal and of the Liturgy of the Hours, even on the days listed in no. 9.
   *
   * **Note:**
   * Optional Memorials (12) are placed after the weekday (13):
   * - For computing performance reasons (sorting performance).
   * - Because as long as they are not celebrated, the Weekday still takes precedence.
   *   The Optional Memorials remains outputted for convenient reasons or any custom usage of romcal generated data.
   *   The output or Optional Memorials can be disabled with the `strictMode: true`.
   */
  OptionalMemorial_12 = 'OPTIONAL_MEMORIAL_12',
}

/**
 * Collection of liturgical day definitions specific to this calendar.
 */
export type DayDefinitions = Record<DayId, DayDefinition>;

/**
 * A liturgical day definition.
 */
export interface DayDefinition {
  // id: DayId;
  // dateDef: DateDef;
  precedence: Precedence;
}
