#[cfg(feature = "schema-gen")]
use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize};

/// Day of week (0-6, where 0=Sunday) with automatic validation
///
/// This type ensures that only valid day-of-week values are accepted during
/// deserialization. The value 0 represents Sunday, 1 represents Monday, etc.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
pub struct DayOfWeek(pub u8);

impl<'de> Deserialize<'de> for DayOfWeek {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u8::deserialize(deserializer)?;
        if (0..=6).contains(&value) {
            Ok(DayOfWeek(value))
        } else {
            Err(serde::de::Error::custom(format!(
                "Day of week must be between 0 and 6 (where 0=Sunday), got {}",
                value
            )))
        }
    }
}

impl DayOfWeek {
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
        let day = DayOfWeek(3);
        let json = serde_json::to_string(&day).unwrap();
        assert_eq!(json, "3");
    }

    #[test]
    fn test_deserialize_valid_values() {
        let valid_values = [0, 1, 2, 3, 4, 5, 6];
        for val in valid_values {
            assert_de_tokens(&DayOfWeek(val), &[Token::U8(val)]);
        }
    }

    #[test]
    fn test_deserialize_invalid_values() {
        let invalid_values = [7, 8, 255];
        for val in invalid_values {
            assert_de_tokens_error::<DayOfWeek>(
                &[Token::U8(val)],
                &format!(
                    "Day of week must be between 0 and 6 (where 0=Sunday), got {}",
                    val
                ),
            );
        }
    }
}
