use serde_json::Value;

use crate::models::AnyChat;
use crate::requests::raw::RawChatTag;
use crate::requests::raw::chat::{
    RawDirect, RawFandomRoot, RawFandomSub, RawGroup, RawMessageable,
};
use crate::{Error, Result};

pub(crate) enum AnyRawChat {
    FandomRoot(RawFandomRoot),
    FandomSub(RawFandomSub),
    Group(RawGroup),
    Direct(RawDirect),
    Unknown(i64),
}

impl RawMessageable for AnyRawChat {
    type Target = AnyChat;

    fn new(data: Value, tag: RawChatTag) -> Result<Self> {
        Ok(match tag {
            RawChatTag::FandomRoot { .. } => AnyRawChat::FandomRoot(RawFandomRoot::new(data, tag)?),
            RawChatTag::FandomSub { .. } => AnyRawChat::FandomSub(RawFandomSub::new(data, tag)?),
            RawChatTag::Group { .. } => AnyRawChat::Group(RawGroup::new(data, tag)?),
            RawChatTag::Direct { .. } => AnyRawChat::Direct(RawDirect::new(data, tag)?),
            RawChatTag::Unknown { kind, .. } => AnyRawChat::Unknown(kind),
        })
    }
}

impl TryFrom<AnyRawChat> for AnyChat {
    type Error = Error;

    fn try_from(value: AnyRawChat) -> Result<Self> {
        Ok(match value {
            AnyRawChat::FandomRoot(chat) => AnyChat::FandomRoot(chat.try_into()?),
            AnyRawChat::FandomSub(chat) => AnyChat::FandomSub(chat.try_into()?),
            AnyRawChat::Group(chat) => AnyChat::Group(chat.try_into()?),
            AnyRawChat::Direct(chat) => AnyChat::Direct(chat.try_into()?),
            AnyRawChat::Unknown(unknown) => return Err(Error::UnknownVariant(Box::new(unknown))),
        })
    }
}
