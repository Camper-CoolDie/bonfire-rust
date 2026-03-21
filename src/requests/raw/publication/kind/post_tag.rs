use serde::Deserialize;

use crate::models::PostTag;
use crate::requests::raw::RawImageRef;
use crate::requests::raw::publication::{RawKind, RawPublishable};
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct InnerData {
    #[serde(rename = "J_NAME")]
    pub name: String,
    #[serde(rename = "image")]
    pub icon: RawImageRef,
}

#[derive(Deserialize)]
pub(crate) struct RawPostTag {
    #[serde(rename = "jsonDB")]
    pub inner: InnerData,
}

impl RawPublishable for RawPostTag {
    type Target = PostTag;

    fn new(data: serde_json::Value, _kind: RawKind) -> Result<Self> {
        Ok(serde_json::from_value::<RawPostTag>(data)?)
    }
}

impl TryFrom<RawPostTag> for PostTag {
    type Error = Error;

    fn try_from(value: RawPostTag) -> Result<Self> {
        Ok(Self {
            name: value.inner.name,
            icon: match value.inner.icon.id {
                0 => None,
                _ => Some(value.inner.icon.into()),
            },
        })
    }
}
