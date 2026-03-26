mod content_kind;
mod event_kind;

pub(crate) use content_kind::RawContentKind;
use content_kind::{IntoContentOptions, IntoRefContentOptions};
pub(crate) use event_kind::RawEventKind;
use serde::Deserialize;

use crate::models::ChatMessage;
use crate::models::publication::ChatMessageRef;
use crate::requests::raw::chat::{RawKind as RawChatKind, RawMemberRole};
use crate::requests::raw::publication::{RawKind, RawPublishable};
use crate::requests::raw::{RawAccount, RawChatTag, RawFandom, RawGender, RawImageRef};
use crate::{Error, Result};

// A reference text is allowed to be 300 characters long (including "Author: "), otherwise it's
// truncated to 299 characters
const REFERENCE_TEXT_MAX_CHARS: usize = 300;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct InnerData {
    #[serde(rename = "J_TEXT")]
    pub text: String,
    #[serde(rename = "J_TYPE")]
    pub content_kind: RawContentKind,
    #[serde(rename = "systemType")]
    pub event_kind: RawEventKind,
    #[serde(rename = "systemOwnerId")]
    pub event_by_account_id: u64,
    #[serde(rename = "systemOwnerName")]
    pub event_by_account_name: String,
    #[serde(rename = "systemOwnerSex")]
    pub event_by_account_gender: RawGender,
    #[serde(rename = "systemTargetName")]
    pub event_target_name: String,
    #[serde(rename = "systemTargetId")]
    pub event_target_id: u64,
    #[serde(rename = "systemComment")]
    pub event_reason: String,
    #[serde(rename = "blockModerationEventId")]
    pub event_moderation_id: u64,
    #[serde(rename = "blockDate")]
    pub event_banned_until: i64,
    #[serde(rename = "systemTag")]
    pub event_new_role: RawMemberRole,
    #[serde(rename = "resource")]
    pub image: RawImageRef,
    pub gif: RawImageRef,
    pub images: Vec<RawImageRef>,
    pub sticker_id: u64,
    pub sticker_image: RawImageRef,
    pub sticker_gif: RawImageRef,
    // It's just an ImageRef under the hood
    pub voice_resource: RawImageRef,
    #[serde(rename = "voiceMs")]
    pub voice_duration: u64,
    #[serde(rename = "voiceMask")]
    pub voice_waveform: Vec<u64>,
    #[serde(rename = "quoteId")]
    pub reference_id: u64,
    #[serde(rename = "quoteText")]
    pub reference_text: String,
    #[serde(rename = "quoteImageRefs")]
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
pub(crate) struct RawChatMessage {
    pub fandom: RawFandom,
    #[serde(rename = "creator")]
    pub author: RawAccount,
    #[serde(rename = "tag_1")]
    pub chat_kind: RawChatKind,
    #[serde(rename = "tag_2")]
    pub chat_first_id: u64,
    #[serde(rename = "tag_3")]
    pub chat_second_id: u64,
    #[serde(rename = "jsonDB")]
    pub inner: InnerData,
}

impl RawPublishable for RawChatMessage {
    type Target = ChatMessage;

    fn new(data: serde_json::Value, _kind: RawKind) -> Result<Self> {
        Ok(serde_json::from_value::<RawChatMessage>(data)?)
    }
}

impl TryFrom<RawChatMessage> for ChatMessage {
    type Error = Error;

    fn try_from(value: RawChatMessage) -> Result<Self> {
        let reply_to = if value.inner.reference_id != 0 {
            // .chars().count() instead of .len() to account for unicode chars
            let chars_count = value.inner.reference_text.chars().count();

            // +3 for "..."
            let is_text_truncated = chars_count == REFERENCE_TEXT_MAX_CHARS + 3;

            // When a reference text is truncated, the server adds "..." to the end, so we remove it
            let text = if is_text_truncated {
                let text_length = value.inner.reference_text.len();
                &value.inner.reference_text[0..text_length - 3]
            } else {
                &value.inner.reference_text
            };

            // The resulting text will always have the following format:
            // "Author: Text"
            let text_without_author = match text.split_once(": ") {
                Some((_, without_author)) => without_author,
                None => text,
            };

            Some(ChatMessageRef {
                id: value.inner.reference_id,
                content: IntoRefContentOptions {
                    text: text_without_author,
                    images: value.inner.reference_images,
                    sticker_id: value.inner.reference_sticker_id,
                    sticker_image: value.inner.reference_sticker_image,
                }
                .into(),
                text: match text_without_author {
                    "" => None,
                    text => Some(text.to_owned()),
                },
                is_text_truncated,
                author_name: value.inner.reference_author_name,
            })
        } else {
            None
        };

        Ok(Self {
            content: IntoContentOptions {
                content_kind: value.inner.content_kind,
                event_kind: value.inner.event_kind,
                event_by_account_id: value.inner.event_by_account_id,
                event_by_account_name: value.inner.event_by_account_name,
                event_by_account_gender: value.inner.event_by_account_gender,
                event_target_name: value.inner.event_target_name,
                event_target_id: value.inner.event_target_id,
                event_reason: value.inner.event_reason,
                event_moderation_id: value.inner.event_moderation_id,
                event_banned_until: value.inner.event_banned_until,
                event_new_role: value.inner.event_new_role,
                image: value.inner.image,
                gif: value.inner.gif,
                images: value.inner.images,
                sticker_id: value.inner.sticker_id,
                sticker_image: value.inner.sticker_image,
                sticker_gif: value.inner.sticker_gif,
                voice_resource: value.inner.voice_resource,
                voice_duration: value.inner.voice_duration,
                voice_waveform: value.inner.voice_waveform,
            }
            .try_into()?,
            fandom: match value.chat_kind {
                RawChatKind::FandomRoot | RawChatKind::FandomSub => Some(value.fandom.try_into()?),
                _ => None,
            },
            author: value.author.try_into()?,
            chat_tag: RawChatTag::from((
                value.chat_kind,
                value.chat_first_id,
                value.chat_second_id,
            ))
            .try_into()?,
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
