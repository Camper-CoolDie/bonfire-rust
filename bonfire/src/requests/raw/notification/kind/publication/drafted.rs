use serde::Deserialize;

use crate::models::notification::PublicationDrafted;
use crate::requests::raw::RawGender;
use crate::requests::raw::publication::{RawKind, RawPostItemKind, RawPostTitle};
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawPublicationDrafted {
    #[serde(rename = "publicationTyoe")]
    pub kind: RawKind,
    #[serde(rename = "maskText")]
    pub title_text: String,
    #[serde(rename = "maskPageType")]
    pub title_item_kind: RawPostItemKind,
    pub moderation_id: u64,
    pub moderator_name: String,
    #[serde(rename = "moderatorSex")]
    pub moderator_gender: RawGender,
    #[serde(rename = "comment")]
    pub reason: String,
}

impl TryFrom<RawPublicationDrafted> for PublicationDrafted {
    type Error = Error;

    fn try_from(value: RawPublicationDrafted) -> Result<Self> {
        Ok(Self {
            title: match value.kind {
                RawKind::Post => Some(
                    RawPostTitle {
                        text: value.title_text,
                        item_kind: value.title_item_kind,
                    }
                    .into(),
                ),
                _ => None,
            },
            kind: value.kind.into(),
            moderation_id: value.moderation_id,
            moderator_name: value.moderator_name,
            moderator_gender: value.moderator_gender.try_into()?,
            reason: value.reason,
        })
    }
}
