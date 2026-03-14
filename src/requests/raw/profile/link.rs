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
            // The index will be set inside Info::try_from()
            index: 0,
            title: value.title,
            uri: value.uri,
        }
    }
}
