use serde::Deserialize;

use crate::models::ImageRef;

#[derive(Deserialize)]
pub(crate) struct RawImageRef {
    #[serde(rename = "i")]
    pub id: u64,
    #[serde(rename = "u")]
    uri: String,
    #[serde(rename = "w")]
    width: usize,
    #[serde(rename = "h")]
    height: usize,
}

impl From<RawImageRef> for ImageRef {
    fn from(value: RawImageRef) -> Self {
        Self {
            id: value.id,
            uri: match value.uri.as_str() {
                "" => None,
                _ => Some(value.uri),
            },
            width: value.width,
            height: value.height,
        }
    }
}
