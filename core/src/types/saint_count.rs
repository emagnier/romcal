use schemars::JsonSchema;

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
#[derive(Debug, Clone, PartialEq, Eq, JsonSchema)]
#[serde(untagged)]
pub enum SaintCount {
    /// Specific number of saints
    Number(u32),
    /// Indeterminate number of saints
    Many,
}

impl serde::Serialize for SaintCount {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            SaintCount::Number(n) => serializer.serialize_u32(*n),
            SaintCount::Many => serializer.serialize_str("MANY"),
        }
    }
}

impl<'de> serde::Deserialize<'de> for SaintCount {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, Visitor};
        use std::fmt;

        struct SaintCountVisitor;

        impl<'de> Visitor<'de> for SaintCountVisitor {
            type Value = SaintCount;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a number between 0 and 4294967295 or the string 'MANY'")
            }

            fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E> {
                Ok(SaintCount::Number(value as u32))
            }

            fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E> {
                Ok(SaintCount::Number(value as u32))
            }

            fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E> {
                Ok(SaintCount::Number(value))
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value <= u32::MAX as u64 {
                    Ok(SaintCount::Number(value as u32))
                } else {
                    Err(de::Error::custom(format!(
                        "number {} too large for u32 (maximum: {})",
                        value,
                        u32::MAX
                    )))
                }
            }

            fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value >= 0 {
                    Ok(SaintCount::Number(value as u32))
                } else {
                    Err(de::Error::custom(format!(
                        "negative number {} not allowed for SaintCount",
                        value
                    )))
                }
            }

            fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value >= 0 {
                    Ok(SaintCount::Number(value as u32))
                } else {
                    Err(de::Error::custom(format!(
                        "negative number {} not allowed for SaintCount",
                        value
                    )))
                }
            }

            fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value >= 0 {
                    Ok(SaintCount::Number(value as u32))
                } else {
                    Err(de::Error::custom(format!(
                        "negative number {} not allowed for SaintCount",
                        value
                    )))
                }
            }

            fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value >= 0 && value <= u32::MAX as i64 {
                    Ok(SaintCount::Number(value as u32))
                } else if value < 0 {
                    Err(de::Error::custom(format!(
                        "negative number {} not allowed for SaintCount",
                        value
                    )))
                } else {
                    Err(de::Error::custom(format!(
                        "number {} too large for u32 (maximum: {})",
                        value,
                        u32::MAX
                    )))
                }
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value == "MANY" {
                    Ok(SaintCount::Many)
                } else {
                    Err(de::Error::custom(format!(
                        "expected 'MANY' or a number, got string: '{}'",
                        value
                    )))
                }
            }

            fn visit_string<E>(self, value: String) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                self.visit_str(&value)
            }
        }

        deserializer.deserialize_any(SaintCountVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_test::{
        assert_de_tokens, assert_de_tokens_error, assert_ser_tokens, assert_tokens, Token,
    };

    #[test]
    fn test_saint_count_serialization_tokens() {
        // Test serialization with tokens (recommended by Serde docs)
        assert_ser_tokens(&SaintCount::Number(42), &[Token::U32(42)]);
        assert_ser_tokens(&SaintCount::Number(0), &[Token::U32(0)]);
        assert_ser_tokens(&SaintCount::Number(u32::MAX), &[Token::U32(u32::MAX)]);
        assert_ser_tokens(&SaintCount::Many, &[Token::Str("MANY")]);
    }

    #[test]
    fn test_saint_count_deserialization_tokens() {
        // Test deserialization with tokens
        assert_de_tokens(&SaintCount::Number(42), &[Token::U32(42)]);
        assert_de_tokens(&SaintCount::Number(0), &[Token::U32(0)]);
        assert_de_tokens(&SaintCount::Number(u32::MAX), &[Token::U32(u32::MAX)]);
        assert_de_tokens(&SaintCount::Many, &[Token::Str("MANY")]);

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
        assert_tokens(&SaintCount::Many, &[Token::Str("MANY")]);
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
        let many = SaintCount::Many;
        let json = serde_json::to_string(&many).unwrap();
        assert_eq!(json, r#""MANY""#);

        let number = SaintCount::Number(42);
        let json = serde_json::to_string(&number).unwrap();
        assert_eq!(json, "42");

        // Test JSON deserialization
        let json_with_many = r#""MANY""#;
        let result: SaintCount = serde_json::from_str(json_with_many).unwrap();
        assert!(matches!(result, SaintCount::Many));

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
