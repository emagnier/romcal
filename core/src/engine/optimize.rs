use serde_json::Value;
use std::collections::{HashMap, HashSet};

use crate::entity_resolution::locale::get_all_parent_locales;
use crate::types::entity::EntityDefinition;
use crate::{CalendarDefinition, Resources, Romcal, RomcalError, RomcalResult};

// Type aliases for clarity
type LocaleMap = HashMap<String, String>;
type EntityIdSet = HashSet<String>;
type PropertySet = HashSet<String>;

/// Create a JSON bundle of the current configuration
/// This method serializes the Romcal config to JSON format
/// and removes null values and empty objects from the output.
///
/// Only includes calendar_definitions that are:
/// 1. The main calendar (Romcal.calendar)
/// 2. Parent calendars of the main calendar
/// 3. The general_roman calendar
pub fn optimize(romcal: &Romcal) -> RomcalResult<String> {
    // Validate that all calendar IDs are unique
    validate_unique_calendar_ids(&romcal.calendar_definitions)?;

    // Validate that all resource locales are unique
    validate_unique_resource_locales(&romcal.resources)?;

    // Create a filtered version of the config with only relevant calendar_definitions and resources
    let mut filtered_config = romcal.clone();
    filtered_config.calendar_definitions = filter_calendar_definitions(romcal)?;
    filtered_config.resources = filter_resources(romcal, &filtered_config.calendar_definitions)?;

    // Reverse resources order for output: general → specific (e.g., [en, fr, fr-ca])
    // This provides a more intuitive reading order
    filtered_config.resources.reverse();

    let value = serde_json::to_value(&filtered_config)
        .map_err(|e| RomcalError::ValidationError(format!("JSON serialization error: {}", e)))?;
    let cleaned_value = remove_null_and_empty_values(value);
    serde_json::to_string_pretty(&cleaned_value)
        .map_err(|e| RomcalError::ValidationError(format!("JSON formatting error: {}", e)))
}

/// Validate that all calendar definitions have unique IDs
/// Returns an error if duplicate calendar IDs are found
fn validate_unique_calendar_ids(calendar_definitions: &[CalendarDefinition]) -> RomcalResult<()> {
    let mut seen_ids = EntityIdSet::new();

    for calendar_def in calendar_definitions {
        if !seen_ids.insert(calendar_def.id.clone()) {
            return Err(RomcalError::ValidationError(format!(
                "Duplicate calendar ID '{}' found in calendar_definitions. Each calendar must have a unique ID.",
                calendar_def.id
            )));
        }
    }

    Ok(())
}

/// Validate that all resource definitions have unique locales
/// Returns an error if duplicate locales are found
fn validate_unique_resource_locales(resources: &[Resources]) -> RomcalResult<()> {
    let mut seen_locales = EntityIdSet::new();

    for resource in resources {
        if !seen_locales.insert(resource.locale.clone()) {
            return Err(RomcalError::ValidationError(format!(
                "Duplicate locale '{}' found in resources. Each resource must have a unique locale.",
                resource.locale
            )));
        }
    }

    Ok(())
}

/// Filter resources to keep only the required locales based on the romcal config
/// Returns resources with hierarchical deduplication: most specific to most general
/// Entities defined in more specific locales are removed from parent locales
/// Only includes entities that are referenced in calendar day_definitions
fn filter_resources(
    romcal: &Romcal,
    filtered_calendars: &[CalendarDefinition],
) -> RomcalResult<Vec<Resources>> {
    let target_locale = &romcal.locale;

    // Build locale maps for efficient lookups
    let (available_locales, resources_by_locale) = build_locale_maps(romcal);

    // Validate target locale exists
    let exact_locale = validate_target_locale(target_locale, &available_locales)?;

    // Collect used entity IDs
    let used_entity_ids = collect_used_entity_ids(filtered_calendars);

    // Build priority list of locales (most specific to most general)
    let priority_locales = build_priority_locales(target_locale, &available_locales, &exact_locale);

    // Apply hierarchical deduplication with entity filtering
    apply_hierarchical_deduplication(priority_locales, &resources_by_locale, &used_entity_ids)
}

/// Collect all entity IDs that are referenced in calendar day_definitions
/// Includes both day_definition IDs and EntityRef references
fn collect_used_entity_ids(calendar_definitions: &[CalendarDefinition]) -> EntityIdSet {
    let mut used_entity_ids = EntityIdSet::new();

    for calendar_def in calendar_definitions {
        for (day_id, day_def) in &calendar_def.days_definitions {
            // Add the day_definition ID itself as a potential entity reference
            used_entity_ids.insert(day_id.clone());

            // Also check EntityRef elements in the day_definition
            if let Some(entities) = &day_def.entities {
                for entity_pointer in entities {
                    match entity_pointer {
                        crate::types::calendar::EntityRef::ResourceId(id) => {
                            used_entity_ids.insert(id.clone());
                        }
                        crate::types::calendar::EntityRef::Override(entity_override) => {
                            used_entity_ids.insert(entity_override.id.clone());
                        }
                    }
                }
            }
        }
    }

    used_entity_ids
}

/// Build locale maps for efficient lookups
fn build_locale_maps(romcal: &Romcal) -> (LocaleMap, HashMap<&str, &Resources>) {
    let available_locales: LocaleMap = romcal
        .resources
        .iter()
        .map(|resource| (resource.locale.to_lowercase(), resource.locale.clone()))
        .collect();

    let resources_by_locale: HashMap<&str, &Resources> = romcal
        .resources
        .iter()
        .map(|resource| (resource.locale.as_str(), resource))
        .collect();

    (available_locales, resources_by_locale)
}

/// Validate that the target locale exists in available resources
fn validate_target_locale(
    target_locale: &str,
    available_locales: &LocaleMap,
) -> RomcalResult<String> {
    let target_locale_lower = target_locale.to_lowercase();
    available_locales
        .get(&target_locale_lower)
        .cloned()
        .ok_or_else(|| {
            RomcalError::ValidationError(format!(
                "Target locale '{}' not found in resources. Available locales: {}",
                target_locale,
                available_locales
                    .values()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
        })
}

/// Build priority list of locales from most specific to most general
fn build_priority_locales(
    target_locale: &str,
    available_locales: &LocaleMap,
    exact_locale: &str,
) -> Vec<String> {
    let mut priority_locales = Vec::new();

    // 1. Add the exact target locale first (most specific)
    priority_locales.push(exact_locale.to_string());

    // 2. Add all parent locales in hierarchy order (most specific to most general)
    let parent_locales = get_all_parent_locales(target_locale);
    for parent in parent_locales {
        if parent != target_locale
            && let Some(parent_locale_actual) = available_locales.get(&parent.to_lowercase())
        {
            priority_locales.push(parent_locale_actual.clone());
        }
    }

    // 3. Always include "en" last (most general fallback)
    if let Some(en_locale) = available_locales.get("en")
        && !priority_locales.contains(en_locale)
    {
        priority_locales.push(en_locale.clone());
    }

    priority_locales
}

/// Apply hierarchical deduplication to resources with entity filtering.
/// Resources are processed from most specific to most general locale.
/// Property-level deduplication ensures parent locales only contain
/// properties that are missing in their child locales.
fn apply_hierarchical_deduplication(
    priority_locales: Vec<String>,
    resources_by_locale: &HashMap<&str, &Resources>,
    used_entity_ids: &EntityIdSet,
) -> RomcalResult<Vec<Resources>> {
    // Build filtered resources list (specific → general)
    let mut result: Vec<Resources> = priority_locales
        .iter()
        .filter_map(|locale| {
            resources_by_locale.get(locale.as_str()).map(|resource| {
                let mut filtered_resource = (*resource).clone();
                // Filter entities to only include those used in calendar day_definitions
                filter_entities_by_usage(&mut filtered_resource, used_entity_ids);
                filtered_resource
            })
        })
        .collect();

    // Apply property-level deduplication across all resources
    deduplicate_entity_properties(&mut result);
    deduplicate_metadata_properties(&mut result);

    // Remove entities that became empty after deduplication
    remove_empty_entities(&mut result);

    Ok(result)
}

/// Filter entities to only include those that are used in calendar day_definitions
fn filter_entities_by_usage(resource: &mut Resources, used_entity_ids: &EntityIdSet) {
    if let Some(entities) = &mut resource.entities {
        entities.retain(|id, _entity| used_entity_ids.contains(id));
    }
}

// ============================================================================
// Entity Property-Level Deduplication
// ============================================================================

/// Type alias for tracking defined properties per entity
type EntityPropertiesMap = HashMap<String, PropertySet>;

/// Deduplicate entity properties across locales (most specific to most general).
/// If a property exists in a more specific locale, remove it from parent locales.
/// Resources must be ordered from most specific to most general.
fn deduplicate_entity_properties(resources: &mut [Resources]) {
    // Track defined properties per entity: entity_id -> set of property names
    let mut defined_props: EntityPropertiesMap = HashMap::new();

    for resource in resources.iter_mut() {
        if let Some(entities) = &mut resource.entities {
            for (entity_id, entity_def) in entities.iter_mut() {
                let props = defined_props.entry(entity_id.clone()).or_default();
                deduplicate_single_entity(entity_def, props);
            }
        }
    }
}

/// Deduplicate properties of a single entity.
/// For each property: if already defined in a more specific locale, set to None;
/// otherwise if Some, mark as defined.
fn deduplicate_single_entity(entity: &mut EntityDefinition, defined: &mut PropertySet) {
    // Macro to deduplicate a single property
    macro_rules! dedup_prop {
        ($field:ident) => {
            if defined.contains(stringify!($field)) {
                entity.$field = None;
            } else if entity.$field.is_some() {
                defined.insert(stringify!($field).to_string());
            }
        };
    }

    dedup_prop!(r#type);
    dedup_prop!(fullname);
    dedup_prop!(name);
    dedup_prop!(canonization_level);
    dedup_prop!(date_of_canonization);
    dedup_prop!(date_of_canonization_is_approximative);
    dedup_prop!(date_of_beatification);
    dedup_prop!(date_of_beatification_is_approximative);
    dedup_prop!(hide_canonization_level);
    dedup_prop!(titles);
    dedup_prop!(sex);
    dedup_prop!(hide_titles);
    dedup_prop!(date_of_dedication);
    dedup_prop!(date_of_birth);
    dedup_prop!(date_of_birth_is_approximative);
    dedup_prop!(date_of_death);
    dedup_prop!(date_of_death_is_approximative);
    dedup_prop!(count);
    dedup_prop!(sources);
}

/// Check if an entity has all properties set to None (empty after deduplication)
fn is_entity_empty(entity: &EntityDefinition) -> bool {
    entity.r#type.is_none()
        && entity.fullname.is_none()
        && entity.name.is_none()
        && entity.canonization_level.is_none()
        && entity.date_of_canonization.is_none()
        && entity.date_of_canonization_is_approximative.is_none()
        && entity.date_of_beatification.is_none()
        && entity.date_of_beatification_is_approximative.is_none()
        && entity.hide_canonization_level.is_none()
        && entity.titles.is_none()
        && entity.sex.is_none()
        && entity.hide_titles.is_none()
        && entity.date_of_dedication.is_none()
        && entity.date_of_birth.is_none()
        && entity.date_of_birth_is_approximative.is_none()
        && entity.date_of_death.is_none()
        && entity.date_of_death_is_approximative.is_none()
        && entity.count.is_none()
        && entity.sources.is_none()
}

/// Remove entities where all properties are None after deduplication.
fn remove_empty_entities(resources: &mut [Resources]) {
    for resource in resources.iter_mut() {
        if let Some(entities) = &mut resource.entities {
            entities.retain(|_, entity| !is_entity_empty(entity));
        }
    }
}

// ============================================================================
// Metadata Property-Level Deduplication
// ============================================================================

use crate::types::resource::{
    AdventSeason, ChristmasTimeSeason, CyclesMetadata, EasterTimeSeason, LentSeason, LocaleColors,
    OrdinaryTimeSeason, PaschalTriduumSeason, PeriodsMetadata, RanksMetadata, ResourcesMetadata,
    SeasonsMetadata,
};

/// Deduplicate metadata properties across locales (most specific to most general).
/// Uses hierarchical property keys (e.g., "seasons.advent.season") for tracking.
fn deduplicate_metadata_properties(resources: &mut [Resources]) {
    let mut defined_props = PropertySet::new();

    for resource in resources.iter_mut() {
        if let Some(metadata) = &mut resource.metadata {
            deduplicate_single_metadata(metadata, &mut defined_props);
        }
    }
}

/// Deduplicate properties of a single metadata object.
fn deduplicate_single_metadata(metadata: &mut ResourcesMetadata, defined: &mut PropertySet) {
    // Macro for simple Option properties
    macro_rules! dedup_prop {
        ($field:ident, $key:expr) => {
            if defined.contains($key) {
                metadata.$field = None;
            } else if metadata.$field.is_some() {
                defined.insert($key.to_string());
            }
        };
    }

    dedup_prop!(ordinal_format, "ordinal_format");
    dedup_prop!(ordinals_letters, "ordinals_letters");
    dedup_prop!(ordinals_numeric, "ordinals_numeric");
    dedup_prop!(weekdays, "weekdays");
    dedup_prop!(months, "months");

    // Nested structures - deduplicate at property level
    deduplicate_colors(&mut metadata.colors, defined);
    deduplicate_seasons(&mut metadata.seasons, defined);
    deduplicate_periods(&mut metadata.periods, defined);
    deduplicate_ranks(&mut metadata.ranks, defined);
    deduplicate_cycles(&mut metadata.cycles, defined);
}

fn deduplicate_colors(colors: &mut Option<LocaleColors>, defined: &mut PropertySet) {
    if let Some(c) = colors {
        macro_rules! dedup {
            ($field:ident) => {
                let key = concat!("colors.", stringify!($field));
                if defined.contains(key) {
                    c.$field = None;
                } else if c.$field.is_some() {
                    defined.insert(key.to_string());
                }
            };
        }
        dedup!(black);
        dedup!(gold);
        dedup!(green);
        dedup!(purple);
        dedup!(red);
        dedup!(rose);
        dedup!(white);

        // Remove colors if empty
        if c.black.is_none()
            && c.gold.is_none()
            && c.green.is_none()
            && c.purple.is_none()
            && c.red.is_none()
            && c.rose.is_none()
            && c.white.is_none()
        {
            *colors = None;
        }
    }
}

fn deduplicate_seasons(seasons: &mut Option<SeasonsMetadata>, defined: &mut PropertySet) {
    if let Some(s) = seasons {
        deduplicate_advent(&mut s.advent, defined);
        deduplicate_christmas_time(&mut s.christmas_time, defined);
        deduplicate_ordinary_time(&mut s.ordinary_time, defined);
        deduplicate_lent(&mut s.lent, defined);
        deduplicate_paschal_triduum(&mut s.paschal_triduum, defined);
        deduplicate_easter_time(&mut s.easter_time, defined);

        // Remove seasons if empty
        if s.advent.is_none()
            && s.christmas_time.is_none()
            && s.ordinary_time.is_none()
            && s.lent.is_none()
            && s.paschal_triduum.is_none()
            && s.easter_time.is_none()
        {
            *seasons = None;
        }
    }
}

fn deduplicate_advent(advent: &mut Option<AdventSeason>, defined: &mut PropertySet) {
    if let Some(a) = advent {
        macro_rules! dedup {
            ($field:ident) => {
                let key = concat!("seasons.advent.", stringify!($field));
                if defined.contains(key) {
                    a.$field = None;
                } else if a.$field.is_some() {
                    defined.insert(key.to_string());
                }
            };
        }
        dedup!(season);
        dedup!(weekday);
        dedup!(sunday);
        dedup!(privileged_weekday);

        if a.season.is_none()
            && a.weekday.is_none()
            && a.sunday.is_none()
            && a.privileged_weekday.is_none()
        {
            *advent = None;
        }
    }
}

fn deduplicate_christmas_time(
    christmas: &mut Option<ChristmasTimeSeason>,
    defined: &mut PropertySet,
) {
    if let Some(c) = christmas {
        macro_rules! dedup {
            ($field:ident) => {
                let key = concat!("seasons.christmas_time.", stringify!($field));
                if defined.contains(key) {
                    c.$field = None;
                } else if c.$field.is_some() {
                    defined.insert(key.to_string());
                }
            };
        }
        dedup!(season);
        dedup!(day);
        dedup!(octave);
        dedup!(before_epiphany);
        dedup!(second_sunday_after_christmas);
        dedup!(after_epiphany);

        if c.season.is_none()
            && c.day.is_none()
            && c.octave.is_none()
            && c.before_epiphany.is_none()
            && c.second_sunday_after_christmas.is_none()
            && c.after_epiphany.is_none()
        {
            *christmas = None;
        }
    }
}

fn deduplicate_ordinary_time(ordinary: &mut Option<OrdinaryTimeSeason>, defined: &mut PropertySet) {
    if let Some(o) = ordinary {
        macro_rules! dedup {
            ($field:ident) => {
                let key = concat!("seasons.ordinary_time.", stringify!($field));
                if defined.contains(key) {
                    o.$field = None;
                } else if o.$field.is_some() {
                    defined.insert(key.to_string());
                }
            };
        }
        dedup!(season);
        dedup!(weekday);
        dedup!(sunday);

        if o.season.is_none() && o.weekday.is_none() && o.sunday.is_none() {
            *ordinary = None;
        }
    }
}

fn deduplicate_lent(lent: &mut Option<LentSeason>, defined: &mut PropertySet) {
    if let Some(l) = lent {
        macro_rules! dedup {
            ($field:ident) => {
                let key = concat!("seasons.lent.", stringify!($field));
                if defined.contains(key) {
                    l.$field = None;
                } else if l.$field.is_some() {
                    defined.insert(key.to_string());
                }
            };
        }
        dedup!(season);
        dedup!(weekday);
        dedup!(sunday);
        dedup!(day_after_ash_wed);
        dedup!(holy_week_day);

        if l.season.is_none()
            && l.weekday.is_none()
            && l.sunday.is_none()
            && l.day_after_ash_wed.is_none()
            && l.holy_week_day.is_none()
        {
            *lent = None;
        }
    }
}

fn deduplicate_paschal_triduum(
    triduum: &mut Option<PaschalTriduumSeason>,
    defined: &mut PropertySet,
) {
    if let Some(t) = triduum {
        let key = "seasons.paschal_triduum.season";
        if defined.contains(key) {
            t.season = None;
        } else if t.season.is_some() {
            defined.insert(key.to_string());
        }

        if t.season.is_none() {
            *triduum = None;
        }
    }
}

fn deduplicate_easter_time(easter: &mut Option<EasterTimeSeason>, defined: &mut PropertySet) {
    if let Some(e) = easter {
        macro_rules! dedup {
            ($field:ident) => {
                let key = concat!("seasons.easter_time.", stringify!($field));
                if defined.contains(key) {
                    e.$field = None;
                } else if e.$field.is_some() {
                    defined.insert(key.to_string());
                }
            };
        }
        dedup!(season);
        dedup!(weekday);
        dedup!(sunday);
        dedup!(octave);

        if e.season.is_none() && e.weekday.is_none() && e.sunday.is_none() && e.octave.is_none() {
            *easter = None;
        }
    }
}

fn deduplicate_periods(periods: &mut Option<PeriodsMetadata>, defined: &mut PropertySet) {
    if let Some(p) = periods {
        macro_rules! dedup {
            ($field:ident) => {
                let key = concat!("periods.", stringify!($field));
                if defined.contains(key) {
                    p.$field = None;
                } else if p.$field.is_some() {
                    defined.insert(key.to_string());
                }
            };
        }
        dedup!(christmas_octave);
        dedup!(days_before_epiphany);
        dedup!(days_from_epiphany);
        dedup!(christmas_to_presentation_of_the_lord);
        dedup!(presentation_of_the_lord_to_holy_thursday);
        dedup!(holy_week);
        dedup!(paschal_triduum);
        dedup!(easter_octave);
        dedup!(early_ordinary_time);
        dedup!(late_ordinary_time);

        if p.christmas_octave.is_none()
            && p.days_before_epiphany.is_none()
            && p.days_from_epiphany.is_none()
            && p.christmas_to_presentation_of_the_lord.is_none()
            && p.presentation_of_the_lord_to_holy_thursday.is_none()
            && p.holy_week.is_none()
            && p.paschal_triduum.is_none()
            && p.easter_octave.is_none()
            && p.early_ordinary_time.is_none()
            && p.late_ordinary_time.is_none()
        {
            *periods = None;
        }
    }
}

fn deduplicate_ranks(ranks: &mut Option<RanksMetadata>, defined: &mut PropertySet) {
    if let Some(r) = ranks {
        macro_rules! dedup {
            ($field:ident) => {
                let key = concat!("ranks.", stringify!($field));
                if defined.contains(key) {
                    r.$field = None;
                } else if r.$field.is_some() {
                    defined.insert(key.to_string());
                }
            };
        }
        dedup!(solemnity);
        dedup!(sunday);
        dedup!(feast);
        dedup!(memorial);
        dedup!(optional_memorial);
        dedup!(weekday);

        if r.solemnity.is_none()
            && r.sunday.is_none()
            && r.feast.is_none()
            && r.memorial.is_none()
            && r.optional_memorial.is_none()
            && r.weekday.is_none()
        {
            *ranks = None;
        }
    }
}

fn deduplicate_cycles(cycles: &mut Option<CyclesMetadata>, defined: &mut PropertySet) {
    if let Some(c) = cycles {
        macro_rules! dedup {
            ($field:ident) => {
                let key = concat!("cycles.", stringify!($field));
                if defined.contains(key) {
                    c.$field = None;
                } else if c.$field.is_some() {
                    defined.insert(key.to_string());
                }
            };
        }
        dedup!(proper_of_time);
        dedup!(proper_of_saints);
        dedup!(sunday_year_a);
        dedup!(sunday_year_b);
        dedup!(sunday_year_c);
        dedup!(weekday_year_1);
        dedup!(weekday_year_2);
        dedup!(psalter_week_1);
        dedup!(psalter_week_2);
        dedup!(psalter_week_3);
        dedup!(psalter_week_4);

        if c.proper_of_time.is_none()
            && c.proper_of_saints.is_none()
            && c.sunday_year_a.is_none()
            && c.sunday_year_b.is_none()
            && c.sunday_year_c.is_none()
            && c.weekday_year_1.is_none()
            && c.weekday_year_2.is_none()
            && c.psalter_week_1.is_none()
            && c.psalter_week_2.is_none()
            && c.psalter_week_3.is_none()
            && c.psalter_week_4.is_none()
        {
            *cycles = None;
        }
    }
}

/// Filter calendar_definitions to keep only:
/// 1. The main calendar (config.calendar)
/// 2. Parent calendars of the main calendar
/// 3. The general_roman calendar
///
/// Returns them ordered according to the priority in keep_ids
/// Returns an error if the main calendar is not found
fn filter_calendar_definitions(romcal: &Romcal) -> RomcalResult<Vec<CalendarDefinition>> {
    // Find the main calendar and its parents
    let main_calendar = romcal
        .calendar_definitions
        .iter()
        .find(|cal| cal.id == romcal.calendar)
        .ok_or_else(|| {
            RomcalError::ValidationError(format!(
                "Main calendar '{}' not found in calendar_definitions",
                romcal.calendar
            ))
        })?;

    // Collect all required calendar IDs (most specific to most general)
    let mut required_ids = Vec::new();

    // Add main calendar first (most specific)
    required_ids.push(main_calendar.id.clone());

    // Add parent calendars (from most specific to most general)
    for parent_id in main_calendar.parent_calendar_ids.iter().rev() {
        if !required_ids.contains(parent_id) {
            required_ids.push(parent_id.clone());
        }
    }

    // Add general_roman last (most general fallback)
    if !required_ids.contains(&"general_roman".to_string()) {
        required_ids.push("general_roman".to_string());
    }

    // Validate that the main calendar is not in its own parent list (circular reference)
    if main_calendar
        .parent_calendar_ids
        .contains(&main_calendar.id)
    {
        return Err(RomcalError::ValidationError(format!(
            "Main calendar '{}' cannot be its own parent (circular reference detected)",
            main_calendar.id
        )));
    }

    // Validate that all required calendars exist
    let available_ids: EntityIdSet = romcal
        .calendar_definitions
        .iter()
        .map(|cal| cal.id.clone())
        .collect();

    for required_id in &required_ids {
        if !available_ids.contains(required_id) {
            return Err(RomcalError::ValidationError(format!(
                "Required calendar '{}' not found in calendar_definitions",
                required_id
            )));
        }
    }

    // Validate that the main calendar is the first in the hierarchy (most specific)
    if required_ids.len() > 1 {
        let first_id = required_ids.first().unwrap();
        if first_id != &main_calendar.id {
            return Err(RomcalError::ValidationError(format!(
                "Main calendar '{}' must be the first in the hierarchy, but found '{}' at the beginning",
                main_calendar.id, first_id
            )));
        }
    }

    // Filter and order calendar_definitions according to required_ids order
    let mut result = Vec::new();
    for id in required_ids {
        if let Some(calendar_def) = romcal.calendar_definitions.iter().find(|cal| cal.id == id) {
            result.push(calendar_def.clone());
        }
    }

    Ok(result)
}

/// Recursively removes null values, empty objects, and $schema properties from a JSON Value
fn remove_null_and_empty_values(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let mut cleaned_map = serde_json::Map::new();
            for (key, val) in map {
                // Skip $schema properties
                if key == "$schema" {
                    continue;
                }
                let cleaned_val = remove_null_and_empty_values(val);
                if !cleaned_val.is_null() {
                    cleaned_map.insert(key, cleaned_val);
                }
            }
            // Return null if the object is empty after cleaning, so it gets filtered out
            if cleaned_map.is_empty() {
                Value::Null
            } else {
                Value::Object(cleaned_map)
            }
        }
        Value::Array(arr) => {
            let cleaned: Vec<Value> = arr
                .into_iter()
                .map(remove_null_and_empty_values)
                .filter(|v| !v.is_null())
                .collect();
            Value::Array(cleaned)
        }
        Value::Null => Value::Null, // This value will be filtered by parent calls
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::calendar::{CalendarJurisdiction, CalendarType, DayDefinition, EntityRef};
    use crate::types::entity::EntityOverride;
    use crate::types::entity::EntityType;

    fn create_test_calendar_definition() -> CalendarDefinition {
        CalendarDefinition {
            schema: None,
            id: "test_calendar".to_string(),
            metadata: crate::types::CalendarMetadata {
                jurisdiction: CalendarJurisdiction::Ecclesiastical,
                r#type: CalendarType::Diocese,
            },
            particular_config: None,
            parent_calendar_ids: vec![],
            days_definitions: {
                let mut map = std::collections::BTreeMap::new();
                map.insert(
                    "saint_john".to_string(),
                    DayDefinition {
                        date_def: None,
                        date_exceptions: None,
                        precedence: None,
                        commons_def: None,
                        is_holy_day_of_obligation: None,
                        allow_similar_rank_items: None,
                        is_optional: None,
                        custom_locale_id: None,
                        entities: Some(vec![
                            EntityRef::ResourceId("john_the_baptist".to_string()),
                            EntityRef::Override(EntityOverride {
                                id: "john_the_evangelist".to_string(),
                                titles: None,
                                hide_titles: None,
                                count: None,
                            }),
                        ]),
                        titles: None,
                        drop: None,
                        colors: None,
                        masses: None,
                    },
                );
                map.insert(
                    "saint_peter".to_string(),
                    DayDefinition {
                        date_def: None,
                        date_exceptions: None,
                        precedence: None,
                        commons_def: None,
                        is_holy_day_of_obligation: None,
                        allow_similar_rank_items: None,
                        is_optional: None,
                        custom_locale_id: None,
                        entities: None,
                        titles: None,
                        drop: None,
                        colors: None,
                        masses: None,
                    },
                );
                map
            },
        }
    }

    fn create_test_resources() -> Resources {
        use crate::types::entity::EntityDefinition;

        let mut entities = std::collections::BTreeMap::new();

        entities.insert(
            "john_the_baptist".to_string(),
            EntityDefinition {
                r#type: Some(EntityType::Person),
                fullname: Some("John the Baptist".to_string()),
                name: Some("John".to_string()),
                ..Default::default()
            },
        );

        entities.insert(
            "john_the_evangelist".to_string(),
            EntityDefinition {
                r#type: Some(EntityType::Person),
                fullname: Some("John the Evangelist".to_string()),
                name: Some("John".to_string()),
                ..Default::default()
            },
        );

        entities.insert(
            "unused_entity".to_string(),
            EntityDefinition {
                r#type: Some(EntityType::Person),
                fullname: Some("Unused Entity".to_string()),
                name: Some("Unused".to_string()),
                ..Default::default()
            },
        );

        Resources {
            schema: None,
            locale: "en".to_string(),
            metadata: None,
            entities: Some(entities),
        }
    }

    #[test]
    fn test_collect_used_entity_ids() {
        let calendar_definitions = vec![create_test_calendar_definition()];
        let used_entity_ids = collect_used_entity_ids(&calendar_definitions);

        // Should include day_definition IDs
        assert!(used_entity_ids.contains("saint_john"));
        assert!(used_entity_ids.contains("saint_peter"));

        // Should include ResourceId references
        assert!(used_entity_ids.contains("john_the_baptist"));

        // Should include Override entity IDs
        assert!(used_entity_ids.contains("john_the_evangelist"));

        // Should not include entities not referenced
        assert!(!used_entity_ids.contains("unused_entity"));
    }

    #[test]
    fn test_collect_used_entity_ids_empty_entities() {
        let calendar_definitions = vec![CalendarDefinition {
            schema: None,
            id: "test_calendar".to_string(),
            metadata: crate::types::CalendarMetadata {
                jurisdiction: CalendarJurisdiction::Ecclesiastical,
                r#type: CalendarType::Diocese,
            },
            particular_config: None,
            parent_calendar_ids: vec![],
            days_definitions: {
                let mut map = std::collections::BTreeMap::new();
                map.insert(
                    "saint_mary".to_string(),
                    DayDefinition {
                        date_def: None,
                        date_exceptions: None,
                        precedence: None,
                        commons_def: None,
                        is_holy_day_of_obligation: None,
                        allow_similar_rank_items: None,
                        is_optional: None,
                        custom_locale_id: None,
                        entities: None, // No entities
                        titles: None,
                        drop: None,
                        colors: None,
                        masses: None,
                    },
                );
                map
            },
        }];

        let used_entity_ids = collect_used_entity_ids(&calendar_definitions);

        // Should only include the day_definition ID
        assert!(used_entity_ids.contains("saint_mary"));
        assert_eq!(used_entity_ids.len(), 1);
    }

    #[test]
    fn test_filter_entities_by_usage() {
        let mut resources = create_test_resources();
        let used_entity_ids: EntityIdSet = ["john_the_baptist", "john_the_evangelist"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        filter_entities_by_usage(&mut resources, &used_entity_ids);

        let entities = resources.entities.unwrap();
        assert_eq!(entities.len(), 2);

        let entity_ids: Vec<String> = entities.keys().cloned().collect();
        assert!(entity_ids.contains(&"john_the_baptist".to_string()));
        assert!(entity_ids.contains(&"john_the_evangelist".to_string()));
        assert!(!entity_ids.contains(&"unused_entity".to_string()));
    }

    #[test]
    fn test_filter_entities_by_usage_empty_used_ids() {
        let mut resources = create_test_resources();
        let used_entity_ids = EntityIdSet::new();

        filter_entities_by_usage(&mut resources, &used_entity_ids);

        let entities = resources.entities.unwrap();
        assert_eq!(entities.len(), 0);
    }

    #[test]
    fn test_filter_entities_by_usage_no_entities() {
        let mut resources = Resources {
            schema: None,
            locale: "en".to_string(),
            metadata: None,
            entities: None,
        };
        let used_entity_ids: EntityIdSet = ["some_entity"].iter().map(|s| s.to_string()).collect();

        // Should not panic when entities is None
        filter_entities_by_usage(&mut resources, &used_entity_ids);
        assert!(resources.entities.is_none());
    }

    // ========================================================================
    // Property-Level Deduplication Tests
    // ========================================================================

    fn create_entity(
        name: Option<&str>,
        fullname: Option<&str>,
        entity_type: Option<EntityType>,
    ) -> EntityDefinition {
        EntityDefinition {
            name: name.map(|s| s.to_string()),
            fullname: fullname.map(|s| s.to_string()),
            r#type: entity_type,
            ..Default::default()
        }
    }

    fn create_resources_with_entity(
        locale: &str,
        entity_id: &str,
        entity: EntityDefinition,
    ) -> Resources {
        let mut entities = std::collections::BTreeMap::new();
        entities.insert(entity_id.to_string(), entity);
        Resources {
            schema: None,
            locale: locale.to_string(),
            metadata: None,
            entities: Some(entities),
        }
    }

    #[test]
    fn test_deduplicate_entity_properties() {
        // Setup: 3 locales with the same entity, different properties
        // Order: most specific → most general (fr-ca, fr, en)
        let mut resources = vec![
            // fr-ca (most specific) - only name
            create_resources_with_entity("fr-ca", "john", create_entity(Some("Jean"), None, None)),
            // fr - name + fullname
            create_resources_with_entity(
                "fr",
                "john",
                create_entity(Some("Jean"), Some("Jean le Baptiste"), None),
            ),
            // en (most general) - name + fullname + type
            create_resources_with_entity(
                "en",
                "john",
                create_entity(
                    Some("John"),
                    Some("John the Baptist"),
                    Some(EntityType::Person),
                ),
            ),
        ];

        deduplicate_entity_properties(&mut resources);

        // fr-ca: keeps name (most specific)
        let fr_ca = resources[0].entities.as_ref().unwrap().get("john").unwrap();
        assert!(fr_ca.name.is_some());
        assert!(fr_ca.fullname.is_none()); // not defined in fr-ca
        assert!(fr_ca.r#type.is_none()); // not defined in fr-ca

        // fr: name removed (exists in fr-ca), keeps fullname
        let fr = resources[1].entities.as_ref().unwrap().get("john").unwrap();
        assert!(fr.name.is_none()); // removed because fr-ca has it
        assert!(fr.fullname.is_some()); // first to define fullname
        assert!(fr.r#type.is_none()); // not defined in fr

        // en: name and fullname removed, keeps type
        let en = resources[2].entities.as_ref().unwrap().get("john").unwrap();
        assert!(en.name.is_none()); // removed because fr-ca has it
        assert!(en.fullname.is_none()); // removed because fr has it
        assert!(en.r#type.is_some()); // first (and only) to define type
    }

    #[test]
    fn test_remove_empty_entities_after_dedup() {
        // Setup: en has only properties that fr also has → en entity becomes empty
        let mut resources = vec![
            // fr (more specific) - only name
            create_resources_with_entity("fr", "john", create_entity(Some("Jean"), None, None)),
            // en (more general) - only name (same property as fr)
            create_resources_with_entity("en", "john", create_entity(Some("John"), None, None)),
        ];

        deduplicate_entity_properties(&mut resources);
        remove_empty_entities(&mut resources);

        // fr: keeps john (has name)
        assert!(resources[0].entities.as_ref().unwrap().contains_key("john"));

        // en: john removed (all properties were deduplicated)
        assert!(!resources[1].entities.as_ref().unwrap().contains_key("john"));
    }

    #[test]
    fn test_deduplicate_metadata_properties() {
        use crate::types::resource::ResourcesMetadata;

        let mut resources = vec![
            // fr (more specific) - has weekdays
            Resources {
                schema: None,
                locale: "fr".to_string(),
                metadata: Some(ResourcesMetadata {
                    weekdays: Some({
                        let mut map = std::collections::BTreeMap::new();
                        map.insert("0".to_string(), "dimanche".to_string());
                        map
                    }),
                    months: None,
                    ordinal_format: None,
                    ordinals_letters: None,
                    ordinals_numeric: None,
                    colors: None,
                    seasons: None,
                    periods: None,
                    ranks: None,
                    cycles: None,
                }),
                entities: None,
            },
            // en (more general) - has weekdays + months
            Resources {
                schema: None,
                locale: "en".to_string(),
                metadata: Some(ResourcesMetadata {
                    weekdays: Some({
                        let mut map = std::collections::BTreeMap::new();
                        map.insert("0".to_string(), "Sunday".to_string());
                        map
                    }),
                    months: Some({
                        let mut map = std::collections::BTreeMap::new();
                        map.insert("1".to_string(), "January".to_string());
                        map
                    }),
                    ordinal_format: None,
                    ordinals_letters: None,
                    ordinals_numeric: None,
                    colors: None,
                    seasons: None,
                    periods: None,
                    ranks: None,
                    cycles: None,
                }),
                entities: None,
            },
        ];

        deduplicate_metadata_properties(&mut resources);

        // fr: keeps weekdays
        assert!(resources[0].metadata.as_ref().unwrap().weekdays.is_some());

        // en: weekdays removed (exists in fr), keeps months
        assert!(resources[1].metadata.as_ref().unwrap().weekdays.is_none());
        assert!(resources[1].metadata.as_ref().unwrap().months.is_some());
    }

    #[test]
    fn test_deduplicate_nested_seasons() {
        use crate::types::resource::{AdventSeason, ResourcesMetadata, SeasonsMetadata};

        let mut resources = vec![
            // fr (more specific) - has advent.season
            Resources {
                schema: None,
                locale: "fr".to_string(),
                metadata: Some(ResourcesMetadata {
                    seasons: Some(SeasonsMetadata {
                        advent: Some(AdventSeason {
                            season: Some("Avent".to_string()),
                            weekday: None,
                            sunday: None,
                            privileged_weekday: None,
                        }),
                        christmas_time: None,
                        ordinary_time: None,
                        lent: None,
                        paschal_triduum: None,
                        easter_time: None,
                    }),
                    weekdays: None,
                    months: None,
                    ordinal_format: None,
                    ordinals_letters: None,
                    ordinals_numeric: None,
                    colors: None,
                    periods: None,
                    ranks: None,
                    cycles: None,
                }),
                entities: None,
            },
            // en (more general) - has advent.season + advent.weekday
            Resources {
                schema: None,
                locale: "en".to_string(),
                metadata: Some(ResourcesMetadata {
                    seasons: Some(SeasonsMetadata {
                        advent: Some(AdventSeason {
                            season: Some("Advent".to_string()),
                            weekday: Some("Weekday of Advent".to_string()),
                            sunday: None,
                            privileged_weekday: None,
                        }),
                        christmas_time: None,
                        ordinary_time: None,
                        lent: None,
                        paschal_triduum: None,
                        easter_time: None,
                    }),
                    weekdays: None,
                    months: None,
                    ordinal_format: None,
                    ordinals_letters: None,
                    ordinals_numeric: None,
                    colors: None,
                    periods: None,
                    ranks: None,
                    cycles: None,
                }),
                entities: None,
            },
        ];

        deduplicate_metadata_properties(&mut resources);

        // fr: keeps advent.season
        let fr_advent = resources[0]
            .metadata
            .as_ref()
            .unwrap()
            .seasons
            .as_ref()
            .unwrap()
            .advent
            .as_ref()
            .unwrap();
        assert!(fr_advent.season.is_some());

        // en: advent.season removed, keeps advent.weekday
        let en_advent = resources[1]
            .metadata
            .as_ref()
            .unwrap()
            .seasons
            .as_ref()
            .unwrap()
            .advent
            .as_ref()
            .unwrap();
        assert!(en_advent.season.is_none());
        assert!(en_advent.weekday.is_some());
    }

    #[test]
    fn test_is_entity_empty() {
        // Empty entity (all properties None)
        let empty = EntityDefinition {
            r#type: None,
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
        };
        assert!(is_entity_empty(&empty));

        // Entity with one property
        let with_name = create_entity(Some("John"), None, None);
        assert!(!is_entity_empty(&with_name));

        // Entity with type only
        let with_type = create_entity(None, None, Some(EntityType::Person));
        assert!(!is_entity_empty(&with_type));
    }

    #[test]
    fn test_deduplicate_entity_independent_entities() {
        // Two different entities should not affect each other
        let mut resources = vec![
            // fr: has john and peter
            Resources {
                schema: None,
                locale: "fr".to_string(),
                metadata: None,
                entities: Some({
                    let mut map = std::collections::BTreeMap::new();
                    map.insert("john".to_string(), create_entity(Some("Jean"), None, None));
                    map.insert(
                        "peter".to_string(),
                        create_entity(Some("Pierre"), None, None),
                    );
                    map
                }),
            },
            // en: has john and peter with same properties
            Resources {
                schema: None,
                locale: "en".to_string(),
                metadata: None,
                entities: Some({
                    let mut map = std::collections::BTreeMap::new();
                    map.insert(
                        "john".to_string(),
                        create_entity(Some("John"), Some("John the Baptist"), None),
                    );
                    map.insert(
                        "peter".to_string(),
                        create_entity(Some("Peter"), Some("Peter the Apostle"), None),
                    );
                    map
                }),
            },
        ];

        deduplicate_entity_properties(&mut resources);

        // fr: both keep name
        let fr_entities = resources[0].entities.as_ref().unwrap();
        assert!(fr_entities.get("john").unwrap().name.is_some());
        assert!(fr_entities.get("peter").unwrap().name.is_some());

        // en: both lose name, keep fullname
        let en_entities = resources[1].entities.as_ref().unwrap();
        assert!(en_entities.get("john").unwrap().name.is_none());
        assert!(en_entities.get("john").unwrap().fullname.is_some());
        assert!(en_entities.get("peter").unwrap().name.is_none());
        assert!(en_entities.get("peter").unwrap().fullname.is_some());
    }
}
