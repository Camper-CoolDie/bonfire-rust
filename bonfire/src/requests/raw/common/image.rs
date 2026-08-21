use serde::Deserialize;

use crate::models::ImageRef;

#[derive(Clone, Deserialize)]
pub(crate) struct RawImageRef {
    #[serde(rename = "i")]
    pub id: u64,
    #[serde(rename = "u")]
    pub uri: String,
    #[serde(rename = "w")]
    pub width: usize,
    #[serde(rename = "h")]
    pub height: usize,
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

impl From<RawImageRef> for Option<ImageRef> {
    fn from(value: RawImageRef) -> Self {
        match value.id {
            0 => None,
            _ => Some(ImageRef {
                id: value.id,
                uri: match value.uri.as_str() {
                    "" => None,
                    _ => Some(value.uri),
                },
                width: value.width,
                height: value.height,
            }),
        }
    }
}
