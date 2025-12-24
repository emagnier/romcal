from enum import Enum
from pydantic import BaseModel
from typing import Optional, Any, List, Union, Dict, TypeVar, Type, cast, Callable


T = TypeVar("T")
EnumT = TypeVar("EnumT", bound=Enum)


def from_int(x: Any) -> int:
    assert isinstance(x, int) and not isinstance(x, bool)
    return x


def from_none(x: Any) -> Any:
    assert x is None
    return x


def from_union(fs, x):
    for f in fs:
        try:
            return f(x)
        except:
            pass
    assert False


def to_enum(c: Type[EnumT], x: Any) -> EnumT:
    assert isinstance(x, c)
    return x.value


def from_bool(x: Any) -> bool:
    assert isinstance(x, bool)
    return x


def to_class(c: Type[T], x: Any) -> dict:
    assert isinstance(x, c)
    return cast(Any, x).to_dict()


def from_list(f: Callable[[Any], T], x: Any) -> List[T]:
    assert isinstance(x, list)
    return [f(y) for y in x]


def from_str(x: Any) -> str:
    assert isinstance(x, str)
    return x


def from_dict(f: Callable[[Any], T], x: Any) -> Dict[str, T]:
    assert isinstance(x, dict)
    return { k: f(v) for (k, v) in x.items() }


class Acclamation(Enum):
    """Acclamations used in liturgical celebrations.
    Acclamations are short liturgical responses or exclamations used during Mass.
    
    Alleluia - joyful acclamation used outside of Lent
    
    Lent - acclamation used during Lenten season
    
    Mixed - combination of different acclamation types
    
    None - no acclamation
    """
    ALLELUIA = "ALLELUIA"
    LENT = "LENT"
    MIXED = "MIXED"
    NONE = "NONE"


class BibleBook(Enum):
    """Books of the Bible using OSIS (Open Scripture Information Standard) identifiers.
    OSIS provides standardized abbreviations for biblical books used in liturgical and
    biblical applications.
    
    Genesis
    
    Exodus
    
    Leviticus
    
    Numbers
    
    Deuteronomy
    
    Joshua
    
    Judges
    
    Ruth
    
    1 Samuel
    
    2 Samuel
    
    1 Kings
    
    2 Kings
    
    1 Chronicles
    
    2 Chronicles
    
    Ezra
    
    Nehemiah
    
    Tobit
    
    Judith
    
    Esther
    
    1 Maccabees
    
    2 Maccabees
    
    Job
    
    Psalms
    
    Proverbs
    
    Ecclesiastes (Qohelet)
    
    Song of Solomon (Canticle of Canticles)
    
    Wisdom of Solomon
    
    Sirach (Ecclesiasticus)
    
    Isaiah
    
    Jeremiah
    
    Lamentations
    
    Baruch
    
    Letter of Jeremiah
    
    Ezekiel
    
    Daniel
    
    Hosea
    
    Joel
    
    Amos
    
    Obadiah
    
    Jonah
    
    Micah
    
    Nahum
    
    Habakkuk
    
    Zephaniah
    
    Haggai
    
    Zechariah
    
    Malachi
    
    Matthew
    
    Mark
    
    Luke
    
    John
    
    Acts
    
    Romans
    
    1 Corinthians
    
    2 Corinthians
    
    Galatians
    
    Ephesians
    
    Philippians
    
    Colossians
    
    1 Thessalonians
    
    2 Thessalonians
    
    1 Timothy
    
    2 Timothy
    
    Titus
    
    Philemon
    
    Hebrews
    
    James
    
    1 Peter
    
    2 Peter
    
    1 John
    
    2 John
    
    3 John
    
    Jude
    
    Revelation
    """
    ACTS = "Acts"
    AMOS = "Amos"
    BAR = "Bar"
    COL = "Col"
    DAN = "Dan"
    DEUT = "Deut"
    ECCL = "Eccl"
    EPH = "Eph"
    EP_JER = "EpJer"
    ESTH = "Esth"
    EXOD = "Exod"
    EZEK = "Ezek"
    EZRA = "Ezra"
    GAL = "Gal"
    GEN = "Gen"
    HAB = "Hab"
    HAG = "Hag"
    HEB = "Heb"
    HOS = "Hos"
    ISA = "Isa"
    JAS = "Jas"
    JDT = "Jdt"
    JER = "Jer"
    JOB = "Job"
    JOEL = "Joel"
    JOHN = "John"
    JONAH = "Jonah"
    JOSH = "Josh"
    JUDE = "Jude"
    JUDG = "Judg"
    LAM = "Lam"
    LEV = "Lev"
    LUKE = "Luke"
    MAL = "Mal"
    MARK = "Mark"
    MATT = "Matt"
    MIC = "Mic"
    NAH = "Nah"
    NEH = "Neh"
    NUM = "Num"
    OBAD = "Obad"
    PHIL = "Phil"
    PHLM = "Phlm"
    PROV = "Prov"
    PS = "Ps"
    REV = "Rev"
    ROM = "Rom"
    RUTH = "Ruth"
    SIR = "Sir"
    SONG = "Song"
    THE_1_CHR = "1Chr"
    THE_1_COR = "1Cor"
    THE_1_JOHN = "1John"
    THE_1_KGS = "1Kgs"
    THE_1_MACC = "1Macc"
    THE_1_PET = "1Pet"
    THE_1_SAM = "1Sam"
    THE_1_THESS = "1Thess"
    THE_1_TIM = "1Tim"
    THE_2_CHR = "2Chr"
    THE_2_COR = "2Cor"
    THE_2_JOHN = "2John"
    THE_2_KGS = "2Kgs"
    THE_2_MACC = "2Macc"
    THE_2_PET = "2Pet"
    THE_2_SAM = "2Sam"
    THE_2_THESS = "2Thess"
    THE_2_TIM = "2Tim"
    THE_3_JOHN = "3John"
    TITUS = "Titus"
    TOB = "Tob"
    WIS = "Wis"
    ZECH = "Zech"
    ZEPH = "Zeph"


class CalendarContext(Enum):
    """Calendar year context for date boundaries.
    
    Determines how the calendar year is structured and which dates are included
    in a given year's calendar output.
    
    Gregorian year (January 1 to December 31)
    
    Liturgical year (first Sunday of Advent to the day before the first Sunday of Advent of
    the next year)
    """
    GREGORIAN = "GREGORIAN"
    LITURGICAL = "LITURGICAL"


class Color(Enum):
    """Liturgical colors used in the celebration of Mass and other liturgical services.
    Each color has specific liturgical significance and is used during particular seasons or
    celebrations.
    
    The color key
    
    Red - used for martyrs, Pentecost, and Palm Sunday
    
    Rose - used on Gaudete Sunday (3rd Advent) and Laetare Sunday (4th Lent)
    
    Purple - used during Advent and Lent
    
    Green - used during Ordinary Time
    
    White - used for Christmas, Easter, and most feasts
    
    Gold - used for solemn celebrations and special occasions
    
    Black - used for funerals and All Souls' Day
    """
    BLACK = "BLACK"
    GOLD = "GOLD"
    GREEN = "GREEN"
    PURPLE = "PURPLE"
    RED = "RED"
    ROSE = "ROSE"
    WHITE = "WHITE"


class CommonDefinition(Enum):
    """Common definition for simplified categorization.
    Provides a simplified version of the Common enum for easier classification.
    
    No common.
    
    Dedication anniversary (in the Church that was Dedicated).
    
    Dedication anniversary (outside the Church that was Dedicated).
    
    Common of the Blessed Virgin Mary.
    
    Common for Martyrs.
    
    Common for Missionary Martyrs.
    
    Common for Virgin Martyrs.
    
    Common for Holy Woman Martyrs.
    
    Common for Pastors.
    
    Common for Popes.
    
    Common for Bishops.
    
    Common for Founders.
    
    Common for Missionaries.
    
    Common for Doctors of the Church.
    
    Common for Virgins.
    
    Common for Holy Men and Women.
    
    Common for Abbots.
    
    Common for Monks.
    
    Common for Nuns.
    
    Common for Religious.
    
    Common for Those Who Practiced Works of Mercy.
    
    Common for Educators.
    
    Common for Holy Women.
    """
    ABBOTS = "ABBOTS"
    BISHOPS = "BISHOPS"
    BLESSED_VIRGIN_MARY = "BLESSED_VIRGIN_MARY"
    DEDICATION_ANNIVERSARY_INSIDE = "DEDICATION_ANNIVERSARY__INSIDE"
    DEDICATION_ANNIVERSARY_OUTSIDE = "DEDICATION_ANNIVERSARY__OUTSIDE"
    DOCTORS_OF_THE_CHURCH = "DOCTORS_OF_THE_CHURCH"
    EDUCATORS = "EDUCATORS"
    FOUNDERS = "FOUNDERS"
    HOLY_WOMEN = "HOLY_WOMEN"
    MARTYRS = "MARTYRS"
    MERCY_WORKERS = "MERCY_WORKERS"
    MISSIONARIES = "MISSIONARIES"
    MISSIONARY_MARTYRS = "MISSIONARY_MARTYRS"
    MONKS = "MONKS"
    NONE = "NONE"
    NUNS = "NUNS"
    PASTORS = "PASTORS"
    POPES = "POPES"
    RELIGIOUS = "RELIGIOUS"
    SAINTS = "SAINTS"
    VIRGINS = "VIRGINS"
    VIRGIN_MARTYRS = "VIRGIN_MARTYRS"
    WOMAN_MARTYRS = "WOMAN_MARTYRS"


class DateFn(Enum):
    """The date function to calculate the base date
    
    Date function for calculating liturgical dates.
    
    Represents movable feasts and special celebrations that require calculation
    based on Easter or other variable dates.
    
    Monday after Pentecost.
    
    Sunday between January 2 and 8 (or January 6 if not transferred).
    
    February 2 (Candlemas).
    
    March 25 (may be transferred if in Holy Week or Easter Octave).
    
    Sunday before Easter.
    
    First Sunday after the Paschal Full Moon.
    
    Second Sunday of Easter.
    
    Saturday after the Second Sunday after Pentecost.
    
    Seventh Sunday after Easter.
    
    Thursday or Sunday after Trinity Sunday.
    
    June 24.
    
    June 29.
    
    August 6.
    
    August 15.
    
    September 14.
    
    November 1.
    
    December 8.
    """
    ALL_SAINTS = "ALL_SAINTS"
    ANNUNCIATION = "ANNUNCIATION"
    ASSUMPTION = "ASSUMPTION"
    CORPUS_CHRISTI_SUNDAY = "CORPUS_CHRISTI_SUNDAY"
    DIVINE_MERCY_SUNDAY = "DIVINE_MERCY_SUNDAY"
    EASTER_SUNDAY = "EASTER_SUNDAY"
    EPIPHANY_SUNDAY = "EPIPHANY_SUNDAY"
    EXALTATION_OF_THE_HOLY_CROSS = "EXALTATION_OF_THE_HOLY_CROSS"
    IMMACULATE_CONCEPTION_OF_MARY = "IMMACULATE_CONCEPTION_OF_MARY"
    IMMACULATE_HEART_OF_MARY = "IMMACULATE_HEART_OF_MARY"
    MARY_MOTHER_OF_THE_CHURCH = "MARY_MOTHER_OF_THE_CHURCH"
    NATIVITY_OF_JOHN_THE_BAPTIST = "NATIVITY_OF_JOHN_THE_BAPTIST"
    PALM_SUNDAY = "PALM_SUNDAY"
    PENTECOST_SUNDAY = "PENTECOST_SUNDAY"
    PETER_AND_PAUL_APOSTLES = "PETER_AND_PAUL_APOSTLES"
    PRESENTATION_OF_THE_LORD = "PRESENTATION_OF_THE_LORD"
    TRANSFIGURATION = "TRANSFIGURATION"


class DateDefClass(BaseModel):
    """Simple month/day specification
    
    Date function calculation (Easter, Epiphany, etc.)
    
    Nth weekday of a specific month
    
    Last weekday of a specific month
    
    Inherited from the proper of time
    """
    date: Optional[int] = None
    """The day of the month (1-31)"""

    day_offset: Optional[int] = None
    """Optional day offset for adjustments"""

    month: Optional[int] = None
    """The month (1-12)"""

    date_fn: Optional[DateFn] = None
    """The date function to calculate the base date"""

    day_of_week: Optional[int] = None
    """The day of the week (0=Sunday, 1=Monday, etc.)"""

    nth_week_in_month: Optional[int] = None
    """Which occurrence of the weekday (1st, 2nd, 3rd, etc.)"""

    last_day_of_week_in_month: Optional[int] = None
    """The day of the week to find the last occurrence of"""

    @staticmethod
    def from_dict(obj: Any) -> 'DateDefClass':
        assert isinstance(obj, dict)
        date = from_union([from_int, from_none], obj.get("date"))
        day_offset = from_union([from_none, from_int], obj.get("day_offset"))
        month = from_union([from_int, from_none], obj.get("month"))
        date_fn = from_union([DateFn, from_none], obj.get("date_fn"))
        day_of_week = from_union([from_int, from_none], obj.get("day_of_week"))
        nth_week_in_month = from_union([from_int, from_none], obj.get("nth_week_in_month"))
        last_day_of_week_in_month = from_union([from_int, from_none], obj.get("last_day_of_week_in_month"))
        return DateDefClass(date, day_offset, month, date_fn, day_of_week, nth_week_in_month, last_day_of_week_in_month)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.date is not None:
            result["date"] = from_union([from_int, from_none], self.date)
        if self.day_offset is not None:
            result["day_offset"] = from_union([from_none, from_int], self.day_offset)
        if self.month is not None:
            result["month"] = from_union([from_int, from_none], self.month)
        if self.date_fn is not None:
            result["date_fn"] = from_union([lambda x: to_enum(DateFn, x), from_none], self.date_fn)
        if self.day_of_week is not None:
            result["day_of_week"] = from_union([from_int, from_none], self.day_of_week)
        if self.nth_week_in_month is not None:
            result["nth_week_in_month"] = from_union([from_int, from_none], self.nth_week_in_month)
        if self.last_day_of_week_in_month is not None:
            result["last_day_of_week_in_month"] = from_union([from_int, from_none], self.last_day_of_week_in_month)
        return result


class DateDefExtended(BaseModel):
    """The date to set when the condition is met
    
    Extended date definition supporting both regular dates and offset dates.
    Provides flexibility for date calculations with optional adjustments.
    
    Simple month/day specification
    
    Date function calculation (Easter, Epiphany, etc.)
    
    Nth weekday of a specific month
    
    Last weekday of a specific month
    
    Inherited from the proper of time
    
    Date definition with offset
    
    Date definition with offset for adjustments.
    Used when a date needs to be shifted by a specific number of days.
    """
    date: Optional[int] = None
    """The day of the month (1-31)"""

    day_offset: Optional[int] = None
    """Optional day offset for adjustments
    
    The number of days to offset the date
    """
    month: Optional[int] = None
    """The month (1-12)"""

    date_fn: Optional[DateFn] = None
    """The date function to calculate the base date"""

    day_of_week: Optional[int] = None
    """The day of the week (0=Sunday, 1=Monday, etc.)"""

    nth_week_in_month: Optional[int] = None
    """Which occurrence of the weekday (1st, 2nd, 3rd, etc.)"""

    last_day_of_week_in_month: Optional[int] = None
    """The day of the week to find the last occurrence of"""

    @staticmethod
    def from_dict(obj: Any) -> 'DateDefExtended':
        assert isinstance(obj, dict)
        date = from_union([from_int, from_none], obj.get("date"))
        day_offset = from_union([from_none, from_int], obj.get("day_offset"))
        month = from_union([from_int, from_none], obj.get("month"))
        date_fn = from_union([DateFn, from_none], obj.get("date_fn"))
        day_of_week = from_union([from_int, from_none], obj.get("day_of_week"))
        nth_week_in_month = from_union([from_int, from_none], obj.get("nth_week_in_month"))
        last_day_of_week_in_month = from_union([from_int, from_none], obj.get("last_day_of_week_in_month"))
        return DateDefExtended(date, day_offset, month, date_fn, day_of_week, nth_week_in_month, last_day_of_week_in_month)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.date is not None:
            result["date"] = from_union([from_int, from_none], self.date)
        if self.day_offset is not None:
            result["day_offset"] = from_union([from_none, from_int], self.day_offset)
        if self.month is not None:
            result["month"] = from_union([from_int, from_none], self.month)
        if self.date_fn is not None:
            result["date_fn"] = from_union([lambda x: to_enum(DateFn, x), from_none], self.date_fn)
        if self.day_of_week is not None:
            result["day_of_week"] = from_union([from_int, from_none], self.day_of_week)
        if self.nth_week_in_month is not None:
            result["nth_week_in_month"] = from_union([from_int, from_none], self.nth_week_in_month)
        if self.last_day_of_week_in_month is not None:
            result["last_day_of_week_in_month"] = from_union([from_int, from_none], self.last_day_of_week_in_month)
        return result


class DateDef(BaseModel):
    """Date definition supporting various date calculation methods.
    Provides flexible ways to specify liturgical dates using different approaches.
    
    Regular date definition
    
    The start date of the range
    
    The end date of the range
    
    The date to compare against
    
    The date definition for this liturgical day.
    
    Simple month/day specification
    
    Date function calculation (Easter, Epiphany, etc.)
    
    Nth weekday of a specific month
    
    Last weekday of a specific month
    
    Inherited from the proper of time
    """
    date: Optional[int] = None
    """The day of the month (1-31)"""

    day_offset: Optional[int] = None
    """Optional day offset for adjustments"""

    month: Optional[int] = None
    """The month (1-12)"""

    date_fn: Optional[DateFn] = None
    """The date function to calculate the base date"""

    day_of_week: Optional[int] = None
    """The day of the week (0=Sunday, 1=Monday, etc.)"""

    nth_week_in_month: Optional[int] = None
    """Which occurrence of the weekday (1st, 2nd, 3rd, etc.)"""

    last_day_of_week_in_month: Optional[int] = None
    """The day of the week to find the last occurrence of"""

    @staticmethod
    def from_dict(obj: Any) -> 'DateDef':
        assert isinstance(obj, dict)
        date = from_union([from_int, from_none], obj.get("date"))
        day_offset = from_union([from_none, from_int], obj.get("day_offset"))
        month = from_union([from_int, from_none], obj.get("month"))
        date_fn = from_union([DateFn, from_none], obj.get("date_fn"))
        day_of_week = from_union([from_int, from_none], obj.get("day_of_week"))
        nth_week_in_month = from_union([from_int, from_none], obj.get("nth_week_in_month"))
        last_day_of_week_in_month = from_union([from_int, from_none], obj.get("last_day_of_week_in_month"))
        return DateDef(date, day_offset, month, date_fn, day_of_week, nth_week_in_month, last_day_of_week_in_month)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.date is not None:
            result["date"] = from_union([from_int, from_none], self.date)
        if self.day_offset is not None:
            result["day_offset"] = from_union([from_none, from_int], self.day_offset)
        if self.month is not None:
            result["month"] = from_union([from_int, from_none], self.month)
        if self.date_fn is not None:
            result["date_fn"] = from_union([lambda x: to_enum(DateFn, x), from_none], self.date_fn)
        if self.day_of_week is not None:
            result["day_of_week"] = from_union([from_int, from_none], self.day_of_week)
        if self.nth_week_in_month is not None:
            result["nth_week_in_month"] = from_union([from_int, from_none], self.nth_week_in_month)
        if self.last_day_of_week_in_month is not None:
            result["last_day_of_week_in_month"] = from_union([from_int, from_none], self.last_day_of_week_in_month)
        return result


class ExceptionCondition(BaseModel):
    """The condition that triggers the exception
    
    Exception conditions that can trigger a date change.
    Defines various conditions under which a date exception applies.
    
    If the date is between two specified dates
    
    If the date is the same as another specified date
    
    If the date falls on a specific day of the week
    """
    exception_condition_from: Optional[DateDef] = None
    """The start date of the range"""

    inclusive: Optional[bool] = None
    """Whether the range is inclusive of the start date and the end date"""

    to: Optional[DateDef] = None
    """The end date of the range"""

    date: Optional[DateDef] = None
    """The date to compare against"""

    day_of_week: Optional[int] = None
    """The day of the week to match"""

    @staticmethod
    def from_dict(obj: Any) -> 'ExceptionCondition':
        assert isinstance(obj, dict)
        exception_condition_from = from_union([DateDef.from_dict, from_none], obj.get("from"))
        inclusive = from_union([from_none, from_bool], obj.get("inclusive"))
        to = from_union([DateDef.from_dict, from_none], obj.get("to"))
        date = from_union([DateDef.from_dict, from_none], obj.get("date"))
        day_of_week = from_union([from_int, from_none], obj.get("day_of_week"))
        return ExceptionCondition(exception_condition_from, inclusive, to, date, day_of_week)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.exception_condition_from is not None:
            result["from"] = from_union([lambda x: to_class(DateDef, x), from_none], self.exception_condition_from)
        if self.inclusive is not None:
            result["inclusive"] = from_union([from_none, from_bool], self.inclusive)
        if self.to is not None:
            result["to"] = from_union([lambda x: to_class(DateDef, x), from_none], self.to)
        if self.date is not None:
            result["date"] = from_union([lambda x: to_class(DateDef, x), from_none], self.date)
        if self.day_of_week is not None:
            result["day_of_week"] = from_union([from_int, from_none], self.day_of_week)
        return result


class DateDefException(BaseModel):
    """Single date exception
    
    The liturgical day date exception.
    Represents a condition and the date to set when that condition is met.
    
    Multiple date exceptions
    """
    then: DateDefExtended
    """The date to set when the condition is met"""

    when: ExceptionCondition
    """The condition that triggers the exception"""

    @staticmethod
    def from_dict(obj: Any) -> 'DateDefException':
        assert isinstance(obj, dict)
        then = DateDefExtended.from_dict(obj.get("then"))
        when = ExceptionCondition.from_dict(obj.get("when"))
        return DateDefException(then, when)

    def to_dict(self) -> dict:
        result: dict = {}
        result["then"] = to_class(DateDefExtended, self.then)
        result["when"] = to_class(ExceptionCondition, self.when)
        return result


class SaintCountEnum(Enum):
    MANY = "MANY"


class Title(Enum):
    """Simple list of titles
    
    Titles and patronages associated with saints and blessed.
    Represents the various ecclesiastical titles and patronages that can be assigned to
    entities.
    """
    ABBESS = "ABBESS"
    ABBOT = "ABBOT"
    APOSTLE = "APOSTLE"
    ARCHANGEL = "ARCHANGEL"
    BISHOP = "BISHOP"
    COPATRONESS_OF_EUROPE = "COPATRONESS_OF_EUROPE"
    COPATRONESS_OF_FRANCE = "COPATRONESS_OF_FRANCE"
    COPATRONESS_OF_IRELAND = "COPATRONESS_OF_IRELAND"
    COPATRONESS_OF_ITALY_AND_EUROPE = "COPATRONESS_OF_ITALY_AND_EUROPE"
    COPATRONESS_OF_THE_PHILIPPINES = "COPATRONESS_OF_THE_PHILIPPINES"
    COPATRON_OF_CANADA = "COPATRON_OF_CANADA"
    COPATRON_OF_EUROPE = "COPATRON_OF_EUROPE"
    COPATRON_OF_IRELAND = "COPATRON_OF_IRELAND"
    DEACON = "DEACON"
    DOCTOR_OF_THE_CHURCH = "DOCTOR_OF_THE_CHURCH"
    EMPRESS = "EMPRESS"
    EVANGELIST = "EVANGELIST"
    FIRST_BISHOP = "FIRST_BISHOP"
    HERMIT = "HERMIT"
    KING = "KING"
    MARTYR = "MARTYR"
    MISSIONARY = "MISSIONARY"
    MONK = "MONK"
    MOTHER_AND_QUEEN_OF_CHILE = "MOTHER_AND_QUEEN_OF_CHILE"
    PARENTS_OF_THE_BLESSED_VIRGIN_MARY = "PARENTS_OF_THE_BLESSED_VIRGIN_MARY"
    PATRIARCH = "PATRIARCH"
    PATRONESS_OF_ALSACE = "PATRONESS_OF_ALSACE"
    PATRONESS_OF_ARGENTINA = "PATRONESS_OF_ARGENTINA"
    PATRONESS_OF_BRAZIL = "PATRONESS_OF_BRAZIL"
    PATRONESS_OF_COSTA_RICA = "PATRONESS_OF_COSTA_RICA"
    PATRONESS_OF_HUNGARY = "PATRONESS_OF_HUNGARY"
    PATRONESS_OF_PUERTO_RICO = "PATRONESS_OF_PUERTO_RICO"
    PATRONESS_OF_SLOVAKIA = "PATRONESS_OF_SLOVAKIA"
    PATRONESS_OF_THE_AMERICAS = "PATRONESS_OF_THE_AMERICAS"
    PATRONESS_OF_THE_PHILIPPINES = "PATRONESS_OF_THE_PHILIPPINES"
    PATRONESS_OF_THE_PROVINCE_OF_QUEBEC = "PATRONESS_OF_THE_PROVINCE_OF_QUEBEC"
    PATRONESS_OF_THE_USA = "PATRONESS_OF_THE_USA"
    PATRON_OF_CANADA = "PATRON_OF_CANADA"
    PATRON_OF_ENGLAND = "PATRON_OF_ENGLAND"
    PATRON_OF_EUROPE = "PATRON_OF_EUROPE"
    PATRON_OF_FRANCE = "PATRON_OF_FRANCE"
    PATRON_OF_IRELAND = "PATRON_OF_IRELAND"
    PATRON_OF_ITALY = "PATRON_OF_ITALY"
    PATRON_OF_OCEANIA = "PATRON_OF_OCEANIA"
    PATRON_OF_POLAND = "PATRON_OF_POLAND"
    PATRON_OF_RUSSIA = "PATRON_OF_RUSSIA"
    PATRON_OF_SCOTLAND = "PATRON_OF_SCOTLAND"
    PATRON_OF_SPAIN = "PATRON_OF_SPAIN"
    PATRON_OF_THE_CITY_OF_LYON = "PATRON_OF_THE_CITY_OF_LYON"
    PATRON_OF_THE_CLERGY_OF_THE_ARCHDIOCESE_OF_LYON = "PATRON_OF_THE_CLERGY_OF_THE_ARCHDIOCESE_OF_LYON"
    PATRON_OF_THE_CZECH_NATION = "PATRON_OF_THE_CZECH_NATION"
    PATRON_OF_THE_DIOCESE = "PATRON_OF_THE_DIOCESE"
    PATRON_OF_WALES = "PATRON_OF_WALES"
    PILGRIM = "PILGRIM"
    POPE = "POPE"
    PRIEST = "PRIEST"
    PRINCIPAL_PATRON_OF_THE_DIOCESE = "PRINCIPAL_PATRON_OF_THE_DIOCESE"
    PROPHET = "PROPHET"
    PROTO_MARTYR_OF_OCEANIA = "PROTO_MARTYR_OF_OCEANIA"
    QUEEN = "QUEEN"
    QUEEN_OF_POLAND = "QUEEN_OF_POLAND"
    RELIGIOUS = "RELIGIOUS"
    SECOND_PATRON_OF_THE_DIOCESE = "SECOND_PATRON_OF_THE_DIOCESE"
    SLAVIC_MISSIONARY = "SLAVIC_MISSIONARY"
    SPOUSE_OF_THE_BLESSED_VIRGIN_MARY = "SPOUSE_OF_THE_BLESSED_VIRGIN_MARY"
    THE_FIRST_MARTYR = "THE_FIRST_MARTYR"
    VIRGIN = "VIRGIN"


class CompoundTitle(BaseModel):
    """Compound title definition with append/prepend operations
    
    Compound title definition for combining multiple titles.
    Allows adding titles to the beginning or end of an existing title list.
    """
    append: Optional[List[Title]] = None
    """The title(s) to add to the end of the existing list of title(s)"""

    prepend: Optional[List[Title]] = None
    """The title(s) to add to the beginning of the existing list of title(s)"""

    @staticmethod
    def from_dict(obj: Any) -> 'CompoundTitle':
        assert isinstance(obj, dict)
        append = from_union([from_none, lambda x: from_list(Title, x)], obj.get("append"))
        prepend = from_union([from_none, lambda x: from_list(Title, x)], obj.get("prepend"))
        return CompoundTitle(append, prepend)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.append is not None:
            result["append"] = from_union([from_none, lambda x: from_list(lambda x: to_enum(Title, x), x)], self.append)
        if self.prepend is not None:
            result["prepend"] = from_union([from_none, lambda x: from_list(lambda x: to_enum(Title, x), x)], self.prepend)
        return result


class EntityOverride(BaseModel):
    """Custom entity definition with additional properties specific to a liturgical day
    
    Custom entity definition that extends or overrides properties from the entity catalog.
    Used when a liturgical day needs specific entity properties that differ from the base
    entity.
    """
    id: str
    """The ID of the entity item (must reference an existing entity in the catalog)"""

    count: Optional[Union[int, SaintCountEnum]] = None
    """The number of persons this entity represents (useful for groups of martyrs or saints)"""

    hide_titles: Optional[bool] = None
    """Whether to hide titles when displaying this entity (useful when titles are already
    included in the entity name)
    """
    titles: Optional[Union[List[Title], CompoundTitle]] = None
    """The custom titles for this entity in the context of this liturgical day"""

    @staticmethod
    def from_dict(obj: Any) -> 'EntityOverride':
        assert isinstance(obj, dict)
        id = from_str(obj.get("id"))
        count = from_union([from_int, from_none, SaintCountEnum], obj.get("count"))
        hide_titles = from_union([from_none, from_bool], obj.get("hide_titles"))
        titles = from_union([lambda x: from_list(Title, x), CompoundTitle.from_dict, from_none], obj.get("titles"))
        return EntityOverride(id, count, hide_titles, titles)

    def to_dict(self) -> dict:
        result: dict = {}
        result["id"] = from_str(self.id)
        if self.count is not None:
            result["count"] = from_union([from_int, from_none, lambda x: to_enum(SaintCountEnum, x)], self.count)
        if self.hide_titles is not None:
            result["hide_titles"] = from_union([from_none, from_bool], self.hide_titles)
        if self.titles is not None:
            result["titles"] = from_union([lambda x: from_list(lambda x: to_enum(Title, x), x), lambda x: to_class(CompoundTitle, x), from_none], self.titles)
        return result


class MassContent(BaseModel):
    """Invariant content that applies to all cycles
    
    Content of a mass for a specific liturgical cycle
    Maps mass parts (readings, psalms, prayers, antiphons, etc.) to their texts
    
    Year 1 of the weekday cycle (Cycle I)
    
    Year 2 of the weekday cycle (Cycle II)
    
    Year A of the Sunday cycle
    
    Combined years A and B of the Sunday cycle
    
    Combined years A and C of the Sunday cycle
    
    Year B of the Sunday cycle
    
    Combined years B and C of the Sunday cycle
    
    Year C of the Sunday cycle
    """
    alleluia: Optional[str] = None
    """Alleluia - acclamation before the Gospel"""

    canticle: Optional[str] = None
    """Canticle - biblical canticle"""

    collect: Optional[str] = None
    """Collect - opening prayer of the Mass"""

    communion_antiphon: Optional[str] = None
    """Communion Antiphon - chant during communion"""

    easter_vigil_canticle_3: Optional[str] = None
    """Canticle 3 (Easter Vigil)"""

    easter_vigil_canticle_5: Optional[str] = None
    """Canticle 5 (Easter Vigil)"""

    easter_vigil_epistle: Optional[str] = None
    """Epistle - reading from the epistles (Easter Vigil)"""

    easter_vigil_psalm_2: Optional[str] = None
    """Psalm 2 (Easter Vigil)"""

    easter_vigil_psalm_4: Optional[str] = None
    """Psalm 4 (Easter Vigil)"""

    easter_vigil_psalm_6: Optional[str] = None
    """Psalm 6 (Easter Vigil)"""

    easter_vigil_psalm_7: Optional[str] = None
    """Psalm 7 (Easter Vigil)"""

    easter_vigil_reading_3: Optional[str] = None
    """Reading 3 - third reading (Easter Vigil)"""

    easter_vigil_reading_4: Optional[str] = None
    """Reading 4 - fourth reading (Easter Vigil)"""

    easter_vigil_reading_5: Optional[str] = None
    """Reading 5 - fifth reading (Easter Vigil)"""

    easter_vigil_reading_6: Optional[str] = None
    """Reading 6 - sixth reading (Easter Vigil)"""

    easter_vigil_reading_7: Optional[str] = None
    """Reading 7 - seventh reading (Easter Vigil)"""

    entrance_antiphon: Optional[str] = None
    """Entrance Antiphon - opening chant of the Mass"""

    gospel: Optional[str] = None
    """Gospel - reading from the Gospels"""

    messianic_entry: Optional[str] = None
    """Messianic entry reading (during the procession with palms, before the Mass of the Passion)"""

    prayer_after_communion: Optional[str] = None
    """Prayer after Communion - concluding prayer"""

    prayer_over_the_offerings: Optional[str] = None
    """Prayer over the Offerings - prayer during the offertory"""

    prayer_over_the_people: Optional[str] = None
    """Prayer over the People - blessing over the congregation"""

    preface: Optional[str] = None
    """Preface - introduction to the Eucharistic Prayer"""

    psalm: Optional[str] = None
    """Psalm - responsorial psalm"""

    reading_1: Optional[str] = None
    """Reading 1 - first reading (usually from the Old Testament)"""

    reading_2: Optional[str] = None
    """Reading 2 - second reading (usually from the New Testament)"""

    sequence: Optional[str] = None
    """Sequence - special chant on certain feasts"""

    solemn_blessing: Optional[str] = None
    """Solemn Blessing - special blessing on certain occasions"""

    @staticmethod
    def from_dict(obj: Any) -> 'MassContent':
        assert isinstance(obj, dict)
        alleluia = from_union([from_str, from_none], obj.get("alleluia"))
        canticle = from_union([from_str, from_none], obj.get("canticle"))
        collect = from_union([from_str, from_none], obj.get("collect"))
        communion_antiphon = from_union([from_str, from_none], obj.get("communion_antiphon"))
        easter_vigil_canticle_3 = from_union([from_str, from_none], obj.get("easter_vigil_canticle_3"))
        easter_vigil_canticle_5 = from_union([from_str, from_none], obj.get("easter_vigil_canticle_5"))
        easter_vigil_epistle = from_union([from_str, from_none], obj.get("easter_vigil_epistle"))
        easter_vigil_psalm_2 = from_union([from_str, from_none], obj.get("easter_vigil_psalm_2"))
        easter_vigil_psalm_4 = from_union([from_str, from_none], obj.get("easter_vigil_psalm_4"))
        easter_vigil_psalm_6 = from_union([from_str, from_none], obj.get("easter_vigil_psalm_6"))
        easter_vigil_psalm_7 = from_union([from_str, from_none], obj.get("easter_vigil_psalm_7"))
        easter_vigil_reading_3 = from_union([from_str, from_none], obj.get("easter_vigil_reading_3"))
        easter_vigil_reading_4 = from_union([from_str, from_none], obj.get("easter_vigil_reading_4"))
        easter_vigil_reading_5 = from_union([from_str, from_none], obj.get("easter_vigil_reading_5"))
        easter_vigil_reading_6 = from_union([from_str, from_none], obj.get("easter_vigil_reading_6"))
        easter_vigil_reading_7 = from_union([from_str, from_none], obj.get("easter_vigil_reading_7"))
        entrance_antiphon = from_union([from_str, from_none], obj.get("entrance_antiphon"))
        gospel = from_union([from_str, from_none], obj.get("gospel"))
        messianic_entry = from_union([from_str, from_none], obj.get("messianic_entry"))
        prayer_after_communion = from_union([from_str, from_none], obj.get("prayer_after_communion"))
        prayer_over_the_offerings = from_union([from_str, from_none], obj.get("prayer_over_the_offerings"))
        prayer_over_the_people = from_union([from_str, from_none], obj.get("prayer_over_the_people"))
        preface = from_union([from_str, from_none], obj.get("preface"))
        psalm = from_union([from_str, from_none], obj.get("psalm"))
        reading_1 = from_union([from_str, from_none], obj.get("reading_1"))
        reading_2 = from_union([from_str, from_none], obj.get("reading_2"))
        sequence = from_union([from_str, from_none], obj.get("sequence"))
        solemn_blessing = from_union([from_str, from_none], obj.get("solemn_blessing"))
        return MassContent(alleluia, canticle, collect, communion_antiphon, easter_vigil_canticle_3, easter_vigil_canticle_5, easter_vigil_epistle, easter_vigil_psalm_2, easter_vigil_psalm_4, easter_vigil_psalm_6, easter_vigil_psalm_7, easter_vigil_reading_3, easter_vigil_reading_4, easter_vigil_reading_5, easter_vigil_reading_6, easter_vigil_reading_7, entrance_antiphon, gospel, messianic_entry, prayer_after_communion, prayer_over_the_offerings, prayer_over_the_people, preface, psalm, reading_1, reading_2, sequence, solemn_blessing)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.alleluia is not None:
            result["alleluia"] = from_union([from_str, from_none], self.alleluia)
        if self.canticle is not None:
            result["canticle"] = from_union([from_str, from_none], self.canticle)
        if self.collect is not None:
            result["collect"] = from_union([from_str, from_none], self.collect)
        if self.communion_antiphon is not None:
            result["communion_antiphon"] = from_union([from_str, from_none], self.communion_antiphon)
        if self.easter_vigil_canticle_3 is not None:
            result["easter_vigil_canticle_3"] = from_union([from_str, from_none], self.easter_vigil_canticle_3)
        if self.easter_vigil_canticle_5 is not None:
            result["easter_vigil_canticle_5"] = from_union([from_str, from_none], self.easter_vigil_canticle_5)
        if self.easter_vigil_epistle is not None:
            result["easter_vigil_epistle"] = from_union([from_str, from_none], self.easter_vigil_epistle)
        if self.easter_vigil_psalm_2 is not None:
            result["easter_vigil_psalm_2"] = from_union([from_str, from_none], self.easter_vigil_psalm_2)
        if self.easter_vigil_psalm_4 is not None:
            result["easter_vigil_psalm_4"] = from_union([from_str, from_none], self.easter_vigil_psalm_4)
        if self.easter_vigil_psalm_6 is not None:
            result["easter_vigil_psalm_6"] = from_union([from_str, from_none], self.easter_vigil_psalm_6)
        if self.easter_vigil_psalm_7 is not None:
            result["easter_vigil_psalm_7"] = from_union([from_str, from_none], self.easter_vigil_psalm_7)
        if self.easter_vigil_reading_3 is not None:
            result["easter_vigil_reading_3"] = from_union([from_str, from_none], self.easter_vigil_reading_3)
        if self.easter_vigil_reading_4 is not None:
            result["easter_vigil_reading_4"] = from_union([from_str, from_none], self.easter_vigil_reading_4)
        if self.easter_vigil_reading_5 is not None:
            result["easter_vigil_reading_5"] = from_union([from_str, from_none], self.easter_vigil_reading_5)
        if self.easter_vigil_reading_6 is not None:
            result["easter_vigil_reading_6"] = from_union([from_str, from_none], self.easter_vigil_reading_6)
        if self.easter_vigil_reading_7 is not None:
            result["easter_vigil_reading_7"] = from_union([from_str, from_none], self.easter_vigil_reading_7)
        if self.entrance_antiphon is not None:
            result["entrance_antiphon"] = from_union([from_str, from_none], self.entrance_antiphon)
        if self.gospel is not None:
            result["gospel"] = from_union([from_str, from_none], self.gospel)
        if self.messianic_entry is not None:
            result["messianic_entry"] = from_union([from_str, from_none], self.messianic_entry)
        if self.prayer_after_communion is not None:
            result["prayer_after_communion"] = from_union([from_str, from_none], self.prayer_after_communion)
        if self.prayer_over_the_offerings is not None:
            result["prayer_over_the_offerings"] = from_union([from_str, from_none], self.prayer_over_the_offerings)
        if self.prayer_over_the_people is not None:
            result["prayer_over_the_people"] = from_union([from_str, from_none], self.prayer_over_the_people)
        if self.preface is not None:
            result["preface"] = from_union([from_str, from_none], self.preface)
        if self.psalm is not None:
            result["psalm"] = from_union([from_str, from_none], self.psalm)
        if self.reading_1 is not None:
            result["reading_1"] = from_union([from_str, from_none], self.reading_1)
        if self.reading_2 is not None:
            result["reading_2"] = from_union([from_str, from_none], self.reading_2)
        if self.sequence is not None:
            result["sequence"] = from_union([from_str, from_none], self.sequence)
        if self.solemn_blessing is not None:
            result["solemn_blessing"] = from_union([from_str, from_none], self.solemn_blessing)
        return result


class MassCycleDefinition(BaseModel):
    """Celebration of the Passion - special celebration of Christ's passion
    
    Mass contents for a specific mass time, organized by liturgical cycle
    
    Chrism Mass - Mass where holy oils are blessed, typically on Holy Thursday morning
    
    Day Mass - regular Mass celebrated during the day
    
    Easter Vigil - the most important Mass of the liturgical year, celebrated on Holy
    Saturday night
    
    Evening Mass of the Lord's Supper - Mass celebrated on Holy Thursday evening
    
    Mass at Dawn - Mass celebrated at dawn, particularly on Easter Sunday
    
    Mass of the Passion - Mass focusing on Christ's passion, beginning with the procession
    with palms
    
    Morning Mass - Mass celebrated in the morning
    
    Night Mass - Mass celebrated during the night hours
    
    Previous Evening Mass - Mass celebrated the evening before a major feast
    """
    invariant: Optional[MassContent] = None
    """Invariant content that applies to all cycles"""

    year_1: Optional[MassContent] = None
    """Year 1 of the weekday cycle (Cycle I)"""

    year_2: Optional[MassContent] = None
    """Year 2 of the weekday cycle (Cycle II)"""

    year_a: Optional[MassContent] = None
    """Year A of the Sunday cycle"""

    year_a_b: Optional[MassContent] = None
    """Combined years A and B of the Sunday cycle"""

    year_a_c: Optional[MassContent] = None
    """Combined years A and C of the Sunday cycle"""

    year_b: Optional[MassContent] = None
    """Year B of the Sunday cycle"""

    year_b_c: Optional[MassContent] = None
    """Combined years B and C of the Sunday cycle"""

    year_c: Optional[MassContent] = None
    """Year C of the Sunday cycle"""

    @staticmethod
    def from_dict(obj: Any) -> 'MassCycleDefinition':
        assert isinstance(obj, dict)
        invariant = from_union([MassContent.from_dict, from_none], obj.get("invariant"))
        year_1 = from_union([MassContent.from_dict, from_none], obj.get("year_1"))
        year_2 = from_union([MassContent.from_dict, from_none], obj.get("year_2"))
        year_a = from_union([MassContent.from_dict, from_none], obj.get("year_a"))
        year_a_b = from_union([MassContent.from_dict, from_none], obj.get("year_a_b"))
        year_a_c = from_union([MassContent.from_dict, from_none], obj.get("year_a_c"))
        year_b = from_union([MassContent.from_dict, from_none], obj.get("year_b"))
        year_b_c = from_union([MassContent.from_dict, from_none], obj.get("year_b_c"))
        year_c = from_union([MassContent.from_dict, from_none], obj.get("year_c"))
        return MassCycleDefinition(invariant, year_1, year_2, year_a, year_a_b, year_a_c, year_b, year_b_c, year_c)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.invariant is not None:
            result["invariant"] = from_union([lambda x: to_class(MassContent, x), from_none], self.invariant)
        if self.year_1 is not None:
            result["year_1"] = from_union([lambda x: to_class(MassContent, x), from_none], self.year_1)
        if self.year_2 is not None:
            result["year_2"] = from_union([lambda x: to_class(MassContent, x), from_none], self.year_2)
        if self.year_a is not None:
            result["year_a"] = from_union([lambda x: to_class(MassContent, x), from_none], self.year_a)
        if self.year_a_b is not None:
            result["year_a_b"] = from_union([lambda x: to_class(MassContent, x), from_none], self.year_a_b)
        if self.year_a_c is not None:
            result["year_a_c"] = from_union([lambda x: to_class(MassContent, x), from_none], self.year_a_c)
        if self.year_b is not None:
            result["year_b"] = from_union([lambda x: to_class(MassContent, x), from_none], self.year_b)
        if self.year_b_c is not None:
            result["year_b_c"] = from_union([lambda x: to_class(MassContent, x), from_none], self.year_b_c)
        if self.year_c is not None:
            result["year_c"] = from_union([lambda x: to_class(MassContent, x), from_none], self.year_c)
        return result


class MassesDefinitions(BaseModel):
    """All mass definitions for a liturgical day"""

    celebration_of_the_passion: Optional[MassCycleDefinition] = None
    """Celebration of the Passion - special celebration of Christ's passion"""

    chrism_mass: Optional[MassCycleDefinition] = None
    """Chrism Mass - Mass where holy oils are blessed, typically on Holy Thursday morning"""

    day_mass: Optional[MassCycleDefinition] = None
    """Day Mass - regular Mass celebrated during the day"""

    easter_vigil: Optional[MassCycleDefinition] = None
    """Easter Vigil - the most important Mass of the liturgical year, celebrated on Holy
    Saturday night
    """
    evening_mass_of_the_lords_supper: Optional[MassCycleDefinition] = None
    """Evening Mass of the Lord's Supper - Mass celebrated on Holy Thursday evening"""

    mass_at_dawn: Optional[MassCycleDefinition] = None
    """Mass at Dawn - Mass celebrated at dawn, particularly on Easter Sunday"""

    mass_of_the_passion: Optional[MassCycleDefinition] = None
    """Mass of the Passion - Mass focusing on Christ's passion, beginning with the procession
    with palms
    """
    morning_mass: Optional[MassCycleDefinition] = None
    """Morning Mass - Mass celebrated in the morning"""

    night_mass: Optional[MassCycleDefinition] = None
    """Night Mass - Mass celebrated during the night hours"""

    previous_evening_mass: Optional[MassCycleDefinition] = None
    """Previous Evening Mass - Mass celebrated the evening before a major feast"""

    @staticmethod
    def from_dict(obj: Any) -> 'MassesDefinitions':
        assert isinstance(obj, dict)
        celebration_of_the_passion = from_union([MassCycleDefinition.from_dict, from_none], obj.get("celebration_of_the_passion"))
        chrism_mass = from_union([MassCycleDefinition.from_dict, from_none], obj.get("chrism_mass"))
        day_mass = from_union([MassCycleDefinition.from_dict, from_none], obj.get("day_mass"))
        easter_vigil = from_union([MassCycleDefinition.from_dict, from_none], obj.get("easter_vigil"))
        evening_mass_of_the_lords_supper = from_union([MassCycleDefinition.from_dict, from_none], obj.get("evening_mass_of_the_lords_supper"))
        mass_at_dawn = from_union([MassCycleDefinition.from_dict, from_none], obj.get("mass_at_dawn"))
        mass_of_the_passion = from_union([MassCycleDefinition.from_dict, from_none], obj.get("mass_of_the_passion"))
        morning_mass = from_union([MassCycleDefinition.from_dict, from_none], obj.get("morning_mass"))
        night_mass = from_union([MassCycleDefinition.from_dict, from_none], obj.get("night_mass"))
        previous_evening_mass = from_union([MassCycleDefinition.from_dict, from_none], obj.get("previous_evening_mass"))
        return MassesDefinitions(celebration_of_the_passion, chrism_mass, day_mass, easter_vigil, evening_mass_of_the_lords_supper, mass_at_dawn, mass_of_the_passion, morning_mass, night_mass, previous_evening_mass)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.celebration_of_the_passion is not None:
            result["celebration_of_the_passion"] = from_union([lambda x: to_class(MassCycleDefinition, x), from_none], self.celebration_of_the_passion)
        if self.chrism_mass is not None:
            result["chrism_mass"] = from_union([lambda x: to_class(MassCycleDefinition, x), from_none], self.chrism_mass)
        if self.day_mass is not None:
            result["day_mass"] = from_union([lambda x: to_class(MassCycleDefinition, x), from_none], self.day_mass)
        if self.easter_vigil is not None:
            result["easter_vigil"] = from_union([lambda x: to_class(MassCycleDefinition, x), from_none], self.easter_vigil)
        if self.evening_mass_of_the_lords_supper is not None:
            result["evening_mass_of_the_lords_supper"] = from_union([lambda x: to_class(MassCycleDefinition, x), from_none], self.evening_mass_of_the_lords_supper)
        if self.mass_at_dawn is not None:
            result["mass_at_dawn"] = from_union([lambda x: to_class(MassCycleDefinition, x), from_none], self.mass_at_dawn)
        if self.mass_of_the_passion is not None:
            result["mass_of_the_passion"] = from_union([lambda x: to_class(MassCycleDefinition, x), from_none], self.mass_of_the_passion)
        if self.morning_mass is not None:
            result["morning_mass"] = from_union([lambda x: to_class(MassCycleDefinition, x), from_none], self.morning_mass)
        if self.night_mass is not None:
            result["night_mass"] = from_union([lambda x: to_class(MassCycleDefinition, x), from_none], self.night_mass)
        if self.previous_evening_mass is not None:
            result["previous_evening_mass"] = from_union([lambda x: to_class(MassCycleDefinition, x), from_none], self.previous_evening_mass)
        return result


class Precedence(Enum):
    """1 - The Paschal Triduum of the Passion and Resurrection of the Lord.
    
    2 - The Nativity of the Lord, the Epiphany, the Ascension, or Pentecost.
    
    2 - A Sunday of Advent, Lent, or Easter.
    
    2 - Ash Wednesday.
    
    2 - A weekday of Holy Week from Monday up to and including Thursday.
    
    2 - A day within the Octave of Easter.
    
    3 - A Solemnity inscribed in the General Calendar, whether of the Lord, of the Blessed
    Virgin Mary, or of a Saint.
    
    3 - The Commemoration of All the Faithful Departed.
    
    4a - A proper Solemnity of the principal Patron of the place, city, or state.
    
    4b - The Solemnity of the dedication and of the anniversary of the dedication of the own
    church.
    
    4c - The solemnity of the title of the own church.
    
    4d - A Solemnity either of the Title or of the Founder or of the principal Patron of an
    Order or Congregation.
    
    5 - A Feast of the Lord inscribed in the General Calendar.
    
    6 - A Sunday of Christmas Time or a Sunday in Ordinary Time.
    
    7 - A Feast of the Blessed Virgin Mary or of a Saint in the General Calendar.
    
    8a - The Proper Feast of the principal Patron of the diocese.
    
    8b - The Proper Feast of the anniversary of the dedication of the cathedral church.
    
    8c - The Proper Feast of the principal Patron of a region or province, or a country, or
    of a wider territory.
    
    8d - The Proper Feast of the Title, Founder, or principal Patron of an Order or
    Congregation.
    
    8e - Other Feast, proper to an individual church.
    
    8f - Other Proper Feast inscribed in the Calendar of each diocese or Order or
    Congregation.
    
    9 - Privileged Weekday.
    
    10 - Obligatory Memorials in the General Calendar.
    
    11a - Proper Obligatory Memorial of a secondary Patron of the place, diocese, region, or
    religious province.
    
    11b - Other Proper Obligatory Memorial inscribed in the Calendar of each diocese, or
    Order or congregation.
    
    12 - Optional Memorial.
    
    13 - Weekday.
    
    Liturgical precedence levels for determining which celebration takes priority.
    Defines the hierarchical order of liturgical celebrations according to UNLY norms.
    
    The liturgical precedence for this liturgical day
    
    The liturgical precedence for this liturgical day.
    
    The liturgical precedence
    """
    ASH_WEDNESDAY_2 = "ASH_WEDNESDAY_2"
    COMMEMORATION_OF_ALL_THE_FAITHFUL_DEPARTED_3 = "COMMEMORATION_OF_ALL_THE_FAITHFUL_DEPARTED_3"
    GENERAL_FEAST_7 = "GENERAL_FEAST_7"
    GENERAL_LORD_FEAST_5 = "GENERAL_LORD_FEAST_5"
    GENERAL_MEMORIAL_10 = "GENERAL_MEMORIAL_10"
    GENERAL_SOLEMNITY_3 = "GENERAL_SOLEMNITY_3"
    OPTIONAL_MEMORIAL_12 = "OPTIONAL_MEMORIAL_12"
    PRIVILEGED_SUNDAY_2 = "PRIVILEGED_SUNDAY_2"
    PRIVILEGED_WEEKDAY_9 = "PRIVILEGED_WEEKDAY_9"
    PROPER_FEAST_8_F = "PROPER_FEAST_8F"
    PROPER_FEAST_DEDICATION_OF_THE_CATHEDRAL_CHURCH_8_B = "PROPER_FEAST__DEDICATION_OF_THE_CATHEDRAL_CHURCH_8B"
    PROPER_FEAST_PRINCIPAL_PATRON_OF_A_DIOCESE_8_A = "PROPER_FEAST__PRINCIPAL_PATRON_OF_A_DIOCESE_8A"
    PROPER_FEAST_PRINCIPAL_PATRON_OF_A_REGION_8_C = "PROPER_FEAST__PRINCIPAL_PATRON_OF_A_REGION_8C"
    PROPER_FEAST_TITLE_OR_FOUNDER_OR_PRIMARY_PATRON_OF_A_RELIGIOUS_ORG_8_D = "PROPER_FEAST__TITLE_OR_FOUNDER_OR_PRIMARY_PATRON_OF_A_RELIGIOUS_ORG_8D"
    PROPER_FEAST_TO_AN_INDIVIDUAL_CHURCH_8_E = "PROPER_FEAST__TO_AN_INDIVIDUAL_CHURCH_8E"
    PROPER_MEMORIAL_11_B = "PROPER_MEMORIAL_11B"
    PROPER_MEMORIAL_SECOND_PATRON_11_A = "PROPER_MEMORIAL__SECOND_PATRON_11A"
    PROPER_OF_TIME_SOLEMNITY_2 = "PROPER_OF_TIME_SOLEMNITY_2"
    PROPER_SOLEMNITY_DEDICATION_OF_THE_OWN_CHURCH_4_B = "PROPER_SOLEMNITY__DEDICATION_OF_THE_OWN_CHURCH_4B"
    PROPER_SOLEMNITY_PRINCIPAL_PATRON_4_A = "PROPER_SOLEMNITY__PRINCIPAL_PATRON_4A"
    PROPER_SOLEMNITY_TITLE_OF_THE_OWN_CHURCH_4_C = "PROPER_SOLEMNITY__TITLE_OF_THE_OWN_CHURCH_4C"
    PROPER_SOLEMNITY_TITLE_OR_FOUNDER_OR_PRIMARY_PATRON_OF_A_RELIGIOUS_ORG_4_D = "PROPER_SOLEMNITY__TITLE_OR_FOUNDER_OR_PRIMARY_PATRON_OF_A_RELIGIOUS_ORG_4D"
    TRIDUUM_1 = "TRIDUUM_1"
    UNPRIVILEGED_SUNDAY_6 = "UNPRIVILEGED_SUNDAY_6"
    WEEKDAY_13 = "WEEKDAY_13"
    WEEKDAY_OF_EASTER_OCTAVE_2 = "WEEKDAY_OF_EASTER_OCTAVE_2"
    WEEKDAY_OF_HOLY_WEEK_2 = "WEEKDAY_OF_HOLY_WEEK_2"


class DayDefinition(BaseModel):
    """Definition of a liturgical day with all its properties and configurations.
    It represents a complete liturgical day definition that can be used
    to generate calendar entries with proper precedence, colors, and entity associations.
    """
    allow_similar_rank_items: Optional[bool] = None
    """Allow similar items that have the same rank and the same or lower precedence
    to coexist with this liturgical day without being overwritten
    """
    colors: Optional[Union[List[Color], Color]] = None
    """The liturgical color(s) of the liturgical day.
    
    **Deprecated:** Rely on the `titles` field of entities instead to determine the
    liturgical color(s).
    """
    commons_def: Optional[Union[List[CommonDefinition], CommonDefinition]] = None
    """The **Common** refers to a set of prayers, readings, and chants used for celebrating
    saints or
    feasts that belong to a specific category, such as martyrs, virgins, pastors, or the
    Blessed
    Virgin Mary.
    """
    custom_locale_id: Optional[str] = None
    """The custom locale ID for this date definition in this calendar"""

    date_def: Optional[DateDefClass] = None
    """The date definition for this liturgical day"""

    date_exceptions: Optional[Union[DateDefException, List[DateDefException]]] = None
    """The date definition exceptions (overrides for specific circumstances)"""

    drop: Optional[bool] = None
    """If this liturgical day must be removed from this calendar and from all parent calendars
    in the final calendar generated by romcal
    """
    entities: Optional[List[Union[EntityOverride, str]]] = None
    """The entities (Saints, Blessed, or Places) linked from the Entity catalog"""

    is_holy_day_of_obligation: Optional[bool] = None
    """Holy days of obligation are days on which the faithful are expected to attend Mass
    and engage in rest from work and recreation
    """
    is_optional: Optional[bool] = None
    """Specify if this liturgical day is optional within a specific liturgical calendar
    
    UNLY #14:
    Memorials are either obligatory or optional; their observance is integrated into
    the celebration of the occurring weekday in accordance with the norms set forth in the
    General Instruction of the Roman Missal and of the Liturgy of the Hours
    
    Note: also used for the dedication of consecrated churches, which is an optional
    solemnity
    that should not overwrite the default weekday.
    """
    masses: Optional[MassesDefinitions] = None
    """The masses definitions for this liturgical day"""

    precedence: Optional[Precedence] = None
    """The precedence type of the liturgical day"""

    titles: Optional[Union[List[Title], CompoundTitle]] = None
    """The combined titles of all entities linked to this date definition"""

    @staticmethod
    def from_dict(obj: Any) -> 'DayDefinition':
        assert isinstance(obj, dict)
        allow_similar_rank_items = from_union([from_none, from_bool], obj.get("allow_similar_rank_items"))
        colors = from_union([lambda x: from_list(Color, x), from_none, Color], obj.get("colors"))
        commons_def = from_union([lambda x: from_list(CommonDefinition, x), from_none, CommonDefinition], obj.get("commons_def"))
        custom_locale_id = from_union([from_none, from_str], obj.get("custom_locale_id"))
        date_def = from_union([DateDefClass.from_dict, from_none], obj.get("date_def"))
        date_exceptions = from_union([DateDefException.from_dict, lambda x: from_list(DateDefException.from_dict, x), from_none], obj.get("date_exceptions"))
        drop = from_union([from_none, from_bool], obj.get("drop"))
        entities = from_union([from_none, lambda x: from_list(lambda x: from_union([EntityOverride.from_dict, from_str], x), x)], obj.get("entities"))
        is_holy_day_of_obligation = from_union([from_none, from_bool], obj.get("is_holy_day_of_obligation"))
        is_optional = from_union([from_none, from_bool], obj.get("is_optional"))
        masses = from_union([MassesDefinitions.from_dict, from_none], obj.get("masses"))
        precedence = from_union([from_none, Precedence], obj.get("precedence"))
        titles = from_union([lambda x: from_list(Title, x), CompoundTitle.from_dict, from_none], obj.get("titles"))
        return DayDefinition(allow_similar_rank_items, colors, commons_def, custom_locale_id, date_def, date_exceptions, drop, entities, is_holy_day_of_obligation, is_optional, masses, precedence, titles)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.allow_similar_rank_items is not None:
            result["allow_similar_rank_items"] = from_union([from_none, from_bool], self.allow_similar_rank_items)
        if self.colors is not None:
            result["colors"] = from_union([lambda x: from_list(lambda x: to_enum(Color, x), x), from_none, lambda x: to_enum(Color, x)], self.colors)
        if self.commons_def is not None:
            result["commons_def"] = from_union([lambda x: from_list(lambda x: to_enum(CommonDefinition, x), x), from_none, lambda x: to_enum(CommonDefinition, x)], self.commons_def)
        if self.custom_locale_id is not None:
            result["custom_locale_id"] = from_union([from_none, from_str], self.custom_locale_id)
        if self.date_def is not None:
            result["date_def"] = from_union([lambda x: to_class(DateDefClass, x), from_none], self.date_def)
        if self.date_exceptions is not None:
            result["date_exceptions"] = from_union([lambda x: to_class(DateDefException, x), lambda x: from_list(lambda x: to_class(DateDefException, x), x), from_none], self.date_exceptions)
        if self.drop is not None:
            result["drop"] = from_union([from_none, from_bool], self.drop)
        if self.entities is not None:
            result["entities"] = from_union([from_none, lambda x: from_list(lambda x: from_union([lambda x: to_class(EntityOverride, x), from_str], x), x)], self.entities)
        if self.is_holy_day_of_obligation is not None:
            result["is_holy_day_of_obligation"] = from_union([from_none, from_bool], self.is_holy_day_of_obligation)
        if self.is_optional is not None:
            result["is_optional"] = from_union([from_none, from_bool], self.is_optional)
        if self.masses is not None:
            result["masses"] = from_union([lambda x: to_class(MassesDefinitions, x), from_none], self.masses)
        if self.precedence is not None:
            result["precedence"] = from_union([from_none, lambda x: to_enum(Precedence, x)], self.precedence)
        if self.titles is not None:
            result["titles"] = from_union([lambda x: from_list(lambda x: to_enum(Title, x), x), lambda x: to_class(CompoundTitle, x), from_none], self.titles)
        return result


class CalendarJurisdiction(Enum):
    """The jurisdiction of the calendar
    
    The jurisdiction of the calendar.
    Determines whether the calendar follows ecclesiastical or civil authority.
    
    Calendar under ecclesiastical authority (Church)
    
    Calendar under civil authority (State)
    """
    CIVIL = "CIVIL"
    ECCLESIASTICAL = "ECCLESIASTICAL"


class CalendarType(Enum):
    """The type of the calendar
    
    The type of the calendar.
    Defines the scope and authority level of the liturgical calendar.
    
    General Roman Calendar (universal)
    
    Regional calendar (multiple countries)
    
    National calendar (single country)
    
    Archdiocesan calendar
    
    Diocesan calendar
    
    City calendar
    
    Parish calendar
    
    General religious community calendar
    
    Regional religious community calendar
    
    Local religious community calendar
    
    Other specialized calendar
    """
    ARCHDIOCESE = "ARCHDIOCESE"
    CITY = "CITY"
    COUNTRY = "COUNTRY"
    DIOCESE = "DIOCESE"
    GENERAL_COMMUNITY = "GENERAL_COMMUNITY"
    GENERAL_ROMAN = "GENERAL_ROMAN"
    LOCAL_COMMUNITY = "LOCAL_COMMUNITY"
    OTHER = "OTHER"
    PARISH = "PARISH"
    REGION = "REGION"
    REGIONAL_COMMUNITY = "REGIONAL_COMMUNITY"


class CalendarMetadata(BaseModel):
    """Metadata for a calendar.
    Contains essential information about the calendar's type and jurisdiction.
    """
    jurisdiction: CalendarJurisdiction
    """The jurisdiction of the calendar"""

    type: CalendarType
    """The type of the calendar"""

    @staticmethod
    def from_dict(obj: Any) -> 'CalendarMetadata':
        assert isinstance(obj, dict)
        jurisdiction = CalendarJurisdiction(obj.get("jurisdiction"))
        type = CalendarType(obj.get("type"))
        return CalendarMetadata(jurisdiction, type)

    def to_dict(self) -> dict:
        result: dict = {}
        result["jurisdiction"] = to_enum(CalendarJurisdiction, self.jurisdiction)
        result["type"] = to_enum(CalendarType, self.type)
        return result


class EasterCalculationType(Enum):
    """Gregorian calculation (default)
    
    Julian calculation converted to Gregorian
    """
    GREGORIAN = "GREGORIAN"
    JULIAN = "JULIAN"


class ParticularConfig(BaseModel):
    """Configuration options for "particular" (local/diocesan) calendars.
    
    In liturgical terminology, a "particular" calendar is one that applies to a specific
    region, diocese, or religious community, as opposed to the General Roman Calendar
    which applies universally.
    
    These settings can override or extend the default Romcal configuration or any parent
    calendar configuration.
    """
    ascension_on_sunday: Optional[bool] = None
    """Ascension is celebrated on a Sunday"""

    corpus_christi_on_sunday: Optional[bool] = None
    """Corpus Christi is celebrated on a Sunday"""

    easter_calculation_type: Optional[EasterCalculationType] = None
    """The type of Easter calculation"""

    epiphany_on_sunday: Optional[bool] = None
    """Epiphany is celebrated on a Sunday"""

    @staticmethod
    def from_dict(obj: Any) -> 'ParticularConfig':
        assert isinstance(obj, dict)
        ascension_on_sunday = from_union([from_none, from_bool], obj.get("ascension_on_sunday"))
        corpus_christi_on_sunday = from_union([from_none, from_bool], obj.get("corpus_christi_on_sunday"))
        easter_calculation_type = from_union([from_none, EasterCalculationType], obj.get("easter_calculation_type"))
        epiphany_on_sunday = from_union([from_none, from_bool], obj.get("epiphany_on_sunday"))
        return ParticularConfig(ascension_on_sunday, corpus_christi_on_sunday, easter_calculation_type, epiphany_on_sunday)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.ascension_on_sunday is not None:
            result["ascension_on_sunday"] = from_union([from_none, from_bool], self.ascension_on_sunday)
        if self.corpus_christi_on_sunday is not None:
            result["corpus_christi_on_sunday"] = from_union([from_none, from_bool], self.corpus_christi_on_sunday)
        if self.easter_calculation_type is not None:
            result["easter_calculation_type"] = from_union([from_none, lambda x: to_enum(EasterCalculationType, x)], self.easter_calculation_type)
        if self.epiphany_on_sunday is not None:
            result["epiphany_on_sunday"] = from_union([from_none, from_bool], self.epiphany_on_sunday)
        return result


class CalendarDefinition(BaseModel):
    """Calendar definition"""

    days_definitions: Dict[str, DayDefinition]
    id: str
    metadata: CalendarMetadata
    parent_calendar_ids: List[str]
    schema: Optional[str] = None
    particular_config: Optional[ParticularConfig] = None

    @staticmethod
    def from_dict(obj: Any) -> 'CalendarDefinition':
        assert isinstance(obj, dict)
        days_definitions = from_dict(DayDefinition.from_dict, obj.get("days_definitions"))
        id = from_str(obj.get("id"))
        metadata = CalendarMetadata.from_dict(obj.get("metadata"))
        parent_calendar_ids = from_list(from_str, obj.get("parent_calendar_ids"))
        schema = from_union([from_none, from_str], obj.get("$schema"))
        particular_config = from_union([ParticularConfig.from_dict, from_none], obj.get("particular_config"))
        return CalendarDefinition(days_definitions, id, metadata, parent_calendar_ids, schema, particular_config)

    def to_dict(self) -> dict:
        result: dict = {}
        result["days_definitions"] = from_dict(lambda x: to_class(DayDefinition, x), self.days_definitions)
        result["id"] = from_str(self.id)
        result["metadata"] = to_class(CalendarMetadata, self.metadata)
        result["parent_calendar_ids"] = from_list(from_str, self.parent_calendar_ids)
        if self.schema is not None:
            result["$schema"] = from_union([from_none, from_str], self.schema)
        if self.particular_config is not None:
            result["particular_config"] = from_union([lambda x: to_class(ParticularConfig, x), from_none], self.particular_config)
        return result


class ColorInfo(BaseModel):
    """Liturgical color information with localized name."""

    key: Color
    """The color key"""

    name: str
    """The localized name of the color"""

    @staticmethod
    def from_dict(obj: Any) -> 'ColorInfo':
        assert isinstance(obj, dict)
        key = Color(obj.get("key"))
        name = from_str(obj.get("name"))
        return ColorInfo(key, name)

    def to_dict(self) -> dict:
        result: dict = {}
        result["key"] = to_enum(Color, self.key)
        result["name"] = from_str(self.name)
        return result


class Common(Enum):
    """The common key
    
    Common prayers and readings for different categories of saints and celebrations.
    Provides standardized liturgical texts for various types of commemorations.
    
    No common.
    
    Dedication anniversary (in the Church that was Dedicated).
    
    Dedication anniversary (outside the Church that was Dedicated).
    
    Common of the Blessed Virgin Mary (Ordinary Time).
    
    Common of the Blessed Virgin Mary (Advent).
    
    Common of the Blessed Virgin Mary (Christmas Time).
    
    Common of the Blessed Virgin Mary (Easter Time).
    
    Common of Several Martyrs (outside Easter).
    
    Common of One Martyr (outside Easter).
    
    Common of Several Martyrs (Easter Time).
    
    Common of One Martyr (Easter Time).
    
    Common for Several Missionary Martyrs.
    
    Common for One Missionary Martyr.
    
    Common for Virgin Martyrs.
    
    Common for Holy Woman Martyrs.
    
    Common for a Pope or for a Bishop
    
    Common for a Bishop
    
    Common for Several Pastors
    
    Common for One Pastor
    
    Common for one Founder
    
    Common for several Founders
    
    Common for Missionaries
    
    Common for Doctors of the Church.
    
    Common for Several Virgins
    
    Common for One Virgin
    
    Common for Several Holy Men and Women
    
    Common for One Holy Man or Woman
    
    Common for an Abbot
    
    Common for a Monk
    
    Common for a Nun
    
    Common for Religious
    
    Common for Those Who Practiced Works of Mercy
    
    Common for Educators
    
    Common for Holy Women
    """
    BLESSED_VIRGIN_MARY_ADVENT = "BLESSED_VIRGIN_MARY__ADVENT"
    BLESSED_VIRGIN_MARY_CHRISTMAS = "BLESSED_VIRGIN_MARY__CHRISTMAS"
    BLESSED_VIRGIN_MARY_EASTER = "BLESSED_VIRGIN_MARY__EASTER"
    BLESSED_VIRGIN_MARY_ORDINARY_TIME = "BLESSED_VIRGIN_MARY__ORDINARY_TIME"
    DEDICATION_ANNIVERSARY_INSIDE = "DEDICATION_ANNIVERSARY__INSIDE"
    DEDICATION_ANNIVERSARY_OUTSIDE = "DEDICATION_ANNIVERSARY__OUTSIDE"
    DOCTORS_OF_THE_CHURCH = "DOCTORS_OF_THE_CHURCH"
    MARTYRS_EASTER_ONE = "MARTYRS__EASTER__ONE"
    MARTYRS_EASTER_SEVERAL = "MARTYRS__EASTER__SEVERAL"
    MARTYRS_MISSIONARY_ONE = "MARTYRS__MISSIONARY__ONE"
    MARTYRS_MISSIONARY_SEVERAL = "MARTYRS__MISSIONARY__SEVERAL"
    MARTYRS_OUTSIDE_EASTER_ONE = "MARTYRS__OUTSIDE_EASTER__ONE"
    MARTYRS_OUTSIDE_EASTER_SEVERAL = "MARTYRS__OUTSIDE_EASTER__SEVERAL"
    MARTYRS_VIRGIN = "MARTYRS__VIRGIN"
    MARTYRS_WOMAN = "MARTYRS__WOMAN"
    NONE = "NONE"
    PASTORS_BISHOP = "PASTORS__BISHOP"
    PASTORS_FOUNDER_ONE = "PASTORS__FOUNDER__ONE"
    PASTORS_FOUNDER_SEVERAL = "PASTORS__FOUNDER__SEVERAL"
    PASTORS_MISSIONARY = "PASTORS__MISSIONARY"
    PASTORS_ONE = "PASTORS__ONE"
    PASTORS_POPE_OR_BISHOP = "PASTORS__POPE_OR_BISHOP"
    PASTORS_SEVERAL = "PASTORS__SEVERAL"
    SAINTS_ABBOT = "SAINTS__ABBOT"
    SAINTS_ALL_ONE = "SAINTS__ALL__ONE"
    SAINTS_ALL_SEVERAL = "SAINTS__ALL__SEVERAL"
    SAINTS_EDUCATORS = "SAINTS__EDUCATORS"
    SAINTS_HOLY_WOMEN = "SAINTS__HOLY_WOMEN"
    SAINTS_MERCY_WORKS = "SAINTS__MERCY_WORKS"
    SAINTS_NUN = "SAINTS__NUN"
    SAINTS_RELIGIOUS = "SAINTS__RELIGIOUS"
    SAINT_MONK = "SAINT__MONK"
    VIRGINS_ONE = "VIRGINS__ONE"
    VIRGINS_SEVERAL = "VIRGINS__SEVERAL"


class CommonInfo(BaseModel):
    """Liturgical common information with localized name."""

    key: Common
    """The common key"""

    name: str
    """The localized name of the common"""

    @staticmethod
    def from_dict(obj: Any) -> 'CommonInfo':
        assert isinstance(obj, dict)
        key = Common(obj.get("key"))
        name = from_str(obj.get("name"))
        return CommonInfo(key, name)

    def to_dict(self) -> dict:
        result: dict = {}
        result["key"] = to_enum(Common, self.key)
        result["name"] = from_str(self.name)
        return result


class CanonizationLevel(Enum):
    """Beatified person (Blessed) - first step toward sainthood
    
    Canonized person (Saint) - fully recognized as a saint
    """
    BLESSED = "BLESSED"
    SAINT = "SAINT"


class SaintDateDef(BaseModel):
    """Date range between two dates
    
    Multiple alternative dates (any one of them)
    
    Century specification (e.g., 12 for 12th century)
    """
    between: Optional[List[Union[int, str]]] = None
    """The date range (start and end dates)"""

    saint_date_def_or: Optional[List[Union[int, str]]] = None
    """The list of alternative dates"""

    century: Optional[int] = None
    """The century number"""

    @staticmethod
    def from_dict(obj: Any) -> 'SaintDateDef':
        assert isinstance(obj, dict)
        between = from_union([lambda x: from_list(lambda x: from_union([from_int, from_str], x), x), from_none], obj.get("between"))
        saint_date_def_or = from_union([lambda x: from_list(lambda x: from_union([from_int, from_str], x), x), from_none], obj.get("or"))
        century = from_union([from_int, from_none], obj.get("century"))
        return SaintDateDef(between, saint_date_def_or, century)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.between is not None:
            result["between"] = from_union([lambda x: from_list(lambda x: from_union([from_int, from_str], x), x), from_none], self.between)
        if self.saint_date_def_or is not None:
            result["or"] = from_union([lambda x: from_list(lambda x: from_union([from_int, from_str], x), x), from_none], self.saint_date_def_or)
        if self.century is not None:
            result["century"] = from_union([from_int, from_none], self.century)
        return result


class Sex(Enum):
    """Male person
    
    Female person
    """
    FEMALE = "FEMALE"
    MALE = "MALE"


class EntityType(Enum):
    """A person (saint, blessed, or other individual)
    
    A place (shrine, city, or geographical location)
    
    An event (historical or liturgical occurrence)
    """
    EVENT = "EVENT"
    PERSON = "PERSON"
    PLACE = "PLACE"


class Entity(BaseModel):
    todo: Optional[List[str]] = None
    """Internal notes (not serialized)."""

    canonization_level: Optional[CanonizationLevel] = None
    """The canonization level of a person."""

    count: Optional[Union[int, SaintCountEnum]] = None
    """Number of person that this definition represent.
    It could be set as 'many' if the number is not defined.
    """
    date_of_beatification: Optional[Union[int, SaintDateDef, str]] = None
    """Date of Beatification, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD'
    format),
    or an object describing date range, multiple possible date, or a century.
    """
    date_of_beatification_is_approximative: Optional[bool] = None
    """Specify whether an approximate indicator should be added, when the date is displayed.
    For example in English: 'c. 201'.
    """
    date_of_birth: Optional[Union[int, SaintDateDef, str]] = None
    """Date of Birth, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
    or an object describing date range, multiple possible date, or a century.
    """
    date_of_birth_is_approximative: Optional[bool] = None
    """Specify whether an approximate indicator should be added, when the date is displayed.
    For example in English: 'c. 201'.
    """
    date_of_canonization: Optional[Union[int, SaintDateDef, str]] = None
    """Date of Canonization, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
    or an object describing date range, multiple possible date, or a century.
    """
    date_of_canonization_is_approximative: Optional[bool] = None
    """Specify whether an approximate indicator should be added, when the date is displayed.
    For example in English: 'c. 201'.
    """
    date_of_death: Optional[Union[int, SaintDateDef, str]] = None
    """Date of Death, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
    or an object describing date range, multiple possible date, or a century.
    """
    date_of_death_is_approximative: Optional[bool] = None
    """Specify whether an approximate indicator should be added, when the date is displayed.
    For example in English: 'c. 201'.
    """
    date_of_dedication: Optional[Union[int, SaintDateDef, str]] = None
    """Date of Dedication of a church, basilica, or cathedral (or other place of worship),
    as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
    or an object describing date range, multiple possible date, or a century.
    """
    fullname: Optional[str] = None
    """The full name of the entity."""

    hide_canonization_level: Optional[bool] = None
    """Specify if the canonization level should not be displayed.
    It's generally the case when the canonization are already included in the name.
    """
    hide_titles: Optional[bool] = None
    """Specify if the titles should not be displayed.
    It's generally the case when titles are already included in the name.
    """
    id: Optional[str] = None
    """The unique identifier of the entity"""

    name: Optional[str] = None
    """The short name of the entity, without the canonization level and titles."""

    sex: Optional[Sex] = None
    """Determine if the Saint or the Blessed is a male or a female."""

    sources: Optional[List[str]] = None
    """Sources for the information about this entity"""

    titles: Optional[List[Title]] = None
    """Titles of the Saint or the Blessed"""

    type: Optional[EntityType] = None
    """The type of the entity.
    
    Defaults to `EntityType::Person`.
    """

    @staticmethod
    def from_dict(obj: Any) -> 'Entity':
        assert isinstance(obj, dict)
        todo = from_union([from_none, lambda x: from_list(from_str, x)], obj.get("_todo"))
        canonization_level = from_union([from_none, CanonizationLevel], obj.get("canonization_level"))
        count = from_union([from_int, from_none, SaintCountEnum], obj.get("count"))
        date_of_beatification = from_union([from_int, SaintDateDef.from_dict, from_none, from_str], obj.get("date_of_beatification"))
        date_of_beatification_is_approximative = from_union([from_none, from_bool], obj.get("date_of_beatification_is_approximative"))
        date_of_birth = from_union([from_int, SaintDateDef.from_dict, from_none, from_str], obj.get("date_of_birth"))
        date_of_birth_is_approximative = from_union([from_none, from_bool], obj.get("date_of_birth_is_approximative"))
        date_of_canonization = from_union([from_int, SaintDateDef.from_dict, from_none, from_str], obj.get("date_of_canonization"))
        date_of_canonization_is_approximative = from_union([from_none, from_bool], obj.get("date_of_canonization_is_approximative"))
        date_of_death = from_union([from_int, SaintDateDef.from_dict, from_none, from_str], obj.get("date_of_death"))
        date_of_death_is_approximative = from_union([from_none, from_bool], obj.get("date_of_death_is_approximative"))
        date_of_dedication = from_union([from_int, SaintDateDef.from_dict, from_none, from_str], obj.get("date_of_dedication"))
        fullname = from_union([from_none, from_str], obj.get("fullname"))
        hide_canonization_level = from_union([from_none, from_bool], obj.get("hide_canonization_level"))
        hide_titles = from_union([from_none, from_bool], obj.get("hide_titles"))
        id = from_union([from_none, from_str], obj.get("id"))
        name = from_union([from_none, from_str], obj.get("name"))
        sex = from_union([from_none, Sex], obj.get("sex"))
        sources = from_union([from_none, lambda x: from_list(from_str, x)], obj.get("sources"))
        titles = from_union([from_none, lambda x: from_list(Title, x)], obj.get("titles"))
        type = from_union([from_none, EntityType], obj.get("type"))
        return Entity(todo, canonization_level, count, date_of_beatification, date_of_beatification_is_approximative, date_of_birth, date_of_birth_is_approximative, date_of_canonization, date_of_canonization_is_approximative, date_of_death, date_of_death_is_approximative, date_of_dedication, fullname, hide_canonization_level, hide_titles, id, name, sex, sources, titles, type)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.todo is not None:
            result["_todo"] = from_union([from_none, lambda x: from_list(from_str, x)], self.todo)
        if self.canonization_level is not None:
            result["canonization_level"] = from_union([from_none, lambda x: to_enum(CanonizationLevel, x)], self.canonization_level)
        if self.count is not None:
            result["count"] = from_union([from_int, from_none, lambda x: to_enum(SaintCountEnum, x)], self.count)
        if self.date_of_beatification is not None:
            result["date_of_beatification"] = from_union([from_int, lambda x: to_class(SaintDateDef, x), from_none, from_str], self.date_of_beatification)
        if self.date_of_beatification_is_approximative is not None:
            result["date_of_beatification_is_approximative"] = from_union([from_none, from_bool], self.date_of_beatification_is_approximative)
        if self.date_of_birth is not None:
            result["date_of_birth"] = from_union([from_int, lambda x: to_class(SaintDateDef, x), from_none, from_str], self.date_of_birth)
        if self.date_of_birth_is_approximative is not None:
            result["date_of_birth_is_approximative"] = from_union([from_none, from_bool], self.date_of_birth_is_approximative)
        if self.date_of_canonization is not None:
            result["date_of_canonization"] = from_union([from_int, lambda x: to_class(SaintDateDef, x), from_none, from_str], self.date_of_canonization)
        if self.date_of_canonization_is_approximative is not None:
            result["date_of_canonization_is_approximative"] = from_union([from_none, from_bool], self.date_of_canonization_is_approximative)
        if self.date_of_death is not None:
            result["date_of_death"] = from_union([from_int, lambda x: to_class(SaintDateDef, x), from_none, from_str], self.date_of_death)
        if self.date_of_death_is_approximative is not None:
            result["date_of_death_is_approximative"] = from_union([from_none, from_bool], self.date_of_death_is_approximative)
        if self.date_of_dedication is not None:
            result["date_of_dedication"] = from_union([from_int, lambda x: to_class(SaintDateDef, x), from_none, from_str], self.date_of_dedication)
        if self.fullname is not None:
            result["fullname"] = from_union([from_none, from_str], self.fullname)
        if self.hide_canonization_level is not None:
            result["hide_canonization_level"] = from_union([from_none, from_bool], self.hide_canonization_level)
        if self.hide_titles is not None:
            result["hide_titles"] = from_union([from_none, from_bool], self.hide_titles)
        if self.id is not None:
            result["id"] = from_union([from_none, from_str], self.id)
        if self.name is not None:
            result["name"] = from_union([from_none, from_str], self.name)
        if self.sex is not None:
            result["sex"] = from_union([from_none, lambda x: to_enum(Sex, x)], self.sex)
        if self.sources is not None:
            result["sources"] = from_union([from_none, lambda x: from_list(from_str, x)], self.sources)
        if self.titles is not None:
            result["titles"] = from_union([from_none, lambda x: from_list(lambda x: to_enum(Title, x), x)], self.titles)
        if self.type is not None:
            result["type"] = from_union([from_none, lambda x: to_enum(EntityType, x)], self.type)
        return result


class Rank(Enum):
    """The liturgical rank for this liturgical day
    
    Liturgical rank indicating the importance and celebration style of a liturgical day
    
    The liturgical rank for this liturgical day.
    
    The liturgical rank
    
    Solemnities are counted among the most important days, whose celebration
    begins with First Vespers (Evening Prayer I) on the preceding day. Some Solemnities
    are also endowed with their own Vigil Mass, which is to be used on the evening of the
    preceding day, if an evening Mass is celebrated. (UNLY #11)
    
    On the first day of each week, which is known as the Day of the Lord or the Lord's
    Day, the Church, by an apostolic tradition that draws its origin from the very day of
    the Resurrection of Christ, celebrates the Paschal Mystery. Hence, Sunday must be
    considered the primordial feast day. (UNLY #4)
    
    Feasts are celebrated within the limits of the natural day; accordingly they have
    no First Vespers (Evening Prayer I), except in the case of Feasts of the Lord that fall
    on a Sunday in Ordinary Time or in Christmas Time and which replace the Sunday
    Office. (UNLY #13)
    
    **Obligatory memorials** are liturgical commemorations of saints, events, or aspects of
    the
    faith. Their observance is mandatory and integrated into the celebration of the occurring
    weekday, following the liturgical norms outlined in the General Instruction of the Roman
    Missal
    and the Liturgy of the Hours.
    When an **obligatory memorial** falls on a weekday during the liturgical season of Lent
    or a
    privileged weekday of Advent, it must only be celebrated as an **optional memorial**, as
    Lent
    and Advent have their own specific liturgical observances that take precedence.
    
    **Optional memorials** are liturgical commemorations of saints, events, or aspects of the
    faith, but they are not obligatory.
    Their observance is integrated into the celebration of the occurring weekday, adhering to
    the
    liturgical norms provided in the General Instruction of the Roman Missal and the Liturgy
    of
    the Hours.
    In cases where multiple **optional memorials** are designated on the same day in the
    liturgical
    calendar, only one of them may be celebrated, and the others must be omitted (UNLY #14).
    This allows for some flexibility in choosing which optional memorial to commemorate when
    multiple options are available.
    
    The days of the week that follow Sunday are called weekdays; however, they are
    celebrated differently according to the importance of each.
    
    a. Ash Wednesday and the weekdays of Holy Week, from Monday up to and including
    Thursday, take precedence over all other celebrations.
    b. The weekdays of Advent from 17 December up to and including 24 December
    and all the weekdays of Lent have precedence over Obligatory Memorials.
    c. Other weekdays give way to all Solemnities and Feasts and are combined with
    Memorials.
    
    (UNLY #16)
    """
    FEAST = "FEAST"
    MEMORIAL = "MEMORIAL"
    OPTIONAL_MEMORIAL = "OPTIONAL_MEMORIAL"
    SOLEMNITY = "SOLEMNITY"
    SUNDAY = "SUNDAY"
    WEEKDAY = "WEEKDAY"


class CelebrationSummary(BaseModel):
    """Summary of a celebration for use in optional celebrations list.
    Contains the essential fields from a LiturgicalDay that identify a celebration.
    """
    colors: List[ColorInfo]
    """The liturgical colors for this liturgical day"""

    commons: List[CommonInfo]
    """The common prayers/readings used for this celebration"""

    entities: List[Entity]
    """The entities (Saints, Blessed, or Places) linked to this liturgical day"""

    from_calendar_id: str
    """The ID of the calendar where this liturgical day is defined"""

    fullname: str
    """The full name of the liturgical day"""

    id: str
    """The unique identifier of the liturgical day"""

    is_holy_day_of_obligation: bool
    """Holy days of obligation"""

    is_optional: bool
    """Indicates if this liturgical day is optional"""

    precedence: Precedence
    """The liturgical precedence for this liturgical day"""

    rank: Rank
    """The liturgical rank for this liturgical day"""

    rank_name: str
    """The localized liturgical rank for this liturgical day"""

    titles: Union[List[Title], CompoundTitle]
    """The titles for this liturgical day"""

    @staticmethod
    def from_dict(obj: Any) -> 'CelebrationSummary':
        assert isinstance(obj, dict)
        colors = from_list(ColorInfo.from_dict, obj.get("colors"))
        commons = from_list(CommonInfo.from_dict, obj.get("commons"))
        entities = from_list(Entity.from_dict, obj.get("entities"))
        from_calendar_id = from_str(obj.get("from_calendar_id"))
        fullname = from_str(obj.get("fullname"))
        id = from_str(obj.get("id"))
        is_holy_day_of_obligation = from_bool(obj.get("is_holy_day_of_obligation"))
        is_optional = from_bool(obj.get("is_optional"))
        precedence = Precedence(obj.get("precedence"))
        rank = Rank(obj.get("rank"))
        rank_name = from_str(obj.get("rank_name"))
        titles = from_union([lambda x: from_list(Title, x), CompoundTitle.from_dict], obj.get("titles"))
        return CelebrationSummary(colors, commons, entities, from_calendar_id, fullname, id, is_holy_day_of_obligation, is_optional, precedence, rank, rank_name, titles)

    def to_dict(self) -> dict:
        result: dict = {}
        result["colors"] = from_list(lambda x: to_class(ColorInfo, x), self.colors)
        result["commons"] = from_list(lambda x: to_class(CommonInfo, x), self.commons)
        result["entities"] = from_list(lambda x: to_class(Entity, x), self.entities)
        result["from_calendar_id"] = from_str(self.from_calendar_id)
        result["fullname"] = from_str(self.fullname)
        result["id"] = from_str(self.id)
        result["is_holy_day_of_obligation"] = from_bool(self.is_holy_day_of_obligation)
        result["is_optional"] = from_bool(self.is_optional)
        result["precedence"] = to_enum(Precedence, self.precedence)
        result["rank"] = to_enum(Rank, self.rank)
        result["rank_name"] = from_str(self.rank_name)
        result["titles"] = from_union([lambda x: from_list(lambda x: to_enum(Title, x), x), lambda x: to_class(CompoundTitle, x)], self.titles)
        return result


class DateDefWithOffset(BaseModel):
    """Date definition with offset
    
    Date definition with offset for adjustments.
    Used when a date needs to be shifted by a specific number of days.
    """
    day_offset: int
    """The number of days to offset the date"""

    @staticmethod
    def from_dict(obj: Any) -> 'DateDefWithOffset':
        assert isinstance(obj, dict)
        day_offset = from_int(obj.get("day_offset"))
        return DateDefWithOffset(day_offset)

    def to_dict(self) -> dict:
        result: dict = {}
        result["day_offset"] = from_int(self.day_offset)
        return result


class LiturgicalCycle(Enum):
    """Liturgical cycle for lectionary readings
    Includes both actual cycles (Year A, B, C, etc.) and invariant content
    
    Invariant content that applies to all cycles
    
    Year A of the Sunday cycle
    
    Year B of the Sunday cycle
    
    Year C of the Sunday cycle
    
    Combined years A and B of the Sunday cycle
    
    Combined years A and C of the Sunday cycle
    
    Combined years B and C of the Sunday cycle
    
    Year 1 of the weekday cycle (Cycle I)
    
    Year 2 of the weekday cycle (Cycle II)
    """
    INVARIANT = "invariant"
    YEAR_1 = "year_1"
    YEAR_2 = "year_2"
    YEAR_A = "year_a"
    YEAR_A_B = "year_a_b"
    YEAR_A_C = "year_a_c"
    YEAR_B = "year_b"
    YEAR_B_C = "year_b_c"
    YEAR_C = "year_c"


class MassTime(Enum):
    """The type of mass (e.g., DayMass, EasterVigil, etc.)
    Serialized as SCREAMING_SNAKE_CASE (e.g., "DAY_MASS")
    
    Times of Mass celebrations in the liturgical calendar.
    Different Masses are celebrated at various times and occasions throughout the liturgical
    year.
    
    Easter Vigil - the most important Mass of the liturgical year, celebrated on Holy
    Saturday night
    
    Previous Evening Mass - Mass celebrated the evening before a major feast
    
    Night Mass - Mass celebrated during the night hours
    
    Mass at Dawn - Mass celebrated at dawn, particularly on Easter Sunday
    
    Morning Mass - Mass celebrated in the morning
    
    Mass of the Passion - Mass focusing on Christ's passion, beginning with the procession
    with palms
    
    Celebration of the Passion - special celebration of Christ's passion
    
    Day Mass - regular Mass celebrated during the day
    
    Chrism Mass - Mass where holy oils are blessed, typically on Holy Thursday morning
    
    Evening Mass of the Lord's Supper - Mass celebrated on Holy Thursday evening
    """
    CELEBRATION_OF_THE_PASSION = "celebration_of_the_passion"
    CHRISM_MASS = "chrism_mass"
    DAY_MASS = "day_mass"
    EASTER_VIGIL = "easter_vigil"
    EVENING_MASS_OF_THE_LORDS_SUPPER = "evening_mass_of_the_lords_supper"
    MASS_AT_DAWN = "mass_at_dawn"
    MASS_OF_THE_PASSION = "mass_of_the_passion"
    MORNING_MASS = "morning_mass"
    NIGHT_MASS = "night_mass"
    PREVIOUS_EVENING_MASS = "previous_evening_mass"


class MassInfo(BaseModel):
    """Information about a mass celebration for a liturgical day.
    Contains the type of mass and its localized name.
    """
    name: str
    """The localized name of the mass type (translation key in snake_case)"""

    type: MassTime
    """The type of mass (e.g., DayMass, EasterVigil, etc.)
    Serialized as SCREAMING_SNAKE_CASE (e.g., "DAY_MASS")
    """

    @staticmethod
    def from_dict(obj: Any) -> 'MassInfo':
        assert isinstance(obj, dict)
        name = from_str(obj.get("name"))
        type = MassTime(obj.get("type"))
        return MassInfo(name, type)

    def to_dict(self) -> dict:
        result: dict = {}
        result["name"] = from_str(self.name)
        result["type"] = to_enum(MassTime, self.type)
        return result


class ParentOverride(BaseModel):
    """Represents the differences between a liturgical day definition and its parent definition.
    This is a lightweight structure that only contains fields that can be overridden.
    """
    from_calendar_id: str
    """The ID of the calendar from which this override originates"""

    allow_similar_rank_items: Optional[bool] = None
    """The allow_similar_rank_items flag if it was changed"""

    colors: Optional[List[ColorInfo]] = None
    """The colors if they were changed"""

    commons_def: Optional[List[CommonDefinition]] = None
    """The commons definition if it was changed"""

    date_def: Optional[DateDefClass] = None
    """The date definition if it was changed"""

    date_exceptions: Optional[List[DateDefException]] = None
    """The date exceptions if they were changed"""

    is_holy_day_of_obligation: Optional[bool] = None
    """The is_holy_day_of_obligation flag if it was changed"""

    is_optional: Optional[bool] = None
    """The is_optional flag if it was changed"""

    precedence: Optional[Precedence] = None
    """The precedence if it was changed"""

    rank: Optional[Rank] = None
    """The rank if it was changed"""

    titles: Optional[Union[List[Title], CompoundTitle]] = None
    """The titles if they were changed"""

    @staticmethod
    def from_dict(obj: Any) -> 'ParentOverride':
        assert isinstance(obj, dict)
        from_calendar_id = from_str(obj.get("from_calendar_id"))
        allow_similar_rank_items = from_union([from_none, from_bool], obj.get("allow_similar_rank_items"))
        colors = from_union([from_none, lambda x: from_list(ColorInfo.from_dict, x)], obj.get("colors"))
        commons_def = from_union([from_none, lambda x: from_list(CommonDefinition, x)], obj.get("commons_def"))
        date_def = from_union([DateDefClass.from_dict, from_none], obj.get("date_def"))
        date_exceptions = from_union([from_none, lambda x: from_list(DateDefException.from_dict, x)], obj.get("date_exceptions"))
        is_holy_day_of_obligation = from_union([from_none, from_bool], obj.get("is_holy_day_of_obligation"))
        is_optional = from_union([from_none, from_bool], obj.get("is_optional"))
        precedence = from_union([from_none, Precedence], obj.get("precedence"))
        rank = from_union([from_none, Rank], obj.get("rank"))
        titles = from_union([lambda x: from_list(Title, x), CompoundTitle.from_dict, from_none], obj.get("titles"))
        return ParentOverride(from_calendar_id, allow_similar_rank_items, colors, commons_def, date_def, date_exceptions, is_holy_day_of_obligation, is_optional, precedence, rank, titles)

    def to_dict(self) -> dict:
        result: dict = {}
        result["from_calendar_id"] = from_str(self.from_calendar_id)
        if self.allow_similar_rank_items is not None:
            result["allow_similar_rank_items"] = from_union([from_none, from_bool], self.allow_similar_rank_items)
        if self.colors is not None:
            result["colors"] = from_union([from_none, lambda x: from_list(lambda x: to_class(ColorInfo, x), x)], self.colors)
        if self.commons_def is not None:
            result["commons_def"] = from_union([from_none, lambda x: from_list(lambda x: to_enum(CommonDefinition, x), x)], self.commons_def)
        if self.date_def is not None:
            result["date_def"] = from_union([lambda x: to_class(DateDefClass, x), from_none], self.date_def)
        if self.date_exceptions is not None:
            result["date_exceptions"] = from_union([from_none, lambda x: from_list(lambda x: to_class(DateDefException, x), x)], self.date_exceptions)
        if self.is_holy_day_of_obligation is not None:
            result["is_holy_day_of_obligation"] = from_union([from_none, from_bool], self.is_holy_day_of_obligation)
        if self.is_optional is not None:
            result["is_optional"] = from_union([from_none, from_bool], self.is_optional)
        if self.precedence is not None:
            result["precedence"] = from_union([from_none, lambda x: to_enum(Precedence, x)], self.precedence)
        if self.rank is not None:
            result["rank"] = from_union([from_none, lambda x: to_enum(Rank, x)], self.rank)
        if self.titles is not None:
            result["titles"] = from_union([lambda x: from_list(lambda x: to_enum(Title, x), x), lambda x: to_class(CompoundTitle, x), from_none], self.titles)
        return result


class Period(Enum):
    """The period key
    
    Specific periods within liturgical seasons.
    Defines sub-periods that have special liturgical characteristics or rules.
    
    The eight days following Christmas (December 25 - January 1)
    
    Days before Epiphany (January 2 to the day before Epiphany)
    
    Days from Epiphany to the Presentation (January 6 to the day before the Presentation of
    the Lord)
    
    Period from Christmas to the Presentation of the Lord
    
    Period from the Presentation to Holy Thursday
    
    Holy Week (Palm Sunday to Holy Saturday)
    
    Paschal Triduum (start from the Thursday of the Lord's Supper to the Easter Sunday
    Vespers)
    
    The eight days following Easter Sunday
    
    Early Ordinary Time (after the Presentation of the Lord to the day before Ash Wednesday)
    
    Late Ordinary Time (after Pentecost to the day before the First Sunday of Advent)
    """
    CHRISTMAS_OCTAVE = "CHRISTMAS_OCTAVE"
    CHRISTMAS_TO_PRESENTATION_OF_THE_LORD = "CHRISTMAS_TO_PRESENTATION_OF_THE_LORD"
    DAYS_BEFORE_EPIPHANY = "DAYS_BEFORE_EPIPHANY"
    DAYS_FROM_EPIPHANY = "DAYS_FROM_EPIPHANY"
    EARLY_ORDINARY_TIME = "EARLY_ORDINARY_TIME"
    EASTER_OCTAVE = "EASTER_OCTAVE"
    HOLY_WEEK = "HOLY_WEEK"
    LATE_ORDINARY_TIME = "LATE_ORDINARY_TIME"
    PASCHAL_TRIDUUM = "PASCHAL_TRIDUUM"
    PRESENTATION_OF_THE_LORD_TO_HOLY_THURSDAY = "PRESENTATION_OF_THE_LORD_TO_HOLY_THURSDAY"


class PeriodInfo(BaseModel):
    """Liturgical period information with localized name."""

    key: Period
    """The period key"""

    name: str
    """The localized name of the period"""

    @staticmethod
    def from_dict(obj: Any) -> 'PeriodInfo':
        assert isinstance(obj, dict)
        key = Period(obj.get("key"))
        name = from_str(obj.get("name"))
        return PeriodInfo(key, name)

    def to_dict(self) -> dict:
        result: dict = {}
        result["key"] = to_enum(Period, self.key)
        result["name"] = from_str(self.name)
        return result


class PsalterWeekCycle(Enum):
    """The psalter week cycle to which this liturgical day belongs.
    
    [GILH §133] The four-week cycle of the psalter is coordinated with the liturgical year in
    such a way that
    on the First Sunday of Advent, the First Sunday in Ordinary Time, the First Sunday of
    Lent,
    and Easter Sunday the cycle is always begun again with Week 1 (others being omitted when
    necessary).
    
    The psalter week cycle (Week 1-4)
    
    Week 1
    
    Week 2
    
    Week 3
    
    Week 4
    """
    WEEK_1 = "WEEK_1"
    WEEK_2 = "WEEK_2"
    WEEK_3 = "WEEK_3"
    WEEK_4 = "WEEK_4"


class Season(Enum):
    """Advent
    
    Christmas Time
    
    Lent
    
    Paschal Triduum
    
    Easter Time
    
    Ordinary Time
    """
    ADVENT = "ADVENT"
    CHRISTMAS_TIME = "CHRISTMAS_TIME"
    EASTER_TIME = "EASTER_TIME"
    LENT = "LENT"
    ORDINARY_TIME = "ORDINARY_TIME"
    PASCHAL_TRIDUUM = "PASCHAL_TRIDUUM"


class SundayCycle(Enum):
    """The Sunday cycle to which this liturgical day belongs.
    
    A three-year cycle for Sunday Mass readings (and some solemnities), designated by A, B,
    or C.
    Each cycle begins on the First Sunday of Advent of the previous civil year and ends on
    Saturday
    after the Christ the King Solemnity. The cycles follow each other in alphabetical order.
    C year is always divisible by 3, A has remainder of 1, and B remainder of 2.
    
    The Sunday cycle (Year A, B, or C)
    
    Year A
    
    Year B
    
    Year C
    """
    YEAR_A = "YEAR_A"
    YEAR_B = "YEAR_B"
    YEAR_C = "YEAR_C"


class WeekdayCycle(Enum):
    """The weekday cycle to which this liturgical day belongs.
    
    A two-year cycle for the weekday Mass readings (also called Cycle I and Cycle II).
    Odd-numbered years are the Cycle I (year 1); even-numbered ones are the Cycle II (year
    2).
    
    The weekday cycle (Year 1 or 2)
    
    Year 1 (Cycle I)
    
    Year 2 (Cycle II)
    """
    YEAR_1 = "YEAR_1"
    YEAR_2 = "YEAR_2"


class LiturgicalDay(BaseModel):
    """A single day in the liturgical calendar with computed values and inheritance information."""

    allow_similar_rank_items: bool
    """Allows similar items with the same rank and same or lower precedence
    to coexist without this liturgical day overwriting them.
    """
    colors: List[ColorInfo]
    """The liturgical colors for this liturgical day."""

    commons: List[CommonInfo]
    """The common prayers, readings, and chants used for celebrating saints or
    feasts that belong to a specific category, such as martyrs, virgins, pastors, or the
    Blessed
    Virgin Mary.
    """
    date: str
    """The computed date of the liturgical day."""

    date_def: DateDef
    """The date definition for this liturgical day."""

    date_exceptions: List[DateDefException]
    """The date definition exceptions for this liturgical day."""

    day_of_week: int
    """The day of the week for this liturgical day.
    Returns a number from 0 (Sunday) to 6 (Saturday).
    """
    end_of_liturgical_year: str
    """The last day of the current liturgical year for this liturgical day,
    i.e. the last Saturday of Ordinary Time, in the 34th week.
    """
    entities: List[Entity]
    """The entities (Saints, Blessed, or Places) linked to this liturgical day."""

    from_calendar_id: str
    """The ID of the calendar where this liturgical day is defined.
    Indicates the source calendar in the inheritance chain.
    """
    fullname: str
    """The full name of the liturgical day"""

    id: str
    """The unique identifier of the liturgical day"""

    is_holy_day_of_obligation: bool
    """Holy days of obligation are days on which the faithful are expected to attend Mass,
    and engage in rest from work and recreation.
    """
    is_optional: bool
    """Indicates if this liturgical day is optional within a specific liturgical calendar."""

    masses: List[MassInfo]
    """The masses celebrated on this liturgical day.
    Most days have a single DayMass, but some have multiple masses
    (e.g., Christmas: PreviousEveningMass, NightMass, MassAtDawn, DayMass).
    Aliturgical days like Holy Saturday have an empty list.
    """
    nth_day_of_week_in_month: int
    """The nth occurrence of this day of the week within the current month.
    For example, the 3rd Sunday of the month would have nth_day_of_week_in_month = 3.
    """
    parent_overrides: List[ParentOverride]
    """Contains the differences between this liturgical day and its parent definitions.
    Each element in the array represents the diff from a parent calendar definition.
    The array is ordered from most general (e.g., general_roman) to most specific.
    """
    periods: List[PeriodInfo]
    """The liturgical periods to which this liturgical day belongs."""

    precedence: Precedence
    """The liturgical precedence for this liturgical day."""

    psalter_week: PsalterWeekCycle
    """The psalter week cycle to which this liturgical day belongs."""

    psalter_week_name: str
    """The localized name of the psalter week cycle to which this liturgical day belongs."""

    rank: Rank
    """The liturgical rank for this liturgical day."""

    rank_name: str
    """The localized liturgical rank for this liturgical day."""

    start_of_liturgical_year: str
    """The first day of the current liturgical year for this liturgical day,
    i.e. the first Sunday of Advent.
    """
    sunday_cycle: SundayCycle
    """The Sunday cycle to which this liturgical day belongs."""

    sunday_cycle_name: str
    """The localized name of the Sunday cycle to which this liturgical day belongs."""

    titles: Union[List[Title], CompoundTitle]
    """The titles for this liturgical day."""

    weekday_cycle: WeekdayCycle
    """The weekday cycle to which this liturgical day belongs."""

    weekday_cycle_name: str
    """The localized name of the weekday cycle to which this liturgical day belongs."""

    day_of_season: Optional[int] = None
    """The day number within the current liturgical season."""

    end_of_season: Optional[str] = None
    """The last day of the current liturgical season for this liturgical day."""

    season: Optional[Season] = None
    """The liturgical seasons to which this liturgical day belongs."""

    season_name: Optional[str] = None
    """The liturgical season name."""

    start_of_season: Optional[str] = None
    """The first day of the current liturgical season for this liturgical day."""

    week_of_season: Optional[int] = None
    """The week number of the current liturgical season.
    Starts from `1`, except in the seasons of lent,
    the week of Ash Wednesday to the next Saturday is counted as `0`.
    """

    @staticmethod
    def from_dict(obj: Any) -> 'LiturgicalDay':
        assert isinstance(obj, dict)
        allow_similar_rank_items = from_bool(obj.get("allow_similar_rank_items"))
        colors = from_list(ColorInfo.from_dict, obj.get("colors"))
        commons = from_list(CommonInfo.from_dict, obj.get("commons"))
        date = from_str(obj.get("date"))
        date_def = DateDef.from_dict(obj.get("date_def"))
        date_exceptions = from_list(DateDefException.from_dict, obj.get("date_exceptions"))
        day_of_week = from_int(obj.get("day_of_week"))
        end_of_liturgical_year = from_str(obj.get("end_of_liturgical_year"))
        entities = from_list(Entity.from_dict, obj.get("entities"))
        from_calendar_id = from_str(obj.get("from_calendar_id"))
        fullname = from_str(obj.get("fullname"))
        id = from_str(obj.get("id"))
        is_holy_day_of_obligation = from_bool(obj.get("is_holy_day_of_obligation"))
        is_optional = from_bool(obj.get("is_optional"))
        masses = from_list(MassInfo.from_dict, obj.get("masses"))
        nth_day_of_week_in_month = from_int(obj.get("nth_day_of_week_in_month"))
        parent_overrides = from_list(ParentOverride.from_dict, obj.get("parent_overrides"))
        periods = from_list(PeriodInfo.from_dict, obj.get("periods"))
        precedence = Precedence(obj.get("precedence"))
        psalter_week = PsalterWeekCycle(obj.get("psalter_week"))
        psalter_week_name = from_str(obj.get("psalter_week_name"))
        rank = Rank(obj.get("rank"))
        rank_name = from_str(obj.get("rank_name"))
        start_of_liturgical_year = from_str(obj.get("start_of_liturgical_year"))
        sunday_cycle = SundayCycle(obj.get("sunday_cycle"))
        sunday_cycle_name = from_str(obj.get("sunday_cycle_name"))
        titles = from_union([lambda x: from_list(Title, x), CompoundTitle.from_dict], obj.get("titles"))
        weekday_cycle = WeekdayCycle(obj.get("weekday_cycle"))
        weekday_cycle_name = from_str(obj.get("weekday_cycle_name"))
        day_of_season = from_union([from_none, from_int], obj.get("day_of_season"))
        end_of_season = from_union([from_none, from_str], obj.get("end_of_season"))
        season = from_union([from_none, Season], obj.get("season"))
        season_name = from_union([from_none, from_str], obj.get("season_name"))
        start_of_season = from_union([from_none, from_str], obj.get("start_of_season"))
        week_of_season = from_union([from_none, from_int], obj.get("week_of_season"))
        return LiturgicalDay(allow_similar_rank_items, colors, commons, date, date_def, date_exceptions, day_of_week, end_of_liturgical_year, entities, from_calendar_id, fullname, id, is_holy_day_of_obligation, is_optional, masses, nth_day_of_week_in_month, parent_overrides, periods, precedence, psalter_week, psalter_week_name, rank, rank_name, start_of_liturgical_year, sunday_cycle, sunday_cycle_name, titles, weekday_cycle, weekday_cycle_name, day_of_season, end_of_season, season, season_name, start_of_season, week_of_season)

    def to_dict(self) -> dict:
        result: dict = {}
        result["allow_similar_rank_items"] = from_bool(self.allow_similar_rank_items)
        result["colors"] = from_list(lambda x: to_class(ColorInfo, x), self.colors)
        result["commons"] = from_list(lambda x: to_class(CommonInfo, x), self.commons)
        result["date"] = from_str(self.date)
        result["date_def"] = to_class(DateDef, self.date_def)
        result["date_exceptions"] = from_list(lambda x: to_class(DateDefException, x), self.date_exceptions)
        result["day_of_week"] = from_int(self.day_of_week)
        result["end_of_liturgical_year"] = from_str(self.end_of_liturgical_year)
        result["entities"] = from_list(lambda x: to_class(Entity, x), self.entities)
        result["from_calendar_id"] = from_str(self.from_calendar_id)
        result["fullname"] = from_str(self.fullname)
        result["id"] = from_str(self.id)
        result["is_holy_day_of_obligation"] = from_bool(self.is_holy_day_of_obligation)
        result["is_optional"] = from_bool(self.is_optional)
        result["masses"] = from_list(lambda x: to_class(MassInfo, x), self.masses)
        result["nth_day_of_week_in_month"] = from_int(self.nth_day_of_week_in_month)
        result["parent_overrides"] = from_list(lambda x: to_class(ParentOverride, x), self.parent_overrides)
        result["periods"] = from_list(lambda x: to_class(PeriodInfo, x), self.periods)
        result["precedence"] = to_enum(Precedence, self.precedence)
        result["psalter_week"] = to_enum(PsalterWeekCycle, self.psalter_week)
        result["psalter_week_name"] = from_str(self.psalter_week_name)
        result["rank"] = to_enum(Rank, self.rank)
        result["rank_name"] = from_str(self.rank_name)
        result["start_of_liturgical_year"] = from_str(self.start_of_liturgical_year)
        result["sunday_cycle"] = to_enum(SundayCycle, self.sunday_cycle)
        result["sunday_cycle_name"] = from_str(self.sunday_cycle_name)
        result["titles"] = from_union([lambda x: from_list(lambda x: to_enum(Title, x), x), lambda x: to_class(CompoundTitle, x)], self.titles)
        result["weekday_cycle"] = to_enum(WeekdayCycle, self.weekday_cycle)
        result["weekday_cycle_name"] = from_str(self.weekday_cycle_name)
        if self.day_of_season is not None:
            result["day_of_season"] = from_union([from_none, from_int], self.day_of_season)
        if self.end_of_season is not None:
            result["end_of_season"] = from_union([from_none, from_str], self.end_of_season)
        if self.season is not None:
            result["season"] = from_union([from_none, lambda x: to_enum(Season, x)], self.season)
        if self.season_name is not None:
            result["season_name"] = from_union([from_none, from_str], self.season_name)
        if self.start_of_season is not None:
            result["start_of_season"] = from_union([from_none, from_str], self.start_of_season)
        if self.week_of_season is not None:
            result["week_of_season"] = from_union([from_none, from_int], self.week_of_season)
        return result


class MassContext(BaseModel):
    """A flat structure representing a single mass with its full liturgical context.
    
    This is the main type for the mass-centric calendar view. It contains:
    - Mass identification (type, name, civil/liturgical dates)
    - Day-level context (season, cycles, periods)
    - Primary celebration data (flattened from LiturgicalDay)
    - Optional alternative celebrations
    
    For evening masses (Easter Vigil, Previous Evening Mass), the `civil_date`
    is shifted to the previous day while `liturgical_date` remains the original
    liturgical celebration date.
    """
    civil_date: str
    """The civil calendar date when this mass is celebrated (YYYY-MM-DD).
    For evening masses (EasterVigil, PreviousEveningMass), this is the day
    BEFORE the liturgical date.
    """
    colors: List[ColorInfo]
    """The liturgical colors"""

    commons: List[CommonInfo]
    """The common prayers/readings used"""

    day_of_week: int
    """The day of the week (0=Sunday to 6=Saturday)"""

    end_of_liturgical_year: str
    """The last day of the liturgical year"""

    entities: List[Entity]
    """The entities (Saints, Blessed, or Places) linked to this day"""

    from_calendar_id: str
    """The ID of the calendar where this liturgical day is defined"""

    fullname: str
    """The full name of the liturgical day"""

    id: str
    """The unique identifier of the liturgical day"""

    is_holy_day_of_obligation: bool
    """Whether this is a holy day of obligation"""

    is_optional: bool
    """Whether this liturgical day is optional"""

    liturgical_date: str
    """The liturgical date this mass belongs to (YYYY-MM-DD).
    This is the "theological" date of the celebration.
    """
    mass_time: MassTime
    """The type of mass (e.g., DayMass, EasterVigil, etc.)
    Serialized as SCREAMING_SNAKE_CASE (e.g., "DAY_MASS")
    """
    mass_time_name: str
    """The localized name of the mass time (translation key in snake_case)"""

    optional_celebrations: List[CelebrationSummary]
    """Optional alternative celebrations (e.g., optional memorials)
    that can be celebrated instead of the primary celebration.
    """
    periods: List[PeriodInfo]
    """The liturgical periods this day belongs to"""

    precedence: Precedence
    """The liturgical precedence"""

    psalter_week: PsalterWeekCycle
    """The psalter week cycle (Week 1-4)"""

    psalter_week_name: str
    """The localized psalter week name"""

    rank: Rank
    """The liturgical rank"""

    rank_name: str
    """The localized liturgical rank name"""

    start_of_liturgical_year: str
    """The first day of the liturgical year (first Sunday of Advent)"""

    sunday_cycle: SundayCycle
    """The Sunday cycle (Year A, B, or C)"""

    sunday_cycle_name: str
    """The localized Sunday cycle name"""

    titles: Union[List[Title], CompoundTitle]
    """The titles for this liturgical day"""

    weekday_cycle: WeekdayCycle
    """The weekday cycle (Year 1 or 2)"""

    weekday_cycle_name: str
    """The localized weekday cycle name"""

    day_of_season: Optional[int] = None
    """The day number within the liturgical season"""

    end_of_season: Optional[str] = None
    """The last day of the current liturgical season"""

    season: Optional[Season] = None
    """The liturgical season"""

    season_name: Optional[str] = None
    """The localized season name"""

    start_of_season: Optional[str] = None
    """The first day of the current liturgical season"""

    week_of_season: Optional[int] = None
    """The week number within the liturgical season"""

    @staticmethod
    def from_dict(obj: Any) -> 'MassContext':
        assert isinstance(obj, dict)
        civil_date = from_str(obj.get("civil_date"))
        colors = from_list(ColorInfo.from_dict, obj.get("colors"))
        commons = from_list(CommonInfo.from_dict, obj.get("commons"))
        day_of_week = from_int(obj.get("day_of_week"))
        end_of_liturgical_year = from_str(obj.get("end_of_liturgical_year"))
        entities = from_list(Entity.from_dict, obj.get("entities"))
        from_calendar_id = from_str(obj.get("from_calendar_id"))
        fullname = from_str(obj.get("fullname"))
        id = from_str(obj.get("id"))
        is_holy_day_of_obligation = from_bool(obj.get("is_holy_day_of_obligation"))
        is_optional = from_bool(obj.get("is_optional"))
        liturgical_date = from_str(obj.get("liturgical_date"))
        mass_time = MassTime(obj.get("mass_time"))
        mass_time_name = from_str(obj.get("mass_time_name"))
        optional_celebrations = from_list(CelebrationSummary.from_dict, obj.get("optional_celebrations"))
        periods = from_list(PeriodInfo.from_dict, obj.get("periods"))
        precedence = Precedence(obj.get("precedence"))
        psalter_week = PsalterWeekCycle(obj.get("psalter_week"))
        psalter_week_name = from_str(obj.get("psalter_week_name"))
        rank = Rank(obj.get("rank"))
        rank_name = from_str(obj.get("rank_name"))
        start_of_liturgical_year = from_str(obj.get("start_of_liturgical_year"))
        sunday_cycle = SundayCycle(obj.get("sunday_cycle"))
        sunday_cycle_name = from_str(obj.get("sunday_cycle_name"))
        titles = from_union([lambda x: from_list(Title, x), CompoundTitle.from_dict], obj.get("titles"))
        weekday_cycle = WeekdayCycle(obj.get("weekday_cycle"))
        weekday_cycle_name = from_str(obj.get("weekday_cycle_name"))
        day_of_season = from_union([from_none, from_int], obj.get("day_of_season"))
        end_of_season = from_union([from_none, from_str], obj.get("end_of_season"))
        season = from_union([from_none, Season], obj.get("season"))
        season_name = from_union([from_none, from_str], obj.get("season_name"))
        start_of_season = from_union([from_none, from_str], obj.get("start_of_season"))
        week_of_season = from_union([from_none, from_int], obj.get("week_of_season"))
        return MassContext(civil_date, colors, commons, day_of_week, end_of_liturgical_year, entities, from_calendar_id, fullname, id, is_holy_day_of_obligation, is_optional, liturgical_date, mass_time, mass_time_name, optional_celebrations, periods, precedence, psalter_week, psalter_week_name, rank, rank_name, start_of_liturgical_year, sunday_cycle, sunday_cycle_name, titles, weekday_cycle, weekday_cycle_name, day_of_season, end_of_season, season, season_name, start_of_season, week_of_season)

    def to_dict(self) -> dict:
        result: dict = {}
        result["civil_date"] = from_str(self.civil_date)
        result["colors"] = from_list(lambda x: to_class(ColorInfo, x), self.colors)
        result["commons"] = from_list(lambda x: to_class(CommonInfo, x), self.commons)
        result["day_of_week"] = from_int(self.day_of_week)
        result["end_of_liturgical_year"] = from_str(self.end_of_liturgical_year)
        result["entities"] = from_list(lambda x: to_class(Entity, x), self.entities)
        result["from_calendar_id"] = from_str(self.from_calendar_id)
        result["fullname"] = from_str(self.fullname)
        result["id"] = from_str(self.id)
        result["is_holy_day_of_obligation"] = from_bool(self.is_holy_day_of_obligation)
        result["is_optional"] = from_bool(self.is_optional)
        result["liturgical_date"] = from_str(self.liturgical_date)
        result["mass_time"] = to_enum(MassTime, self.mass_time)
        result["mass_time_name"] = from_str(self.mass_time_name)
        result["optional_celebrations"] = from_list(lambda x: to_class(CelebrationSummary, x), self.optional_celebrations)
        result["periods"] = from_list(lambda x: to_class(PeriodInfo, x), self.periods)
        result["precedence"] = to_enum(Precedence, self.precedence)
        result["psalter_week"] = to_enum(PsalterWeekCycle, self.psalter_week)
        result["psalter_week_name"] = from_str(self.psalter_week_name)
        result["rank"] = to_enum(Rank, self.rank)
        result["rank_name"] = from_str(self.rank_name)
        result["start_of_liturgical_year"] = from_str(self.start_of_liturgical_year)
        result["sunday_cycle"] = to_enum(SundayCycle, self.sunday_cycle)
        result["sunday_cycle_name"] = from_str(self.sunday_cycle_name)
        result["titles"] = from_union([lambda x: from_list(lambda x: to_enum(Title, x), x), lambda x: to_class(CompoundTitle, x)], self.titles)
        result["weekday_cycle"] = to_enum(WeekdayCycle, self.weekday_cycle)
        result["weekday_cycle_name"] = from_str(self.weekday_cycle_name)
        if self.day_of_season is not None:
            result["day_of_season"] = from_union([from_none, from_int], self.day_of_season)
        if self.end_of_season is not None:
            result["end_of_season"] = from_union([from_none, from_str], self.end_of_season)
        if self.season is not None:
            result["season"] = from_union([from_none, lambda x: to_enum(Season, x)], self.season)
        if self.season_name is not None:
            result["season_name"] = from_union([from_none, from_str], self.season_name)
        if self.start_of_season is not None:
            result["start_of_season"] = from_union([from_none, from_str], self.start_of_season)
        if self.week_of_season is not None:
            result["week_of_season"] = from_union([from_none, from_int], self.week_of_season)
        return result


class MassPart(Enum):
    """Parts that make up the Mass celebration.
    Each part represents a specific element of the liturgical celebration.
    
    Messianic entry reading (during the procession with palms, before the Mass of the
    Passion)
    
    Entrance Antiphon - opening chant of the Mass
    
    Collect - opening prayer of the Mass
    
    Reading 1 - first reading (usually from the Old Testament)
    
    Psalm - responsorial psalm
    
    Canticle - biblical canticle
    
    Reading 2 - second reading (usually from the New Testament)
    
    Psalm (Easter Vigil)
    
    Reading 3 - third reading (Easter Vigil)
    
    Canticle 3 (Easter Vigil)
    
    Reading 4 - fourth reading (Easter Vigil)
    
    Psalm 4 (Easter Vigil)
    
    Reading 5 - fifth reading (Easter Vigil)
    
    Canticle 5 (Easter Vigil)
    
    Reading 6 - sixth reading (Easter Vigil)
    
    Psalm 6 (Easter Vigil)
    
    Reading 7 - seventh reading (Easter Vigil)
    
    Psalm 7 (Easter Vigil)
    
    Epistle - reading from the epistles (Easter Vigil)
    
    Sequence - special chant on certain feasts
    
    Alleluia - acclamation before the Gospel
    
    Gospel - reading from the Gospels
    
    Prayer over the Offerings - prayer during the offertory
    
    Preface - introduction to the Eucharistic Prayer
    
    Communion Antiphon - chant during communion
    
    Prayer after Communion - concluding prayer
    
    Solemn Blessing - special blessing on certain occasions
    
    Prayer over the People - blessing over the congregation
    """
    ALLELUIA = "alleluia"
    CANTICLE = "canticle"
    COLLECT = "collect"
    COMMUNION_ANTIPHON = "communion_antiphon"
    EASTER_VIGIL_CANTICLE_3 = "easter_vigil_canticle_3"
    EASTER_VIGIL_CANTICLE_5 = "easter_vigil_canticle_5"
    EASTER_VIGIL_EPISTLE = "easter_vigil_epistle"
    EASTER_VIGIL_PSALM_2 = "easter_vigil_psalm_2"
    EASTER_VIGIL_PSALM_4 = "easter_vigil_psalm_4"
    EASTER_VIGIL_PSALM_6 = "easter_vigil_psalm_6"
    EASTER_VIGIL_PSALM_7 = "easter_vigil_psalm_7"
    EASTER_VIGIL_READING_3 = "easter_vigil_reading_3"
    EASTER_VIGIL_READING_4 = "easter_vigil_reading_4"
    EASTER_VIGIL_READING_5 = "easter_vigil_reading_5"
    EASTER_VIGIL_READING_6 = "easter_vigil_reading_6"
    EASTER_VIGIL_READING_7 = "easter_vigil_reading_7"
    ENTRANCE_ANTIPHON = "entrance_antiphon"
    GOSPEL = "gospel"
    MESSIANIC_ENTRY = "messianic_entry"
    PRAYER_AFTER_COMMUNION = "prayer_after_communion"
    PRAYER_OVER_THE_OFFERINGS = "prayer_over_the_offerings"
    PRAYER_OVER_THE_PEOPLE = "prayer_over_the_people"
    PREFACE = "preface"
    PSALM = "psalm"
    READING_1 = "reading_1"
    READING_2 = "reading_2"
    SEQUENCE = "sequence"
    SOLEMN_BLESSING = "solemn_blessing"


class LocaleColors(BaseModel):
    """Liturgical color names in the locale language.
    Provides localized names for each liturgical color.
    """
    black: Optional[str] = None
    """Black color name in the locale language"""

    gold: Optional[str] = None
    """Gold color name in the locale language"""

    green: Optional[str] = None
    """Green color name in the locale language"""

    purple: Optional[str] = None
    """Purple color name in the locale language"""

    red: Optional[str] = None
    """Red color name in the locale language"""

    rose: Optional[str] = None
    """Rose color name in the locale language"""

    white: Optional[str] = None
    """White color name in the locale language"""

    @staticmethod
    def from_dict(obj: Any) -> 'LocaleColors':
        assert isinstance(obj, dict)
        black = from_union([from_none, from_str], obj.get("black"))
        gold = from_union([from_none, from_str], obj.get("gold"))
        green = from_union([from_none, from_str], obj.get("green"))
        purple = from_union([from_none, from_str], obj.get("purple"))
        red = from_union([from_none, from_str], obj.get("red"))
        rose = from_union([from_none, from_str], obj.get("rose"))
        white = from_union([from_none, from_str], obj.get("white"))
        return LocaleColors(black, gold, green, purple, red, rose, white)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.black is not None:
            result["black"] = from_union([from_none, from_str], self.black)
        if self.gold is not None:
            result["gold"] = from_union([from_none, from_str], self.gold)
        if self.green is not None:
            result["green"] = from_union([from_none, from_str], self.green)
        if self.purple is not None:
            result["purple"] = from_union([from_none, from_str], self.purple)
        if self.red is not None:
            result["red"] = from_union([from_none, from_str], self.red)
        if self.rose is not None:
            result["rose"] = from_union([from_none, from_str], self.rose)
        if self.white is not None:
            result["white"] = from_union([from_none, from_str], self.white)
        return result


class CyclesMetadata(BaseModel):
    """Liturgical cycle names in the locale language."""

    proper_of_saints: Optional[str] = None
    """Proper of Saints cycle name"""

    proper_of_time: Optional[str] = None
    """Proper of Time cycle name"""

    psalter_week_1: Optional[str] = None
    """Psalter Week 1 cycle name"""

    psalter_week_2: Optional[str] = None
    """Psalter Week 2 cycle name"""

    psalter_week_3: Optional[str] = None
    """Psalter Week 3 cycle name"""

    psalter_week_4: Optional[str] = None
    """Psalter Week 4 cycle name"""

    sunday_year_a: Optional[str] = None
    """Sunday Year A cycle name"""

    sunday_year_b: Optional[str] = None
    """Sunday Year B cycle name"""

    sunday_year_c: Optional[str] = None
    """Sunday Year C cycle name"""

    weekday_year_1: Optional[str] = None
    """Weekday Year 1 cycle name"""

    weekday_year_2: Optional[str] = None
    """Weekday Year 2 cycle name"""

    @staticmethod
    def from_dict(obj: Any) -> 'CyclesMetadata':
        assert isinstance(obj, dict)
        proper_of_saints = from_union([from_none, from_str], obj.get("proper_of_saints"))
        proper_of_time = from_union([from_none, from_str], obj.get("proper_of_time"))
        psalter_week_1 = from_union([from_none, from_str], obj.get("psalter_week_1"))
        psalter_week_2 = from_union([from_none, from_str], obj.get("psalter_week_2"))
        psalter_week_3 = from_union([from_none, from_str], obj.get("psalter_week_3"))
        psalter_week_4 = from_union([from_none, from_str], obj.get("psalter_week_4"))
        sunday_year_a = from_union([from_none, from_str], obj.get("sunday_year_a"))
        sunday_year_b = from_union([from_none, from_str], obj.get("sunday_year_b"))
        sunday_year_c = from_union([from_none, from_str], obj.get("sunday_year_c"))
        weekday_year_1 = from_union([from_none, from_str], obj.get("weekday_year_1"))
        weekday_year_2 = from_union([from_none, from_str], obj.get("weekday_year_2"))
        return CyclesMetadata(proper_of_saints, proper_of_time, psalter_week_1, psalter_week_2, psalter_week_3, psalter_week_4, sunday_year_a, sunday_year_b, sunday_year_c, weekday_year_1, weekday_year_2)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.proper_of_saints is not None:
            result["proper_of_saints"] = from_union([from_none, from_str], self.proper_of_saints)
        if self.proper_of_time is not None:
            result["proper_of_time"] = from_union([from_none, from_str], self.proper_of_time)
        if self.psalter_week_1 is not None:
            result["psalter_week_1"] = from_union([from_none, from_str], self.psalter_week_1)
        if self.psalter_week_2 is not None:
            result["psalter_week_2"] = from_union([from_none, from_str], self.psalter_week_2)
        if self.psalter_week_3 is not None:
            result["psalter_week_3"] = from_union([from_none, from_str], self.psalter_week_3)
        if self.psalter_week_4 is not None:
            result["psalter_week_4"] = from_union([from_none, from_str], self.psalter_week_4)
        if self.sunday_year_a is not None:
            result["sunday_year_a"] = from_union([from_none, from_str], self.sunday_year_a)
        if self.sunday_year_b is not None:
            result["sunday_year_b"] = from_union([from_none, from_str], self.sunday_year_b)
        if self.sunday_year_c is not None:
            result["sunday_year_c"] = from_union([from_none, from_str], self.sunday_year_c)
        if self.weekday_year_1 is not None:
            result["weekday_year_1"] = from_union([from_none, from_str], self.weekday_year_1)
        if self.weekday_year_2 is not None:
            result["weekday_year_2"] = from_union([from_none, from_str], self.weekday_year_2)
        return result


class OrdinalFormat(Enum):
    """Ordinals displayed as words
    
    Ordinals displayed as numbers with suffixes (default)
    """
    LETTERS = "letters"
    NUMERIC = "numeric"


class PeriodsMetadata(BaseModel):
    """Liturgical period names in the locale language."""

    christmas_octave: Optional[str] = None
    """Christmas Octave period name"""

    christmas_to_presentation_of_the_lord: Optional[str] = None
    """Christmas to Presentation of the Lord period name"""

    days_before_epiphany: Optional[str] = None
    """Days before Epiphany period name"""

    days_from_epiphany: Optional[str] = None
    """Days from Epiphany period name"""

    early_ordinary_time: Optional[str] = None
    """Early Ordinary Time period name"""

    easter_octave: Optional[str] = None
    """Easter Octave period name"""

    holy_week: Optional[str] = None
    """Holy Week period name"""

    late_ordinary_time: Optional[str] = None
    """Late Ordinary Time period name"""

    paschal_triduum: Optional[str] = None
    """Paschal Triduum period name"""

    presentation_of_the_lord_to_holy_thursday: Optional[str] = None
    """Presentation of the Lord to Holy Thursday period name"""

    @staticmethod
    def from_dict(obj: Any) -> 'PeriodsMetadata':
        assert isinstance(obj, dict)
        christmas_octave = from_union([from_none, from_str], obj.get("christmas_octave"))
        christmas_to_presentation_of_the_lord = from_union([from_none, from_str], obj.get("christmas_to_presentation_of_the_lord"))
        days_before_epiphany = from_union([from_none, from_str], obj.get("days_before_epiphany"))
        days_from_epiphany = from_union([from_none, from_str], obj.get("days_from_epiphany"))
        early_ordinary_time = from_union([from_none, from_str], obj.get("early_ordinary_time"))
        easter_octave = from_union([from_none, from_str], obj.get("easter_octave"))
        holy_week = from_union([from_none, from_str], obj.get("holy_week"))
        late_ordinary_time = from_union([from_none, from_str], obj.get("late_ordinary_time"))
        paschal_triduum = from_union([from_none, from_str], obj.get("paschal_triduum"))
        presentation_of_the_lord_to_holy_thursday = from_union([from_none, from_str], obj.get("presentation_of_the_lord_to_holy_thursday"))
        return PeriodsMetadata(christmas_octave, christmas_to_presentation_of_the_lord, days_before_epiphany, days_from_epiphany, early_ordinary_time, easter_octave, holy_week, late_ordinary_time, paschal_triduum, presentation_of_the_lord_to_holy_thursday)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.christmas_octave is not None:
            result["christmas_octave"] = from_union([from_none, from_str], self.christmas_octave)
        if self.christmas_to_presentation_of_the_lord is not None:
            result["christmas_to_presentation_of_the_lord"] = from_union([from_none, from_str], self.christmas_to_presentation_of_the_lord)
        if self.days_before_epiphany is not None:
            result["days_before_epiphany"] = from_union([from_none, from_str], self.days_before_epiphany)
        if self.days_from_epiphany is not None:
            result["days_from_epiphany"] = from_union([from_none, from_str], self.days_from_epiphany)
        if self.early_ordinary_time is not None:
            result["early_ordinary_time"] = from_union([from_none, from_str], self.early_ordinary_time)
        if self.easter_octave is not None:
            result["easter_octave"] = from_union([from_none, from_str], self.easter_octave)
        if self.holy_week is not None:
            result["holy_week"] = from_union([from_none, from_str], self.holy_week)
        if self.late_ordinary_time is not None:
            result["late_ordinary_time"] = from_union([from_none, from_str], self.late_ordinary_time)
        if self.paschal_triduum is not None:
            result["paschal_triduum"] = from_union([from_none, from_str], self.paschal_triduum)
        if self.presentation_of_the_lord_to_holy_thursday is not None:
            result["presentation_of_the_lord_to_holy_thursday"] = from_union([from_none, from_str], self.presentation_of_the_lord_to_holy_thursday)
        return result


class RanksMetadata(BaseModel):
    """Liturgical rank names in the locale language."""

    feast: Optional[str] = None
    """Feast rank name"""

    memorial: Optional[str] = None
    """Memorial rank name"""

    optional_memorial: Optional[str] = None
    """Optional memorial rank name"""

    solemnity: Optional[str] = None
    """Solemnity rank name"""

    sunday: Optional[str] = None
    """Sunday rank name"""

    weekday: Optional[str] = None
    """Weekday rank name"""

    @staticmethod
    def from_dict(obj: Any) -> 'RanksMetadata':
        assert isinstance(obj, dict)
        feast = from_union([from_none, from_str], obj.get("feast"))
        memorial = from_union([from_none, from_str], obj.get("memorial"))
        optional_memorial = from_union([from_none, from_str], obj.get("optional_memorial"))
        solemnity = from_union([from_none, from_str], obj.get("solemnity"))
        sunday = from_union([from_none, from_str], obj.get("sunday"))
        weekday = from_union([from_none, from_str], obj.get("weekday"))
        return RanksMetadata(feast, memorial, optional_memorial, solemnity, sunday, weekday)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.feast is not None:
            result["feast"] = from_union([from_none, from_str], self.feast)
        if self.memorial is not None:
            result["memorial"] = from_union([from_none, from_str], self.memorial)
        if self.optional_memorial is not None:
            result["optional_memorial"] = from_union([from_none, from_str], self.optional_memorial)
        if self.solemnity is not None:
            result["solemnity"] = from_union([from_none, from_str], self.solemnity)
        if self.sunday is not None:
            result["sunday"] = from_union([from_none, from_str], self.sunday)
        if self.weekday is not None:
            result["weekday"] = from_union([from_none, from_str], self.weekday)
        return result


class AdventSeason(BaseModel):
    """Advent season localized names and descriptions.
    Provides specific terminology for the Advent season in the locale language.
    """
    privileged_weekday: Optional[str] = None
    """Privileged weekday terminology during Advent"""

    season: Optional[str] = None
    """General season name for Advent"""

    sunday: Optional[str] = None
    """Sunday terminology during Advent"""

    weekday: Optional[str] = None
    """Weekday terminology during Advent"""

    @staticmethod
    def from_dict(obj: Any) -> 'AdventSeason':
        assert isinstance(obj, dict)
        privileged_weekday = from_union([from_none, from_str], obj.get("privileged_weekday"))
        season = from_union([from_none, from_str], obj.get("season"))
        sunday = from_union([from_none, from_str], obj.get("sunday"))
        weekday = from_union([from_none, from_str], obj.get("weekday"))
        return AdventSeason(privileged_weekday, season, sunday, weekday)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.privileged_weekday is not None:
            result["privileged_weekday"] = from_union([from_none, from_str], self.privileged_weekday)
        if self.season is not None:
            result["season"] = from_union([from_none, from_str], self.season)
        if self.sunday is not None:
            result["sunday"] = from_union([from_none, from_str], self.sunday)
        if self.weekday is not None:
            result["weekday"] = from_union([from_none, from_str], self.weekday)
        return result


class ChristmasTimeSeason(BaseModel):
    """Christmas Time season localized names and descriptions."""

    after_epiphany: Optional[str] = None
    """After Epiphany terminology"""

    before_epiphany: Optional[str] = None
    """Before Epiphany terminology"""

    day: Optional[str] = None
    """Day terminology during Christmas Time"""

    octave: Optional[str] = None
    """Octave terminology during Christmas Time"""

    season: Optional[str] = None
    """General season name for Christmas Time"""

    second_sunday_after_christmas: Optional[str] = None
    """Second Sunday after Christmas terminology"""

    @staticmethod
    def from_dict(obj: Any) -> 'ChristmasTimeSeason':
        assert isinstance(obj, dict)
        after_epiphany = from_union([from_none, from_str], obj.get("after_epiphany"))
        before_epiphany = from_union([from_none, from_str], obj.get("before_epiphany"))
        day = from_union([from_none, from_str], obj.get("day"))
        octave = from_union([from_none, from_str], obj.get("octave"))
        season = from_union([from_none, from_str], obj.get("season"))
        second_sunday_after_christmas = from_union([from_none, from_str], obj.get("second_sunday_after_christmas"))
        return ChristmasTimeSeason(after_epiphany, before_epiphany, day, octave, season, second_sunday_after_christmas)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.after_epiphany is not None:
            result["after_epiphany"] = from_union([from_none, from_str], self.after_epiphany)
        if self.before_epiphany is not None:
            result["before_epiphany"] = from_union([from_none, from_str], self.before_epiphany)
        if self.day is not None:
            result["day"] = from_union([from_none, from_str], self.day)
        if self.octave is not None:
            result["octave"] = from_union([from_none, from_str], self.octave)
        if self.season is not None:
            result["season"] = from_union([from_none, from_str], self.season)
        if self.second_sunday_after_christmas is not None:
            result["second_sunday_after_christmas"] = from_union([from_none, from_str], self.second_sunday_after_christmas)
        return result


class EasterTimeSeason(BaseModel):
    """Easter Time season localized names and descriptions."""

    octave: Optional[str] = None
    """Octave terminology during Easter Time"""

    season: Optional[str] = None
    """General season name for Easter Time"""

    sunday: Optional[str] = None
    """Sunday terminology during Easter Time"""

    weekday: Optional[str] = None
    """Weekday terminology during Easter Time"""

    @staticmethod
    def from_dict(obj: Any) -> 'EasterTimeSeason':
        assert isinstance(obj, dict)
        octave = from_union([from_none, from_str], obj.get("octave"))
        season = from_union([from_none, from_str], obj.get("season"))
        sunday = from_union([from_none, from_str], obj.get("sunday"))
        weekday = from_union([from_none, from_str], obj.get("weekday"))
        return EasterTimeSeason(octave, season, sunday, weekday)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.octave is not None:
            result["octave"] = from_union([from_none, from_str], self.octave)
        if self.season is not None:
            result["season"] = from_union([from_none, from_str], self.season)
        if self.sunday is not None:
            result["sunday"] = from_union([from_none, from_str], self.sunday)
        if self.weekday is not None:
            result["weekday"] = from_union([from_none, from_str], self.weekday)
        return result


class LentSeason(BaseModel):
    """Lent season localized names and descriptions."""

    day_after_ash_wed: Optional[str] = None
    """Day after Ash Wednesday terminology"""

    holy_week_day: Optional[str] = None
    """Holy Week day terminology"""

    season: Optional[str] = None
    """General season name for Lent"""

    sunday: Optional[str] = None
    """Sunday terminology during Lent"""

    weekday: Optional[str] = None
    """Weekday terminology during Lent"""

    @staticmethod
    def from_dict(obj: Any) -> 'LentSeason':
        assert isinstance(obj, dict)
        day_after_ash_wed = from_union([from_none, from_str], obj.get("day_after_ash_wed"))
        holy_week_day = from_union([from_none, from_str], obj.get("holy_week_day"))
        season = from_union([from_none, from_str], obj.get("season"))
        sunday = from_union([from_none, from_str], obj.get("sunday"))
        weekday = from_union([from_none, from_str], obj.get("weekday"))
        return LentSeason(day_after_ash_wed, holy_week_day, season, sunday, weekday)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.day_after_ash_wed is not None:
            result["day_after_ash_wed"] = from_union([from_none, from_str], self.day_after_ash_wed)
        if self.holy_week_day is not None:
            result["holy_week_day"] = from_union([from_none, from_str], self.holy_week_day)
        if self.season is not None:
            result["season"] = from_union([from_none, from_str], self.season)
        if self.sunday is not None:
            result["sunday"] = from_union([from_none, from_str], self.sunday)
        if self.weekday is not None:
            result["weekday"] = from_union([from_none, from_str], self.weekday)
        return result


class OrdinaryTimeSeason(BaseModel):
    """Ordinary Time season localized names and descriptions."""

    season: Optional[str] = None
    """General season name for Ordinary Time"""

    sunday: Optional[str] = None
    """Sunday terminology during Ordinary Time"""

    weekday: Optional[str] = None
    """Weekday terminology during Ordinary Time"""

    @staticmethod
    def from_dict(obj: Any) -> 'OrdinaryTimeSeason':
        assert isinstance(obj, dict)
        season = from_union([from_none, from_str], obj.get("season"))
        sunday = from_union([from_none, from_str], obj.get("sunday"))
        weekday = from_union([from_none, from_str], obj.get("weekday"))
        return OrdinaryTimeSeason(season, sunday, weekday)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.season is not None:
            result["season"] = from_union([from_none, from_str], self.season)
        if self.sunday is not None:
            result["sunday"] = from_union([from_none, from_str], self.sunday)
        if self.weekday is not None:
            result["weekday"] = from_union([from_none, from_str], self.weekday)
        return result


class PaschalTriduumSeason(BaseModel):
    """Paschal Triduum season localized names and descriptions."""

    season: Optional[str] = None
    """General season name for Paschal Triduum"""

    @staticmethod
    def from_dict(obj: Any) -> 'PaschalTriduumSeason':
        assert isinstance(obj, dict)
        season = from_union([from_none, from_str], obj.get("season"))
        return PaschalTriduumSeason(season)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.season is not None:
            result["season"] = from_union([from_none, from_str], self.season)
        return result


class SeasonsMetadata(BaseModel):
    """Liturgical season names and descriptions in the locale language.
    Provides localized names for each liturgical season and their components.
    """
    advent: Optional[AdventSeason] = None
    """Advent season names and descriptions"""

    christmas_time: Optional[ChristmasTimeSeason] = None
    """Christmas Time season names and descriptions"""

    easter_time: Optional[EasterTimeSeason] = None
    """Easter Time season names and descriptions"""

    lent: Optional[LentSeason] = None
    """Lent season names and descriptions"""

    ordinary_time: Optional[OrdinaryTimeSeason] = None
    """Ordinary Time season names and descriptions"""

    paschal_triduum: Optional[PaschalTriduumSeason] = None
    """Paschal Triduum season names and descriptions"""

    @staticmethod
    def from_dict(obj: Any) -> 'SeasonsMetadata':
        assert isinstance(obj, dict)
        advent = from_union([AdventSeason.from_dict, from_none], obj.get("advent"))
        christmas_time = from_union([ChristmasTimeSeason.from_dict, from_none], obj.get("christmas_time"))
        easter_time = from_union([EasterTimeSeason.from_dict, from_none], obj.get("easter_time"))
        lent = from_union([LentSeason.from_dict, from_none], obj.get("lent"))
        ordinary_time = from_union([OrdinaryTimeSeason.from_dict, from_none], obj.get("ordinary_time"))
        paschal_triduum = from_union([PaschalTriduumSeason.from_dict, from_none], obj.get("paschal_triduum"))
        return SeasonsMetadata(advent, christmas_time, easter_time, lent, ordinary_time, paschal_triduum)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.advent is not None:
            result["advent"] = from_union([lambda x: to_class(AdventSeason, x), from_none], self.advent)
        if self.christmas_time is not None:
            result["christmas_time"] = from_union([lambda x: to_class(ChristmasTimeSeason, x), from_none], self.christmas_time)
        if self.easter_time is not None:
            result["easter_time"] = from_union([lambda x: to_class(EasterTimeSeason, x), from_none], self.easter_time)
        if self.lent is not None:
            result["lent"] = from_union([lambda x: to_class(LentSeason, x), from_none], self.lent)
        if self.ordinary_time is not None:
            result["ordinary_time"] = from_union([lambda x: to_class(OrdinaryTimeSeason, x), from_none], self.ordinary_time)
        if self.paschal_triduum is not None:
            result["paschal_triduum"] = from_union([lambda x: to_class(PaschalTriduumSeason, x), from_none], self.paschal_triduum)
        return result


class ResourcesMetadata(BaseModel):
    """Metadata for localized resources.
    Contains all the localized strings and configurations for a specific locale.
    """
    colors: Optional[LocaleColors] = None
    """Liturgical color names in the locale language"""

    cycles: Optional[CyclesMetadata] = None
    """Liturgical cycle names in the locale language"""

    months: Optional[Dict[str, str]] = None
    """Month names (January, February, etc.) in the locale language"""

    ordinal_format: Optional[OrdinalFormat] = None
    """Format for displaying ordinal numbers (defaults to Numeric if not specified)"""

    ordinals_letters: Optional[Dict[str, str]] = None
    """Ordinal numbers as words (first, second, third, etc.) in the locale language"""

    ordinals_numeric: Optional[Dict[str, str]] = None
    """Ordinal numbers as numeric with suffix (1st, 2nd, 3rd, etc.) in the locale language"""

    periods: Optional[PeriodsMetadata] = None
    """Liturgical period names in the locale language"""

    ranks: Optional[RanksMetadata] = None
    """Liturgical rank names in the locale language"""

    seasons: Optional[SeasonsMetadata] = None
    """Liturgical season names and descriptions in the locale language"""

    weekdays: Optional[Dict[str, str]] = None
    """Weekday names (Sunday, Monday, etc.) in the locale language"""

    @staticmethod
    def from_dict(obj: Any) -> 'ResourcesMetadata':
        assert isinstance(obj, dict)
        colors = from_union([LocaleColors.from_dict, from_none], obj.get("colors"))
        cycles = from_union([CyclesMetadata.from_dict, from_none], obj.get("cycles"))
        months = from_union([from_none, lambda x: from_dict(from_str, x)], obj.get("months"))
        ordinal_format = from_union([from_none, OrdinalFormat], obj.get("ordinal_format"))
        ordinals_letters = from_union([from_none, lambda x: from_dict(from_str, x)], obj.get("ordinals_letters"))
        ordinals_numeric = from_union([from_none, lambda x: from_dict(from_str, x)], obj.get("ordinals_numeric"))
        periods = from_union([PeriodsMetadata.from_dict, from_none], obj.get("periods"))
        ranks = from_union([RanksMetadata.from_dict, from_none], obj.get("ranks"))
        seasons = from_union([SeasonsMetadata.from_dict, from_none], obj.get("seasons"))
        weekdays = from_union([from_none, lambda x: from_dict(from_str, x)], obj.get("weekdays"))
        return ResourcesMetadata(colors, cycles, months, ordinal_format, ordinals_letters, ordinals_numeric, periods, ranks, seasons, weekdays)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.colors is not None:
            result["colors"] = from_union([lambda x: to_class(LocaleColors, x), from_none], self.colors)
        if self.cycles is not None:
            result["cycles"] = from_union([lambda x: to_class(CyclesMetadata, x), from_none], self.cycles)
        if self.months is not None:
            result["months"] = from_union([from_none, lambda x: from_dict(from_str, x)], self.months)
        if self.ordinal_format is not None:
            result["ordinal_format"] = from_union([from_none, lambda x: to_enum(OrdinalFormat, x)], self.ordinal_format)
        if self.ordinals_letters is not None:
            result["ordinals_letters"] = from_union([from_none, lambda x: from_dict(from_str, x)], self.ordinals_letters)
        if self.ordinals_numeric is not None:
            result["ordinals_numeric"] = from_union([from_none, lambda x: from_dict(from_str, x)], self.ordinals_numeric)
        if self.periods is not None:
            result["periods"] = from_union([lambda x: to_class(PeriodsMetadata, x), from_none], self.periods)
        if self.ranks is not None:
            result["ranks"] = from_union([lambda x: to_class(RanksMetadata, x), from_none], self.ranks)
        if self.seasons is not None:
            result["seasons"] = from_union([lambda x: to_class(SeasonsMetadata, x), from_none], self.seasons)
        if self.weekdays is not None:
            result["weekdays"] = from_union([from_none, lambda x: from_dict(from_str, x)], self.weekdays)
        return result


class Resources(BaseModel):
    """Resources definition"""

    locale: str
    """Locale code of the resources, in BCP-47 IETF tag format"""

    schema: Optional[str] = None
    entities: Optional[Dict[str, Entity]] = None
    """Entities of the resources: a person, a place, an event, etc."""

    metadata: Optional[ResourcesMetadata] = None
    """Metadata of the resources"""

    @staticmethod
    def from_dict(obj: Any) -> 'Resources':
        assert isinstance(obj, dict)
        locale = from_str(obj.get("locale"))
        schema = from_union([from_none, from_str], obj.get("$schema"))
        entities = from_union([from_none, lambda x: from_dict(Entity.from_dict, x)], obj.get("entities"))
        metadata = from_union([ResourcesMetadata.from_dict, from_none], obj.get("metadata"))
        return Resources(locale, schema, entities, metadata)

    def to_dict(self) -> dict:
        result: dict = {}
        result["locale"] = from_str(self.locale)
        if self.schema is not None:
            result["$schema"] = from_union([from_none, from_str], self.schema)
        if self.entities is not None:
            result["entities"] = from_union([from_none, lambda x: from_dict(lambda x: to_class(Entity, x), x)], self.entities)
        if self.metadata is not None:
            result["metadata"] = from_union([lambda x: to_class(ResourcesMetadata, x), from_none], self.metadata)
        return result


class SundayCycleCombined(Enum):
    """Combined Sunday cycle for cases where readings can apply to multiple years.
    This allows for flexible configuration where the same readings can be used
    across different combinations of Sunday cycles.
    
    Years A and B combined
    
    Years A and C combined
    
    Years B and C combined
    """
    YEAR_A_B = "YEAR_A_B"
    YEAR_A_C = "YEAR_A_C"
    YEAR_B_C = "YEAR_B_C"


class Types(BaseModel):
    acclamation: Optional[Acclamation] = None
    bible_book: Optional[BibleBook] = None
    calendar_context: Optional[CalendarContext] = None
    calendar_definition: Optional[CalendarDefinition] = None
    celebration_summary: Optional[CelebrationSummary] = None
    date_def_with_offset: Optional[DateDefWithOffset] = None
    day_of_week: Optional[int] = None
    liturgical_cycle: Optional[LiturgicalCycle] = None
    liturgical_day: Optional[LiturgicalDay] = None
    mass_context: Optional[MassContext] = None
    mass_part: Optional[MassPart] = None
    month_index: Optional[int] = None
    resources: Optional[Resources] = None
    saint_count: Optional[Union[int, SaintCountEnum]] = None
    sunday_cycle_combined: Optional[SundayCycleCombined] = None

    @staticmethod
    def from_dict(obj: Any) -> 'Types':
        assert isinstance(obj, dict)
        acclamation = from_union([Acclamation, from_none], obj.get("acclamation"))
        bible_book = from_union([BibleBook, from_none], obj.get("bibleBook"))
        calendar_context = from_union([CalendarContext, from_none], obj.get("calendarContext"))
        calendar_definition = from_union([CalendarDefinition.from_dict, from_none], obj.get("calendarDefinition"))
        celebration_summary = from_union([CelebrationSummary.from_dict, from_none], obj.get("celebrationSummary"))
        date_def_with_offset = from_union([DateDefWithOffset.from_dict, from_none], obj.get("dateDefWithOffset"))
        day_of_week = from_union([from_int, from_none], obj.get("dayOfWeek"))
        liturgical_cycle = from_union([LiturgicalCycle, from_none], obj.get("liturgicalCycle"))
        liturgical_day = from_union([LiturgicalDay.from_dict, from_none], obj.get("liturgicalDay"))
        mass_context = from_union([MassContext.from_dict, from_none], obj.get("massContext"))
        mass_part = from_union([MassPart, from_none], obj.get("massPart"))
        month_index = from_union([from_int, from_none], obj.get("monthIndex"))
        resources = from_union([Resources.from_dict, from_none], obj.get("resources"))
        saint_count = from_union([from_int, from_none, SaintCountEnum], obj.get("saintCount"))
        sunday_cycle_combined = from_union([SundayCycleCombined, from_none], obj.get("sundayCycleCombined"))
        return Types(acclamation, bible_book, calendar_context, calendar_definition, celebration_summary, date_def_with_offset, day_of_week, liturgical_cycle, liturgical_day, mass_context, mass_part, month_index, resources, saint_count, sunday_cycle_combined)

    def to_dict(self) -> dict:
        result: dict = {}
        if self.acclamation is not None:
            result["acclamation"] = from_union([lambda x: to_enum(Acclamation, x), from_none], self.acclamation)
        if self.bible_book is not None:
            result["bibleBook"] = from_union([lambda x: to_enum(BibleBook, x), from_none], self.bible_book)
        if self.calendar_context is not None:
            result["calendarContext"] = from_union([lambda x: to_enum(CalendarContext, x), from_none], self.calendar_context)
        if self.calendar_definition is not None:
            result["calendarDefinition"] = from_union([lambda x: to_class(CalendarDefinition, x), from_none], self.calendar_definition)
        if self.celebration_summary is not None:
            result["celebrationSummary"] = from_union([lambda x: to_class(CelebrationSummary, x), from_none], self.celebration_summary)
        if self.date_def_with_offset is not None:
            result["dateDefWithOffset"] = from_union([lambda x: to_class(DateDefWithOffset, x), from_none], self.date_def_with_offset)
        if self.day_of_week is not None:
            result["dayOfWeek"] = from_union([from_int, from_none], self.day_of_week)
        if self.liturgical_cycle is not None:
            result["liturgicalCycle"] = from_union([lambda x: to_enum(LiturgicalCycle, x), from_none], self.liturgical_cycle)
        if self.liturgical_day is not None:
            result["liturgicalDay"] = from_union([lambda x: to_class(LiturgicalDay, x), from_none], self.liturgical_day)
        if self.mass_context is not None:
            result["massContext"] = from_union([lambda x: to_class(MassContext, x), from_none], self.mass_context)
        if self.mass_part is not None:
            result["massPart"] = from_union([lambda x: to_enum(MassPart, x), from_none], self.mass_part)
        if self.month_index is not None:
            result["monthIndex"] = from_union([from_int, from_none], self.month_index)
        if self.resources is not None:
            result["resources"] = from_union([lambda x: to_class(Resources, x), from_none], self.resources)
        if self.saint_count is not None:
            result["saintCount"] = from_union([from_int, from_none, lambda x: to_enum(SaintCountEnum, x)], self.saint_count)
        if self.sunday_cycle_combined is not None:
            result["sundayCycleCombined"] = from_union([lambda x: to_enum(SundayCycleCombined, x), from_none], self.sunday_cycle_combined)
        return result


def types_from_dict(s: Any) -> Types:
    return Types.from_dict(s)


def types_to_dict(x: Types) -> Any:
    return to_class(Types, x)
