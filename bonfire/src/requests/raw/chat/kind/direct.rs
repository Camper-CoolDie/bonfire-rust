use serde::Deserialize;
use serde_json::Value;

use crate::models::Direct;
use crate::requests::raw::chat::{RawKind, RawMessageable};
use crate::requests::raw::conversions::timestamp_from_millis;
use crate::requests::raw::{RawAccount, RawChatTag};
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawDirect {
    #[serde(skip)]
    pub ids: (u64, u64),
    #[serde(rename = "anotherAccount")]
    pub recipient: RawAccount,
    #[serde(rename = "anotherAccountReadDate")]
    pub recipient_read_at: i64,
}

impl RawMessageable for RawDirect {
    type Target = Direct;

    fn new(data: Value, tag: RawChatTag) -> Result<Self> {
        if let RawChatTag::Direct { ids } = tag {
            let mut direct = serde_json::from_value::<RawDirect>(data)?;
            direct.ids = ids;
            Ok(direct)
        } else {
            let kind = RawKind::from(tag);
            Err(Error::UnknownVariant(Box::new(kind)))
        }
    }
}

impl TryFrom<RawDirect> for Direct {
    type Error = Error;

    fn try_from(value: RawDirect) -> Result<Self> {
        Ok(Self {
            ids: value.ids,
            recipient: value.recipient.try_into()?,
            recipient_read_at: match value.recipient_read_at {
                0 => None,
                timestamp => Some(timestamp_from_millis(timestamp)?),
            },
        })
    }
}
