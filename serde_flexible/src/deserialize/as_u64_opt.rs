use super::{as_u64};
use serde::de::{Deserializer, Error, Visitor};
use std::fmt;
use crate::deserialize::wrappers::str_wrap_as_opt;

const EXPECTED: &str = "null, an unsigned integer, or a string";

pub fn as_u64_opt<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<u64>, D::Error> {
    deserializer.deserialize_any(AsOptUInt)
}

struct AsOptUInt;

impl<'de> Visitor<'de> for AsOptUInt {
    type Value = Option<u64>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(EXPECTED)
    }

    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> { as_u64::parse_i64(v, &EXPECTED).map(Some) }
    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> { Ok(v).map(Some) }
    fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> { as_u64::parse_f64(v, &EXPECTED).map(Some) }
    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> { str_wrap_as_opt(v, &EXPECTED, as_u64::parse_str) }
    fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use crate::deserialize::unexpected::OUT_OF_RANGE_ERROR_BEGIN;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Test {
        #[serde(deserialize_with = "as_u64_opt")]
        int: Option<u64>,
    }

    #[test]
    fn test_base_good_parse() {
        assert!(serde_json::from_str::<Test>(r#"{"int": "null"          }"#).unwrap().int.is_none());
        assert!(serde_json::from_str::<Test>(r#"{"int": "none"          }"#).unwrap().int.is_none());
        assert!(serde_json::from_str::<Test>(r#"{"int": "unknown"       }"#).unwrap().int.is_none());
        assert!(serde_json::from_str::<Test>(r#"{"int": "Unknown"       }"#).unwrap().int.is_none());
        assert!(serde_json::from_str::<Test>(r#"{"int": "NONE"          }"#).unwrap().int.is_none());
        assert!(serde_json::from_str::<Test>(r#"{"int": null            }"#).unwrap().int.is_none());

        assert_eq!(serde_json::from_str::<Test>(r#"{"int": 100          }"#).unwrap().int.unwrap(), 100);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": 100.00       }"#).unwrap().int.unwrap(), 100);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": 100.1        }"#).unwrap().int.unwrap(), 100);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": 100.499      }"#).unwrap().int.unwrap(), 100);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": 100.5        }"#).unwrap().int.unwrap(), 101);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": 100.999      }"#).unwrap().int.unwrap(), 101);

        assert_eq!(serde_json::from_str::<Test>(r#"{"int": "100"        }"#).unwrap().int.unwrap(), 100);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": "100.00"     }"#).unwrap().int.unwrap(), 100);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": "100.1"      }"#).unwrap().int.unwrap(), 100);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": "100.499"    }"#).unwrap().int.unwrap(), 100);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": "100.5"      }"#).unwrap().int.unwrap(), 101);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": "100.999"    }"#).unwrap().int.unwrap(), 101);

        assert_eq!(serde_json::from_str::<Test>(r#"{"int": "1.12e12"    }"#).unwrap().int.unwrap(), 1120000000000);
        assert_eq!(serde_json::from_str::<Test>(r#"{"int": "11.1e5"     }"#).unwrap().int.unwrap(), 1110000);
    }

    #[test]
    fn test_base_error() {
        assert!(serde_json::from_str::<Test>(r#"{"int": ""        }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"int": "hello"   }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"int": "123a"    }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"int": "a123"    }"#).is_err());

        assert!(serde_json::from_str::<Test>(r#"{"int": "1.12z12" }"#).is_err()); // invalid format
        assert!(serde_json::from_str::<Test>(r#"{"int": "1.12e20" }"#).is_err()); // out range
        assert!(serde_json::from_str::<Test>(r#"{"int": "-1.12e20" }"#).is_err()); // out range
    }

    #[test]
    fn test_parse_error_message() {
        // out fo range error messages must contains OUT_OF_RANGE_ERROR_BEGIN
        assert!(serde_json::from_str::<Test>(r#"{"int": -1     }"#).unwrap_err().to_string().contains(OUT_OF_RANGE_ERROR_BEGIN)); // i64 -> u64
        assert!(serde_json::from_str::<Test>(r#"{"int": 121341234122134214120000 }"#).unwrap_err().to_string().contains(OUT_OF_RANGE_ERROR_BEGIN)); // f64 -> u64
    }
}