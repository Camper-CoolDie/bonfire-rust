use chrono::Utc;
use serde::Deserialize;

use crate::models::Link;
use crate::models::account::Info;
use crate::requests::raw::{RawImageRef, RawLink, RawPost, RawPublication, timestamp_from_millis};
use crate::{Error, Result};

#[derive(Deserialize)]
pub(crate) struct Links {
    #[serde(rename = "links")]
    pub inner: Vec<RawLink>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RawInfo {
    #[serde(rename = "dateCreate")]
    pub created_at: i64,
    #[serde(rename = "banDate")]
    pub banned_until: i64,
    #[serde(rename = "titleImage")]
    pub background: RawImageRef,
    #[serde(rename = "titleImageGif")]
    pub background_gif: RawImageRef,
    #[serde(rename = "isFollow")]
    pub is_following: bool,
    #[serde(rename = "followsYou")]
    pub follows_me: bool,
    pub follows_count: u64,
    pub followers_count: u64,
    pub status: String,
    pub age: i64,
    pub description: String,
    pub links: Links,
    pub note: String,
    pub pinned_post: Option<RawPublication<RawPost>>,
    pub bans_count: u64,
    pub warns_count: u64,
    pub karma_total: f64,
    #[serde(rename = "rates")]
    pub rates_count: u64,
    #[serde(rename = "ratesPositive")]
    pub positive_rates_sum: i64,
    #[serde(rename = "ratesNegative")]
    pub negative_rates_sum: i64,
    #[serde(rename = "moderationFandomsCount")]
    pub moderated_fandoms_count: u64,
    #[serde(rename = "viceroyFandomsCount")]
    pub curated_fandoms_count: u64,
    #[serde(rename = "subscribedFandomsCount")]
    pub subscriptions_count: u64,
    pub stickers_count: u64,
    #[serde(rename = "blackAccountsCount")]
    pub blocked_accounts_count: u64,
    #[serde(rename = "blackFandomsCount")]
    pub blocked_fandoms_count: u64,
}

impl TryFrom<RawInfo> for Info {
    type Error = Error;

    fn try_from(value: RawInfo) -> Result<Self> {
        Ok(Self {
            created_at: timestamp_from_millis(value.created_at)?,
            banned_until: match value.banned_until {
                0 => None,
                timestamp => Some(timestamp_from_millis(timestamp)?),
            }
            .filter(|date| *date > Utc::now()),
            background: value.background.into(),
            background_gif: value.background_gif.into(),
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
                .enumerate()
                .filter_map(
                    // There cannot be more than LINKS_MAX_COUNT links
                    #[expect(clippy::cast_possible_truncation)]
                    |(index, raw_link)| {
                        (!raw_link.title.is_empty() && !raw_link.uri.is_empty()).then(|| {
                            let mut link = Link::from(raw_link);
                            link.index = index as u32;
                            link
                        })
                    },
                )
                .collect(),
            note: match value.note.as_str() {
                "" => None,
                _ => Some(value.note),
            },
            pinned_post: value.pinned_post.map(TryInto::try_into).transpose()?,
            bans_count: value.bans_count,
            warns_count: value.warns_count,
            karma_total: value.karma_total / 100.0,
            rates_count: value.rates_count,
            positive_rates_sum: value.positive_rates_sum,
            negative_rates_sum: value.negative_rates_sum,
            moderated_fandoms_count: value.moderated_fandoms_count,
            subscriptions_count: value.subscriptions_count,
            curated_fandoms_count: value.curated_fandoms_count,
            stickers_count: value.stickers_count,
            blocked_accounts_count: value.blocked_accounts_count,
            blocked_fandoms_count: value.blocked_fandoms_count,
        })
    }
}
