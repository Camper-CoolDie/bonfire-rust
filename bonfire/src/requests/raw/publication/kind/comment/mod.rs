mod content_kind;

pub(crate) use content_kind::RawContentKind;
use content_kind::{IntoContentOptions, IntoRefContentOptions};
use serde::Deserialize;

use crate::models::Comment;
use crate::models::publication::CommentRef;
use crate::requests::raw::publication::{RawKind, RawPublishable};
use crate::requests::raw::{RawAccount, RawFandom, RawImageRef};
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
    #[serde(rename = "answerName")]
    pub answering_name: String,
    #[serde(rename = "changed")]
    pub is_edited: bool,
    #[serde(rename = "newFormatting")]
    pub has_new_formatting: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawComment {
    pub fandom: RawFandom,
    #[serde(rename = "creator")]
    pub author: RawAccount,
    #[serde(rename = "parentUnitId")]
    pub parent_id: u64,
    #[serde(rename = "parentUnitType")]
    pub parent_kind: RawKind,
    #[serde(rename = "karmaCount")]
    pub karma: f64,
    pub my_karma: f64,
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
        let reply_to = if value.inner.reference_id != 0 {
            // The returned reference text will always have the following format:
            // "Author: Text"
            let text_without_author = match value.inner.reference_text.split_once(": ") {
                Some((_, without_author)) => without_author,
                None => &value.inner.reference_text,
            };

            Some(CommentRef {
                id: value.inner.reference_id,
                content: IntoRefContentOptions {
                    images: value.inner.reference_images,
                    sticker_id: value.inner.reference_sticker_id,
                    sticker_image: value.inner.reference_sticker_image,
                }
                .into(),
                text: match text_without_author {
                    "" => None,
                    text => Some(text.to_owned()),
                },
                author_name: value.inner.reference_author_name,
            })
        } else {
            None
        };

        Ok(Self {
            content: IntoContentOptions {
                content_kind: value.inner.content_kind,
                image: value.inner.image,
                gif: value.inner.gif,
                images: value.inner.images,
                sticker_id: value.inner.sticker_id,
                sticker_image: value.inner.sticker_image,
                sticker_gif: value.inner.sticker_gif,
            }
            .into(),
            fandom: value.fandom.try_into()?,
            author: value.author.try_into()?,
            parent_id: value.parent_id,
            parent_kind: value.parent_kind.into(),
            karma: value.karma / 100.0,
            my_karma: match value.my_karma {
                0.0 => None,
                karma => Some(karma / 100.0),
            },
            reply_to,
            answering_name: match value.inner.answering_name.as_str() {
                "" => None,
                _ => Some(value.inner.answering_name)
                    .filter(|name| value.inner.text.starts_with(name)),
            },
            text: match value.inner.text.as_str() {
                "" => None,
                _ => Some(value.inner.text),
            },
            is_edited: value.inner.is_edited,
            has_new_formatting: value.inner.has_new_formatting,
        })
    }
}
