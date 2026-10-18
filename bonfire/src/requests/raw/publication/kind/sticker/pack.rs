use serde::Deserialize;

use crate::models::StickerPack;
use crate::requests::raw::publication::{RawKind, RawPublishable};
use crate::requests::raw::{RawAccount, RawComment, RawImageRef, RawPublication};
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct InnerData {
    name: String,
    #[serde(rename = "image")]
    icon: RawImageRef,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawPack {
    #[serde(rename = "creator")]
    pub author: RawAccount,
    pub best_comment: Option<RawPublication<RawComment>>,
    #[serde(rename = "karmaCount")]
    pub karma: f64,
    pub my_karma: f64,
    #[serde(rename = "subUnitsCount")]
    pub comments_count: u64,
    #[serde(rename = "jsonDB")]
    pub inner: InnerData,
}

impl RawPublishable for RawPack {
    type Target = StickerPack;

    fn new(data: serde_json::Value, _kind: RawKind) -> Result<Self> {
        Ok(serde_json::from_value(data)?)
    }
}

impl TryFrom<RawPack> for StickerPack {
    type Error = Error;

    fn try_from(value: RawPack) -> Result<Self> {
        Ok(Self {
            name: value.inner.name,
            author: value.author.try_into()?,
            icon: value.inner.icon.into(),
            // Filled by the caller
            stickers: Vec::new(),
            best_comment: value.best_comment.map(TryInto::try_into).transpose()?,
            karma: value.karma / 100.0,
            my_karma: match value.my_karma {
                0.0 => None,
                karma => Some(karma / 100.0),
            },
            comments_count: value.comments_count,
        })
    }
}
