use std::result::Result as StdResult;
use std::time::Duration;

use serde::Deserialize;

use super::RawEventKind;
use super::event_kind::IntoEventOptions;
use crate::models::publication::{ChatMessageContent, ChatMessageRefContent};
use crate::requests::raw::chat::RawMemberRole;
use crate::requests::raw::{RawGender, RawImageRef};
use crate::{Error, Result};

pub(crate) enum RawContentKind {
    Text,
    Event,
    Image,
    Gif,
    Images,
    Voice,
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
            1 => RawContentKind::Event,
            3 => RawContentKind::Image,
            4 => RawContentKind::Gif,
            5 => RawContentKind::Images,
            6 => RawContentKind::Voice,
            7 => RawContentKind::Sticker,
            other => RawContentKind::Unknown(other),
        })
    }
}

pub(super) struct IntoContentOptions {
    pub content_kind: RawContentKind,
    pub event_kind: RawEventKind,
    pub event_by_account_id: u64,
    pub event_by_account_name: String,
    pub event_by_account_gender: RawGender,
    pub event_target_name: String,
    pub event_target_id: u64,
    pub event_reason: String,
    pub event_moderation_id: u64,
    pub event_banned_until: i64,
    pub event_new_role: RawMemberRole,
    pub image: RawImageRef,
    pub gif: RawImageRef,
    pub images: Vec<RawImageRef>,
    pub sticker_id: u64,
    pub sticker_image: RawImageRef,
    pub sticker_gif: RawImageRef,
    pub voice_resource: RawImageRef,
    pub voice_duration: u64,
    pub voice_waveform: Vec<u64>,
}

impl TryFrom<IntoContentOptions> for ChatMessageContent {
    type Error = Error;

    fn try_from(value: IntoContentOptions) -> Result<Self> {
        Ok(match value.content_kind {
            RawContentKind::Text => ChatMessageContent::Text,
            RawContentKind::Event => IntoEventOptions {
                event_kind: value.event_kind,
                by_account_id: value.event_by_account_id,
                by_account_name: value.event_by_account_name,
                by_account_gender: value.event_by_account_gender,
                target_name: value.event_target_name,
                target_id: value.event_target_id,
                reason: value.event_reason,
                moderation_id: value.event_moderation_id,
                banned_until: value.event_banned_until,
                new_role: value.event_new_role,
            }
            .try_into()?,
            RawContentKind::Image => ChatMessageContent::Image(value.image.into()),
            RawContentKind::Gif => ChatMessageContent::Gif {
                first_frame: value.image.into(),
                animated: value.gif.into(),
            },
            RawContentKind::Images => {
                ChatMessageContent::Images(value.images.into_iter().map(Into::into).collect())
            }
            RawContentKind::Voice => ChatMessageContent::Voice {
                id: value.voice_resource.id,
                uri: value.voice_resource.uri,
                duration: Duration::from_millis(value.voice_duration),
                waveform: value.voice_waveform,
            },
            RawContentKind::Sticker => ChatMessageContent::Sticker {
                id: value.sticker_id,
                image: value.sticker_image.into(),
                gif: value.sticker_gif.into(),
            },
            RawContentKind::Unknown(unknown) => ChatMessageContent::Unknown(unknown),
        })
    }
}

pub(super) struct IntoRefContentOptions {
    pub resources: Vec<RawImageRef>,
    pub sticker_id: u64,
    pub sticker_image: RawImageRef,
}

impl From<IntoRefContentOptions> for ChatMessageRefContent {
    fn from(value: IntoRefContentOptions) -> Self {
        if value.resources.len() > 1 {
            ChatMessageRefContent::Images(value.resources.into_iter().map(Into::into).collect())
        } else if let Some(resource) = value.resources.first() {
            // We know it's a voice message when the dimensions aren't given
            if resource.width == 0 && resource.height == 0 {
                ChatMessageRefContent::Voice {
                    id: resource.id,
                    uri: resource.uri.clone(),
                }
            } else {
                ChatMessageRefContent::Image(resource.clone().into())
            }
        } else if value.sticker_id != 0 {
            ChatMessageRefContent::Sticker {
                id: value.sticker_id,
                image: value.sticker_image.into(),
            }
        } else {
            ChatMessageRefContent::Text
        }
    }
}
