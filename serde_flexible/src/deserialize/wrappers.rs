use serde::de::{Unexpected, Error, Expected};

pub(super) fn str_wrap_as_opt<T, E, F>(
    v: &str,
    exp: &dyn Expected,
    parser: F,
) -> Result<Option<T>, E>
where
    F: Fn(&str, &dyn Expected) -> Result<T, E>,
    E: Error
{
    match v {
        "null" => Ok(None),
        _ =>
            match parser(v, exp) {
                Ok(f) => Ok(Some(f)),
                Err(_) => match v.to_lowercase().as_str() {
                    "null" | "none" | "unknown" => Ok(None),
                    _ => Err(Error::invalid_value(Unexpected::Str(v), exp)),
                }
            }
    }
}