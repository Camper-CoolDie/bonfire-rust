use serde::Deserialize;

use crate::models::notification::CommentReplied;
use crate::requests::raw::publication::RawKind;
use crate::requests::raw::{RawAccountRef, RawGender, RawPublicationRef};
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawCommentReplied {
    #[serde(rename = "J_COMMENT_ID")]
    pub id: u64,
    #[serde(rename = "J_ACCOUNT_ID")]
    pub author_id: u64,
    #[serde(rename = "J_ACCOUNT_NAME")]
    pub author_name: String,
    #[serde(rename = "accountSex")]
    pub author_gender: RawGender,
    #[serde(rename = "J_UNIT_ID")]
    pub parent_id: u64,
    #[serde(rename = "J_PARENT_UNIT_TYPE")]
    pub parent_kind: RawKind,
    #[serde(rename = "commentText")]
    pub text: String,
}

impl TryFrom<RawCommentReplied> for CommentReplied {
    type Error = Error;

    fn try_from(value: RawCommentReplied) -> Result<Self> {
        Ok(Self {
            id: value.id,
            author: RawAccountRef {
                id: value.author_id,
                name: value.author_name,
                gender: value.author_gender,
            }
            .try_into()?,
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
