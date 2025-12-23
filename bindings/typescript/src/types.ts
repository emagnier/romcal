/**
 * Calendar definition
 *
 * Resources definition
 *
 * A single day in the liturgical calendar with computed values and inheritance information.
 */
export type AllTypes = {
    $schema?:          null | string;
    days_definitions?: { [key: string]: DayDefinition };
    /**
     * The unique identifier of the liturgical day
     */
    id?: string;
    /**
     * Metadata of the resources
     */
    metadata?:            ResourcesMetadata | null;
    parent_calendar_ids?: string[];
    particular_config?:   ParticularConfig | null;
    /**
     * Entities of the resources: a person, a place, an event, etc.
     *
     * The entities (Saints, Blessed, or Places) linked to this liturgical day.
     */
    entities?: Entities;
    /**
     * Locale code of the resources, in BCP-47 IETF tag format
     */
    locale?: string;
    /**
     * Allows similar items with the same rank and same or lower precedence
     * to coexist without this liturgical day overwriting them.
     */
    allow_similar_rank_items?: boolean;
    /**
     * The liturgical colors for this liturgical day.
     */
    colors?: ColorInfo[];
    /**
     * The common prayers, readings, and chants used for celebrating saints or
     * feasts that belong to a specific category, such as martyrs, virgins, pastors, or the
     * Blessed
     * Virgin Mary.
     */
    commons?: CommonInfo[];
    /**
     * The computed date of the liturgical day.
     */
    date?: string;
    /**
     * The date definition for this liturgical day.
     */
    date_def?: DateDef;
    /**
     * The date definition exceptions for this liturgical day.
     */
    date_exceptions?: DateDefException[];
    /**
     * The day number within the current liturgical season.
     */
    day_of_season?: number | null;
    /**
     * The day of the week for this liturgical day.
     * Returns a number from 0 (Sunday) to 6 (Saturday).
     */
    day_of_week?: number;
    /**
     * The last day of the current liturgical year for this liturgical day,
     * i.e. the last Saturday of Ordinary Time, in the 34th week.
     */
    end_of_liturgical_year?: string;
    /**
     * The last day of the current liturgical season for this liturgical day.
     */
    end_of_season?: null | string;
    /**
     * The ID of the calendar where this liturgical day is defined.
     * Indicates the source calendar in the inheritance chain.
     */
    from_calendar_id?: string;
    /**
     * The full name of the liturgical day
     */
    fullname?: string;
    /**
     * Holy days of obligation are days on which the faithful are expected to attend Mass,
     * and engage in rest from work and recreation.
     */
    is_holy_day_of_obligation?: boolean;
    /**
     * Indicates if this liturgical day is optional within a specific liturgical calendar.
     */
    is_optional?: boolean;
    /**
     * The masses celebrated on this liturgical day.
     * Most days have a single DayMass, but some have multiple masses
     * (e.g., Christmas: PreviousEveningMass, NightMass, MassAtDawn, DayMass).
     * Aliturgical days like Holy Saturday have an empty list.
     */
    masses?: MassInfo[];
    /**
     * The nth occurrence of this day of the week within the current month.
     * For example, the 3rd Sunday of the month would have nth_day_of_week_in_month = 3.
     */
    nth_day_of_week_in_month?: number;
    /**
     * Contains the differences between this liturgical day and its parent definitions.
     * Each element in the array represents the diff from a parent calendar definition.
     * The array is ordered from most general (e.g., general_roman) to most specific.
     */
    parent_overrides?: ParentOverride[];
    /**
     * The liturgical periods to which this liturgical day belongs.
     */
    periods?: PeriodInfo[];
    /**
     * The liturgical precedence for this liturgical day.
     */
    precedence?: Precedence;
    /**
     * The psalter week cycle to which this liturgical day belongs.
     */
    psalter_week?: PsalterWeekCycle;
    /**
     * The localized name of the psalter week cycle to which this liturgical day belongs.
     */
    psalter_week_name?: string;
    /**
     * The liturgical rank for this liturgical day.
     */
    rank?: Rank;
    /**
     * The localized liturgical rank for this liturgical day.
     */
    rank_name?: string;
    /**
     * The liturgical seasons to which this liturgical day belongs.
     */
    season?: Season | null;
    /**
     * The liturgical season name.
     */
    season_name?: null | string;
    /**
     * The first day of the current liturgical year for this liturgical day,
     * i.e. the first Sunday of Advent.
     */
    start_of_liturgical_year?: string;
    /**
     * The first day of the current liturgical season for this liturgical day.
     */
    start_of_season?: null | string;
    /**
     * The Sunday cycle to which this liturgical day belongs.
     */
    sunday_cycle?: SundayCycle;
    /**
     * The localized name of the Sunday cycle to which this liturgical day belongs.
     */
    sunday_cycle_name?: string;
    /**
     * The titles for this liturgical day.
     */
    titles?: TitlesDef;
    /**
     * The week number of the current liturgical season.
     * Starts from `1`, except in the seasons of lent,
     * the week of Ash Wednesday to the next Saturday is counted as `0`.
     */
    week_of_season?: number | null;
    /**
     * The weekday cycle to which this liturgical day belongs.
     */
    weekday_cycle?: WeekdayCycle;
    /**
     * The localized name of the weekday cycle to which this liturgical day belongs.
     */
    weekday_cycle_name?: string;
}

/**
 * Liturgical color information with localized name.
 */
export type ColorInfo = {
    /**
     * The color key
     */
    key: Color;
    /**
     * The localized name of the color
     */
    name: string;
}

/**
 * Liturgical colors used in the celebration of Mass and other liturgical services.
 * Each color has specific liturgical significance and is used during particular seasons or
 * celebrations.
 *
 * The color key
 *
 * Red - used for martyrs, Pentecost, and Palm Sunday
 *
 * Rose - used on Gaudete Sunday (3rd Advent) and Laetare Sunday (4th Lent)
 *
 * Purple - used during Advent and Lent
 *
 * Green - used during Ordinary Time
 *
 * White - used for Christmas, Easter, and most feasts
 *
 * Gold - used for solemn celebrations and special occasions
 *
 * Black - used for funerals and All Souls' Day
 */
export enum Color {
    Black = "BLACK",
    Gold = "GOLD",
    Green = "GREEN",
    Purple = "PURPLE",
    Red = "RED",
    Rose = "ROSE",
    White = "WHITE",
}

/**
 * Liturgical common information with localized name.
 */
export type CommonInfo = {
    /**
     * The common key
     */
    key: Common;
    /**
     * The localized name of the common
     */
    name: string;
}

/**
 * The common key
 *
 * Common prayers and readings for different categories of saints and celebrations.
 * Provides standardized liturgical texts for various types of commemorations.
 *
 * No common.
 *
 * Dedication anniversary (in the Church that was Dedicated).
 *
 * Dedication anniversary (outside the Church that was Dedicated).
 *
 * Common of the Blessed Virgin Mary (Ordinary Time).
 *
 * Common of the Blessed Virgin Mary (Advent).
 *
 * Common of the Blessed Virgin Mary (Christmas Time).
 *
 * Common of the Blessed Virgin Mary (Easter Time).
 *
 * Common of Several Martyrs (outside Easter).
 *
 * Common of One Martyr (outside Easter).
 *
 * Common of Several Martyrs (Easter Time).
 *
 * Common of One Martyr (Easter Time).
 *
 * Common for Several Missionary Martyrs.
 *
 * Common for One Missionary Martyr.
 *
 * Common for Virgin Martyrs.
 *
 * Common for Holy Woman Martyrs.
 *
 * Common for a Pope or for a Bishop
 *
 * Common for a Bishop
 *
 * Common for Several Pastors
 *
 * Common for One Pastor
 *
 * Common for one Founder
 *
 * Common for several Founders
 *
 * Common for Missionaries
 *
 * Common for Doctors of the Church.
 *
 * Common for Several Virgins
 *
 * Common for One Virgin
 *
 * Common for Several Holy Men and Women
 *
 * Common for One Holy Man or Woman
 *
 * Common for an Abbot
 *
 * Common for a Monk
 *
 * Common for a Nun
 *
 * Common for Religious
 *
 * Common for Those Who Practiced Works of Mercy
 *
 * Common for Educators
 *
 * Common for Holy Women
 */
export enum Common {
    BlessedVirginMaryAdvent = "BLESSED_VIRGIN_MARY__ADVENT",
    BlessedVirginMaryChristmas = "BLESSED_VIRGIN_MARY__CHRISTMAS",
    BlessedVirginMaryEaster = "BLESSED_VIRGIN_MARY__EASTER",
    BlessedVirginMaryOrdinaryTime = "BLESSED_VIRGIN_MARY__ORDINARY_TIME",
    DedicationAnniversaryInside = "DEDICATION_ANNIVERSARY__INSIDE",
    DedicationAnniversaryOutside = "DEDICATION_ANNIVERSARY__OUTSIDE",
    DoctorsOfTheChurch = "DOCTORS_OF_THE_CHURCH",
    MartyrsEasterOne = "MARTYRS__EASTER__ONE",
    MartyrsEasterSeveral = "MARTYRS__EASTER__SEVERAL",
    MartyrsMissionaryOne = "MARTYRS__MISSIONARY__ONE",
    MartyrsMissionarySeveral = "MARTYRS__MISSIONARY__SEVERAL",
    MartyrsOutsideEasterOne = "MARTYRS__OUTSIDE_EASTER__ONE",
    MartyrsOutsideEasterSeveral = "MARTYRS__OUTSIDE_EASTER__SEVERAL",
    MartyrsVirgin = "MARTYRS__VIRGIN",
    MartyrsWoman = "MARTYRS__WOMAN",
    None = "NONE",
    PastorsBishop = "PASTORS__BISHOP",
    PastorsFounderOne = "PASTORS__FOUNDER__ONE",
    PastorsFounderSeveral = "PASTORS__FOUNDER__SEVERAL",
    PastorsMissionary = "PASTORS__MISSIONARY",
    PastorsOne = "PASTORS__ONE",
    PastorsPopeOrBishop = "PASTORS__POPE_OR_BISHOP",
    PastorsSeveral = "PASTORS__SEVERAL",
    SaintMonk = "SAINT__MONK",
    SaintsAbbot = "SAINTS__ABBOT",
    SaintsAllOne = "SAINTS__ALL__ONE",
    SaintsAllSeveral = "SAINTS__ALL__SEVERAL",
    SaintsEducators = "SAINTS__EDUCATORS",
    SaintsHolyWomen = "SAINTS__HOLY_WOMEN",
    SaintsMercyWorks = "SAINTS__MERCY_WORKS",
    SaintsNun = "SAINTS__NUN",
    SaintsReligious = "SAINTS__RELIGIOUS",
    VirginsOne = "VIRGINS__ONE",
    VirginsSeveral = "VIRGINS__SEVERAL",
}

/**
 * Date definition supporting various date calculation methods.
 * Provides flexible ways to specify liturgical dates using different approaches.
 *
 * Regular date definition
 *
 * The start date of the range
 *
 * The end date of the range
 *
 * The date to compare against
 *
 * The date definition for this liturgical day.
 *
 * Simple month/day specification
 *
 * Date function calculation (Easter, Epiphany, etc.)
 *
 * Nth weekday of a specific month
 *
 * Last weekday of a specific month
 *
 * Inherited from the proper of time
 */
export type DateDef = {
    /**
     * The day of the month (1-31)
     */
    date?: number;
    /**
     * Optional day offset for adjustments
     */
    day_offset?: number | null;
    /**
     * The month (1-12)
     */
    month?: number;
    /**
     * The date function to calculate the base date
     */
    date_fn?: DateFn;
    /**
     * The day of the week (0=Sunday, 1=Monday, etc.)
     */
    day_of_week?: number;
    /**
     * Which occurrence of the weekday (1st, 2nd, 3rd, etc.)
     */
    nth_week_in_month?: number;
    /**
     * The day of the week to find the last occurrence of
     */
    last_day_of_week_in_month?: number;
}

/**
 * The date function to calculate the base date
 *
 * Date function for calculating liturgical dates.
 *
 * Represents movable feasts and special celebrations that require calculation
 * based on Easter or other variable dates.
 *
 * Monday after Pentecost.
 *
 * Sunday between January 2 and 8 (or January 6 if not transferred).
 *
 * February 2 (Candlemas).
 *
 * March 25 (may be transferred if in Holy Week or Easter Octave).
 *
 * Sunday before Easter.
 *
 * First Sunday after the Paschal Full Moon.
 *
 * Second Sunday of Easter.
 *
 * Saturday after the Second Sunday after Pentecost.
 *
 * Seventh Sunday after Easter.
 *
 * Thursday or Sunday after Trinity Sunday.
 *
 * June 24.
 *
 * June 29.
 *
 * August 6.
 *
 * August 15.
 *
 * September 14.
 *
 * November 1.
 *
 * December 8.
 */
export enum DateFn {
    AllSaints = "ALL_SAINTS",
    Annunciation = "ANNUNCIATION",
    Assumption = "ASSUMPTION",
    CorpusChristiSunday = "CORPUS_CHRISTI_SUNDAY",
    DivineMercySunday = "DIVINE_MERCY_SUNDAY",
    EasterSunday = "EASTER_SUNDAY",
    EpiphanySunday = "EPIPHANY_SUNDAY",
    ExaltationOfTheHolyCross = "EXALTATION_OF_THE_HOLY_CROSS",
    ImmaculateConceptionOfMary = "IMMACULATE_CONCEPTION_OF_MARY",
    ImmaculateHeartOfMary = "IMMACULATE_HEART_OF_MARY",
    MaryMotherOfTheChurch = "MARY_MOTHER_OF_THE_CHURCH",
    NativityOfJohnTheBaptist = "NATIVITY_OF_JOHN_THE_BAPTIST",
    PalmSunday = "PALM_SUNDAY",
    PentecostSunday = "PENTECOST_SUNDAY",
    PeterAndPaulApostles = "PETER_AND_PAUL_APOSTLES",
    PresentationOfTheLord = "PRESENTATION_OF_THE_LORD",
    Transfiguration = "TRANSFIGURATION",
}

/**
 * Single date exception
 *
 * The liturgical day date exception.
 * Represents a condition and the date to set when that condition is met.
 *
 * Multiple date exceptions
 */
export type DateDefException = {
    /**
     * The date to set when the condition is met
     */
    then: DateDefExtended;
    /**
     * The condition that triggers the exception
     */
    when: ExceptionCondition;
}

/**
 * The date to set when the condition is met
 *
 * Extended date definition supporting both regular dates and offset dates.
 * Provides flexibility for date calculations with optional adjustments.
 *
 * Simple month/day specification
 *
 * Date function calculation (Easter, Epiphany, etc.)
 *
 * Nth weekday of a specific month
 *
 * Last weekday of a specific month
 *
 * Inherited from the proper of time
 *
 * Date definition with offset
 *
 * Date definition with offset for adjustments.
 * Used when a date needs to be shifted by a specific number of days.
 */
export type DateDefExtended = {
    /**
     * The day of the month (1-31)
     */
    date?: number;
    /**
     * Optional day offset for adjustments
     *
     * The number of days to offset the date
     */
    day_offset?: number | null;
    /**
     * The month (1-12)
     */
    month?: number;
    /**
     * The date function to calculate the base date
     */
    date_fn?: DateFn;
    /**
     * The day of the week (0=Sunday, 1=Monday, etc.)
     */
    day_of_week?: number;
    /**
     * Which occurrence of the weekday (1st, 2nd, 3rd, etc.)
     */
    nth_week_in_month?: number;
    /**
     * The day of the week to find the last occurrence of
     */
    last_day_of_week_in_month?: number;
}

/**
 * The condition that triggers the exception
 *
 * Exception conditions that can trigger a date change.
 * Defines various conditions under which a date exception applies.
 *
 * If the date is between two specified dates
 *
 * If the date is the same as another specified date
 *
 * If the date falls on a specific day of the week
 */
export type ExceptionCondition = {
    /**
     * The start date of the range
     */
    from?: DateDef;
    /**
     * Whether the range is inclusive of the start date and the end date
     */
    inclusive?: boolean;
    /**
     * The end date of the range
     */
    to?: DateDef;
    /**
     * The date to compare against
     */
    date?: DateDef;
    /**
     * The day of the week to match
     */
    day_of_week?: number;
}

/**
 * Definition of a liturgical day with all its properties and configurations.
 * It represents a complete liturgical day definition that can be used
 * to generate calendar entries with proper precedence, colors, and entity associations.
 */
export type DayDefinition = {
    /**
     * Allow similar items that have the same rank and the same or lower precedence
     * to coexist with this liturgical day without being overwritten
     */
    allow_similar_rank_items?: boolean | null;
    /**
     * The liturgical color(s) of the liturgical day.
     *
     * **Deprecated:** Rely on the `titles` field of entities instead to determine the
     * liturgical color(s).
     */
    colors?: ColorsUnion;
    /**
     * The **Common** refers to a set of prayers, readings, and chants used for celebrating
     * saints or
     * feasts that belong to a specific category, such as martyrs, virgins, pastors, or the
     * Blessed
     * Virgin Mary.
     */
    commons_def?: CommonsDefUnion;
    /**
     * The custom locale ID for this date definition in this calendar
     */
    custom_locale_id?: null | string;
    /**
     * The date definition for this liturgical day
     */
    date_def?: DateDefClass | null;
    /**
     * The date definition exceptions (overrides for specific circumstances)
     */
    date_exceptions?: DateDefExceptions;
    /**
     * If this liturgical day must be removed from this calendar and from all parent calendars
     * in the final calendar generated by romcal
     */
    drop?: boolean | null;
    /**
     * The entities (Saints, Blessed, or Places) linked from the Entity catalog
     */
    entities?: EntityRef[] | null;
    /**
     * Holy days of obligation are days on which the faithful are expected to attend Mass
     * and engage in rest from work and recreation
     */
    is_holy_day_of_obligation?: boolean | null;
    /**
     * Specify if this liturgical day is optional within a specific liturgical calendar
     *
     * UNLY #14:
     * Memorials are either obligatory or optional; their observance is integrated into
     * the celebration of the occurring weekday in accordance with the norms set forth in the
     * General Instruction of the Roman Missal and of the Liturgy of the Hours
     *
     * Note: also used for the dedication of consecrated churches, which is an optional
     * solemnity
     * that should not overwrite the default weekday.
     */
    is_optional?: boolean | null;
    /**
     * The masses definitions for this liturgical day
     */
    masses?: MassesDefinitions | null;
    /**
     * The precedence type of the liturgical day
     */
    precedence?: Precedence | null;
    /**
     * The combined titles of all entities linked to this date definition
     */
    titles?: TitlesUnion;
}

/**
 * The liturgical color(s) of the liturgical day.
 *
 * **Deprecated:** Rely on the `titles` field of entities instead to determine the
 * liturgical color(s).
 */
export type ColorsUnion = Color[] | Color | null;

/**
 * The **Common** refers to a set of prayers, readings, and chants used for celebrating
 * saints or
 * feasts that belong to a specific category, such as martyrs, virgins, pastors, or the
 * Blessed
 * Virgin Mary.
 */
export type CommonsDefUnion = CommonDefinition[] | CommonDefinition | null;

/**
 * Common definition for simplified categorization.
 * Provides a simplified version of the Common enum for easier classification.
 *
 * No common.
 *
 * Dedication anniversary (in the Church that was Dedicated).
 *
 * Dedication anniversary (outside the Church that was Dedicated).
 *
 * Common of the Blessed Virgin Mary.
 *
 * Common for Martyrs.
 *
 * Common for Missionary Martyrs.
 *
 * Common for Virgin Martyrs.
 *
 * Common for Holy Woman Martyrs.
 *
 * Common for Pastors.
 *
 * Common for Popes.
 *
 * Common for Bishops.
 *
 * Common for Founders.
 *
 * Common for Missionaries.
 *
 * Common for Doctors of the Church.
 *
 * Common for Virgins.
 *
 * Common for Holy Men and Women.
 *
 * Common for Abbots.
 *
 * Common for Monks.
 *
 * Common for Nuns.
 *
 * Common for Religious.
 *
 * Common for Those Who Practiced Works of Mercy.
 *
 * Common for Educators.
 *
 * Common for Holy Women.
 */
export enum CommonDefinition {
    Abbots = "ABBOTS",
    Bishops = "BISHOPS",
    BlessedVirginMary = "BLESSED_VIRGIN_MARY",
    DedicationAnniversaryInside = "DEDICATION_ANNIVERSARY__INSIDE",
    DedicationAnniversaryOutside = "DEDICATION_ANNIVERSARY__OUTSIDE",
    DoctorsOfTheChurch = "DOCTORS_OF_THE_CHURCH",
    Educators = "EDUCATORS",
    Founders = "FOUNDERS",
    HolyWomen = "HOLY_WOMEN",
    Martyrs = "MARTYRS",
    MercyWorkers = "MERCY_WORKERS",
    Missionaries = "MISSIONARIES",
    MissionaryMartyrs = "MISSIONARY_MARTYRS",
    Monks = "MONKS",
    None = "NONE",
    Nuns = "NUNS",
    Pastors = "PASTORS",
    Popes = "POPES",
    Religious = "RELIGIOUS",
    Saints = "SAINTS",
    VirginMartyrs = "VIRGIN_MARTYRS",
    Virgins = "VIRGINS",
    WomanMartyrs = "WOMAN_MARTYRS",
}

/**
 * Simple month/day specification
 *
 * Date function calculation (Easter, Epiphany, etc.)
 *
 * Nth weekday of a specific month
 *
 * Last weekday of a specific month
 *
 * Inherited from the proper of time
 */
export type DateDefClass = {
    /**
     * The day of the month (1-31)
     */
    date?: number;
    /**
     * Optional day offset for adjustments
     */
    day_offset?: number | null;
    /**
     * The month (1-12)
     */
    month?: number;
    /**
     * The date function to calculate the base date
     */
    date_fn?: DateFn;
    /**
     * The day of the week (0=Sunday, 1=Monday, etc.)
     */
    day_of_week?: number;
    /**
     * Which occurrence of the weekday (1st, 2nd, 3rd, etc.)
     */
    nth_week_in_month?: number;
    /**
     * The day of the week to find the last occurrence of
     */
    last_day_of_week_in_month?: number;
}

/**
 * The date definition exceptions (overrides for specific circumstances)
 */
export type DateDefExceptions = DateDefException[] | DateDefException | null;

/**
 * A reference to an entity in the entity catalog.
 * Can either reference an existing entity by ID or define a custom entity with additional
 * properties.
 */
export type EntityRef = EntityOverride | string;

/**
 * Custom entity definition with additional properties specific to a liturgical day
 *
 * Custom entity definition that extends or overrides properties from the entity catalog.
 * Used when a liturgical day needs specific entity properties that differ from the base
 * entity.
 */
export type EntityOverride = {
    /**
     * The number of persons this entity represents (useful for groups of martyrs or saints)
     */
    count?: CountUnion;
    /**
     * Whether to hide titles when displaying this entity (useful when titles are already
     * included in the entity name)
     */
    hide_titles?: boolean | null;
    /**
     * The ID of the entity item (must reference an existing entity in the catalog)
     */
    id: string;
    /**
     * The custom titles for this entity in the context of this liturgical day
     */
    titles?: TitlesUnion;
}

export type CountUnion = CountEnum | number | null;

export enum CountEnum {
    Many = "MANY",
}

export type TitlesUnion = Title[] | CompoundTitle | null;

/**
 * Simple list of titles
 *
 * Titles and patronages associated with saints and blessed.
 * Represents the various ecclesiastical titles and patronages that can be assigned to
 * entities.
 */
export enum Title {
    Abbess = "ABBESS",
    Abbot = "ABBOT",
    Apostle = "APOSTLE",
    Archangel = "ARCHANGEL",
    Bishop = "BISHOP",
    CopatronOfCanada = "COPATRON_OF_CANADA",
    CopatronOfEurope = "COPATRON_OF_EUROPE",
    CopatronOfIreland = "COPATRON_OF_IRELAND",
    CopatronessOfEurope = "COPATRONESS_OF_EUROPE",
    CopatronessOfFrance = "COPATRONESS_OF_FRANCE",
    CopatronessOfIreland = "COPATRONESS_OF_IRELAND",
    CopatronessOfItalyAndEurope = "COPATRONESS_OF_ITALY_AND_EUROPE",
    CopatronessOfThePhilippines = "COPATRONESS_OF_THE_PHILIPPINES",
    Deacon = "DEACON",
    DoctorOfTheChurch = "DOCTOR_OF_THE_CHURCH",
    Empress = "EMPRESS",
    Evangelist = "EVANGELIST",
    FirstBishop = "FIRST_BISHOP",
    Hermit = "HERMIT",
    King = "KING",
    Martyr = "MARTYR",
    Missionary = "MISSIONARY",
    Monk = "MONK",
    MotherAndQueenOfChile = "MOTHER_AND_QUEEN_OF_CHILE",
    ParentsOfTheBlessedVirginMary = "PARENTS_OF_THE_BLESSED_VIRGIN_MARY",
    Patriarch = "PATRIARCH",
    PatronOfCanada = "PATRON_OF_CANADA",
    PatronOfEngland = "PATRON_OF_ENGLAND",
    PatronOfEurope = "PATRON_OF_EUROPE",
    PatronOfFrance = "PATRON_OF_FRANCE",
    PatronOfIreland = "PATRON_OF_IRELAND",
    PatronOfItaly = "PATRON_OF_ITALY",
    PatronOfOceania = "PATRON_OF_OCEANIA",
    PatronOfPoland = "PATRON_OF_POLAND",
    PatronOfRussia = "PATRON_OF_RUSSIA",
    PatronOfScotland = "PATRON_OF_SCOTLAND",
    PatronOfSpain = "PATRON_OF_SPAIN",
    PatronOfTheCityOfLyon = "PATRON_OF_THE_CITY_OF_LYON",
    PatronOfTheClergyOfTheArchdioceseOfLyon = "PATRON_OF_THE_CLERGY_OF_THE_ARCHDIOCESE_OF_LYON",
    PatronOfTheCzechNation = "PATRON_OF_THE_CZECH_NATION",
    PatronOfTheDiocese = "PATRON_OF_THE_DIOCESE",
    PatronOfWales = "PATRON_OF_WALES",
    PatronessOfAlsace = "PATRONESS_OF_ALSACE",
    PatronessOfArgentina = "PATRONESS_OF_ARGENTINA",
    PatronessOfBrazil = "PATRONESS_OF_BRAZIL",
    PatronessOfCostaRica = "PATRONESS_OF_COSTA_RICA",
    PatronessOfHungary = "PATRONESS_OF_HUNGARY",
    PatronessOfPuertoRico = "PATRONESS_OF_PUERTO_RICO",
    PatronessOfSlovakia = "PATRONESS_OF_SLOVAKIA",
    PatronessOfTheAmericas = "PATRONESS_OF_THE_AMERICAS",
    PatronessOfThePhilippines = "PATRONESS_OF_THE_PHILIPPINES",
    PatronessOfTheProvinceOfQuebec = "PATRONESS_OF_THE_PROVINCE_OF_QUEBEC",
    PatronessOfTheUsa = "PATRONESS_OF_THE_USA",
    Pilgrim = "PILGRIM",
    Pope = "POPE",
    Priest = "PRIEST",
    PrincipalPatronOfTheDiocese = "PRINCIPAL_PATRON_OF_THE_DIOCESE",
    Prophet = "PROPHET",
    ProtoMartyrOfOceania = "PROTO_MARTYR_OF_OCEANIA",
    Queen = "QUEEN",
    QueenOfPoland = "QUEEN_OF_POLAND",
    Religious = "RELIGIOUS",
    SecondPatronOfTheDiocese = "SECOND_PATRON_OF_THE_DIOCESE",
    SlavicMissionary = "SLAVIC_MISSIONARY",
    SpouseOfTheBlessedVirginMary = "SPOUSE_OF_THE_BLESSED_VIRGIN_MARY",
    TheFirstMartyr = "THE_FIRST_MARTYR",
    Virgin = "VIRGIN",
}

/**
 * Compound title definition with append/prepend operations
 *
 * Compound title definition for combining multiple titles.
 * Allows adding titles to the beginning or end of an existing title list.
 */
export type CompoundTitle = {
    /**
     * The title(s) to add to the end of the existing list of title(s)
     */
    append?: Title[] | null;
    /**
     * The title(s) to add to the beginning of the existing list of title(s)
     */
    prepend?: Title[] | null;
}

/**
 * All mass definitions for a liturgical day
 */
export type MassesDefinitions = {
    /**
     * Celebration of the Passion - special celebration of Christ's passion
     */
    celebration_of_the_passion?: MassCycleDefinition;
    /**
     * Chrism Mass - Mass where holy oils are blessed, typically on Holy Thursday morning
     */
    chrism_mass?: MassCycleDefinition;
    /**
     * Day Mass - regular Mass celebrated during the day
     */
    day_mass?: MassCycleDefinition;
    /**
     * Easter Vigil - the most important Mass of the liturgical year, celebrated on Holy
     * Saturday night
     */
    easter_vigil?: MassCycleDefinition;
    /**
     * Evening Mass of the Lord's Supper - Mass celebrated on Holy Thursday evening
     */
    evening_mass_of_the_lords_supper?: MassCycleDefinition;
    /**
     * Mass at Dawn - Mass celebrated at dawn, particularly on Easter Sunday
     */
    mass_at_dawn?: MassCycleDefinition;
    /**
     * Mass of the Passion - Mass focusing on Christ's passion, beginning with the procession
     * with palms
     */
    mass_of_the_passion?: MassCycleDefinition;
    /**
     * Morning Mass - Mass celebrated in the morning
     */
    morning_mass?: MassCycleDefinition;
    /**
     * Night Mass - Mass celebrated during the night hours
     */
    night_mass?: MassCycleDefinition;
    /**
     * Previous Evening Mass - Mass celebrated the evening before a major feast
     */
    previous_evening_mass?: MassCycleDefinition;
}

/**
 * Celebration of the Passion - special celebration of Christ's passion
 *
 * Mass contents for a specific mass time, organized by liturgical cycle
 *
 * Chrism Mass - Mass where holy oils are blessed, typically on Holy Thursday morning
 *
 * Day Mass - regular Mass celebrated during the day
 *
 * Easter Vigil - the most important Mass of the liturgical year, celebrated on Holy
 * Saturday night
 *
 * Evening Mass of the Lord's Supper - Mass celebrated on Holy Thursday evening
 *
 * Mass at Dawn - Mass celebrated at dawn, particularly on Easter Sunday
 *
 * Mass of the Passion - Mass focusing on Christ's passion, beginning with the procession
 * with palms
 *
 * Morning Mass - Mass celebrated in the morning
 *
 * Night Mass - Mass celebrated during the night hours
 *
 * Previous Evening Mass - Mass celebrated the evening before a major feast
 */
export type MassCycleDefinition = {
    /**
     * Invariant content that applies to all cycles
     */
    invariant?: MassContent;
    /**
     * Year 1 of the weekday cycle (Cycle I)
     */
    year_1?: MassContent;
    /**
     * Year 2 of the weekday cycle (Cycle II)
     */
    year_2?: MassContent;
    /**
     * Year A of the Sunday cycle
     */
    year_a?: MassContent;
    /**
     * Combined years A and B of the Sunday cycle
     */
    year_a_b?: MassContent;
    /**
     * Combined years A and C of the Sunday cycle
     */
    year_a_c?: MassContent;
    /**
     * Year B of the Sunday cycle
     */
    year_b?: MassContent;
    /**
     * Combined years B and C of the Sunday cycle
     */
    year_b_c?: MassContent;
    /**
     * Year C of the Sunday cycle
     */
    year_c?: MassContent;
}

/**
 * Invariant content that applies to all cycles
 *
 * Content of a mass for a specific liturgical cycle
 * Maps mass parts (readings, psalms, prayers, antiphons, etc.) to their texts
 *
 * Year 1 of the weekday cycle (Cycle I)
 *
 * Year 2 of the weekday cycle (Cycle II)
 *
 * Year A of the Sunday cycle
 *
 * Combined years A and B of the Sunday cycle
 *
 * Combined years A and C of the Sunday cycle
 *
 * Year B of the Sunday cycle
 *
 * Combined years B and C of the Sunday cycle
 *
 * Year C of the Sunday cycle
 */
export type MassContent = {
    /**
     * Alleluia - acclamation before the Gospel
     */
    alleluia?: string;
    /**
     * Canticle - biblical canticle
     */
    canticle?: string;
    /**
     * Collect - opening prayer of the Mass
     */
    collect?: string;
    /**
     * Communion Antiphon - chant during communion
     */
    communion_antiphon?: string;
    /**
     * Canticle 3 (Easter Vigil)
     */
    easter_vigil_canticle_3?: string;
    /**
     * Canticle 5 (Easter Vigil)
     */
    easter_vigil_canticle_5?: string;
    /**
     * Epistle - reading from the epistles (Easter Vigil)
     */
    easter_vigil_epistle?: string;
    /**
     * Psalm 2 (Easter Vigil)
     */
    easter_vigil_psalm_2?: string;
    /**
     * Psalm 4 (Easter Vigil)
     */
    easter_vigil_psalm_4?: string;
    /**
     * Psalm 6 (Easter Vigil)
     */
    easter_vigil_psalm_6?: string;
    /**
     * Psalm 7 (Easter Vigil)
     */
    easter_vigil_psalm_7?: string;
    /**
     * Reading 3 - third reading (Easter Vigil)
     */
    easter_vigil_reading_3?: string;
    /**
     * Reading 4 - fourth reading (Easter Vigil)
     */
    easter_vigil_reading_4?: string;
    /**
     * Reading 5 - fifth reading (Easter Vigil)
     */
    easter_vigil_reading_5?: string;
    /**
     * Reading 6 - sixth reading (Easter Vigil)
     */
    easter_vigil_reading_6?: string;
    /**
     * Reading 7 - seventh reading (Easter Vigil)
     */
    easter_vigil_reading_7?: string;
    /**
     * Entrance Antiphon - opening chant of the Mass
     */
    entrance_antiphon?: string;
    /**
     * Gospel - reading from the Gospels
     */
    gospel?: string;
    /**
     * Messianic entry reading (during the procession with palms, before the Mass of the Passion)
     */
    messianic_entry?: string;
    /**
     * Prayer after Communion - concluding prayer
     */
    prayer_after_communion?: string;
    /**
     * Prayer over the Offerings - prayer during the offertory
     */
    prayer_over_the_offerings?: string;
    /**
     * Prayer over the People - blessing over the congregation
     */
    prayer_over_the_people?: string;
    /**
     * Preface - introduction to the Eucharistic Prayer
     */
    preface?: string;
    /**
     * Psalm - responsorial psalm
     */
    psalm?: string;
    /**
     * Reading 1 - first reading (usually from the Old Testament)
     */
    reading_1?: string;
    /**
     * Reading 2 - second reading (usually from the New Testament)
     */
    reading_2?: string;
    /**
     * Sequence - special chant on certain feasts
     */
    sequence?: string;
    /**
     * Solemn Blessing - special blessing on certain occasions
     */
    solemn_blessing?: string;
}

/**
 * 1 - The Paschal Triduum of the Passion and Resurrection of the Lord.
 *
 * 2 - The Nativity of the Lord, the Epiphany, the Ascension, or Pentecost.
 *
 * 2 - A Sunday of Advent, Lent, or Easter.
 *
 * 2 - Ash Wednesday.
 *
 * 2 - A weekday of Holy Week from Monday up to and including Thursday.
 *
 * 2 - A day within the Octave of Easter.
 *
 * 3 - A Solemnity inscribed in the General Calendar, whether of the Lord, of the Blessed
 * Virgin Mary, or of a Saint.
 *
 * 3 - The Commemoration of All the Faithful Departed.
 *
 * 4a - A proper Solemnity of the principal Patron of the place, city, or state.
 *
 * 4b - The Solemnity of the dedication and of the anniversary of the dedication of the own
 * church.
 *
 * 4c - The solemnity of the title of the own church.
 *
 * 4d - A Solemnity either of the Title or of the Founder or of the principal Patron of an
 * Order or Congregation.
 *
 * 5 - A Feast of the Lord inscribed in the General Calendar.
 *
 * 6 - A Sunday of Christmas Time or a Sunday in Ordinary Time.
 *
 * 7 - A Feast of the Blessed Virgin Mary or of a Saint in the General Calendar.
 *
 * 8a - The Proper Feast of the principal Patron of the diocese.
 *
 * 8b - The Proper Feast of the anniversary of the dedication of the cathedral church.
 *
 * 8c - The Proper Feast of the principal Patron of a region or province, or a country, or
 * of a wider territory.
 *
 * 8d - The Proper Feast of the Title, Founder, or principal Patron of an Order or
 * Congregation.
 *
 * 8e - Other Feast, proper to an individual church.
 *
 * 8f - Other Proper Feast inscribed in the Calendar of each diocese or Order or
 * Congregation.
 *
 * 9 - Privileged Weekday.
 *
 * 10 - Obligatory Memorials in the General Calendar.
 *
 * 11a - Proper Obligatory Memorial of a secondary Patron of the place, diocese, region, or
 * religious province.
 *
 * 11b - Other Proper Obligatory Memorial inscribed in the Calendar of each diocese, or
 * Order or congregation.
 *
 * 12 - Optional Memorial.
 *
 * 13 - Weekday.
 *
 * Liturgical precedence levels for determining which celebration takes priority.
 * Defines the hierarchical order of liturgical celebrations according to UNLY norms.
 *
 * The liturgical precedence for this liturgical day.
 */
export enum Precedence {
    AshWednesday2 = "ASH_WEDNESDAY_2",
    CommemorationOfAllTheFaithfulDeparted3 = "COMMEMORATION_OF_ALL_THE_FAITHFUL_DEPARTED_3",
    GeneralFeast7 = "GENERAL_FEAST_7",
    GeneralLordFeast5 = "GENERAL_LORD_FEAST_5",
    GeneralMemorial10 = "GENERAL_MEMORIAL_10",
    GeneralSolemnity3 = "GENERAL_SOLEMNITY_3",
    OptionalMemorial12 = "OPTIONAL_MEMORIAL_12",
    PrivilegedSunday2 = "PRIVILEGED_SUNDAY_2",
    PrivilegedWeekday9 = "PRIVILEGED_WEEKDAY_9",
    ProperFeast8F = "PROPER_FEAST_8F",
    ProperFeastDedicationOfTheCathedralChurch8B = "PROPER_FEAST__DEDICATION_OF_THE_CATHEDRAL_CHURCH_8B",
    ProperFeastPrincipalPatronOfADiocese8A = "PROPER_FEAST__PRINCIPAL_PATRON_OF_A_DIOCESE_8A",
    ProperFeastPrincipalPatronOfARegion8C = "PROPER_FEAST__PRINCIPAL_PATRON_OF_A_REGION_8C",
    ProperFeastTitleOrFounderOrPrimaryPatronOfAReligiousOrg8D = "PROPER_FEAST__TITLE_OR_FOUNDER_OR_PRIMARY_PATRON_OF_A_RELIGIOUS_ORG_8D",
    ProperFeastToAnIndividualChurch8E = "PROPER_FEAST__TO_AN_INDIVIDUAL_CHURCH_8E",
    ProperMemorial11B = "PROPER_MEMORIAL_11B",
    ProperMemorialSecondPatron11A = "PROPER_MEMORIAL__SECOND_PATRON_11A",
    ProperOfTimeSolemnity2 = "PROPER_OF_TIME_SOLEMNITY_2",
    ProperSolemnityDedicationOfTheOwnChurch4B = "PROPER_SOLEMNITY__DEDICATION_OF_THE_OWN_CHURCH_4B",
    ProperSolemnityPrincipalPatron4A = "PROPER_SOLEMNITY__PRINCIPAL_PATRON_4A",
    ProperSolemnityTitleOfTheOwnChurch4C = "PROPER_SOLEMNITY__TITLE_OF_THE_OWN_CHURCH_4C",
    ProperSolemnityTitleOrFounderOrPrimaryPatronOfAReligiousOrg4D = "PROPER_SOLEMNITY__TITLE_OR_FOUNDER_OR_PRIMARY_PATRON_OF_A_RELIGIOUS_ORG_4D",
    Triduum1 = "TRIDUUM_1",
    UnprivilegedSunday6 = "UNPRIVILEGED_SUNDAY_6",
    Weekday13 = "WEEKDAY_13",
    WeekdayOfEasterOctave2 = "WEEKDAY_OF_EASTER_OCTAVE_2",
    WeekdayOfHolyWeek2 = "WEEKDAY_OF_HOLY_WEEK_2",
}

export type Entities = Entity[] | { [key: string]: Entity } | null;

export type Entity = {
    /**
     * Internal notes (not serialized).
     */
    _todo?: string[] | null;
    /**
     * The canonization level of a person.
     */
    canonization_level?: CanonizationLevel | null;
    /**
     * Number of person that this definition represent.
     * It could be set as 'many' if the number is not defined.
     */
    count?: CountUnion;
    /**
     * Date of Beatification, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD'
     * format),
     * or an object describing date range, multiple possible date, or a century.
     */
    date_of_beatification?: DateOfBeatificationUnion;
    /**
     * Specify whether an approximate indicator should be added, when the date is displayed.
     * For example in English: 'c. 201'.
     */
    date_of_beatification_is_approximative?: boolean | null;
    /**
     * Date of Birth, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
     * or an object describing date range, multiple possible date, or a century.
     */
    date_of_birth?: DateOfBeatificationUnion;
    /**
     * Specify whether an approximate indicator should be added, when the date is displayed.
     * For example in English: 'c. 201'.
     */
    date_of_birth_is_approximative?: boolean | null;
    /**
     * Date of Canonization, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
     * or an object describing date range, multiple possible date, or a century.
     */
    date_of_canonization?: DateOfBeatificationUnion;
    /**
     * Specify whether an approximate indicator should be added, when the date is displayed.
     * For example in English: 'c. 201'.
     */
    date_of_canonization_is_approximative?: boolean | null;
    /**
     * Date of Death, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
     * or an object describing date range, multiple possible date, or a century.
     */
    date_of_death?: DateOfBeatificationUnion;
    /**
     * Specify whether an approximate indicator should be added, when the date is displayed.
     * For example in English: 'c. 201'.
     */
    date_of_death_is_approximative?: boolean | null;
    /**
     * Date of Dedication of a church, basilica, or cathedral (or other place of worship),
     * as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
     * or an object describing date range, multiple possible date, or a century.
     */
    date_of_dedication?: DateOfBeatificationUnion;
    /**
     * The full name of the entity.
     */
    fullname?: null | string;
    /**
     * Specify if the canonization level should not be displayed.
     * It's generally the case when the canonization are already included in the name.
     */
    hide_canonization_level?: boolean | null;
    /**
     * Specify if the titles should not be displayed.
     * It's generally the case when titles are already included in the name.
     */
    hide_titles?: boolean | null;
    /**
     * The unique identifier of the entity
     */
    id?: null | string;
    /**
     * The short name of the entity, without the canonization level and titles.
     */
    name?: null | string;
    /**
     * Determine if the Saint or the Blessed is a male or a female.
     */
    sex?: Sex | null;
    /**
     * Sources for the information about this entity
     */
    sources?: string[] | null;
    /**
     * Titles of the Saint or the Blessed
     */
    titles?: Title[] | null;
    /**
     * The type of the entity.
     *
     * Defaults to `EntityType::Person`.
     */
    type?: EntityType | null;
}

/**
 * Beatified person (Blessed) - first step toward sainthood
 *
 * Canonized person (Saint) - fully recognized as a saint
 */
export enum CanonizationLevel {
    Blessed = "BLESSED",
    Saint = "SAINT",
}

export type DateOfBeatificationUnion = SaintDateDef | number | null | string;

/**
 * Date range between two dates
 *
 * Multiple alternative dates (any one of them)
 *
 * Century specification (e.g., 12 for 12th century)
 */
export type SaintDateDef = {
    /**
     * The date range (start and end dates)
     */
    between?: SaintDate[];
    /**
     * The list of alternative dates
     */
    or?: SaintDate[];
    /**
     * The century number
     */
    century?: number;
}

/**
 * Single date specification
 *
 * Saint date representation with different precision levels.
 * Supports year-only, year-month, or full date specifications.
 */
export type SaintDate = number | string;

/**
 * Male person
 *
 * Female person
 */
export enum Sex {
    Female = "FEMALE",
    Male = "MALE",
}

/**
 * A person (saint, blessed, or other individual)
 *
 * A place (shrine, city, or geographical location)
 *
 * An event (historical or liturgical occurrence)
 */
export enum EntityType {
    Event = "EVENT",
    Person = "PERSON",
    Place = "PLACE",
}

/**
 * Information about a mass celebration for a liturgical day.
 * Contains the type of mass and its localized name.
 */
export type MassInfo = {
    /**
     * The localized name of the mass type (translation key in snake_case)
     */
    name: string;
    /**
     * The type of mass (e.g., DayMass, EasterVigil, etc.)
     * Serialized as SCREAMING_SNAKE_CASE (e.g., "DAY_MASS")
     */
    type: MassTime;
}

/**
 * The type of mass (e.g., DayMass, EasterVigil, etc.)
 * Serialized as SCREAMING_SNAKE_CASE (e.g., "DAY_MASS")
 *
 * Times of Mass celebrations in the liturgical calendar.
 * Different Masses are celebrated at various times and occasions throughout the liturgical
 * year.
 *
 * Easter Vigil - the most important Mass of the liturgical year, celebrated on Holy
 * Saturday night
 *
 * Previous Evening Mass - Mass celebrated the evening before a major feast
 *
 * Night Mass - Mass celebrated during the night hours
 *
 * Mass at Dawn - Mass celebrated at dawn, particularly on Easter Sunday
 *
 * Morning Mass - Mass celebrated in the morning
 *
 * Mass of the Passion - Mass focusing on Christ's passion, beginning with the procession
 * with palms
 *
 * Celebration of the Passion - special celebration of Christ's passion
 *
 * Day Mass - regular Mass celebrated during the day
 *
 * Chrism Mass - Mass where holy oils are blessed, typically on Holy Thursday morning
 *
 * Evening Mass of the Lord's Supper - Mass celebrated on Holy Thursday evening
 */
export enum MassTime {
    CelebrationOfThePassion = "celebration_of_the_passion",
    ChrismMass = "chrism_mass",
    DayMass = "day_mass",
    EasterVigil = "easter_vigil",
    EveningMassOfTheLordsSupper = "evening_mass_of_the_lords_supper",
    MassAtDawn = "mass_at_dawn",
    MassOfThePassion = "mass_of_the_passion",
    MorningMass = "morning_mass",
    NightMass = "night_mass",
    PreviousEveningMass = "previous_evening_mass",
}

/**
 * Metadata for a calendar.
 * Contains essential information about the calendar's type and jurisdiction.
 *
 * Metadata for localized resources.
 * Contains all the localized strings and configurations for a specific locale.
 */
export type ResourcesMetadata = {
    /**
     * The jurisdiction of the calendar
     */
    jurisdiction?: CalendarJurisdiction;
    /**
     * The type of the calendar
     */
    type?: CalendarType;
    /**
     * Liturgical color names in the locale language
     */
    colors?: LocaleColors | null;
    /**
     * Liturgical cycle names in the locale language
     */
    cycles?: CyclesMetadata | null;
    /**
     * Month names (January, February, etc.) in the locale language
     */
    months?: { [key: string]: string } | null;
    /**
     * Format for displaying ordinal numbers (defaults to Numeric if not specified)
     */
    ordinal_format?: OrdinalFormat | null;
    /**
     * Ordinal numbers as words (first, second, third, etc.) in the locale language
     */
    ordinals_letters?: { [key: string]: string } | null;
    /**
     * Ordinal numbers as numeric with suffix (1st, 2nd, 3rd, etc.) in the locale language
     */
    ordinals_numeric?: { [key: string]: string } | null;
    /**
     * Liturgical period names in the locale language
     */
    periods?: PeriodsMetadata | null;
    /**
     * Liturgical rank names in the locale language
     */
    ranks?: RanksMetadata | null;
    /**
     * Liturgical season names and descriptions in the locale language
     */
    seasons?: SeasonsMetadata | null;
    /**
     * Weekday names (Sunday, Monday, etc.) in the locale language
     */
    weekdays?: { [key: string]: string } | null;
}

/**
 * Liturgical color names in the locale language.
 * Provides localized names for each liturgical color.
 */
export type LocaleColors = {
    /**
     * Black color name in the locale language
     */
    black?: null | string;
    /**
     * Gold color name in the locale language
     */
    gold?: null | string;
    /**
     * Green color name in the locale language
     */
    green?: null | string;
    /**
     * Purple color name in the locale language
     */
    purple?: null | string;
    /**
     * Red color name in the locale language
     */
    red?: null | string;
    /**
     * Rose color name in the locale language
     */
    rose?: null | string;
    /**
     * White color name in the locale language
     */
    white?: null | string;
}

/**
 * Liturgical cycle names in the locale language.
 */
export type CyclesMetadata = {
    /**
     * Proper of Saints cycle name
     */
    proper_of_saints?: null | string;
    /**
     * Proper of Time cycle name
     */
    proper_of_time?: null | string;
    /**
     * Psalter Week 1 cycle name
     */
    psalter_week_1?: null | string;
    /**
     * Psalter Week 2 cycle name
     */
    psalter_week_2?: null | string;
    /**
     * Psalter Week 3 cycle name
     */
    psalter_week_3?: null | string;
    /**
     * Psalter Week 4 cycle name
     */
    psalter_week_4?: null | string;
    /**
     * Sunday Year A cycle name
     */
    sunday_year_a?: null | string;
    /**
     * Sunday Year B cycle name
     */
    sunday_year_b?: null | string;
    /**
     * Sunday Year C cycle name
     */
    sunday_year_c?: null | string;
    /**
     * Weekday Year 1 cycle name
     */
    weekday_year_1?: null | string;
    /**
     * Weekday Year 2 cycle name
     */
    weekday_year_2?: null | string;
}

/**
 * The jurisdiction of the calendar
 *
 * The jurisdiction of the calendar.
 * Determines whether the calendar follows ecclesiastical or civil authority.
 *
 * Calendar under ecclesiastical authority (Church)
 *
 * Calendar under civil authority (State)
 */
export enum CalendarJurisdiction {
    Civil = "CIVIL",
    Ecclesiastical = "ECCLESIASTICAL",
}

/**
 * Ordinals displayed as words
 *
 * Ordinals displayed as numbers with suffixes (default)
 */
export enum OrdinalFormat {
    Letters = "letters",
    Numeric = "numeric",
}

/**
 * Liturgical period names in the locale language.
 */
export type PeriodsMetadata = {
    /**
     * Christmas Octave period name
     */
    christmas_octave?: null | string;
    /**
     * Christmas to Presentation of the Lord period name
     */
    christmas_to_presentation_of_the_lord?: null | string;
    /**
     * Days before Epiphany period name
     */
    days_before_epiphany?: null | string;
    /**
     * Days from Epiphany period name
     */
    days_from_epiphany?: null | string;
    /**
     * Early Ordinary Time period name
     */
    early_ordinary_time?: null | string;
    /**
     * Easter Octave period name
     */
    easter_octave?: null | string;
    /**
     * Holy Week period name
     */
    holy_week?: null | string;
    /**
     * Late Ordinary Time period name
     */
    late_ordinary_time?: null | string;
    /**
     * Paschal Triduum period name
     */
    paschal_triduum?: null | string;
    /**
     * Presentation of the Lord to Holy Thursday period name
     */
    presentation_of_the_lord_to_holy_thursday?: null | string;
}

/**
 * Liturgical rank names in the locale language.
 */
export type RanksMetadata = {
    /**
     * Feast rank name
     */
    feast?: null | string;
    /**
     * Memorial rank name
     */
    memorial?: null | string;
    /**
     * Optional memorial rank name
     */
    optional_memorial?: null | string;
    /**
     * Solemnity rank name
     */
    solemnity?: null | string;
    /**
     * Sunday rank name
     */
    sunday?: null | string;
    /**
     * Weekday rank name
     */
    weekday?: null | string;
}

/**
 * Liturgical season names and descriptions in the locale language.
 * Provides localized names for each liturgical season and their components.
 */
export type SeasonsMetadata = {
    /**
     * Advent season names and descriptions
     */
    advent?: AdventSeason | null;
    /**
     * Christmas Time season names and descriptions
     */
    christmas_time?: ChristmasTimeSeason | null;
    /**
     * Easter Time season names and descriptions
     */
    easter_time?: EasterTimeSeason | null;
    /**
     * Lent season names and descriptions
     */
    lent?: LentSeason | null;
    /**
     * Ordinary Time season names and descriptions
     */
    ordinary_time?: OrdinaryTimeSeason | null;
    /**
     * Paschal Triduum season names and descriptions
     */
    paschal_triduum?: PaschalTriduumSeason | null;
}

/**
 * Advent season localized names and descriptions.
 * Provides specific terminology for the Advent season in the locale language.
 */
export type AdventSeason = {
    /**
     * Privileged weekday terminology during Advent
     */
    privileged_weekday?: null | string;
    /**
     * General season name for Advent
     */
    season?: null | string;
    /**
     * Sunday terminology during Advent
     */
    sunday?: null | string;
    /**
     * Weekday terminology during Advent
     */
    weekday?: null | string;
}

/**
 * Christmas Time season localized names and descriptions.
 */
export type ChristmasTimeSeason = {
    /**
     * After Epiphany terminology
     */
    after_epiphany?: null | string;
    /**
     * Before Epiphany terminology
     */
    before_epiphany?: null | string;
    /**
     * Day terminology during Christmas Time
     */
    day?: null | string;
    /**
     * Octave terminology during Christmas Time
     */
    octave?: null | string;
    /**
     * General season name for Christmas Time
     */
    season?: null | string;
    /**
     * Second Sunday after Christmas terminology
     */
    second_sunday_after_christmas?: null | string;
}

/**
 * Easter Time season localized names and descriptions.
 */
export type EasterTimeSeason = {
    /**
     * Octave terminology during Easter Time
     */
    octave?: null | string;
    /**
     * General season name for Easter Time
     */
    season?: null | string;
    /**
     * Sunday terminology during Easter Time
     */
    sunday?: null | string;
    /**
     * Weekday terminology during Easter Time
     */
    weekday?: null | string;
}

/**
 * Lent season localized names and descriptions.
 */
export type LentSeason = {
    /**
     * Day after Ash Wednesday terminology
     */
    day_after_ash_wed?: null | string;
    /**
     * Holy Week day terminology
     */
    holy_week_day?: null | string;
    /**
     * General season name for Lent
     */
    season?: null | string;
    /**
     * Sunday terminology during Lent
     */
    sunday?: null | string;
    /**
     * Weekday terminology during Lent
     */
    weekday?: null | string;
}

/**
 * Ordinary Time season localized names and descriptions.
 */
export type OrdinaryTimeSeason = {
    /**
     * General season name for Ordinary Time
     */
    season?: null | string;
    /**
     * Sunday terminology during Ordinary Time
     */
    sunday?: null | string;
    /**
     * Weekday terminology during Ordinary Time
     */
    weekday?: null | string;
}

/**
 * Paschal Triduum season localized names and descriptions.
 */
export type PaschalTriduumSeason = {
    /**
     * General season name for Paschal Triduum
     */
    season?: null | string;
}

/**
 * The type of the calendar
 *
 * The type of the calendar.
 * Defines the scope and authority level of the liturgical calendar.
 *
 * General Roman Calendar (universal)
 *
 * Regional calendar (multiple countries)
 *
 * National calendar (single country)
 *
 * Archdiocesan calendar
 *
 * Diocesan calendar
 *
 * City calendar
 *
 * Parish calendar
 *
 * General religious community calendar
 *
 * Regional religious community calendar
 *
 * Local religious community calendar
 *
 * Other specialized calendar
 */
export enum CalendarType {
    Archdiocese = "ARCHDIOCESE",
    City = "CITY",
    Country = "COUNTRY",
    Diocese = "DIOCESE",
    GeneralCommunity = "GENERAL_COMMUNITY",
    GeneralRoman = "GENERAL_ROMAN",
    LocalCommunity = "LOCAL_COMMUNITY",
    Other = "OTHER",
    Parish = "PARISH",
    Region = "REGION",
    RegionalCommunity = "REGIONAL_COMMUNITY",
}

/**
 * Represents the differences between a liturgical day definition and its parent definition.
 * This is a lightweight structure that only contains fields that can be overridden.
 */
export type ParentOverride = {
    /**
     * The allow_similar_rank_items flag if it was changed
     */
    allow_similar_rank_items?: boolean | null;
    /**
     * The colors if they were changed
     */
    colors?: ColorInfo[] | null;
    /**
     * The commons definition if it was changed
     */
    commons_def?: CommonDefinition[] | null;
    /**
     * The date definition if it was changed
     */
    date_def?: DateDefClass | null;
    /**
     * The date exceptions if they were changed
     */
    date_exceptions?: DateDefException[] | null;
    /**
     * The ID of the calendar from which this override originates
     */
    from_calendar_id: string;
    /**
     * The is_holy_day_of_obligation flag if it was changed
     */
    is_holy_day_of_obligation?: boolean | null;
    /**
     * The is_optional flag if it was changed
     */
    is_optional?: boolean | null;
    /**
     * The precedence if it was changed
     */
    precedence?: Precedence | null;
    /**
     * The rank if it was changed
     */
    rank?: Rank | null;
    /**
     * The titles if they were changed
     */
    titles?: TitlesUnion;
}

/**
 * Solemnities are counted among the most important days, whose celebration
 * begins with First Vespers (Evening Prayer I) on the preceding day. Some Solemnities
 * are also endowed with their own Vigil Mass, which is to be used on the evening of the
 * preceding day, if an evening Mass is celebrated. (UNLY #11)
 *
 * On the first day of each week, which is known as the Day of the Lord or the Lord's
 * Day, the Church, by an apostolic tradition that draws its origin from the very day of
 * the Resurrection of Christ, celebrates the Paschal Mystery. Hence, Sunday must be
 * considered the primordial feast day. (UNLY #4)
 *
 * Feasts are celebrated within the limits of the natural day; accordingly they have
 * no First Vespers (Evening Prayer I), except in the case of Feasts of the Lord that fall
 * on a Sunday in Ordinary Time or in Christmas Time and which replace the Sunday
 * Office. (UNLY #13)
 *
 * **Obligatory memorials** are liturgical commemorations of saints, events, or aspects of
 * the
 * faith. Their observance is mandatory and integrated into the celebration of the occurring
 * weekday, following the liturgical norms outlined in the General Instruction of the Roman
 * Missal
 * and the Liturgy of the Hours.
 * When an **obligatory memorial** falls on a weekday during the liturgical season of Lent
 * or a
 * privileged weekday of Advent, it must only be celebrated as an **optional memorial**, as
 * Lent
 * and Advent have their own specific liturgical observances that take precedence.
 *
 * **Optional memorials** are liturgical commemorations of saints, events, or aspects of the
 * faith, but they are not obligatory.
 * Their observance is integrated into the celebration of the occurring weekday, adhering to
 * the
 * liturgical norms provided in the General Instruction of the Roman Missal and the Liturgy
 * of
 * the Hours.
 * In cases where multiple **optional memorials** are designated on the same day in the
 * liturgical
 * calendar, only one of them may be celebrated, and the others must be omitted (UNLY #14).
 * This allows for some flexibility in choosing which optional memorial to commemorate when
 * multiple options are available.
 *
 * The days of the week that follow Sunday are called weekdays; however, they are
 * celebrated differently according to the importance of each.
 *
 * a. Ash Wednesday and the weekdays of Holy Week, from Monday up to and including
 * Thursday, take precedence over all other celebrations.
 * b. The weekdays of Advent from 17 December up to and including 24 December
 * and all the weekdays of Lent have precedence over Obligatory Memorials.
 * c. Other weekdays give way to all Solemnities and Feasts and are combined with
 * Memorials.
 *
 * (UNLY #16)
 *
 * Liturgical rank indicating the importance and celebration style of a liturgical day
 *
 * The liturgical rank for this liturgical day.
 */
export enum Rank {
    Feast = "FEAST",
    Memorial = "MEMORIAL",
    OptionalMemorial = "OPTIONAL_MEMORIAL",
    Solemnity = "SOLEMNITY",
    Sunday = "SUNDAY",
    Weekday = "WEEKDAY",
}

/**
 * Configuration options for "particular" (local/diocesan) calendars.
 *
 * In liturgical terminology, a "particular" calendar is one that applies to a specific
 * region, diocese, or religious community, as opposed to the General Roman Calendar
 * which applies universally.
 *
 * These settings can override or extend the default Romcal configuration or any parent
 * calendar configuration.
 */
export type ParticularConfig = {
    /**
     * Ascension is celebrated on a Sunday
     */
    ascension_on_sunday?: boolean | null;
    /**
     * Corpus Christi is celebrated on a Sunday
     */
    corpus_christi_on_sunday?: boolean | null;
    /**
     * The type of Easter calculation
     */
    easter_calculation_type?: EasterCalculationType | null;
    /**
     * Epiphany is celebrated on a Sunday
     */
    epiphany_on_sunday?: boolean | null;
}

/**
 * Gregorian calculation (default)
 *
 * Julian calculation converted to Gregorian
 */
export enum EasterCalculationType {
    Gregorian = "GREGORIAN",
    Julian = "JULIAN",
}

/**
 * Liturgical period information with localized name.
 */
export type PeriodInfo = {
    /**
     * The period key
     */
    key: Period;
    /**
     * The localized name of the period
     */
    name: string;
}

/**
 * The period key
 *
 * Specific periods within liturgical seasons.
 * Defines sub-periods that have special liturgical characteristics or rules.
 *
 * The eight days following Christmas (December 25 - January 1)
 *
 * Days before Epiphany (January 2 to the day before Epiphany)
 *
 * Days from Epiphany to the Presentation (January 6 to the day before the Presentation of
 * the Lord)
 *
 * Period from Christmas to the Presentation of the Lord
 *
 * Period from the Presentation to Holy Thursday
 *
 * Holy Week (Palm Sunday to Holy Saturday)
 *
 * Paschal Triduum (start from the Thursday of the Lord's Supper to the Easter Sunday
 * Vespers)
 *
 * The eight days following Easter Sunday
 *
 * Early Ordinary Time (after the Presentation of the Lord to the day before Ash Wednesday)
 *
 * Late Ordinary Time (after Pentecost to the day before the First Sunday of Advent)
 */
export enum Period {
    ChristmasOctave = "CHRISTMAS_OCTAVE",
    ChristmasToPresentationOfTheLord = "CHRISTMAS_TO_PRESENTATION_OF_THE_LORD",
    DaysBeforeEpiphany = "DAYS_BEFORE_EPIPHANY",
    DaysFromEpiphany = "DAYS_FROM_EPIPHANY",
    EarlyOrdinaryTime = "EARLY_ORDINARY_TIME",
    EasterOctave = "EASTER_OCTAVE",
    HolyWeek = "HOLY_WEEK",
    LateOrdinaryTime = "LATE_ORDINARY_TIME",
    PaschalTriduum = "PASCHAL_TRIDUUM",
    PresentationOfTheLordToHolyThursday = "PRESENTATION_OF_THE_LORD_TO_HOLY_THURSDAY",
}

/**
 * The psalter week cycle to which this liturgical day belongs.
 *
 * [GILH §133] The four-week cycle of the psalter is coordinated with the liturgical year in
 * such a way that
 * on the First Sunday of Advent, the First Sunday in Ordinary Time, the First Sunday of
 * Lent,
 * and Easter Sunday the cycle is always begun again with Week 1 (others being omitted when
 * necessary).
 *
 * Week 1
 *
 * Week 2
 *
 * Week 3
 *
 * Week 4
 */
export enum PsalterWeekCycle {
    Week1 = "WEEK_1",
    Week2 = "WEEK_2",
    Week3 = "WEEK_3",
    Week4 = "WEEK_4",
}

/**
 * Advent
 *
 * Christmas Time
 *
 * Lent
 *
 * Paschal Triduum
 *
 * Easter Time
 *
 * Ordinary Time
 */
export enum Season {
    Advent = "ADVENT",
    ChristmasTime = "CHRISTMAS_TIME",
    EasterTime = "EASTER_TIME",
    Lent = "LENT",
    OrdinaryTime = "ORDINARY_TIME",
    PaschalTriduum = "PASCHAL_TRIDUUM",
}

/**
 * The Sunday cycle to which this liturgical day belongs.
 *
 * A three-year cycle for Sunday Mass readings (and some solemnities), designated by A, B,
 * or C.
 * Each cycle begins on the First Sunday of Advent of the previous civil year and ends on
 * Saturday
 * after the Christ the King Solemnity. The cycles follow each other in alphabetical order.
 * C year is always divisible by 3, A has remainder of 1, and B remainder of 2.
 *
 * Year A
 *
 * Year B
 *
 * Year C
 */
export enum SundayCycle {
    YearA = "YEAR_A",
    YearB = "YEAR_B",
    YearC = "YEAR_C",
}

/**
 * Title definition that can be either a simple list or a compound definition.
 * Supports both direct title lists and compound title operations.
 *
 * The titles for this liturgical day.
 */
export type TitlesDef = Title[] | CompoundTitle;

/**
 * The weekday cycle to which this liturgical day belongs.
 *
 * A two-year cycle for the weekday Mass readings (also called Cycle I and Cycle II).
 * Odd-numbered years are the Cycle I (year 1); even-numbered ones are the Cycle II (year
 * 2).
 *
 * Year 1 (Cycle I)
 *
 * Year 2 (Cycle II)
 */
export enum WeekdayCycle {
    Year1 = "YEAR_1",
    Year2 = "YEAR_2",
}
