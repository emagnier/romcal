use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::SaintCount;
use crate::types::TitlesDef;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct MartyrologyItemRedefined {
    /// The ID of the martyrology item.
    pub id: String,
    /// The redefined titles of the martyrology item.
    pub titles: Option<TitlesDef>,
    /// Specify if titles should not be displayed. This can occur when a title is already included in
    /// the name of the martyrology item.
    pub hide_titles: Option<bool>,
    /// Specify the number of persons this martyrology item is representing.
    pub count: Option<SaintCount>,
}
