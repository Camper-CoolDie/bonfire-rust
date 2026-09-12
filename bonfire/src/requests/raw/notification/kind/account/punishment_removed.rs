use serde::Deserialize;

use crate::models::notification::PunishmentRemoved;
use crate::requests::raw::{RawAccountRef, RawGender};
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawPunishmentRemoved {
    #[serde(rename = "fromAccountId")]
    pub admin_id: u64,
    #[serde(rename = "fromAccountName")]
    pub admin_name: String,
    #[serde(rename = "fromAccountSex")]
    pub admin_gender: RawGender,
    #[serde(rename = "comment")]
    pub reason: String,
}

impl TryFrom<RawPunishmentRemoved> for PunishmentRemoved {
    type Error = Error;

    fn try_from(value: RawPunishmentRemoved) -> Result<Self> {
        Ok(Self {
            admin: RawAccountRef {
                id: value.admin_id,
                name: value.admin_name,
                gender: value.admin_gender,
            }
            .try_into()?,
            reason: value.reason,
        })
    }
}
