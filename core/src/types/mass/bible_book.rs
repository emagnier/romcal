#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::EnumIter;

/// Books of the Bible using OSIS (Open Scripture Information Standard) identifiers.
/// OSIS provides standardized abbreviations for biblical books used in liturgical and biblical applications.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
pub enum BibleBook {
    // Old Testament (47 books)

    // - Pentateuch (5 books)
    /// Genesis
    #[serde(rename = "Gen")]
    Gen,
    /// Exodus
    #[serde(rename = "Exod")]
    Exod,
    /// Leviticus
    #[serde(rename = "Lev")]
    Lev,
    /// Numbers
    #[serde(rename = "Num")]
    Num,
    /// Deuteronomy
    #[serde(rename = "Deut")]
    Deut,

    // - Historical Books (16 books)
    /// Joshua
    #[serde(rename = "Josh")]
    Josh,
    /// Judges
    #[serde(rename = "Judg")]
    Judg,
    /// Ruth
    #[serde(rename = "Ruth")]
    Ruth,
    /// 1 Samuel
    #[serde(rename = "1Sam")]
    FirstSam,
    /// 2 Samuel
    #[serde(rename = "2Sam")]
    SecondSam,
    /// 1 Kings
    #[serde(rename = "1Kgs")]
    FirstKgs,
    /// 2 Kings
    #[serde(rename = "2Kgs")]
    SecondKgs,
    /// 1 Chronicles
    #[serde(rename = "1Chr")]
    FirstChr,
    /// 2 Chronicles
    #[serde(rename = "2Chr")]
    SecondChr,
    /// Ezra
    #[serde(rename = "Ezra")]
    Ezra,
    /// Nehemiah
    #[serde(rename = "Neh")]
    Neh,
    /// Tobit
    #[serde(rename = "Tob")]
    Tob,
    /// Judith
    #[serde(rename = "Jdt")]
    Jdt,
    /// Esther
    #[serde(rename = "Esth")]
    Esth,
    /// 1 Maccabees
    #[serde(rename = "1Macc")]
    FirstMacc,
    /// 2 Maccabees
    #[serde(rename = "2Macc")]
    SecondMacc,

    // - Poetic and Wisdom Books (7 books)
    /// Job
    #[serde(rename = "Job")]
    Job,
    /// Psalms
    #[serde(rename = "Ps")]
    Ps,
    /// Proverbs
    #[serde(rename = "Prov")]
    Prov,
    /// Ecclesiastes (Qohelet)
    #[serde(rename = "Eccl")]
    Eccl,
    /// Song of Solomon (Canticle of Canticles)
    #[serde(rename = "Song")]
    Song,
    /// Wisdom of Solomon
    #[serde(rename = "Wis")]
    Wis,
    /// Sirach (Ecclesiasticus)
    #[serde(rename = "Sir")]
    Sir,

    // - Prophetic Books (19 books)
    /// Isaiah
    #[serde(rename = "Isa")]
    Isa,
    /// Jeremiah
    #[serde(rename = "Jer")]
    Jer,
    /// Lamentations
    #[serde(rename = "Lam")]
    Lam,
    /// Baruch
    #[serde(rename = "Bar")]
    Bar,
    /// Letter of Jeremiah
    #[serde(rename = "EpJer")]
    EpJer,
    /// Ezekiel
    #[serde(rename = "Ezek")]
    Ezek,
    /// Daniel
    #[serde(rename = "Dan")]
    Dan,
    /// Hosea
    #[serde(rename = "Hos")]
    Hos,
    /// Joel
    #[serde(rename = "Joel")]
    Joel,
    /// Amos
    #[serde(rename = "Amos")]
    Amos,
    /// Obadiah
    #[serde(rename = "Obad")]
    Obad,
    /// Jonah
    #[serde(rename = "Jonah")]
    Jonah,
    /// Micah
    #[serde(rename = "Mic")]
    Mic,
    /// Nahum
    #[serde(rename = "Nah")]
    Nah,
    /// Habakkuk
    #[serde(rename = "Hab")]
    Hab,
    /// Zephaniah
    #[serde(rename = "Zeph")]
    Zeph,
    /// Haggai
    #[serde(rename = "Hag")]
    Hag,
    /// Zechariah
    #[serde(rename = "Zech")]
    Zech,
    /// Malachi
    #[serde(rename = "Mal")]
    Mal,

    // New Testament (27 books)

    // - Gospels (4 books)
    /// Matthew
    #[serde(rename = "Matt")]
    Matt,
    /// Mark
    #[serde(rename = "Mark")]
    Mark,
    /// Luke
    #[serde(rename = "Luke")]
    Luke,
    /// John
    #[serde(rename = "John")]
    John,

    /// Acts
    #[serde(rename = "Acts")]
    Acts,

    // - Pauline Letters (14 books)
    /// Romans
    #[serde(rename = "Rom")]
    Rom,
    /// 1 Corinthians
    #[serde(rename = "1Cor")]
    FirstCor,
    /// 2 Corinthians
    #[serde(rename = "2Cor")]
    SecondCor,
    /// Galatians
    #[serde(rename = "Gal")]
    Gal,
    /// Ephesians
    #[serde(rename = "Eph")]
    Eph,
    /// Philippians
    #[serde(rename = "Phil")]
    Phil,
    /// Colossians
    #[serde(rename = "Col")]
    Col,
    /// 1 Thessalonians
    #[serde(rename = "1Thess")]
    FirstThess,
    /// 2 Thessalonians
    #[serde(rename = "2Thess")]
    SecondThess,
    /// 1 Timothy
    #[serde(rename = "1Tim")]
    FirstTim,
    /// 2 Timothy
    #[serde(rename = "2Tim")]
    SecondTim,
    /// Titus
    #[serde(rename = "Titus")]
    Titus,
    /// Philemon
    #[serde(rename = "Phlm")]
    Phlm,
    /// Hebrews
    #[serde(rename = "Heb")]
    Heb,

    // - Catholic Letters (7 books)
    /// James
    #[serde(rename = "Jas")]
    Jas,
    /// 1 Peter
    #[serde(rename = "1Pet")]
    FirstPet,
    /// 2 Peter
    #[serde(rename = "2Pet")]
    SecondPet,
    /// 1 John
    #[serde(rename = "1John")]
    FirstJohn,
    /// 2 John
    #[serde(rename = "2John")]
    SecondJohn,
    /// 3 John
    #[serde(rename = "3John")]
    ThirdJohn,
    /// Jude
    #[serde(rename = "Jude")]
    Jude,

    /// Revelation
    #[serde(rename = "Rev")]
    Rev,
}

impl fmt::Display for BibleBook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Use serde serialization to get the osisID
        let json = serde_json::to_string(self).unwrap();
        let osis_id = json.trim_matches('"');
        write!(f, "{}", osis_id)
    }
}

impl BibleBook {
    /// Check if this book is from the Old Testament.
    pub fn is_old_testament(&self) -> bool {
        matches!(
            self,
            BibleBook::Gen
                | BibleBook::Exod
                | BibleBook::Lev
                | BibleBook::Num
                | BibleBook::Deut
                | BibleBook::Josh
                | BibleBook::Judg
                | BibleBook::Ruth
                | BibleBook::FirstSam
                | BibleBook::SecondSam
                | BibleBook::FirstKgs
                | BibleBook::SecondKgs
                | BibleBook::FirstChr
                | BibleBook::SecondChr
                | BibleBook::Ezra
                | BibleBook::Neh
                | BibleBook::Tob
                | BibleBook::Jdt
                | BibleBook::Esth
                | BibleBook::FirstMacc
                | BibleBook::SecondMacc
                | BibleBook::Job
                | BibleBook::Ps
                | BibleBook::Prov
                | BibleBook::Eccl
                | BibleBook::Song
                | BibleBook::Wis
                | BibleBook::Sir
                | BibleBook::Isa
                | BibleBook::Jer
                | BibleBook::Lam
                | BibleBook::Ezek
                | BibleBook::Bar
                | BibleBook::EpJer
                | BibleBook::Dan
                | BibleBook::Hos
                | BibleBook::Joel
                | BibleBook::Amos
                | BibleBook::Obad
                | BibleBook::Jonah
                | BibleBook::Mic
                | BibleBook::Nah
                | BibleBook::Hab
                | BibleBook::Zeph
                | BibleBook::Hag
                | BibleBook::Zech
                | BibleBook::Mal
        )
    }

    /// Check if this book is from the New Testament.
    pub fn is_new_testament(&self) -> bool {
        !self.is_old_testament()
    }

    /// Get all Old Testament books.
    pub fn old_testament_books() -> &'static [BibleBook] {
        &[
            BibleBook::Gen,
            BibleBook::Exod,
            BibleBook::Lev,
            BibleBook::Num,
            BibleBook::Deut,
            BibleBook::Josh,
            BibleBook::Judg,
            BibleBook::Ruth,
            BibleBook::FirstSam,
            BibleBook::SecondSam,
            BibleBook::FirstKgs,
            BibleBook::SecondKgs,
            BibleBook::FirstChr,
            BibleBook::SecondChr,
            BibleBook::Ezra,
            BibleBook::Neh,
            BibleBook::Tob,
            BibleBook::Jdt,
            BibleBook::Esth,
            BibleBook::FirstMacc,
            BibleBook::SecondMacc,
            BibleBook::Job,
            BibleBook::Ps,
            BibleBook::Prov,
            BibleBook::Eccl,
            BibleBook::Song,
            BibleBook::Wis,
            BibleBook::Sir,
            BibleBook::Isa,
            BibleBook::Jer,
            BibleBook::Lam,
            BibleBook::Ezek,
            BibleBook::Bar,
            BibleBook::EpJer,
            BibleBook::Dan,
            BibleBook::Hos,
            BibleBook::Joel,
            BibleBook::Amos,
            BibleBook::Obad,
            BibleBook::Jonah,
            BibleBook::Mic,
            BibleBook::Nah,
            BibleBook::Hab,
            BibleBook::Zeph,
            BibleBook::Hag,
            BibleBook::Zech,
            BibleBook::Mal,
        ]
    }

    /// Get all New Testament books.
    pub fn new_testament_books() -> &'static [BibleBook] {
        &[
            BibleBook::Matt,
            BibleBook::Mark,
            BibleBook::Luke,
            BibleBook::John,
            BibleBook::Acts,
            BibleBook::Rom,
            BibleBook::FirstCor,
            BibleBook::SecondCor,
            BibleBook::Gal,
            BibleBook::Eph,
            BibleBook::Phil,
            BibleBook::Col,
            BibleBook::FirstThess,
            BibleBook::SecondThess,
            BibleBook::FirstTim,
            BibleBook::SecondTim,
            BibleBook::Titus,
            BibleBook::Phlm,
            BibleBook::Heb,
            BibleBook::Jas,
            BibleBook::FirstPet,
            BibleBook::SecondPet,
            BibleBook::FirstJohn,
            BibleBook::SecondJohn,
            BibleBook::ThirdJohn,
            BibleBook::Jude,
            BibleBook::Rev,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_trait() {
        // Test Display trait
        assert_eq!(format!("{}", BibleBook::Ps), "Ps");
        assert_eq!(format!("{}", BibleBook::FirstCor), "1Cor");
        assert_eq!(format!("{}", BibleBook::Matt), "Matt");

        // Test enriched BCV format
        let reference = format!("{} 1,1", BibleBook::Ps);
        assert_eq!(reference, "Ps 1,1");
    }

    #[test]
    fn test_json_serialization() {
        // Test JSON serialization
        let json_psalms = serde_json::to_string(&BibleBook::Ps).unwrap();
        let json_first_cor = serde_json::to_string(&BibleBook::FirstCor).unwrap();

        assert_eq!(json_psalms, "\"Ps\"");
        assert_eq!(json_first_cor, "\"1Cor\"");
    }

    #[test]
    fn test_old_new_testament() {
        assert!(BibleBook::Ps.is_old_testament());
        assert!(BibleBook::Matt.is_new_testament());
        assert!(!BibleBook::Ps.is_new_testament());
        assert!(!BibleBook::Matt.is_old_testament());
    }
}
