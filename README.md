# serde-flexible

`serde-flexible` is a Rust library that enhances the flexibility of deserialization with Serde. It is designed to handle scenarios where clients may provide input data in varying formats or make mistakes with data types. This library allows you to define custom deserialization behavior for specific fields, making it easier to work with non-standard or inconsistent data.

---

## **Features**

- Flexible deserialization for fields that can accept multiple formats.
- Support for uniquely reducible types (e.g., treating integers as strings during deserialization).
- Simplifies handling of real-world data from unreliable sources.

---

## **Example**

```rust
use serde::Deserialize;
use serde_flexible::{as_string, as_string_opt};

#[derive(Debug, Deserialize, PartialEq)]
struct Message {
    #[serde(deserialize_with = "as_string")]
    owner: String,

    #[serde(deserialize_with = "as_string_opt")]
    referral: Option<String>,
}

#[test]
fn test_base_good_parse() {
    assert_eq!(
        serde_json::from_str::<Message>(r#"{
            "owner": "8a8cc628-88e3-4550-90c0-a64bd8f446dd", 
            "referral": 7132,
        }"#).unwrap(),
        Message {
            owner: "8a8cc628-88e3-4550-90c0-a64bd8f446dd".to_string(),
            referral: Some("7132".to_string())
        }
    );
}
```

---

## **All Possible Deserializers**

Below are all the deserializers provided by `serde-flexible`, along with their descriptions:

### Standard Deserializers

- **`as_bool`**: Converts any input that can be interpreted as a boolean to `bool`. For example, `"true"`, `1`, and `0` can all be deserialized as `true` or `false`.
- **`as_f64`**: Converts any input that can be interpreted as a floating-point number to `f64`. For example, `"3.14"` will be deserialized as `3.14`.
- **`as_i64`**: Converts any input that can be interpreted as an integer to `i64`. For example, `"42"` and `42` will be deserialized as `42`.
- **`as_string`**: Converts any input that can be interpreted as a string to `String`. For example, `123` will be converted to `"123"`.
- **`as_u64`**: Converts any input that can be interpreted as an unsigned integer to `u64`. For example, `"100"` and `100` will be deserialized as `100`.

### Optional Deserializers

These deserializers allow the field to accept `null` or missing values in addition to valid inputs:

- **`as_bool_opt`**: Similar to `as_bool`, but also allows `null` values, deserializing as `Option<bool>`.
- **`as_f64_opt`**: Similar to `as_f64`, but also allows `null` values, deserializing as `Option<f64>`.
- **`as_i64_opt`**: Similar to `as_i64`, but also allows `null` values, deserializing as `Option<i64>`.
- **`as_string_opt`**: Similar to `as_string`, but also allows `null` values, deserializing as `Option<String>`.
- **`as_u64_opt`**: Similar to `as_u64`, but also allows `null` values, deserializing as `Option<u64>`.
