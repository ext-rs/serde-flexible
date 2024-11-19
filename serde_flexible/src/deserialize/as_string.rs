use serde::de::{Deserializer, Error, Visitor};
use std::fmt;

const EXPECTED: &str = "a string, bool, or a number";

pub fn as_string<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    deserializer.deserialize_any(AsString)
}

struct AsString;

impl<'de> Visitor<'de> for AsString {
    type Value = String;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(EXPECTED)
    }

    fn visit_bool<E: Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(v.to_string())
    }
    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
        Ok(v.to_string())
    }
    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(v.to_string())
    }
    fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> {
        Ok(v.to_string())
    }
    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        Ok(v.to_owned())
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Test {
        #[serde(deserialize_with = "as_string")]
        str: String,
    }

    #[test]
    fn test_base_good_parse() {
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": "Foo Boo" }"#).unwrap().str, "Foo Boo");
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": "hello"   }"#).unwrap().str, "hello");
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": 100       }"#).unwrap().str, "100");
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": true      }"#).unwrap().str, "true");
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": false     }"#).unwrap().str, "false");
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": 12345     }"#).unwrap().str, "12345");
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": -12345    }"#).unwrap().str, "-12345");
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": -0        }"#).unwrap().str, "-0");
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": 0         }"#).unwrap().str, "0");
        assert_eq!(serde_json::from_str::<Test>(r#"{"str": 3.14      }"#).unwrap().str, "3.14");
    }

    #[test]
    fn test_base_error() {
        assert!(serde_json::from_str::<Test>(r#"{"str": null         }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"str": ["hello"]]   }"#).is_err());
    }
}