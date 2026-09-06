use std::result::Result as StdResult;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::models::publication::PostItemKind;

#[derive(Debug)]
pub(crate) enum RawKind {
    Text,
    Image,
    Images,
    Video,
    Quote,
    Link,
    Spoiler,
    Poll,
    LinkWithThumbnail,
    Table,
    YoutubeVideo,
    CodeBlock,
    BonfireLink,
    PostRelay,
    Unknown(i64),
}

impl Serialize for RawKind {
    fn serialize<S>(&self, serializer: S) -> StdResult<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let kind = match self {
            RawKind::Text => 1,
            RawKind::Image => 2,
            RawKind::Images => 3,
            RawKind::Link => 4,
            RawKind::Quote => 5,
            RawKind::Spoiler => 6,
            RawKind::Poll => 7,
            RawKind::YoutubeVideo => 9,
            RawKind::Table => 10,
            RawKind::BonfireLink => 12,
            RawKind::PostRelay => 13,
            RawKind::LinkWithThumbnail => 14,
            RawKind::CodeBlock => 16,
            RawKind::Video => 17,
            RawKind::Unknown(unknown) => *unknown,
        };

        serializer.serialize_i64(kind)
    }
}

impl<'de> Deserialize<'de> for RawKind {
    fn deserialize<D>(deserializer: D) -> StdResult<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match i64::deserialize(deserializer)? {
            1 => RawKind::Text,
            2 => RawKind::Image,
            3 => RawKind::Images,
            4 => RawKind::Link,
            5 => RawKind::Quote,
            6 => RawKind::Spoiler,
            7 => RawKind::Poll,
            9 => RawKind::YoutubeVideo,
            10 => RawKind::Table,
            12 => RawKind::BonfireLink,
            13 => RawKind::PostRelay,
            14 => RawKind::LinkWithThumbnail,
            16 => RawKind::CodeBlock,
            17 => RawKind::Video,
            other => RawKind::Unknown(other),
        })
    }
}

impl From<RawKind> for PostItemKind {
    fn from(value: RawKind) -> Self {
        match value {
            RawKind::Text => PostItemKind::Text,
            RawKind::Image => PostItemKind::Image,
            RawKind::Images => PostItemKind::Images,
            RawKind::Link => PostItemKind::Link,
            RawKind::Quote => PostItemKind::Quote,
            RawKind::Spoiler => PostItemKind::Spoiler,
            RawKind::Poll => PostItemKind::Poll,
            RawKind::YoutubeVideo => PostItemKind::YoutubeVideo,
            RawKind::Table => PostItemKind::Table,
            RawKind::BonfireLink => PostItemKind::BonfireLink,
            RawKind::PostRelay => PostItemKind::PostRelay,
            RawKind::LinkWithThumbnail => PostItemKind::LinkWithThumbnail,
            RawKind::CodeBlock => PostItemKind::CodeBlock,
            RawKind::Video => PostItemKind::Video,
            RawKind::Unknown(kind) => PostItemKind::Unknown(kind),
        }
    }
}

impl From<PostItemKind> for RawKind {
    fn from(value: PostItemKind) -> Self {
        match value {
            PostItemKind::Text => RawKind::Text,
            PostItemKind::Image => RawKind::Image,
            PostItemKind::Images => RawKind::Images,
            PostItemKind::Link => RawKind::Link,
            PostItemKind::Quote => RawKind::Quote,
            PostItemKind::Spoiler => RawKind::Spoiler,
            PostItemKind::Poll => RawKind::Poll,
            PostItemKind::YoutubeVideo => RawKind::YoutubeVideo,
            PostItemKind::Table => RawKind::Table,
            PostItemKind::BonfireLink => RawKind::BonfireLink,
            PostItemKind::PostRelay => RawKind::PostRelay,
            PostItemKind::LinkWithThumbnail => RawKind::LinkWithThumbnail,
            PostItemKind::CodeBlock => RawKind::CodeBlock,
            PostItemKind::Video => RawKind::Video,
            PostItemKind::Unknown(kind) => RawKind::Unknown(kind),
        }
    }
}
