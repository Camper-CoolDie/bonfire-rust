use serde::Deserialize;

use crate::models::notification::PublicationRated;
use crate::requests::raw::publication::{RawKind, RawPostItemKind, RawPostTitle};
use crate::requests::raw::{RawAccountRef, RawGender, RawPublicationRef};
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawRated {
    #[serde(rename = "J_KARMA_COUNT")]
    pub amount: f64,
    #[serde(rename = "J_ACCOUNT_ID")]
    pub account_id: u64,
    #[serde(rename = "J_ACCOUNT_NAME")]
    pub account_name: String,
    #[serde(rename = "accountSex")]
    pub account_gender: RawGender,
    #[serde(rename = "J_UNIT_ID")]
    pub publication_id: u64,
    #[serde(rename = "J_UNIT_TYPE")]
    pub publication_kind: RawKind,
    #[serde(rename = "maskText")]
    pub post_title_text: String,
    #[serde(rename = "maskPageType")]
    pub post_title_item_kind: RawPostItemKind,
    #[serde(rename = "J_PARENT_UNIT_ID")]
    pub parent_id: u64,
    #[serde(rename = "J_PARENT_UNIT_TYPE")]
    pub parent_kind: RawKind,
}

impl TryFrom<RawRated> for PublicationRated {
    type Error = Error;

    fn try_from(value: RawRated) -> Result<Self> {
        Ok(Self {
            amount: value.amount / 100.0,
            account: RawAccountRef {
                id: value.account_id,
                name: value.account_name,
                gender: value.account_gender,
            }
            .try_into()?,
            post_title: match value.publication_kind {
                RawKind::Post => Some(
                    RawPostTitle {
                        text: value.post_title_text,
                        item_kind: value.post_title_item_kind,
                    }
                    .into(),
                ),
                _ => None,
            },
            publication: RawPublicationRef {
                id: value.publication_id,
                kind: value.publication_kind,
            }
            .into(),
            parent: RawPublicationRef {
                id: value.parent_id,
                kind: value.parent_kind,
            }
            .into(),
        })
    }
}
