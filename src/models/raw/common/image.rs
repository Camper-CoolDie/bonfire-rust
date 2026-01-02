use serde::Deserialize;

use crate::models::ImageRef;

#[derive(Deserialize)]
pub(crate) struct RawImageRef {
    #[serde(rename = "i")]
    pub id: i64,
    #[serde(rename = "u")]
    uri: String,
    #[serde(rename = "w")]
    width: i32,
    #[serde(rename = "h")]
    height: i32,
}

impl From<RawImageRef> for ImageRef {
    fn from(value: RawImageRef) -> Self {
        Self {
            id: value.id,
            uri: value.uri,
            width: value.width,
            height: value.height,
        }
    }
}
