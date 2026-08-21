use serde::Deserialize;

use crate::models::account::Stat;
use crate::models::publication::Kind;

#[derive(Deserialize)]
pub(crate) struct RawStat {
    #[serde(rename = "totalRatesPlus")]
    pub positive_rates_sum: f64,
    #[serde(rename = "totalRatesMinus")]
    pub negative_rates_sum: f64,
    #[serde(rename = "totalKarmaPlus")]
    pub others_positive_rates_sum: f64,
    #[serde(rename = "totalKarmaMinus")]
    pub others_negative_rates_sum: f64,
    #[serde(rename = "totalPosts")]
    pub posts_count: u64,
    #[serde(rename = "totalComments")]
    pub comments_count: u64,
    #[serde(rename = "totalMessages")]
    pub messages_count: u64,
    #[serde(rename = "bestPost")]
    pub best_post_id: u64,
    #[serde(rename = "bestComment")]
    pub best_comment_id: u64,
    #[serde(rename = "bestCommentUnitId")]
    pub best_comment_parent_id: u64,
    #[serde(rename = "bestCommentUnitType")]
    pub best_comment_parent_kind: i64,
}

impl From<RawStat> for Stat {
    fn from(value: RawStat) -> Self {
        let best_comment_parent_kind =
            (value.best_comment_id != 0).then_some(match value.best_comment_parent_kind {
                1 => Kind::Comment,
                8 => Kind::ChatMessage,
                9 => Kind::Post,
                10 => Kind::PostTag,
                11 => Kind::Moderation,
                12 => Kind::UserEvent,
                15 => Kind::StickerPack,
                16 => Kind::Sticker,
                17 => Kind::ModerationEvent,
                18 => Kind::AdminEvent,
                19 => Kind::FandomEvent,
                21 => Kind::Quest,
                other => Kind::Unknown(other),
            });

        Self {
            positive_rates_sum: value.positive_rates_sum / 100.0,
            negative_rates_sum: value.negative_rates_sum / 100.0,
            others_positive_rates_sum: value.others_positive_rates_sum / 100.0,
            others_negative_rates_sum: value.others_negative_rates_sum / 100.0,
            posts_count: value.posts_count,
            comments_count: value.comments_count,
            messages_count: value.messages_count,
            best_post_id: match value.best_post_id {
                0 => None,
                _ => Some(value.best_post_id),
            },
            best_comment_id: match value.best_comment_id {
                0 => None,
                _ => Some(value.best_comment_id),
            },
            best_comment_parent_id: match value.best_comment_id {
                0 => None,
                _ => Some(value.best_comment_parent_id),
            },
            best_comment_parent_kind,
        }
    }
}
