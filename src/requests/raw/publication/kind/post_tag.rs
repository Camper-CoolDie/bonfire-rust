use serde::Deserialize;

use crate::models::PostTag;
use crate::requests::raw::publication::{RawKind, RawPublishable};
use crate::requests::raw::{RawAccount, RawFandom, RawImageRef};
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
    pub fandom: RawFandom,
    pub creator: RawAccount,
    #[serde(rename = "parentId")]
    pub category_id: u64,
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
            fandom: value.fandom.try_into()?,
            creator: value.creator.try_into()?,
            category_id: match value.category_id {
                0 => None,
                id => Some(id),
            },
            name: value.inner.name,
            icon: value.inner.icon.into(),
        })
    }
}
