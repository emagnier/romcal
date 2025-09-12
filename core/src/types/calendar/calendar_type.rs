use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CalendarType {
    GeneralRoman,
    Region,
    Country,
    Archdiocese,
    Diocese,
    City,
    Parish,
    GeneralCommunity,
    RegionalCommunity,
    LocalCommunity,
    Other,
}
