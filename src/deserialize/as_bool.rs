use serde::de::{Deserializer, Error, Expected, Unexpected, Visitor};
use std::fmt;

const EXPECTED: &dyn Expected = &"an integer (0 or 1) or a case insensitive string (true/false, yes/no, on/off, y/n, t/f, 1/0, ok)";

pub fn as_bool<'de, D: Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    deserializer.deserialize_any(Convertor)
}

struct Convertor;

impl<'de> Visitor<'de> for Convertor {
    type Value = bool;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(EXPECTED.to_string().as_str())
    }

    fn visit_bool<E: Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(v)
    }

    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
        match v {
            0 => Ok(false),
            1 => Ok(true),
            other => Err(Error::invalid_value(Unexpected::Signed(other), EXPECTED)),
        }
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        match v {
            0 => Ok(false),
            1 => Ok(true),
            other => Err(Error::invalid_value(Unexpected::Unsigned(other), EXPECTED)),
        }
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        match v {
            "1" | "true" | "True" => Ok(true),
            "0" | "false" | "False" => Ok(false),
            other => {
                match other.to_lowercase().as_str() {
                    "true" | "yes" | "on" | "y" | "t" | "ok" => Ok(true),
                    "false" | "no" | "off" | "n" | "f" => Ok(false),
                    _ => Err(Error::invalid_value(Unexpected::Str(v), EXPECTED)),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Deserialize, PartialEq)]
    struct Test {
        #[serde(deserialize_with = "as_bool")]
        bool: bool,
    }

    #[test]
    fn test_base_good_parse() {
        assert!(serde_json::from_str::<Test>(r#"{"bool": true     }"#).unwrap().bool);
        assert!(serde_json::from_str::<Test>(r#"{"bool": 1        }"#).unwrap().bool);
        assert!(serde_json::from_str::<Test>(r#"{"bool": "1"      }"#).unwrap().bool);
        assert!(serde_json::from_str::<Test>(r#"{"bool": "true"   }"#).unwrap().bool);
        assert!(serde_json::from_str::<Test>(r#"{"bool": "True"   }"#).unwrap().bool);
        assert!(serde_json::from_str::<Test>(r#"{"bool": "TRue"   }"#).unwrap().bool);
        assert!(serde_json::from_str::<Test>(r#"{"bool": "TRUE"   }"#).unwrap().bool);
        assert!(serde_json::from_str::<Test>(r#"{"bool": "yes"    }"#).unwrap().bool);
        assert!(serde_json::from_str::<Test>(r#"{"bool": "Yes"    }"#).unwrap().bool);
        assert!(serde_json::from_str::<Test>(r#"{"bool": "YeS"    }"#).unwrap().bool);
        assert!(serde_json::from_str::<Test>(r#"{"bool": "on"     }"#).unwrap().bool);
        assert!(serde_json::from_str::<Test>(r#"{"bool": "On"     }"#).unwrap().bool);
        assert!(serde_json::from_str::<Test>(r#"{"bool": "ON"     }"#).unwrap().bool);
        assert!(serde_json::from_str::<Test>(r#"{"bool": "oN"     }"#).unwrap().bool);
        assert!(serde_json::from_str::<Test>(r#"{"bool": "y"      }"#).unwrap().bool);
        assert!(serde_json::from_str::<Test>(r#"{"bool": "Y"      }"#).unwrap().bool);
        assert!(serde_json::from_str::<Test>(r#"{"bool": "t"      }"#).unwrap().bool);
        assert!(serde_json::from_str::<Test>(r#"{"bool": "T"      }"#).unwrap().bool);
        assert!(serde_json::from_str::<Test>(r#"{"bool": "ok"     }"#).unwrap().bool);
        assert!(serde_json::from_str::<Test>(r#"{"bool": "Ok"     }"#).unwrap().bool);
        assert!(serde_json::from_str::<Test>(r#"{"bool": "OK"     }"#).unwrap().bool);
        assert!(serde_json::from_str::<Test>(r#"{"bool": "oK"     }"#).unwrap().bool);

        assert!(!serde_json::from_str::<Test>(r#"{"bool": false   }"#).unwrap().bool);
        assert!(!serde_json::from_str::<Test>(r#"{"bool": 0       }"#).unwrap().bool);
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "0"     }"#).unwrap().bool);
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "false" }"#).unwrap().bool);
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "False" }"#).unwrap().bool);
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "FalsE" }"#).unwrap().bool);
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "FALSE" }"#).unwrap().bool);
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "no"   }"#).unwrap().bool);
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "NO"   }"#).unwrap().bool);
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "nO"   }"#).unwrap().bool);
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "No"   }"#).unwrap().bool);
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "off"   }"#).unwrap().bool);
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "Off"   }"#).unwrap().bool);
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "OfF"   }"#).unwrap().bool);
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "n"   }"#).unwrap().bool);
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "N"   }"#).unwrap().bool);
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "f"   }"#).unwrap().bool);
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "F"   }"#).unwrap().bool);
    }

    #[test]
    fn test_parse_error() {
        assert!(serde_json::from_str::<Test>(r#"{"bool": null          }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"bool": 2             }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"bool": -1            }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"bool": 1.0           }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"bool": 0.0           }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"bool": 3.14          }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"bool": "false 100%"  }"#).is_err());
    }

    #[test]
    fn test_parse_error_message() {
        assert!(serde_json::from_str::<Test>(r#"{"bool": null}"#).unwrap_err().to_string().contains(EXPECTED.to_string().as_str()));
        assert!(serde_json::from_str::<Test>(r#"{"bool": ["first", "second"]}"#).unwrap_err().to_string().contains(EXPECTED.to_string().as_str()));
        assert!(serde_json::from_str::<Test>(r#"{"bool": -100}"#).unwrap_err().to_string().contains(EXPECTED.to_string().as_str()));
        assert!(serde_json::from_str::<Test>(r#"{"bool": "unknown"}"#).unwrap_err().to_string().contains(EXPECTED.to_string().as_str()));
    }
}