mod content_kind;

pub(crate) use content_kind::RawContentKind;
use serde::Deserialize;

use crate::models::Comment;
use crate::models::publication::{CommentContent, CommentReference};
use crate::requests::raw::RawImageRef;
use crate::requests::raw::publication::{RawKind, RawPublishable};
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InnerData {
    #[serde(rename = "J_TEXT")]
    pub text: String,
    #[serde(rename = "type")]
    pub content_kind: RawContentKind,
    pub image: RawImageRef,
    pub gif: RawImageRef,
    pub images: Vec<RawImageRef>,
    pub sticker_id: u64,
    pub sticker_image: RawImageRef,
    pub sticker_gif: RawImageRef,
    #[serde(rename = "quoteId")]
    pub reference_id: u64,
    #[serde(rename = "quoteText")]
    pub reference_text: String,
    #[serde(rename = "quoteImagesRefs")]
    pub reference_images: Vec<RawImageRef>,
    #[serde(rename = "quoteStickerId")]
    pub reference_sticker_id: u64,
    #[serde(rename = "quoteStickerImage")]
    pub reference_sticker_image: RawImageRef,
    #[serde(rename = "quoteCreatorName")]
    pub reference_author_name: String,
    #[serde(rename = "changed")]
    pub is_edited: bool,
    #[serde(rename = "newFormatting")]
    pub has_new_formatting: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawComment {
    #[serde(rename = "jsonDB")]
    pub inner: InnerData,
}

impl RawPublishable for RawComment {
    type Target = Comment;

    fn new(data: serde_json::Value, _kind: RawKind) -> Result<Self> {
        Ok(serde_json::from_value::<RawComment>(data)?)
    }
}

impl TryFrom<RawComment> for Comment {
    type Error = Error;

    fn try_from(value: RawComment) -> Result<Self> {
        let content = match value.inner.content_kind {
            RawContentKind::Text => CommentContent::Text,
            RawContentKind::Image => CommentContent::Image(value.inner.image.into()),
            RawContentKind::Gif => CommentContent::Gif {
                first_frame: value.inner.image.into(),
                animated: value.inner.gif.into(),
            },
            RawContentKind::Images => {
                CommentContent::Images(value.inner.images.into_iter().map(Into::into).collect())
            }
            RawContentKind::Sticker => CommentContent::Sticker {
                id: value.inner.sticker_id,
                image: value.inner.sticker_image.into(),
                gif: value.inner.sticker_gif.into(),
            },
            RawContentKind::Unknown(unknown) => CommentContent::Unknown(unknown),
        };

        let reference = if value.inner.reference_id != 0 {
            let content = if value.inner.reference_images.len() > 1 {
                CommentContent::Images(
                    value
                        .inner
                        .reference_images
                        .into_iter()
                        .map(Into::into)
                        .collect(),
                )
            } else if let Some(image) = value.inner.reference_images.first() {
                CommentContent::Image(image.clone().into())
            } else if value.inner.reference_sticker_id != 0 {
                CommentContent::Sticker {
                    id: value.inner.reference_sticker_id,
                    image: value.inner.reference_sticker_image.into(),
                    gif: None,
                }
            } else {
                CommentContent::Text
            };

            // The returned reference text will most likely have the following format:
            // "Author: Text"
            let text = match value.inner.reference_text.split_once(": ") {
                Some((_, text)) => text,
                None => &value.inner.reference_text,
            };

            Some(CommentReference {
                id: value.inner.reference_id,
                content,
                text: match text {
                    "" => None,
                    _ => Some(text.to_owned()),
                },
                author_name: value.inner.reference_author_name,
            })
        } else {
            None
        };

        Ok(Self {
            content,
            text: match value.inner.text.as_str() {
                "" => None,
                _ => Some(value.inner.text),
            },
            reference,
            is_edited: value.inner.is_edited,
            has_new_formatting: value.inner.has_new_formatting,
        })
    }
}
