use serde::Deserialize;

use crate::models::notification::AdminActionRejected;
use crate::requests::raw::RawAccount;
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawAdminActionRejected {
    // #[serde(rename = "mAdminVote")]
    // pub action: RawAdminAction,
    #[serde(rename = "cancelAdminAccount")]
    pub rejected_by: RawAccount,
    #[serde(rename = "actionAdminAccount")]
    pub created_by: RawAccount,
    #[serde(rename = "comment")]
    pub reason: String,
}

impl TryFrom<RawAdminActionRejected> for AdminActionRejected {
    type Error = Error;

    fn try_from(value: RawAdminActionRejected) -> Result<Self> {
        Ok(Self {
            rejected_by: value.rejected_by.try_into()?,
            created_by: value.created_by.try_into()?,
            reason: value.reason,
        })
    }
}
