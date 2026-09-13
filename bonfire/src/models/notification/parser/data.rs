use serde::Deserialize;

#[derive(Deserialize)]
pub(super) enum Data {
    #[serde(rename = "my_data")]
    Legacy(String),
    // "melior_payload": unused for now
}
