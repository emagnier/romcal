#[cfg(feature = "schema-gen")]
use schemars::{JsonSchema, Schema, SchemaGenerator};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// ============================================================================
// Schema generation function (only compiled when feature "schema-gen" is enabled)
// ============================================================================

/// Custom JSON Schema for SaintCount.
/// Generates a schema that accepts either an integer or the string "MANY".
#[cfg(feature = "schema-gen")]
fn saint_count_schema(_gen: &mut SchemaGenerator) -> Schema {
    serde_json::from_value(serde_json::json!({
        "anyOf": [
            {
                "type": "integer",
                "format": "uint32",
                "minimum": 0
            },
            {
                "const": "MANY",
                "type": "string"
            },
            {
                "type": "null"
            }
        ]
    }))
    .unwrap()
}

/// Represents the number of saints for an entity or a group of entities.
///
/// Can be either a specific number (u32) or "MANY" to indicate
/// an indeterminate number of saints.
///
/// # Serialization
/// - `Number(n)` serializes as integer `n`
/// - `Many` serializes as string `"MANY"`
///
/// # Deserialization
/// - Integers are converted to `Number(u32)`
/// - String `"MANY"` is converted to `Many`
/// - All other types generate an error
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "schema-gen", derive(JsonSchema))]
#[cfg_attr(feature = "schema-gen", schemars(schema_with = "saint_count_schema"))]
pub enum SaintCount {
    /// Specific number of saints
    Number(u32),
    /// Indeterminate number of saints
    Many(String),
}

impl SaintCount {
    /// Create a new SaintCount with a specific number
    pub fn number(n: u32) -> Self {
        Self::Number(n)
    }

    /// Create a new SaintCount representing "MANY"
    pub fn many() -> Self {
        Self::Many("MANY".to_string())
    }

    /// Check if this represents "MANY"
    pub fn is_many(&self) -> bool {
        matches!(self, Self::Many(s) if s == "MANY")
    }

    /// Get the number if it's a specific number, None if it's "MANY"
    pub fn as_number(&self) -> Option<u32> {
        match self {
            Self::Number(n) => Some(*n),
            Self::Many(_) => None,
        }
    }
}

impl Serialize for SaintCount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Number(n) => serializer.serialize_u32(*n),
            Self::Many(s) => serializer.serialize_str(s),
        }
    }
}

impl<'de> Deserialize<'de> for SaintCount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde_json::Value;

        let value = Value::deserialize(deserializer)?;

        match value {
            Value::Number(n) => {
                if let Some(u) = n.as_u64() {
                    if u <= u32::MAX as u64 {
                        Ok(Self::Number(u as u32))
                    } else {
                        Err(serde::de::Error::custom(format!(
                            "number {} too large for u32 (maximum: {})",
                            u,
                            u32::MAX
                        )))
                    }
                } else {
                    Err(serde::de::Error::custom(format!(
                        "negative number {} not allowed for SaintCount",
                        n
                    )))
                }
            }
            Value::String(s) => {
                if s == "MANY" {
                    Ok(Self::Many(s))
                } else {
                    Err(serde::de::Error::custom(format!(
                        "expected 'MANY' or a number, got string: '{}'",
                        s
                    )))
                }
            }
            _ => Err(serde::de::Error::custom("expected 'MANY' or a number")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{
        Token, assert_de_tokens, assert_de_tokens_error, assert_ser_tokens, assert_tokens,
    };

    #[test]
    fn test_saint_count_serialization_tokens() {
        // Test serialization with tokens (recommended by Serde docs)
        assert_ser_tokens(&SaintCount::Number(42), &[Token::U32(42)]);
        assert_ser_tokens(&SaintCount::Number(0), &[Token::U32(0)]);
        assert_ser_tokens(&SaintCount::Number(u32::MAX), &[Token::U32(u32::MAX)]);
        assert_ser_tokens(&SaintCount::Many("MANY".to_string()), &[Token::Str("MANY")]);
    }

    #[test]
    fn test_saint_count_deserialization_tokens() {
        // Test deserialization with tokens
        assert_de_tokens(&SaintCount::Number(42), &[Token::U32(42)]);
        assert_de_tokens(&SaintCount::Number(0), &[Token::U32(0)]);
        assert_de_tokens(&SaintCount::Number(u32::MAX), &[Token::U32(u32::MAX)]);
        assert_de_tokens(&SaintCount::Many("MANY".to_string()), &[Token::Str("MANY")]);

        // Test with different numeric types
        assert_de_tokens(&SaintCount::Number(42), &[Token::U8(42)]);
        assert_de_tokens(&SaintCount::Number(42), &[Token::U16(42)]);
        assert_de_tokens(&SaintCount::Number(42), &[Token::I32(42)]);
        assert_de_tokens(&SaintCount::Number(42), &[Token::I64(42)]);
    }

    #[test]
    fn test_saint_count_roundtrip() {
        // Test complete roundtrip
        assert_tokens(&SaintCount::Number(42), &[Token::U32(42)]);
        assert_tokens(&SaintCount::Many("MANY".to_string()), &[Token::Str("MANY")]);
    }

    #[test]
    fn test_saint_count_deserialization_errors() {
        // Test deserialization errors
        assert_de_tokens_error::<SaintCount>(
            &[Token::Str("INVALID")],
            "expected 'MANY' or a number, got string: 'INVALID'",
        );

        assert_de_tokens_error::<SaintCount>(
            &[Token::U64(4294967296)], // u32::MAX + 1
            "number 4294967296 too large for u32 (maximum: 4294967295)",
        );

        assert_de_tokens_error::<SaintCount>(
            &[Token::I32(-1)],
            "negative number -1 not allowed for SaintCount",
        );

        assert_de_tokens_error::<SaintCount>(
            &[Token::I64(-1)],
            "negative number -1 not allowed for SaintCount",
        );

        assert_de_tokens_error::<SaintCount>(
            &[Token::I64(4294967296)], // u32::MAX + 1
            "number 4294967296 too large for u32 (maximum: 4294967295)",
        );
    }

    #[test]
    fn test_saint_count_json_compatibility() {
        // Test JSON compatibility (to ensure changes don't break anything)
        use serde_json;

        // Test JSON serialization
        let many = SaintCount::Many("MANY".to_string());
        let json = serde_json::to_string(&many).unwrap();
        assert_eq!(json, r#""MANY""#);

        let number = SaintCount::Number(42);
        let json = serde_json::to_string(&number).unwrap();
        assert_eq!(json, "42");

        // Test JSON deserialization
        let json_with_many = r#""MANY""#;
        let result: SaintCount = serde_json::from_str(json_with_many).unwrap();
        assert!(matches!(result, SaintCount::Many(s) if s == "MANY"));

        let json_with_number = r#"42"#;
        let result: SaintCount = serde_json::from_str(json_with_number).unwrap();
        assert!(matches!(result, SaintCount::Number(42)));

        // Test with invalid values
        let json_invalid = r#""INVALID""#;
        let result: Result<SaintCount, _> = serde_json::from_str(json_invalid);
        assert!(result.is_err());

        let json_too_large = r#"4294967296"#; // u32::MAX + 1
        let result: Result<SaintCount, _> = serde_json::from_str(json_too_large);
        assert!(result.is_err());
    }
}
