type Without<T, U> = { [P in Exclude<keyof T, keyof U>]?: never };
export type XOR<T, U> = T | U extends object ? (Without<T, U> & U) | (Without<U, T> & T) : T | U;

// biome-ignore lint/suspicious/noExplicitAny: <explanation>
export type AllXOR<T extends any[]> = T extends [infer Only]
  ? Only
  : T extends [infer A, infer B, ...infer Rest]
    ? AllXOR<[XOR<A, B>, ...Rest]>
    : never;

export type CalendarId = string;
export type DayId = string;
export type LocaleId = string;
export type ResourceId = string;

export enum CalendarType {
  GENERAL_ROMAN = 'GENERAL_ROMAN',
  REGION = 'REGION',
  COUNTRY = 'COUNTRY',
  ARCHDIOCESE = 'ARCHDIOCESE',
  DIOCESE = 'DIOCESE',
  CITY = 'CITY',
  PARISH = 'PARISH',
  GENERAL_COMMUNITY = 'GENERAL_COMMUNITY',
  REGIONAL_COMMUNITY = 'REGIONAL_COMMUNITY',
  LOCAL_COMMUNITY = 'LOCAL_COMMUNITY',
  OTHER = 'OTHER',
}

export enum CalendarJurisdiction {
  ECCLESIASTICAL = 'ECCLESIASTICAL',
  CIVIL = 'CIVIL',
}

export type CalendarMetadata = {
  type: CalendarType;
  jurisdiction: CalendarJurisdiction;
};

export enum EasterCalculationType {
  GREGORIAN = 'GREGORIAN',
  JULIAN = 'JULIAN',
}

/**
 * Configuration options specific to this calendar.
 * These settings can override or extend the default Romcal configuration or any parent calendar
 * configuration.
 */
export type ParticularConfig = {
  ascensionOnSunday: boolean;
  epiphanyOnSunday: boolean;
  corpusChristiOnSunday: boolean;
  easterCalculationType: EasterCalculationType;
};

export type CalendarDefinition = {
  id: CalendarId;

  metadata: CalendarMetadata;

  particularConfig?: ParticularConfig;

  parentCalendarIds: CalendarId[];

  daysDefinitions: DayDefinition[];
};

/**
 * The liturgical day date definition
 */
export type DateDef = AllXOR<
  [
    DateDefMonthDate,
    DateDefDateFnAddDay,
    DateDefDateFnSubtractDay,
    DateDefMonthDowNthWeekInMonth,
    DateDefMonthLastDowInMonth,
  ]
>;

export type MonthIndex = 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12;
export type DayOfWeek = 0 | 1 | 2 | 3 | 4 | 5 | 6;

export type DateFn = 'easter_sunday' | 'epiphany_sunday' | 'corpus_christi_sunday' | 'easter_monday';

export type DateDefMonthDate = {
  /**
   * The month of this liturgical day.
   */
  month: MonthIndex;

  /**
   * The date of this liturgical day.
   */
  date: number;

  /**
   * Offset the current year to compute dates in the scope of different years.
   */
  yearOffset?: number;
};
export type DateDefDateFnAddDay = {
  /**
   * A date function name from the [Date] class.
   */
  dateFn: keyof DateFn;

  /**
   * Possible date function arguments that may be required.
   */
  dateArgs?: number[];

  /**
   * Add additional day(s) to the date computed from the 'dateFn' option.
   */
  addDay?: number;

  /**
   * Offset the current year to compute dates in the scope of different years.
   */
  yearOffset?: number;
};
export type DateDefDateFnSubtractDay = {
  /**
   * A date function name from the [Date] class.
   */
  dateFn: keyof DateFn;

  /**
   * Possible date function arguments that may be required.
   */
  dateArgs?: number[];

  /**
   * Subtract some day(s) to the date computed from the 'dateFn' option.
   */
  subtractDay?: number;

  /**
   * Offset the current year to compute dates in the scope of different years.
   */
  yearOffset?: number;
};
export type DateDefMonthDowNthWeekInMonth = {
  /**
   * The month of this liturgical day.
   */
  month: MonthIndex;

  /**
   * The day of week this liturgical year must occur.
   */
  dayOfWeek: DayOfWeek;

  /**
   * The nth week in the month this liturgical year must occur.
   */
  nthWeekInMonth: number;

  /**
   * Offset the current year to compute dates in the scope of different years.
   */
  yearOffset?: number;
};
export type DateDefMonthLastDowInMonth = {
  /**
   * The month of this liturgical day.
   */
  month: MonthIndex;

  /**
   * The last day of week in the month this liturgical year must occur.
   */
  lastDayOfWeekInMonth: DayOfWeek;

  /**
   * Offset the current year to compute dates in the scope of different years.
   */
  yearOffset?: number;
};

/**
 * The liturgical day date definition, can extend a previously defined date
 */
export type DateDefExtended = AllXOR<[DateDef, DateDefAddDay, DateDefSubtractDay]>;

export type DateDefAddDay = { addDay: number };
export type DateDefSubtractDay = { subtractDay: number };

/**
 * The liturgical day date exception
 */
export type DateDefException = AllXOR<
  [
    {
      /**
       * Add an exception if the computed date occur between two dates.
       */
      ifIsBetween: { from: DateDef; to: DateDef; inclusive: boolean };
    },
    {
      /**
       * Add an exception if the computed date occur the same day as another date.
       */
      ifIsSameAsDate: DateDef;
    },
    {
      /**
       * Add an exception if the computed date occur on a specific day of week.
       */
      ifIsDayOfWeek: DayOfWeek;
    },
  ]
> & {
  /**
   * Set an updated date from the exception rules
   */
  setDate: DateDefExtended;
};

/**
 * Precedence of a liturgical day (UNLY #59)
 * Order is important: higher precedence type first, lower precedence type at the end.
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
 * Rank of a liturgical day.
 */
export enum Rank {
  /**
   * Solemnities are counted among the most important days, whose celebration
   * begins with First Vespers (Evening Prayer I) on the preceding day. Some Solemnities
   * are also endowed with their own Vigil Mass, which is to be used on the evening of the
   * preceding day, if an evening Mass is celebrated. (UNLY #11)
   */
  Solemnity = 'SOLEMNITY',

  /**
   * On the first day of each week, which is known as the Day of the Lord or the Lord’s
   * Day, the Church, by an apostolic tradition that draws its origin from the very day of
   * the Resurrection of Christ, celebrates the Paschal Mystery. Hence, Sunday must be
   * considered the primordial feast day. (UNLY #4)
   */
  Sunday = 'SUNDAY',

  /**
   * Feasts are celebrated within the limits of the natural day; accordingly they have
   * no First Vespers (Evening Prayer I), except in the case of Feasts of the Lord that fall
   * on a Sunday in Ordinary Time or in Christmas Time and which replace the Sunday
   * Office. (UNLY #13)
   */
  Feast = 'FEAST',

  /**
   * **Obligatory memorials** are liturgical commemorations of saints, events, or aspects of the
   * faith. Their observance is mandatory and integrated into the celebration of the occurring
   * weekday, following the liturgical norms outlined in the General Instruction of the Roman Missal
   * and the Liturgy of the Hours.
   * When an **obligatory memorial** falls on a weekday during the liturgical season of Lent or a
   * privileged weekday of Advent, it must only be celebrated as an **optional memorial**, as Lent
   * and Advent have their own specific liturgical observances that take precedence.
   */
  Memorial = 'MEMORIAL',

  /**
   * **Optional memorials** are liturgical commemorations of saints, events, or aspects of the
   * faith, but they are not obligatory.
   * Their observance is integrated into the celebration of the occurring weekday, adhering to the
   * liturgical norms provided in the General Instruction of the Roman Missal and the Liturgy of
   * the Hours.
   * In cases where multiple **optional memorials** are designated on the same day in the liturgical
   * calendar, only one of them may be celebrated, and the others must be omitted (UNLY #14).
   * This allows for some flexibility in choosing which optional memorial to commemorate when
   * multiple options are available.
   */
  OptionalMemorial = 'OPTIONAL_MEMORIAL',

  /**
   * The days of the week that follow Sunday are called weekdays; however, they are
   * celebrated differently according to the importance of each.
   *
   * a. Ash Wednesday and the weekdays of Holy Week, from Monday up to and including
   *    Thursday, take precedence over all other celebrations.
   * b. The weekdays of Advent from 17 December up to and including 24 December
   *    and all the weekdays of Lent have precedence over Obligatory Memorials.
   * c. Other weekdays give way to all Solemnities and Feasts and are combined with
   *    Memorials.
   *
   *  (UNLY #16)
   */
  Weekday = 'WEEKDAY',
}

export enum Common {
  // No common
  None = 'None',

  // Dedication of a Church
  DedicationAnniversary_Inside = 'DEDICATION_ANNIVERSARY__INSIDE',
  DedicationAnniversary_Outside = 'DEDICATION_ANNIVERSARY__OUTSIDE',

  // Blessed Virgin Mary
  BlessedVirginMary_OrdinaryTime = 'BLESSED_VIRGIN_MARY__ORDINARY_TIME',
  BlessedVirginMary_Advent = 'BLESSED_VIRGIN_MARY__ADVENT',
  BlessedVirginMary_Christmas = 'BLESSED_VIRGIN_MARY__CHRISTMAS',
  BlessedVirginMary_Easter = 'BLESSED_VIRGIN_MARY__EASTER',

  // Martyrs
  Martyrs_OutsideEaster_Several = 'MARTYRS__OUTSIDE_EASTER__SEVERAL',
  Martyrs_OutsideEaster_One = 'MARTYRS__OUTSIDE_EASTER__ONE',
  Martyrs_Easter_Several = 'MARTYRS__EASTER__SEVERAL',
  Martyrs_Easter_One = 'MARTYRS__EASTER__ONE',
  Martyrs_Missionary_Several = 'MARTYRS__MISSIONARY__SEVERAL',
  Martyrs_Missionary_One = 'MARTYRS__MISSIONARY__ONE',
  Martyrs_Virgin = 'MARTYRS__VIRGIN',
  Martyrs_Woman = 'MARTYRS__WOMAN',

  // Pastors
  Pastors_PopeOrBishop = 'PASTORS__POPE_OR_BISHOP',
  Pastors_Bishop = 'PASTORS__BISHOP',
  Pastors_Several = 'PASTORS__SEVERAL',
  Pastors_One = 'PASTORS__ONE',
  Pastors_Founder_One = 'PASTORS__FOUNDER__ONE',
  Pastors_Founder_Several = 'PASTORS__FOUNDER__SEVERAL',
  Pastors_Missionary = 'PASTORS__MISSIONARY',

  // Doctors of the Church
  DoctorsOfTheChurch = 'DOCTORS_OF_THE_CHURCH',

  // Virgins
  Virgins_Several = 'VIRGINS__SEVERAL',
  Virgins_One = 'VIRGINS__ONE',

  // Holy Men and Women
  Saints_All_Several = 'SAINTS__ALL__SEVERAL',
  Saints_All_One = 'SAINTS__ALL__ONE',
  Saints_Abbot = 'SAINTS__ABBOT',
  Saint_Monk = 'SAINTS__MONK',
  Saints_Nun = 'SAINTS__NUN',
  Saints_Religious = 'SAINTS__RELIGIOUS',
  Saints_MercyWorks = 'SAINTS__MERCY_WORKS',
  Saints_Educators = 'SAINTS__EDUCATORS',
  Saints_HolyWomen = 'SAINTS__HOLY_WOMEN',
}

/**
 * The **CommonDefinition** refers to a simplified version of the **Commons** enum.
 * To be used in the martyrology metadata.
 */
export enum CommonDefinition {
  // No common
  None = 'None',

  // Dedication of a Church
  DedicationAnniversary_Inside = 'DEDICATION_ANNIVERSARY__INSIDE',
  DedicationAnniversary_Outside = 'DEDICATION_ANNIVERSARY__OUTSIDE',

  // Blessed Virgin Mary
  BlessedVirginMary = 'BLESSED_VIRGIN_MARY',

  // Martyrs
  Martyrs = 'MARTYRS',
  MissionaryMartyrs = 'MISSIONARY_MARTYRS',
  VirginMartyrs = 'VIRGIN_MARTYRS',
  WomanMartyrs = 'WOMAN_MARTYRS',

  // Pastors
  Pastors = 'PASTORS',
  Popes = 'POPE_OR_BISHOP',
  Bishops = 'BISHOPS',
  Founders = 'FOUNDERS',
  Missionaries = 'MISSIONARIES',

  // Doctors of the Church
  DoctorsOfTheChurch = 'DOCTORS_OF_THE_CHURCH',

  // Virgins
  Virgins = 'VIRGINS',

  // Holy Men and Women
  Saints = 'SAINTS',
  Abbots = 'ABBOTS',
  Monks = 'MONKS',
  Nuns = 'NUNS',
  Religious = 'RELIGIOUS',
  MercyWorkers = 'MERCY_WORKERS',
  Educators = 'EDUCATORS',
  HolyWomen = 'HOLY_WOMEN',
}

/**
 * Liturgical seasons are segments of time that when combined, form the liturgical year.
 * Liturgical seasons are distinguished by specific names that signify the character
 * of the season.
 */
export enum Season {
  Advent = 'ADVENT',
  ChristmasTime = 'CHRISTMAS_TIME',
  Lent = 'LENT',
  PaschalTriduum = 'PASCHAL_TRIDUUM',
  EasterTime = 'EASTER_TIME',
  OrdinaryTime = 'ORDINARY_TIME',
}

export enum Period {
  ChristmasOctave = 'CHRISTMAS_OCTAVE',
  DaysBeforeEpiphany = 'DAYS_BEFORE_EPIPHANY',
  DaysFromEpiphany = 'DAYS_FROM_EPIPHANY',
  ChristmasToPresentationOfTheLord = 'CHRISTMAS_TO_PRESENTATION_OF_THE_LORD',
  PresentationOfTheLordToHolyThursday = 'PRESENTATION_OF_THE_LORD_TO_HOLY_THURSDAY',
  HolyWeek = 'HOLY_WEEK',
  EasterOctave = 'EASTER_OCTAVE',
  EarlyOrdinaryTime = 'EARLY_ORDINARY_TIME',
  LateOrdinaryTime = 'LATE_ORDINARY_TIME',
}

/**
 * Titles of Saints and Blessed from the Martyrology catalog.
 */
export enum Title {
  Abbess = 'ABBESS',
  Abbot = 'ABBOT',
  Apostle = 'APOSTLE',
  Archangel = 'ARCHANGEL',
  Bishop = 'BISHOP',
  Deacon = 'DEACON',
  DoctorOfTheChurch = 'DOCTOR_OF_THE_CHURCH',
  Empress = 'EMPRESS',
  Evangelist = 'EVANGELIST',
  FirstBishop = 'FIRST_BISHOP',
  Hermit = 'HERMIT',
  King = 'KING',
  Martyr = 'MARTYR',
  Missionary = 'MISSIONARY',
  Monk = 'MONK',
  MotherAndQueenOfChile = 'MOTHER_AND_QUEEN_OF_CHILE',
  ParentsOfTheBlessedVirginMary = 'PARENTS_OF_THE_BLESSED_VIRGIN_MARY',
  Pope = 'POPE',
  Patriarch = 'PATRIARCH',
  Pilgrim = 'PILGRIM',
  Priest = 'PRIEST',
  Prophet = 'PROPHET',
  ProtoMartyrOfOceania = 'PROTO_MARTYR_OF_OCEANIA',
  Queen = 'QUEEN',
  QueenOfPoland = 'QUEEN_OF_POLAND',
  Religious = 'RELIGIOUS',
  SlavicMissionary = 'SLAVIC_MISSIONARY',
  SpouseOfTheBlessedVirginMary = 'SPOUSE_OF_THE_BLESSED_VIRGIN_MARY',
  TheFirstMartyr = 'THE_FIRST_MARTYR',
  Virgin = 'VIRGIN',

  /**
   *  Patron Titles of Saints and Blessed from the Martyrology catalog.
   */
  CopatronOfEurope = 'COPATRON_OF_EUROPE',
  CopatronOfIreland = 'COPATRON_OF_IRELAND',
  CopatronOfCanada = 'COPATRON_OF_CANADA',
  CopatronessOfEurope = 'COPATRONESS_OF_EUROPE',
  CopatronessOfFrance = 'COPATRONESS_OF_FRANCE',
  CopatronessOfIreland = 'COPATRONESS_OF_IRELAND',
  CopatronessOfItalyAndEurope = 'COPATRONESS_OF_ITALY_AND_EUROPE',
  CopatronessOfThePhilippines = 'COPATRONESS_OF_THE_PHILIPPINES',
  PatronOfCanada = 'PATRON_OF_CANADA',
  PatronOfEngland = 'PATRON_OF_ENGLAND',
  PatronOfEurope = 'PATRON_OF_EUROPE',
  PatronOfFrance = 'PATRON_OF_FRANCE',
  PatronOfIreland = 'PATRON_OF_IRELAND',
  PatronOfItaly = 'PATRON_OF_ITALY',
  PatronOfOceania = 'PATRON_OF_OCEANIA',
  PatronOfPoland = 'PATRON_OF_POLAND',
  PatronOfRussia = 'PATRON_OF_RUSSIA',
  PatronOfScotland = 'PATRON_OF_SCOTLAND',
  PatronOfSpain = 'PATRON_OF_SPAIN',
  PatronOfTheCzechNation = 'PATRON_OF_THE_CZECH_NATION',
  PatronOfTheDiocese = 'PATRON_OF_THE_DIOCESE',
  PatronOfWales = 'PATRON_OF_WALES',
  PatronessOfAlsace = 'PATRONESS_OF_ALSACE',
  PatronessOfArgentina = 'PATRONESS_OF_ARGENTINA',
  PatronessOfBrazil = 'PATRONESS_OF_BRAZIL',
  PatronessOfHungary = 'PATRONESS_OF_HUNGARY',
  PatronessOfPuertoRico = 'PATRONESS_OF_PUERTO_RICO',
  PatronessOfSlovakia = 'PATRONESS_OF_SLOVAKIA',
  PatronessOfTheAmericas = 'PATRONESS_OF_THE_AMERICAS',
  PatronessOfThePhilippines = 'PATRONESS_OF_THE_PHILIPPINES',
  PatronessOfTheProvinceOfQuebec = 'PATRONESS_OF_THE_PROVINCE_OF_QUEBEC',
  PatronessOfTheUSA = 'PATRONESS_OF_THE_U_S_A',
  PatronOfTheClergyOfTheArchdioceseOfLyon = 'PATRON_OF_THE_CLERGY_OF_THE_ARCHDIOCESE_OF_LYON',
  PatronOfTheCityOfLyon = 'PATRON_OF_THE_CITY_OF_LYON',
  PatronessOfCostaRica = 'PATRONESS_OF_COSTA_RICA',
  PrincipalPatronOfTheDiocese = 'PRINCIPAL_PATRON_OF_THE_DIOCESE',
  SecondPatronOfTheDiocese = 'SECOND_PATRON_OF_THE_DIOCESE',
}

export type CompoundTitle = {
  /**
   * Add title(s) to the end of the existing list of title(s).
   */
  append?: Title;
  /**
   * Add title(s) to the  beginning of the existing list of title(s).
   */
  prepend?: Title;
};
export type TitlesDef = Title | CompoundTitle;

/**
 * The associated martyrology item.
 */
export type MartyrologyItemPointer = ResourceId | MartyrologyItemRedefined;
export type SaintCount = number | 'many';

/**
 * The associated martyrology item, with its overridden properties.
 */
export type MartyrologyItemRedefined = {
  /**
   * The ID of the martyrology item.
   */
  id: string;

  /**
   * The redefined titles of the martyrology item.
   */
  titles?: TitlesDef;

  /**
   * Specify if titles should not be displayed. This can occur when a title is already included in
   * the name of the martyrology item.
   */
  hideTitles?: boolean;

  /**
   * Specify the number of persons this martyrology item is representing.
   */
  count?: SaintCount;
};

/**
 * Liturgical colors that can be used as metadata for celebrations.
 */
export enum Color {
  Red = 'RED',
  Rose = 'ROSE',
  Purple = 'PURPLE',
  Green = 'GREEN',
  White = 'WHITE',
  Gold = 'GOLD',
  Black = 'BLACK',
}

export type DayDefinition = {
  id: DayId;

  /**
   * Date definition
   */
  dateDef?: DateDef;

  /**
   * Date definition exception
   */
  dateExceptions?: DateDefException[];

  /**
   * The precedence type of the liturgical day.
   */
  precedence?: Precedence;

  /**
   * The **Common** refers to a set of prayers, readings, and chants used for celebrating saints or
   * feasts that belong to a specific category, such as martyrs, virgins, pastors, or the Blessed
   * Virgin Mary.
   */
  commonsDef?: CommonDefinition | CommonDefinition[];

  /**
   * Holy days of obligation are days on which the faithful are expected to attend Mass,
   * and engage in rest from work and recreation.
   */
  isHolyDayOfObligation?: boolean;

  /**
   * In addition to this liturgical day, allow similar items that have the same rank,
   * and the same or lower precedence,
   * so the current liturgical day will not overwrite another defined item.
   */
  allowSimilarRankItems?: boolean;

  /**
   * Specify is this LiturgicalDay is optional within a specific liturgical calendar.
   *
   * UNLY #14:
   * Memorials are either obligatory or optional; their observance is integrated into
   * the celebration of the occurring weekday in accordance with the norms set forth in the
   * General Instruction of the Roman Missal and of the Liturgy of the Hours
   *
   * Note: also used for the dedication of consecrated churches, which is an optional solemnity
   * that should not overwrite the default weekday.
   */
  isOptional?: boolean;

  /**
   * Specify a custom locale ID for this date definition, in this calendar.
   */
  customLocaleId?: LocaleId;

  /**
   * Link one or multiple Saints, Blessed, or any other celebrations from the Martyrology catalog.
   */
  martyrology?: MartyrologyItemPointer[];

  /**
   * Combined titles of each Saints linked to this date definition.
   */
  titles?: TitlesDef;

  /**
   * If this liturgical day must be removed from this calendar and from all those it inherits
   * (the parent calendars), on the final calendar generated by romcal.
   */
  drop?: boolean;

  /**
   * The liturgical color(s) of the liturgical day.
   * @deprecated
   */
  colors?: Color | Color[];
};
