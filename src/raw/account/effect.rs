use chrono::DateTime;
use serde::de::Error;
use serde::Deserialize;
use serde_repr::Deserialize_repr;

use crate::models::account::{EffectKind, EffectReasonKind};
use crate::models::Effect;
use crate::Result;

#[derive(Deserialize_repr)]
#[repr(i64)]
pub(crate) enum RawEffectKind {
    Hater = 1,
    Pig = 2,
    Watchman = 3,
    Goose = 4,
    EternalWinter = 5,
    Punished = 6,
    Translator = 7,
    MentionLock = 8,
}

impl From<RawEffectKind> for EffectKind {
    fn from(value: RawEffectKind) -> Self {
        match value {
            RawEffectKind::Hater => EffectKind::Hater,
            RawEffectKind::Pig => EffectKind::Pig,
            RawEffectKind::Watchman => EffectKind::Watchman,
            RawEffectKind::Goose => EffectKind::Goose,
            RawEffectKind::EternalWinter => EffectKind::EternalWinter,
            RawEffectKind::Punished => EffectKind::Punished,
            RawEffectKind::Translator => EffectKind::Translator,
            RawEffectKind::MentionLock => EffectKind::MentionLock,
        }
    }
}

#[derive(Deserialize_repr)]
#[repr(i64)]
pub(crate) enum RawEffectReasonKind {
    None = 0,
    Gods = 1,
    RejectedBlocks = 2,
    TooManyBlocks = 3,
    Swearing = 4,
    Hater = 5,
    Uncultured = 6,
}

impl From<RawEffectReasonKind> for EffectReasonKind {
    fn from(value: RawEffectReasonKind) -> Self {
        match value {
            RawEffectReasonKind::None => EffectReasonKind::None,
            RawEffectReasonKind::Gods => EffectReasonKind::Gods,
            RawEffectReasonKind::RejectedBlocks => EffectReasonKind::RejectedBlocks,
            RawEffectReasonKind::TooManyBlocks => EffectReasonKind::TooManyBlocks,
            RawEffectReasonKind::Swearing => EffectReasonKind::Swearing,
            RawEffectReasonKind::Hater => EffectReasonKind::Hater,
            RawEffectReasonKind::Uncultured => EffectReasonKind::Uncultured,
        }
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawEffect {
    pub id: i64,
    account_id: i64,
    #[serde(rename = "dateCreate")]
    applied_at: i64,
    #[serde(rename = "dateEnd")]
    ends_at: i64,
    #[serde(rename = "comment")]
    reason: String,
    #[serde(rename = "effectIndex")]
    kind: RawEffectKind,
    #[serde(rename = "tag")]
    is_system: i64,
    #[serde(rename = "commentTag")]
    reason_kind: RawEffectReasonKind,
    from_account_name: String,
}

impl TryFrom<RawEffect> for Effect {
    type Error = crate::Error;

    fn try_from(value: RawEffect) -> Result<Self> {
        Ok(Self {
            id: value.id,
            account_id: value.account_id,
            applied_at: DateTime::from_timestamp_millis(value.applied_at).ok_or_else(|| {
                serde_json::Error::custom(format!("timestamp {} is out of range", value.applied_at))
            })?,
            ends_at: DateTime::from_timestamp_millis(value.ends_at).ok_or_else(|| {
                serde_json::Error::custom(format!("timestamp {} is out of range", value.ends_at))
            })?,
            reason: value.reason,
            kind: value.kind.into(),
            is_system: value.is_system == 1,
            reason_kind: value.reason_kind.into(),
            from_account_name: value.from_account_name,
        })
    }
}
