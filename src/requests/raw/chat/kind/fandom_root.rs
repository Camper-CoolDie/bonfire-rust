use serde::Deserialize;
use serde_json::Value;

use crate::models::FandomRoot;
use crate::requests::raw::chat::{RawKind, RawMessageable};
use crate::requests::raw::{RawChatTag, RawImageRef, RawLanguage};
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawFandomRoot {
    #[serde(skip)]
    pub fandom_id: u64,
    #[serde(skip)]
    pub language: RawLanguage,
    #[serde(rename = "customName")]
    pub name: String,
    #[serde(rename = "customImage")]
    pub icon: RawImageRef,
    #[serde(rename = "subscribed")]
    pub is_subscribed: bool,
    #[serde(rename = "membersCount")]
    pub subscribers_count: u64,
}

impl RawMessageable for RawFandomRoot {
    type Target = FandomRoot;

    fn new(data: Value, tag: RawChatTag) -> Result<Self> {
        if let RawChatTag::FandomRoot {
            fandom_id,
            language,
        } = tag
        {
            let mut fandom_root = serde_json::from_value::<RawFandomRoot>(data)?;
            fandom_root.fandom_id = fandom_id;
            fandom_root.language = language;
            Ok(fandom_root)
        } else {
            let kind = RawKind::from(tag);
            Err(Error::UnknownVariant(Box::new(kind)))
        }
    }
}

impl TryFrom<RawFandomRoot> for FandomRoot {
    type Error = Error;

    fn try_from(value: RawFandomRoot) -> Result<Self> {
        Ok(Self {
            fandom_id: value.fandom_id,
            language: value.language.try_into()?,
            name: value.name,
            icon: value.icon.into(),
            is_subscribed: value.is_subscribed,
            subscribers_count: value.subscribers_count,
        })
    }
}
