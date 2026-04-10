mod badge;
mod ban_entry;
mod customization;
mod effect;
mod info;
mod stat;

pub(crate) use badge::RawBadge;
pub(crate) use ban_entry::RawBanEntry;
use chrono::Utc;
pub(crate) use customization::RawCustomization;
pub(crate) use effect::{
    RawEffect, RawKind as RawEffectKind, RawReasonKind as RawEffectReasonKind,
};
pub(crate) use info::RawInfo;
use serde::Deserialize;
pub(crate) use stat::RawStat;

use crate::models::Account;
use crate::requests::raw::{RawGender, RawImageRef, timestamp_from_millis};
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct RawAccount {
    #[serde(rename = "J_ID")]
    pub id: u64,
    #[serde(rename = "J_LVL")]
    pub level: f64,
    #[serde(rename = "J_LAST_ONLINE_DATE")]
    pub last_online_at: i64,
    #[serde(rename = "J_NAME")]
    pub name: String,
    pub avatar: RawImageRef,
    #[serde(rename = "sex")]
    pub gender: RawGender,
    pub karma30: f64,
    #[serde(rename = "sponsor")]
    pub sponsor_amount: u64,
    #[serde(rename = "sponsorTimes")]
    pub sponsor_count: u64,
    #[serde(rename = "accountEffects")]
    pub effects: Vec<RawEffect>,
    #[serde(rename = "czt")]
    pub customization: RawCustomization,
}

impl TryFrom<RawAccount> for Account {
    type Error = Error;

    fn try_from(value: RawAccount) -> Result<Self> {
        Ok(Self {
            id: value.id,
            level: value.level / 100.0,
            last_online_at: timestamp_from_millis(value.last_online_at)?,
            name: value.name,
            avatar: value.avatar.into(),
            gender: value.gender.try_into()?,
            karma30: value.karma30 / 100.0,
            sponsor_amount: value.sponsor_amount,
            sponsor_count: value.sponsor_count,
            effects: value
                .effects
                .into_iter()
                .filter(|effect| effect.ends_at > Utc::now().timestamp_millis())
                .map(TryInto::try_into)
                .collect::<Result<_>>()?,
            name_color: value.customization.name_color.map(
                // The server uses i32 to represent colors
                #[expect(clippy::cast_sign_loss)]
                |color| color as u32,
            ),
            active_badge: value.customization.active_badge.map(Into::into),
        })
    }
}

impl TryFrom<RawAccount> for Option<Account> {
    type Error = Error;

    fn try_from(value: RawAccount) -> Result<Self> {
        Ok(match value.id {
            0 => None,
            _ => Some(value.try_into()?),
        })
    }
}
