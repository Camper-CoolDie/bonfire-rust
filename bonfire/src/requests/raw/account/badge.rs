use serde::Deserialize;

use crate::models::Badge;
use crate::requests::raw::RawImageRef;

#[derive(Deserialize)]
pub(crate) struct RawBadge {
    #[serde(rename = "id")]
    pub index: i64,
    #[serde(rename = "mi")]
    pub image: RawImageRef,
}

impl From<RawBadge> for Badge {
    fn from(value: RawBadge) -> Self {
        Self {
            index: value.index,
            image: value.image.into(),
        }
    }
}
