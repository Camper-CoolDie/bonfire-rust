use serde::Deserialize;

use crate::models::notification::PublicationReacted;
use crate::requests::raw::publication::RawKind;
use crate::requests::raw::{RawAccountRef, RawGender, RawPublicationRef};
use crate::{Error, Result};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawReacted {
    #[serde(rename = "reactionIndex")]
    pub index: i64,
    pub account_id: u64,
    pub account_name: String,
    #[serde(rename = "accountSex")]
    pub account_gender: RawGender,
    #[serde(rename = "unitId")]
    pub publication_id: u64,
    #[serde(rename = "unitType")]
    pub publication_kind: RawKind,
    #[serde(rename = "parentUnitId")]
    pub parent_id: u64,
    #[serde(rename = "parentUnitType")]
    pub parent_kind: RawKind,
}

impl TryFrom<RawReacted> for PublicationReacted {
    type Error = Error;

    fn try_from(value: RawReacted) -> Result<Self> {
        Ok(Self {
            index: value.index,
            account: RawAccountRef {
                id: value.account_id,
                name: value.account_name,
                gender: value.account_gender,
            }
            .try_into()?,
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
