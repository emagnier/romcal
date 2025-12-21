use serde::{Deserialize, Serialize};
use strum::EnumIter;

/// Parts that make up the Mass celebration.
/// Each part represents a specific element of the liturgical celebration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter, PartialOrd, Ord)]
#[cfg_attr(feature = "schema-gen", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum MassPart {
    /// Messianic entry reading (during the procession with palms, before the Mass of the Passion)
    MessianicEntry,
    /// Entrance Antiphon - opening chant of the Mass
    EntranceAntiphon,
    /// Collect - opening prayer of the Mass
    Collect,
    /// Reading 1 - first reading (usually from the Old Testament)
    #[serde(rename = "reading_1")]
    Reading1,
    /// Psalm - responsorial psalm
    Psalm,
    /// Canticle - biblical canticle
    Canticle,
    /// Reading 2 - second reading (usually from the New Testament)
    #[serde(rename = "reading_2")]
    Reading2,
    /// Psalm (Easter Vigil)
    #[serde(rename = "easter_vigil_psalm_2")]
    EasterVigilPsalm2,
    /// Reading 3 - third reading (Easter Vigil)
    #[serde(rename = "easter_vigil_reading_3")]
    EasterVigilReading3,
    #[serde(rename = "easter_vigil_canticle_3")]
    /// Canticle 3 (Easter Vigil)
    EasterVigilCanticle3,
    /// Reading 4 - fourth reading (Easter Vigil)
    #[serde(rename = "easter_vigil_reading_4")]
    EasterVigilReading4,
    #[serde(rename = "easter_vigil_psalm_4")]
    /// Psalm 4 (Easter Vigil)
    EasterVigilPsalm4,
    /// Reading 5 - fifth reading (Easter Vigil)
    #[serde(rename = "easter_vigil_reading_5")]
    EasterVigilReading5,
    #[serde(rename = "easter_vigil_canticle_5")]
    /// Canticle 5 (Easter Vigil)
    EasterVigilCanticle5,
    /// Reading 6 - sixth reading (Easter Vigil)
    #[serde(rename = "easter_vigil_reading_6")]
    EasterVigilReading6,
    #[serde(rename = "easter_vigil_psalm_6")]
    /// Psalm 6 (Easter Vigil)
    EasterVigilPsalm6,
    /// Reading 7 - seventh reading (Easter Vigil)
    #[serde(rename = "easter_vigil_reading_7")]
    EasterVigilReading7,
    #[serde(rename = "easter_vigil_psalm_7")]
    /// Psalm 7 (Easter Vigil)
    EasterVigilPsalm7,
    /// Epistle - reading from the epistles (Easter Vigil)
    EasterVigilEpistle,
    /// Sequence - special chant on certain feasts
    Sequence,
    /// Alleluia - acclamation before the Gospel
    Alleluia,
    /// Gospel - reading from the Gospels
    Gospel,
    /// Prayer over the Offerings - prayer during the offertory
    PrayerOverTheOfferings,
    /// Preface - introduction to the Eucharistic Prayer
    Preface,
    /// Communion Antiphon - chant during communion
    CommunionAntiphon,
    /// Prayer after Communion - concluding prayer
    PrayerAfterCommunion,
    /// Solemn Blessing - special blessing on certain occasions
    SolemnBlessing,
    /// Prayer over the People - blessing over the congregation
    PrayerOverThePeople,
}

impl MassPart {
    /// Get all reading mass parts.
    /// This corresponds to the TypeScript `ReadingsPartTypes` array.
    pub fn reading_parts() -> &'static [MassPart] {
        &[
            MassPart::MessianicEntry,
            MassPart::Reading1,
            MassPart::Reading2,
            MassPart::EasterVigilReading3,
            MassPart::EasterVigilReading4,
            MassPart::EasterVigilReading5,
            MassPart::EasterVigilReading6,
            MassPart::EasterVigilReading7,
            MassPart::EasterVigilEpistle,
            MassPart::Gospel,
        ]
    }

    /// Check if a mass part is a reading part.
    /// This corresponds to the TypeScript `isReadingPartType` function.
    pub fn is_reading_part(&self) -> bool {
        Self::reading_parts().contains(self)
    }

    /// Get all antiphon mass parts.
    /// This corresponds to the TypeScript `AntiphonsPartTypes` array.
    pub fn antiphon_parts() -> &'static [MassPart] {
        &[MassPart::EntranceAntiphon, MassPart::CommunionAntiphon]
    }

    /// Check if a mass part is an antiphon part.
    /// This corresponds to the TypeScript `isAntiphonPartType` function.
    pub fn is_antiphon_part(&self) -> bool {
        Self::antiphon_parts().contains(self)
    }

    /// Get all prayer mass parts.
    /// This corresponds to the TypeScript `PrayersPartTypes` array.
    pub fn prayer_parts() -> &'static [MassPart] {
        &[
            MassPart::Collect,
            MassPart::PrayerOverTheOfferings,
            MassPart::Preface,
            MassPart::PrayerAfterCommunion,
            MassPart::SolemnBlessing,
            MassPart::PrayerOverThePeople,
        ]
    }

    /// Check if a mass part is a prayer part.
    pub fn is_prayer_part(&self) -> bool {
        Self::prayer_parts().contains(self)
    }

    /// Get all psalm mass parts.
    /// This corresponds to the TypeScript `PsalmsPartTypes` array.
    pub fn psalm_parts() -> &'static [MassPart] {
        &[
            MassPart::Psalm,
            MassPart::Canticle,
            MassPart::EasterVigilPsalm2,
            MassPart::EasterVigilCanticle3,
            MassPart::EasterVigilPsalm4,
            MassPart::EasterVigilCanticle5,
            MassPart::EasterVigilPsalm6,
            MassPart::EasterVigilPsalm7,
        ]
    }

    /// Check if a mass part is a psalm part.
    pub fn is_psalm_part(&self) -> bool {
        Self::psalm_parts().contains(self)
    }
}

// Schema generation functions (only compiled when feature "schema-gen" is enabled)
#[cfg(feature = "schema-gen")]
pub fn get_mass_part_description(part: &MassPart) -> &'static str {
    match part {
        MassPart::MessianicEntry => {
            "Messianic entry reading (during the procession with palms, before the Mass of the Passion)"
        }
        MassPart::EntranceAntiphon => "Entrance Antiphon - opening chant of the Mass",
        MassPart::Collect => "Collect - opening prayer of the Mass",
        MassPart::Reading1 => "Reading 1 - first reading (usually from the Old Testament)",
        MassPart::Psalm => "Psalm - responsorial psalm",
        MassPart::Canticle => "Canticle - biblical canticle",
        MassPart::Reading2 => "Reading 2 - second reading (usually from the New Testament)",
        MassPart::EasterVigilPsalm2 => "Psalm 2 (Easter Vigil)",
        MassPart::EasterVigilReading3 => "Reading 3 - third reading (Easter Vigil)",
        MassPart::EasterVigilCanticle3 => "Canticle 3 (Easter Vigil)",
        MassPart::EasterVigilReading4 => "Reading 4 - fourth reading (Easter Vigil)",
        MassPart::EasterVigilPsalm4 => "Psalm 4 (Easter Vigil)",
        MassPart::EasterVigilReading5 => "Reading 5 - fifth reading (Easter Vigil)",
        MassPart::EasterVigilCanticle5 => "Canticle 5 (Easter Vigil)",
        MassPart::EasterVigilReading6 => "Reading 6 - sixth reading (Easter Vigil)",
        MassPart::EasterVigilPsalm6 => "Psalm 6 (Easter Vigil)",
        MassPart::EasterVigilReading7 => "Reading 7 - seventh reading (Easter Vigil)",
        MassPart::EasterVigilPsalm7 => "Psalm 7 (Easter Vigil)",
        MassPart::EasterVigilEpistle => "Epistle - reading from the epistles (Easter Vigil)",
        MassPart::Sequence => "Sequence - special chant on certain feasts",
        MassPart::Alleluia => "Alleluia - acclamation before the Gospel",
        MassPart::Gospel => "Gospel - reading from the Gospels",
        MassPart::PrayerOverTheOfferings => {
            "Prayer over the Offerings - prayer during the offertory"
        }
        MassPart::Preface => "Preface - introduction to the Eucharistic Prayer",
        MassPart::CommunionAntiphon => "Communion Antiphon - chant during communion",
        MassPart::PrayerAfterCommunion => "Prayer after Communion - concluding prayer",
        MassPart::SolemnBlessing => "Solemn Blessing - special blessing on certain occasions",
        MassPart::PrayerOverThePeople => "Prayer over the People - blessing over the congregation",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    #[test]
    fn test_mass_part_iteration_order() {
        let variants: Vec<MassPart> = MassPart::iter().collect();

        // Verify that the order is exactly the declaration order
        assert_eq!(variants[0], MassPart::MessianicEntry);
        assert_eq!(variants[1], MassPart::EntranceAntiphon);
        assert_eq!(variants[2], MassPart::Collect);
        assert_eq!(variants[3], MassPart::Reading1);
        assert_eq!(variants[4], MassPart::Psalm);
        assert_eq!(variants[5], MassPart::Canticle);
        assert_eq!(variants[6], MassPart::Reading2);
        assert_eq!(variants[7], MassPart::EasterVigilPsalm2);
        assert_eq!(variants[8], MassPart::EasterVigilReading3);
        assert_eq!(variants[9], MassPart::EasterVigilCanticle3);
        assert_eq!(variants[10], MassPart::EasterVigilReading4);
        assert_eq!(variants[11], MassPart::EasterVigilPsalm4);
        assert_eq!(variants[12], MassPart::EasterVigilReading5);
        assert_eq!(variants[13], MassPart::EasterVigilCanticle5);
        assert_eq!(variants[14], MassPart::EasterVigilReading6);
        assert_eq!(variants[15], MassPart::EasterVigilPsalm6);
        assert_eq!(variants[16], MassPart::EasterVigilReading7);
        assert_eq!(variants[17], MassPart::EasterVigilPsalm7);
        assert_eq!(variants[18], MassPart::EasterVigilEpistle);
        assert_eq!(variants[19], MassPart::Sequence);
        assert_eq!(variants[20], MassPart::Alleluia);
        assert_eq!(variants[21], MassPart::Gospel);
        assert_eq!(variants[22], MassPart::PrayerOverTheOfferings);
        assert_eq!(variants[23], MassPart::Preface);
        assert_eq!(variants[24], MassPart::CommunionAntiphon);
        assert_eq!(variants[25], MassPart::PrayerAfterCommunion);
        assert_eq!(variants[26], MassPart::SolemnBlessing);
        assert_eq!(variants[27], MassPart::PrayerOverThePeople);

        // Verify that we have all variants
        assert_eq!(variants.len(), 28);
    }

    #[test]
    fn test_mass_part_iteration_consistency() {
        // Verify that the order is always the same across multiple iterations
        let first_iteration: Vec<MassPart> = MassPart::iter().collect();
        let second_iteration: Vec<MassPart> = MassPart::iter().collect();

        assert_eq!(first_iteration, second_iteration);
    }

    #[test]
    fn test_mass_part_serialization() {
        // Verify that serialization works
        let mass_part = MassPart::Gospel;
        let json = serde_json::to_string(&mass_part).unwrap();
        assert_eq!(json, "\"gospel\"");

        let deserialized: MassPart = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, MassPart::Gospel);
    }

    #[test]
    fn test_mass_part_categories() {
        // Test that the category methods still work correctly
        assert!(MassPart::Gospel.is_reading_part());
        assert!(MassPart::EntranceAntiphon.is_antiphon_part());
        assert!(MassPart::Collect.is_prayer_part());
        assert!(MassPart::Psalm.is_psalm_part());

        // Test that non-matching parts return false
        assert!(!MassPart::EntranceAntiphon.is_reading_part());
        assert!(!MassPart::Gospel.is_antiphon_part());
        assert!(!MassPart::Psalm.is_prayer_part());
        assert!(!MassPart::Collect.is_psalm_part());
    }
}
