#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::EnumIter;
#[cfg(feature = "ts-bindings")]
use ts_rs::TS;

/// The jurisdiction of the calendar.
/// Determines whether the calendar follows ecclesiastical or civil authority.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter, Default)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[cfg_attr(feature = "ts-bindings", derive(TS))]
#[cfg_attr(feature = "ts-bindings", ts(export))]
#[serde(rename_all = "snake_case")]
pub enum CalendarJurisdiction {
    /// Calendar under ecclesiastical authority (Church)
    #[default]
    Ecclesiastical,
    /// Calendar under civil authority (State)
    Civil,
}
