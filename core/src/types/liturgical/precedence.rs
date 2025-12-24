use crate::types::liturgical::Rank;
#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Liturgical precedence levels for determining which celebration takes priority.
/// Defines the hierarchical order of liturgical celebrations according to UNLY norms.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[allow(non_camel_case_types)]
pub enum Precedence {
    /// 1 - The Paschal Triduum of the Passion and Resurrection of the Lord.
    Triduum_1,

    /// 2 - The Nativity of the Lord, the Epiphany, the Ascension, or Pentecost.
    ProperOfTimeSolemnity_2,
    /// 2 - A Sunday of Advent, Lent, or Easter.
    PrivilegedSunday_2,
    /// 2 - Ash Wednesday.
    AshWednesday_2,
    /// 2 - A weekday of Holy Week from Monday up to and including Thursday.
    WeekdayOfHolyWeek_2,
    /// 2 - A day within the Octave of Easter.
    WeekdayOfEasterOctave_2,

    /// 3 - A Solemnity inscribed in the General Calendar, whether of the Lord, of the Blessed Virgin Mary, or of a Saint.
    GeneralSolemnity_3,

    /// 3 - The Commemoration of All the Faithful Departed.
    CommemorationOfAllTheFaithfulDeparted_3,

    /// 4a - A proper Solemnity of the principal Patron of the place, city, or state.
    ProperSolemnity_PrincipalPatron_4a,
    /// 4b - The Solemnity of the dedication and of the anniversary of the dedication of the own church.
    ProperSolemnity_DedicationOfTheOwnChurch_4b,
    /// 4c - The solemnity of the title of the own church.
    ProperSolemnity_TitleOfTheOwnChurch_4c,
    /// 4d - A Solemnity either of the Title or of the Founder or of the principal Patron of an Order or Congregation.
    ProperSolemnity_TitleOrFounderOrPrimaryPatronOfAReligiousOrg_4d,

    /// 5 - A Feast of the Lord inscribed in the General Calendar.
    GeneralLordFeast_5,

    /// 6 - A Sunday of Christmas Time or a Sunday in Ordinary Time.
    UnprivilegedSunday_6,

    /// 7 - A Feast of the Blessed Virgin Mary or of a Saint in the General Calendar.
    GeneralFeast_7,

    /// 8a - The Proper Feast of the principal Patron of the diocese.
    ProperFeast_PrincipalPatronOfADiocese_8a,
    /// 8b - The Proper Feast of the anniversary of the dedication of the cathedral church.
    ProperFeast_DedicationOfTheCathedralChurch_8b,
    /// 8c - The Proper Feast of the principal Patron of a region or province, or a country, or of a wider territory.
    ProperFeast_PrincipalPatronOfARegion_8c,
    /// 8d - The Proper Feast of the Title, Founder, or principal Patron of an Order or Congregation.
    ProperFeast_TitleOrFounderOrPrimaryPatronOfAReligiousOrg_8d,
    /// 8e - Other Feast, proper to an individual church.
    ProperFeast_ToAnIndividualChurch_8e,
    /// 8f - Other Proper Feast inscribed in the Calendar of each diocese or Order or Congregation.
    ProperFeast_8f,

    /// 9 - Privileged Weekday.
    PrivilegedWeekday_9,

    /// 10 - Obligatory Memorials in the General Calendar.
    GeneralMemorial_10,

    /// 11a - Proper Obligatory Memorial of a secondary Patron of the place, diocese, region, or religious province.
    ProperMemorial_SecondPatron_11a,
    /// 11b - Other Proper Obligatory Memorial inscribed in the Calendar of each diocese, or Order or congregation.
    ProperMemorial_11b,

    /// 12 - Optional Memorial.
    OptionalMemorial_12,

    /// 13 - Weekday.
    Weekday_13,
}

impl Precedence {
    /// Returns the corresponding liturgical rank for this precedence level.
    ///
    /// This method provides a convenient way to get the liturgical rank directly
    /// from a precedence level, following the hierarchical order defined in the
    /// Universal Norms on the Liturgical Year and the Calendar (UNLY).
    ///
    /// # Returns
    ///
    /// The corresponding liturgical rank for this precedence level
    ///
    /// # Examples
    ///
    /// ```rust
    /// use romcal::types::liturgical::{Precedence, Rank};
    ///
    /// let precedence = Precedence::Triduum_1;
    /// let rank = precedence.to_rank();
    /// assert_eq!(rank, Rank::Weekday);
    ///
    /// let precedence = Precedence::GeneralSolemnity_3;
    /// let rank = precedence.to_rank();
    /// assert_eq!(rank, Rank::Solemnity);
    /// ```
    pub fn to_rank(&self) -> Rank {
        match self {
            // 1 - The Paschal Triduum of the Passion and Resurrection of the Lord
            Precedence::Triduum_1 => Rank::Weekday,

            // 2 - Proper of Time Solemnities
            Precedence::ProperOfTimeSolemnity_2 => Rank::Solemnity,
            // 2 - Privileged Sundays
            Precedence::PrivilegedSunday_2 => Rank::Sunday,
            // 2 - Ash Wednesday
            Precedence::AshWednesday_2 => Rank::Weekday,
            // 2 - Weekdays of Holy Week
            Precedence::WeekdayOfHolyWeek_2 => Rank::Weekday,
            // 2 - Weekdays of Easter Octave
            Precedence::WeekdayOfEasterOctave_2 => Rank::Solemnity,

            // 3 - General Solemnities
            Precedence::GeneralSolemnity_3 => Rank::Solemnity,
            // 3 - Commemoration of All the Faithful Departed
            Precedence::CommemorationOfAllTheFaithfulDeparted_3 => Rank::Feast,

            // 4 - Proper Solemnities
            Precedence::ProperSolemnity_PrincipalPatron_4a => Rank::Solemnity,
            Precedence::ProperSolemnity_DedicationOfTheOwnChurch_4b => Rank::Solemnity,
            Precedence::ProperSolemnity_TitleOfTheOwnChurch_4c => Rank::Solemnity,
            Precedence::ProperSolemnity_TitleOrFounderOrPrimaryPatronOfAReligiousOrg_4d => {
                Rank::Solemnity
            }

            // 5 - General Lord Feasts
            Precedence::GeneralLordFeast_5 => Rank::Feast,

            // 6 - Unprivileged Sundays
            Precedence::UnprivilegedSunday_6 => Rank::Sunday,

            // 7 - General Feasts
            Precedence::GeneralFeast_7 => Rank::Feast,

            // 8 - Proper Feasts
            Precedence::ProperFeast_PrincipalPatronOfADiocese_8a => Rank::Feast,
            Precedence::ProperFeast_DedicationOfTheCathedralChurch_8b => Rank::Feast,
            Precedence::ProperFeast_PrincipalPatronOfARegion_8c => Rank::Feast,
            Precedence::ProperFeast_TitleOrFounderOrPrimaryPatronOfAReligiousOrg_8d => Rank::Feast,
            Precedence::ProperFeast_ToAnIndividualChurch_8e => Rank::Feast,
            Precedence::ProperFeast_8f => Rank::Feast,

            // 9 - Privileged Weekdays
            Precedence::PrivilegedWeekday_9 => Rank::Weekday,

            // 10 - General Memorials
            Precedence::GeneralMemorial_10 => Rank::Memorial,

            // 11 - Proper Memorials
            Precedence::ProperMemorial_SecondPatron_11a => Rank::Memorial,
            Precedence::ProperMemorial_11b => Rank::Memorial,

            // 12 - Optional Memorials
            Precedence::OptionalMemorial_12 => Rank::OptionalMemorial,

            // 13 - Weekdays
            Precedence::Weekday_13 => Rank::Weekday,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_precedence_iteration_order() {
        let variants: Vec<Precedence> = Precedence::iter().collect();

        // Verify that the order is exactly the declaration order
        assert_eq!(variants[0], Precedence::Triduum_1);
        assert_eq!(variants[1], Precedence::ProperOfTimeSolemnity_2);
        assert_eq!(variants[2], Precedence::PrivilegedSunday_2);
        assert_eq!(variants[3], Precedence::AshWednesday_2);
        assert_eq!(variants[4], Precedence::WeekdayOfHolyWeek_2);
        assert_eq!(variants[5], Precedence::WeekdayOfEasterOctave_2);
        assert_eq!(variants[6], Precedence::GeneralSolemnity_3);
        assert_eq!(
            variants[7],
            Precedence::CommemorationOfAllTheFaithfulDeparted_3
        );
        assert_eq!(variants[8], Precedence::ProperSolemnity_PrincipalPatron_4a);
        assert_eq!(
            variants[9],
            Precedence::ProperSolemnity_DedicationOfTheOwnChurch_4b
        );
        assert_eq!(
            variants[10],
            Precedence::ProperSolemnity_TitleOfTheOwnChurch_4c
        );
        assert_eq!(
            variants[11],
            Precedence::ProperSolemnity_TitleOrFounderOrPrimaryPatronOfAReligiousOrg_4d
        );
        assert_eq!(variants[12], Precedence::GeneralLordFeast_5);
        assert_eq!(variants[13], Precedence::UnprivilegedSunday_6);
        assert_eq!(variants[14], Precedence::GeneralFeast_7);
        assert_eq!(
            variants[15],
            Precedence::ProperFeast_PrincipalPatronOfADiocese_8a
        );
        assert_eq!(
            variants[16],
            Precedence::ProperFeast_DedicationOfTheCathedralChurch_8b
        );
        assert_eq!(
            variants[17],
            Precedence::ProperFeast_PrincipalPatronOfARegion_8c
        );
        assert_eq!(
            variants[18],
            Precedence::ProperFeast_TitleOrFounderOrPrimaryPatronOfAReligiousOrg_8d
        );
        assert_eq!(
            variants[19],
            Precedence::ProperFeast_ToAnIndividualChurch_8e
        );
        assert_eq!(variants[20], Precedence::ProperFeast_8f);
        assert_eq!(variants[21], Precedence::PrivilegedWeekday_9);
        assert_eq!(variants[22], Precedence::GeneralMemorial_10);
        assert_eq!(variants[23], Precedence::ProperMemorial_SecondPatron_11a);
        assert_eq!(variants[24], Precedence::ProperMemorial_11b);
        assert_eq!(variants[25], Precedence::OptionalMemorial_12);
        assert_eq!(variants[26], Precedence::Weekday_13);

        // Verify that we have all variants
        assert_eq!(variants.len(), 27);
    }

    #[test]
    fn test_precedence_iteration_consistency() {
        // Verify that the order is always the same across multiple iterations
        let first_iteration: Vec<Precedence> = Precedence::iter().collect();
        let second_iteration: Vec<Precedence> = Precedence::iter().collect();

        assert_eq!(first_iteration, second_iteration);
    }

    #[test]
    fn test_precedence_serialization() {
        // Verify that serialization works
        let precedence = Precedence::Triduum_1;
        let json = serde_json::to_string(&precedence).unwrap();
        assert_eq!(json, "\"TRIDUUM_1\"");

        let deserialized: Precedence = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, Precedence::Triduum_1);
    }

    #[test]
    fn test_precedence_to_rank() {
        // Test that to_rank method works correctly
        assert_eq!(Precedence::Triduum_1.to_rank(), Rank::Weekday);
        assert_eq!(Precedence::GeneralSolemnity_3.to_rank(), Rank::Solemnity);
        assert_eq!(Precedence::PrivilegedSunday_2.to_rank(), Rank::Sunday);
        assert_eq!(Precedence::GeneralFeast_7.to_rank(), Rank::Feast);
        assert_eq!(Precedence::GeneralMemorial_10.to_rank(), Rank::Memorial);
        assert_eq!(
            Precedence::OptionalMemorial_12.to_rank(),
            Rank::OptionalMemorial
        );
    }

    #[test]
    fn test_precedence_hierarchical_order() {
        // Test that precedence follows the correct hierarchical order
        let variants: Vec<Precedence> = Precedence::iter().collect();

        // The first few should be the highest precedence (1-2)
        assert!(matches!(variants[0], Precedence::Triduum_1));
        assert!(matches!(variants[1], Precedence::ProperOfTimeSolemnity_2));
        assert!(matches!(variants[2], Precedence::PrivilegedSunday_2));

        // The last should be the lowest precedence (13)
        assert!(matches!(variants[26], Precedence::Weekday_13));
    }
}
