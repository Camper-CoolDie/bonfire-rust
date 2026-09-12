use serde::Deserialize;

use crate::models::notification::PublicationCommented;
use crate::requests::raw::publication::{RawKind, RawPostItemKind, RawPostTitle};
use crate::requests::raw::{RawAccountRef, RawGender, RawPublicationRef};
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawCommented {
    #[serde(rename = "J_COMMENT_ID")]
    pub id: u64,
    #[serde(rename = "J_ACCOUNT_ID")]
    pub author_id: u64,
    #[serde(rename = "J_ACCOUNT_NAME")]
    pub author_name: String,
    #[serde(rename = "accountSex")]
    pub author_gender: RawGender,
    pub fandom_name: String,
    #[serde(rename = "J_UNIT_ID")]
    pub parent_id: u64,
    #[serde(rename = "J_PARENT_UNIT_TYPE")]
    pub parent_kind: RawKind,
    #[serde(rename = "unitCreatorId")]
    pub parent_author_id: u64,
    #[serde(rename = "maskText")]
    pub parent_post_title_text: String,
    #[serde(rename = "maskPageType")]
    pub parent_post_title_item_kind: RawPostItemKind,
    #[serde(rename = "commentText")]
    pub text: String,
}

impl TryFrom<RawCommented> for PublicationCommented {
    type Error = Error;

    fn try_from(value: RawCommented) -> Result<Self> {
        Ok(Self {
            id: value.id,
            author: RawAccountRef {
                id: value.author_id,
                name: value.author_name,
                gender: value.author_gender,
            }
            .try_into()?,
            fandom_name: match value.parent_kind {
                RawKind::Quest => None,
                _ => Some(value.fandom_name),
            },
            parent_author_id: value.parent_author_id,
            parent_post_title: match value.parent_kind {
                RawKind::Post => Some(
                    RawPostTitle {
                        text: value.parent_post_title_text,
                        item_kind: value.parent_post_title_item_kind,
                    }
                    .into(),
                ),
                _ => None,
            },
            parent: RawPublicationRef {
                id: value.parent_id,
                kind: value.parent_kind,
            }
            .into(),
            text: match value.text.as_str() {
                "" => None,
                _ => Some(value.text),
            },
        })
    }
}
