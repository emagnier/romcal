use crate::types::{MassPart, MassTime};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Liturgical cycle for lectionary readings
/// Includes both actual cycles (Year A, B, C, etc.) and invariant content
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, PartialOrd, Ord,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LiturgicalCycle {
    /// Invariant content that applies to all cycles
    Invariant,
    /// Year A of the Sunday cycle
    YearA,
    /// Year B of the Sunday cycle
    YearB,
    /// Year C of the Sunday cycle
    YearC,
    /// Combined years A and B of the Sunday cycle
    YearAB,
    /// Combined years A and C of the Sunday cycle
    YearAC,
    /// Combined years B and C of the Sunday cycle
    YearBC,
    /// Year 1 of the weekday cycle (Cycle I)
    Year1,
    /// Year 2 of the weekday cycle (Cycle II)
    Year2,
}

/// Content of a mass for a specific liturgical cycle
/// Maps mass parts (readings, psalms, prayers, antiphons, etc.) to their texts
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct MassContent(BTreeMap<MassPart, String>);

impl MassContent {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(&mut self, key: MassPart, value: String) {
        self.0.insert(key, value);
    }

    pub fn get(&self, key: &MassPart) -> Option<&String> {
        self.0.get(key)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn contains_key(&self, key: &MassPart) -> bool {
        self.0.contains_key(key)
    }
}

impl Default for MassContent {
    fn default() -> Self {
        Self::new()
    }
}

/// Mass contents for a specific mass time, organized by liturgical cycle
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct MassCycleDefinition(BTreeMap<LiturgicalCycle, MassContent>);

impl MassCycleDefinition {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(&mut self, key: LiturgicalCycle, value: MassContent) {
        self.0.insert(key, value);
    }

    pub fn get(&self, key: &LiturgicalCycle) -> Option<&MassContent> {
        self.0.get(key)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn contains_key(&self, key: &LiturgicalCycle) -> bool {
        self.0.contains_key(key)
    }
}

impl Default for MassCycleDefinition {
    fn default() -> Self {
        Self::new()
    }
}

/// All mass definitions for a liturgical day
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct MassesDefinitions(BTreeMap<MassTime, MassCycleDefinition>);

impl MassesDefinitions {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn insert(&mut self, key: MassTime, value: MassCycleDefinition) {
        self.0.insert(key, value);
    }

    pub fn get(&self, key: &MassTime) -> Option<&MassCycleDefinition> {
        self.0.get(key)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn contains_key(&self, key: &MassTime) -> bool {
        self.0.contains_key(key)
    }
}

impl Default for MassesDefinitions {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_liturgical_cycle_serialization() {
        // Test invariant cycle
        let invariant = LiturgicalCycle::Invariant;
        let json = serde_json::to_string(&invariant).unwrap();
        assert_eq!(json, "\"INVARIANT\"");

        let deserialized: LiturgicalCycle = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, LiturgicalCycle::Invariant);

        // Test year A cycle
        let year_a = LiturgicalCycle::YearA;
        let json = serde_json::to_string(&year_a).unwrap();
        assert_eq!(json, "\"YEAR_A\"");

        let deserialized: LiturgicalCycle = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, LiturgicalCycle::YearA);
    }

    #[test]
    fn test_mass_content_serialization() {
        let mut content = MassContent::new();
        content.insert(MassPart::Reading1, "Is 2:1-5".to_string());
        content.insert(MassPart::Gospel, "Mt 24:37-44".to_string());

        let json = serde_json::to_string_pretty(&content).unwrap();
        println!("MassContent JSON: {}", json);
        assert!(json.contains("\"READING_1\""));
        assert!(json.contains("\"GOSPEL\""));

        let deserialized: MassContent = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.len(), 2);
        assert_eq!(
            deserialized.get(&MassPart::Reading1),
            Some(&"Is 2:1-5".to_string())
        );
    }

    #[test]
    fn test_mass_lectionary_serialization() {
        let mut lectionary = MassCycleDefinition::new();

        // Add invariant content
        let mut invariant = MassContent::new();
        invariant.insert(MassPart::Alleluia, "Ps 88:8".to_string());
        lectionary.insert(LiturgicalCycle::Invariant, invariant);

        // Add year A content
        let mut year_a = MassContent::new();
        year_a.insert(MassPart::Reading1, "Is 2:1-5".to_string());
        year_a.insert(MassPart::Gospel, "Mt 24:37-44".to_string());
        lectionary.insert(LiturgicalCycle::YearA, year_a);

        let json = serde_json::to_string_pretty(&lectionary).unwrap();
        assert!(json.contains("\"INVARIANT\""));
        assert!(json.contains("\"YEAR_A\""));
        assert!(json.contains("\"ALLELUIA\""));
        assert!(json.contains("\"READING_1\""));

        let deserialized: MassCycleDefinition = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.len(), 2);
        assert!(deserialized.contains_key(&LiturgicalCycle::Invariant));
        assert!(deserialized.contains_key(&LiturgicalCycle::YearA));
    }

    #[test]
    fn test_mass_readings_serialization() {
        let mut masses = MassesDefinitions::new();

        // Add day mass
        let mut day_mass = MassCycleDefinition::new();
        let mut invariant = MassContent::new();
        invariant.insert(MassPart::Alleluia, "Ps 88:8".to_string());
        day_mass.insert(LiturgicalCycle::Invariant, invariant);
        masses.insert(MassTime::DayMass, day_mass);

        let json = serde_json::to_string_pretty(&masses).unwrap();
        assert!(json.contains("\"DAY_MASS\""));
        assert!(json.contains("\"INVARIANT\""));
        assert!(json.contains("\"ALLELUIA\""));

        let deserialized: MassesDefinitions = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.len(), 1);
        assert!(deserialized.contains_key(&MassTime::DayMass));
    }

    #[test]
    fn test_mass_readings_example() {
        // Create a realistic example of mass readings for a liturgical day
        let mut masses = MassesDefinitions::new();

        // Day Mass with invariant and cycle-specific readings
        let mut day_mass = MassCycleDefinition::new();

        // Invariant readings (apply to all cycles)
        let mut invariant = MassContent::new();
        invariant.insert(MassPart::Alleluia, "Ps 88:8".to_string());
        day_mass.insert(LiturgicalCycle::Invariant, invariant);

        // Year A readings
        let mut year_a = MassContent::new();
        year_a.insert(MassPart::Reading1, "Is 2:1-5".to_string());
        year_a.insert(
            MassPart::Psalm,
            "Ps 121:1-2,3-4ab,4cd-5,6-7,8-9".to_string(),
        );
        year_a.insert(MassPart::Reading2, "Rom 13:11-14a".to_string());
        year_a.insert(MassPart::Gospel, "Mt 24:37-44".to_string());
        day_mass.insert(LiturgicalCycle::YearA, year_a);

        // Year B readings
        let mut year_b = MassContent::new();
        year_b.insert(MassPart::Reading1, "Is 40:1-5".to_string());
        year_b.insert(MassPart::Psalm, "Ps 85:9-14".to_string());
        year_b.insert(MassPart::Reading2, "2 Pet 3:8-14".to_string());
        year_b.insert(MassPart::Gospel, "Mk 13:33-37".to_string());
        day_mass.insert(LiturgicalCycle::YearB, year_b);

        masses.insert(MassTime::DayMass, day_mass);

        // Evening Mass with only invariant readings
        let mut evening_mass = MassCycleDefinition::new();
        let mut evening_invariant = MassContent::new();
        evening_invariant.insert(MassPart::Reading1, "Gen 1:1-5".to_string());
        evening_invariant.insert(MassPart::Psalm, "Ps 104:1-2".to_string());
        evening_invariant.insert(MassPart::Gospel, "Jn 1:1-5".to_string());
        evening_mass.insert(LiturgicalCycle::Invariant, evening_invariant);
        masses.insert(MassTime::EveningMass, evening_mass);

        // Serialize to JSON
        let json = serde_json::to_string_pretty(&masses).unwrap();
        println!("Example JSON structure:");
        println!("{}", json);

        // Verify JSON structure
        assert!(json.contains("\"DAY_MASS\""));
        assert!(json.contains("\"EVENING_MASS\""));
        assert!(json.contains("\"INVARIANT\""));
        assert!(json.contains("\"YEAR_A\""));
        assert!(json.contains("\"YEAR_B\""));
        assert!(json.contains("\"ALLELUIA\""));
        assert!(json.contains("\"READING_1\""));
        assert!(json.contains("\"PSALM\""));
        assert!(json.contains("\"GOSPEL\""));

        // Deserialize back
        let deserialized: MassesDefinitions = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.len(), 2);
        assert!(deserialized.contains_key(&MassTime::DayMass));
        assert!(deserialized.contains_key(&MassTime::EveningMass));

        // Verify day mass structure
        let day_mass_content = deserialized.get(&MassTime::DayMass).unwrap();
        assert_eq!(day_mass_content.len(), 3); // invariant, year_a, year_b
        assert!(day_mass_content.contains_key(&LiturgicalCycle::Invariant));
        assert!(day_mass_content.contains_key(&LiturgicalCycle::YearA));
        assert!(day_mass_content.contains_key(&LiturgicalCycle::YearB));

        // Verify invariant content
        let invariant_content = day_mass_content.get(&LiturgicalCycle::Invariant).unwrap();
        assert_eq!(invariant_content.len(), 1);
        assert_eq!(
            invariant_content.get(&MassPart::Alleluia),
            Some(&"Ps 88:8".to_string())
        );

        // Verify year A content
        let year_a_content = day_mass_content.get(&LiturgicalCycle::YearA).unwrap();
        assert_eq!(year_a_content.len(), 4);
        assert_eq!(
            year_a_content.get(&MassPart::Reading1),
            Some(&"Is 2:1-5".to_string())
        );
        assert_eq!(
            year_a_content.get(&MassPart::Gospel),
            Some(&"Mt 24:37-44".to_string())
        );
    }

    #[test]
    fn test_liturgical_cycle_ordering() {
        // Test that cycles can be used as BTreeMap keys
        let mut map = BTreeMap::new();
        map.insert(LiturgicalCycle::YearB, "B".to_string());
        map.insert(LiturgicalCycle::YearA, "A".to_string());
        map.insert(LiturgicalCycle::Invariant, "Invariant".to_string());

        // Keys should be ordered: Invariant, YearA, YearB
        let keys: Vec<_> = map.keys().collect();
        assert_eq!(keys[0], &LiturgicalCycle::Invariant);
        assert_eq!(keys[1], &LiturgicalCycle::YearA);
        assert_eq!(keys[2], &LiturgicalCycle::YearB);
    }
}
