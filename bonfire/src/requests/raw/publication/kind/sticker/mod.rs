mod pack;

pub(crate) use pack::RawPack;
use serde::Deserialize;

use crate::models::Sticker;
use crate::models::publication::StickerContent;
use crate::requests::raw::publication::{RawKind, RawPublishable};
use crate::requests::raw::{RawAccount, RawImageRef};
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct InnerData {
    image: RawImageRef,
    gif: RawImageRef,
}

#[derive(Deserialize)]
pub(crate) struct RawSticker {
    #[serde(rename = "creator")]
    pub author: RawAccount,
    #[serde(rename = "tag_1")]
    pub pack_id: u64,
    #[serde(rename = "jsonDB")]
    pub inner: InnerData,
}

impl RawPublishable for RawSticker {
    type Target = Sticker;

    fn new(data: serde_json::Value, _kind: RawKind) -> Result<Self> {
        Ok(serde_json::from_value(data)?)
    }
}

impl TryFrom<RawSticker> for Sticker {
    type Error = Error;

    fn try_from(value: RawSticker) -> Result<Self> {
        let image = value.inner.image.into();
        let gif = value.inner.gif.into();

        Ok(Self {
            content: match gif {
                None => StickerContent::Normal { image },
                Some(gif) => StickerContent::Gif {
                    first_frame: image,
                    animated: gif,
                },
            },
            author: value.author.try_into()?,
            pack_id: value.pack_id,
        })
    }
}
