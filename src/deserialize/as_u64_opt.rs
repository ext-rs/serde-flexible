use serde::de::{Deserializer, Error, Visitor};
use std::fmt;

const GLOBAL_EXP: &str = "null, an unsigned integer, or a string";

pub fn as_u64_opt<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<u64>, D::Error> {
    deserializer.deserialize_any(Convertor)
}

struct Convertor;

impl<'de> Visitor<'de> for Convertor {
    type Value = Option<u64>;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(GLOBAL_EXP)
    }

    fn visit_bool<E: Error>(self, _: bool) -> Result<Self::Value, E> {
        Ok(None)
    }

    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> {
        Ok(u64::try_from(v).ok())
    }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> {
        Ok(Some(v))
    }

    fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> {
        Ok(Some(v.round() as u64))
    }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        if let Ok(n) = v.parse::<u64>() {
            Ok(Some(n))
        } else if v.is_empty() {
            Ok(None)
        } else if let Ok(f) = v.parse::<f64>() {
            Ok(Some(f.round() as u64))
        } else {
            Ok(None)
        }
    }

    fn visit_unit<E: Error>(self) -> Result<Self::Value, E> {
        Ok(None)
    }
}