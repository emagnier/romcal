#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Liturgical rank indicating the importance and celebration style of a liturgical day
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
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

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_rank_iteration_order() {
        let variants: Vec<Rank> = Rank::iter().collect();

        // Verify that the order is exactly the declaration order
        assert_eq!(variants[0], Rank::Solemnity);
        assert_eq!(variants[1], Rank::Sunday);
        assert_eq!(variants[2], Rank::Feast);
        assert_eq!(variants[3], Rank::Memorial);
        assert_eq!(variants[4], Rank::OptionalMemorial);
        assert_eq!(variants[5], Rank::Weekday);

        // Verify that we have all variants
        assert_eq!(variants.len(), 6);
    }

    #[test]
    fn test_rank_iteration_consistency() {
        // Verify that the order is always the same across multiple iterations
        let first_iteration: Vec<Rank> = Rank::iter().collect();
        let second_iteration: Vec<Rank> = Rank::iter().collect();

        assert_eq!(first_iteration, second_iteration);
    }

    #[test]
    fn test_rank_serialization() {
        // Verify that serialization works
        let rank = Rank::Solemnity;
        let json = serde_json::to_string(&rank).unwrap();
        assert_eq!(json, "\"SOLEMNITY\"");

        let deserialized: Rank = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Rank::Solemnity);
    }

    #[test]
    fn test_rank_hierarchical_order() {
        // Test that rank follows the correct hierarchical order (highest to lowest importance)
        let variants: Vec<Rank> = Rank::iter().collect();

        // The first should be the highest rank (Solemnity)
        assert_eq!(variants[0], Rank::Solemnity);

        // The last should be the lowest rank (Weekday)
        assert_eq!(variants[5], Rank::Weekday);

        // Verify the complete hierarchy
        assert_eq!(variants[0], Rank::Solemnity); // Highest
        assert_eq!(variants[1], Rank::Sunday); // Second highest
        assert_eq!(variants[2], Rank::Feast); // Third
        assert_eq!(variants[3], Rank::Memorial); // Fourth
        assert_eq!(variants[4], Rank::OptionalMemorial); // Fifth
        assert_eq!(variants[5], Rank::Weekday); // Lowest
    }

    #[test]
    fn test_rank_comparison() {
        // Test that rank comparison works as expected
        assert!(Rank::Solemnity == Rank::Solemnity);
        assert!(Rank::Sunday != Rank::Feast);
        assert!(Rank::Memorial != Rank::OptionalMemorial);
    }
}
