use crate::deserialize::as_f64;
use crate::deserialize::wrappers::str_wrap_as_opt;
use serde::de::{Deserializer, Error, Visitor};
use std::fmt;

const EXPECTED: &str = "null, a float, an integer, or a string";

pub fn as_f64_opt<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<f64>, D::Error> {
    deserializer.deserialize_any(AsOptFloat)
}

struct AsOptFloat;

impl<'de> Visitor<'de> for AsOptFloat {
    type Value = Option<f64>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result { formatter.write_str(EXPECTED) }
    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> { as_f64::parse_i64(v).map(Some) }
    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> { as_f64::parse_u64(v).map(Some) }
    fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> { Ok(v).map(Some) }
    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> { str_wrap_as_opt(v, &EXPECTED, as_f64::parse_str) }
    fn visit_unit<E: Error>(self) -> Result<Self::Value, E> { Ok(None) }
}


#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Test {
        #[serde(deserialize_with = "as_f64_opt")]
        float: Option<f64>,
    }

    #[test]
    fn test_base_good_parse() {
        assert!(serde_json::from_str::<Test>(r#"{"float": "null"      }"#).unwrap().float.is_none());
        assert!(serde_json::from_str::<Test>(r#"{"float": "none"      }"#).unwrap().float.is_none());
        assert!(serde_json::from_str::<Test>(r#"{"float": "unknown"   }"#).unwrap().float.is_none());
        assert!(serde_json::from_str::<Test>(r#"{"float": "Unknown"   }"#).unwrap().float.is_none());
        assert!(serde_json::from_str::<Test>(r#"{"float": "NONE"      }"#).unwrap().float.is_none());
        assert!(serde_json::from_str::<Test>(r#"{"float": null        }"#).unwrap().float.is_none());

        assert_eq!(serde_json::from_str::<Test>(r#"{"float": 3.14     }"#).unwrap().float.unwrap(), 3.14);
        assert_eq!(serde_json::from_str::<Test>(r#"{"float": "3.14"   }"#).unwrap().float.unwrap(), 3.14);
        assert_eq!(serde_json::from_str::<Test>(r#"{"float": -3.14    }"#).unwrap().float.unwrap(), -3.14);
        assert_eq!(serde_json::from_str::<Test>(r#"{"float": "-3.14"  }"#).unwrap().float.unwrap(), -3.14);

        assert_eq!(serde_json::from_str::<Test>(r#"{"float": 3        }"#).unwrap().float.unwrap(), 3.0);
        assert_eq!(serde_json::from_str::<Test>(r#"{"float": "3"      }"#).unwrap().float.unwrap(), 3.0);
        assert_eq!(serde_json::from_str::<Test>(r#"{"float": 3.0      }"#).unwrap().float.unwrap(), 3.0);
        assert_eq!(serde_json::from_str::<Test>(r#"{"float": -3       }"#).unwrap().float.unwrap(), -3.0);
        assert_eq!(serde_json::from_str::<Test>(r#"{"float": "-3"     }"#).unwrap().float.unwrap(), -3.0);
        assert_eq!(serde_json::from_str::<Test>(r#"{"float": -3.0     }"#).unwrap().float.unwrap(), -3.0);
    }

    #[test]
    fn test_base_errors() {
        assert!(serde_json::from_str::<Test>(r#"{"float": ["hello"]]  }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"float": false       }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"float": true        }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"float": "abc"       }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"float": ""          }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"foo": 3.14          }"#).is_err());
    }

    #[test]
    fn test_parse_error_message() {
        assert!(serde_json::from_str::<Test>(r#"{"float": ["hello"]}"#).unwrap_err().to_string().contains(EXPECTED));
    }
}