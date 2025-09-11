use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, JsonSchema)]
pub enum SaintCount {
    Number(u32),
    Many,
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
                formatter.write_str("a number or the string 'MANY'")
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value <= u32::MAX as u64 {
                    Ok(SaintCount::Number(value as u32))
                } else {
                    Err(de::Error::custom("number too large for u32"))
                }
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                if value == "MANY" {
                    Ok(SaintCount::Many)
                } else {
                    Err(de::Error::custom("expected 'MANY' or a number"))
                }
            }
        }

        deserializer.deserialize_any(SaintCountVisitor)
    }
}
