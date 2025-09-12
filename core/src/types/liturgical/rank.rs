use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Liturgical rank indicating the importance and celebration style of a liturgical day
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Rank {
    /// Solemnities are counted among the most important days, whose celebration
    /// begins with First Vespers (Evening Prayer I) on the preceding day. Some Solemnities
    /// are also endowed with their own Vigil Mass, which is to be used on the evening of the
    /// preceding day, if an evening Mass is celebrated. (UNLY #11)
    Solemnity,

    /// On the first day of each week, which is known as the Day of the Lord or the Lord's
    /// Day, the Church, by an apostolic tradition that draws its origin from the very day of
    /// the Resurrection of Christ, celebrates the Paschal Mystery. Hence, Sunday must be
    /// considered the primordial feast day. (UNLY #4)
    Sunday,

    /// Feasts are celebrated within the limits of the natural day; accordingly they have
    /// no First Vespers (Evening Prayer I), except in the case of Feasts of the Lord that fall
    /// on a Sunday in Ordinary Time or in Christmas Time and which replace the Sunday
    /// Office. (UNLY #13)
    Feast,

    /// **Obligatory memorials** are liturgical commemorations of saints, events, or aspects of the
    /// faith. Their observance is mandatory and integrated into the celebration of the occurring
    /// weekday, following the liturgical norms outlined in the General Instruction of the Roman Missal
    /// and the Liturgy of the Hours.
    /// When an **obligatory memorial** falls on a weekday during the liturgical season of Lent or a
    /// privileged weekday of Advent, it must only be celebrated as an **optional memorial**, as Lent
    /// and Advent have their own specific liturgical observances that take precedence.
    Memorial,

    /// **Optional memorials** are liturgical commemorations of saints, events, or aspects of the
    /// faith, but they are not obligatory.
    /// Their observance is integrated into the celebration of the occurring weekday, adhering to the
    /// liturgical norms provided in the General Instruction of the Roman Missal and the Liturgy of
    /// the Hours.
    /// In cases where multiple **optional memorials** are designated on the same day in the liturgical
    /// calendar, only one of them may be celebrated, and the others must be omitted (UNLY #14).
    /// This allows for some flexibility in choosing which optional memorial to commemorate when
    /// multiple options are available.
    OptionalMemorial,

    /// The days of the week that follow Sunday are called weekdays; however, they are
    /// celebrated differently according to the importance of each.
    ///
    /// a. Ash Wednesday and the weekdays of Holy Week, from Monday up to and including
    ///    Thursday, take precedence over all other celebrations.
    /// b. The weekdays of Advent from 17 December up to and including 24 December
    ///    and all the weekdays of Lent have precedence over Obligatory Memorials.
    /// c. Other weekdays give way to all Solemnities and Feasts and are combined with
    ///    Memorials.
    ///
    ///  (UNLY #16)
    Weekday,
}
