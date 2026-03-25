use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct RawParams {
    #[serde(rename = "text")]
    pub intro: String,
}
