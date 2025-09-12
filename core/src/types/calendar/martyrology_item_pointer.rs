use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::types::MartyrologyItemRedefined;

// Type alias
pub type ResourceId = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum MartyrologyItemPointer {
    ResourceId(ResourceId),
    Redefined(MartyrologyItemRedefined),
}
