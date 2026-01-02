use serde::Deserialize;

use crate::models::raw::RawImageRef;
use crate::models::Badge;

#[derive(Deserialize)]
pub(crate) struct RawBadge {
    #[serde(rename = "id")]
    index: i64,
    #[serde(rename = "mi")]
    image: RawImageRef,
}

impl From<RawBadge> for Badge {
    fn from(value: RawBadge) -> Self {
        Self {
            index: value.index,
            image: value.image.into(),
        }
    }
}
