use serde::de::{Deserializer, Error, Unexpected, Visitor};
use std::fmt;

const GLOBAL_EXP: &str = "a float or a string";

pub fn as_f64<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f64, D::Error> {
    deserializer.deserialize_any(Convertor)
}

struct Convertor;

impl<'de> Visitor<'de> for Convertor {
    type Value = f64;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(GLOBAL_EXP)
    }

    fn visit_i64<E: Error>(self, v: i64) -> Result<Self::Value, E> { Ok(v as f64) }

    fn visit_u64<E: Error>(self, v: u64) -> Result<Self::Value, E> { Ok(v as f64) }

    fn visit_f64<E: Error>(self, v: f64) -> Result<Self::Value, E> { Ok(v) }

    fn visit_str<E: Error>(self, v: &str) -> Result<Self::Value, E> {
        if let Ok(f) = v.parse::<f64>() {
            Ok(f)
        } else if v.is_empty() {
            Ok(0.0)
        } else {
            Err(E::invalid_value(Unexpected::Str(v), &self))
        }
    }
}