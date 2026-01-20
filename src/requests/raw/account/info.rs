use chrono::{DateTime, Utc};
use serde::de::Error as _;
use serde::Deserialize;

use crate::models::account::Info;
use crate::requests::raw::{RawImageRef, RawLink, RawPost, RawPublication};
use crate::{Error, Result};

#[derive(Deserialize)]
struct Links {
    #[serde(rename = "links")]
    inner: Vec<RawLink>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawInfo {
    #[serde(rename = "dateCreate")]
    created_at: i64,
    #[serde(rename = "banDate")]
    banned_until: i64,
    #[serde(rename = "titleImage")]
    background: RawImageRef,
    #[serde(rename = "titleImageGif")]
    background_gif: RawImageRef,
    #[serde(rename = "isFollow")]
    is_following: bool,
    #[serde(rename = "followsYou")]
    follows_me: bool,
    follows_count: u64,
    followers_count: u64,
    status: String,
    age: i64,
    description: String,
    links: Links,
    note: String,
    pinned_post: Option<RawPublication<RawPost>>,
    bans_count: u64,
    warns_count: u64,
    karma_total: f64,
    #[serde(rename = "rates")]
    rates_count: u64,
    #[serde(rename = "ratesPositive")]
    positive_rates_sum: i64,
    #[serde(rename = "ratesNegative")]
    negative_rates_sum: i64,
    #[serde(rename = "moderationFandomsCount")]
    moderating_fandoms_count: u64,
    #[serde(rename = "subscribedFandomsCount")]
    subscriptions_count: u64,
    #[serde(rename = "viceroyFandomsCount")]
    viceroys_count: u64,
    stickers_count: u64,
    #[serde(rename = "blackAccountsCount")]
    blacklisted_accounts_count: u64,
    #[serde(rename = "blackFandomsCount")]
    blacklisted_fandoms_count: u64,
}

impl TryFrom<RawInfo> for Info {
    type Error = Error;

    fn try_from(value: RawInfo) -> Result<Self> {
        Ok(Self {
            created_at: DateTime::from_timestamp_millis(value.created_at).ok_or_else(|| {
                serde_json::Error::custom(format!("timestamp {} is out of range", value.created_at))
            })?,
            banned_until: match value.banned_until {
                0 => None,
                timestamp => Some(DateTime::from_timestamp_millis(timestamp).ok_or_else(|| {
                    serde_json::Error::custom(format!("timestamp {timestamp} is out of range"))
                })?),
            }
            .filter(|date| *date > Utc::now()),
            background: match value.background.id {
                0 => None,
                _ => Some(value.background.into()),
            },
            background_gif: match value.background_gif.id {
                0 => None,
                _ => Some(value.background_gif.into()),
            },
            is_following: value.is_following,
            follows_me: value.follows_me,
            follows_count: value.follows_count,
            followers_count: value.followers_count,
            status: match value.status.as_str() {
                "" => None,
                _ => Some(value.status),
            },
            age: match value.age {
                0 => None,
                _ => Some(value.age),
            },
            description: match value.description.as_str() {
                "" => None,
                _ => Some(value.description),
            },
            links: value
                .links
                .inner
                .into_iter()
                .filter_map(|link| {
                    (!link.title.is_empty() && !link.uri.is_empty()).then(|| link.into())
                })
                .collect(),
            note: match value.note.as_str() {
                "" => None,
                _ => Some(value.note),
            },
            pinned_post: value.pinned_post.map(TryInto::try_into).transpose()?,
            bans_count: value.bans_count,
            warns_count: value.warns_count,
            karma_total: value.karma_total / 100.,
            rates_count: value.rates_count,
            positive_rates_sum: value.positive_rates_sum,
            negative_rates_sum: value.negative_rates_sum,
            moderating_fandoms_count: value.moderating_fandoms_count,
            subscriptions_count: value.subscriptions_count,
            viceroys_count: value.viceroys_count,
            stickers_count: value.stickers_count,
            blacklisted_accounts_count: value.blacklisted_accounts_count,
            blacklisted_fandoms_count: value.blacklisted_fandoms_count,
        })
    }
}
