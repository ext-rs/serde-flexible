use serde::de::{Deserializer, Error, Expected, Unexpected, Visitor};
use std::fmt;

const EXPECTED: &str = "a float, an integer, or a string";

pub fn as_f64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
    deserializer.deserialize_any(AsFloat)
}

struct AsFloat;

impl<'de> Visitor<'de> for AsFloat {
    type Value = f64;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result { formatter.write_str(EXPECTED) }
    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> { parse_i64(v) }
    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> { parse_u64(v) }
    fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> { Ok(v) }
    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> { parse_str(v, &EXPECTED) }
}

pub(super) fn parse_i64<E: Error>(v: i64) -> Result<f64, E> {
    Ok(v as f64)
}

pub(super) fn parse_u64<E: Error>(v: u64) -> Result<f64, E> {
    Ok(v as f64)
}

pub(super) fn parse_str<E: Error>(v: &str, exp: &dyn Expected) -> Result<f64, E> {
    match v.parse::<f64>() {
        Ok(f) => Ok(f),
        Err(_) => Err(E::invalid_value(Unexpected::Str(v), exp)),
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Test {
        #[serde(deserialize_with = "as_f64")]
        float: f64,
    }

    #[test]
    fn test_base_good_parse() {
        assert_eq!(serde_json::from_str::<Test>(r#"{"float": 3.14     }"#).unwrap().float, 3.14);
        assert_eq!(serde_json::from_str::<Test>(r#"{"float": "3.14"   }"#).unwrap().float, 3.14);
        assert_eq!(serde_json::from_str::<Test>(r#"{"float": -3.14    }"#).unwrap().float, -3.14);
        assert_eq!(serde_json::from_str::<Test>(r#"{"float": "-3.14"  }"#).unwrap().float, -3.14);

        assert_eq!(serde_json::from_str::<Test>(r#"{"float": 3        }"#).unwrap().float, 3.0);
        assert_eq!(serde_json::from_str::<Test>(r#"{"float": "3"      }"#).unwrap().float, 3.0);
        assert_eq!(serde_json::from_str::<Test>(r#"{"float": 3.0      }"#).unwrap().float, 3.0);
        assert_eq!(serde_json::from_str::<Test>(r#"{"float": -3       }"#).unwrap().float, -3.0);
        assert_eq!(serde_json::from_str::<Test>(r#"{"float": "-3"     }"#).unwrap().float, -3.0);
        assert_eq!(serde_json::from_str::<Test>(r#"{"float": -3.0     }"#).unwrap().float, -3.0);
    }

    #[test]
    fn test_base_errors() {
        assert!(serde_json::from_str::<Test>(r#"{"float": null   }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"float": false  }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"float": true   }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"float": "abc"  }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"float": ""     }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"foo": 3.14     }"#).is_err());
    }

    #[test]
    fn test_parse_error_message() {
        assert!(serde_json::from_str::<Test>(r#"{"float": null}"#).unwrap_err().to_string().contains(EXPECTED));
    }
}