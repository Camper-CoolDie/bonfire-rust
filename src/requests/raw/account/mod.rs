mod badge;
mod customization;
mod effect;
mod info;
mod prison;
mod stat;

pub(crate) use badge::RawBadge;
use chrono::DateTime;
pub(crate) use customization::RawCustomization;
pub(crate) use effect::{
    RawEffect, RawKind as RawEffectKind, RawReasonKind as RawEffectReasonKind,
};
pub(crate) use info::RawInfo;
pub(crate) use prison::RawPrisonEntry;
use serde::Deserialize;
use serde::de::Error as _;
pub(crate) use stat::RawStat;

use crate::models::Account;
use crate::requests::raw::{RawGender, RawImageRef};
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
            last_online_at: DateTime::from_timestamp_millis(value.last_online_at).ok_or_else(
                || {
                    serde_json::Error::custom(format!(
                        "timestamp {} is out of range",
                        value.last_online_at
                    ))
                },
            )?,
            name: value.name,
            avatar: value.avatar.into(),
            gender: value.gender.try_into()?,
            karma30: value.karma30 / 100.0,
            sponsor_amount: value.sponsor_amount,
            sponsor_count: value.sponsor_count,
            effects: value
                .effects
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<_>>()?,
            customization: value.customization.into(),
        })
    }
}
