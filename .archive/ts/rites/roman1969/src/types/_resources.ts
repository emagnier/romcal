export interface ResourcesDefinition {
  $schema?: string;

  /** Locale code of the resources, in BCP-47 IETF tag format */
  locale: string;

  /**
   * Metadata of the resources
   */
  metadata?: {
    ordinals?: LocaleOrdinals;
    weekdays?: LocaleWeeks;
    months?: LocaleMonths;
    colors?: LocaleColors;
    seasons?: {
      advent?: {
        season?: string;
        weekday?: string;
        sunday?: string;
        privileged_weekday?: string;
      };
      christmas_time?: {
        season?: string;
        day?: string;
        octave?: string;
        before_epiphany?: string;
        second_sunday_after_christmas?: string;
        after_epiphany?: string;
      };
      ordinary_time?: {
        season?: string;
        weekday?: string;
        sunday?: string;
      };
      lent?: {
        season?: string;
        weekday?: string;
        sunday?: string;
        day_after_ash_wed?: string;
        holy_week_day?: string;
      };
      paschal_triduum?: {
        season?: string;
      };
      easter_time?: {
        season?: string;
        weekday?: string;
        sunday?: string;
        octave?: string;
      };
    };
    periods?: {
      epiphany?: string;
      holy_week?: string;
    };
    ranks?: {
      solemnity?: string;
      sunday?: string;
      feast?: string;
      memorial?: string;
      optional_memorial?: string;
      weekday?: string;
    };
    cycles?: {
      proper_of_time?: string;
      proper_of_saints?: string;
      sunday_year_a?: string;
      sunday_year_b?: string;
      sunday_year_c?: string;
      weekday_year_1?: string;
      weekday_year_2?: string;
      psalter_week_1?: string;
      psalter_week_2?: string;
      psalter_week_3?: string;
      psalter_week_4?: string;
    };
  };

  /**
   * Entities of the resources: a person, a place, an event, etc.
   */
  entities?: EntityDefinition[];
}

// export type LocaleOrdinals = Record<string, string | ((n: number) => string)>;
export type LocaleOrdinals = Record<string, string>;

export type LocaleWeeks = Record<string, string>;

export type LocaleMonths = Record<string, string>;

export type LocaleColors = {
  black?: string;
  gold?: string;
  green?: string;
  purple?: string;
  red?: string;
  rose: string;
  white?: string;
};

export enum EntityType {
  Person = 'PERSON',
  Place = 'PLACE',
  Event = 'EVENT',
}

export type EntityId = string;

export type EntityDefinition = {
  id: EntityId;

  /**
   * The type of the entity.
   * @default EntityType.Person
   */
  type?: EntityType;

  /**
   * The full name of the entity.
   */
  fullname?: string;

  /**
   * The short name of the entity, without the canonization level and titles.
   */
  name?: string;

  /**
   * The canonization level of a person.
   */
  canonizationLevel?: CanonizationLevel;

  /**
   * Date of Canonization, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
   * or an object describing date range, multiple possible date, or a century.
   */
  dateOfCanonization?: SaintDateDef;

  /**
   * Specify whether an approximate indicator should be added, when the date is displayed.
   * For example in English: 'c. 201'.
   */
  dateOfCanonizationIsApproximative?: boolean;

  /**
   * Date of Beatification, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
   * or an object describing date range, multiple possible date, or a century.
   */
  dateOfBeatification?: SaintDateDef;

  /**
   * Specify whether an approximate indicator should be added, when the date is displayed.
   * For example in English: 'c. 201'.
   */
  dateOfBeatificationIsApproximative?: boolean;

  /**
   * Specify if the canonization level should not be displayed.
   * It's generally the case when the canonization are already included in the name.
   */
  hideCanonizationLevel?: boolean;

  /**
   * Titles of the Saint or the Blessed
   */
  titles?: Title | Title[];

  /**
   * Determine if the Saint or the Blessed is a male or a female.
   */
  sex?: Sex;

  /**
   * Specify if the titles should not be displayed.
   * It's generally the case when titles are already included in the name.
   */
  hideTitles?: boolean;

  /**
   * Date of Dedication of a church, basilica, or cathedral (or other place of worship),
   * as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
   * or an object describing date range, multiple possible date, or a century.
   */
  dateOfDedication?: SaintDateDef;

  /**
   * Date of Birth, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
   * or an object describing date range, multiple possible date, or a century.
   */
  dateOfBirth?: SaintDateDef;

  /**
   * Specify whether an approximate indicator should be added, when the date is displayed.
   * For example in English: 'c. 201'.
   */
  dateOfBirthIsApproximative?: boolean;

  /**
   * Date of Death, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
   * or an object describing date range, multiple possible date, or a century.
   */
  dateOfDeath?: SaintDateDef;

  /**
   * Specify whether an approximate indicator should be added, when the date is displayed.
   * For example in English: 'c. 201'.
   */
  dateOfDeathIsApproximative?: boolean;

  /**
   * Number of person that this definition represent.
   * It could be set as 'many' if the number is not defined.
   */
  count?: SaintCount;

  /**
   * Sources for the information about this entity
   */
  sources?: string[];

  /**
   * Internal notes
   * @private
   */
  _todo?: string[];
};

export type SaintCount = number | 'many';
export type SaintDate = number | string;
export type SaintDateDef = SaintDate | { between: [SaintDate, SaintDate] } | { or: SaintDate[] } | { century: number };

/**
 * Titles of a Saint or a Blessed
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
   *  Patron Titles of a Saint or a Blessed
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

/**
 * Canonization level of a Saint or a Blessed.
 */
export enum CanonizationLevel {
  Blessed = 'BLESSED',
  Saint = 'SAINT',
}

/**
 * Sex of a Saint or a Blessed.
 */
export enum Sex {
  Male = 'MALE',
  Female = 'FEMALE',
}
