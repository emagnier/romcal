#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};

/// Month index (1-12) with automatic validation
///
/// This type ensures that only valid month values are accepted during
/// deserialization. The value 1 represents January, 2 represents February, etc.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
pub struct MonthIndex(pub u8);

impl<'de> Deserialize<'de> for MonthIndex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        if (1..=12).contains(&value) {
            Ok(MonthIndex(value))
        } else {
            Err(serde::de::Error::custom(format!(
                "Month must be between 1 and 12 (where 1=January), got {}",
                value
            )))
        }
    }
}

impl MonthIndex {
    /// Returns the underlying u8 value
    pub fn value(&self) -> u8 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{Token, assert_de_tokens, assert_de_tokens_error};

    #[test]
    fn test_serialize() {
        let month = MonthIndex(3);
        let json = serde_json::to_string(&month).unwrap();
        assert_eq!(json, "3");
    }

    #[test]
    fn test_deserialize_valid_values() {
        let valid_values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
        for val in valid_values {
            assert_de_tokens(&MonthIndex(val), &[Token::U8(val)]);
        }
    }

    #[test]
    fn test_deserialize_invalid_values() {
        let invalid_values = [0, 13, 255];
        for val in invalid_values {
            assert_de_tokens_error::<MonthIndex>(
                &[Token::U8(val)],
                &format!(
                    "Month must be between 1 and 12 (where 1=January), got {}",
                    val
                ),
            );
        }
    }
}
