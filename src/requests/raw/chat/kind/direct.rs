use serde::Deserialize;
use serde_json::Value;

use crate::models::Direct;
use crate::requests::raw::chat::{RawKind, RawMessageable};
use crate::requests::raw::{RawAccount, RawChatTag, timestamp_from_millis};
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawDirect {
    #[serde(skip)]
    pub my_id: u64,
    #[serde(rename = "anotherAccount")]
    pub recipient: RawAccount,
    #[serde(rename = "anotherAccountReadDate")]
    pub recipient_read_at: i64,
}

impl RawMessageable for RawDirect {
    type Target = Direct;

    fn new(data: Value, tag: RawChatTag) -> Result<Self> {
        if let RawChatTag::Direct { my_id, .. } = tag {
            let mut direct = serde_json::from_value::<RawDirect>(data)?;
            direct.my_id = my_id;
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
            my_id: value.my_id,
            recipient: value.recipient.try_into()?,
            recipient_read_at: match value.recipient_read_at {
                0 => None,
                timestamp => Some(timestamp_from_millis(timestamp)?),
            },
        })
    }
}
