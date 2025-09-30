use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Parts that make up the Mass celebration.
/// Each part represents a specific element of the liturgical celebration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MassPart {
    /// Messianic Entry - entrance chant for special occasions
    MessianicEntry,
    /// Entrance Antiphon - opening chant of the Mass
    EntranceAntiphon,
    /// Collect - opening prayer of the Mass
    Collect,
    /// Reading 1 - first reading from the Old Testament
    Reading1,
    /// Psalm - responsorial psalm
    Psalm,
    /// Canticle - biblical canticle
    Canticle,
    /// Reading 2 - second reading (usually from the New Testament)
    Reading2,
    /// Reading 3 - third reading (when applicable)
    Reading3,
    /// Reading 4 - fourth reading (when applicable)
    Reading4,
    /// Reading 5 - fifth reading (when applicable)
    Reading5,
    /// Reading 6 - sixth reading (when applicable)
    Reading6,
    /// Reading 7 - seventh reading (when applicable)
    Reading7,
    /// Epistle - reading from the epistles
    Epistle,
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
            MassPart::Reading3,
            MassPart::Reading4,
            MassPart::Reading5,
            MassPart::Reading6,
            MassPart::Reading7,
            MassPart::Epistle,
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
        &[MassPart::Psalm, MassPart::Canticle]
    }

    /// Check if a mass part is a psalm part.
    pub fn is_psalm_part(&self) -> bool {
        Self::psalm_parts().contains(self)
    }
}
