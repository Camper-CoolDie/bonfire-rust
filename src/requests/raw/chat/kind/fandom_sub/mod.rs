mod params;

pub(crate) use params::RawParams;
use serde::Deserialize;
use serde_json::Value;

use crate::models::FandomSub;
use crate::requests::raw::chat::{RawKind, RawMessageable};
use crate::requests::raw::{RawChatTag, RawImageRef};
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawFandomSub {
    #[serde(skip)]
    pub id: u64,
    #[serde(rename = "customName")]
    pub name: String,
    #[serde(rename = "customImage")]
    pub icon: RawImageRef,
    #[serde(rename = "backgroundImage")]
    pub background: RawImageRef,
    pub params: RawParams,
}

impl RawMessageable for RawFandomSub {
    type Target = FandomSub;

    fn new(data: Value, tag: RawChatTag) -> Result<Self> {
        if let RawChatTag::FandomSub { id } = tag {
            let mut fandom_sub = serde_json::from_value::<RawFandomSub>(data)?;
            fandom_sub.id = id;
            Ok(fandom_sub)
        } else {
            let kind: i64 = RawKind::from(tag).into();
            Err(Error::UnknownVariant(kind))
        }
    }
}

impl TryFrom<RawFandomSub> for FandomSub {
    type Error = Error;

    fn try_from(value: RawFandomSub) -> Result<Self> {
        Ok(Self {
            id: value.id,
            name: value.name,
            icon: value.icon.into(),
            background: value.background.into(),
            intro: match value.params.intro.as_str() {
                "" => None,
                _ => Some(value.params.intro),
            },
        })
    }
}
