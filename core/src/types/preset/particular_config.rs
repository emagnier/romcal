#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ts-bindings")]
use ts_rs::TS;

use crate::types::EasterCalculationType;

/// Configuration options for "particular" (local/diocesan) calendars.
///
/// In liturgical terminology, a "particular" calendar is one that applies to a specific
/// region, diocese, or religious community, as opposed to the General Roman Calendar
/// which applies universally.
///
/// These settings can override or extend the default Romcal configuration or any parent
/// calendar configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[cfg_attr(feature = "ts-bindings", derive(TS))]
#[cfg_attr(feature = "ts-bindings", ts(export))]
pub struct ParticularConfig {
    /// Epiphany is celebrated on a Sunday
    pub epiphany_on_sunday: Option<bool>,
    /// Ascension is celebrated on a Sunday
    pub ascension_on_sunday: Option<bool>,
    /// Corpus Christi is celebrated on a Sunday
    pub corpus_christi_on_sunday: Option<bool>,
    /// The type of Easter calculation
    pub easter_calculation_type: Option<EasterCalculationType>,
}
