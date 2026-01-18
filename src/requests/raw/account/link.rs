use serde::Deserialize;

use crate::models::Link;

#[derive(Deserialize)]
pub(crate) struct RawLink {
    pub title: String,
    #[serde(rename = "url")]
    pub uri: String,
}

impl From<RawLink> for Link {
    fn from(value: RawLink) -> Self {
        Self {
            title: value.title,
            uri: value.uri,
        }
    }
}
