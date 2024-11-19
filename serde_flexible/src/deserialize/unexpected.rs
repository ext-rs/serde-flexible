use serde::de::{Error, Expected, Unexpected};

pub(super) const OUT_OF_RANGE_ERROR_BEGIN: &str = "out or range value";

pub fn out_or_range_value<E: Error>(unexp: Unexpected, exp: &dyn Expected) -> E {
    Error::custom(format_args!("{}: {}, expected {}", OUT_OF_RANGE_ERROR_BEGIN, unexp, exp))
}