use serde::Deserialize;

use crate::models::notification::AccountMentioned;
use crate::requests::raw::chat::RawKind as RawChatKind;
use crate::requests::raw::publication::RawKind as RawPublicationKind;
use crate::requests::raw::{RawAccountRef, RawChatTag, RawGender, RawPublicationRef};
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawMentioned {
    #[serde(rename = "fromAccountId")]
    pub account_id: u64,
    #[serde(rename = "fromAccountName")]
    pub account_name: String,
    #[serde(rename = "fromAccountSex")]
    pub account_gender: RawGender,
    #[serde(rename = "unitId")]
    pub publication_id: u64,
    #[serde(rename = "unitType")]
    pub publication_kind: RawPublicationKind,
    #[serde(rename = "tag1")]
    pub chat_kind: RawChatKind,
    #[serde(rename = "tag2")]
    pub chat_first_id: u64,
    #[serde(rename = "tag3")]
    pub chat_second_id: u64,
    pub text: String,
}

impl TryFrom<RawMentioned> for AccountMentioned {
    type Error = Error;

    fn try_from(value: RawMentioned) -> Result<Self> {
        Ok(Self {
            account: RawAccountRef {
                id: value.account_id,
                name: value.account_name,
                gender: value.account_gender,
            }
            .try_into()?,
            chat_tag: match value.publication_kind {
                RawPublicationKind::ChatMessage => Some(
                    RawChatTag::from((value.chat_kind, value.chat_first_id, value.chat_second_id))
                        .try_into()?,
                ),
                _ => None,
            },
            publication: RawPublicationRef {
                id: value.publication_id,
                kind: value.publication_kind,
            }
            .into(),
            text: value.text,
        })
    }
}
