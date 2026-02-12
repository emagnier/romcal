//! Calendar and resource filtering for bundle generation.

use std::collections::HashMap;

use crate::martyrology_resolution::locale::get_all_parent_locales;
use crate::{CalendarDefinition, Resources, Romcal, RomcalError, RomcalResult};

use super::deduplicate::{
    deduplicate_martyrology_properties, deduplicate_metadata_properties,
    remove_empty_martyrology_entries,
};
use super::{IdSet, LocaleMap};

// ============================================================================
// Calendar Filtering
// ============================================================================

/// Filter calendar definitions to include only the hierarchy chain.
///
/// Keeps: main calendar → parent calendars → general_roman (fallback).
/// Returns calendars ordered from most specific to most general.
pub(super) fn filter_calendar_definitions(
    romcal: &Romcal,
) -> RomcalResult<Vec<CalendarDefinition>> {
    // Find the main calendar
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

    // Check for circular reference
    if main_calendar
        .parent_calendar_ids
        .contains(&main_calendar.id)
    {
        return Err(RomcalError::ValidationError(format!(
            "Calendar '{}' cannot be its own parent (circular reference)",
            main_calendar.id
        )));
    }

    // Build required IDs list: main → parents → general_roman (specific → general)
    // bundle() reverses to: general_roman → parents → main (general → specific)
    let mut required_ids = vec![main_calendar.id.clone()];
    for parent_id in &main_calendar.parent_calendar_ids {
        if !required_ids.contains(parent_id) {
            required_ids.push(parent_id.clone());
        }
    }
    if !required_ids.contains(&"general_roman".to_string()) {
        required_ids.push("general_roman".to_string());
    }

    // Validate all required calendars exist
    let available: IdSet = romcal
        .calendar_definitions
        .iter()
        .map(|c| c.id.clone())
        .collect();
    for id in &required_ids {
        if !available.contains(id) {
            return Err(RomcalError::ValidationError(format!(
                "Required calendar '{}' not found in calendar_definitions",
                id
            )));
        }
    }

    // Collect in order
    Ok(required_ids
        .iter()
        .filter_map(|id| {
            romcal
                .calendar_definitions
                .iter()
                .find(|c| &c.id == id)
                .cloned()
        })
        .collect())
}

// ============================================================================
// Resource Filtering
// ============================================================================

/// Filter resources to include only the locale hierarchy with property-level deduplication.
///
/// For locale `fr-ca`, keeps: `fr-ca` → `fr` → `en` (fallback).
/// Applies property-level deduplication so parent locales only contain
/// properties not defined in child locales.
pub(super) fn filter_resources(
    romcal: &Romcal,
    filtered_calendars: &[CalendarDefinition],
) -> RomcalResult<Vec<Resources>> {
    let target_locale = &romcal.locale;

    // Build lookup maps
    let (available_locales, resources_by_locale) = build_locale_maps(romcal);

    // Validate target locale exists
    let exact_locale = available_locales
        .get(&target_locale.to_lowercase())
        .cloned()
        .ok_or_else(|| {
            RomcalError::ValidationError(format!(
                "Locale '{}' not found. Available: {}",
                target_locale,
                available_locales
                    .values()
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ")
            ))
        })?;

    // Collect martyrology entry IDs used in calendar definitions
    let used_martyrology_ids = collect_used_martyrology_ids(filtered_calendars);

    // Build locale priority: specific → general → en
    let priority_locales = build_priority_locales(target_locale, &available_locales, &exact_locale);

    // Apply hierarchical deduplication
    apply_hierarchical_deduplication(
        priority_locales,
        &resources_by_locale,
        &used_martyrology_ids,
    )
}

/// Build maps for efficient locale lookups.
fn build_locale_maps(romcal: &Romcal) -> (LocaleMap, HashMap<&str, &Resources>) {
    let available_locales: LocaleMap = romcal
        .resources
        .iter()
        .map(|r| (r.locale.to_lowercase(), r.locale.clone()))
        .collect();

    let resources_by_locale: HashMap<&str, &Resources> = romcal
        .resources
        .iter()
        .map(|r| (r.locale.as_str(), r))
        .collect();

    (available_locales, resources_by_locale)
}

/// Build priority list of locales from most specific to most general.
fn build_priority_locales(
    target_locale: &str,
    available_locales: &LocaleMap,
    exact_locale: &str,
) -> Vec<String> {
    let mut locales = vec![exact_locale.to_string()];

    // Add parent locales in hierarchy order
    for parent in get_all_parent_locales(target_locale) {
        if parent != target_locale
            && let Some(actual) = available_locales.get(&parent.to_lowercase())
        {
            locales.push(actual.clone());
        }
    }

    // Always include "en" as fallback
    if let Some(en) = available_locales.get("en")
        && !locales.contains(en)
    {
        locales.push(en.clone());
    }

    locales
}

/// Collect all martyrology entry IDs referenced in calendar day definitions.
pub(super) fn collect_used_martyrology_ids(calendars: &[CalendarDefinition]) -> IdSet {
    let mut ids = IdSet::new();

    for cal in calendars {
        for (day_id, day_def) in &cal.days_definitions {
            // Day definition ID is itself a potential martyrology reference
            ids.insert(day_id.clone());

            // Collect martyrology references
            if let Some(martyrology) = &day_def.martyrology {
                for martyrology_ref in martyrology {
                    match martyrology_ref {
                        crate::types::calendar::MartyrologyRef::ResourceId(id) => {
                            ids.insert(id.clone());
                        }
                        crate::types::calendar::MartyrologyRef::Override(o) => {
                            ids.insert(o.id.clone());
                        }
                    }
                }
            }
        }
    }

    ids
}

/// Apply hierarchical deduplication to resources.
///
/// Resources are processed from most specific to most general locale.
/// Property-level deduplication ensures parent locales only contain
/// properties that are missing in their child locales.
fn apply_hierarchical_deduplication(
    priority_locales: Vec<String>,
    resources_by_locale: &HashMap<&str, &Resources>,
    used_martyrology_ids: &IdSet,
) -> RomcalResult<Vec<Resources>> {
    // Build filtered resources list (specific → general)
    let mut resources: Vec<Resources> = priority_locales
        .iter()
        .filter_map(|locale| {
            resources_by_locale.get(locale.as_str()).map(|r| {
                let mut filtered = (*r).clone();
                filter_martyrology_by_usage(&mut filtered, used_martyrology_ids);
                filtered
            })
        })
        .collect();

    // Apply property-level deduplication
    deduplicate_martyrology_properties(&mut resources);
    deduplicate_metadata_properties(&mut resources);

    // Clean up empty martyrology entries
    remove_empty_martyrology_entries(&mut resources);

    Ok(resources)
}

/// Filter martyrology entries to only include those referenced in calendar definitions.
pub(super) fn filter_martyrology_by_usage(resource: &mut Resources, used_ids: &IdSet) {
    if let Some(martyrology) = &mut resource.martyrology {
        martyrology.retain(|id, _| used_ids.contains(id));
    }
}
