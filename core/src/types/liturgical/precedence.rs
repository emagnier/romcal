use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Liturgical precedence levels for determining which celebration takes priority
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(non_camel_case_types)]
pub enum Precedence {
    // 1 - The Paschal Triduum of the Passion and Resurrection of the Lord.
    Triduum_1,

    // 2 - The Nativity of the Lord, the Epiphany, the Ascension, or Pentecost.
    ProperOfTimeSolemnity_2,
    // 2 - A Sunday of Advent, Lent, or Easter.
    PrivilegedSunday_2,
    // 2 - Ash Wednesday.
    AshWednesday_2,
    // 2 - A weekday of Holy Week from Monday up to and including Thursday.
    WeekdayOfHolyWeek_2,
    // 2 - A day within the Octave of Easter.
    WeekdayOfEasterOctave_2,

    // 3 - A Solemnity inscribed in the General Calendar, whether of the Lord, of the Blessed Virgin Mary, or of a Saint.
    GeneralSolemnity_3,

    // 3 - The Commemoration of All the Faithful Departed.
    CommemorationOfAllTheFaithfulDeparted_3,

    // 4 - Proper Solemnity.
    // 4a - A proper Solemnity of the principal Patron of the place, city, or state.
    ProperSolemnity_PrincipalPatron_4a,
    // 4b - The Solemnity of the dedication and of the anniversary of the dedication of the own church.
    ProperSolemnity_DedicationOfTheOwnChurch_4b,
    // 4c - The solemnity of the title of the own church.
    ProperSolemnity_TitleOfTheOwnChurch_4c,
    // 4d - A Solemnity either of the Title or of the Founder or of the principal Patron of an Order or Congregation.
    ProperSolemnity_TitleOrFounderOrPrimaryPatronOfAReligiousOrg_4d,

    // 5 - A Feast of the Lord inscribed in the General Calendar.
    GeneralLordFeast_5,

    // 6 - A Sunday of Christmas Time or a Sunday in Ordinary Time.
    UnprivilegedSunday_6,

    // 7 - A Feast of the Blessed Virgin Mary or of a Saint in the General Calendar.
    GeneralFeast_7,

    // 8 - Proper Feast
    // 8a - The Proper Feast of the principal Patron of the diocese.
    ProperFeast_PrincipalPatronOfADiocese_8a,
    // 8b - The Proper Feast of the anniversary of the dedication of the cathedral church
    ProperFeast_DedicationOfTheCathedralChurch_8b,
    // 8c - The Proper Feast of the principal Patron of a region or province, or a country, or of a wider territory.
    ProperFeast_PrincipalPatronOfARegion_8c,
    // 8d - The Proper Feast of the Title, Founder, or principal Patron of an Order or Congregation
    ProperFeast_TitleOrFounderOrPrimaryPatronOfAReligiousOrg_8d,
    // 8e - Other Feast, proper to an individual church.
    ProperFeast_ToAnIndividualChurch_8e,
    // 8f - Other Proper Feast inscribed in the Calendar of each diocese or Order or Congregation.
    ProperFeast_8f,

    // 9 - Privileged Weekday
    PrivilegedWeekday_9,

    // 10 - Obligatory Memorials in the General Calendar.
    GeneralMemorial_10,

    // 11 - Proper Obligatory Memorial.
    // 11a - Proper Obligatory Memorial of a secondary Patron of the place, diocese, region, or religious province.
    ProperMemorial_SecondPatron_11a,
    // 11b - Other Proper Obligatory Memorial inscribed in the Calendar of each diocese, or Order or congregation.
    ProperMemorial_11b,

    // 12 - Optional Memorial
    OptionalMemorial_12,

    // 13 - Weekday
    Weekday_13,
}
