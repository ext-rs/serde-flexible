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
            referral: Some(
                "7132".to_string()
            )
        }
    )
}
