//! Property-level deduplication for bundle generation.

use crate::Resources;
use crate::types::martyrology::MartyrologyEntryDef;
use crate::types::resource::{
    AdventSeason, ChristmasTimeSeason, CyclesMetadata, EasterTimeSeason, LentSeason, LocaleColors,
    OrdinaryTimeSeason, PaschalTriduumSeason, PeriodsMetadata, RanksMetadata, ResourcesMetadata,
    SeasonsMetadata,
};

use super::{MartyrologyPropertiesMap, PropertySet};

// ============================================================================
// Martyrology Property-Level Deduplication
// ============================================================================

/// Deduplicate martyrology entry properties across locales.
///
/// For each entry, if a property exists in a more specific locale,
/// it is removed from parent locales. Resources must be ordered
/// from most specific to most general.
pub(super) fn deduplicate_martyrology_properties(resources: &mut [Resources]) {
    let mut defined_props: MartyrologyPropertiesMap = std::collections::HashMap::new();

    for resource in resources.iter_mut() {
        if let Some(martyrology) = &mut resource.martyrology {
            for (entry_id, entry) in martyrology.iter_mut() {
                let props = defined_props.entry(entry_id.clone()).or_default();
                deduplicate_single_entry(entry, props);
            }
        }
    }
}

/// Deduplicate properties of a single martyrology entry.
///
/// For each property: if already defined in a more specific locale, set to None;
/// otherwise if Some, mark as defined for parent locales.
fn deduplicate_single_entry(entry: &mut MartyrologyEntryDef, defined: &mut PropertySet) {
    /// Macro to deduplicate a single Option field.
    /// If already defined → set to None. If Some → mark as defined.
    macro_rules! dedup {
        ($field:ident) => {
            if defined.contains(stringify!($field)) {
                entry.$field = None;
            } else if entry.$field.is_some() {
                defined.insert(stringify!($field).to_string());
            }
        };
    }

    dedup!(r#type);
    dedup!(fullname);
    dedup!(name);
    dedup!(canonization_level);
    dedup!(date_of_canonization);
    dedup!(date_of_canonization_is_approximative);
    dedup!(date_of_beatification);
    dedup!(date_of_beatification_is_approximative);
    dedup!(hide_canonization_level);
    dedup!(titles);
    dedup!(sex);
    dedup!(hide_titles);
    dedup!(date_of_dedication);
    dedup!(date_of_birth);
    dedup!(date_of_birth_is_approximative);
    dedup!(date_of_death);
    dedup!(date_of_death_is_approximative);
    dedup!(count);
    dedup!(sources);
}

/// Check if a martyrology entry has all properties set to None.
pub(super) fn is_entry_empty(entry: &MartyrologyEntryDef) -> bool {
    /// Macro to check if a field is None.
    macro_rules! is_none {
        ($($field:ident),+) => {
            $(entry.$field.is_none())&&+
        };
    }

    // Note: _todo is excluded as it's internal metadata (not serialized)
    is_none!(
        r#type,
        fullname,
        name,
        canonization_level,
        date_of_canonization,
        date_of_canonization_is_approximative,
        date_of_beatification,
        date_of_beatification_is_approximative,
        hide_canonization_level,
        titles,
        sex,
        hide_titles,
        date_of_dedication,
        date_of_birth,
        date_of_birth_is_approximative,
        date_of_death,
        date_of_death_is_approximative,
        count,
        sources
    )
}

/// Remove martyrology entries where all properties are None after deduplication.
pub(super) fn remove_empty_martyrology_entries(resources: &mut [Resources]) {
    for resource in resources.iter_mut() {
        if let Some(martyrology) = &mut resource.martyrology {
            martyrology.retain(|_, entry| !is_entry_empty(entry));
        }
    }
}

// ============================================================================
// Metadata Property-Level Deduplication
// ============================================================================

/// Deduplicate metadata properties across locales.
///
/// Uses hierarchical property keys (e.g., `seasons.advent.season`) for tracking.
/// Nested structures are recursively deduplicated.
pub(super) fn deduplicate_metadata_properties(resources: &mut [Resources]) {
    let mut defined = PropertySet::new();

    for resource in resources.iter_mut() {
        if let Some(metadata) = &mut resource.metadata {
            deduplicate_single_metadata(metadata, &mut defined);
        }
    }
}

/// Deduplicate properties of a single metadata object.
fn deduplicate_single_metadata(metadata: &mut ResourcesMetadata, defined: &mut PropertySet) {
    /// Macro for simple Option properties at metadata level.
    macro_rules! dedup {
        ($field:ident) => {
            if defined.contains(stringify!($field)) {
                metadata.$field = None;
            } else if metadata.$field.is_some() {
                defined.insert(stringify!($field).to_string());
            }
        };
    }

    dedup!(ordinal_format);
    dedup!(ordinals_letters);
    dedup!(ordinals_numeric);
    dedup!(weekdays);
    dedup!(months);

    // Nested structures
    deduplicate_colors(&mut metadata.colors, defined);
    deduplicate_seasons(&mut metadata.seasons, defined);
    deduplicate_periods(&mut metadata.periods, defined);
    deduplicate_ranks(&mut metadata.ranks, defined);
    deduplicate_cycles(&mut metadata.cycles, defined);
}

// ============================================================================
// Nested Metadata Deduplication Helpers
// ============================================================================

/// Macro to generate deduplication functions for nested metadata structs.
///
/// This macro generates a function that:
/// 1. Deduplicates each field using a prefixed key
/// 2. Sets the entire struct to None if all fields become None
macro_rules! impl_nested_dedup {
    (
        $fn_name:ident,
        $struct_type:ty,
        $prefix:literal,
        $($field:ident),+
    ) => {
        fn $fn_name(opt: &mut Option<$struct_type>, defined: &mut PropertySet) {
            if let Some(s) = opt {
                $(
                    let key = concat!($prefix, ".", stringify!($field));
                    if defined.contains(key) {
                        s.$field = None;
                    } else if s.$field.is_some() {
                        defined.insert(key.to_string());
                    }
                )+

                // Remove struct if all fields are None
                if true $(&& s.$field.is_none())+ {
                    *opt = None;
                }
            }
        }
    };
}

impl_nested_dedup!(
    deduplicate_colors,
    LocaleColors,
    "colors",
    black,
    gold,
    green,
    purple,
    red,
    rose,
    white
);

impl_nested_dedup!(
    deduplicate_advent,
    AdventSeason,
    "seasons.advent",
    season,
    weekday,
    sunday,
    privileged_weekday
);

impl_nested_dedup!(
    deduplicate_christmas_time,
    ChristmasTimeSeason,
    "seasons.christmas_time",
    season,
    day,
    octave,
    before_epiphany,
    second_sunday_after_christmas,
    after_epiphany
);

impl_nested_dedup!(
    deduplicate_ordinary_time,
    OrdinaryTimeSeason,
    "seasons.ordinary_time",
    season,
    weekday,
    sunday
);

impl_nested_dedup!(
    deduplicate_lent,
    LentSeason,
    "seasons.lent",
    season,
    weekday,
    sunday,
    day_after_ash_wed,
    holy_week_day
);

impl_nested_dedup!(
    deduplicate_paschal_triduum,
    PaschalTriduumSeason,
    "seasons.paschal_triduum",
    season
);

impl_nested_dedup!(
    deduplicate_easter_time,
    EasterTimeSeason,
    "seasons.easter_time",
    season,
    weekday,
    sunday,
    octave
);

impl_nested_dedup!(
    deduplicate_periods,
    PeriodsMetadata,
    "periods",
    christmas_octave,
    days_before_epiphany,
    days_from_epiphany,
    christmas_to_presentation_of_the_lord,
    presentation_of_the_lord_to_holy_thursday,
    holy_week,
    paschal_triduum,
    easter_octave,
    early_ordinary_time,
    late_ordinary_time
);

impl_nested_dedup!(
    deduplicate_ranks,
    RanksMetadata,
    "ranks",
    solemnity,
    sunday,
    feast,
    memorial,
    optional_memorial,
    weekday
);

impl_nested_dedup!(
    deduplicate_cycles,
    CyclesMetadata,
    "cycles",
    proper_of_time,
    proper_of_saints,
    sunday_year_a,
    sunday_year_b,
    sunday_year_c,
    weekday_year_1,
    weekday_year_2,
    psalter_week_1,
    psalter_week_2,
    psalter_week_3,
    psalter_week_4
);

/// Deduplicate seasons metadata (container for all season types).
fn deduplicate_seasons(seasons: &mut Option<SeasonsMetadata>, defined: &mut PropertySet) {
    if let Some(s) = seasons {
        deduplicate_advent(&mut s.advent, defined);
        deduplicate_christmas_time(&mut s.christmas_time, defined);
        deduplicate_ordinary_time(&mut s.ordinary_time, defined);
        deduplicate_lent(&mut s.lent, defined);
        deduplicate_paschal_triduum(&mut s.paschal_triduum, defined);
        deduplicate_easter_time(&mut s.easter_time, defined);

        // Remove seasons if all are None
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
