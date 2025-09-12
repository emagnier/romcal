use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::EasterCalculationType;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ParticularConfig {
    /// Configuration options specific to this calendar.
    /// These settings can override or extend the default Romcal configuration or any parent calendar
    /// configuration.
    pub ascension_on_sunday: Option<bool>,
    pub epiphany_on_sunday: Option<bool>,
    pub corpus_christi_on_sunday: Option<bool>,
    pub easter_calculation_type: Option<EasterCalculationType>,
}
