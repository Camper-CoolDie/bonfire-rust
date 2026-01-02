mod badge;
mod effect;
mod gender;
mod info;
mod link;

pub(crate) use badge::RawBadge;
use chrono::DateTime;
pub(crate) use effect::{RawEffect, RawEffectKind, RawEffectReasonKind};
pub(crate) use gender::RawGender;
pub(crate) use info::RawInfo;
pub(crate) use link::RawLink;
use serde::de::Error;
use serde::Deserialize;

use crate::models::account::AccountCustomization;
use crate::models::raw::RawImageRef;
use crate::models::{Account, Effect};
use crate::Result;

#[derive(Deserialize)]
pub(crate) struct RawAccountCustomization {
    #[serde(rename = "nc")]
    name_color: Option<i32>,
    #[serde(rename = "ab")]
    active_badge: Option<RawBadge>,
}

impl From<RawAccountCustomization> for AccountCustomization {
    fn from(value: RawAccountCustomization) -> Self {
        Self {
            name_color: value.name_color,
            active_badge: value.active_badge.map(Into::into),
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct RawAccount {
    #[serde(rename = "J_ID")]
    pub id: i64,
    #[serde(rename = "J_LVL")]
    level: i64,
    #[serde(rename = "J_LAST_ONLINE_DATE")]
    last_online_at: i64,
    #[serde(rename = "J_NAME")]
    name: String,
    avatar: RawImageRef,
    #[serde(rename = "sex")]
    gender: RawGender,
    karma30: i64,
    #[serde(rename = "sponsor")]
    sponsor_amount: i64,
    #[serde(rename = "sponsorTimes")]
    sponsor_count: i64,
    #[serde(rename = "accountEffects")]
    effects: Vec<RawEffect>,
    #[serde(rename = "czt")]
    customization: RawAccountCustomization,
}

impl TryFrom<RawAccount> for Account {
    type Error = crate::Error;

    fn try_from(value: RawAccount) -> Result<Self> {
        Ok(Self {
            id: value.id,
            level: value.level as f32 / 100.,
            last_online_at: DateTime::from_timestamp_millis(value.last_online_at).ok_or_else(
                || {
                    serde_json::Error::custom(format!(
                        "timestamp {} is out of range",
                        value.last_online_at
                    ))
                },
            )?,
            name: value.name,
            avatar: match value.avatar.id {
                0 => None,
                _ => Some(value.avatar.into()),
            },
            gender: value.gender.into(),
            karma30: value.karma30 as f32 / 100.,
            sponsor_amount: value.sponsor_amount,
            sponsor_count: value.sponsor_count,
            effects: value
                .effects
                .into_iter()
                .map(TryInto::try_into)
                .collect::<Result<Vec<Effect>>>()?,
            customization: value.customization.into(),
        })
    }
}
