use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Re-export existing types from calendar_def
use crate::calendar_def::Title;
use crate::types::saint_count::SaintCount;

// Type aliases
pub type LocaleId = String;
pub type EntityId = String;

// Enums
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EntityType {
    #[default]
    Person,
    Place,
    Event,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CanonizationLevel {
    Blessed,
    Saint,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Sex {
    Male,
    Female,
}

// Union types using enums
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum SaintDate {
    Year(u32),
    YearMonth(String),    // Format: "YYYY-MM"
    YearMonthDay(String), // Format: "YYYY-MM-DD"
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum SaintDateDef {
    Date(SaintDate),
    Between { between: [SaintDate; 2] },
    Or { or: Vec<SaintDate> },
    Century { century: u32 },
}

// Structs
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ResourcesDefinition {
    #[serde(rename = "$schema")]
    pub schema: Option<String>,

    /// Locale code of the resources, in BCP-47 IETF tag format
    pub locale: LocaleId,

    /// Metadata of the resources
    pub metadata: Option<ResourcesMetadata>,

    /// Entities of the resources: a person, a place, an event, etc.
    pub entities: Option<Vec<EntityDefinition>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ResourcesMetadata {
    pub ordinals: Option<HashMap<String, String>>,
    pub weekdays: Option<HashMap<String, String>>,
    pub months: Option<HashMap<String, String>>,
    pub colors: Option<LocaleColors>,
    pub seasons: Option<SeasonsMetadata>,
    pub periods: Option<PeriodsMetadata>,
    pub ranks: Option<RanksMetadata>,
    pub cycles: Option<CyclesMetadata>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct LocaleColors {
    pub black: Option<String>,
    pub gold: Option<String>,
    pub green: Option<String>,
    pub purple: Option<String>,
    pub red: Option<String>,
    pub rose: Option<String>,
    pub white: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SeasonsMetadata {
    pub advent: Option<AdventSeason>,
    pub christmas_time: Option<ChristmasTimeSeason>,
    pub ordinary_time: Option<OrdinaryTimeSeason>,
    pub lent: Option<LentSeason>,
    pub paschal_triduum: Option<PaschalTriduumSeason>,
    pub easter_time: Option<EasterTimeSeason>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct AdventSeason {
    pub season: Option<String>,
    pub weekday: Option<String>,
    pub sunday: Option<String>,
    pub privileged_weekday: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ChristmasTimeSeason {
    pub season: Option<String>,
    pub day: Option<String>,
    pub octave: Option<String>,
    pub before_epiphany: Option<String>,
    pub second_sunday_after_christmas: Option<String>,
    pub after_epiphany: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct OrdinaryTimeSeason {
    pub season: Option<String>,
    pub weekday: Option<String>,
    pub sunday: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct LentSeason {
    pub season: Option<String>,
    pub weekday: Option<String>,
    pub sunday: Option<String>,
    pub day_after_ash_wed: Option<String>,
    pub holy_week_day: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PaschalTriduumSeason {
    pub season: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct EasterTimeSeason {
    pub season: Option<String>,
    pub weekday: Option<String>,
    pub sunday: Option<String>,
    pub octave: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PeriodsMetadata {
    pub epiphany: Option<String>,
    pub holy_week: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RanksMetadata {
    pub solemnity: Option<String>,
    pub sunday: Option<String>,
    pub feast: Option<String>,
    pub memorial: Option<String>,
    pub optional_memorial: Option<String>,
    pub weekday: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CyclesMetadata {
    pub proper_of_time: Option<String>,
    pub proper_of_saints: Option<String>,
    pub sunday_year_a: Option<String>,
    pub sunday_year_b: Option<String>,
    pub sunday_year_c: Option<String>,
    pub weekday_year_1: Option<String>,
    pub weekday_year_2: Option<String>,
    pub psalter_week_1: Option<String>,
    pub psalter_week_2: Option<String>,
    pub psalter_week_3: Option<String>,
    pub psalter_week_4: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct EntityDefinition {
    /// The unique identifier of the entity
    pub id: EntityId,

    /// The type of the entity.
    /// @default EntityType.Person
    pub r#type: Option<EntityType>,

    /// The full name of the entity.
    pub fullname: Option<String>,

    /// The short name of the entity, without the canonization level and titles.
    pub name: Option<String>,

    /// The canonization level of a person.
    pub canonization_level: Option<CanonizationLevel>,

    /// Date of Canonization, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
    /// or an object describing date range, multiple possible date, or a century.
    pub date_of_canonization: Option<SaintDateDef>,

    /// Specify whether an approximate indicator should be added, when the date is displayed.
    /// For example in English: 'c. 201'.
    pub date_of_canonization_is_approximative: Option<bool>,

    /// Date of Beatification, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
    /// or an object describing date range, multiple possible date, or a century.
    pub date_of_beatification: Option<SaintDateDef>,

    /// Specify whether an approximate indicator should be added, when the date is displayed.
    /// For example in English: 'c. 201'.
    pub date_of_beatification_is_approximative: Option<bool>,

    /// Specify if the canonization level should not be displayed.
    /// It's generally the case when the canonization are already included in the name.
    pub hide_canonization_level: Option<bool>,

    /// Titles of the Saint or the Blessed
    pub titles: Option<Vec<Title>>,

    /// Determine if the Saint or the Blessed is a male or a female.
    pub sex: Option<Sex>,

    /// Specify if the titles should not be displayed.
    /// It's generally the case when titles are already included in the name.
    pub hide_titles: Option<bool>,

    /// Date of Dedication of a church, basilica, or cathedral (or other place of worship),
    /// as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
    /// or an object describing date range, multiple possible date, or a century.
    pub date_of_dedication: Option<SaintDateDef>,

    /// Date of Birth, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
    /// or an object describing date range, multiple possible date, or a century.
    pub date_of_birth: Option<SaintDateDef>,

    /// Specify whether an approximate indicator should be added, when the date is displayed.
    /// For example in English: 'c. 201'.
    pub date_of_birth_is_approximative: Option<bool>,

    /// Date of Death, as a Number (year), a String (in 'YYYY-MM' or 'YYYY-MM-DD' format),
    /// or an object describing date range, multiple possible date, or a century.
    pub date_of_death: Option<SaintDateDef>,

    /// Specify whether an approximate indicator should be added, when the date is displayed.
    /// For example in English: 'c. 201'.
    pub date_of_death_is_approximative: Option<bool>,

    /// Number of person that this definition represent.
    /// It could be set as 'many' if the number is not defined.
    pub count: Option<SaintCount>,

    /// Sources for the information about this entity
    pub sources: Option<Vec<String>>,

    /// Internal notes
    /// @private
    pub _todo: Option<Vec<String>>,
}

// Implementations

impl EntityDefinition {
    /// Create a new EntityDefinition with the given ID and default values
    pub fn new(id: EntityId) -> Self {
        Self {
            id,
            r#type: Some(EntityType::Person),
            fullname: None,
            name: None,
            canonization_level: None,
            date_of_canonization: None,
            date_of_canonization_is_approximative: None,
            date_of_beatification: None,
            date_of_beatification_is_approximative: None,
            hide_canonization_level: None,
            titles: None,
            sex: None,
            hide_titles: None,
            date_of_dedication: None,
            date_of_birth: None,
            date_of_birth_is_approximative: None,
            date_of_death: None,
            date_of_death_is_approximative: None,
            count: None,
            sources: None,
            _todo: None,
        }
    }
}

impl ResourcesDefinition {
    /// Create a new ResourcesDefinition with the given locale
    pub fn new(locale: LocaleId) -> Self {
        Self {
            schema: None,
            locale,
            metadata: None,
            entities: None,
        }
    }

    /// Add an entity to the resources
    pub fn add_entity(&mut self, entity: EntityDefinition) {
        let entities = self.entities.get_or_insert_with(Vec::new);
        entities.push(entity);
    }

    /// Get an entity by its ID
    pub fn get_entity(&self, id: &str) -> Option<&EntityDefinition> {
        self.entities
            .as_ref()?
            .iter()
            .find(|entity| entity.id == id)
    }

    /// Get a mutable reference to an entity by its ID
    pub fn get_entity_mut(&mut self, id: &str) -> Option<&mut EntityDefinition> {
        self.entities
            .as_mut()?
            .iter_mut()
            .find(|entity| entity.id == id)
    }

    /// Remove an entity by its ID
    pub fn remove_entity(&mut self, id: &str) -> Option<EntityDefinition> {
        if let Some(entities) = &mut self.entities {
            if let Some(pos) = entities.iter().position(|entity| entity.id == id) {
                return Some(entities.remove(pos));
            }
        }
        None
    }

    /// Get all entity IDs
    pub fn get_entity_ids(&self) -> Vec<&String> {
        self.entities
            .as_ref()
            .map(|entities| entities.iter().map(|entity| &entity.id).collect())
            .unwrap_or_default()
    }

    /// Validate that all entities are properly structured
    /// Check for uniqueness of IDs and entity structure
    pub fn validate_entities(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        let mut seen_ids = std::collections::HashSet::new();

        if let Some(entities) = &self.entities {
            for entity in entities {
                // Check for duplicate IDs
                if seen_ids.contains(&entity.id) {
                    errors.push(format!("Duplicate entity ID: '{}'", entity.id));
                } else {
                    seen_ids.insert(&entity.id);
                }

                // Validate entity structure
                if entity.name.is_none() && entity.fullname.is_none() {
                    errors.push(format!(
                        "Entity '{}' must have either 'name' or 'fullname'",
                        entity.id
                    ));
                }

                // Validate canonization level consistency
                if let Some(level) = &entity.canonization_level {
                    if entity.hide_canonization_level == Some(true) && entity.fullname.is_some() {
                        // This is OK - canonization level is hidden because it's in the fullname
                    } else if entity.fullname.is_none() {
                        errors.push(format!(
                            "Entity '{}' has canonization level '{}' but no fullname to display it",
                            entity.id,
                            match level {
                                CanonizationLevel::Blessed => "BLESSED",
                                CanonizationLevel::Saint => "SAINT",
                            }
                        ));
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Merge entities from another ResourcesDefinition
    pub fn merge_entities(&mut self, other: &ResourcesDefinition) {
        if let Some(other_entities) = &other.entities {
            let entities = self.entities.get_or_insert_with(Vec::new);
            entities.extend(other_entities.clone());
        }
    }

    /// Get all entities as a reference to the vector
    pub fn get_entities(&self) -> Option<&Vec<EntityDefinition>> {
        self.entities.as_ref()
    }

    /// Get all entities as a mutable reference to the vector
    pub fn get_entities_mut(&mut self) -> Option<&mut Vec<EntityDefinition>> {
        self.entities.as_mut()
    }
}
