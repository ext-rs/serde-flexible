use serde::de::{Deserializer, Error, Expected, Unexpected, Visitor};
use std::fmt;

const EXPECTED: &dyn Expected = &"null or an integer (0 or 1) or a case insensitive string (true/false, yes/no, y/n, t/f, 1/0, on/off, ok)";

pub fn as_bool_opt<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<bool>, D::Error> {
    deserializer.deserialize_any(Convertor)
}

struct Convertor;

impl<'de> Visitor<'de> for Convertor {
    type Value = Option<bool>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(EXPECTED.to_string().as_str())
    }

    fn visit_bool<E: Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(Some(v))
    }

    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
        match v {
            0 => Ok(Some(false)),
            1 => Ok(Some(true)),
            other => Err(Error::invalid_value(Unexpected::Signed(other), EXPECTED)),
        }
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        match v {
            0 => Ok(Some(false)),
            1 => Ok(Some(true)),
            other => Err(Error::invalid_value(Unexpected::Unsigned(other), EXPECTED)),
        }
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        match v {
            "1" | "true" | "True" => Ok(Some(true)),
            "0" | "false" | "False" => Ok(Some(false)),
            other => {
                match other.to_lowercase().as_str() {
                    "true" | "yes" | "on" | "y" | "t" | "ok" => Ok(Some(true)),
                    "false" | "no" | "off" | "n" | "f" => Ok(Some(false)),
                    _ => Err(Error::invalid_value(Unexpected::Str(v), EXPECTED)),
                }
            }
        }
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
        #[serde(deserialize_with = "as_bool_opt")]
        bool: Option<bool>,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestDefNull {
        #[serde(default)]
        #[serde(deserialize_with = "as_bool_opt")]
        bool: Option<bool>,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct TestDefTrue {
        #[serde(default = "default_true")]
        #[serde(deserialize_with = "as_bool_opt")]
        bool: Option<bool>,
    }

    fn default_true() -> Option<bool> { Some(true) }

    #[test]
    fn test_base_good_parse() {
        assert!(serde_json::from_str::<Test>(r#"{"bool": null     }"#).unwrap().bool.is_none());
        assert!(serde_json::from_str::<Test>(r#"{"bool": true     }"#).unwrap().bool.unwrap());
        assert!(serde_json::from_str::<Test>(r#"{"bool": 1        }"#).unwrap().bool.unwrap());
        assert!(serde_json::from_str::<Test>(r#"{"bool": "1"      }"#).unwrap().bool.unwrap());
        assert!(serde_json::from_str::<Test>(r#"{"bool": "true"   }"#).unwrap().bool.unwrap());
        assert!(serde_json::from_str::<Test>(r#"{"bool": "True"   }"#).unwrap().bool.unwrap());
        assert!(serde_json::from_str::<Test>(r#"{"bool": "TRue"   }"#).unwrap().bool.unwrap());
        assert!(serde_json::from_str::<Test>(r#"{"bool": "TRUE"   }"#).unwrap().bool.unwrap());
        assert!(serde_json::from_str::<Test>(r#"{"bool": "yes"    }"#).unwrap().bool.unwrap());
        assert!(serde_json::from_str::<Test>(r#"{"bool": "Yes"    }"#).unwrap().bool.unwrap());
        assert!(serde_json::from_str::<Test>(r#"{"bool": "YeS"    }"#).unwrap().bool.unwrap());
        assert!(serde_json::from_str::<Test>(r#"{"bool": "on"     }"#).unwrap().bool.unwrap());
        assert!(serde_json::from_str::<Test>(r#"{"bool": "On"     }"#).unwrap().bool.unwrap());
        assert!(serde_json::from_str::<Test>(r#"{"bool": "ON"     }"#).unwrap().bool.unwrap());
        assert!(serde_json::from_str::<Test>(r#"{"bool": "oN"     }"#).unwrap().bool.unwrap());
        assert!(serde_json::from_str::<Test>(r#"{"bool": "y"      }"#).unwrap().bool.unwrap());
        assert!(serde_json::from_str::<Test>(r#"{"bool": "Y"      }"#).unwrap().bool.unwrap());
        assert!(serde_json::from_str::<Test>(r#"{"bool": "t"      }"#).unwrap().bool.unwrap());
        assert!(serde_json::from_str::<Test>(r#"{"bool": "T"      }"#).unwrap().bool.unwrap());
        assert!(serde_json::from_str::<Test>(r#"{"bool": "ok"     }"#).unwrap().bool.unwrap());
        assert!(serde_json::from_str::<Test>(r#"{"bool": "Ok"     }"#).unwrap().bool.unwrap());
        assert!(serde_json::from_str::<Test>(r#"{"bool": "OK"     }"#).unwrap().bool.unwrap());
        assert!(serde_json::from_str::<Test>(r#"{"bool": "oK"     }"#).unwrap().bool.unwrap());

        assert!(!serde_json::from_str::<Test>(r#"{"bool": false   }"#).unwrap().bool.unwrap());
        assert!(!serde_json::from_str::<Test>(r#"{"bool": 0       }"#).unwrap().bool.unwrap());
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "0"     }"#).unwrap().bool.unwrap());
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "false" }"#).unwrap().bool.unwrap());
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "False" }"#).unwrap().bool.unwrap());
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "FalsE" }"#).unwrap().bool.unwrap());
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "FALSE" }"#).unwrap().bool.unwrap());
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "no"    }"#).unwrap().bool.unwrap());
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "NO"    }"#).unwrap().bool.unwrap());
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "nO"    }"#).unwrap().bool.unwrap());
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "No"    }"#).unwrap().bool.unwrap());
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "off"   }"#).unwrap().bool.unwrap());
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "Off"   }"#).unwrap().bool.unwrap());
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "OfF"   }"#).unwrap().bool.unwrap());
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "n"     }"#).unwrap().bool.unwrap());
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "N"     }"#).unwrap().bool.unwrap());
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "f"     }"#).unwrap().bool.unwrap());
        assert!(!serde_json::from_str::<Test>(r#"{"bool": "F"     }"#).unwrap().bool.unwrap());
    }

    #[test]
    fn test_default() {
        assert!(serde_json::from_str::<TestDefNull>(r#"{"some": "foo"}"#).unwrap().bool.is_none());
        assert!(serde_json::from_str::<TestDefTrue>(r#"{"some": "foo"}"#).unwrap().bool.unwrap());
    }

    #[test]
    fn test_parse_error() {
        assert!(serde_json::from_str::<Test>(r#"{"bool": 2             }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"bool": -1            }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"bool": 1.0           }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"bool": 0.0           }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"bool": 3.14          }"#).is_err());
        assert!(serde_json::from_str::<Test>(r#"{"bool": "false 100%"  }"#).is_err());
    }

    #[test]
    fn test_parse_error_message() {
        assert!(serde_json::from_str::<Test>(r#"{"bool": ["first", "second"]}"#).unwrap_err().to_string().contains(EXPECTED.to_string().as_str()));
        assert!(serde_json::from_str::<Test>(r#"{"bool": -100}"#).unwrap_err().to_string().contains(EXPECTED.to_string().as_str()));
        assert!(serde_json::from_str::<Test>(r#"{"bool": "unknown"}"#).unwrap_err().to_string().contains(EXPECTED.to_string().as_str()));
    }
}