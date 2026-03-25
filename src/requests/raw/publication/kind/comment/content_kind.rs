use std::result::Result as StdResult;

use serde::Deserialize;

use crate::models::publication::{CommentContent, CommentRefContent};
use crate::requests::raw::RawImageRef;

pub(crate) enum RawContentKind {
    Text,
    Image,
    Gif,
    Images,
    Sticker,
    Unknown(i64),
}

impl<'de> Deserialize<'de> for RawContentKind {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(match i64::deserialize(deserializer)? {
            0 => RawContentKind::Text,
            3 => RawContentKind::Image,
            4 => RawContentKind::Gif,
            5 => RawContentKind::Images,
            6 => RawContentKind::Sticker,
            other => RawContentKind::Unknown(other),
        })
    }
}

pub(super) struct IntoContentOptions {
    pub content_kind: RawContentKind,
    pub image: RawImageRef,
    pub gif: RawImageRef,
    pub images: Vec<RawImageRef>,
    pub sticker_id: u64,
    pub sticker_image: RawImageRef,
    pub sticker_gif: RawImageRef,
}

impl From<IntoContentOptions> for CommentContent {
    fn from(value: IntoContentOptions) -> Self {
        match value.content_kind {
            RawContentKind::Text => CommentContent::Text,
            RawContentKind::Image => CommentContent::Image(value.image.into()),
            RawContentKind::Gif => CommentContent::Gif {
                first_frame: value.image.into(),
                animated: value.gif.into(),
            },
            RawContentKind::Images => {
                CommentContent::Images(value.images.into_iter().map(Into::into).collect())
            }
            RawContentKind::Sticker => CommentContent::Sticker {
                id: value.sticker_id,
                image: value.sticker_image.into(),
                gif: value.sticker_gif.into(),
            },
            RawContentKind::Unknown(unknown) => CommentContent::Unknown(unknown),
        }
    }
}

pub(super) struct IntoRefContentOptions {
    pub images: Vec<RawImageRef>,
    pub sticker_id: u64,
    pub sticker_image: RawImageRef,
}

impl From<IntoRefContentOptions> for CommentRefContent {
    fn from(value: IntoRefContentOptions) -> Self {
        if value.images.len() > 1 {
            CommentRefContent::Images(value.images.into_iter().map(Into::into).collect())
        } else if let Some(image) = value.images.first() {
            CommentRefContent::Image(image.clone().into())
        } else if value.sticker_id != 0 {
            CommentRefContent::Sticker {
                id: value.sticker_id,
                image: value.sticker_image.into(),
            }
        } else {
            CommentRefContent::Text
        }
    }
}
