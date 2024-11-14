use serde::de::{Deserializer, Error, Unexpected, Visitor};
use std::fmt;

const GLOBAL_EXP: &str = "an unsigned integer or a string";

pub fn as_u64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u64, D::Error> {
    deserializer.deserialize_any(Convertor)
}

struct Convertor;

impl<'de> Visitor<'de> for Convertor {
    type Value = u64;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(GLOBAL_EXP)
    }

    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
        match u64::try_from(v) {
            Ok(v) => Ok(v),
            Err(_) => Err(E::custom(format!(
                "overflow: Unable to convert signed value `{v:?}` to u64"
            ))),
        }
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(v)
    }

    fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> {
        Ok(v.round() as u64)
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        if let Ok(n) = v.parse::<u64>() {
            Ok(n)
        } else if v.is_empty() {
            Ok(0)
        } else if let Ok(f) = v.parse::<f64>() {
            Ok(f.round() as u64)
        } else {
            Err(E::invalid_value(Unexpected::Str(v), &self))
        }
    }

    fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
        Ok(0)
    }
}