/**
 * Type definitions for the Calendar Editor
 * Based on Romcal's TypeScript bindings
 */

// Calendar Types
export type CalendarType =
  | 'GENERAL_ROMAN'
  | 'REGION'
  | 'COUNTRY'
  | 'ARCHDIOCESE'
  | 'DIOCESE'
  | 'CITY'
  | 'PARISH'
  | 'GENERAL_COMMUNITY'
  | 'REGIONAL_COMMUNITY'
  | 'LOCAL_COMMUNITY'
  | 'OTHER';

export type CalendarJurisdiction = 'ECCLESIASTICAL' | 'CIVIL';

export type EasterCalculationType = 'GREGORIAN' | 'JULIAN';

export interface CalendarMetadata {
  type: CalendarType;
  jurisdiction: CalendarJurisdiction;
}

export interface ParticularConfig {
  epiphany_on_sunday?: boolean | null;
  ascension_on_sunday?: boolean | null;
  corpus_christi_on_sunday?: boolean | null;
  easter_calculation_type?: EasterCalculationType | null;
}

// Date Definition Types
export type MonthIndex = number; // 1-12
export type DayOfWeek = number; // 0=Sunday, 1=Monday, etc.

export type DateFn =
  | 'MARY_MOTHER_OF_THE_CHURCH'
  | 'EPIPHANY_SUNDAY'
  | 'PRESENTATION_OF_THE_LORD'
  | 'ANNUNCIATION'
  | 'PALM_SUNDAY'
  | 'EASTER_SUNDAY'
  | 'DIVINE_MERCY_SUNDAY'
  | 'IMMACULATE_HEART_OF_MARY'
  | 'PENTECOST_SUNDAY'
  | 'CORPUS_CHRISTI_SUNDAY'
  | 'NATIVITY_OF_JOHN_THE_BAPTIST'
  | 'PETER_AND_PAUL_APOSTLES'
  | 'TRANSFIGURATION'
  | 'ASSUMPTION'
  | 'EXALTATION_OF_THE_HOLY_CROSS'
  | 'ALL_SAINTS'
  | 'IMMACULATE_CONCEPTION_OF_MARY';

export type DateDef =
  | {
      month: MonthIndex;
      date: number;
      day_offset?: number;
    }
  | {
      date_fn: DateFn;
      day_offset?: number;
    }
  | {
      month: MonthIndex;
      day_of_week: DayOfWeek;
      nth_week_in_month: number;
      day_offset?: number;
    }
  | {
      month: MonthIndex;
      last_day_of_week_in_month: DayOfWeek;
      day_offset?: number;
    }
  | Record<string, never>;

// Precedence Types
export type Precedence =
  | 'TRIDUUM_1'
  | 'PROPER_OF_TIME_SOLEMNITY_2'
  | 'PRIVILEGED_SUNDAY_2'
  | 'ASH_WEDNESDAY_2'
  | 'WEEKDAY_OF_HOLY_WEEK_2'
  | 'WEEKDAY_OF_EASTER_OCTAVE_2'
  | 'GENERAL_SOLEMNITY_3'
  | 'COMMEMORATION_OF_ALL_THE_FAITHFUL_DEPARTED_3'
  | 'PROPER_SOLEMNITY__PRINCIPAL_PATRON_4A'
  | 'PROPER_SOLEMNITY__DEDICATION_OF_THE_OWN_CHURCH_4B'
  | 'PROPER_SOLEMNITY__TITLE_OF_THE_OWN_CHURCH_4C'
  | 'PROPER_SOLEMNITY__TITLE_OR_FOUNDER_OR_PRIMARY_PATRON_OF_A_RELIGIOUS_ORG_4D'
  | 'GENERAL_LORD_FEAST_5'
  | 'UNPRIVILEGED_SUNDAY_6'
  | 'GENERAL_FEAST_7'
  | 'PROPER_FEAST__PRINCIPAL_PATRON_OF_A_DIOCESE_8A'
  | 'PROPER_FEAST__DEDICATION_OF_THE_CATHEDRAL_CHURCH_8B'
  | 'PROPER_FEAST__PRINCIPAL_PATRON_OF_A_REGION_8C'
  | 'PROPER_FEAST__TITLE_OR_FOUNDER_OR_PRIMARY_PATRON_OF_A_RELIGIOUS_ORG_8D'
  | 'PROPER_FEAST__TO_AN_INDIVIDUAL_CHURCH_8E'
  | 'PROPER_FEAST_8F'
  | 'PRIVILEGED_WEEKDAY_9'
  | 'GENERAL_MEMORIAL_10'
  | 'PROPER_MEMORIAL__SECOND_PATRON_11A'
  | 'PROPER_MEMORIAL_11B'
  | 'OPTIONAL_MEMORIAL_12'
  | 'WEEKDAY_13';

// Common Definition Types
export type CommonDefinition =
  | 'NONE'
  | 'DEDICATION_ANNIVERSARY__INSIDE'
  | 'DEDICATION_ANNIVERSARY__OUTSIDE'
  | 'BLESSED_VIRGIN_MARY'
  | 'MARTYRS'
  | 'MISSIONARY_MARTYRS'
  | 'VIRGIN_MARTYRS'
  | 'WOMAN_MARTYRS'
  | 'PASTORS'
  | 'POPES'
  | 'BISHOPS'
  | 'FOUNDERS'
  | 'MISSIONARIES'
  | 'DOCTORS_OF_THE_CHURCH'
  | 'VIRGINS'
  | 'SAINTS'
  | 'ABBOTS'
  | 'MONKS'
  | 'NUNS'
  | 'RELIGIOUS'
  | 'MERCY_WORKERS'
  | 'EDUCATORS'
  | 'HOLY_WOMEN';

export type CommonsDef = CommonDefinition | CommonDefinition[];

// Title Types
export type Title =
  | 'ABBESS'
  | 'ABBOT'
  | 'APOSTLE'
  | 'ARCHANGEL'
  | 'BISHOP'
  | 'DEACON'
  | 'DOCTOR_OF_THE_CHURCH'
  | 'EMPRESS'
  | 'EVANGELIST'
  | 'FIRST_BISHOP'
  | 'HERMIT'
  | 'KING'
  | 'MARTYR'
  | 'MISSIONARY'
  | 'MONK'
  | 'MOTHER_AND_QUEEN_OF_CHILE'
  | 'PARENTS_OF_THE_BLESSED_VIRGIN_MARY'
  | 'POPE'
  | 'PATRIARCH'
  | 'PILGRIM'
  | 'PRIEST'
  | 'PROPHET'
  | 'PROTO_MARTYR_OF_OCEANIA'
  | 'QUEEN'
  | 'QUEEN_OF_POLAND'
  | 'RELIGIOUS'
  | 'SLAVIC_MISSIONARY'
  | 'SPOUSE_OF_THE_BLESSED_VIRGIN_MARY'
  | 'THE_FIRST_MARTYR'
  | 'VIRGIN'
  | 'COPATRON_OF_EUROPE'
  | 'COPATRON_OF_IRELAND'
  | 'COPATRON_OF_CANADA'
  | 'COPATRONESS_OF_EUROPE'
  | 'COPATRONESS_OF_FRANCE'
  | 'COPATRONESS_OF_IRELAND'
  | 'COPATRONESS_OF_ITALY_AND_EUROPE'
  | 'COPATRONESS_OF_THE_PHILIPPINES'
  | 'PATRON_OF_CANADA'
  | 'PATRON_OF_ENGLAND'
  | 'PATRON_OF_EUROPE'
  | 'PATRON_OF_FRANCE'
  | 'PATRON_OF_IRELAND'
  | 'PATRON_OF_ITALY'
  | 'PATRON_OF_OCEANIA'
  | 'PATRON_OF_POLAND'
  | 'PATRON_OF_RUSSIA'
  | 'PATRON_OF_SCOTLAND'
  | 'PATRON_OF_SPAIN'
  | 'PATRON_OF_THE_CZECH_NATION'
  | 'PATRON_OF_THE_DIOCESE'
  | 'PATRON_OF_WALES'
  | 'PATRONESS_OF_ALSACE'
  | 'PATRONESS_OF_ARGENTINA'
  | 'PATRONESS_OF_BRAZIL'
  | 'PATRONESS_OF_HUNGARY'
  | 'PATRONESS_OF_PUERTO_RICO'
  | 'PATRONESS_OF_SLOVAKIA'
  | 'PATRONESS_OF_THE_AMERICAS'
  | 'PATRONESS_OF_THE_PHILIPPINES'
  | 'PATRONESS_OF_THE_PROVINCE_OF_QUEBEC'
  | 'PATRONESS_OF_THE_USA'
  | 'PATRON_OF_THE_CLERGY_OF_THE_ARCHDIOCESE_OF_LYON'
  | 'PATRON_OF_THE_CITY_OF_LYON'
  | 'PATRONESS_OF_COSTA_RICA'
  | 'PRINCIPAL_PATRON_OF_THE_DIOCESE'
  | 'SECOND_PATRON_OF_THE_DIOCESE';

export interface CompoundTitle {
  append?: Title[];
  prepend?: Title[];
}

export type TitlesDef = Title[] | CompoundTitle;

// Color Types
export type Color = 'RED' | 'ROSE' | 'PURPLE' | 'GREEN' | 'WHITE' | 'GOLD' | 'BLACK';

export type ColorsDef = Color | Color[];

// Entity Types
export type SaintCount = number | 'MANY' | null;

export interface EntityOverride {
  id: string;
  titles?: TitlesDef;
  hide_titles?: boolean;
  count?: SaintCount;
}

export type EntityRef = string | EntityOverride;

// Day Definition
export interface DayDefinition {
  date_def?: DateDef | null;
  date_exceptions?: unknown; // Complex type, simplified for editor
  precedence?: Precedence | null;
  commons_def?: CommonsDef | null;
  is_holy_day_of_obligation?: boolean | null;
  allow_similar_rank_items?: boolean | null;
  is_optional?: boolean | null;
  custom_locale_id?: string | null;
  entities?: EntityRef[] | null;
  titles?: TitlesDef | null;
  drop?: boolean | null;
  colors?: ColorsDef | null;
  masses?: unknown; // Complex type, simplified for editor
}

// Calendar Definition
export interface CalendarDefinition {
  $schema?: string | null;
  id: string;
  metadata: CalendarMetadata;
  particular_config?: ParticularConfig | null;
  parent_calendar_ids: string[];
  days_definitions: Record<string, DayDefinition>;
}

// Entity Definition for Resources
export type EntityType = 'PERSON' | 'PLACE' | 'EVENT';
export type CanonizationLevel = 'BLESSED' | 'SAINT';
export type Sex = 'MALE' | 'FEMALE';

export type SaintDate = number | string;
export type SaintDateDef =
  | SaintDate
  | { between: [SaintDate, SaintDate] }
  | { or: SaintDate[] }
  | { century: number };

export interface EntityDefinition {
  type?: EntityType;
  fullname?: string;
  name?: string;
  canonization_level?: CanonizationLevel;
  date_of_canonization?: SaintDateDef;
  date_of_canonization_is_approximative?: boolean;
  date_of_beatification?: SaintDateDef;
  date_of_beatification_is_approximative?: boolean;
  hide_canonization_level?: boolean;
  titles?: Title[];
  sex?: Sex;
  hide_titles?: boolean;
  date_of_dedication?: SaintDateDef;
  date_of_birth?: SaintDateDef;
  date_of_birth_is_approximative?: boolean;
  date_of_death?: SaintDateDef;
  date_of_death_is_approximative?: boolean;
  count?: SaintCount;
  sources?: string[];
}

// Resources
export interface Resources {
  $schema?: string | null;
  locale: string;
  metadata?: unknown; // Complex type, simplified for editor
  entities?: Record<string, EntityDefinition>;
}

// Editor-specific types
export interface EditorCalendarInfo {
  id: string;
  name: string;
  type: CalendarType;
  path: string;
}

export interface EditorEntityInfo {
  id: string;
  fullname?: string;
  name?: string;
  canonization_level?: CanonizationLevel;
  locale: string;
}

export type DateDefType = 'fixed' | 'date_fn' | 'nth_weekday' | 'last_weekday' | 'inherited';

export interface NavigationItem {
  type: 'metadata' | 'parents' | 'config' | 'days' | 'day' | 'entities' | 'entity';
  id?: string;
  locale?: string;
}
