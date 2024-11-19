use crate::deserialize::unexpected::out_or_range_value;
use serde::de::{Deserializer, Error, Expected, Unexpected, Visitor};
use std::fmt;

const EXPECTED: &str = "an integer, a float, or a string";

pub fn as_i64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<i64, D::Error> {
    deserializer.deserialize_any(AsInt)
}

pub(super) fn parse_u64<E: Error>(v: u64, exp: &dyn Expected) -> Result<i64, E> {
    i64::try_from(v).map_err(|_| out_or_range_value(Unexpected::Unsigned(v), exp))
}

pub(super) fn parse_f64<E: Error>(v: f64, exp: &dyn Expected) -> Result<i64, E> {
    if v < i64::MIN as f64 || v > i64::MAX as f64 {
        Err(out_or_range_value(Unexpected::Float(v), exp))
    } else {
        Ok(v.round() as i64)
    }
}

pub(super) fn parse_str<E: Error>(v: &str, exp: &dyn Expected) -> Result<i64, E> {
    match v.parse::<i64>() {
        Ok(i64) => Ok(i64),
        _ => match v.parse::<f64>() {
            Ok(f64) => parse_f64(f64, exp),
            _ => Err(out_or_range_value(Unexpected::Str(v), exp))
        }
    }
}

struct AsInt;

impl<'de> Visitor<'de> for AsInt {
    type Value = i64;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(EXPECTED)
    }

    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> { Ok(v) }
    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> { parse_u64(v, &EXPECTED) }
    fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> { parse_f64(v, &EXPECTED) }
    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> { parse_str(v, &EXPECTED) }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::deserialize::unexpected::OUT_OF_RANGE_ERROR_BEGIN;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Test {
        #[serde(deserialize_with = "as_i64")]
        int: i64,
    }

    #[test]
    fn test_base_good_parse() {
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": -100         }"#).unwrap().int, -100);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": 100          }"#).unwrap().int, 100);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": 100.00       }"#).unwrap().int, 100);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": 100.1        }"#).unwrap().int, 100);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": 100.499      }"#).unwrap().int, 100);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": -100.499     }"#).unwrap().int, -100);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": 100.5        }"#).unwrap().int, 101);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": 100.999      }"#).unwrap().int, 101);

        assert_eq!(serde_json::from_str::<Test>(r#"{"int": "100"        }"#).unwrap().int, 100);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": "100.00"     }"#).unwrap().int, 100);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": "100.1"      }"#).unwrap().int, 100);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": "100.499"    }"#).unwrap().int, 100);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": "100.5"      }"#).unwrap().int, 101);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": "100.999"    }"#).unwrap().int, 101);

        assert_eq!(serde_json::from_str::<Test>(r#"{"int": "1.12e12"    }"#).unwrap().int, 1120000000000);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": "-1.12e12"   }"#).unwrap().int, -1120000000000);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": "11.1e5"     }"#).unwrap().int, 1110000);
    }

    #[test]
    fn test_base_error() {
        assert!(serde_json::from_str::<Test>(r#"{"int": ""        }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"int": "hello"   }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"int": "123a"    }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"int": "a123"    }"#).is_err());

        assert!(serde_json::from_str::<Test>(r#"{"int": "1.12z12" }"#).is_err()); // invalid format
        assert!(serde_json::from_str::<Test>(r#"{"int": "1.12e19" }"#).is_err()); // out range
    }

    #[test]
    fn test_parse_error_message() {
        // out fo range error messages must contains OUT_OF_RANGE_ERROR_BEGIN
        assert!(serde_json::from_str::<Test>(r#"{"int": 12134123412213421411     }"#).unwrap_err().to_string().contains(OUT_OF_RANGE_ERROR_BEGIN)); // u64 -> i64
        assert!(serde_json::from_str::<Test>(r#"{"int": 121341234122134214120000 }"#).unwrap_err().to_string().contains(OUT_OF_RANGE_ERROR_BEGIN)); // f64 -> i64
    }
}