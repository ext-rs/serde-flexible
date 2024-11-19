use serde::de::{Deserializer, Error, Visitor};
use std::fmt;

const EXPECTED: &str = "null, a string, bool, or a number";

pub fn as_string_opt<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<String>, D::Error> {
    deserializer.deserialize_any(AsOptString)
}

struct AsOptString;

impl<'de> Visitor<'de> for AsOptString {
    type Value = Option<String>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(EXPECTED)
    }

    fn visit_bool<E: Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(Some(v.to_string()))
    }
    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
        Ok(Some(v.to_string()))
    }
    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(Some(v.to_string()))
    }
    fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> {
        Ok(Some(v.to_string()))
    }
    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(Some(v.to_owned()))
    }
    fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Test {
        #[serde(deserialize_with = "as_string_opt")]
        str: Option<String>,
    }

    #[test]
    fn test_base_good_parse() {
        // important: str no use fn str_wrap_as_opt() and will not parse to none for strings like "null", "none", "unknown" ...
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": "null"      }"#).unwrap().str.unwrap(), "null");
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": "none"      }"#).unwrap().str.unwrap(), "none");
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": "unknown"   }"#).unwrap().str.unwrap(), "unknown");
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": "Unknown"   }"#).unwrap().str.unwrap(), "Unknown");
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": "NONE"      }"#).unwrap().str.unwrap(), "NONE");
        assert!(serde_json::from_str::<Test>(r#"{"str": null         }"#).unwrap().str.is_none());

        assert_eq!(serde_json::from_str::<Test>(r#"{"str": "Foo Boo" }"#).unwrap().str.unwrap(), "Foo Boo");
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": "hello"   }"#).unwrap().str.unwrap(), "hello");
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": 100       }"#).unwrap().str.unwrap(), "100");
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": true      }"#).unwrap().str.unwrap(), "true");
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": false     }"#).unwrap().str.unwrap(), "false");
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": 12345     }"#).unwrap().str.unwrap(), "12345");
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": -12345    }"#).unwrap().str.unwrap(), "-12345");
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": -0        }"#).unwrap().str.unwrap(), "-0");
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": 0         }"#).unwrap().str.unwrap(), "0");
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": 3.14      }"#).unwrap().str.unwrap(), "3.14");

        assert!(serde_json::from_str::<Test>(r#"{"str": null         }"#).unwrap().str.is_none());
    }

    #[test]
    fn test_base_error() {
        assert!(serde_json::from_str::<Test>(r#"{"str": ["hello"]]   }"#).is_err());
    }
}