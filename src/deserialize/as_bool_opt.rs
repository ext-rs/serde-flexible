use serde::de::{Deserializer, Error, Expected, Unexpected, Visitor};
use std::fmt;

const NUMBER_EXP: &dyn Expected = &"null or 0 or 1";
const STRING_EXP: &dyn Expected = &"null or true/false, yes/no, y/n, t/f, 1/0, on/off, ok";
const GLOBAL_EXP: &str = "null or an integer (0 or 1) or a case insensitive string (true/false, yes/no, y/n, t/f, 1/0, on/off, ok)";

pub fn as_bool_opt<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<bool>, D::Error> {
    deserializer.deserialize_any(Convertor)
}

struct Convertor;

impl<'de> Visitor<'de> for Convertor {
    type Value = Option<bool>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(GLOBAL_EXP)
    }

    fn visit_bool<E: Error>(self, v: bool) -> Result<Self::Value, E> {
        Ok(Some(v))
    }

    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
        match v {
            0 => Ok(Some(false)),
            1 => Ok(Some(true)),
            other => Err(Error::invalid_value(Unexpected::Signed(other), NUMBER_EXP)),
        }
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        match v {
            0 => Ok(Some(false)),
            1 => Ok(Some(true)),
            other => Err(Error::invalid_value(Unexpected::Unsigned(other), NUMBER_EXP)),
        }
    }

    fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> {
        match v as u8 {
            0 => Ok(Some(false)),
            1 => Ok(Some(true)),
            _ => Err(Error::invalid_value(Unexpected::Float(v), NUMBER_EXP)),
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
                    _ => Ok(None),
                }
            }
        }
    }

    fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }
}